---
draw:
title: numpy$
date created: 2025-05-02
date modified: 2025-05-02
---

明白了，我将为您生成一个中文版的 NumPy 基础教程，采用 Exercism 风格、共约 10 小时学习量，面向已有 Python 基础的用户，聚焦 NumPy 高频使用的概念与技能，配有逐步练习与验证代码。请稍等，我会尽快整理完成并呈现给您。

## NumPy 基础教程：逐步练习指南

欢迎来到 **NumPy** 入门教程！本教程针对具有 Python 基础的中文读者，采用类似 Exercism 的逐步练习风格，帮助您在大约 10 小时内掌握 NumPy 的核心概念和高频应用场景。教程共分为多个步骤，每个步骤包含：

- **理论介绍**：简要讲解本步骤涉及的 NumPy 概念；
- **编程任务**：提供练习说明和代码框架，鼓励您亲自动手编码；
- **验证机制**：通过断言或输出检查您的代码是否正确。
    

通过这些循序渐进的练习，您将学习：

1. 数组创建与基本属性
2. 元素访问与切片
3. 广播机制
4. 数组运算
5. 布尔索引与条件筛选
6. 轴向聚合（如 `sum`、`mean`）
7. 数组变形（`reshape`、`ravel`）
8. 随机数生成（NumPy 的 `random` 模块）
9. 简单线性代数操作（矩阵乘法、转置、逆矩阵）
10. 数组的拼接与拆分
    

接下来，让我们从基础开始，一步步体验 NumPy 强大的数组操作功能吧！

### 步骤1：数组创建与基本属性

#### 理论介绍

NumPy 最重要的对象是 **多维数组**（即 `ndarray`）。NumPy 数组能够存储大量同质数据，并支持高效的向量化运算，这使其在数值计算中比 Python 原生列表更快也更方便。创建 NumPy 数组的常用方法包括：

- 从 Python 列表或元组转换：使用 `np.array([...])` 创建数组。
    
- 使用 NumPy 内建的初始化函数：如全零数组 `np.zeros(shape)`、全一数组 `np.ones(shape)`、指定常数填充的数组 `np.full(shape, value)`、等差序列 `np.arange(start, stop, step)`、等间隔序列 `np.linspace(start, stop, num)` 等等。
    

NumPy 数组具有一些重要的 **属性**：

- `ndarray.shape`：数组的形状，由各维度大小组成的元组。例如，形状 `(3, 4)` 表示数组有3行4列。
    
- `ndarray.dtype`：数组中元素的数据类型，例如 `float64`、`int32` 等。
    
- `ndarray.ndim`：数组的维度（轴）数量。
    
- `ndarray.size`：数组元素总个数（各维度大小的乘积）。
    

通过这些属性，我们可以了解数组的结构和类型。在使用 NumPy 时，一定要养成检查数组形状和类型的习惯，以避免由于维度或类型不匹配导致的错误。

#### 编程任务

请尝试完成以下练习，练习 NumPy 数组的创建和属性访问：

1. 将 Python 列表 `[1, 2, 3]` 转换为 NumPy 一维数组，并命名为 `arr1d`。然后，查看该数组的形状和数据类型。
    
2. 创建一个 3x3 的全零矩阵（元素类型默认为浮点型），命名为 `zeros`。
    
3. 创建一个包含 0 到 8 的等差序列的一维数组，并将其重塑（reshape）为 3x3 的二维数组，命名为 `arr2d`。
    

在下方的代码框架中填写相应代码来完成上述任务：

```python
import numpy as np

# 任务1：从列表创建 NumPy 一维数组
arr1d = None  # 使用 np.array 将 [1, 2, 3] 转换为数组

# 打印数组的形状和数据类型作为参考（可以在验证前查看）
print("arr1d =", arr1d)
print("shape =", arr1d.shape, "dtype =", arr1d.dtype)

# 任务2：创建一个 3x3 的全零数组
zeros = None  # 使用 np.zeros 创建形状为 (3,3) 的数组

print("zeros =\n", zeros)
print("shape =", zeros.shape, "dtype =", zeros.dtype)

# 任务3：创建一个 0-8 的数组并重塑为 3x3
arr2d = None  # 使用 np.arange 和 reshape 创建数组

print("arr2d =\n", arr2d)
print("shape =", arr2d.shape, "dtype =", arr2d.dtype)
```

