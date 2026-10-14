---
uid: 122511b1-570f-4c55-84c8-1af69b20bca5
date created: 2025-07-19
date modified: 2025-07-26
---
# Summary

**FLOPs（Floating‑Point Operations）是什么？**
- **定义**：一次 **浮点运算**（加、减、乘、除、乘加等）计为 1 FLOP。

# Cues



# Notes

计算资源（FLOPs）怎么算？

训练时的总计算量（FLOPs）约等于：

> **FLOPs ≈ 6 × 模型参数量 × 训练数据量 (tokens)**

例如：
- 一个有 **16** 个参数的模型
- 用 **100** 个 token 的数据进行训练
- 6 次：**前向传播的 2 次** + **反向传播的 4 次**
- 总计算量 = `6 × 16 × 100 = 9,600 FLOPs`
