---
draw:
title: Untitled
date created: 2024-08-17
date modified: 2025-02-06
---

ConcurrentHashMap使用segment来分段和管理锁，segment继承自[ReentrantLock](ReentrantLock.md)，因此ConcurrentHashMap使用ReentrantLock来保证线程安全。

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F08%2F17%2F00-40-36-de8b8811b8d66bdf3adc754872e12546-202408170040997-825441.png)
