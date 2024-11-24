---
draw:
tags: []
title: BigDecimal
date created: 2024-07-16
date modified: 2024-11-12
---

ROUND_HALF_UP 四舍五入

compareTo

## int、float、double 转 BigDecimal

不要直接用 new 方法，而是用 value of

```java
public static void main(String[] args) {  
    double a = 0.05;  
    System.out.println(new BigDecimal(a));  
    System.out.println(BigDecimal.valueOf(a));  
}
```
