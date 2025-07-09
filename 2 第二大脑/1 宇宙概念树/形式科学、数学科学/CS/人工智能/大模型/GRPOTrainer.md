---
draw:
title: GRPOTrainer
date created: 2025-02-24
date modified: 2025-02-24
---

这个名字由两个部分组成：

1. **GRPOTrainer**：
    "GRPO"代表"Group Relative Policy Optimization"，是一种基于策略优化的训练方法，用来根据生成结果的奖励对模型进行微调。这里的"Trainer"指的是使用这种方法进行训练的工具或训练器。
    
2. **DeepseekR1**：
    "DeepseekR1"是所使用的预训练模型的名称或版本，表示这是 Deepseek 系列中的 R1 版本。
    

综合起来，**GRPOTrainer-DeepseekR1** 的意思就是"利用基于 Group Relative Policy Optimization 方法的训练器对 Deepseek R1 模型进行微调"。这也说明了整个项目或 Notebook 的核心内容。

下面我将以比较通俗易懂的方式，按代码块（cell）来逐行解释这份 Notebook 的主要代码内容。注意：由于代码量比较大，我会重点解释核心部分和关键行，让你对整个流程有个清晰的认识。

---

## 1. 安装依赖包

```python
!python -m pip install --no-index -v --find-links=/kaggle/input/aimo-packages/offline_packages trl --pre
```

- **解释**：这一行通过 pip 安装名为 `trl` 的包（一个用于强化学习训练语言模型的工具包）。
- 参数说明：
    - `--no-index`：不从网络上查找，而是从本地指定的目录安装。
    - `--find-links` 指定了离线包所在的路径。
    - `--pre`：允许安装预发布版本。

接下来的几个安装命令类似，分别安装了其他依赖包（如 `levenshtein` 和 `bitsandbytes`），保证整个训练环境的依赖都就绪。

---

## 2. 设置环境变量

```python
import os
os.environ["CUDA_VISIBLE_DEVICES"] = "0,1,2,3"
os.environ["TOKENIZERS_PARALLELISM"] = "false"
```

- **解释**：
    - 第一行设置使用 GPU 设备的编号为 0、1、2、3（即使用四块 GPU）。
    - 第二行关闭了 Tokenizer 的并行处理，以避免可能的多线程警告或错误。

---

## 3. 导入所需模块

```python
from datasets import load_dataset, Dataset
from peft import LoraConfig, get_peft_model, prepare_model_for_kbit_training, PeftModel
from trl import GRPOConfig, GRPOTrainer

import datetime
from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    BitsAndBytesConfig,
    HfArgumentParser,
    AutoTokenizer,
    TrainingArguments,
    Trainer,
    GenerationConfig,
    PrinterCallback,
)
from tqdm import tqdm
import torch
import time
import transformers
import pandas as pd
import numpy as np

from Levenshtein import ratio as levenshtein_ratio
transformers.set_seed(42)
```

- **解释**：
    - 导入了 Hugging Face 的数据集工具、模型和 tokenizer、以及用于 PEFT（参数高效微调）的工具。
    - `trl` 包中导入了 GRPOTrainer 及相关配置，用于基于策略优化的训练。
    - 导入了 `tqdm` 用于显示进度条、`torch`、`pandas`、`numpy` 等常用包。
    - 最后一行设置了随机种子为 42，以确保实验可重复。

---

## 4. 配置参数类

```python
class CFG:
    MAX_TRAIN = 100
    MAX_TOKENS = 2048
    NUM_GENERATIONS = 4
    USE_PEFT = True
    BATCH_SIZE = 1
    MAX_STEPS = 80
    
    BETA = 0.04
    LR = 1.e-5
    
    model_name = '/kaggle/input/deepseek-r1/transformers/deepseek-r1-distill-qwen-1.5b/1'
    splitter = '<｜Assistant｜>'
    
    step_count = 10
    DEBUG = False
```

