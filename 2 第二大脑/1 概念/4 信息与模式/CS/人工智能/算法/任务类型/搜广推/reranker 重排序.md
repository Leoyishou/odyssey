---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-11-01
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

**重排序是用一个「新模型」重新计算 query 和 doc 的相关性分数**

## 在搜索中的位置

用户查询
→ Embedding检索 + [[ElasticSearch、ES]]（召回阶段）
→ **Reranker（精排阶段）**
→ Top结果

## 原理
```python
# 拼接后一起编码
input_text = "[CLS] query [SEP] doc [SEP]"
score = reranker(input_text)  # 直接输出相关性分数

# 优点：query 和 doc 充分交互，精度更高
# 缺点：必须在线计算，慢（只能处理几十到几百个候选）
```
# Cues

[[双塔模型（Bi-Encoder）、粗召回（粗排）]]

# Notes


## 一、与双塔模型（Bi-Encoder）的区别

**双塔模型（用于召回）**：

```python
# 分别编码，可以离线计算
query_vector = encoder(query)          # [1, 768]
doc_vector = encoder(doc)              # [1, 768]
score = cosine_similarity(query_vector, doc_vector)  # 点积

# 优点：速度快，支持百万级检索
# 缺点：query 和 doc 独立编码，交互不足
```

**Cross-Encoder（用于重排序）**：

```python
# 拼接后一起编码
input_text = "[CLS] query [SEP] doc [SEP]"
score = reranker(input_text)  # 直接输出相关性分数

# 优点：query 和 doc 充分交互，精度更高
# 缺点：必须在线计算，慢（只能处理几十到几百个候选）
```



## 二、CLS 和 SEP 是什么?

这是 **BERT 类模型的特殊标记符号**,可以把它们理解为"标点符号",帮助模型理解文本结构:

**`[CLS]`** - Classification Token (分类标记)，> 就像看一篇文章,标题写着"[总结]",你知道这里会给出整体结论
- **位置**: 永远放在**最开头**
- **作用**: 像一个"总结者",模型会把整段文本的**综合信息**都汇聚到这里
- **输出**: Reranker 最后就看这个位置的输出来判断相关性分数

**`[SEP]`** - Separator Token (分隔符) > 就像聊天时用 "---" 分隔不同话题
- **位置**: 放在**不同部分之间**
- **作用**: 告诉模型"这是两个不同的东西,别混淆"
- **类似**: 中文的顿号、分号

---

## 实际例子

假设你搜索 **"北京天气"**,候选文档是 **"今天北京多云转晴"**

```python
# Reranker 实际看到的输入:
"[CLS] 北京天气 [SEP] 今天北京多云转晴 [SEP]"

# 模型内部处理:
# 1. [CLS] 的位置会综合理解整个句子
# 2. [SEP] 告诉模型: 前面是query,后面是文档
# 3. 最后输出 [CLS] 位置的分数 → 比如 0.92 (很相关!)
```

---

## 为什么需要这些标记?

|场景|不用标记|用了标记|
|---|---|---|
|**区分部分**|"北京天气今天北京多云" <br> 😵 模型分不清哪是query|"[CLS]北京天气[SEP]今天..." <br> ✅ 清楚知道结构|
|**获取分数**|不知道看哪个位置的输出|直接看[CLS]的输出|

---

## 核心要点

- **[CLS]** = 开头的"总结位置",输出最终分数
- **[SEP]** = 中间的"分隔符",区分 query 和 doc
- 这是 BERT 训练时就定好的**约定俗成**,就像 HTTP 协议里的 `GET`、`POST` 一样
