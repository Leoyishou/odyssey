---
date created: 2024-07-14
date modified: 2025-07-10
uid: 694c3cbe-0cd2-4f0d-b84c-540bef340964
---

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

## 本质

Netty是基于[Java NIO](Java%20NIO.md)技术封装的一套框架。为什么要封装，因为原生的Java NIO使用起来没那么方便，而且还有臭名昭著的bug，Netty把它封装之后，提供了一个易于操作的使用模式和接口，用户使用起来也就便捷多了。

## 参考资料

通俗地讲，Netty 能做什么？- 老钱的回答 - 知乎  
https://www.zhihu.com/question/24322387/answer/282001188
