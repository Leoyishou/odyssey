---
draw:
title: Map
date created: 2024-07-19
date modified: 2025-02-06
---

## Map 的本质是一个带了查找功能的 `List<Entry>`

```java
ImmutableMap<Long, PreRunTabItem> dataMap = Maps.uniqueIndex(lastChoice, PreRunTabItem::getId);
```

## computeIfAbsent 是分了三段

```java
strategyMap.computeIfAbsent(*****).add(strategy);
```

[HashMap](HashMap.md)

Hashtable在实现Map接口时保证了线程安全性，而HashMap则是非线程安全的。
**  
Hashtable不允许存入null，无论是以null作为key或value，都会引发异常。
而HashMap是允许存入null的，无论是以null作为key或value，都是可以的。

## 判空

[判空](判空.md)

## 通过 OrDefault 避免大量的判空

```java
Set<String> roomSet = hotelMap.getOrDefault("all", hotelMap.getOrDefault(hotelSeq, Sets.newHashSet()));
```

在 com.google.common.collect.ImmutableMap 中，无论是键（key）还是值（value）都不能为 null。这种设计有其特定的原因和优势：

- [不可变的](不可变的.md)和线程安全：
    - ImmutableMap 设计为完全不可变的数据结构。
    - 禁止 null 值有助于保证其[不可变的](不可变的.md)和线程安全性。
- 防止 NullPointerException：
    - 不允许 null 可以在使用时避免许多潜在的 NullPointerException。
    - 使用者可以确信从 map 中获取的值永远不会是 null。
