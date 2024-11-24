---
draw:
tags: []
title: Throwable
date created: 2024-08-20
date modified: 2024-11-12
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F08%2F20%2F11-27-07-48ce5245af8e69526d54787f51fab89d-202408201127685-ec753b.png)

[异常](异常.md)：

- 需要 try-catch 的是红色部分，也就是checked exception
- 蓝色的是写的时候应该自己细心避免的，也就是 unchecked exception蓝色部分比如空指针、数组越界、ArithmeticException(1除以 0)都是需要写的时候就注意的，不能用 try-catch 规避，错了就是错了，出现了就不能弥补  
Error
- 根本无法处理的，和硬件配置也有关系的
