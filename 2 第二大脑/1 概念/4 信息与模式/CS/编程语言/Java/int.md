---
date created: 2024-07-22
date modified: 2025-07-10
uid: 99fcf8e0-6563-42f9-bbce-906935d187cc
---

4 个 byte  
4 × 8 个 bit  
00000000 00000000 00000000 00000000

int 取值范围在正负 21 亿之间，这也解释了黄子韬直播间为啥点赞变为负数，因为溢出了

```java
public final class Integer extends Number implements Comparable<Integer> {  
    public static final int MIN_VALUE = -21 4748 3648;  
    public static final int MAX_VALUE = 21 4748 3647;
```

int 取值范围在正负 21 亿之间，这也解释了黄子韬直播间为啥点赞变为负数，因为溢出了

```java
public final class Integer extends Number implements Comparable<Integer> {  
    public static final int MIN_VALUE = -21 4748 3648;  
    public static final int MAX_VALUE = 21 4748 3647;
```
