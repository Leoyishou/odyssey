---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  

## 给句子赋予角色
```
[0][0][0][0][0][0][0] [1][1][1][1][1][1]
北 京 的 首 都 是 哪 里  北 京 是 中 国 的 首 都
     ↑问题部分↑              ↑答案部分↑
     
     
```

## 但是只有两种角色，二元

BERT 只有两种 Segment Embedding

python

```python
# BERT 内部只有两个向量
segment_embeddings = {
    0: [0.123, 0.456, 0.789, ...],  # 768维向量，代表"第一部分"
    1: [0.234, 0.567, 0.891, ...]   # 768维向量，代表"第二部分"
}

# 所以只能标记两种类型！
```

这就是为什么我们之前讨论"BERT 只支持 2 个 segment"。

## 可以 0000011111110000011111 这样交替吗？

### 1. **技术上可行**

```python
from transformers import BertTokenizer, BertModel
import torch

tokenizer = BertTokenizer.from_pretrained('bert-base-chinese')

# 手动构造交替的 segment ids
text = "去三亚 请补充信息 飞机 推荐方案 海边"
tokens = tokenizer.tokenize(text)
input_ids = tokenizer.convert_tokens_to_ids(tokens)

# 手动指定 segment_ids (交替)
segment_ids = [0, 0, 0,  # 去三亚
               1, 1, 1, 1,  # 请补充信息
               0, 0,  # 飞机
               1, 1, 1,  # 推荐方案
               0, 0]  # 海边

inputs = {
    'input_ids': torch.tensor([input_ids]),
    'token_type_ids': torch.tensor([segment_ids]),  # 交替的0和1
    'attention_mask': torch.ones(len(input_ids)).unsqueeze(0)
}

model = BertModel.from_pretrained('bert-base-chinese')
outputs = model(**inputs)
print("✅ 可以运行！")
```

### 2. **但这样做有严重问题**

#### ❌ 问题1：违背了 BERT 的预训练假设

BERT 在预训练时学到的 Segment Embedding 语义是：

```
Segment 0 = 第一个完整句子
Segment 1 = 第二个完整句子
```

**BERT 的预训练数据**：

```
[CLS] 北京是中国的首都 [SEP] 它有很多名胜古迹 [SEP]
  0    0  0  0  0  0  0   0     1  1  1  1  1  1  1   1
      ↑----句子A------↑         ↑-----句子B-----↑
```

BERT 学会了：

- Segment 0 和 1 通常是**两个连续的、完整的句子**
- 它们之间有语义关系（比如因果、转折）
- **不会频繁切换**

#### ❌ 问题2：交替使用违反了预训练的模式

python

```python
# 你这样做：
0 0 0 1 1 1 1 0 0 1 1 1 0 0
去三亚 请补充信息 飞机 推荐方案 海边

# BERT 会困惑：
# "为什么 Segment 频繁切换？"
# "这不符合我在预训练时见过的模式！"
```

#### ❌ 问题3：Segment Embedding 的语义会混乱

python

```python
# BERT 预训练时学到的：
Segment 0 的向量代表："这是第一句话的特征"
Segment 1 的向量代表："这是第二句话的特征"

# 你这样用：
"去三亚"   → Segment 0 ✅
"请补充"   → Segment 1 ✅  
"飞机"     → Segment 0 ❌ 等等，为什么又是第一句？
"推荐方案" → Segment 1 ❌ 为什么又是第二句？
"海边"     → Segment 0 ❌ 又变回第一句了？

# BERT：我晕了 😵
```
# 🧩 Cues



# 🪞Notes
