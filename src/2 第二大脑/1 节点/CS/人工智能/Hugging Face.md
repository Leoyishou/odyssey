---
draw:
tags: []
title: Hugging Face
date created: 2024-07-17
date modified: 2024-11-12
---

机器学习界的 github

Huggingface 超详细介绍 - 基本粒子的文章 - 知乎  
https://zhuanlan.zhihu.com/p/535100411

```java
model_name = "sentence-transformers/all-MiniLM-L6-v2" 
tokenizer = AutoTokenizer.from_pretrained(model_name) 
model = AutoModel.from_pretrained(model_name)
```

- [分词器](分词器.md) 可以生成 [token](token.md)，作为模型的 input
- model 可以用输入的 embedding 计算得到输出结果
- [数据集](数据集.md)：您可以使用 `datasets` 库来加载和处理数据集。
- 评估：使用 `evaluate` 库来评估模型性能。
- [Pipeline](Pipeline.md)：使用 `pipeline` 快速实现常见 NLP 任务。