#### 验证机制

执行以下断言语句以验证您实现的正确性。如果没有错误提示，则说明通过本步骤练习：

```python
# 验证 步骤1
assert arr1d is not None and isinstance(arr1d, np.ndarray), "任务1: arr1d 应该是一个 NumPy 数组"
assert arr1d.shape == (3,), "任务1: arr1d 的形状应为 (3,)"
assert np.array_equal(arr1d, np.array([1, 2, 3])), "任务1: arr1d 的元素值不正确"

assert zeros is not None and isinstance(zeros, np.ndarray), "任务2: zeros 应为 NumPy 数组"
assert zeros.shape == (3, 3), "任务2: zeros 的形状应为 (3,3)"
assert (zeros == 0).all(), "任务2: zeros 数组的所有元素都应为 0"

assert arr2d is not None and isinstance(arr2d, np.ndarray), "任务3: arr2d 应为 NumPy 数组"
assert arr2d.shape == (3, 3), "任务3: arr2d 的形状应为 (3,3)"
expected = np.arange(9).reshape(3, 3)
assert np.array_equal(arr2d, expected), "任务3: arr2d 的值应包含从0到8的元素并重塑为3x3"

print("步骤1所有测试通过！")
```

### 步骤2：元素访问与切片

#### 理论介绍

要高效地操作数组，我们需要熟练掌握 **索引** 和 **切片**。NumPy 的索引规则与 Python 列表类似，采用从 0 开始的索引：

- **一维数组**：使用单个索引访问元素，例如 `arr[0]` 获取第一个元素，`arr[-1]` 获取最后一个元素（负索引表示从末尾开始计数）。
    
- **多维数组**：使用逗号分隔的索引元组访问元素，例如 `arr[1, 2]` 访问二维数组中第二行第三列的元素（索引从 0 计数，因此对应原始数组的第2行第3列）。同理，可以使用 `arr[i, j, k]` 访问三维数组的元素。
    
- **切片**：使用 `start:stop:step` 语法来批量获取元素的子集，例如 `arr[0:3]` 获取下标 0 到 2 的元素，`arr[:, 1:3]` 获取所有行的第2列到第3列（不包含索引3）等。切片中的 `start` 默认是0，`stop` 默认是数组长度，`step` 默认是1。
    

需要注意的是，对 NumPy 数组进行切片操作返回的是 **原数组的视图（view）**，并没有产生数据拷贝。这意味着如果通过切片得到的子数组进行修改，原数组也会随之改变（与 Python 列表的切片行为不同，列表切片是拷贝）。

#### 编程任务

我们将使用一个示例数组来练习索引和切片。假设有如下 3x4 数组：

```python
arr = np.arange(1, 13).reshape(3, 4)
# arr 内容：
# [[ 1,  2,  3,  4],
#  [ 5,  6,  7,  8],
#  [ 9, 10, 11, 12]]
```

请完成以下任务：

1. 获取 `arr` 中第二行第三列的元素（即 `7`），将其存为变量 `elem`。提示：索引为 `[1, 2]`。
    
2. 提取 `arr` 中前两行和后两列组成的子数组，将其存为变量 `subarr`。提示：使用切片 `:2` 获取前两行，`-2:` 获取最后两列。
    
3. 利用负索引获取 `arr` 的最后一行，将其存为变量 `last_row`。
    

请在下方代码框架中填写代码：

```python
import numpy as np

arr = np.arange(1, 13).reshape(3, 4)
# 任务1：获取第二行第三列的元素
elem = None  # 使用 arr[...] 索引获取元素

# 任务2：提取前两行和后两列的子数组
subarr = None  # 使用切片 arr[...] 获取子数组

# 任务3：获取最后一行
last_row = None  # 使用负索引获取最后一行

print("elem =", elem)
print("subarr =\n", subarr)
print("last_row =", last_row)
```

#### 验证机制

运行以下代码检查您的切片操作是否正确：

