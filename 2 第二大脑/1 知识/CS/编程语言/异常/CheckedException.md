---
draw:
tags: []
title: CheckedException
date created: 2024-12-01
date modified: 2024-12-27
---

受检异常：编译器强制要求必须用 try-catch 处理或者用 throws 声明抛出

常见的受检异常包括：

1. IOException - 输入输出异常
   - FileNotFoundException
   - EOFException

2. SQLException - 数据库相关异常
3. ClassNotFoundException - 找不到类异常
4. InterruptedException - 线程中断异常
5. CloneNotSupportedException - 克隆不被支持异常

这些异常在编译时必须被处理(try-catch)或声明(throws)。
