---
draw:
title: HashMap
date created: 2024-08-19
date modified: 2025-02-06
---

## 拉链法解决哈希冲突

 当两个不同的键映射到同一个哈希桶时，HashMap不会简单地覆盖已存在的元素。

```Java
索引    HashMap数组
 0  ->  [ ] -> (key1, value1) -> (key2, value2)
 1  ->  [ ]
 2  ->  [ ] -> (key3, value3)
 3  ->  [ ] -> (key4, value4) -> (key5, value5)
 4  ->  [ ]
 ...
```
