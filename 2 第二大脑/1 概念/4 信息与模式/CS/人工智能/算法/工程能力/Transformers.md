---
date created: 2024-07-18
date modified: 2025-10-05
uid: 7e558a2c-e8d2-4561-ae99-58c6d43f241d
---

# Summary

相当于[[后端开发]]的[[Spring]]，这个库大大简化了使用和微调这些强大模型的过程。

# Notes


**Hugging Face `transformers` 库速查表 (Cheatsheet)**

|   |   |   |
|---|---|---|
|**类别 (Category)**|**方法/类/函数 (Method/Class/Function)**|**描述 (Description in Transformers Context)**|
|**核心组件**|||
|模型加载与配置|`AutoConfig.from_pretrained(model_name)`|从 Hugging Face Hub 或本地路径加载模型的配置信息 (如层数、隐藏单元大小等)。|
||`AutoModel.from_pretrained(model_name)`|根据检查点名称自动推断并加载预训练的基础 Transformer 模型 (不带特定任务头)。|
||`AutoModelFor[Task].from_pretrained(model_name)`|加载预训练模型，并附加适用于特定任务的头部 (例如 `AutoModelForSequenceClassification`, `AutoModelForCausalLM`)。|
||`BertModel`, `GPT2Model`, etc.|直接加载特定架构的基础模型类。|
||`BertForSequenceClassification`, etc.|直接加载特定架构且带有任务头的模型类。|
||`model.config`|访问已加载模型的配置对象。|
|分词器 (Tokenizer)|`AutoTokenizer.from_pretrained(model_name)`|根据检查点名称自动推断并加载对应的分词器，处理文本到模型输入的转换。|
||`BertTokenizer`, `GPT2Tokenizer`, etc.|直接加载特定模型对应的分词器类。|
||`tokenizer(text_input,...)`|对单个文本或文本列表进行分词、编码、添加特殊标记，并可选地进行填充和截断。常用参数：`padding`, `truncation`, `max_length`, `return_tensors`。|
||`tokenizer.encode()` / `tokenizer.decode()`|单独进行编码（文本 -> ID）或解码（ID -> 文本）操作。|
||`tokenizer.batch_encode_plus()`|`tokenizer()` 的更底层版本，提供更多控制。|
||`tokenizer.pad_token`, `tokenizer.eos_token`, etc.|访问分词器使用的特殊标记。|
|**数据准备**|||
|数据集整合|`datasets.load_dataset()`|(来自 `datasets` 库) 从 Hub 或本地加载数据集，常与 `transformers` 配合使用。|
||`dataset.map(preprocess_function, batched=True)`|(来自 `datasets` 库) 对数据集中的每个样本应用分词等预处理函数。|
|数据收集器 (Collator)|`DataCollatorWithPadding(tokenizer)`|在批处理 (batching) 时动态地将序列填充到批内最大长度。|
||`DataCollatorForSeq2Seq(tokenizer, model)`|专门为序列到序列任务 (如翻译、摘要) 设计的数据收集器，处理编码器和解码器的输入。|
||`DataCollatorForLanguageModeling(tokenizer, mlm=...)`|为语言模型任务 (如 MLM, CLM) 准备数据，例如进行掩码操作。|
|**训练 (Trainer API)**|||
|训练参数|`TrainingArguments(output_dir,...)`|定义训练过程的所有超参数和设置 (如学习率、批大小、轮数、保存策略、评估策略、日志记录等)。|
|训练器|`Trainer(model, args, train_dataset, eval_dataset, tokenizer, data_collator, compute_metrics)`|封装了训练和评估循环的高级 API。管理模型移动到设备、优化器、学习率调度器、梯度累积、混合精度等。|
||`trainer.train()`|启动训练过程。|
||`trainer.evaluate()`|在评估数据集上运行评估。|
||`trainer.predict(test_dataset)`|在测试数据集上进行预测。|
|**推理 (Pipelines)**|||
|简化推理|`pipeline(task_name, model, tokenizer, device)`|创建一个易于使用的流水线对象，用于特定的 NLP 任务 (如 'sentiment-analysis', 'text-generation', 'ner', 'fill-mask', 'translation_xx_to_yy')。|
||`pipe = pipeline(...)`|创建流水线实例。|
||`pipe(input_data)`|使用创建的流水线对输入数据进行推理。|
|**评估**|||
|指标计算|`evaluate.load("metric_name")`|(来自 `evaluate` 库) 加载评估指标 (如 'accuracy', 'f1', 'rouge', 'bleu')。`datasets.load_metric` 是旧版 API。|
||`metric.compute(predictions, references)`|计算给定预测和参考标签的指标分数。|
||`compute_metrics(eval_pred)`|一个传递给 `Trainer` 的函数，用于在评估过程中解码预测并计算指标。`eval_pred` 是包含 `predictions` 和 `label_ids` 的元组。|
|**保存与加载**|||
|模型与分词器|`model.save_pretrained(save_directory)`|将模型权重和配置文件保存到指定目录。|
||`tokenizer.save_pretrained(save_directory)`|将分词器相关文件 (词汇表、配置) 保存到指定目录。|
||`trainer.save_model(output_dir)`|`Trainer` 提供的方法，用于保存模型、分词器（如果提供给 Trainer）和训练状态。|
||`*.from_pretrained(path_or_model_name)`|(适用于 `Auto*` 类, 具体模型类, 分词器类) 从本地目录或 Hub 加载之前保存的模型或分词器。|
|**实用工具与概念**|||
|设备管理|`.to(device)`|(继承自 PyTorch/TF) 将模型或张量移动到指定设备（CPU 或 GPU）。`Trainer` 会自动处理。|
|学习率调度器|`get_scheduler(name, optimizer, num_warmup_steps, num_training_steps)`|创建学习率调度器，例如线性预热衰减 (`get_linear_schedule_with_warmup`)。`Trainer` 通常会自动处理。|
|模型输出|`model(**inputs)`|模型前向传播调用。输入通常是 `input_ids`, `attention_mask` 等分词器输出。|
||`outputs.loss`|模型输出对象中通常包含的损失值 (如果提供了 `labels`)。|
||`outputs.logits`|模型输出对象中通常包含的未经 Softmax/Sigmoid 的原始预测分数。|
||`BaseModelOutputWithPooling`, etc.|模型输出通常是特定的数据类对象，包含 `loss`, `logits`, `hidden_states`, `attentions` 等属性。|
|日志控制|`transformers.logging.set_verbosity_info()`|设置 `transformers` 库的日志详细程度 (如 `DEBUG`, `INFO`, `WARNING`, `ERROR`)。|

