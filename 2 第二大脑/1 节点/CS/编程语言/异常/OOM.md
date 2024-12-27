---
draw:
tags: []
title: OOM
date created: 2024-07-31
date modified: 2024-12-27
---

OOM（Out of Memory）后，服务直接挂掉，需要重启

这篇文章主要讲述了JVM中的错误处理机制，核心内容是：

1. JVM遇到 OutOfMemoryError 或 StackOverflowError 不一定会退出
- 这些错误可以被catch捕获
- 一个线程崩溃不会影响其他线程
- JVM会隔离单个线程的问题

1. 错误处理过程
- 操作系统发现内存访问错误，发送SIGSEGV信号
- JVM拦截并处理这些信号
- 转换成Java中的Error抛给用户

1. 实际崩溃的情况
- 未注册处理函数的信号会导致JVM退出
- 某些JVM参数配置(如OnOutOfMemoryError)可以强制JVM在OOM时退出

简单说，就是JVM像个保护层，把系统级的严重错误转换成Java可处理的错误，让程序有机会恢复而不是直接崩溃。