```python
# 验证 步骤2
assert elem == 7, "任务1: 提取的元素值应为 7"
expected_subarr = np.array([[3, 4],
                            [7, 8]])
assert isinstance(subarr, np.ndarray), "任务2: subarr 应为 NumPy 数组"
assert subarr.shape == (2, 2), "任务2: subarr 形状应为 (2,2)"
assert np.array_equal(subarr, expected_subarr), "任务2: subarr 元素不正确，应为 [[3,4],[7,8]]"

expected_last = np.array([9, 10, 11, 12])
assert isinstance(last_row, np.ndarray), "任务3: last_row 应为 NumPy 数组"
assert last_row.shape == (4,), "任务3: last_row 应为一维数组（长度4）"
assert np.array_equal(last_row, expected_last), "任务3: last_row 元素不正确，应为 [9,10,11,12]"

print("步骤2所有测试通过！")
```

### 步骤3：广播机制

#### 理论介绍

**广播（Broadcasting）** 是 NumPy 支持的一种强大机制，使得对不同形状的数组进行算术运算成为可能。简单来说，如果两个数组的形状不同，NumPy 会尝试通过“扩展”较小的数组来匹配较大数组的形状，从而执行元素级的运算，而不需要实际复制数据。

广播的规则可以概括为：从最后一维开始比较，如果维度大小相同，或者其中一方为1（可以扩展），则这两个维度是兼容的。若比较完所有维度后都兼容，则可进行广播。常见的广播场景包括：

- **标量和数组**：标量可视为形状为 `(1,1,...)`，与任何数组的形状兼容。例如 `arr + 3` 会将 `3` 加到 `arr` 的每个元素上。
    
- **一维数组和二维数组**：例如将一个长度为 `n` 的一维数组加到形状为 `(m, n)` 的二维数组上，会将该一维数组视作每一行都相同的行向量，加到二维数组的每一行。
    
- **列向量和矩阵**：如果将形状为 `(m, 1)` 的数组（列向量）加到形状为 `(m, n)` 的数组上，则会将该列向量扩展到每一列参与计算。
    

如果数组在某个轴上的尺寸不匹配且又不为1，广播将无法进行，NumPy 会抛出错误。

#### 编程任务

下面我们通过具体例子体验广播机制。给定以下数组：

```python
arr = np.array([[1, 2, 3],
                [4, 5, 6]])      # 形状 (2, 3)
row_vec = np.array([10, 20, 30])  # 形状 (3,) 相当于 (1, 3)
col_vec = np.array([[10],
                    [20]])       # 形状 (2, 1)
```

请完成以下任务：

1. 计算 `arr + row_vec`，将结果存为 `result1`。这将把长度为3的一维数组 `row_vec` 广播到每一行，与 `arr` 的每一行相加。
    
2. 计算 `arr + col_vec`，将结果存为 `result2`。这将把形状为 (2,1) 的 `col_vec` 广播到每一列，与 `arr` 的每一列相加。
    

填写下方代码框架来完成计算：

```python
import numpy as np

arr = np.array([[1, 2, 3],
                [4, 5, 6]])
row_vec = np.array([10, 20, 30])
col_vec = np.array([[10],
                    [20]])

# 任务1：将 row_vec 广播加到 arr 的每一行
result1 = None  # 计算 arr + row_vec

# 任务2：将 col_vec 广播加到 arr 的每一列
result2 = None  # 计算 arr + col_vec

print("result1 =\n", result1)
print("result2 =\n", result2)
```

#### 验证机制

验证广播运算的结果：

```python
# 验证 步骤3
expected1 = np.array([[11, 22, 33],
                      [14, 25, 36]])
assert isinstance(result1, np.ndarray), "任务1: result1 应为 NumPy 数组"
assert result1.shape == (2, 3), "任务1: result1 形状应为 (2,3)"
assert np.array_equal(result1, expected1), "任务1: result1 计算不正确"

expected2 = np.array([[11, 12, 13],
                      [24, 25, 26]])
assert isinstance(result2, np.ndarray), "任务2: result2 应为 NumPy 数组"
assert result2.shape == (2, 3), "任务2: result2 形状应为 (2,3)"
assert np.array_equal(result2, expected2), "任务2: result2 计算不正确"

print("步骤3所有测试通过！")
```

### 步骤4：数组运算

#### 理论介绍

NumPy 对数组的运算采用 **逐元素** 的方式，即默认情况下算术运算符会应用到数组的每个对应元素上。这意味着：

- `arr1 + arr2` 会对两个形状相同的数组进行对应元素相加，得到相同形状的新数组。
    
