---
date created: 2024-08-13
date modified: 2025-07-10
uid: fdc277b9-6cf9-4bba-b013-c91d155e4f88
---
## 认知

Set 的本质是一个带了去重功能的 `List<Entry>`

涉及 contains 操作的一定要用 set 而不是[List](List.md)

##  [CheatSheet](CheatSheet.md)

|   |   |   |   |
|---|---|---|---|
|**目的 (Purpose)**|**Python (set) 最简方式**|**Java (Set/HashSet) 最简方式**|**备注 (Remarks)**|
|创建集合|`my_set = {1, 2, 3}` / `set(iterable)`|`Set<T> s = new HashSet<>(List.of(1,2,3))` / `Set.of(1,2,3)` (Immutable, Java 9+)|Python `{}` 字面量简洁；Java 通常用构造函数或 `Set.of` (不可变)。|
|添加元素|`my_set.add(item)`|`mySet.add(item)`|方法名相同。重复添加无效果。Java `add` 返回 boolean。|
|检查元素是否存在|`item in my_set`|`mySet.contains(item)`|Python 用 `in` 操作符；Java 用方法。集合查找效率高。|
|删除元素|`my_set.remove(item)` / `.discard(item)`|`mySet.remove(item)`|Python `remove` 元素不存在抛 KeyError，`discard` 不抛异常；Java `remove` 返回 boolean。|
|获取大小|`len(my_set)`|`mySet.size()`|Python 函数 vs Java 方法。|
|遍历元素|`for item in my_set:`|`for (Type item: mySet)` / `mySet.forEach(...)`|迭代方式类似。`set`/`HashSet` 无序；`LinkedHashSet` 插入顺序；`TreeSet` 排序。|
|**集合运算**|||**显著差异**：Python 对集合运算支持更直接（操作符/方法）。|
|- 并集 (Union)|`set1|set2`/`.union()`|`Set<T> u = new HashSet<>(s1); u.addAll(s2);`|
|- 交集 (Intersection)|`set1 & set2` / `.intersection()`|`Set<T> i = new HashSet<>(s1); i.retainAll(s2);`|Python 有操作符/方法；Java 需 `retainAll`。|
|- 差集 (Difference)|`set1 - set2` / `.difference()`|`Set<T> d = new HashSet<>(s1); d.removeAll(s2);`|Python 有操作符/方法；Java 需 `removeAll`。|
|- 对称差集 (Symmetric Diff.)|`set1 ^ set2` / `.symmetric_difference()`|_需手动组合 `addAll`/`removeAll`/`retainAll`_|Python 有操作符/方法；Java 无直接方法，实现较繁琐。|
|子集检查 (`A` 是 `B` 的子集?)|`set_A <= set_B` / `.issubset()`|`set_B.containsAll(set_A)`|Python 有操作符/方法；Java 用 `containsAll`。|
|超集检查 (`A` 是 `B` 的超集?)|`set_A >= set_B` / `.issuperset()`|`set_A.containsAll(set_B)`|Python 有操作符/方法；Java 用 `containsAll`。|
|清空集合|`my_set.clear()`|`mySet.clear()`|方法名和功能相同。|
|**有序性**|**无序** (`set`)|**无序** (`HashSet`), **插入顺序** (`LinkedHashSet`), **排序** (`TreeSet`)|Java 提供不同实现来控制顺序。Python `set` 不保证顺序 (虽然 CPython 内部实现可能表现出一定规律)。|

**关于其他数据结构:**

- **Tuple (元组) vs Java?** - 如之前讨论，Java 没有完全对应的内置类型。最接近的是不可变 `List` (`List.of`) 或 `record` (Java 16+，但通过名称访问)，或第三方库的 Tuple。因无直接对应，不太适合放入此表格。
- **String (字符串) vs `String`** - 两者都是**不可变**的字符序列。它们的操作非常多，可以单独列一个大表，但核心操作如获取长度 (`len()` vs `.length()`), 拼接 (`+` vs `+`/`StringBuilder`), 子串 (`slice` vs `.substring()`), 查找 (`in`/`.find()` vs `.contains()`/`.indexOf()`), 替换 (`.replace()` vs `.replace()`/`.replaceAll()`) 等各有异同。

这个表格涵盖了 Set 的主要对比。如果你对 String 或其他特定结构的比较感兴趣，可以再提出来。
