---
date created: 2024-10-25
date modified: 2025-07-10
uid: 5b6cd4d1-d1ce-4966-94a9-131141262ca3
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F25%2F11-49-57-e927f53ea46bd4a5adfcda031168c752-202410251149040-96a728.png)

在 IDEA 的调试视图中，这个"靶子"或"目标"符号🎯表示要为该变量设置**字段观察点**（Field Watchpoint）。

字段观察点的作用是：

1. 当该字段的值被读取时（Field Access）会触发断点
2. 当该字段的值被修改时（Field Modification）会触发断点

这对于调试非常有用，特别是在：

1. 想要追踪某个字段在何时、被谁修改的场景
2. 需要监控某个重要字段的访问情况
3. 调试并发问题时，观察共享变量的访问和修改

在你的代码中，设置对 `strategyMap` 的观察点可以帮助你：

1. 监控策略映射表何时被初始化
2. 追踪哪些代码在添加或修改策略
3. 了解策略映射表的访问模式

使用方法：

1. 右键点击行号旁边的空白区域
2. 选择 "Add Field Watchpoint"
3. 可以选择监控字段的读取、修改或两者都监控
