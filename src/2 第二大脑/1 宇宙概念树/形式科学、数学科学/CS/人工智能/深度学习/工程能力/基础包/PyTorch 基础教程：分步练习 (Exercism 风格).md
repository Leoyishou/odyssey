---
draw:
title: PyTorch 基础教程：分步练习 (Exercism 风格)
date created: 2025-03-27
date modified: 2025-04-22
---

## PyTorch 基础教程：分步练习 (Exercism 风格)

本教程面向有一定 Python 编程基础的读者，通过一系列循序渐进的练习，在大约 20 小时内掌握 PyTorch 的基础概念和编程方法。每一步练习围绕一个核心概念，包含理论介绍、编程任务以及用于验证结果的简单测试。**请按照顺序逐步完成每个练习**，可以在 Google [[colab]] 等环境运行。

> **使用说明：** 建议在 Colab 中新建笔记本，将每个步骤的代码和文本分别复制运行。Colab 通常已经安装了 PyTorch，如未安装可先执行 `!pip install torch`。为确保GPU可用，可在 Colab “修改→笔记本设置”中将硬件加速器设为GPU。

### 第1步：张量与基本运算 (Tensor Operations)

**理论介绍：** PyTorch 的张量（Tensor）是其基本数据结构，类似于 NumPy 的多维数组，但能够利用 GPU 加速计算，并支持自动微分功能。张量可以是标量（0维）、向量（1维）、矩阵（2维）或更高维数组。创建张量的方式多种多样，例如直接从Python列表创建，或使用内置函数如 `torch.zeros()`、`torch.ones()`、`torch.randn()` 随机初始化等。张量拥有**形状（shape）**和**数据类型（dtype）**等属性，可通过 `x.shape` 或 `x.size()` 获取形状。PyTorch 提供了丰富的张量运算，包括逐元素算术运算（加减乘除）、矩阵乘法（如 `torch.matmul` 或 `@` 符号）、索引和切片操作、形状变换（如 `view`/`reshape`）等。这些操作大部分与 NumPy 用法类似，但计算可以在GPU上执行从而显著加速。需要注意的是，PyTorch的操作会产生新的张量结果，除非使用带下划线的方法表示原地操作（如 `x.add_()` 是在原张量上加）。本练习将熟悉张量的创建和基本运算。

**编程任务：** 创建一个一维张量，包含元素 `[0, 1, 2, 3, 4]`。对这个张量执行一些基本运算：将其每个元素乘以2再加上1。预期结果是得到一个新张量，其元素为 `[1, 3, 5, 7, 9]`。请完成以下代码来实现这些操作。

```python
# 导入 PyTorch 库
import torch

# 创建初始张量 [0, 1, 2, 3, 4]
x = torch.tensor([0, 1, 2, 3, 4])
# 对张量执行运算: 元素乘2再加1
y = x * 2 + 1

print(y)  # 打印结果张量
```

**验证：** 运行上述代码后，程序会打印张量 `y` 的值。我们可以使用断言检查结果是否符合预期。

```python
expected = torch.tensor([1, 3, 5, 7, 9])
assert torch.equal(y, expected), "张量运算结果不正确，请检查代码实现！"
print("第1步通过：张量基本运算正确。")
```

如果断言未触发且打印出“第1步通过”，说明您正确地完成了张量的创建和运算。本练习中，我们验证了 PyTorch 张量的基本操作功能。

---

### 第2步：自动微分 (Autograd)

