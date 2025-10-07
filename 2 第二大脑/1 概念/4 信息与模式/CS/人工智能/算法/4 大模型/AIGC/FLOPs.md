---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-26
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary



# Cues



# Notes

计算资源（FLOPs）怎么算？

训练时的总计算量（FLOPs）约等于：

> **FLOPs ≈ 6 × 模型参数量 × 训练数据量 (tokens)**

例如：

- 一个有 **16** 个参数的模型
- 用 **100** 个 token 的数据进行训练
- 总计算量 = `6 × 16 × 100 = 9,600 FLOPs`