- **解释**：
    - 定义了一个配置类 `CFG`，里面保存了训练中的各种超参数：
        - `MAX_TRAIN`：用于训练的数据量（这里只取前 100 个例子）。
        - `MAX_TOKENS`：每个生成样本最大的 token 数量。
        - `NUM_GENERATIONS`：生成多少个答案版本（用于评估奖励）。
        - `USE_PEFT`：是否使用 PEFT 技术来微调模型（参数高效微调）。
        - `BATCH_SIZE`、`MAX_STEPS`、`BETA`（可能控制奖励项的权重）、学习率 `LR` 等。
        - `model_name`：预训练模型所在的路径。
        - `splitter`：用来分割生成文本的标记。
        - `step_count`：日志打印或保存的步数间隔。
        - `DEBUG`：是否处于调试模式。

---

## 5. 定义辅助函数：提取答案

```python
import re

def extract_boxed_text(text):
    pattern = r'oxed{(.*?)}'
    matches = re.findall(pattern, text)
    if not matches:
        return ""
    for match in matches[::-1]:
        if match != "":
            return match
    return ""
```

- **解释**：
    - 这个函数用正则表达式从文本中提取被"oxed{...}"包围的部分（可能原本意图是提取 `\boxed{...}` 里的内容，但正则中只写了 `oxed{...}`）。
    - 先搜索所有匹配项，若没有匹配则返回空字符串；否则返回最后一个非空的匹配结果。

---

## 6. 读取与处理数据集

```python
df = pd.read_parquet('/kaggle/input/math-problems-imo/math_problems.parquet')
df = df.reset_index().rename({'index': 'id'}, axis=1)
df['answer'] = df['solution'].map(extract_boxed_text)

def is_valid_answer(s):
    try:
        if float(s) == int(s):
            i = int(s)
            return 0 <= i < 1000
        else:
            return False
    except ValueError:
        return False
    
mask = df['answer'].map(is_valid_answer)
df = df[mask]
```

- **解释**：
    - 使用 pandas 读取一个 parquet 格式的数据集，该数据集包含数学问题（以及解决方案）。
    - 重置索引并将原索引重命名为 `id`。
    - 对每个 `solution` 字段调用前面定义的 `extract_boxed_text` 函数，提取出答案部分，并保存在 `answer` 列中。
    - 定义了 `is_valid_answer` 函数，检查提取出的答案是否为数字，并且在 0 到 1000 范围内。
    - 利用这个函数过滤数据，只保留答案有效的行。

接着：

```python
df = df.iloc[:CFG.MAX_TRAIN]
```

- 只取前 100 个样本（由配置中的 `MAX_TRAIN` 控制）。

然后：

```python
dataset = Dataset.from_pandas(df)
dataset = dataset.train_test_split(test_size=0.1)
```

- 将 pandas DataFrame 转换为 Hugging Face 的 Dataset 对象，并按 90%/10% 划分训练和测试集。

---

## 7. 创建 prompt（提示）模板

```python
def create_prompt(sample):
    question = sample['problem']
    chat = [{"role": "system", "content": "A conversation between User and Assistant. The user asks a question, and the Assistant solves it.  The assistant first thinks about the reasoning process in the mind and then provides the user with the answer. The reasoning process and answer are enclosed within <think> </think> and <answer> </answer> tags, respectively, i.e., <think> reasoning process here </think> <answer> answer here </answer>"},
            {"role": "user", "content": question + " Return final answer within \\boxed{}, after taking modulo 1000."},]
    sample['prompt'] = tokenizer.apply_chat_template(
            conversation=chat,
            tokenize=False,
            add_generation_prompt=True
        )
    return sample
```

- **解释**：
    - 该函数为每个样本创建一个对话式的提示：
        - 第一条信息是系统消息，说明角色分工：用户提问，助手思考后回答，并且要求回答过程（思考部分）和最终答案分别放在 `<think>` 与 `<answer>`（在这里实际使用了 `\\boxed{}` 标记）中。
        - 用户消息为实际的问题，加上一段指令"返回答案时将答案放在 \boxed{} 内，且对答案取模 1000"。
    - 最后利用 tokenizer 的 `apply_chat_template` 方法将对话内容转换为模型所需的 prompt 格式，并保存在样本的 `prompt` 字段中。

