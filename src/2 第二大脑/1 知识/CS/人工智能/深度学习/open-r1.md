---
draw:
title: open-r1
date created: 2025-03-23
date modified: 2025-04-05
---

[微调实战 -- 用Open-R1微调Qwen模型，强化AI数学推理能力](微调实战%20--%20用Open-R1微调Qwen模型，强化AI数学推理能力.md)

[[分布式训练策略]]

[[vLLM]]

[[max_seq_length]]

## 🚀 在AutoDL平台上运行Open-R1项目详细教程

​Open-R1 是一个旨在完全开源复现 DeepSeek-R1 的项目，目标是构建 DeepSeek-R1 流程中缺失的部分，使每个人都能复制并在其基础上构建。

DeepSeek-R1 是由 DeepSeek 团队开发的一个大型语言模型，旨在提升模型的推理能力。​然而，DeepSeek-R1 的训练代码和数据集并未公开。​因此，Open-R1 项目致力于重建 DeepSeek-R1 的数据和训练流程，验证其声明，并推动开放推理模型的发展。

Open-R1 项目包括以下主要步骤：​[GitHub+2Hugging Face+2Hugging Face+2](https://huggingface.co/blog/open-r1?utm_source=chatgpt.com)

1. **复制 R1-Distill 模型**：​通过从 DeepSeek-R1 中提取高质量的推理数据集，重现 R1-Distill 模型。​[YouTube+7Hugging Face+7GitHub+7](https://huggingface.co/blog/open-r1?utm_source=chatgpt.com)
2. **复制纯强化学习流程**：​重现 DeepSeek 用于创建 R1-Zero 的纯强化学习流程，这可能涉及策划新的大规模数学、推理和代码数据集。​[YouTube+6GitHub+6GitHub+6](https://github.com/huggingface/open-r1?utm_source=chatgpt.com)
3. **多阶段训练**：​展示如何从基础模型经过监督微调（SFT）和强化学习（RL）进行多阶段训练。​[Medium+3GitHub+3Hugging Face+3](https://github.com/huggingface/open-r1?utm_source=chatgpt.com)

通过 Open-R1 项目，研究人员和开发者可以更深入地理解 DeepSeek-R1 的工作原理，并在此基础上进行进一步的研究和开发。​该项目的开源性质也促进了社区的参与和贡献，共同推动人工智能领域的进步。

---

| 目的                    | 对应步骤                       | 具体实现命令                                                                                                |
| --------------------- | -------------------------- | ----------------------------------------------------------------------------------------------------- |
| ✅ 创建隔离的运行环境           | 一（创建虚拟环境）| `conda create -n openr1 python=3.11 -y` `conda activate openr1`                                       |
| ✅ 安装项目运行的基础依赖         | 一（安装基本工具）| `pip install --upgrade pip setuptools``apt-get update && apt-get install git-lfs -y``git lfs install` |
| ✅ 获取项目源码              | 二（克隆项目）| `git clone https://github.com/huggingface/open-r1.git`                                                |
| ✅ 安装项目特定依赖（确保库版本正确）| 三（依赖安装）| 推荐：`uv pip install vllm==0.7.2 flash-attn --no-build-isolation``uv pip install -e ".[dev]"`           |
| ✅ 验证登录以获取模型和记录实验      | 四（Hugging Face 和 WandB 登录）| `huggingface-cli login``wandb login`                                                                  |
| ✅ 测试项目环境正常运行          | 五（模型推理测试）| `lighteval vllm pretrained=deepseek-ai/... --custom-tasks src/open_r1/evaluate.py`                    |
| ✅ 执行监督微调训练（SFT）| 六（模型SFT训练）| `accelerate launch --config_file=... src/open_r1/sft.py...`                                           |
| ✅ 执行强化学习训练（GRPO，进阶用法）| 七（模型RL训练）| `accelerate launch --config_file recipes/.../grpo.py --config...`                                     |
| ✅ 评估训练效果（推理性能验证）| 八（[[评估模型]]）| `lighteval vllm pretrained=... --custom-tasks src/open_r1/evaluate.py`                                |
| ✅ 合成和生成新的数据集（可选高阶）| 九（数据生成）| `python scripts/generate.py --dataset... --model... --output-dataset...`                              |

### 📌 配置环境说明

在[[AutoDL]]平台运行的是：

```java
镜像:
- PyTorch 2.5.1
- Python 3.12 (Ubuntu 22.04)
- CUDA 12.4

GPU:
- H20-NVLink (96GB) * 1

CPU:
- 20 vCPU Intel(R) Xeon(R) Platinum 8457C

内存:
- 200GB

硬盘:
- 系统盘: 30GB
- 数据盘: 免费50GB，付费600GB
```

---

### 🟢 一、准备环境与安装基础依赖

#### ✅ 解决系统盘空间不足（软链接方式）

```bash
rm -rf ~/.cache
ln -s /root/autodl-tmp ~/.cache
cd ~ && ll  # 验证成功标志: .cache -> /root/autodl-tmp/
```

---

#### ✅ 创建虚拟环境并激活

```bash
conda create -n openr1_py311 python=3.11 -y
source ~/miniconda3/bin/activate
conda activate openr1_py311
```

---

#### ✅ 安装基础依赖

```bash
pip install --upgrade pip setuptools
apt-get update && apt-get install git-lfs -y
git lfs install
```

---

### 🟢 二、获取项目源码

```bash
git clone https://github.com/huggingface/open-r1.git
cd open-r1
```

---

### 🟢 三、安装项目特定依赖

```bash
pip install vllm==0.6.6.post1 --extra-index-url https://download.pytorch.org/whl/cu121
pip install ninja
pip install git+https://github.com/microsoft/DeepSpeed.git@master
pip install -e ".[dev]"
```

---

### 🟢 四、登录 Hugging Face 和 WandB

```bash
huggingface-cli login    # 获取token: https://huggingface.co/settings/tokens
wandb login              # 获取token: https://wandb.ai/authorize
```

---

### 🟢 五、环境测试与评估

```bash
huggingface-cli download open-r1/OpenR1-Math-220k --repo-type dataset --local-dir ./data/OpenR1-Math-220k
huggingface-cli download deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B --local-dir ./model_cache

lighteval vllm \
    pretrained=deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B,dtype=bfloat16,max_model_length=8192,gpu_memory_utilization=0.6 \
    "custom|aime24|0|0" \
    --custom-tasks src/open_r1/evaluate.py \
    --use-chat-template \
    --output-dir data/evals/DeepSeek-R1-Distill-Qwen-1.5B
```

---

### 🟢 六、执行监督微调训练（SFT示例）

**基本命令**：

```bash
## 启动新训练
accelerate launch --config_file=recipes/accelerate_configs/zero2.yaml src/open_r1/sft.py \
    --model_name_or_path Qwen/Qwen2.5-1.5B-Instruct \
    --dataset_name /root/autodl-tmp/open-r1/data/OpenR1-Math-220k \
    --learning_rate 1.0e-5 \
    --num_train_epochs 1 \
    --packing \
    --max_seq_length 2048 \
    --per_device_train_batch_size 8 \
    --gradient_accumulation_steps 2 \
    --gradient_checkpointing \
    --fp16 \
    --output_dir data/Qwen2.5-1.5B-Open-R1-Distill \
    --eval_strategy no

## 从 checkpoint 恢复训练

    --resume_from_checkpoint data/Qwen2.5-1.5B-Open-R1-Distill/checkpoint-103500 \
```

如果显存不足，加入：

```bash
--gradient_accumulation_steps 4
```

|                             |                                   |                          |                                                    |
| --------------------------- | --------------------------------- | ------------------------ | -------------------------------------------------- |
| 名字                          | 值                                 | 通俗解释                     | 改为                                                 |
| model_name_or_path          | Qwen/Qwen2.5-1.5B-Instruct        | 要微调的基础模型名字或路径            |                                                    |
| model_revision              | main                              | 模型仓库分支（默认主分支）|                                                    |
| torch_dtype                 | bfloat16                          | 模型精度类型，训练更快更省显存          |                                                    |
| attn_implementation         | flash_attention_2                 | 注意力机制实现方式，更快的GPU运算       |                                                    |
| dataset_name                | open-r1/OpenR1-Math-220k          | 用于微调的数据集名称               |                                                    |
| dataset_num_proc            | 8                                 | 并发加载数据集时使用的线程数量          | 8 下载东西时候的并发线程数                                     |
| bf16                        | TRUE                              | 开启bf16格式（更高效的训练格式）| 用fp16，bf16 更适合 H100 的显卡                            |
| do_eval                     | FALSE                             | 是否进行模型评估（这里不评估）|                                                    |
| eval_strategy               | no                                | 评估策略（无评估）|                                                    |
| gradient_accumulation_steps | 1                                 | 梯度累积次数（多少个batch更新一次模型）| 32                                                 |
| gradient_checkpointing      | TRUE                              | 节省显存，用计算换显存              | 可以关掉，速度会更快                                         |
| use_reentrant               | FALSE                             | 使用更高效的梯度checkpoint方式     |                                                    |
| hub_model_id                | Qwen2.5-1.5B-Open-R1-Distill      | 模型在Hub上发布的名字（如果推送）|                                                    |
| hub_strategy                | every_save                        | 上传模型到Hub的频率（每次保存都上传）|                                                    |
| learning_rate               | 5.00E-05                          | 学习率，决定模型参数更新幅度           |                                                    |
| log_level                   | info                              | 日志记录级别（一般记录）|                                                    |
| logging_steps               | 5                                 | 每隔多少步记录一次训练日志            |                                                    |
| logging_strategy            | steps                             | 按步数记录日志                  |                                                    |
| lr_scheduler_type           | cosine_with_min_lr                | 学习率调度方式（余弦退火，有最低限度）|                                                    |
| min_lr_rate                 | 0.1                               | 最终学习率会降到初始学习率的10%        |                                                    |
| packing                     | TRUE                              | 是否将多个短样本合并成长序列，加速训练      |                                                    |
| max_length                  | 16384                             | 模型支持的最大token序列长度         | 4096，如果这个数字非常大，意味着模型要一次性处理长度达1万多个token的数据，会消耗巨量显存。|
| max_steps                   | -1                                | 最大训练步数（-1代表不限制，按epoch训练）|                                                    |
| num_train_epochs            | 1                                 | 数据集训练的遍数                 |                                                    |
| output_dir                  | data/Qwen2.5-1.5B-Open-R1-Distill | 训练模型保存的文件夹               |                                                    |
| overwrite_output_dir        | TRUE                              | 如果输出目录存在则覆盖              |                                                    |
| per_device_eval_batch_size  | 16                                | 每个设备（GPU）评估时的批次大小        |                                                    |
| per_device_train_batch_size | 16                                | 每个设备（GPU）训练时的批次大小        | 8，根据 GPU 显存占用比例调，85%的话算跑满                          |
| push_to_hub                 | TRUE                              | 是否将模型推送到HuggingFace Hub  | FALSE                                              |
| report_to                   | wandb                             | 训练指标不上报到任何平台             | "none"                                             |
| save_strategy               | "steps"                           | 模型保存策略，按步数保存             |                                                    |
| save_steps                  | 100                               | 每100步保存一次检查点             |                                                    |
| save_total_limit            | 1                                 | 最多只保留1个最近的模型检查点          |                                                    |
| seed                        | 42                                | 随机种子，确保结果可重复             |                                                    |
| use_liger                   | TRUE                              | 使用Liger优化库，加快训练          |                                                    |
| warmup_ratio                | 0.05                              | 前5%的训练阶段逐渐升高学习率，避免不稳定    |                                                    |
| max_seq_length              |                                   |                          | 2048，修改这个会触发重新 packing                             |

**训练进度条已经达到100%后，生成了最终微调后的模型**

```Java
- model.safetensors（模型权重文件）
    
- config.json（模型架构配置）
    
- tokenizer.json、tokenizer_config.json、vocab.json（分词器相关文件）
    
- generation_config.json（生成推理的默认参数）
    
```

![CleanShot 2025-03-25 at 17.23.48@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_HScb9GxJuV%2F2025%2F03%2F25%2F17-24-53-34b6d6e92937282b6ef10ca405415d25-CleanShot%202025-03-25%20at%2017.23.48-2x-fdad89.png)

**train metrics（训练指标）会显示**：
```Java
***** train metrics *****
  total_flos               =   463968GF
  train_loss               =     0.1392
  train_runtime            = 5:21:43.37
  train_samples            =      93733
  train_samples_per_second =     14.198
  train_steps_per_second   =      7.099


训练损失（loss）已经稳定，最终为`0.1392`，说明模型已经很好地收敛了。

```

![CleanShot 2025-03-25 at 17.22.15@2x.png|1500](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_Sv3gJdYXEX%2F2025%2F03%2F25%2F17-22-27-eb61907a4bd35481284eb78502dbfa89-CleanShot%202025-03-25%20at%2017.22.15-2x-ed8da5.png)

---

### 🟢 七、执行强化学习训练（GRPO示例，进阶用法）

```bash
ACCELERATE_LOG_LEVEL=info accelerate launch --config_file recipes/accelerate_configs/zero2.yaml \
    --num_processes=7 src/open_r1/grpo.py \
    --config recipes/Qwen2.5-Math-7B/grpo/config_simple_rl.yaml
```

- 推荐显存 ≥ 40GB，A100、H100 等更佳。

---

## 训练后

- **评估模型**：
    使用脚本如`evaluate.py`，运行对应的评估benchmark（如MATH-500或AIME-2024），确认模型性能。
    
- **保存和上传模型**（可选）：
```shell
huggingface-cli login
huggingface-cli repo create your-model-name
git add .
git commit -m "helloworld"
git push
```
    
- **模型推理和生成数据**：
    使用训练好的模型进行推理，验证模型效果，或用模型生成新的数据。

### 🟢 八、评估训练效果（推理性能验证）

[评估模型](评估模型.md)

```bash
MODEL=data/Qwen2.5-1.5B-Open-R1-Distill
MODEL_ARGS="pretrained=$MODEL,dtype=bfloat16,max_model_length=32768,gpu_memory_utilization=0.8"
OUTPUT_DIR=data/evals/$MODEL

lighteval vllm $MODEL_ARGS "custom|math_500|0|0" \
    --custom-tasks src/open_r1/evaluate.py \
    --use-chat-template \
    --output-dir $OUTPUT_DIR

MODEL=data/Qwen2.5-1.5B-Open-R1-Distill
MODEL_ARGS="pretrained=$MODEL,dtype=fp16,max_model_length=8192,gpu_memory_utilization=0.5"
OUTPUT_DIR=data/evals/Qwen2.5-1.5B-eval

lighteval vllm $MODEL_ARGS "custom|math_500|0|0" \
    --custom-tasks src/open_r1/evaluate.py \
    --use-chat-template \
    --output-dir $OUTPUT_DIR



类似的方式，你也可以评估： 
• math_500：数学推理任务 
• gpqa：diamond ：常识推理任务 
• lcb：codegeneration：代码生成任务
```

---

### 🟢 九、合成与生成新数据集（高级用法）

```bash
uv pip install "distilabel[vllm]>=1.5.2"
python scripts/generate.py \
  --dataset AI-MO/NuminaMath-TIR \
  --prompt-column problem \
  --model deepseek-ai/DeepSeek-R1-Distill-Qwen-7B \
  --temperature 0.6 \
  --output-dataset your-username/generated-r1-dataset
```

---

### 🚨 常见问题排查（针对出现的SIGFPE错误）

**快速解决步骤：**

- 将启动命令中的`--bf16`改为`--fp16`。
- DeepSpeed配置中关闭CPU offload（`offload_optimizer_device: none`）。
- 降低学习率到`5e-6`。

修正后的DeepSpeed配置示例：

```yaml
compute_environment: LOCAL_MACHINE
deepspeed_config:
  gradient_accumulation_steps: 4
  gradient_clipping: 1.0
  offload_optimizer_device: none
  offload_param_device: none
  zero3_init_flag: true
  zero_stage: 3
distributed_type: DEEPSPEED
downcast_bf16: 'no'
machine_rank: 0
num_machines: 1
num_processes: 1
rdzv_backend: static
use_cpu: false
```

---

完成以上步骤后，你将顺畅且稳定地在AutoDL平台运行和扩展Open-R1项目。

希望本次整理的教程能帮助你高效开展后续的研究与开发工作！🚀✨

---


---

### 在AutoDL平台上运行

下面我以详细的步骤教你如何在AutoDL平台上成功运行你提到的这个开源项目（**huggingface/open-r1**）：

以上步骤确保你能完整、顺畅地在 AutoDL 平台运行和扩展 Open-R1 项目，理解其核心技术，并为进一步的研究和开发奠定坚实基础。

#### 🟢 一、准备环境与安装基础依赖

在AutoDL中，首先进入你的服务器（使用带有 GPU 环境，如A100、H100、RTX3090 等高显存机器更好）。

##### 软连接

彻底解决了因系统盘空间不足导致无法下载大模型的问题

| 步骤  | 操作     | 命令示例                                            | 作用           |
| --- | ------ | ----------------------------------------------- | ------------ |
| ①   | 删除原有缓存 | `rm -rf ~/.cache`                               | 删除占满系统盘的缓存文件 |
| ②   | 建立软链接  | `ln -s /root/autodl-tmp ~/.cache`               | 将缓存目录指向数据盘   |
| ③   | 验证链接   | `cd ~ && ll`                                    | 检查软链接是否建立成功  |
| ④   | 开始训练   | `xtuner train internlm_20b_qlora_oasst1_512_e3` | 启动训练任务       |

验证成功的标志：

```Java
.cache -> /root/autodl-tmp/
```

##### 1. 创建虚拟环境并激活

```bash
conda create -n openr1_py311 python=3.11 -y
source ~/miniconda3/bin/activate
conda activate openr1_py311
```

##### 2. 安装基本工具与依赖

```bash
pip install --upgrade pip setuptools
```

安装 Git LFS (用于下载大模型权重):

```bash
apt-get update && apt-get install git-lfs -y
git lfs install
```

---

#### 🟢 二、克隆项目到本地

```bash
git clone https://github.com/huggingface/open-r1.git
cd open-r1
```

---

#### 🟢 三、安装项目依赖

```bash


pip install vllm==0.6.6.post1 --extra-index-url https://download.pytorch.org/whl/cu121

pip install ninja
pip install git+https://github.com/microsoft/DeepSpeed.git@master


pip install -e ".[dev]"
```

此过程会自动安装PyTorch 2.5.1、Accelerate、transformers 等重要库。

---

#### 🟢 四、登录 Hugging Face 和 WandB（权重管理和训练监控）

登录 Hugging Face:

```bash
huggingface-cli login
```

（访问 [https://huggingface.co/settings/tokens](https://huggingface.co/settings/tokens) 获取token）

登录 Weights & Biases:

```bash
wandb login
```

（访问 [https://wandb.ai/authorize](https://wandb.ai/authorize) 获取token）

---

#### 🟢 五、测试项目是否正常运行（可选但推荐）

执行一次简单的评测，确保环境正常运行，例如：

```bash

# 自己手动下载模型和数据
huggingface-cli download open-r1/OpenR1-Math-220k --repo-type dataset --local-dir ./data/OpenR1-Math-220k
huggingface-cli download deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B --local-dir ./model_cache



lighteval vllm \
    pretrained=deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B,dtype=bfloat16,max_model_length=8192,gpu_memory_utilization=0.6 \
    "custom|aime24|0|0" \
    --custom-tasks src/open_r1/evaluate.py \
    --use-chat-template \
    --output-dir data/evals/DeepSeek-R1-Distill-Qwen-1.5B
```

这一步若成功则说明环境配置正确，模型下载和加载没有问题。

---

#### 🟢 六、运行模型训练（SFT示例）

示例：在`open-r1/OpenR1-Math-220k`数据集上运行SFT（监督微调）训练：

```bash
accelerate launch --config_file=recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py \
    --model_name_or_path Qwen/Qwen2.5-1.5B-Instruct \
    --dataset_name /root/autodl-tmp/open-r1/data/OpenR1-Math-220k \
    --learning_rate 1.0e-5 \
    --num_train_epochs 1 \
    --packing \
    --max_seq_length 4096 \
    --per_device_train_batch_size 2 \
    --gradient_checkpointing \
    --bf16 \
    --output_dir data/Qwen2.5-1.5B-Open-R1-Distill








accelerate launch --config_file=recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py \
    --model_name_or_path Qwen/Qwen2.5-1.5B-Instruct \
    --dataset_name /root/autodl-tmp/open-r1/data/OpenR1-Math-220k \
    --learning_rate 1.0e-5 \
    --num_train_epochs 1 \
    --packing \
    --max_seq_length 8192 \
    --per_device_train_batch_size 8 \
    --gradient_checkpointing \
    --bf16 \
    --output_dir data/Qwen2.5-1.5B-Open-R1-Distill

accelerate launch --config_file=recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py \
  --model_name_or_path /root/autodl-tmp/Qwen2.5-1.5B-Instruct \
  --dataset_name /root/autodl-tmp/open-r1/data/OpenR1-Math-220k \
  --learning_rate 1.0e-5 \
  --num_train_epochs 1 \
  --packing \
  --max_seq_length 8192 \
  --per_device_train_batch_size 8 \
  --gradient_accumulation_steps 2 \
  --gradient_checkpointing \
  --bf16 \
  --output_dir data/Qwen2.5-1.5B-Open-R1-Distill

```

- 你可能需要根据GPU显存情况调整`per_device_train_batch_size`（一般设置为4、8或16）。
- 如果GPU显存不足，建议增加`--gradient_accumulation_steps`参数，如：
    
    ```bash
    --per_device_train_batch_size 4 
    --gradient_accumulation_steps 4
    ```

*针对 1卡 RTX 4090

```bash


cd ~/autodl-tmp/projects/open-r1

export PYTORCH_CUDA_ALLOC_CONF=expandable_segments:True

accelerate launch --config_file=recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py \
    --model_name_or_path /root/autodl-tmp/Qwen2.5-1.5B-Instruct \
    --dataset_name /root/autodl-tmp/projects/open-r1/data/OpenR1-Math-220k \
    --learning_rate 1.0e-5 \
    --num_train_epochs 1 \
    --packing \
    --max_seq_length 2048 \
    --per_device_train_batch_size 1 \
    --gradient_accumulation_steps 8 \
    --gradient_checkpointing \
    --bf16 \
    --output_dir data/Qwen2.5-1.5B-Open-R1-Distill
```

**参数调整原因：**

- 改用`zero2`配置降低单卡负载。
- 调整`max_seq_length=8192`，缓解显存占用。
- 增加`gradient_accumulation_steps=8`，可保证更高的等效batch。

[[OutOfMemoryError]]

```Java
torch.OutOfMemoryError: CUDA out of memory.

- 模型（Qwen2.5-1.5B）本身虽然不算特别大，但你当前的训练配置中设置了：
    - `--per_device_train_batch_size 8` （每个设备的批次大小）
    - `--max_seq_length 16384` （序列长度达到16384 token，非常长）
    
这两个因素同时作用，导致显存使用远远超过你的GPU（RTX 4090，24GB）的容量，从而训练启动后直接显存溢出。
```

| 步骤               | 操作内容                        | 说明                              | 耗时                |
| ---------------- | --------------------------- | ------------------------------- | ----------------- |
| 1. 环境初始化         | 自动探测并设置DeepSpeed的CUDA加速器    | 设置ds_accelerator至cuda，初始化NCCL后端 | 几秒                |
| 2. 数据集处理         | 解析数据文件                      | 读取本地数据集`OpenR1-Math-220k`       | 几秒                |
|                  | 生成训练数据集 (train split)       | 共处理93,733条训练样本                  | 约12秒              |
| 3. 模型初始化         | 加载模型`Qwen2.5-1.5B-Instruct` | 模型参数共约17.8亿个元素                  | 几秒                |
|                  | 加载checkpoint权重              | 完成checkpoint文件加载                | 约1秒               |
| 4. 数据预处理         | 转换数据为ChatML格式               | 将数据转化为模型训练的ChatML输入格式           | 约21秒              |
|                  | 应用聊天模板到数据集                  | 应用chat template到训练数据            | 约28秒              |
| 5. Tokenization  | 数据集Tokenization（分词）| 对93,733条数据进行tokenize            | 约18分钟             |
| 6. 数据[[Packing]] | 数据打包操作（packing）| 提升训练效率，高效打包tokenized后的数据        | 约37分钟             |
| 7. DeepSpeed参数   | 参数offload                   | 确定参数的offload策略，提高GPU内存使用效率      | 几秒                |
| 8. WandB 初始化     | 启动Weights & Biases服务        | 提供3种方式供用户选择（创建账号/使用账号/不进行可视化）| 等待用户输入（此处你选择创建账号）|

以上步骤完成后，你选择了“创建一个新的 WandB 账号”并开始了后续的登录和使用操作。

---

#### 🟢 七、运行RL训练（GRPO示例，可选进阶）

```bash
ACCELERATE_LOG_LEVEL=info accelerate launch --config_file recipes/accelerate_configs/zero2.yaml \
    --num_processes=7 src/open_r1/grpo.py \
    --config recipes/Qwen2.5-Math-7B/grpo/config_simple_rl.yaml
```

注意：

- GRPO训练涉及到更多资源，需要至少2-8张高性能GPU。
- 推荐显存 ≥ 40GB（例如A100、H100）确保稳定训练。

---

#### 🟢 八、评估已训练好的模型（评测推理阶段）

例如，运行 MATH-500 评估：

```bash
MODEL=deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B
MODEL_ARGS="pretrained=$MODEL,dtype=bfloat16,max_model_length=32768,gpu_memory_utilization=0.8"
OUTPUT_DIR=data/evals/$MODEL

lighteval vllm $MODEL_ARGS "custom|math_500|0|0" \
    --custom-tasks src/open_r1/evaluate.py \
    --use-chat-template \
    --output-dir $OUTPUT_DIR
```

---

#### 🟢 九、运行数据生成（合成新数据集，可选高级用法）

以小模型为例生成数据：

```bash
uv pip install "distilabel[vllm]>=1.5.2"
python scripts/generate.py \
  --dataset AI-MO/NuminaMath-TIR \
  --prompt-column problem \
  --model deepseek-ai/DeepSeek-R1-Distill-Qwen-7B \
  --temperature 0.6 \
  --output-dataset your-username/generated-r1-dataset
```

生成数据集后可上传到Hugging Face以便后续使用。

---

#### 🚨 常见问题排查与注意事项

- 如果出现 **CUDA Segmentation Fault**，请确认CUDA版本为12.4：
    
    ```bash
    nvcc --version
    ```
    
- 显存不够时尝试：
    
    - 降低batch size
    - 增加`gradient_accumulation_steps`
    - 开启`gradient_checkpointing`
- 确认网络正常（AutoDL机器下载模型时可能较慢），建议使用镜像加速或者提前手动下载模型权重。
    

---

#### 🎯 总结（核心命令速查）

| 动作     | 核心命令                                       |
| ------ | ------------------------------------------ |
| 创建环境   | `conda create -n openr1 python=3.11`       |
| 激活环境   | `conda activate openr1`                    |
| 安装依赖   | `uv pip install -e ".[dev]"`               |
| SFT训练  | `accelerate launch... src/open_r1/sft.py`  |
| GRPO训练 | `accelerate launch... src/open_r1/grpo.py` |
| 模型评估   | `lighteval vllm pretrained=...`            |

---

### AutoDI系统盘数文件移到数据盘（详细教程）

#### 📌 你的整体思路分析

你当前遇到的问题是：

- AutoDL平台的**系统盘空间较小且无法扩容**；
- 模型文件默认下载到系统盘的缓存目录（`~/.cache`）导致空间不足；
- 数据盘（`/root/autodl-tmp`）空间大且可用。

你给出的解决方案是：

**『将默认的缓存目录 `~/.cache` 转移到数据盘，通过软链接形式将数据盘与系统盘目录关联』**

---

#### 🔗 具体实现步骤（已完善）

| 步骤  | 操作     | 命令示例                                            | 作用           |
| --- | ------ | ----------------------------------------------- | ------------ |
| ①   | 删除原有缓存 | `rm -rf ~/.cache`                               | 删除占满系统盘的缓存文件 |
| ②   | 建立软链接  | `ln -s /root/autodl-tmp ~/.cache`               | 将缓存目录指向数据盘   |
| ③   | 验证链接   | `cd ~ && ll`                                    | 检查软链接是否建立成功  |
| ④   | 开始训练   | `xtuner train internlm_20b_qlora_oasst1_512_e3` | 启动训练任务       |

验证成功的标志：

```Java
.cache -> /root/autodl-tmp/
```

---

#### 🚩 该方案的优势

- **长期有效**：彻底解决了因系统盘空间不足导致无法下载大模型的问题；
- **简单易行**：不需要复杂操作，命令简单易懂；
- **灵活可扩展**：今后下载的模型自动存储到数据盘，再大文件也不用担心。

---

#### ⚠️ 但注意几个细节

1. **数据安全性**：
    
    - 确认`/root/autodl-tmp`目录不是临时目录（AutoDL一般会保留该目录数据直至实例释放）。
2. **权限与路径**：
    
    - 执行软链接时确保以root用户执行命令。
3. **原目录文件备份**：
    
    - 若原缓存目录下有重要文件，建议先备份至数据盘再执行删除。

---

#### 🚨 可能的问题排查

| 可能问题 | 解决方案 |
| ---- | ---- |
| 建立软  |      |

你遇到的这个错误：

yaml

CopyEdit

`exitcode: -8 (pid: 2116) traceback: Signal 8 (SIGFPE) received by PID 2116`

表示程序触发了浮点运算错误（Floating Point Exception，SIGFPE）。

在深度学习模型训练中，这通常由以下情况导致：

---

### 🚩 一、问题的可能原因

- **数值异常**：
    
    - 出现了 NaN 或者 Inf，特别在启用 `bf16` 或者较激进的优化器配置时，可能导致数值不稳定。
        
    - 计算loss过程中出现了数值溢出（overflow）或除零操作。
        
- **DeepSpeed 或 PyTorch 本身的兼容性问题**：
    
    - 使用`bf16`训练时，某些计算可能出现不兼容或bug。
        
    - CPU offload 和 DeepSpeed ZeRO3 结合，可能存在稳定性问题。
        
- **梯度裁剪未生效**：
    
    - 如果`gradient_clipping`参数未正确生效，可能会导致数值异常。
        

---

### 🚩 二、针对你当前的情况快速解决方案

建议你逐步做以下调整：

#### ✅ 第一步（最关键，先尝试此步）

##### **改用fp16而非bf16** (bf16偶尔可能引发兼容性问题)

将启动命令中的`--bf16`替换为：

bash

CopyEdit

`--fp16`

> **注意：**fp16在NVIDIA A100 GPU上高度稳定，尤其当bf16出现问题时fp16通常更稳妥。

#### 修复后启动命令示例

bash

CopyEdit

`accelerate launch --config_file=recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py \     --model_name_or_path Qwen/Qwen2.5-1.5B-Instruct \     --dataset_name /root/autodl-tmp/open-r1/data/OpenR1-Math-220k \     --learning_rate 1.0e-5 \     --num_train_epochs 1 \     --packing \     --max_seq_length 4096 \     --per_device_train_batch_size 2 \     --gradient_checkpointing \     --fp16 \     --output_dir data/Qwen2.5-1.5B-Open-R1-Distill`

---

#### ✅ 第二步（如果上述未解决，继续尝试此步）

##### **调整 DeepSpeed 配置以确保稳定性**

修改你的`zero3.yaml`为：

yaml

CopyEdit

`compute_environment: LOCAL_MACHINE deepspeed_config:   gradient_accumulation_steps: 4   gradient_clipping: 1.0   offload_optimizer_device: none   offload_param_device: none   zero3_init_flag: true   zero_stage: 3 distributed_type: DEEPSPEED downcast_bf16: 'no' machine_rank: 0 num_machines: 1 num_processes: 1 rdzv_backend: static use_cpu: false`

> 去掉 CPU offload，完全在GPU上执行，有时能解决由DeepSpeed CPU offload导致的SIGFPE问题。

---

#### 🚨 **额外检查与预防措施**

1. **数值稳定性检查**：
    训练前或训练初期加一段代码检查loss值，如果出现NaN或inf立刻停止并降低学习率或调整优化器。
    
2. **学习率降低**：
    若还出现数值异常，尝试将`learning_rate`降低为`5e-6`甚至`1e-6`：
    

    bash

    

    CopyEdit

    

    `--learning_rate 5.0e-6`

    
3. **重启环境**：
    有时环境异常（特别是GPU进程异常）可能导致这个问题，可以尝试重启Jupyter kernel或直接重启容器。
    

---

### ⏳ **预期效果与训练时长**

- 上述改用fp16后，对训练速度无负面影响（甚至略有提升）。
    
- 显存占用与bf16接近或略有减少，仍然稳定在30-50GB之间。
    
- 预计训练时长保持不变 (约3小时