- `arr1 - arr2`、`arr1 * arr2`、`arr1 / arr2` 等类似，对应元素相减、相乘、相除。
    
- 将数组与标量运算时（如 `arr + 5`），标量会广播为与数组相同形状，然后逐元素运算。
    

此外，NumPy 提供了大量 **通用函数（ufunc）** 来对数组进行元素级操作，例如 `np.sqrt`（平方根）、`np.sin`、`np.log` 等。这些函数也会自动对数组的每个元素执行运算并返回新数组。

需要注意的是，NumPy 中的 `*` 运算符进行的是逐元素乘法，而非矩阵乘法。如果需要矩阵乘法，可以使用 `np.dot` 或 `@` 运算符（将在后续步骤中介绍）。

利用 NumPy 的矢量化运算可以大大提高计算效率。例如，相比在 Python 中用循环将每个元素乘以2，使用 `arr * 2` 将显著更简洁和高速。

#### 编程任务

已知以下数组：

```python
arr1 = np.array([1, 2, 3])
arr2 = np.array([4, 5, 6])
arr3 = np.array([1, 4, 9, 16])
```

请完成以下运算：

1. 计算 `arr1` 和 `arr2` 的逐元素加法，将结果存为 `arr_sum`。结果应为 `[5, 7, 9]`。
    
2. 计算 `arr1` 和 `arr2` 的逐元素乘法，将结果存为 `arr_prod`。结果应为 `[4, 10, 18]`。
    
3. 计算 `arr3` 中每个元素的平方根，将结果存为 `arr_sqrt`。结果应为 `[1.0, 2.0, 3.0, 4.0]`。
    

填写下方代码框架来完成这些计算：

```python
import numpy as np

arr1 = np.array([1, 2, 3])
arr2 = np.array([4, 5, 6])
arr3 = np.array([1, 4, 9, 16])

# 任务1：逐元素相加
arr_sum = None  # 计算 arr1 + arr2

# 任务2：逐元素相乘
arr_prod = None  # 计算 arr1 * arr2

# 任务3：计算平方根
arr_sqrt = None  # 计算 np.sqrt(arr3)

print("arr_sum =", arr_sum)
print("arr_prod =", arr_prod)
print("arr_sqrt =", arr_sqrt)
```

#### 验证机制

运行以下断言检查结果：

```python
# 验证 步骤4
assert np.array_equal(arr_sum, np.array([5, 7, 9])), "任务1: arr_sum 应为 [5,7,9]"
assert np.array_equal(arr_prod, np.array([4, 10, 18])), "任务2: arr_prod 应为 [4,10,18]"
expected_sqrt = np.array([1.0, 2.0, 3.0, 4.0])
assert arr_sqrt is not None and np.allclose(arr_sqrt, expected_sqrt), "任务3: arr_sqrt 应为 [1.0, 2.0, 3.0, 4.0]"

print("步骤4所有测试通过！")
```

### 步骤5：布尔索引与条件筛选

#### 理论介绍

**布尔索引**是指使用布尔型（True/False）数组来索引数据。对于 NumPy 数组，可以用一个形状相同的布尔数组（由 True/False 组成）来选择原数组中对应位置为 True 的元素。这通常和条件过滤配合使用。例如：

```python
arr = np.array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
mask = arr % 2 == 0  # 计算每个元素是否为偶数，得到布尔数组
even_numbers = arr[mask]  # 提取 arr 中所有偶数
```

上例中，`mask` 是一个与 `arr` 等长的布尔数组，表示对应位置是否满足 `arr % 2 == 0`。用 `arr[mask]` 即可获取所有满足条件的元素，结果将是 `[0, 2, 4, 6, 8]`。

除了读取，布尔索引也可用于修改元素的值。比如，可以将满足条件的元素全部赋值为新值：

```python
arr[arr % 2 == 0] = -1  # 将 arr 中所有偶数替换为 -1
```

执行后，`arr` 中原本为偶数的位置现在都会变成 -1。

布尔索引可以实现按条件筛选、替换数组中的元素，比使用循环实现相同功能要简洁得多。

#### 编程任务

利用布尔索引进行条件筛选和修改。给定数组：

```python
arr = np.arange(10)  # 数组内容为 [0, 1, 2, ..., 9]
```

请完成以下任务：