---

## 8. 查看一个样本

```python
dataset['train'][0]
```

- **解释**：显示训练集中的第一个样本，以便你了解数据结构和 prompt 格式是否正确。

---

## 9. 定义奖励函数

奖励函数用于给模型生成的回答打分，主要有三种：

### a. 格式奖励函数

```python
def format_reward_func(completions, **kwargs):
    """Reward function that checks if the completion has a specific format."""
    pattern = r"^<think>.*?</think>.*?oxed{(.*?)}.*?$"
    matches = [re.match(pattern, content, re.DOTALL) for content in completions]
    return [1.0 if match else 0.0 for match in matches]
```

- **解释**：
    - 检查生成文本是否符合要求：即必须包含 `<think>...</think>` 部分，并且后面包含 "oxed{…}"（意图是答案被标记在某个格式中）。
    - 如果符合格式，奖励为 1；否则为 0。

### b. 准确率奖励函数

```python
def accuracy_reward_func(completions, answer, **kwargs):
    # Regular expression to capture content inside \boxed{}
    contents = [extract_boxed_text(completion) for completion in completions]
    # Reward 1 if the content is the same as the ground truth, 0 otherwise
    return [1.0 if c == str(gt) else 0.0 for c, gt in zip(contents, answer)]
```

- **解释**：
    - 对每个生成结果，用之前的 `extract_boxed_text` 函数提取出答案部分。
    - 若提取结果与真实答案相同，则奖励 1，否则 0。

### c. 方案质量奖励函数（基于编辑距离）

```python
def levenshtein_reward_func(completions, solution, **kwargs):
    res = []
    for completion, sol in zip(completions, solution):
        if '</think>' in completion:
            t = completion.split('</think>')[-1]
            res.append(levenshtein_ratio(t, sol))
        else:
            res.append(0.0)
    return res
```

- **解释**：
    - 对每个生成结果，提取 `</think>` 后面的部分（认为这是最终答案的摘要）。
    - 计算这个摘要与原始解决方案之间的 Levenshtein（编辑）相似度，并用这个相似度作为奖励。
    - 如果文本中没有 `</think>`，则奖励为 0。

最后，我们将三个奖励函数存入一个字典：

```python
reward_functions = {'formatting': format_reward_func, 'accuracy': accuracy_reward_func, 'solution_quality': levenshtein_reward_func}
```

---

## 10. 模型与 Tokenizer 的加载

```python
device_map = 'auto'
if CFG.USE_PEFT:
    compute_dtype = getattr(torch, "float16")
    bnb_config = BitsAndBytesConfig(
            load_in_4bit=True,
            bnb_4bit_quant_type='nf4',
            bnb_4bit_compute_dtype=compute_dtype,
            bnb_4bit_use_double_quant=False,
        )
    original_model = AutoModelForCausalLM.from_pretrained(CFG.model_name, 
                                                          device_map=device_map,
                                                          quantization_config=bnb_config,
                                                          trust_remote_code=True)
else:
    original_model = AutoModelForCausalLM.from_pretrained(CFG.model_name, 
                                                          device_map=device_map,
                                                          trust_remote_code=True)
```

- **解释**：
    - 设置设备映射为自动选择（通常选择合适的 GPU）。
    - 如果使用 PEFT：
        - 定义量化配置 `bnb_config`，以 4-bit 模式加载模型（减少显存占用）。
        - 从指定路径加载预训练因果语言模型（CausalLM），同时使用量化配置。
    - 否则，直接加载模型（不使用 4-bit 量化）。

接着加载 Tokenizer：

```python
tokenizer = AutoTokenizer.from_pretrained(CFG.model_name, trust_remote_code=True, padding_side="left")
```

