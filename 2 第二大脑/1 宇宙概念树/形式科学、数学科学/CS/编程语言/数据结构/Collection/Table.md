---
comment_id: e8dda8c1
date created: 2024-10-30
date modified: 2025-03-27
draw: null
title: Table
---
Guava的Table是一个双键映射的数据结构,类似于Map<R, `Map<C, V>`>。主要特点:

1. 基本操作:

```java
// 创建
Table<String, String, Integer> table = HashBasedTable.create();

// 放入值
table.put("row1", "col1", 1);

// 获取值
Integer value = table.get("row1", "col1");
```

1. 常用视图:

```java
// 获取所有行的Map
Map<String, Map<String, Integer>> rowMap = table.rowMap();

// 获取特定行
Map<String, Integer> row = table.row("row1");

// 获取所有列
Map<String, Map<String, Integer>> columnMap = table.columnMap();
```

1. 主要实现类:
- HashBasedTable: 基于HashMap
- TreeBasedTable: 基于TreeMap,支持排序
- ImmutableTable: 不可变实现
- ArrayTable: 密集数据的固定大小实现

适用场景:需要两个键来确定一个值的场合,如:坐标系统、评分系统等。
