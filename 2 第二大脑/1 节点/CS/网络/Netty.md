---
draw:
tags: []
title: Netty
date created: 2024-07-14
date modified: 2024-11-12
---

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

## 本质

Netty是基于[Java NIO](Java%20NIO.md)技术封装的一套框架。为什么要封装，因为原生的Java NIO使用起来没那么方便，而且还有臭名昭著的bug，Netty把它封装之后，提供了一个易于操作的使用模式和接口，用户使用起来也就便捷多了。

## 参考资料

通俗地讲，Netty 能做什么？- 老钱的回答 - 知乎  
https://www.zhihu.com/question/24322387/answer/282001188
