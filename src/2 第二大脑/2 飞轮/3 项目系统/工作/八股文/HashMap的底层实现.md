---
draw:
tags: []
title: HashMap的底层实现
date created: 2024-08-13
date modified: 2024-12-27
---

【集合】HashMap的底层实现

[Map, Dict](Map,%20Dict.md)

1、两次哈希取模找到数组位置，equals比较覆盖，超过8且数组超过64时变红黑树，小于6退回链表，68不同是为了避免震荡。

2、当length为2的幂次方时 取模` hash%length==hash&(length-1)`

3、多线程的时候可能导致链表循环指向
