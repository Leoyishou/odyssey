---
uid: e19abe79-06da-4224-8030-48e55bda8fa6
date created: 2025-07-19
date modified: 2025-10-20
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

标题: 大模型面试：为什么几乎所有的大模型都采用了 weight decay 呢？｜频次：★

现在大模型训练很少使用dropout，原因是目前预训练阶段训练数据足够多，多样性足够好，没有过拟合的风险，所以不需要加入 dropout 去放置过拟合。加入之后反而因为训练和预测的不一致性影响模型的效果。weight decay 通常也被认为是一种防止过拟合的手段，那为什么几乎所有的大模型都采用了 weight decay IE？