1. 使用布尔条件提取出 `arr` 中所有的偶数，存入变量 `evens`。提示：条件判断 `arr % 2 == 0` 可生成偶数掩码（mask）。
    
2. 将 `arr` 中的所有偶数元素修改为 -1。
    

填写下方代码框架来完成上述任务：

```python
import numpy as np

arr = np.arange(10)
# 任务1：提取偶数
evens = None  # 使用布尔索引获取偶数数组

print("evens =", evens)

# 任务2：将偶数赋值为 -1
# 提示：直接对 arr 使用布尔索引进行赋值
# 在此操作之前，arr 应该还是原始的 0-9 序列（不要使用上一步提取出来的 evens 来赋值）
arr[ ... ] = ...  # 将满足条件的元素赋值为 -1

print("modified arr =", arr)
```

#### 验证机制

通过断言检查筛选和修改是否正确：

```python
# 验证 步骤5
assert isinstance(evens, np.ndarray), "任务1: evens 应为 NumPy 数组"
assert np.array_equal(evens, np.array([0, 2, 4, 6, 8])), "任务1: 提取的偶数不正确"
expected_modified = np.array([-1, 1, -1, 3, -1, 5, -1, 7, -1, 9])
assert np.array_equal(arr, expected_modified), "任务2: arr 中的偶数元素应已被替换为 -1"

print("步骤5所有测试通过！")
```

### 步骤6：轴向聚合（sum、mean 等）

#### 理论介绍

**轴向聚合**指在指定轴（dimension）上对数组进行归纳计算，例如求和、求平均值、找最大最小值等。NumPy 提供了方便的方法来沿任一轴执行这些操作：

- `arr.sum()` 不带参数时会将数组展平后求和，即计算所有元素的和。
    
- `arr.sum(axis=0)` 会沿着第0轴（行）方向求和，即计算每列的和，结果是一个按列聚合的数组。
    
- `arr.sum(axis=1)` 会沿着第1轴（列）方向求和，即计算每行的和。
    
- 类似地，`arr.mean(axis=0)` 计算每列的平均值，`arr.mean(axis=1)` 计算每行的平均值。
    
- 其他聚合函数如 `np.prod`（乘积）、`np.min`/`np.max`（最小值/最大值）、`np.std`（标准差）等也可以使用 `axis` 参数以按轴计算。
    

需要注意，指定 `axis` 后返回的结果维度会比原数组少。例如，对形状 `(m, n)` 的数组使用 `axis=1` 计算平均，会返回长度为 `m` 的一维数组，每个元素对应原数组一行的平均值。

#### 编程任务

给定数组：

```python
arr = np.array([[1, 2, 3],
                [4, 5, 6]])
```

请计算以下结果：

1. 所有元素的总和，存为 `total_sum`。
    
2. 每列的和（沿 axis=0 聚合），存为 `col_sum`。预期结果为包含两列和的数组，如 `[5, 7, 9]`。
    
3. 每行的平均值（沿 axis=1 聚合），存为 `row_mean`。预期结果为 `[2.0, 5.0]`。
    

填写下方代码框架来完成计算：

```python
import numpy as np

arr = np.array([[1, 2, 3],
                [4, 5, 6]])

# 任务1：计算所有元素之和
total_sum = None  # 使用 sum 函数（不指定 axis）

# 任务2：计算每列之和
col_sum = None    # 使用 sum 函数，指定 axis=0

# 任务3：计算每行的平均值
row_mean = None   # 使用 mean 函数，指定 axis=1

print("total_sum =", total_sum)
print("col_sum =", col_sum)
print("row_mean =", row_mean)
```

#### 验证机制

运行以下代码验证结果：

```python
# 验证 步骤6
assert total_sum == 21, "任务1: total_sum 应为 21"
assert np.array_equal(col_sum, np.array([5, 7, 9])), "任务2: col_sum 应为 [5, 7, 9]"
expected_mean = np.array([2.0, 5.0])
assert np.allclose(row_mean, expected_mean), "任务3: row_mean 应为 [2.0, 5.0]"

print("步骤6所有测试通过！")
```

### 步骤7：数组变形（reshape、ravel）

#### 理论介绍

NumPy 提供了灵活的函数来改变数组的形状，而不改变其数据。在数据分析中，经常需要在不同形状之间转换数组，例如将二维表格展开成一维，或把一维数组重塑为矩阵。

