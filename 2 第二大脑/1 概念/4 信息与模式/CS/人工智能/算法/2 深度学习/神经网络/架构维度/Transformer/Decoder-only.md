---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-06
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

```shell
原始 Transformer (Encoder-Decoder):
├── Encoder: 双向注意力,看到全文
│   ├── Token Embedding
│   ├── Position Embedding  
│   └── N × (Self-Attention + FFN)
│
└── Decoder: 单向注意力 + Cross-Attention
    ├── Token Embedding
    ├── Position Embedding
    ├── N × (Masked Self-Attention + Cross-Attention + FFN)
    └── 输出层

【Encoder 部分】- 理解中文
	输入: "我" "爱" "北京"
	↓ Token Embedding
	↓ Position Embedding  
	↓ 多层双向 Self-Attention (每个词可以看到所有词)
	↓
	输出: [h1, h2, h3]  ← 这是"我爱北京"的语义表示
	       ↑
	       这些向量包含了整句话的含义

【Decoder 部分】- 生成英文
	输入: "<start>" 
	↓ 看 Encoder 的输出 [h1, h2, h3] (通过 Cross-Attention)
	↓ 生成 "I"
	↓
	输入: "<start> I"
	↓ 再看 Encoder 的输出
	↓ 生成 "love"
	↓
	...最终输出 "I love Beijing"


GPT (Decoder-only):
└── 只有 Decoder 部分
    ├── Token Embedding         ← 有 Embedding!
    ├── Position Embedding      ← 有 Embedding!
    ├── N × (Causal Self-Attention + FFN)  ← 单向
    └── lm_head 输出层

	没有独立的 Encoder!
	所有工作都在一个 Decoder 里完成
	
	输入: "翻译成英文: 我爱北京 → I love"
	                           ↓
	自回归生成: "Beijing"


BERT (Encoder-only):
└── 只有 Encoder 部分
    ├── Token Embedding         ← 也有 Embedding!
    ├── Position Embedding
    ├── N × (Bidirectional Self-Attention + FFN)  ← 双向
    └── 分类/MLM 输出层
    
    
    只有 Encoder (理解输入)

	输入: "我[MASK]北京"
	  ↓ 双向注意力
	输出: "我[爱]北京"  ← 填空任务
```



- **架构**：仅使用 Decoder 的 Transformer（GPT 系列）
- **任务**：自回归语言建模（Autoregressive Language Modeling）
- **核心思想**：基于前文预测下一个 token


![CleanShot 2025-07-13 at 19.14.39@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_06KynsXg3C%2F2025%2F07%2F13%2F19-15-06-78e189d2bf6a1a0608590ef8542d9151-CleanShot%202025-07-13%20at%2019.14.39-2x-c8fa06.png)

Decoder-Only：它不是让模型从O开始去生成，而是把你输入的内容，先输入到模型里面，让模型把你的内容看作是它已经生成过的内容，它只需要接着继续生成后续的内容。



好久不见 + `<BOS>` -> long
`<BOS>` + long  -> time
`<bos>` + long + time -> no
`<bos>` + long + time + no -> see
`<bos>` + long + time + no + see -> `<eos>`



# Cues



# Notes

  | 特征              | Encoder | Decoder | Decoder-only |
  |-----------------|---------|---------|--------------|
  | Token Embedding | ✓       | ✓       | ✓            |
  | Self-Attention  | ✓       | ✓       | ✓            |
  | 因果掩码            | ✗       | ✓       | ✓            |
  | 双向注意力           | ✓       | ✗       | ✗            |
  | Cross-Attention | ✗       | ✓       | ✗            |

  Decoder-only = 有 Decoder 的因果注意力，但没有 Encoder 和 Cross-Attention

  这种架构适合自回归语言建模任务，如 GPT 系列模型。
