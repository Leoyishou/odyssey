---
draw:
title: computeIfAbsent
tags: []
date created: 2024-05-16
date modified: 2024-12-27
---

excerpt

<!-- more -->

## 取代的是 get 的地位

```java
groupedWinners
	.computeIfAbsent(r.getLinkedPrices(), k -> Lists.newArrayList())
	.add(r);

groupedWinners
	.get(r.getLinkedPrices(), k -> Lists.newArrayList())
	.add(r);
```