- `arr.reshape(new_shape)`：返回一个拥有相同数据但形状改变的新数组。`new_shape` 可以是一个整数元组，代表目标形状（各维度大小）。注意，新形状的总元素数量必须与原数组相同。通常情况下，reshape 不会拷贝数据，而是返回原数据的一个视图；除非新的形状无法适配内存布局，此时可能会产生副本。
    
- `arr.ravel()`：返回一个将原数组展平成一维的视图（view）。如果需要一个副本，可以使用 `arr.flatten()` 方法，它总是返回数据的拷贝。
    

例如，形状 `(2, 3)` 的数组可以 reshape 为形状 `(3, 2)` 或 `(6,)`（一维长度6），但不能 reshape 为不相等元素数量的形状。

#### 编程任务

练习数组变形操作：

1. 创建一个包含 0 到 5 的一维数组 `arr1d`（长度为6）。
    
2. 将 `arr1d` 重塑（reshape）为 2x3 的二维数组，命名为 `arr2d`。
    
3. 将 `arr2d` 展平（ravel）为一维数组，命名为 `arr_flat`。
    

填写下方代码框架来完成这些任务：

```python
import numpy as np

# 任务1：创建长度为6的数组 0,1,2,3,4,5
arr1d = None  # 使用 np.arange 创建 0-5 的数组

# 任务2：重塑为形状 (2, 3)
arr2d = None  # 使用 arr1d.reshape(2, 3)

# 任务3：将二维数组展平成一维
arr_flat = None  # 使用 arr2d.ravel()

print("arr1d =", arr1d)
print("arr2d =\n", arr2d)
print("arr_flat =", arr_flat)
```

#### 验证机制

运行以下断言检查：

```python
# 验证 步骤7
assert np.array_equal(arr1d, np.array([0, 1, 2, 3, 4, 5])), "任务1: arr1d 应为 [0,1,2,3,4,5]"
assert arr2d.shape == (2, 3), "任务2: arr2d 形状应为 (2,3)"
expected2d = np.array([[0, 1, 2],
                       [3, 4, 5]])
assert np.array_equal(arr2d, expected2d), "任务2: arr2d 内容不正确"
assert arr_flat.shape == (6,) , "任务3: arr_flat 形状应为 (6,)"
assert np.array_equal(arr_flat, arr1d), "任务3: arr_flat 的元素应与 arr1d 相同"

print("步骤7所有测试通过！")
```

### 步骤8：随机数生成（np.random 模块）

#### 理论介绍

NumPy 的 `random` 子模块提供了生成随机数的功能，包括各种概率分布的随机样本。在科学计算和模拟中，经常需要产生随机数据来测试算法或初始化模型。

常用的随机数函数有：

- `np.random.rand(d0, d1,...)`：生成 0 到 1 区间均匀分布的随机数，参数为维度尺寸。例如 `np.random.rand(3,3)` 生成一个 3x3 的随机浮点数组，每个值 ∈[0,1)。
    
- `np.random.randn(d0, d1,...)`：生成均值0、标准差1的标准正态分布随机数。
    
- `np.random.randint(low, high, size)`：生成位于 `[low, high)` 区间的随机整数。比如 `np.random.randint(0, 10, size=5)` 会返回5个介于0至9的随机整数。
    
- 其他诸如 `np.random.choice`（从给定列表中随机取值）、`np.random.random_sample`（类似 rand）等函数也很有用。
    

为了确保 **结果可复现**，可以使用 `np.random.seed(seed)` 设置随机数生成器的种子。相同的种子将产生相同的随机数序列，这在需要调试或比较结果时非常重要。

#### 编程任务

练习随机数的生成和种子设置：

1. 使用种子 `42` 初始化随机数生成器，然后生成一个 0 到 1 的随机浮点数，存为 `rand1`。
    
2. 再次将随机种子重设为 `42`，再生成一个随机浮点数，存为 `rand2`。比较 `rand1` 和 `rand2`，验证种子作用：在相同种子下，两次生成的随机数应该相同。
    
3. 生成一个 3x3 的随机浮点数组 `arr_rand`，要求服从均匀分布 `[0,1)`。
    
4. 生成一个包含5个随机整数的一维数组 `arr_ints`，随机整数范围在 0 到 9（包含）之间。
    

填写下方代码框架来完成这些任务：

