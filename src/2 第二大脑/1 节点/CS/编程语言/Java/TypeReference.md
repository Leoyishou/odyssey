---
draw:
tags: []
title: TypeReference
date created: 2024-08-14
date modified: 2024-12-27
---

1. 用的时候的姿势是[匿名内部类](匿名内部类.md)的方式`new TypeReference<Map<String, Map<String, Set<String>>>>(){}`，这样可以使得泛型内容传给其 superClass 的`<T>`
2. 这里的泛型信息是会保留在字节码中，且可以同过[反射](反射.md)的 API 拿到的。

也就是说通过`((ParameterizedType) superClass).getActualTypeArguments()[0]`这个反射函数，把本来会被擦除的类型信息，变为可以保存的成员变量信息，来实现记录和保存某种类型
