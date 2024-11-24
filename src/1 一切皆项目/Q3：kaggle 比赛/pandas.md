---
draw:
tags: []
title: pandas
date created: 2024-10-13
date modified: 2024-11-12
---

pandas 在使用时语法感觉很乱，有什么学习的技巧吗？- 奕高的回答 - 知乎  
https://www.zhihu.com/question/289788451/answer/2167204031

## DataFrame

DataFramed 的本质是`Map<String, List>`，而 ndArray 的本质是 `List<List>`

DataFrame 是 pandas 库中最常用的数据结构之一。你可以把它想象成一个表格或者电子表格,就像 Excel 里的工作表。这个 " 表格 " 有行和列:

1. 每一列代表一种类型的数据 (比如姓名、年龄、成绩等)。
2. 每一行代表一条完整的记录。

NumPy的核心数据结构是ndarray（N-dimensional array，N维数组）。这与pandas的DataFrame有些不同，更像是Java中的数组，但可以有多个维度。

   - 所有元素类型相同，这使得操作更快，NumPy的ndarray更适合数学运算和科学计算。
   - pandas的DataFrame更适合处理表格型数据，特别是当你需要不同类型的数据或带标签的数据时。

实际上，pandas的DataFrame在内部使用了NumPy的ndarray。你可以把DataFrame看作是在ndarray基础上增加了更多功能的数据结构。
