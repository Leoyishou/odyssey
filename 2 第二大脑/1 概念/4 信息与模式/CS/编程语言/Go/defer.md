---
date created: 2024-07-24
date modified: 2025-07-10
uid: 3dd36bd9-d2f0-4cbe-a337-a6affc07ea24
---

Go 的 `defer` 通常用于资源清理，如解锁互斥锁、关闭文件等。与 [Java@](Java@.md) 的 `finnaly` 类似，多个 defer 同时存在时，按照 [栈](栈) 的方式，先进后出的顺序调用。