- **解释**：加载与模型对应的 Tokenizer，并设置填充方向为左边（通常用于对话生成）。

---

## 11. 给数据集创建 Prompt

```python
dataset = dataset.map(create_prompt)  # 给每个样本调用 create_prompt 函数生成 prompt
```

- **解释**：对数据集中每个样本应用前面定义的 `create_prompt` 函数，从而为训练做好输入准备。

---

## 12. 定义生成函数

```python
def gen(model, text, max_tokens):
    model_input = tokenizer(text, return_tensors='pt').to(model.device)
    model.eval()
    with torch.no_grad():
        tok = model.generate(**model_input, max_new_tokens=max_tokens, pad_token_id=tokenizer.pad_token_type_id)
        outputs = []
        for i in range(len(tok)):
            res = tokenizer.decode(tok[i], skip_special_tokens=True)
            output = res.split(CFG.splitter)[-1]
            outputs.append(output)
        return outputs[0] if len(outputs) == 1 else outputs
```

- **解释**：
    - 该函数负责用模型生成文本：
        - 将输入文本转换成模型张量，并移动到模型所在设备。
        - 设置模型为评估模式，并关闭梯度计算（节省内存）。
        - 调用模型的 `generate` 方法，生成指定数量的新 tokens。
        - 解码生成的 token 序列，按照配置中的 `splitter` 分割，取最后一部分作为生成结果。
        - 如果只生成了一个结果，则返回单个文本；如果有多个，则返回列表。

---

## 13. 定义评估奖励的函数

```python
def evaluate_rewards(model, dataset, reward_functions: dict[str, callable], max_tokens: int, num_generations: int):
    completions = []
    other_info = []
    for example in tqdm(dataset):
        txt = example['prompt']
        kw = {k: v for k, v in example.items() if k not in {'prompt', 'completion'}}
        for _ in range(num_generations):
            other_info.append(kw)
        completion = gen(model, [txt]*num_generations, max_tokens)
        if isinstance(completion, str):
            completions.append(completion)
        else:
            completions += completion
        
    kwargs = {k: [d[k] for d in other_info] for k in other_info[0].keys()}
    res = {}
    for nm, reward_func in reward_functions.items():
        v = reward_func(completions=completions, **kwargs)
        print(nm, np.mean(v))
        res[nm] = np.mean(v)
    return res
```

- **解释**：
    - 该函数遍历数据集中的样本，对每个样本生成多个答案（`num_generations` 个）。
    - 收集每个样本生成的结果（`completions`）和其他附加信息（如真实答案）。
    - 然后对每个奖励函数（格式、准确率、方案质量），计算生成结果的平均奖励值并打印出来，最后返回一个包含所有奖励的字典。

---

## 14. 训练部分

### a. 设置训练参数

```python
dtstr = datetime.datetime.now().datetime.strftime('%Y%m%d%H%M%S')
output_directory = f"./DEEPSEEK-GRPO-{dtstr}"

training_args = GRPOConfig(
    output_dir=output_directory,
    learning_rate=CFG.LR,
    per_device_train_batch_size=CFG.BATCH_SIZE,
    gradient_accumulation_steps=1,
    max_steps=CFG.MAX_STEPS,
    max_completion_length=CFG.MAX_TOKENS,  #8192
    num_generations=CFG.NUM_GENERATIONS,
    beta=CFG.BETA,
    logging_steps=CFG.step_count,
    logging_dir="./logs",
    save_strategy="steps",
    save_steps=CFG.step_count,
    report_to="none",
    overwrite_output_dir = 'True',    
)
```

- **解释**：
    - 生成一个基于当前时间戳的输出目录名称。
    - 使用 `GRPOConfig` 定义训练时的超参数，包括学习率、批次大小、最大训练步数、生成文本的最大长度、奖励参数 beta、日志记录和保存的步数等。

### b. 配置 PEFT（LoRA）并创建 Trainer

