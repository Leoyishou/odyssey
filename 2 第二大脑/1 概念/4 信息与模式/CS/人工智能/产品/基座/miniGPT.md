---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-08-10
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

这是一个简洁的 GPT (Generative Pre-trained Transformer) 实现，旨在教育目的。核心代码仅约 300 行。

核心特点：
- 简洁清晰 - 专注核心 Transformer 实现
- 教育导向 - 代码可读性高，适合学习
- 灵活配置 - 支持多种模型大小和训练配置

# Cues

[[自回归]]

# Notes
## 主要功能

1. GPT 模型训练和推理 - 支持从头训练或加载预训练模型
2. 多种模型规格 - 从 nano (3层) 到 xl (48层)
3. 字符级和 [[BPE]] 分词 - 支持文本生成任务

## 代码执行链路

### 1. 模型定义 (mingpt/model.py:95-310)

GPT类
├── 初始化: 构建 Transformer 架构
│   ├── wte: 词嵌入层
│   ├── wpe: 位置嵌入层
│   ├── Block x N层: Transformer块
│   │   ├── LayerNorm1 $\rightarrow$ 自注意力 (CausalSelfAttention)
│   │   └── LayerNorm2 $\rightarrow$ 前馈网络 (MLP)
│   └── lm_head: 输出投影层
│
├── forward(): 前向传播
│   ├── 输入token $\rightarrow$ 词嵌入 + 位置嵌入
│   ├── 通过N个Transformer块
│   └── 输出logits (+ 可选loss计算)
│
└── generate(): 文本生成
    └── 自回归预测下一个token



### 2. 训练流程 (mingpt/trainer.py:61-109)
```Java
Trainer.run()
├── 配置优化器 (AdamW)
├── 创建 DataLoader
└── 训练循环:
    ├── 获取批次数据 (x, y)
    ├── 前向传播: logits, loss = model(x, y)
    ├── 反向传播: loss.backward()
    └── 参数更新: optimizer.step()
```


### 3. 应用示例
- 字符级语言模型 (projects/chargpt/):
  - 读取文本 $\rightarrow$ 字符编码 $\rightarrow$ 训练 GPT $\rightarrow$ 生成文本
- 数字加法任务 (projects/adder/):
  - 生成加法数据 $\rightarrow$ 训练 GPT 学习加法规律
- 预训练模型使用 (simple_generate.py):
  - 加载 GPT-2 权重 $\rightarrow$ BPE 分词 $\rightarrow$ 生成文本
