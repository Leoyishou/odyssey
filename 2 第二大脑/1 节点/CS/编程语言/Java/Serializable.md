---
draw:
tags: []
title: Serializable
date created: 2024-10-31
date modified: 2024-11-12
---

**可序列化的本质是：一个对象是可 IO 的**
1. 磁盘 IO，对象持久化 - 将对象状态保存到磁盘
2. 网络 IO，在网络上传输对象数据、在分布式系统比如不同JVM间传递对象

Serializable主要用于Java原生的序列化机制，是**不能跨语言**的，只能在 Java 程序之间传递:

1. 使用ObjectOutputStream序列化到文件
2. RMI远程调用
3. 某些缓存框架(如早期的Redis序列化)

现代微服务，序列化方式**支持多种语言**:

1. JSON/Protocol Buffers等序列化格式，如Jackson/Gson/Fastjson不需要实现Serializable:
2. Spring Cloud/Dubbo等RPC框架，Dubbo支持多种序列化方式：
	1. 使用Java原生序列化(dubbo配置serialization="java")
	2. 使用RMI协议(dubbo配置protocol="rmi")
	3. Kryo
	4. FST
	5. Protostuff
	6. JSON
