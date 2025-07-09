---
comment_id: 75fc7369
date created: 2024-05-16
date modified: 2025-02-06
draw: null
title: computeIfAbsent
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