```python
import numpy as np

# 任务1：使用种子42生成一个随机数
np.random.seed(42)
rand1 = None  # 生成一个 [0,1) 间随机浮点数

# 任务2：重设种子42再生成一个随机数
np.random.seed(42)
rand2 = None  # 生成一个随机浮点数

# 任务3：生成3x3的随机浮点数组
arr_rand = None  # 使用 np.random.rand 生成

# 任务4：生成长度为5的随机整数数组（0-9）
arr_ints = None  # 使用 np.random.randint 生成

print("rand1 =", rand1)
print("rand2 =", rand2)
print("arr_rand =\n", arr_rand)
print("arr_ints =", arr_ints)
```

#### 验证机制

运行以下断言验证结果特征（由于随机性，不检查具体值，只检查性质）：

```python
# 验证 步骤8
assert rand1 is not None and rand2 is not None, "任务1/2: 未正确生成随机数"
assert np.allclose(rand1, rand2), "任务1/2: 在相同种子下两次生成的随机数应相同"

assert arr_rand is not None and arr_rand.shape == (3, 3), "任务3: arr_rand 应为 3x3 数组"
assert ((arr_rand >= 0) & (arr_rand < 1)).all(), "任务3: arr_rand 的元素应位于 [0,1) 区间"
assert arr_ints is not None and arr_ints.shape == (5,), "任务4: arr_ints 应为长度5的一维数组"
assert np.issubdtype(arr_ints.dtype, np.integer), "任务4: arr_ints 元素应为整数"
assert (arr_ints >= 0).all() and (arr_ints < 10).all(), "任务4: arr_ints 元素应在 [0,9] 范围内"

print("步骤8所有测试通过！")
```

### 步骤9：简单线性代数操作（矩阵乘法、转置、逆）

#### 理论介绍

NumPy 包含线性代数模块 `numpy.linalg`，提供了矩阵乘法、矩阵分解、求逆等操作。我们在这里介绍最基本的几个：

- **矩阵乘法**：使用运算符 `@` 或函数 `np.matmul`/`np.dot`。与元素乘法不同，矩阵乘法遵循线性代数规则，例如形状 `(m,n)` 的矩阵乘以形状 `(n,p)` 的矩阵得到形状 `(m,p)` 的结果。例：`C = A @ B`。
    
- **转置**：使用 `A.T` 或 `np.transpose(A)` 可以得到矩阵 `A` 的转置矩阵，将行和列交换。
    
- **逆矩阵**：对于方阵，可以使用 `np.linalg.inv(A)` 计算其逆。如果矩阵不可逆（行列式为0），则会报错。验证矩阵逆是否正确的一个方法是看 `A @ A_inv` 是否等于单位矩阵 `I`（考虑浮点误差，用近似比较）。
    

#### 编程任务

给定矩阵：

```python
A = np.array([[1, 2],
              [3, 4]])
B = np.array([[5, 6],
              [7, 8]])
```

请完成以下任务：

1. 计算矩阵乘积 `C = A @ B`（或 `np.matmul(A, B)`），将结果存为 `product`。
    
2. 计算矩阵 `A` 的转置，将结果存为 `A_T`。
    
3. 计算矩阵 `A` 的逆矩阵，将结果存为 `invA`。(_提示_: 可用 `np.linalg.inv`)
    

在下方代码框架中填写代码：

```python
import numpy as np

A = np.array([[1, 2],
              [3, 4]])
B = np.array([[5, 6],
              [7, 8]])

# 任务1：矩阵乘法 A @ B
product = None  # 计算 A @ B

# 任务2：矩阵 A 的转置
A_T = None      # 计算 A.T

# 任务3：矩阵 A 的逆
invA = None     # 计算 np.linalg.inv(A)

print("product =\n", product)
print("A_T =\n", A_T)
print("invA =\n", invA)
```

#### 验证机制

通过断言检查计算结果：

```python
# 验证 步骤9
expected_product = np.array([[19, 22],
                             [43, 50]])
assert np.array_equal(product, expected_product), "任务1: 矩阵乘法结果不正确"

expected_AT = np.array([[1, 3],
                        [2, 4]])
assert np.array_equal(A_T, expected_AT), "任务2: A_T 不正确"

# 检查 A @ invA 是否约等于单位阵 I
I = np.eye(2)
assert invA is not None, "任务3: 未计算出 invA"
assert np.allclose(A @ invA, I), "任务3: invA 并非 A 的逆矩阵（检查计算是否正确）"

print("步骤9所有测试通过！")
```

