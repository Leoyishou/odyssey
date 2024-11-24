---
draw:
tags: []
title: HashMap扩容
date created: 2024-08-13
date modified: 2024-11-12
---

【集合】HashMap扩容

[Map](Map.md)

1、map允许一个null key，多个null value，table都不允许

2、扩容map：16，75%，2n

数组大小总是2的幂次方，将取模操作变成与操作

key 的 hashCode 经过扰动函数处理过后得到 hash 值，然后 hash%length==hash&(length-1)得到下标位置
