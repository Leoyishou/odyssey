---
draw:
tags: []
title: defer
date created: 2024-07-24
date modified: 2024-11-12
---

Go 的 `defer` 通常用于资源清理，如解锁互斥锁、关闭文件等。与 [Java@](Java@.md) 的 `finnaly` 类似，多个 defer 同时存在时，按照 [栈](栈) 的方式，先进后出的顺序调用。
