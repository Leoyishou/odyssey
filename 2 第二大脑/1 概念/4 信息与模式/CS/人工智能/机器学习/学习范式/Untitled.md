---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-28
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary



# Cues



# Notes
# 飞书云文档

---

## 代码熊的大模型知识库

* **企业公开**
* **搜索**
* **目录（暂无内容）**
* **与我分享**

---

## 📚 目录

* **🎯 从0到∞: 大语言模型知识面试一本通**
    * 1. 大语言模型基础
        * 1.1 Transformer
        * 1.2 模型参数量计算
        * 1.3 面试考点解析
    * 2. 大语言模型架构
    * 3. 大语言模型预训练
        * 3.1 预训练相关概念
        * 3.2 预训练数据相关
        * 3.3 Tokenizer训练
        * 3.4 大模型训练侧优化
        * 3.5 训练/推理显存占用分析
        * 3.6 LoRA训练显存占用分析
        * 3.7 大模型训练节约显存的方法？
        * 3.8 面试考点解析
    * 4. 大模型监督微调
        * 4.1 微调基础概念
        * 4.2 Prompt Tuning
        * 4.3 Instruction Tuning
        * 4.4 参数高效微调PEFT
        * 4.5 面试考点解析
    * 5. 大语言模型强化学习
        * 5.1 强化学习基础概念
        * 5.2 RLHF与近端策略优化PPO
        * 5.3 直接偏好优化DPO
        * 5.4 o1相关技术
        * 5.5 面试考点解析
        * 5.6 从概率模型上看SFT和RL的区别
    * 6. 提示工程与RAG
    * 7. 大模型推理侧优化
    * 8. 大语言模型评测
    * 9. 多模态笔记
    * 10. 大模型项目实践
* **2025年最好的大模型学习路线**
* **缓冲区：论文笔记**
* **补充篇：笔试、面试技巧**
* **新鲜面经**
* **互联网提前批入口总结**
* **手撕合集（更新中）**

---

<small>代码熊的大模型知识库 > 🎯从0到∞: 大语言模型知识面试一本通 > 1. 大语言模型基础 > **1.3 面试考点解析**</small>

# 1.3 面试考点解析

* **Authors:** 古希腊掌管代码的神, 飞猪环游记THU
* **Last Modified:** 7月12日 21:15

> ### 🤖 AI 速览
>
> 本文讨论了面试中关于Transformer相关知识的考点解析，涵盖手撕代码、原理问答、参数量计算、解码方法等多个方面。关键要点包括：
>
> 1.  **手撕Transformer模块**：给出了MHA、FFN、Positional Embedding、Encoder、Decoder等模块的代码实现，还给出了Transformer整体的代码及测试代码。
> 2.  **手撕Attention**：提供了使用`np.einsum`和`np.matmul`两种方式实现Multi-Head Attention的代码及测试脚本。
> 3.  **K、Q、V使用问题**：K和Q使用不同的投影矩阵计算，能增加表达能力和泛化能力；若K和Q相同，attention score矩阵是对称矩阵，泛化能力差。
> 4.  **Decoder的Cross Attn Mask**：使用padding mask屏蔽来自encoder的padding token，不用融合casual mask也不会出现attn信息泄漏问题。
> 5.  **大模型输出解码方法**：有Greedy、Random Sampling、Top-K、Top-P、BeamSearch、Blockwise Parallel Decoding、Speculative Sampling、Contrastive Decoding等方法，各方法在解码速度、质量和多样性上各有优劣。
> 6.  **扩充词表**：优点是提高编解码效率、上下文长度和中文能力（存疑），步骤为扩充词表（去重）和Embedding初始化（随机或用原词表均值扩充）。
> 7.  **Prenorm与Postnorm区别**：Pre-Norm结构往往更容易训练，但最终效果通常不如Post-Norm，在transfomer layers较多时，Pre-Norm会造成退化，Post-Norm训练任务更难但更不易收敛。

## Q1: 手撕Transformer中的各个模块

**A1: 参考代码**

主要几个部分：MHA、FFN、Positional Embedding、Encoder & Decoder

### MHA (Multi-Head Attention)

```python
import torch
import torch.nn as nn
import torch.nn.functional as F
import math

class MultiHeadAttention(nn.Module):
  def __init__(self, embed_size, heads):
    super(MultiHeadAttention, self).__init__()
    self.embed_size = embed_size
    self.heads = heads
    self.head_dim = embed_size // heads
    assert (self.head_dim * self.heads == self.embed_size), "embed_dim needs to be divisible by heads"
    
    self.k = nn.Linear(self.embed_size, self.embed_size, bias=False)
    self.q = nn.Linear(self.embed_size, self.embed_size, bias=False)
    self.v = nn.Linear(self.embed_size, self.embed_size, bias=False)
    self.fc_out = nn.Linear(self.heads * self.head_dim, self.embed_size)
    
  def forward(self, value, key, query, mask):
    b = query.shape[0]
    value_len, key_len, query_len = value.shape[1], key.shape[1], query.shape[1]
    
    q, k, v = self.q(query), self.k(key), self.v(value)
    
    # Split the embedding into self.heads different pieces
    q = q.reshape(b, query_len, self.heads, self.head_dim)
    k = k.reshape(b, key_len, self.heads, self.head_dim)
    v = v.reshape(b, value_len, self.heads, self.head_dim)
    
    # Einsum does matrix multiplication for query*keys for each training example
    # with every other key, then scale, mask, and apply softmax
    attention = torch.einsum("bqhd,bkhd->bhqk", [q, k])
    if mask is not None:
      attention = attention.masked_fill(mask == 0, float("-1e20"))
      
    attention = F.softmax(attention / math.sqrt(self.head_dim), dim=3)
    out = torch.einsum("bhqk,bkhd->bqhd", [attention, v]).reshape(b, query_len, self.heads * self.head_dim)
    out = self.fc_out(out)
    return out
```

### Transformer Block

```python
class TransformerBlock(nn.Module):
  def __init__(self, embed_size, heads, dropout, inner_dim):
    super(TransformerBlock, self).__init__()
    self.attention = MultiHeadAttention(embed_size, heads)
    self.norm1 = nn.LayerNorm(embed_size)
    self.norm2 = nn.LayerNorm(embed_size)
    
    self.feed_forward = nn.Sequential(
        nn.Linear(embed_size, inner_dim),
        nn.ReLU(),
        nn.Linear(inner_dim, embed_size)
    )
    self.dropout = nn.Dropout(dropout)

  def forward(self, value, key, query, mask):
    attention = self.attention(value, key, query, mask)
    x = self.dropout(self.norm1(attention + query))
    forward = self.feed_forward(x)
    out = self.dropout(self.norm2(forward + x))
    return out
```
