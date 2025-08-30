---
uid: 818d8771-06b1-4080-adab-4ed17dbd8089
date created: 2025-07-19
date modified: 2025-07-26
---
# Summary

想象一个工厂有 1000 个工人（GPU 核心）：

**PyTorch 方式**：3 个[GPU Kernel 内核](GPU%20Kernel%20内核.md)
1. 所有 1000 个工人一起完成步骤 1（所有产品都过一遍）
2. 等待，搬运中间产品到仓库
3. 所有 1000 个工人一起完成步骤 2
4. 等待，搬运...
5. 重复直到完成

**Triton 方式**：合并为 1 个[GPU Kernel 内核](GPU%20Kernel%20内核.md)
1. 把 1000 个工人分成 6 组
2. 每组负责一个产品的**全部步骤**
3. 不需要中间搬运，每组在自己工位上完成所有工序


总的来说，Triton实现通过kernel融合技术大幅减少了*内存访问*和*kernel调用次数*，在大规模深度学习训练中能带来显著的性能提升。

Triton /traiten/ 允许开发者使用类似 Python 的语法编写自定义 [GPU Kernel 内核](GPU%20Kernel%20内核.md)，同时能达到与专业 [[CUDA]] 代码相当的性能，而无需深入了解底层的 CUDA 细节。


**Triton**: 在计算领域，Triton 是一种开源的 GPU 编程语言和编译器。它的设计旨在简化为人工智能和深度学习编写高性能 GPU 代码的过程。

**Parallelism (并行性/并行计算)**: 指的是一种计算类型，其中许多计算或进程可以同时进行。它通过将一个大的计算任务分解成可以同时在多个处理器或核心上执行的小型子任务，从而缩短总的计算时间。并行性可以分为多种形式，例如位级并行、指令级并行、数据并行和任务并行。在计算机体系结构中，并行计算已成为主流范式，尤其是在多核处理器中。

# Notes

已[[RMSNorm]]的实现方式为例：

## 1. **PyTorch原生实现** vs **Triton GPU实现**

### PyTorch实现（第一种）

```python
x_sq = x.pow(2)
mean_sq = x_sq.mean(-1, keepdim=True)
r_rms = torch.rsqrt(mean_sq + eps)
y = x * r_rms * g
```

### Triton实现（第二种）

```python
# 准备数据形状
x_reshaped = x.reshape(-1, H)
y_reshaped = y.reshape(-1, H)

# 调用GPU kernel
rmsnorm_forward_kernel[(num_rows,)](
    x_reshaped, g, y_reshaped, ...
)
```

## 2. **关键区别**

|特性|PyTorch实现|Triton实现|
|---|---|---|
|**执行位置**|CPU/GPU自动选择|专门的GPU kernel|
|**内存访问**|多次读写显存|优化的内存访问模式|
|**中间变量**|创建x_sq, mean_sq等|kernel内部计算，无中间张量|
|**并行策略**|PyTorch自动并行|手动控制BLOCK_SIZE|
|**性能**|较慢，多次kernel调用|快速，单次kernel完成|

## GPU 的硬件结构

一个 GPU 包含成千上万个 CUDA 核心，比如：

- RTX 3090 有 10,496 个 CUDA 核心
- A100 有 6,912 个 CUDA 核心

这些核心被组织成多个 SM（Streaming Multiprocessor）。

## PyTorch 的执行方式

```python
x_sq = x.pow(2)  # 调用 CUDA kernel 1：使用所有可用核心
mean_sq = x_sq.mean(-1, keepdim=True)  # 调用 CUDA kernel 2：使用所有可用核心
r_rms = torch.rsqrt(mean_sq + eps)  # 调用 CUDA kernel 3：使用所有可用核心
y = x * r_rms * g  # 调用 CUDA kernel 4：使用所有可用核心
```

每个操作都是一个独立的 kernel：

- **Kernel 1 (pow)**：所有核心并行计算所有元素的平方
- **Kernel 2 (mean)**：所有核心并行计算各行的均值
- **Kernel 3 (rsqrt)**：所有核心并行计算倒数平方根
- **Kernel 4 (mul)**：所有核心并行执行乘法

问题是：每个 kernel 之间需要**全局同步**和**内存读写**！

## Triton 的执行方式

```python
rmsnorm_forward_kernel[(6,)](...)  # 一个融合的 kernel
```

这里的 `(6,)` 不是指 6 个 GPU，而是指将工作分成 6 个**线程块**（thread blocks）：

```Java
GPU 上的所有核心被组织成 6 个工作组：
- 线程块 0：处理第 0 行，使用部分核心
- 线程块 1：处理第 1 行，使用部分核心
- 线程块 2：处理第 2 行，使用部分核心
- ... 同时执行 ...
```

## 关键区别：内存访问模式

### PyTorch（多个 kernel）

```Java
时间线：
t1: [所有核心] 读取全部数据 → 计算 pow → 写回全部结果
t2: [所有核心] 读取全部数据 → 计算 mean → 写回全部结果  
t3: [所有核心] 读取全部数据 → 计算 rsqrt → 写回全部结果
t4: [所有核心] 读取全部数据 → 计算 mul → 写回全部结果
```

### Triton（一个融合 kernel）

```Java
时间线：
t1: [所有核心分成多组] 每组读取一行 → 在寄存器中完成所有计算 → 写回结果
```

## 用更准确的比喻

想象一个工厂有 1000 个工人（GPU 核心）：

**PyTorch 方式**：

1. 所有 1000 个工人一起完成步骤 1（所有产品都过一遍）
2. 等待，搬运中间产品到仓库
3. 所有 1000 个工人一起完成步骤 2
4. 等待，搬运...
5. 重复直到完成

**Triton 方式**：

1. 把 1000 个工人分成 6 组
2. 每组负责一个产品的**全部步骤**
3. 不需要中间搬运，每组在自己工位上完成所有工序

## 实际性能差异来源

1. **内存带宽**：Triton 减少了 3/4 的内存访问
2. **缓存利用**：数据在寄存器/L1 缓存中完成计算
3. **延迟隐藏**：不需要等待内存读写完成

所以 Triton 快不是因为用了更多 GPU，而是因为**更高效地使用了同一个 GPU**！
