---
aliases: [Remote Procedure Call]
draw: 
tags: []
title: RPC
date created: 2024-09-21
date modified: 2024-11-12
---

## 本质

RPC（Remote Procedure Call），又叫做远程过程调用。远程调用的本质就是 `resp = func（req）`一个方法门面，用本地 CPU 干，就是本地调用，让远方的 CPU 干就是远程过程调用。

它本身并不是一个具体的协议，而是一种调用方式，如果说 RPC 是汽车，HTTP 和 `gRPC`、`thrift`。分别是吉利、奇瑞、特斯拉。可以说，RPC 约等于应用层，[HTTP 1.1](HTTP%201.1.md) 和 `gRPC`、`thrift`都是它的实现。

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F21%2F18-39-52-d35889869d2d69d43cf854f36da06dc8-202409211839575-f3fb8e.png)

## 为啥需要 RPC 协议？

简单的[TCP](TCP.md)因为粘包，并不能实现让远方的 CPU 干活的这个诉求，所以需要一些协议，早在 70 年代就涌现初很多 RPC 协议。
但是这些 RPC 本质都是方言，应用范围很小，类似与电脑上的 360 管家和 360 公司的服务器通话。后来，有了浏览器以后，不同公司的服务器要互相通话了，这就需要一种普通话。[HTTP 1.1](HTTP%201.1.md)就诞生了！

## 有了[HTTP 1.1](HTTP%201.1.md)为啥还需要其他 RPC 协议 呢？

| 特性     | HTTP/1.1                              | 其他RPC协议                                   |
| ------ | ------------------------------------- | ----------------------------------------- |
| 服务发现   | 通过DNS解析域名获得IP地址，默认80端口                | 通常使用专门的中间服务（如Consul、Etcd、Redis）存储服务名和IP信息 |
| 底层连接形式 | TCP长连接（Keep Alive），可选用连接池             | TCP长连接，通常使用连接池                            |
| 传输内容   | 主要为字符串，包括Header和Body<br>Body常用JSON序列化 | 更灵活，常用Protobuf等更高效的序列化方式                  |
| 设计初衷   | 网页文本展示，后扩展到多媒体                        | 定制化程度高，面向特定服务调用                           |
| 数据冗余   | 较高，Header和Body中存在重复信息                 | 较低，可以通过协议约定减少冗余                           |
| 性能     | 相对较低，但通用性强                            | 相对较高，适合内部微服务                              |
| 使用场景   | 公共互联网服务，需要广泛兼容性                       | 公司内部微服务，追求高性能                             |

所以这个阶段，RPC 的性能是要比[HTTP 1.1](HTTP%201.1.md)好的，所以很多场景下还是继续用 RPC 的

## 既然有了 [HTTP 2](HTTP%202.md)，还需要其他 RPC 协议吗？

[HTTP 2](HTTP%202.md) 在 HTTP/1.1 的基础上做了很多改进，所以性能可能比很多 RPC 协议还要好，甚至连 `gRPC` 底层都直接用的 HTTP/2。

**历史原因**，这个是由于 HTTP/2是2015年出来的。那时候很多公司内部的 RPC协议都已经跑了好些年了，基于历史原因，一般也没必要去换了。

## 参考资料

既然有 HTTP 协议，为什么还要有 RPC - 其所以然的文章 - 知乎  
https://zhuanlan.zhihu.com/p/678738021
