---
date created: 2024-07-19
date modified: 2025-07-10
uid: 2cac7ac0-3c62-478b-8468-7c915ea19155
---
## 认知

Map 的本质是一个带了查找功能的 `List<Entry>`

```java
ImmutableMap<Long, PreRunTabItem> dataMap = Maps.uniqueIndex(lastChoice, PreRunTabItem::getId);
```

computeIfAbsent 是分了三段

```java
strategyMap.computeIfAbsent(*****).add(strategy);
```

##  [CheatSheet](CheatSheet.md)

|                  |                                                                                |                                                                                                                 |                                                                                                                      |
| ---------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| **目的 (Purpose)** | **Python (dict) 最简方式**                                                         | **Java (Map/HashMap) 最简方式**                                                                                     | **备注 (Remarks)**                                                                                                     |
| 新建               | 1. const param = {}<br>2. `dict()`构造函数（可接受另一个字典、键值对序列、关键字参数或可迭代对象作为参数）以及字典推导式。|                                                                                                                 |                                                                                                                      |
| 添加/更新键值对         | `my_dict[key]= value`                                                          | `myMap.put(key, value)`                                                                                         |                                                                                                                      |
| 访问值 (按键)         | `my_dict[key]`                                                                 | `myMap.get(key)`                                                                                                | Python 用下标访问，键不存在时抛 `KeyError`；Java `get` 在键不存在时返回 `null`。|
| 获取值 (带默认值)       | `my_dict.get(key, default)`                                                    | `myMap.getOrDefault(key, defaultValue)`                                                                         | 两者都提供方法以避免因键不存在而产生的错误或 `null`。(Java 8+)                                                                              |
| 检查键是否存在          | `key in my_dict`                                                               | `myMap.containsKey(key)`                                                                                        | Python 用 `in` 操作符；Java 用方法。|
| 获取大小 (键值对数量)     | `len(my_dict)`                                                                 | `myMap.size()`                                                                                                  |                                                                                                                      |
| 删除键值对 (按键)       | `del my_dict[key]` / `my_dict.pop(key)`                                        | `myMap.remove(key)`                                                                                             | Python 有 `del` 关键字或 `pop` 方法；Java 用 `remove` 方法。`pop`/`remove` 通常返回被删除的值。|
| 遍历键              | `for key in my_dict:`                                                          | `for (KeyType key: myMap.keySet())`                                                                             | Python 默认迭代键；Java 迭代 `keySet()` 视图。|
| 遍历键值对            | `for key, value in my_dict.items():`                                           | `for (Map.Entry<K, V> entry: myMap.entrySet())` / `myMap.forEach(...)`                                          | Python `.items()` 很方便；Java 用 `entrySet()` 遍历或 `forEach` Lambda。|
|                  |                                                                                |                                                                                                                 |                                                                                                                      |
| 遍历值              | `for value in my_dict.values():`                                               | `for (ValueType value: myMap.values())`                                                                         |                                                                                                                      |
|                  | `    for key, value in recipe_updates:`                                        |                                                                                                                 |                                                                                                                      |
| 获取所有键 (视图)       | `my_dict.keys()`                                                               | `myMap.keySet()`                                                                                                | 返回键的视图，通常不是独立副本。|
| 获取所有值 (视图)       | `my_dict.values()`                                                             | `myMap.values()`                                                                                                | 返回值的集合视图，通常不是独立副本。|
| 清空               | `my_dict.clear()`                                                              | `myMap.clear()`                                                                                                 | 方法名和功能相同。|
| **有序性**          | **插入顺序** (Python 3.7+)                                                         | **无序** (`HashMap`), **插入顺序** (`LinkedHashMap`), **排序** (`TreeMap`)                                              | **重要区别**：Python 3.7+ 字典默认保持插入顺序。Java 需要选择具体 Map 实现来确定顺序 (`HashMap` 通常无序, `LinkedHashMap` 保证插入顺序, `TreeMap` 自然/指定排序)。|
| 判空               |                                                                                | import org.apache.commons.collections.CollectionUtils;  <br>import org.apache.commons.collections.MapUtils;<br> |                                                                                                                      |

在 com.google.common.collect.ImmutableMap 中，无论是键（key）还是值（value）都不能为 null。这种设计有其特定的原因和优势：

- [不可变的](不可变的.md)和线程安全：
    - ImmutableMap 设计为完全不可变的数据结构。
    - 禁止 null 值有助于保证其[不可变的](不可变的.md)和线程安全性。
- 防止 NullPointerException：
    - 不允许 null 可以在使用时避免许多潜在的 NullPointerException。
    - 使用者可以确信从 map 中获取的值永远不会是 null。

[HashMap](HashMap.md)

Hashtable在实现Map接口时保证了线程安全性，而HashMap则是非线程安全的。

Hashtable不允许存入null，无论是以null作为key或value，都会引发异常。

而HashMap是允许存入null的，无论是以null作为key或value，都是可以的。

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
