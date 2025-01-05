---
draw:
title: TCP
tags: []
date created: 2024-06-05
date modified: 2025-01-02
---

序列号：在建立连接时由计算机生成的随机数作为其初始值，通过 SYN包传给接收端主机，每发送一次数据，就「累加」一次该「数据字节数」的大小。用来解决网络包乱序问题。

确认应答号：指下一次「期望」收到的数据的序列号，发送端收到这个确认应答以后可以认为在这个序号以前的数据都已经被正常接收。用来解决丢包的问题。

控制位：

- ACK：Acknowledgment 该位为 1时，「确认应答」的字段变为有效，TCP 规定除了最初建立连接时的 SYN 包之外该位必须设置为1。
- RST：：Reset 该位为 1 时，表示 TCP 连接中出现异常必须强制断开连接。
- SYN：Synchronize 该位为 1 时，表示希望建立连接，并在其「序列号」的字段进行序列号初始值的设定。
- FIN：Finish 该位为 1 时，表示今后不会再有数据发送，希望断开连接。当通信结束希望断开连接时，通信双方的主机之间就可以相互交换 FIN 位为1的TCP段。
- ACK: Acknowledgment (确认)
- RST: Reset (重置)
- SYN: Synchronize (同步)
- FIN: Finish (结束)

## 本质

TCP连接是双方达成三个信息的共识。
1、Socket：由 IP 地址和端⼝号组成  
2、序列号：⽤来解决乱序问题等  
3、窗⼝⼤⼩：⽤来做流量控制

## 特性

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F21%2F18-07-27-77585d31bf89152811dccd757ea50d22-202409211807101-a218e9.png)

### 面向连接的

三次握手、三次挥手

TCP连接中，客户端突然崩了，服务端怎么办？
有一个询问机制，如果距离上次收到客户端消息超过两个小时，服务器会向客户端发探测报文段，发十次，间隔75秒，还没反应就关闭连接

### 可靠的

[可靠传输](可靠传输.md)  
**使命必达**，无差错、不丢失、不重复且有序。

1、分组 + 确认机制 + 重传（发现丢了 | 超时重传）+ 丢弃 + 校验和  
2、实际的发送窗口取拥塞窗口和发送窗口的最小值  
	- [拥塞控制](拥塞控制.md) *网不好的时候说慢点* 发送端有个拥塞窗口  
	- 流量控制 *说太多接收端脑子记不住*，那就说慢点，靠滑动窗口实现，发送窗口等于接收窗口

### 基于字节流的 / 无标点符号的

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F21%2F18-08-37-909c9065f591e7c5809e7c8ac0e9d980-202409211808895-647087.png)

粘包：

1. 你写了三封信给朋友。
2. 邮局为了省钱,把这三封信放在一个大信封里一起寄出。
3. 你的朋友收到这个大信封,但不知道里面actually有几封信。
在计算机网络中:
- 你写的信就是你要发送的数据。
- 邮局就是TCP协议。
- 大信封就是TCP数据包。

匀速说话，没有顿挫，正因为这个没有任何边界的特点，所以当我们选择使用 TCP发送"夏洛"和"特烦恼"的时候，接收端收到的就是"夏洛特烦恼"，这时候接收端没发区分你是想要表达"夏洛"＋"特烦恼"还是"夏洛特"＋"烦恼"。

解决方法通常有:

1. 固定长度:每条消息都一样长。
2. 特殊分隔符:在每条消息之间加一个特殊符号。
3. 头部标记:在每条消息前加上一个"头",说明这条消息有多长。

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F23%2F11-50-33-db39c931fae1dfc92061fb4cb67345d0-202409231150613-e38346.png)  
TCP  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F23%2F19-21-12-f4544ff73dbc03500d76aa8a298ae026-202409231921874-3e7688.png)  
UDP  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F23%2F19-22-28-078f6caf9c31fe593b5afbab70dab4ae-202409231922180-80c814.png)