```python
if CFG.USE_PEFT:
    peft_config = LoraConfig(
        r=32,  #Rank
        lora_alpha=32,
        target_modules=['q_proj', 'k_proj', 'v_proj', 'dense'],
        bias="none",
        lora_dropout=0.05,  # Conventional
        task_type="CAUSAL_LM",
    )
    trainer = GRPOTrainer(
        model=original_model,
        reward_funcs=list(reward_functions.values()),
        args=training_args,
        train_dataset=dataset['train'],
        peft_config=peft_config,
        callbacks=[PrinterCallback()]
    )
else:
    trainer = GRPOTrainer(
        model=original_model,
        reward_funcs=list(reward_functions.values()),
        args=training_args,
        train_dataset=dataset['train'],
        callbacks=[PrinterCallback()]
    )
```

- **解释**：
    - 如果使用 PEFT（CFG.USE_PEFT 为 True），就创建一个 LoRA 配置：
        - 设置参数 `r`（低秩矩阵的秩）、`lora_alpha`（缩放因子）、指定要微调的模块（比如注意力中的 q、k、v 投影和全连接层 dense）。
        - 指定 dropout 和任务类型为因果语言模型（CAUSAL_LM）。
    - 接着利用 GRPOTrainer（策略优化的训练器）构建训练器对象，传入模型、奖励函数、训练参数、训练数据集以及回调函数（PrinterCallback 用于打印训练信息）。

### c. 开始训练

```python
trainer.train()
```

- **解释**：开始训练过程。训练过程中，模型会根据生成的回答获得奖励并更新参数。

训练结束后，会输出一系列训练指标，例如训练损失、梯度范数、奖励值以及学习率等。

---

## 15. 训练后模型的加载与评估

### a. 加载训练后的模型

```python
if CFG.USE_PEFT:
    print('Loading trained model')
    CHKPT = CFG.MAX_STEPS
    adapter_model_name = f'{output_directory}/checkpoint-{CHKPT}/'
    new_model = PeftModel.from_pretrained(original_model, adapter_model_name)
else:
    new_model = original_model
```

- **解释**：
    - 如果使用了 PEFT，则加载训练后保存的 LoRA 权重（adapter）。
    - 否则直接使用原始模型。

### b. 使用新模型评估奖励

```python
rewards = evaluate_rewards(model=new_model, dataset=dataset['test'], reward_functions=reward_functions, max_tokens=CFG.MAX_TOKENS, num_generations=CFG.NUM_GENERATIONS)
print(rewards)
```

- **解释**：
    - 在测试集上，用新模型生成答案，并计算每种奖励函数的平均分。
    - 打印出"格式奖励"、"准确率奖励"和"方案质量奖励"的最终分值。

---

## 总结

整个 Notebook 的流程可以归纳为以下几个步骤：

1. **安装依赖**：确保所有必要的包（如 trl、levenshtein、bitsandbytes 等）安装正确。
2. **环境设置与导入库**：设置 GPU 使用和导入深度学习、数据处理、以及训练相关的库。
3. **数据预处理**：读取数学题数据，对答案进行提取与过滤，然后将数据转换为 Hugging Face Dataset 格式，并划分训练/测试集。
4. **生成 Prompt**：为每个问题构造对话提示，告知模型如何回答（包括"思考"和"答案"部分）。
5. **定义奖励函数**：设计三个奖励函数来评估生成回答的格式、正确性和与原解法的相似度。
6. **加载模型和 Tokenizer**：使用预训练模型（支持 4-bit 量化以节省内存）和对应的 Tokenizer。
7. **生成和评估**：定义生成函数和评估奖励的函数，用于在训练和测试阶段生成回答并打分。
8. **训练**：利用 GRPOTrainer 配合 LoRA（PEFT）进行训练，微调模型以获得更好的回答。
9. **模型加载与测试**：训练结束后加载最新的模型，评估测试集上的奖励，检查模型效果。

希望以上逐步解释能帮助你更好地理解这份代码。如果有任何不明白的地方，欢迎继续提问！
