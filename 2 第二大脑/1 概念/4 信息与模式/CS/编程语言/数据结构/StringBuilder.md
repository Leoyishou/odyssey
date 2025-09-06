---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-01
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

StringBuilder 的核心优化就是**预分配容量**机制：
大部分 append 操作都是直接写入预留空间，避免了频繁的内存分配和复制。

# Cues



# Notes

## 工作原理

1. **初始容量分配**
    
    - StringBuilder 创建时会分配一个字符数组（通常默认16个字符）
    - 这个数组有预留空间，不是刚好贴合当前字符串长度
2. **append 时的行为**
    
    - 如果预留空间足够：直接把新字符写入数组的下一个位置，O(1)操作
    - 如果空间不够：才会扩容（通常是当前容量的2倍），然后复制

## 对比普通字符串拼接

```java
// 普通字符串拼接 - 每次都创建新对象
String s = "Hello";
s = s + " World";  // 创建新String对象，复制"Hello"，再加" World"
s = s + "!";       // 又创建新对象，复制"Hello World"，再加"!"

// StringBuilder - 利用预留空间
StringBuilder sb = new StringBuilder();  // 预分配16个字符空间
sb.append("Hello");   // 直接写入，还剩11个空间
sb.append(" World");  // 直接写入，还剩5个空间  
sb.append("!");       // 直接写入，无需复制
```

## 具体例子

```java
StringBuilder sb = new StringBuilder(20);  // 预分配20个字符的容量
// 内部数组: [_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]

sb.append("Hello");  // 长度5，容量20
// 内部数组: [H, e, l, l, o, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]

sb.append(" World"); // 长度11，容量20，直接append无需复制
// 内部数组: [H, e, l, l, o, , W, o, r, l, d, _, _, _, _, _, _, _, _, _]
```

这就是为什么 StringBuilder 在大量字符串拼接时效率高得多 - 大部分 append 操作都是直接写入预留空间，避免了频繁的内存分配和复制。
