---
draw:
title: numpy
date created: 2025-05-01
date modified: 2025-05-02
---

![CleanShot 2025-05-02 at 01.12.36@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_jzOmk6moae%2F2025%2F05%2F02%2F01-13-39-7cf6eebd668b80f9b99970571048771e-CleanShot%202025-05-02%20at%2001.12.36-2x-627d49.png)

axis0是填横条的那些格子（按列计算），1 是填竖条的那些格子（按行计算）

#第一性原理 numpy 广播机制的本质是通过按照类似 excel 的智能扩展的思想，在矩阵乘法中，快速把维度对不上的那个小矩形，智能扩展到对得上维度

## 核心定位一眼看

| 维度         | **NumPy**                   | **pandas**                  |
| ---------- | --------------------------- | --------------------------- |
|            | 本质是矩阵，是纯粹的张量                | 和 excel 一样，啥都能装的表格          |
| **关注点**    | _高性能、同质、数值数组_               | _带索引的异质表格数据_                |
| **主要数据结构** | `ndarray`—任意维度、元素类型一致     | `Series`（一维）`DataFrame`（二维）|
| **典型场景**   | 线性代数、信号处理、深度学习底层张量          | 数据清洗、统计分析、商业报表              |
| **缺省索引**   | 位置索引（0 … n-1）| 显式行列标签，可自动对齐                |
| **缺失值**    | 没有语义层支持，需用 `np.nan`、掩码等手动处理 | 内置缺失值标记、填补、丢弃、对齐            |
| **运算习惯**   | 向量化数值运算、广播、u-func           | 分组聚合、时间序列重采样、透视表            |

> **一句话**：**NumPy 是“高维数字矩阵库”，pandas 是“带索引的表格分析库”，后者底层就用到了前者的数组。**



## 快速选型指南

> **先问自己：数据像“矩阵”还是“表格”？**
>
> - **矩阵/张量** ➜ 用 **NumPy**：高性能数学运算、可 GPU 化。
>
> - **表格/记录** ➜ 用 **pandas**：索引 + 缺失值 + 分组聚合。
>

如果仍拿不准，可以先用 pandas 做清洗，最后 `.values` / `.to_numpy()` 交给 NumPy 或深度学习框架；这也是大多数数据科学流程的默认路线。

```py
import numpy as np
import matplotlib.pyplot as plt

t = np.linspace(0, 1, 1000, endpoint=False)
x = np.sin(2*np.pi*50*t) + 0.5*np.sin(2*np.pi*120*t)
X = np.fft.fft(x)
freqs = np.fft.fftfreq(len(t), d=t[1]-t[0])

plt.plot(freqs[:500], np.abs(X)[:500])  # 显示前半谱
plt.title("50 Hz + 120 Hz 频谱")
plt.show()

```

_FFT_（Fast Fourier Transform）

以下 **NumPy 高频 API Cheatsheet** 汇总了上面教程中用到的常见函数（含属性）和典型用法，便于快速查阅。表格按功能分区，示例均假设已 `import numpy as np`。

## 1. 数组创建与基本属性

|API / 属性|作用|典型示例|
|---|---|---|
|`np.array(obj)`|将列表/元组等转成 `ndarray`|`np.array([1,2,3])`|
|`np.zeros(shape)` / `np.ones(shape)`|生成全 0 / 全 1 数组|`np.zeros((2,3))`|
|`np.full(shape, val)`|生成填充常数的数组|`np.full((2,2), 7)`|
|`np.arange(start, stop, step)`|等差序列|`np.arange(0,10,2)` → `[0 2 4 6 8]`|
|`np.linspace(start, stop, num)`|均分序列|`np.linspace(0,1,5)`|
|`arr.shape / arr.ndim / arr.size`|形状、维度数、元素总数|`arr.shape → (3,4)`|
|`arr.dtype`|数据类型|`arr.dtype → float64`|

