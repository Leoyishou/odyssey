---
draw:
title: Stream
tags: []
date created: 2024-04-03
date modified: 2024-12-27
---

流我只用来过滤，分组，处理集合转换等等，涉及复杂业务逻辑都是用for循环写清楚。没毛病吧!

<!-- more -->

```mermaid
graph TB
    subgraph Map操作
    A["Stream<String><br/>[apple, banana]("Stream<String><br/>[apple,%20banana)
    B["map(s -> s.length())"]
    C["Stream<Integer><br/>[5, 6]("Stream<Integer><br/>[5,%206)
    A -->|"映射转换"| B
    B -->|"结果"| C
    end

    subgraph FlatMap操作
    D["Stream<String><br/>[hello, world]("Stream<String><br/>[hello,%20world)
    E["flatMap(s -> <br/>Arrays.stream(s.split('')))"]
    F["Stream<String><br/>[h,e,l,l,o,w,o,r,l,d]("Stream<String><br/>[h,e,l,l,o,w,o,r,l,d)
    D -->|"拆分并展平"| E
    E -->|"结果"| F
    end

    style A fill:#e1f5fe
    style C fill:#e1f5fe
    style D fill:#e1f5fe
    style F fill:#e1f5fe
    style B fill:#fff3e0
    style E fill:#fff3e0

```

## map n -> n

   - 输入是一个 Stream
   - 对每个元素进行 1:1 的转换
   - 输出仍然是一个 Stream，但元素类型可能改变

## flatMap n -> n × m

   - 输入是一个 Stream
   - 对每个元素进行 1:n 的转换（产生多个结果）
   - 将所有结果合并成一个扁平的 Stream

List<`Map<Integer, Integer>`> -> `Map<Integer, Integer>`

|   |
|---|
|`Map<Integer, Integer>` mergedDiscount = sellPromotions.stream()    <br>.reduce(Maps.newHashMap(), (merge, map) -> {    <br>        map.forEach((key, value) -> merge.merge(key, value, (v1, v2) -> nullToZero(v1) + nullToZero(v2)));    <br>        return merge;    <br>    });|

这段代码是使用 Java 的 Stream API 来合并一个 `sellPromotions` 列表中的所有促销折扣信息到一个新的 `Map` 中。这个列表里的每个元素都是一个 `Map<Integer, Integer>` 类型，表示的是商品的 ID 和对应的促销折扣值。合并的逻辑是将所有这些 Map 中的键值对加入到一个新的 Map 中，如果出现相同的键（即相同的商品 ID），则将它们的值（促销折扣）相加。如果任何一个值为 `null`，则在相加之前将其视为 `0`。

让我们通过一个例子来说明这段代码的工作原理：

假设我们有以下的促销折扣列表（`sellPromotions`）：

- 第一个促销折扣 Map：`{1=10, 2=20}`
- 第二个促销折扣 Map：`{2=5, 3=15}`

这里，`nullToZero` 是一个假设的函数，用于将 `sellPromotions`0 值转换为 `sellPromotions`1。这段代码中并没有直接展示这个函数的实现，但其功能是明确的。

现在，我们使用提供的代码片段来合并这些促销折扣：

1. 初始化一个新的 HashMap。
2. 遍历 `sellPromotions`2 列表中的每一个 Map：
    - 对于第一个 Map：将 `sellPromotions`3 中的键值对添加到新的 Map 中，现在新的 Map 变为 `sellPromotions`4。
    - 对于第二个 Map：尝试将 `sellPromotions`5 中的键值对加入到新的 Map 中。
        - 对于键 `sellPromotions`6，它在新的 Map 中已存在，因此将两个值相加：`sellPromotions`7，更新后的 Map 为 `sellPromotions`8。
        - 对于键 `sellPromotions`9，它在新的 Map 中不存在，直接加入，最终 Map 为 `Map`0。

最终结果，新的 Map 包含了所有促销信息的合并，其中相同商品 ID 的促销折扣被累加。

这段代码的关键是使用 `Map`1 函数来处理键值对的合并逻辑。`Map`2 函数接受三个参数：键、要合并的值以及一个合并函数（如果键已经存在）。在这个例子中，合并函数是 `Map`3，它确保了如果有任何 `Map`4 值，它们会被当作 `Map`5 来处理，并将两个值相加。
