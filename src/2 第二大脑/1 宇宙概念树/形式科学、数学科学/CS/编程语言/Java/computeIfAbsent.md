---
date created: 2024-05-16
date modified: 2025-07-10
uid: 90bdec72-1359-4e63-bd0a-d8d90f2a6ede
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
