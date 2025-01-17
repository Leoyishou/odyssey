---
aliases: [Netty框架分析与思考]
draw:
tags: []
title: Netty框架分析与思考
date created: 2024-07-14
date modified: 2025-01-15
linter-yaml-title-alias: Netty框架分析与思考
---

好的，我来重新整理这篇文章的排版：

## Netty框架分析与思考

### 框架概述

曾经用Netty写过几个fq的小工具，可以说：Netty是一个从概念上来说外光鲜亮丽，但内部略显肮脏的网络处理框架。

Netty的精髓就在于其链式调用handler来处理协议中的每一层，将middleware思想引入了网络协议的处理中。

### 实践经验

在fq的工具的实现中，我可以从最底层的raw socket写起，对于tcp或者udp协议可以任意且随时进行切换，上层的加密协议也可以动态进行替换，甚至于可以无限套娃下去，写起来无比的爽快。

甚至对于一层协议中的每一个阶段，都可以将Handler的加载和卸载，作为一个状态机进行处理。比如说将socks5协议的某种认证方法，在一个handler中单独实现，当握手检测到对应的认证方法时，直接将挂载链上，待到认证完毕后，再卸载掉，后面的数据就无需再经过认证的Handler了。这个过程非常自然且顺畅。

而且在协议的处理过程中，基本上能做到将数据zero-copy的传递到下一层级（排除加解密等操作）。

### 存在的问题

但一旦到了具体Handler实现，就显得有些糟糕了。很多时候，Handler细分到一定程度无法细分了，就不得不在Handler手撸一个状态机，手撸状态机的结果就是，不得不再一个Handler里维护多个状态，导致一个Handler内部实现得复杂。

另一个糟糕之处就在于对于ByteBuf的使用，ByteBuf带来的zero-copy固然美好，但各种奇怪的ByteBuf，其生命周期的转移和维护，也是一个及其糟糕和麻烦的事情，一不留神就带了各种奇怪的内存泄漏的问题，是各种中间件和大数据组件的噩梦。

### 改进建议

如果让我改进Netty，我会考虑以下几点：

1. 首先，使用Handler分层的大思想应该是不变的，但Handler应该允许嵌套，可以灵活的进行加载和组装。
2. 其次：虚拟线程走起，java.nio给我滚开，Netty就是给这坨东西擦屁股的。
3. 然后，没了java.nio，ByteBuf就可以被简化了，不仅方法被简化，而且各种奇怪的也应该删掉，什么pooled/unpooled，direct/heap，safe/unsafe滚开，只要pooled/unpooled + heap就够了，是觉得zgc回收不够快么。

如果让我改进C#或者Kotlin里实现一个等价于Netty的东西，我还会考虑以下改进：

- Handler里的channelRead和channelWrite应该是一个可挂起的函数，在一个Handler的生命周期中，应该只运行一次。
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F12%2F15%2F01-19-10-bf254d4e3407430afe8a2c274df1964a-202412150119247-7b0969.png)

Langchain 是写大模型应用的框架，而 Netty 就是写聊天室应用的框架。

一个好使的处理Socket的东东 如果没有Netty？

远古：

```text
java.net + java.io
```

近代：

```text
java.nio
```

其他：

```text
Mina，Grizzly
```

### 本质

Netty是基于[Java NIO](Java%20NIO.md)技术封装的一套框架。为什么要封装，因为原生的Java NIO使用起来没那么方便，而且还有臭名昭著的bug，Netty把它封装之后，提供了一个易于操作的使用模式和接口，用户使用起来也就便捷多了。

### 参考资料

通俗地讲，Netty 能做什么？- 老钱的回答 - 知乎  
https://www.zhihu.com/question/24322387/answer/282001188
