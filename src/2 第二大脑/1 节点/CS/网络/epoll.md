---
draw:
tags: []
title: epoll
date created: 2024-09-21
date modified: 2024-11-12
---

epoll 是Linux系统中的一个超级强大的工具，它帮助程序更有效地处理大量的网络连接。早期这个操作系统调用的名字是 select，但是性能低下，后来渐渐演化成了[观察者模式](观察者模式.md)。Linux 下的 epoll 和 Mac 里的 kqueue。我们一般就说是 [epoll](https://zhida.zhihu.com/search?q=epoll&zhida_source=entity&is_preview=1)，因为没有人拿苹果电脑作为服务器使用对外提供服务。

想象你是一个管理员，负责一个巨大的邮局：

1. 传统方式：你要不停地检查每个邮箱是否有新邮件。
2. epoll方式：你给每个邮箱装了一个小灯。有新邮件时灯会亮，你只需要查看亮灯的邮箱。

epoll的优点：

- 效率高：只关注有变化的连接。
- 可扩展：能处理非常多的连接，适合高并发的情况。

客户端：用户 服务器：骑手  
轮询 (Polling)：一直问骑手我的外卖到哪儿了  
Long Polling：你别问，送到了快递员会打电话通知你。
WebSockets：和骑手使用对讲机进行通信，随时交流，这是一个双向、持续的通信渠道。
Server-Sent Events (SSE)：骑手持续地告诉每一个更新，但是用户不能回应他。
