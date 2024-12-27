---
draw:
tags: []
title: HTTP 2
date created: 2024-09-21
date modified: 2024-12-27
---

![](https://cdn-a.markji.com/files/631575926dda045792bf5318_hd.jpg?e=1727092647503&token=xX63b9jqTlDOcGmctt5K9254rV0LG8hS9BmDeFBy:mopBSl-XxBH6o00PeSVwjbYgTZw=)

| 特性         | HTTP/1.1                                             | HTTP/2                       |
| ------------ | ---------------------------------------------------- | ---------------------------- |
| 服务发现     | 通过DNS解析域名获得IP地址，默认80端口                | 与HTTP/1.1相同               |
| 底层连接形式 | TCP长连接（Keep Alive），可选用连接池                | 单一TCP连接，多路复用        |
| 传输内容     | 主要为字符串，包括Header和Body<br>Body常用JSON序列化 | 二进制帧，Header压缩         |
| 设计初衷     | 网页文本展示，后扩展到多媒体                         | 提高性能，解决HTTP/1.1的限制 |
| 数据冗余     | 较高，Header和Body中存在重复信息                     | 较低，Header压缩减少冗余     |
| 性能         | 相对较低，但通用性强                                 | 显著提升，特别是多请求场景   |
| 使用场景     | 公共互联网服务，需要广泛兼容性                       | 现代Web应用，需要高性能      |
| 多路复用     | 不支持                                               | 支持，单连接内多个并行请求   |
| 服务器推送   | 不支持                                               | 支持                         |
| 头部压缩     | 不支持                                               | 支持，HPACK算法              |
