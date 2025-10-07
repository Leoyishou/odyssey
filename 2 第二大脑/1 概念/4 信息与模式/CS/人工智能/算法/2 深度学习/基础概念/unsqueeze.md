---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-20
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary



# Cues



# Notes

这个操作是**给mask添加维度**，让它能与多头注意力的Q、K、V张量进行**广播（broadcasting）**！

## 🎯 **背景：需要匹配的张量形状**

```python
# 多头注意力中的张量形状：
Q.shape = [batch_size, num_heads, seq_len, d_k]     # [2, 4, 10, 16]
K.shape = [batch_size, num_heads, seq_len, d_k]     # [2, 4, 10, 16]  
V.shape = [batch_size, num_heads, seq_len, d_k]     # [2, 4, 10, 16]

# 注意力分数的形状：
scores.shape = [batch_size, num_heads, seq_len, seq_len]  # [2, 4, 10, 10]
```

## 📊 **mask的维度扩展过程**

### **原始mask（操作前）：**
```python
# 因果mask（下三角矩阵）
mask = torch.triu(torch.ones(seq_len, seq_len) * float("-inf"), diagonal=1)

原始形状: [seq_len, seq_len] = [10, 10]

具体数值:
mask = [[  0, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
        [  0,    0, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
        [  0,    0,    0, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
        [  0,    0,    0,    0, -inf, -inf, -inf, -inf, -inf, -inf],
        [  0,    0,    0,    0,    0, -inf, -inf, -inf, -inf, -inf],
        [  0,    0,    0,    0,    0,    0, -inf, -inf, -inf, -inf],
        [  0,    0,    0,    0,    0,    0,    0, -inf, -inf, -inf],
        [  0,    0,    0,    0,    0,    0,    0,    0, -inf, -inf],
        [  0,    0,    0,    0,    0,    0,    0,    0,    0, -inf],
        [  0,    0,    0,    0,    0,    0,    0,    0,    0,    0]]
```

### **第1次 unsqueeze(0)：添加batch维度**
```python
mask = mask.unsqueeze(0)

新形状: [1, seq_len, seq_len] = [1, 10, 10]

# 现在mask变成：
mask = [[[  0, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
         [  0,    0, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
         [  0,    0,    0, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
         ...
         [  0,    0,    0,    0,    0,    0,    0,    0,    0,    0]]]
```

### **第2次 unsqueeze(0)：添加head维度**
```python
mask = mask.unsqueeze(0)

最终形状: [1, 1, seq_len, seq_len] = [1, 1, 10, 10]

# 现在mask变成4维张量：
mask = [[[[  0, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
          [  0,    0, -inf, -inf, -inf, -inf, -inf, -inf, -inf, -inf],  
          [  0,    0,    0, -inf, -inf, -inf, -inf, -inf, -inf, -inf],
          ...
          [  0,    0,    0,    0,    0,    0,    0,    0,    0,    0]]]]
```

## 🔄 **广播机制的工作原理**

### **张量形状对比：**
```python
scores.shape = [2, 4, 10, 10]  # [batch, heads, seq, seq]
mask.shape   = [1, 1, 10, 10]  # [1,     1,     seq, seq]
```

### **广播规则：**
```python
# PyTorch会自动扩展较小的维度
scores + mask 时：

mask[1, 1, 10, 10] 自动广播为 → mask[2, 4, 10, 10]

# 相当于：
expanded_mask = mask.expand(2, 4, 10, 10)
```

## 🎭 **具体的广播过程**

### **广播前：**
```python
# 每个batch、每个head都需要相同的因果mask
batch_0_head_0: 需要 [10, 10] 的因果mask
batch_0_head_1: 需要 [10, 10] 的因果mask  
batch_0_head_2: 需要 [10, 10] 的因果mask
batch_0_head_3: 需要 [10, 10] 的因果mask
batch_1_head_0: 需要 [10, 10] 的因果mask
...
```

### **广播后：**
```python
# 一个 [1, 1, 10, 10] 的mask被复制到所有位置
final_mask.shape = [2, 4, 10, 10]

# 实际上是：
final_mask[0, 0, :, :] = 原始mask  # batch0_head0
final_mask[0, 1, :, :] = 原始mask  # batch0_head1  
final_mask[0, 2, :, :] = 原始mask  # batch0_head2
final_mask[0, 3, :, :] = 原始mask  # batch0_head3
final_mask[1, 0, :, :] = 原始mask  # batch1_head0
final_mask[1, 1, :, :] = 原始mask  # batch1_head1
final_mask[1, 2, :, :] = 原始mask  # batch1_head2
final_mask[1, 3, :, :] = 原始mask  # batch1_head3
```

## 💡 **为什么这样做？**

### **内存效率：**
```python
# 不需要实际创建 [2, 4, 10, 10] 的大tensor
# 只需 [1, 1, 10, 10]，让PyTorch自动广播
```

### **计算便利：**
```python
# 现在可以直接相加：
masked_scores = scores + mask  # 形状完全匹配！
```

## 🎯 **总结**

这两次`unsqueeze(0)`操作就是：

**把2D的因果mask → 变成4D的广播兼容mask**

让同一个mask pattern能够应用到所有batch和所有head上，实现"一次定义，到处使用"！🎯
