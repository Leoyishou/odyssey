---
draw:
title: Immutable List，tuple
date created: 2025-03-26
date modified: 2025-03-29
---
**
两者都保证了实例本身是不可变的（不能添加、删除、替换元素）。

|   |   |   |   |
|---|---|---|---|
|**操作目的 (Purpose)**|**Python tuple**|**Java ImmutableList (e.g., List.of / Guava)**|**备注 (Remarks)**|
|**创建 (Creation)**|`t = (item1, item2,...)` <br> `t = tuple(iterable)`|`List<T> l = List.of(item1, item2,...);` (Java 9+) <br> `ImmutableList<T> l = ImmutableList.of(...);` (Guava) <br> `ImmutableList<T> l = ImmutableList.copyOf(iterable);` (Guava)|Python 使用字面量 `()` 或构造函数。Java 9+ 提供 `List.of` 创建不可变 List；Guava 提供更丰富的创建方式。|
|**访问元素 (按索引)**|`value = my_tuple[index]`|`ValueType value = myList.get(index);`|Python 使用下标 `[]`，更简洁。Java 使用 `.get()` 方法。两者索引越界都会抛异常。|
|**获取长度/大小**|`length = len(my_tuple)`|`int size = myList.size();`|Python 使用内置 `len()` 函数。Java 使用 `.size()` 方法。|
|**遍历/迭代元素**|`for item in my_tuple:`|`for (Type item: myList)` <br> `myList.forEach(item ->...);` (Java 8+)|两者都支持简洁的 for-each 循环。Java 8+ 还提供 `forEach` 方法。|
|**检查元素是否存在**|`is_present = item in my_tuple`|`boolean isPresent = myList.contains(item);`|Python 使用 `in` 操作符，更自然。Java 使用 `.contains()` 方法。|
|**获取子序列 (切片/子列表)**|`sub = my_tuple[start:stop:step]`|`List<T> subList = myList.subList(fromIndex, toIndex);`|Python 切片非常灵活，返回新的 tuple。Java `subList` 返回 `List` 视图（对于不可变 List，该视图也是不可变的），不包含 `toIndex`。|
|**查找元素索引**|`index = my_tuple.index(item)`|`int index = myList.indexOf(item);` <br> `int lastIdx = myList.lastIndexOf(item);`|Python `.index()` 未找到时抛 `ValueError`。Java `.indexOf()` 未找到时返回 -1。Java 还提供 `.lastIndexOf()`。|
|**计算元素出现次数**|`count = my_tuple.count(item)`|`int count = Collections.frequency(myList, item);`|Python `tuple` 有内置 `.count()` 方法。Java 需要使用 `Collections` 工具类的静态方法 `.frequency()`。|
|**连接 (创建新序列)**|`new_tuple = tuple1 + tuple2`|_(无直接操作符)_ <br> (需手动创建或使用 Stream/Builder)|Python 使用 `+` 直接连接创建新 tuple。Java 没有 `+` 操作符，需要更复杂的步骤（如 Stream API 或 Guava 的 Builder）来合并创建新的不可变 List。|
|**转换 (e.g., to List)**|`mutable_list = list(my_tuple)`|`List<T> mutableList = new ArrayList<>(myImmutableList);`|两者都可以方便地转换为对应的可变列表类型。|

**核心差异总结:**

- **语法简洁性:** Python 在访问 (`[]`)、获取长度 (`len()`)、成员检查 (`in`)、切片 (`[:]`) 和连接 (`+`) 上通常语法更简洁、更符合直觉。
- **内置方法 vs. 接口/工具类:** Python `tuple` 有一些内置方法 (`.index()`, `.count()`)。Java 的操作大多来自 `List` 接口 (`.get()`, `.size()`, `.contains()`, `.indexOf()`, `.subList()`)，少数需要借助外部工具类 (`Collections.frequency()`)。
- **不变性保证:** 两者都保证了实例本身是不可变的（不能添加、删除、替换元素）。
- **来源:** `tuple` 是 Python 的核心内置类型。`ImmutableList` 在 Java 中主要指 Java 9+ `List.of()` 的返回类型或像 Guava 这样的库提供的实现，它们都实现了 `java.util.List` 接口（但修改操作会抛异常）。
