---
draw:
title: List
date created: 2024-08-13
date modified: 2025-02-19
---
**Java 中 `List` 接口的 `equals()` 方法是被设计用来比较两个列表的内容是否相同，并且元素的顺序也要相同。** 它并不会比较这两个 `List` 对象在内存中的地址是否一致。

**详细解释:**

1. **内容比较 (Content Comparison):**
    
    - `List` 接口的 `equals()` 方法会逐个比较两个列表中**对应位置**的元素。
    - 对于列表中的每个元素，它会调用元素的 `equals()` 方法来判断元素是否相等。因此，如果列表中的元素是自定义对象，那么要确保您的自定义对象类正确地重写了 `equals()` 方法，以便能够进行有意义的内容比较。对于 `Long` 类型 (以及其他基本类型的包装类，如 `Integer`, `String` 等)，Java 已经提供了正确的 `equals()` 实现，会比较它们的值。
    - 只有当两个列表的**长度相同**，并且**所有对应位置的元素都相等** (根据元素的 `equals()` 方法判断)，`List.equals()` 方法才会返回 `true`。
2. **内存地址比较 (Memory Address Comparison):**
    
    - 如果您想要比较两个 `List` 对象是否是同一个对象 (即在内存中占据相同的地址)，您应该使用 `==` 运算符。
    - `==` 运算符对于引用类型 (如 `List` 对象) 来说，比较的是两个引用是否指向内存中的同一个对象。只有当两个引用指向同一个内存地址时，`==` 才会返回 `true`。

## 增加外层size

```Java
List<List<T>> mainList = new ArrayList<>();
mainList.addAll(list1);
mainList.addAll(list2);
```

示意图:

```Java
初始状态:    mainList = []

添加list1:   mainList = [A,B](A,B)
                          ^      ^
                          |      |
                        从list1添加

添加list2:   mainList = [A,B](A,B)
                                        ^      ^
                                        |      |
                                      从list2添加
```

## 对应 item 结合

```Java
List<List<T>> mainList = new ArrayList<>(list1);
for (int i = 0; i < mainList.size(); i++) {
    mainList.get(i).addAll(list2.get(i));
}
```

示意图:

```Java
初始状态:    mainList = [A,B](A,B)  (从list1复制)
             list2 =    [E,F](E,F)

合并后:      mainList = [A,B,E,F](A,B,E,F)
                           ^  ^       ^  ^
                           |  |       |  |
                          从list2对应位置添加
```

主要区别:

- 方式一会增加mainList的长度,保持子列表不变
- 方式二保持mainList的长度不变,但会增加每个子列表的长度
