---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-13
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

**Batch size** 是指**一次训练中同时处理的样本数量**。

想象你是一位老师批改作业：

```Java
方式1：一本一本改（batch_size = 1）
😫 效率低，但能仔细看每本

方式2：10本一起改（batch_size = 10）  
😊 效率适中，能发现共同问题

方式3：100本一起改（batch_size = 100）
😰 效率高，但可能忽略个体差异
```


# Cues



# Notes

## 直观理解

## 具体例子

### 图像分类任务

```python
# 假设有1000张猫狗图片
total_images = 1000

# batch_size = 32 意味着：
# - 每次取32张图片一起处理
# - 需要 1000/32 ≈ 32次 才能看完所有图片

batch_size = 32
num_batches = total_images // batch_size  # 31个完整批次
```

### 代码实现

```python
import torch
from torch.utils.data import DataLoader

# 创建数据集
dataset = YourDataset()  # 假设有1000个样本

# 设置batch_size
train_loader = DataLoader(dataset, batch_size=32, shuffle=True)

# 训练循环
for batch_idx, (images, labels) in enumerate(train_loader):
    print(f"批次 {batch_idx}:")
    print(f"  图片形状: {images.shape}")  # [32, 3, 224, 224]
    print(f"  标签形状: {labels.shape}")  # [32]
    
    # 这32张图片会一起：
    # 1. 前向传播
    # 2. 计算损失
    # 3. 反向传播
    # 4. 更新参数
```

## 数据形状变化

```python
# 单个样本
image = torch.randn(3, 224, 224)  # [通道, 高, 宽]

# 批处理后
batch = torch.randn(32, 3, 224, 224)  # [batch_size, 通道, 高, 宽]
#                   ↑
#              新增的批次维度
```

## 不同Batch Size的影响

### Batch Size = 1（SGD）

```python
for i in range(1000):
    x, y = dataset[i]  # 取一个样本
    loss = model(x) - y
    loss.backward()    # 每个样本都更新
    optimizer.step()
    
# 优点：更新频繁，能处理在线数据
# 缺点：噪声大，训练不稳定，GPU利用率低
```

### Batch Size = 32（Mini-batch）

```python
for batch in dataloader:  # 每批32个
    x, y = batch
    loss = model(x) - y
    loss.backward()       # 32个样本的平均梯度
    optimizer.step()
    
# 优点：平衡了效率和稳定性
# 缺点：需要调参找合适的值
```

### Batch Size = 1000（Full Batch）

```python
x, y = dataset[:]  # 一次取所有数据
loss = model(x) - y
loss.backward()    # 所有样本的精确梯度
optimizer.step()

# 优点：梯度准确，收敛稳定
# 缺点：内存需求大，更新慢，容易陷入局部最优
```

## 实际例子：训练过程

```python
# 假设训练MNIST手写数字识别
epochs = 10
batch_size = 64
total_samples = 60000  # MNIST训练集大小

for epoch in range(epochs):
    for batch_idx, (images, labels) in enumerate(train_loader):
        # images: [64, 1, 28, 28] - 64张28x28的灰度图
        # labels: [64] - 64个数字标签
        
        # 前向传播
        outputs = model(images)  # [64, 10] - 64个样本的10类预测
        
        # 计算损失（64个样本的平均损失）
        loss = criterion(outputs, labels)
        
        # 反向传播
        loss.backward()
        
        # 更新参数
        optimizer.step()
        optimizer.zero_grad()
        
        if batch_idx % 100 == 0:
            print(f"第{epoch}轮，第{batch_idx}批，损失: {loss.item():.4f}")
```

## Batch Size对内存的影响

```python
# 模型输入: [batch_size, 3, 224, 224]
# 内存使用 ≈ batch_size × 3 × 224 × 224 × 4字节

batch_size = 1:    约 0.6 MB
batch_size = 32:   约 19 MB  
batch_size = 128:  约 76 MB
batch_size = 512:  约 304 MB

# GPU显存不够时会报错：
# RuntimeError: CUDA out of memory
```

## 常见Batch Size选择

|任务类型|典型Batch Size|原因|
|---|---|---|
|图像分类|32-256|平衡效率和性能|
|语言模型|8-64|序列长，显存限制|
|BERT微调|16-32|模型大，显存限制|
|GAN训练|64-128|需要足够样本多样性|
|强化学习|32-512|需要足够探索|

## 动态Batch Size

```python
# 学习率预热时使用小batch
warmup_batch_size = 16

# 正常训练时使用大batch  
normal_batch_size = 128

# 梯度累积实现大batch效果
accumulation_steps = 4
for i, (x, y) in enumerate(dataloader):
    loss = model(x, y) / accumulation_steps
    loss.backward()
    
    if (i + 1) % accumulation_steps == 0:
        optimizer.step()
        optimizer.zero_grad()
```

## 总结

Batch Size就是：

- **定义**：一次处理的样本数量
- **作用**：平衡训练效率和稳定性
- **权衡**：大batch稳定但慢，小batch快但不稳定
- **限制**：主要受显存/内存限制
- **技巧**：可以通过梯度累积等方法灵活调整

就像批改作业，一次改几本需要根据实际情况权衡！
