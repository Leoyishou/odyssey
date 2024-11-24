---
draw:
tags: []
title: Channel
date created: 2024-07-22
date modified: 2024-11-12
---

```go
c.server()          //普通调用一个函数
go c.schedule()     //异步起一个线程
```

Channel: 是 goroutine 之间通信的管道。想象一下,它就像两个人之间传递消息的管道。

- `c.heartbeatCh` 是一个通道 (channel)
- `<-` 是从通道接收数据的操作符
