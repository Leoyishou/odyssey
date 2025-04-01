---
draw:
title: PyTorch
date created: 2025-02-14
date modified: 2025-03-28
---

用 pytorch 手写[多层感知机 MLP](多层感知机%20MLP.md)    https://colab.research.google.com/drive/1l1GlZ36sftvNzdQ0YmDT3iADzb8igCSo#scrollTo=irUPC7YOZ7zO

|                     | **方法/类 (Method/Class)**                   | **描述 (Description from Tutorial Context)**      |
| ------------------- | ----------------------------------------- | ----------------------------------------------- |
|                     |                                           |                                                 |
| 算力                  | `.to(device)`                             | 将张量或模型移动到指定的设备（CPU 或 GPU）。|
|                     | `torch.device()`                          | 创建表示计算设备（CPU/CUDA）的对象。|
|                     | `torch.cuda.is_available()`               | 检查 CUDA (GPU) 是否可用。|
|                     |                                           |                                                 |
| [[数据集]]<br>         | `torch.utils.data.Dataset`                | 表示数据集的抽象基类。|
|                     | `torch.utils.data.DataLoader`             | 类似 Java 的分片Lists.partition(list, 2);            |
|                     |                                           |                                                 |
| 模型                  | `torch.nn.Module`                         | nn 的意思是 **Neural Network**（神经网络）,构建神经网络模型和层的基类。|
|                     | `model.parameters()`                      | 获取模型所有可学习参数的迭代器，通常传递给优化器。|
|                     |                                           |                                                 |
| 训练                  | `model.train()`                           | 将模型设置为训练模式（启用 Dropout, BatchNorm 更新等）。|
|                     |                                           |                                                 |
| 前向传播                | `forward()` (in `nn.Module`)              | 定义模型的前向传播逻辑，当调用模型实例时自动执行。|
| [[损失函数]]            | `torch.nn.MSELoss`                        | 计算均方误差损失的类。|
|                     | `torch.nn.functional` (as `F`)            | 包含无状态的函数式接口，如 `F.mse_loss` (理论中提及)。|
|                     | `nn.CrossEntropyLoss` / `F.cross_entropy` | 常用的分类损失 (理论中提及)。|
| [反向传播算法](反向传播算法.md) | `loss.backward()`                         | 计算损失相对于需要梯度的参数的梯度。|
| [梯度下降](梯度下降.md)     | `torch.optim.SGD`                         | 随机梯度下降优化器类。|
|                     | `torch.optim.SGD`<br>`torch.optim.adm`    | 另一种常用的优化器 (理论中提及)。|
| 参数更新                | `optimizer.step()`                        | 根据计算出的梯度更新模型参数。|
|                     |                                           |                                                 |
| 评估                  | `model.eval()`                            | 将模型设置为评估模式（禁用 Dropout, BatchNorm 不更新等）。|
|                     | `torch.no_grad()`                         | 上下文管理器，在其内部禁用梯度计算，用于评估或推理。|
|                     |                                           |                                                 |
|                     |                                           |                                                 |
| [[张量]]              | `torch.tensor()`                          | 从 Python 列表或 NumPy 数组创建张量的主要方式；也用于创建标量和训练数据。|
|                     | `torch.randn()`                           | 创建服从标准正态分布的随机数张量，常用于初始化参数。|
|                     | `torch.zeros()` / `torch.ones()`          | 创建全零或全一张量。|
|                     | `.shape` / `.size()`                      | 获取张量的形状（维度）。|
|                     | `requires_grad=True`                      | 创建张量时指定需要计算梯度。|
|                     | `.grad`                                   | 张量的属性，存储 `backward()` 后计算出的梯度。|
|                     | `.item()`                                 | 从只包含一个元素的张量中获取其 Python 数值。|
|                     | `.mean()`                                 | 计算张量元素的平均值，用于手动实现 MSE。|
|                     | `**`                                      |  幂运算                                            |
|                     | `torch.arange()`                          | 创建一个包含等差序列数值的一维张量。|
|                     | `torch.utils.data.TensorDataset`          | 快速包装张量数据成为 Dataset 的便捷类。|
|                     | `torch.nn.Parameter()`                    | 将张量包装为模型的可学习参数。|
|                     | `torch.equal()`                           | 检查两个张量是否逐元素完全相等。|
|                     | `torch.allclose()`                        | 检查两个张量是否在容忍误差内逐元素接近（适用于浮点数比较）。|
|                     | `torch.matmul()` / `@`                    | 矩阵乘法 (理论中提及)。|
|                     | `.view()` / `.reshape()`                  | 改变张量形状 (理论中提及)。|
|                     | `.add_()`                                 | 原地加法操作 (理论中提及作为原地操作示例)。|
|                     |                                           |                                                 |
|                     |                                           |                                                 |
|                     |                                           |                                                 |
|                     |                                           |                                                 |
|                     | `nn.Linear` / `nn.Conv2d`                 | PyTorch 提供的预定义层 (理论中提及)。|
|                     |                                           |                                                 |
