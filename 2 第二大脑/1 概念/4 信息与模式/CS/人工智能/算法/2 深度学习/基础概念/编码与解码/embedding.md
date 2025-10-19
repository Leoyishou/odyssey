---
uid: 34b912d9-2675-4e26-94ca-3add6a143f97
date created: 2025-02-14
date modified: 2025-10-19
---
# Summary
让模型学会把“词语”映射成一个向量，使得语义相似的词在向量空间中距离也近。


# Notes

## 接口与实现

Embedding 是一种接口，[[Word2Vec]]、[[FastText]]、[[BGE]]等都是实现类，榜单是[[MTEB]]

```Java
Embedding（嵌入）
    ├─ Word Embedding（词嵌入）
    │   ├─ Word2Vec ← 具体方法
    │   ├─ GloVe ← 具体方法
    │   ├─ FastText ← 具体方法
    │   └─ BERT/GPT 的词嵌入 ← 具体方法
    │
    ├─ Image Embedding（图像嵌入）
    ├─ User Embedding（用户嵌入）
    └─ Graph Embedding（图嵌入）
```



## 需不需要做向量的归一化

|你的目标|是否归一化|原因|
|---|---|---|
|找最相似的文档|✅ 是|只看方向|
|推荐系统|✅ 是|公平比较|
|语义搜索|✅ 是|标准做法|
|情感强度分析|❌ 否|需要保留强度|
|异常检测|❌ 否|长度也是特征|
|需要加权组合|❌ 否|保留权重|
