---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-09
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 🧩 Summary  
## RRF（Reciprocal Rank Fusion）算法详解

### **核心问题：如何融合不同来源的排序结果？**

text

```Java
场景：混合检索
├─ BM25检索结果：    [Doc5, Doc2, Doc9, Doc1, Doc7]
└─ 向量检索结果：    [Doc2, Doc5, Doc1, Doc12, Doc8]

问题：怎么合并成一个排序？
```

### **❌ 错误方案：直接加权分数**

python

```python
# 问题：不同算法的分数量纲不同

bm25_scores = {
    "Doc5": 15.2,  # BM25分数范围：0-20
    "Doc2": 12.8,
    "Doc1": 8.5
}

vector_scores = {
    "Doc2": 0.92,  # 余弦相似度：0-1
    "Doc5": 0.88,
    "Doc1": 0.85
}

# 直接相加没有意义！
final_score = bm25_score + vector_score  # 15.2 + 0.88 = ???
```

### **✅ RRF方案：只看排名，不看分数**

#### **RRF公式**

text

```Java
RRF_score(d) = Σ  1 / (k + rank_i(d))
              i∈所有检索系统

参数：
- k: 常数（通常取60），防止除零并平滑排名差异
- rank_i(d): 文档d在第i个检索系统中的排名
```

#### **完整**


# 💡 Cues



# Notes
