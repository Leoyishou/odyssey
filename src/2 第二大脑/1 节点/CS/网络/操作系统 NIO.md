---
aliases: [New Input/Output, NoneBlocking IO]
draw: 
tags: []
title: 操作系统 NIO
date created: 2024-09-21
date modified: 2024-11-12
---

NIO 是由操作系统提供的系统调用，早期这个操作系统调用的名字是 select，但是性能低下，后来渐渐演化成了 Linux 下的 epoll 和 Mac 里的 kqueue。我们一般就说是 [epoll](https://zhida.zhihu.com/search?q=epoll&zhida_source=entity&is_preview=1)，因为没有人拿苹果电脑作为服务器使用对外提供服务。

NIO 代表的一个词汇叫着 IO多路复用

想象你在图书馆看书。传统的[IO](IO.md)就像你一次只能看一本书，看完才能换下一本。而NIO就像你可以同时打开很多本书，在它们之间来回切换。

1. 非阻塞：你可以快速浏览多本书的目录，而不是被一本书卡住。
2. 选择器select：就像你的眼睛，可以快速扫视桌上所有打开的书，看哪本有新内容了。
3. 缓冲区：像是你的笔记本，可以先记下重要的内容，之后再慢慢处理。
