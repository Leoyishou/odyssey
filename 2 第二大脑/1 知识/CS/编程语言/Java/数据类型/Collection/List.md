---
draw:
tags: []
title: List
date created: 2024-08-13
date modified: 2024-12-27
---

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