## 2. 索引、切片与布尔筛选

先行i，后列 j

|API / 语法|作用|示例|
|---|---|---|
|`arr[i]/ arr[i,j]`|基本索引|`arr[1,2]`|
|`arr[start:stop:step]`|切片|`arr[::2]`|
|`arr[:, -1]`|所有行最后一列||
|`mask = arr > 0`|生成布尔掩码||
|`arr[mask]`|布尔索引筛选|`arr[arr%2==0]`|
|`arr[mask]= val`|条件赋值|`arr[arr<0]=0`|

## 3. 形状变换与拼接拆分

|API|作用|示例|
|---|---|---|
|`arr.reshape(new_shape)`|不拷贝重塑|`arr.reshape(2,3)`|
|`arr.ravel()` / `arr.flatten()`|展平成 1 维（`flatten` 返回拷贝）||
|`arr.T`|转置|`A.T`|
|`np.concatenate((a,b), axis)`|任意轴拼接|`np.concatenate((A,B),1)`|
|`np.vstack((a,b))` / `np.hstack((a,b))`|竖直 / 水平拼接||
|`np.split(arr, n, axis)`|等分拆分|`np.split(X,2,0)`|
|`np.vsplit` / `np.hsplit`|按行/列拆分||

## 4. 广播与逐元素运算

|API / 运算符|作用|示例|
|---|---|---|
|`+ - * /`|逐元素算术（支持广播）|`arr + 5`, `A * B`|
|`np.sqrt(arr)`、`np.sin(arr)` 等 ufunc|通用函数|`np.log(arr)`|
|**广播规则**|小维度扩展匹配大维度|`arr + row_vec`|

## 5. 聚合统计

|API|作用|典型示例|
|---|---|---|
|`arr.sum(axis=None)`|求和（`axis=None` 时全部元素）|`arr.sum(0)` 每列和|
|`arr.mean(axis)`|均值|`arr.mean(1)` 每行均值|
|`arr.min/max(axis)`|极值|`arr.max()`|
|`arr.std(axis)`|标准差||
|`np.prod`, `np.cumsum`, `np.cumprod`|乘积、累加、累乘||

## 6. 随机数生成（`np.random` 子模块）

|API|作用|示例|
|---|---|---|
|`np.random.seed(s)`|设定随机种子|`np.random.seed(42)`|
|`np.random.rand(d0, …)`|均匀分布 `[0,1)`|`np.random.rand(3,3)`|
|`np.random.randn(d0, …)`|标准正态分布|`np.random.randn(100)`|
|`np.random.randint(low, high, size)`|随机整数 `[low, high)`|`np.random.randint(0,10,5)`|
|`np.random.choice(seq, size, replace)`|随机抽样|`np.random.choice([1,2,3],2)`|

## 7. 线性代数（`np.linalg`）

| API / 运算符                         | 作用      | 示例                         |
| --------------------------------- | ------- | -------------------------- |
| `@` / `np.matmul(A,B)` / `np.dot` | 矩阵乘法    | `C = A @ B`                |
| `np.linalg.inv(A)`                | [[矩阵逆]] | `A_inv = np.linalg.inv(A)` |
| `np.linalg.det(A)`                | 行列式     | `np.linalg.det(A)`         |
| `np.eye(n)`                       | 单位矩阵    | `I = np.eye(3)`            |
| ` A.T`                            | 矩阵转置    |                            |

---

> **使用贴士**
>
> - **查看帮助**：在交互式环境用 `np.func?` 或 `help(np.func)` 快速查看文档。
>
> - **性能优化**：尽量用矢量化运算和广播，避免 Python 循环。
>
> - **避免不必要复制**：`reshape`、切片默认返回视图；只有在需要独立副本时用 `copy()` 或 `flatten()`。
>

拿着这份 Cheatsheet，就可以更高效地回忆和使用 NumPy 的核心 API 了。祝你编码顺利!