**理论介绍：** 自动微分是 PyTorch 的强大功能，使深度学习的反向传播计算梯度变得简单。PyTorch 会在你对张量进行操作时，动态地构建计算图（computational graph）并跟踪这些操作，以便稍后进行梯度计算 ([PyTorch’s Dynamic Graphs (Autograd) | by Zach Wolpe | Medium](https://zachcolinwolpe.medium.com/pytorchs-dynamic-graphs-autograd-96ecb3efc158#:~:text=,connections%20between%20parameters%20and%20gradients))。每个张量都有一个 `requires_grad` 属性，表示是否需要对其求导数。默认情况下，创建的张量 `requires_grad=False`（不追踪梯度）。如果将 `x.requires_grad=True`，则对 `x` 的任何操作，PyTorch 都会记录计算图。之后，对结果调用 `.backward()` 方法时，Autograd 会自动计算所有需要求导的张量的梯度，并存储在它们的 `.grad` 属性中。**梯度（gradient）**指的是某个标量输出对张量的偏导数，在神经网络训练中对应误差对参数的导数。PyTorch 使用动态计算图（dynamic computation graph），意味着计算图在每个前向运算时即时构建，因此可以支持变化的网络结构和复杂控制流，而不像某些框架需要事先定义静态计算图 ([PyTorch’s Dynamic Graphs (Autograd) | by Zach Wolpe | Medium](https://zachcolinwolpe.medium.com/pytorchs-dynamic-graphs-autograd-96ecb3efc158#:~:text=Dynamicity%20%26%20Control%20Flow))。计算梯度后，如果不再需要构建的计算图，PyTorch会释放它以节省内存（每次调用 `.backward()` 默认都会释放计算图，除非指定保留）。在使用自动微分时要注意：梯度是累积的，也就是每次调用 `.backward()` 时，会将梯度加入到 `.grad` 属性中而不是覆盖，这就需要我们在适当的时候手动将梯度清零（后续训练环节会讨论）。本练习将通过一个简单示例体验 Autograd 计算梯度。

**编程任务：** 我们将对一个简单函数 f(x)=x2f(x) = x^2 求导。具体地，创建一个标量张量 x=2.0x = 2.0（需要梯度），计算 y=f(x)=x2y = f(x) = x^2。然后调用 `y.backward()` 让 PyTorch 计算梯度。预期结果是 dydx=2x\frac{dy}{dx} = 2x，在 x=2.0x=2.0 处梯度应为 4。请完成以下代码实现这个过程，并获取 xx 的梯度值。

```python
# 创建需要求导的张量 x
x = torch.tensor(2.0, requires_grad=True)
# 计算 y = x^2
y = x ** 2
# 反向传播计算梯度
y.backward()
# 获取 x 的梯度
grad_x = x.grad

print(grad_x)  # 打印梯度
```

**验证：** 检查计算得到的梯度是否正确。对于 x=2.0x=2.0，理论梯度应为 4。我们使用断言比较结果：

```python
assert x.grad.item() == 4.0, f"计算得到的梯度为{x.grad.item()}，不等于预期的 4.0！"
print("第2步通过：Autograd 梯度计算正确。")
```

当输出显示“第2步通过”时，表示您成功利用 PyTorch 的自动微分机制计算出了正确的梯度值。在实际训练中，Autograd 会用于计算模型参数对损失的梯度，从而用于更新参数。

---

### 第3步：神经网络模块 (nn.Module) 与前向传播 (forward)

**理论介绍：** PyTorch 提供了 `torch.nn.Module` 基类，用于构建神经网络模型和层。通过继承 `nn.Module`，我们可以定义自己的模型或层，包括其参数和前向传播逻辑。每个 `nn.Module` 的子类都应重写 `forward()` 方法，定义当该模块接收输入时如何计算输出 ([Understanding the Forward Function Output in PyTorch - GeeksforGeeks](https://www.geeksforgeeks.org/understanding-the-forward-function-output-in-pytorch/#:~:text=The%20forward,and%20the%20task%20at%20hand))。在 `__init__` 方法中，我们通常会初始化模型需要的**参数（parameters）**和子模块。例如，一个线性层（全连接层）会包含可学习的权重和偏置参数。我们可以利用 `torch.nn.Parameter` 将张量注册为参数，这样在调用 `.backward()` 时这些参数的 `.grad` 会被自动计算。`nn.Module` 的另一个便利之处是，它能够自动将其中的参数注册到模型中，使用 `model.parameters()` 可以方便地获取所有参数以用于优化器。**前向传播（forward propagation）**是指将输入经过模型各层计算出输出的过程，对于PyTorch模型来说，就是调用 `model(input)` 时实际执行的 `forward()` 方法计算。注意，在继承 `nn.Module` 时，不需要显式调用 `forward` 方法；只需像函数一样调用模型实例，如 `output = model(x)`，PyTorch会自动调用其内部的 `forward`。`nn.Module` 还可以包含子模块（即其他模块），这有助于搭建复杂的网络。PyTorch已提供许多现成的模块，如线性层 `nn.Linear`、卷积层 `nn.Conv2d` 等，这些模块内部都继承自 `nn.Module` 并实现了自己的 `forward`。例如，`nn.Linear(in_features, out_features)` 已封装了权重和偏置，并在前向中执行线性变换。但为了加深理解，我们将在练习中亲自实现一个简化的线性模型模块。

**编程任务：** 实现一个自定义的线性回归模型 `LinearRegression`，继承自 `nn.Module`。这个模型只有一个输入特征和一个输出（相当于 y=w∗x+by = w * x + b 的形式）。在构造函数 `__init__` 中，定义两个可学习参数：权重 `weight` 和偏置 `bias`。可以使用 `nn.Parameter()` 将普通张量包裹为参数。我们将 `weight` 和 `bias` 初始化为一些随机数（具体值无所谓）。然后，在 `forward(self, x)` 方法中，实现前向计算：返回 `weight * x + bias` 作为模型的预测输出。完成模型的实现后，我们会进行一个简单测试：手动设置参数值并验证前向计算结果是否正确。

```python
import torch.nn as nn

class LinearRegression(nn.Module):
    def __init__(self):
        super().__init__()
        # 定义模型的参数 weight 和 bias，并初始化
        self.weight = nn.Parameter(torch.randn(()))   # 标量参数
        self.bias = nn.Parameter(torch.randn(()))     # 标量参数

    def forward(self, x):
        # 实现前向传播公式: y = weight * x + bias
        return self.weight * x + self.bias

# 测试 LinearRegression 模型的前向传播
model = LinearRegression()
# 为了测试，手动设置参数值
model.weight.data = torch.tensor(2.0)
model.bias.data  = torch.tensor(3.0)
# 准备测试输入张量
x_test = torch.tensor([1.0, 2.0, 3.0])
# 计算模型输出
y_pred = model(x_test)
print(y_pred)
```

**验证：** 在上述测试中，我们将模型参数设置为 w=2.0,b=3.0w=2.0, b=3.0，对于输入[1.0,2.0,3.0][1.0, 2.0, 3.0]，模型预测输出应为 y=2∗x+3y = 2*x + 3，即[5.0,7.0,9.0][5.0, 7.0, 9.0]。以下断言将验证模型输出是否符合预期：

```python
expected = torch.tensor([5.0, 7.0, 9.0])
assert torch.allclose(y_pred, expected), "LinearRegression 模型的前向计算不正确！"
print("第3步通过：nn.Module 模型实现和前向传播正确。")
```

当输出显示“第3步通过”时，说明您成功实现了自定义的 `nn.Module` 并正确完成了前向传播。在实践中，我们通常不会手动设置参数为特定值来测试，而是将模型用于训练。然而通过此练习，您了解了如何定义模型的参数和前向计算。接下来，我们将利用损失函数来度量模型输出与目标的差距。

---

### 第4步：损失函数 (Loss Functions)

**理论介绍：** **损失函数（loss function）**用于衡量模型预测结果与真实目标值之间的差异，是模型训练过程中指导参数调整的依据。不同任务会使用不同的损失函数：回归问题常用均方误差 (Mean Squared Error, **MSE**)，分类问题常用交叉熵损失 (Cross-Entropy loss) 等。PyTorch 在 `torch.nn` 模块中提供了多种内置损失函数类（如 `nn.MSELoss`, `nn.CrossEntropyLoss` 等），也可使用 `torch.nn.functional` 中的函数式接口（如 `F.mse_loss`, `F.cross_entropy`）。以 MSE 为例，它计算输出和目标之差的平方的平均值，其公式为 MSE=1n∑i=1n(ypred,i−ytrue,i)2\text{MSE} = \frac{1}{n}\sum_{i=1}^{n}(y_{\text{pred},i} - y_{\text{true},i})^2。在 PyTorch 中，`nn.MSELoss()` 默认返回**平均损失**（可以通过参数 `reduction` 调整为求和等）。通常，我们希望损失越小越好，训练的目标就是最小化损失函数。了解如何计算损失有助于检查模型性能和调试训练过程。在本练习中，我们将实践使用 MSE 损失计算预测与目标之间的误差。

**编程任务：** 给定预测值张量 `pred` 和真实目标张量 `target`，请计算它们之间的均方误差损失。一方面，尝试不使用现成函数，直接根据公式手动计算 MSE；另一方面，使用 PyTorch 提供的 `nn.MSELoss` 来计算，并对比两者结果是否一致。下面提供了 `pred` 和 `target` 的示例值，请完成代码计算损失并比较。

```python
import torch.nn.functional as F

# 示例预测值和目标值张量
pred = torch.tensor([5.0, 7.0, 9.0])
target = torch.tensor([2.0, 4.0, 6.0])

# 手动计算 MSE 损失
mse_manual = ((pred - target) ** 2).mean()
print("手动计算的 MSE:", mse_manual.item())

# 使用 PyTorch 内置的 MSELoss 计算损失
loss_fn = nn.MSELoss()               # 定义 MSE 损失函数实例
mse_loss = loss_fn(pred, target)     # 计算损失
print("nn.MSELoss 计算的 MSE:", mse_loss.item())
```

**验证：** 比较手动计算的结果 `mse_manual` 与 `mse_loss` 是否相同。它们应该完全一致。我们通过断言验证这一点：

```python
assert torch.allclose(mse_manual, mse_loss), "MSE 计算不一致，请检查实现。"
print("第4步通过：损失函数计算正确一致。")
```

如果输出“第4步通过”，说明您正确计算了均方误差，并验证了手动计算与 PyTorch 内置函数的一致性。在训练神经网络时，我们会使用损失函数来衡量模型好坏，并通过自动微分获取损失对模型参数的梯度。接下来，我们将介绍如何使用优化器根据梯度来更新模型参数。

---

### 第5步：优化器 (Optimizers)

**理论介绍：** 优化器是利用损失函数计算得到的梯度来更新模型参数的算法。在深度学习中，最常用的优化方法是基于梯度下降的变体，例如随机梯度下降（Stochastic Gradient Descent, **SGD**）、Adam 等。PyTorch 提供了 `torch.optim` 模块实现了多种优化器。使用优化器的一般步骤是：先创建优化器实例，将模型参数和学习率等超参数传入，例如：`optimizer = torch.optim.SGD(model.parameters(), lr=0.01)`。然后在每次迭代中，先调用 `optimizer.zero_grad()` 将梯度缓存清零（因为 PyTorch 的梯度默认会累加 ([PyTorch’s Dynamic Graphs (Autograd) | by Zach Wolpe | Medium](https://zachcolinwolpe.medium.com/pytorchs-dynamic-graphs-autograd-96ecb3efc158#:~:text=Note%3A%20,would%20destabilise%20our%20optimization%20procedure))），再执行前向计算得到损失 `loss`，调用 `loss.backward()` 计算梯度，最后调用 `optimizer.step()` 根据梯度更新参数。一旦调用了 `optimizer.step()`，优化器会按照所选算法调整每个参数的值，例如 SGD 就是简单地做 θ:=θ−α∇L(θ)\theta:= \theta - \alpha \nabla L(\theta)（其中 α\alpha 是学习率）。需要注意学习率对训练收敛速度和效果的重要性：学习率太大可能导致震荡甚至发散，太小则收敛缓慢。在实践中，常常需要试验或自适应调整学习率。通过优化器迭代，模型的损失一般会逐步下降，模型性能提升。本练习将模拟一个简单的训练过程，对上一步骤实现的线性回归模型使用梯度下降优化其参数。

**编程任务：** 我们以一个简单的回归问题来演示优化过程：假设真实关系为 y=3x−1y = 3x - 1。我们用之前实现的 `LinearRegression` 模型来拟合这个关系。首先，生成训练数据，例如 x=[0,1,2,3]x =[0,1,2,3]对应的 ytrue=[−1,2,5,8]y_{\text{true}} =[-1, 2, 5, 8]（代入 y=3x−1y=3x-1 得到）。然后使用 MSE 作为损失，选择 SGD 优化器（学习率设为 0.1）。执行多次迭代训练模型，每次通过模型预测计算损失，反向传播梯度并调用优化器一步步调整参数。编写训练循环代码，观察损失的下降。完成以下代码中的 `TODO` 部分。

```python
# 初始化模型和优化器
model = LinearRegression()
optimizer = torch.optim.SGD(model.parameters(), lr=0.1)
loss_fn = nn.MSELoss()

# 构造训练数据: y = 3x - 1
x_train = torch.tensor([0.0, 1.0, 2.0, 3.0])
y_train = torch.tensor([-1.0, 2.0, 5.0, 8.0])  # 对应真实关系 3*x-1

# 在训练前计算初始损失
initial_loss = loss_fn(model(x_train), y_train).item()

# 训练模型迭代 100 次
for epoch in range(100):
    # TODO 1: 清零梯度
    # TODO 2: 正向传播计算模型在 x_train 上的预测值
    # TODO 3: 计算预测值与 y_train 间的损失 loss
    # TODO 4: 反向传播计算梯度
    # TODO 5: 使用优化器更新参数
    pass

# 训练结束后计算最终损失
final_loss = loss_fn(model(x_train), y_train).item()
print("初始损失:", initial_loss, "训练结束损失:", final_loss)
```

**验证：** 检查训练前后的损失对比，`final_loss` 应显著小于 `initial_loss`，表明模型通过优化参数成功降低了误差。以下断言验证这一点：

```python
assert final_loss < initial_loss, "损失没有降低，模型可能没有正确训练，请检查实现。"
print("第5步通过：模型训练损失降低，优化器步骤正确。")
```

如果输出显示“第5步通过”，说明您的训练循环实现是正确的，模型损失成功下降。这意味着优化器正确地利用梯度更新了模型参数。在实际训练中，我们会运行更多次迭代，直到损失收敛到满意的程度。本示例只是简单演示了参数更新的机制。接下来，我们将学习如何更方便地管理数据集和数据加载。

---

### 第6步：数据集 (Dataset) 与数据加载 (DataLoader)

**理论介绍：** 在训练模型时，良好的数据加载机制十分重要。PyTorch 提供了数据集抽象 `torch.utils.data.Dataset` 和数据加载器 `torch.utils.data.DataLoader`，可以方便地管理和批量加载数据 ([Datasets & DataLoaders — PyTorch Tutorials 2.6.0+cu124 documentation](https://pytorch.org/tutorials/beginner/basics/data_tutorial.html#:~:text=readability%20and%20modularity,easy%20access%20to%20the%20samples))。**数据集（Dataset）**通常表示一组样本及其标注，它是一个抽象类，我们需要继承并实现两个方法：`__len__` (返回数据集中样本数) 和 `__getitem__` (根据索引返回一个样本)。自定义数据集类可以将数据的读取、预处理封装其中，使训练代码更简洁。对于常见数据，PyTorch提供了一些内置子类（如 `torchvision.datasets` 下的MNIST等）或者辅助类如 `TensorDataset` 来快速包装现有数据张量。**数据加载器（DataLoader）**则接收一个 Dataset，对其进行批次加载、打乱顺序（shuffle）等操作，返回一个可迭代对象用于遍历数据集的小批量。通过设定 `batch_size` 可以决定每个批次加载多少样本，`shuffle=True` 在每轮迭代前随机打乱数据顺序，这对训练非常重要以减少偏差。DataLoader 还支持多进程加速数据读取（通过 `num_workers` 参数），不过初学阶段可以使用默认的单进程。总之，Dataset 和 DataLoader 可以将**数据处理**与**模型训练**解耦，提高代码的模块化和可读性 ([Datasets & DataLoaders — PyTorch Tutorials 2.6.0+cu124 documentation](https://pytorch.org/tutorials/beginner/basics/data_tutorial.html#:~:text=Code%20for%20processing%20data%20samples,easy%20access%20to%20the%20samples))。本练习将引导您创建一个简单的数据集并使用数据加载器按批次读取数据。

**编程任务：** 定义一个自定义数据集类 `MyDataset`，继承自 `torch.utils.data.Dataset`。为简单起见，我们使用几个定死的数据点：比如 X=[1.0,2.0,3.0,4.0]X =[1.0, 2.0, 3.0, 4.0]，对应目标 Y=[2.0,4.0,6.0,8.0]Y =[2.0, 4.0, 6.0, 8.0]（这里目标就是输入的2倍，您也可以换成别的关系）。在 `__init__` 中初始化保存数据的张量或列表，在 `__len__` 返回样本数（比如4），在 `__getitem__` 根据给定索引返回 `(X[idx], Y[idx])` 这一对数据和标签。实现完成后，使用 `DataLoader` 来加载该数据集，设置 `batch_size=2` 进行小批量迭代，并验证 DataLoader 是否正确输出了我们期望的批次数据。请完成以下代码中的 `TODO` 部分。

```python
from torch.utils.data import Dataset, DataLoader

class MyDataset(Dataset):
    def __init__(self):
        # TODO: 初始化数据集（在此处创建张量 self.X 和 self.Y）
        self.X = torch.tensor([1.0, 2.0, 3.0, 4.0])
        self.Y = torch.tensor([2.0, 4.0, 6.0, 8.0])

    def __len__(self):
        # TODO: 返回数据集样本数量
        return len(self.X)

    def __getitem__(self, idx):
        # TODO: 根据索引返回对应的 (X, Y) 元组
        x = self.X[idx]
        y = self.Y[idx]
        return x, y

# 实例化数据集
dataset = MyDataset()
print("数据集大小:", len(dataset))

# 使用 DataLoader 加载数据，每个批次2个样本
loader = DataLoader(dataset, batch_size=2, shuffle=False)
for batch in loader:
    print(batch)
```

**验证：** 验证自定义数据集和数据加载器的输出是否正确。`len(dataset)` 应该等于4。索引0处的样本应为 `(tensor(1.0), tensor(2.0))`，DataLoader 第一个批次应包含前两个样本 `[1.0, 2.0]` 对应的 `[2.0, 4.0]`。我们通过断言检查：

```python
# 检查数据集长度
assert len(dataset) == 4, "数据集 __len__ 实现不正确"
# 检查 __getitem__ 第0项
x0, y0 = dataset[0]
assert x0.item() == 1.0 and y0.item() == 2.0, "数据集 __getitem__ 实现不正确"
# 检查 DataLoader 首个批次
first_batch_X, first_batch_Y = next(iter(loader))
assert torch.allclose(first_batch_X, torch.tensor([1.0, 2.0])) and \
       torch.allclose(first_batch_Y, torch.tensor([2.0, 4.0])), "DataLoader 输出的批次内容不正确"
print("第6步通过：自定义 Dataset 和 DataLoader 正确。")
```

当看到“第6步通过”时，说明您成功实现了数据集类，并使用 DataLoader 正确地按批读取了数据。在实际任务中，我们通常会用 DataLoader 将大量数据分成小批喂给模型，以充分利用GPU内存并加速计算。在下一步中，我们将把前面学到的数据加载、模型、损失和优化器组合起来，完成一个模型的训练与评估流程。

---

### 第7步：模型训练与评估流程 (Model Training & Evaluation Workflow)

**理论介绍：** 现在我们把之前各部分串联起来，完整演示模型的训练和评估流程。在实际场景中，通常会将数据分为训练集和测试集（或验证集）。训练阶段（training phase）通过多轮迭代（epoch）让模型在训练集上学习，每轮遍历整个训练集，每次从 DataLoader 获取一个批次，计算前向输出和损失，然后反向传播更新参数。模型在训练时通常要启用训练模式：调用 `model.train()` 可以启用某些层的训练行为（例如 Dropout 随机失活，BatchNorm 更新均值方差）。**评估阶段（evaluation phase）在训练若干轮后，用未见过的测试集来评估模型表现，以判断泛化能力。这时应将模型切换到评估模式：`model.eval()`，这会关闭 Dropout 等随机行为，确保结果可重复。评估时一般不需要计算梯度，因此在 `torch.no_grad()` 上下文中进行，以加快计算和节省显存。评估指标视任务而定，回归可直接看损失值或均方根误差等，分类则看准确率（accuracy）等指标。典型的训练流程如下：

1. **准备数据**：加载并划分训练/测试数据集，使用 DataLoader 按批提供数据。
2. **模型初始化**：定义模型实例，设置损失函数和优化器。
3. **训练循环**：重复执行若干 epoch：
    - 对每个训练批次：模型设为训练模式，清零梯度，前向计算预测，计算损失，反向传播梯度，优化器更新参数。
    - 可选地，每个 epoch 结束后计算并输出在训练集（或验证集）上的性能指标，监控训练进展。
4. **评估**：在测试集上，用模型的评估模式计算指标，评估最终模型效果。

在训练过程中，应关注损失随 epoch 下降的趋势，若损失不降可能需要调整学习率或模型等。评估结果则反映模型的泛化能力。下面我们通过一个简单示例再现上述流程。

**编程任务：** 延续之前线性关系 y=3x−1y = 3x - 1 的模拟数据，我们将其分为训练集和测试集。比如取 x=0...7x = 0...7 的点作为训练数据，对应 y=3x−1y = 3x-1；x=8,9x = 8,9 的点作为测试数据。使用 `TensorDataset` 快速创建数据集并用 DataLoader 加载。然后使用我们的 `LinearRegression` 模型，定义优化器（SGD）和损失（MSE）。编写训练循环运行若干个 epoch，每个 epoch 后在测试集上评估当前模型的平均损失。最后，检查训练得到的模型参数是否接近真实值（3 和 -1）。请完成以下代码中的训练循环部分。

```python
from torch.utils.data import TensorDataset

# 准备训练和测试数据
X_all = torch.arange(0., 10.)
Y_all = 3 * X_all - 1
X_train, Y_train = X_all[:8], Y_all[:8]   # 0..7 作为训练数据
X_test,  Y_test  = X_all[8:], Y_all[8:]   # 8..9 作为测试数据

train_dataset = TensorDataset(X_train, Y_train)
test_dataset = TensorDataset(X_test, Y_test)
train_loader = DataLoader(train_dataset, batch_size=4, shuffle=True)
test_loader = DataLoader(test_dataset, batch_size=2, shuffle=False)

model = LinearRegression()
optimizer = torch.optim.SGD(model.parameters(), lr=0.1)
loss_fn = nn.MSELoss()

# 训练多个 epoch
epochs = 50
for epoch in range(epochs):
    model.train()  # 切换训练模式
    for X_batch, Y_batch in train_loader:
        # TODO: 清零梯度
        # TODO: 正向传播计算当前批次的预测输出和损失
        # TODO: 反向传播计算梯度并更新模型参数
        pass

    # 每个 epoch 结束后评估在测试集上的表现
    model.eval()  # 切换评估模式
    test_loss = 0.0
    with torch.no_grad():  # 评估不计算梯度
        for X_batch, Y_batch in test_loader:
            pred = model(X_batch)
            test_loss += loss_fn(pred, Y_batch).item()
    test_loss /= len(test_loader)
    print(f"Epoch {epoch+1}: 测试集平均损失 = {test_loss:.4f}")

# 查看训练后的模型参数
final_w = model.weight.item()
final_b = model.bias.item()
print("训练得到的参数: ", final_w, final_b)
```

**验证：** 训练过程中，每个 epoch 打印的测试集损失应逐步下降。训练结束后，模型学得的参数 `final_w` 和 `final_b` 应接近真实值 3 和 -1。下面的断言允许一定的误差范围来验证这一点：

```python
assert abs(final_w - 3.0) < 0.3 and abs(final_b + 1.0) < 0.3, "训练后的参数偏离真实值较大，可能训练未收敛"
print("第7步通过：模型训练与评估流程正确完成！")
```

当看到“第7步通过”时，说明您成功地综合运用了前几步的知识，完成了一个简单模型的训练和测试流程。在这个过程中，我们按照标准范式组织了代码，使其具有良好的可读性和可维护性。从数据准备、模型定义到训练循环和评估，各部分各司其职。实际任务中还可能需要根据评估结果调整模型或训练策略，例如引入验证集监控过拟合、调整学习率等。对于本教程的简单线性模型而言，我们已经可以看到训练使模型参数逼近了真实值。

[[不收敛]]

---

### 第8步：使用 GPU 进行计算 (Using GPU)

**理论介绍：** GPU（图形处理器）具有大规模并行计算能力，在深度学习中常用于加速张量运算。PyTorch 可以灵活地将张量和模型加载到GPU上计算，从而大幅缩短训练和推理时间。使用 GPU 的步骤通常是：首先检查是否有可用 GPU（例如调用 `torch.cuda.is_available()`），然后指定设备 `device = torch.device("cuda")`（如果有多块GPU也可以指定索引如 `"cuda:0"`），接着将模型和数据迁移到该设备，比如 `model.to(device)`，`tensor.to(device)`。需要注意，**在 GPU 上的张量只能与同在 GPU 上的张量进行运算**，因此模型和输入数据需要在同一设备上。计算完成后，如果需要将结果拿到CPU上处理或输出，可以使用 `.to("cpu")` 将张量搬回 CPU，或者直接调用 `.cpu()` 方法。还有一种情况是苹果的M1/M2芯片可以使用 `'mps'` 设备，但本教程聚焦 CUDA GPU。一般来说，在训练代码中，会在最开始设置好 `device`，后续创建模型和张量时都搬移到设备上，以确保所有计算发生在GPU。这样做往往能把计算速度成倍提升。不过，也要留意GPU显存的使用，在处理大型模型和数据时需要考虑显存是否足够。本练习将演示如何检测GPU并在GPU上进行简单的张量计算。

**编程任务：** 编写代码检查当前是否有可用GPU，如果有则在GPU上进行一次简单的张量计算，将结果再搬回CPU验证正确性。如果没有GPU则退回在CPU上运算。具体来说：创建一个张量 `a =[1.0, 2.0, 3.0]`，将其移动到选定设备上，计算 `result = a * 2`（所有元素乘2），然后把结果移回CPU并与直接在CPU上计算的结果对比，确保一致。请完成下面的代码。

```python
# 检查是否有可用 GPU，没有则使用 CPU
device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
print("使用设备：", device)

# 在选定设备上创建/移动张量并计算
a = torch.tensor([1.0, 2.0, 3.0])
a_gpu = a.to(device)                   # 将张量移到设备（GPU或CPU）
result_gpu = (a_gpu * 2).to("cpu")     # 在GPU上计算，再将结果移回CPU
result_cpu = a * 2                     # 在CPU上计算

print("GPU 计算结果:", result_gpu)
print("CPU 计算结果:", result_cpu)
```

**验证：** 比较 GPU 计算得到的 `result_gpu` 和 CPU 计算的 `result_cpu` 是否一致。它们应该相等为 `[2.0, 4.0, 6.0]`。通过断言验证这一点：

```python
assert torch.allclose(result_gpu, result_cpu), "GPU 计算结果与 CPU 不一致！"
print("第8步通过：GPU 运算正确，结果与 CPU 一致。")
```

如果您的环境有GPU，输出会显示使用设备为“[[CUDA]]”，并打印“第8步通过”。即便没有GPU可用，本步骤也会在CPU上执行并通过测试（因为代码会自动选用CPU）。这说明您已经掌握了如何让 PyTorch 利用 GPU 进行计算。在真实训练中，将模型和数据迁移到GPU通常只需很少的代码改动，但能换来大幅的性能提升。

---

恭喜你！完成以上所有步骤后，您已经循序渐进地掌握了 PyTorch 编程的基础要点。从张量操作、自动微分，到构建模型、计算损失、使用优化器训练模型，以及数据加载和GPU加速，这些构成了使用 PyTorch 进行深度学习的基本工作流程。通过这些练习，您不仅了解了原理，还亲自动手实践并验证了每一步的效果。接下来，您可以尝试用学到的知识构建更复杂的模型，或深入学习PyTorch更高级的功能，例如自定义层、保存加载模型、分布式训练等。祝您在深度学习的旅途中学有所成！