### 步骤10：数组的拼接与拆分

#### 理论介绍

在处理数据时，经常需要将多个数组在不同方向上拼接（concatenate）成一个更大的数组，或将一个数组拆分（split）成多个小数组。NumPy 提供了灵活的拼接与拆分函数：

- **拼接（Concatenate）**：使用 `np.concatenate((arr1, arr2,...), axis=?)` 在给定轴上拼接数组。`axis=0` 表示按行纵向拼接（增加行数），要求待拼接数组列数相同；`axis=1` 表示按列横向拼接（增加列数），要求待拼接数组行数相同。还有便捷的 `np.vstack`（竖直堆叠，相当于 `axis=0`）和 `np.hstack`（水平堆叠，相当于 `axis=1`）等函数。
    
- **拆分（Split）**：使用 `np.split(arr, sections, axis=?)` 可以将一个数组按指定份数在某轴上均分。例如，`np.split(X, 2, axis=0)` 如果 X 有偶数行，会拆分为上下两个子数组。如果不能整分，也可以使用 `np.array_split` 进行不等分割。还有 `np.vsplit`、`np.hsplit` 分别针对 axis=0 和 axis=1 的简化接口。
    

#### 编程任务

给定两个 2x2 矩阵：

```python
A = np.array([[1, 2],
              [3, 4]])
B = np.array([[5, 6],
              [7, 8]])
```

请完成以下任务：

1. 将矩阵 `A` 和 `B` 在水平方向拼接（按列拼接），得到矩阵 `h_concat`。其形状应为 2x4。
    
2. 将矩阵 `A` 和 `B` 在垂直方向拼接（按行拼接），得到矩阵 `v_concat`。其形状应为 4x2。
    
3. 将矩阵 `v_concat` 在第0轴方向拆分为两个 2x2 的子矩阵，分别存为 `A2` 和 `B2`。拆分后应使 `A2` 等于原矩阵 `A`，`B2` 等于原矩阵 `B`。
    

填写下方代码框架来完成这些任务：

```python
import numpy as np

A = np.array([[1, 2],
              [3, 4]])
B = np.array([[5, 6],
              [7, 8]])

# 任务1：水平拼接 A 和 B
h_concat = None  # 使用 np.concatenate((A, B), axis=1)

# 任务2：垂直拼接 A 和 B
v_concat = None  # 使用 np.concatenate((A, B), axis=0)

# 任务3：将 v_concat 拆分为两个 2x2 子矩阵
# 提示：使用 np.split 或 np.vsplit
A2, B2 = None, None  # 拆分 v_concat

print("h_concat =\n", h_concat)
print("v_concat =\n", v_concat)
print("A2 =\n", A2)
print("B2 =\n", B2)
```

#### 验证机制

运行以下断言，验证拼接和拆分结果：

```python
# 验证 步骤10
expected_h = np.array([[1, 2, 5, 6],
                       [3, 4, 7, 8]])
assert np.array_equal(h_concat, expected_h), "任务1: 水平拼接结果不正确"

expected_v = np.array([[1, 2],
                       [3, 4],
                       [5, 6],
                       [7, 8]])
assert np.array_equal(v_concat, expected_v), "任务2: 垂直拼接结果不正确"

assert A2 is not None and B2 is not None, "任务3: 没有正确拆分 v_concat"
assert np.array_equal(A2, A), "任务3: A2 应等于原矩阵 A"
assert np.array_equal(B2, B), "任务3: B2 应等于原矩阵 B"

print("步骤10所有测试通过！")
```

### 总结

恭喜您完成了 NumPy 入门教程的所有练习！通过以上 10 个步骤的实践，您已经掌握 NumPy 数组的创建、索引操作、广播机制、常见运算、条件筛选、聚合函数、形状变换、随机数生成以及基本的线性代数运算等核心知识。在实际应用中，NumPy 还有更多高级功能（如更复杂的随机分布、线性代数分解、快速傅里叶变换等）。建议您在此基础上进一步探索 NumPy 的官方文档并进行更多练习，不断巩固所学知识。祝您在数据科学之旅中一切顺利！