## trainer_state.json

`trainer_state.json` 是 Hugging Face Transformers 训练器自动生成的**训练状态记录文件**，它详细记录了整个训练过程的状态信息。
这个文件是训练过程的"黑匣子"，记录了训练的完整状态，是判断训练是否完成的最可靠指标之一！

## 📋 trainer_state.json 的作用

### 1. **训练进度跟踪**
- 记录当前训练到第几个epoch
- 记录全局步数（global_step）
- 记录训练是否完成
### 2. **断点续训支持**
- 如果训练中断，可以从这个状态文件恢复
- 保存优化器状态、学习率调度器状态等
### 3. **训练历史记录**
- 每个检查点的损失值
- 学习率变化历史
- 评估指标历史

## 🔍 文件内容详解

让我展示一个典型的 `trainer_state.json` 结构：

```json
{
  "best_metric": null,
  "best_model_checkpoint": null,
  "epoch": 1.9956236323851204,           // 当前epoch进度
  "global_step": 228,                    // 全局训练步数
  "is_hyper_param_search": false,
  "is_local_process_zero": true,
  "is_world_process_zero": true,
  "log_history": [                       // 训练历史记录
    {
      "epoch": 0.437636761487965,
      "learning_rate": 9.123e-05,
      "loss": 2.1234,
      "step": 50
    },
    {
      "epoch": 0.87527352297593,
      "learning_rate": 8.456e-05,
      "loss": 1.8765,
      "step": 100
    }
    // ... 更多历史记录
  ],
  "max_steps": -1,                       // 最大步数限制
  "num_train_epochs": 2,                 // 计划训练的epoch数
  "total_flos": 3.173173923466445e+17,   // 总浮点运算次数
  "trial_name": null,
  "trial_params": null
}
```

## 💡 关键字段含义

| 字段 | 含义 | 您的值 |
|------|------|--------|
| `epoch` | 当前训练进度 | `1.9956` (99.56%完成) |
| `global_step` | 总训练步数 | `228` |
| `num_train_epochs` | 计划训练轮数 | `2` |
| `total_flos` | 浮点运算总量 | `3.17e+17` (衡量计算量) |
| `log_history` | 训练历史 | 每个检查点的损失、学习率等 |

## 🔧 实际用途

### 1. **监控训练进度**
```bash
# 快速查看训练进度
cat outputs/checkpoint-228/trainer_state.json | grep '"epoch"'
```

### 2. **断点续训**
如果训练中断，Transformers会自动读取这个文件来恢复训练状态

### 3. **性能分析**
```bash
# 查看损失变化趋势
cat outputs/checkpoint-228/trainer_state.json | grep -A5 -B5 '"loss"'
```

### 4. **调试和优化**
通过历史记录分析学习率调度、损失收敛等情况

## 🎯 在您的项目中

您的 `trainer_state.json` 显示：
- ✅ **训练接近完成**：epoch = 1.9956/2.0 (99.56%)
- ✅ **步数合理**：228步，符合预期
- ✅ **计算量大**：total_flos = 3.17e+17，说明进行了充分训练

**总结**：这个文件是训练过程的"黑匣子"，记录了训练的完整状态，是判断训练是否完成的最可靠指标之一！







<!-- more -->

好的，这是一个基于 Hugging Face `transformers` 库的常用 API 和概念速查表，格式类似于你提供的 PyTorch 速查表。`transformers` 库建立在 PyTorch、TensorFlow 或 JAX 之上，所以你会看到一些底层的概念（如设备管理）与 PyTorch 类似，但 `transformers` 提供了更高层次的抽象，特别是在模型加载、分词和训练/推理流程方面。

这个速查表涵盖了使用 `transformers` 库进行模型开发、训练和推理时的许多核心操作。由于 `transformers` 库非常庞大且功能丰富，这只是其中最常用的一部分。对于更高级的功能或特定模型/任务的细节，建议查阅 Hugging Face 的官方文档。
