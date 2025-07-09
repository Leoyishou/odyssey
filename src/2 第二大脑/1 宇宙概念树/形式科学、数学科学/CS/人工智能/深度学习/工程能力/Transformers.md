---
comment_id: f7b90af9
date created: 2024-07-18
date modified: 2025-04-07
draw: null
title: Transformers
---
就像[Java@](Java@.md)中的 List 和 Lists 的关系一样

架构与实现：

   - Transformer 是一种架构设计。
   - Transformers  是一个提供了多种基于 Transformer（及其他）架构的预训练模型的库。这个库大大简化了使用和微调这些强大模型的过程。


<!-- more -->

好的，这是一个基于 Hugging Face `transformers` 库的常用 API 和概念速查表，格式类似于你提供的 PyTorch 速查表。`transformers` 库建立在 PyTorch、TensorFlow 或 JAX 之上，所以你会看到一些底层的概念（如设备管理）与 PyTorch 类似，但 `transformers` 提供了更高层次的抽象，特别是在模型加载、分词和训练/推理流程方面。

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

这个速查表涵盖了使用 `transformers` 库进行模型开发、训练和推理时的许多核心操作。由于 `transformers` 库非常庞大且功能丰富，这只是其中最常用的一部分。对于更高级的功能或特定模型/任务的细节，建议查阅 Hugging Face 的官方文档。
