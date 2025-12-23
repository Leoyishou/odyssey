---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-11
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

1. 是一种特殊的equals，还是得时间复杂度为  O(N)，一个个去比，想加速 O(log N) 需要靠[[HNSW]]或者[[IVF]]，所以如果用[[PostgreSQL]]，需要新建 HNSW 索引


、
**核心特点：**
- **两个独立的编码器**（可以是同一个模型）
- Query 和 Document **分别编码**，互不影响
- 编码后得到固定维度的向量
- 通过向量相似度计算相关性

# Cues



# Notes
