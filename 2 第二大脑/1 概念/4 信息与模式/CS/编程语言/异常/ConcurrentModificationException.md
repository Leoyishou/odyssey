---
date created: 2025-03-21
date modified: 2025-07-10
uid: f4993005-96dd-4dab-b0cf-0f82a84f25d3
---

以下的改写一定能彻底避免此问题：

**方案（1）强烈推荐：使用`computeIfAbsent`方法（完全线程安全）**

将你原有代码：

```java
if (metricsValueMap.get(i.getId()) == null) {
    MetricsValue value = calcCompoundMetricsValue(compoundMetricsMap.get(i.getId()), metricsValueMap);
    metricsValueMap.put(i.getId(), value);
}
```

改为安全版本：

```java
metricsValueMap.computeIfAbsent(i.getId(), id -> 
    calcCompoundMetricsValue(compoundMetricsMap.get(id), metricsValueMap)
);

```

`computeIfAbsent`方法内部做了**完全原子性保证**，即使并发调用，也只会计算一次。这样就能完全消除并发安全隐患。

---

## 🔎 **为什么一定要使用`computeIfAbsent`？**

- 因为**它能确保原子性**，完全避免竞态条件。
- 否则，**即便使用`ConcurrentHashMap`，仍可能存在业务逻辑中其他集合或状态的并发问题**。
