---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-05
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 🧩 Summary  

## Weight Decay 的数学原理

传统的 L2 正则化：
```Java
损失函数：
Loss = 原始Loss + λ * Σ(weight²)
```
效果：权重倾向于变小，模型更"平滑"

Weight Decay 在优化器中的实现：
```Java

# 梯度更新时
weight = weight - lr * gradient - lr * weight_decay * weight
#                 ↑ 常规梯度      ↑ 额外的衰减项

```

# 💡 Cues



# Notes
