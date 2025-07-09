---
comment_id: 46d752cd
date created: 2024-06-05
date modified: 2025-01-23
draw: null
tags: []
title: TCP
---
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F01%2F23%2F16-14-37-aa6311aba36bf79a9845a6da3c62664e-202501231614289-d4c6c5.png)

1. 状态显示：
- 顶部状态栏显示关键的 TCP 参数：
    - RTO (Retransmission Timeout): 重传超时时间
    - 连续重传次数 (_conse_retrans)
    - 接收窗口大小 (_rwindow)

1. 消息展示：
- 使用类似微信的气泡样式展示消息
- 发送和接收的消息使用不同的颜色区分
- 展示了三次握手的过程 (SYN, SYN-ACK, ACK)

1. 底部状态栏：
- 显示序号相关信息：
    - 下一个发送序号 (_next_seqno)
    - 已确认序号 (_acknos)
    - 飞行中的字节数 (bytes_in_flight)

1. 交互设计：
- 输入框用于模拟数据发送
- 发送按钮触发数据传输
- 整体布局模仿微信的直观界面

这个设计将 TCP 的抽象概念可视化，帮助理解：

- 发送窗口的滑动过程
- 确认机制的工作方式
- 重传超时的处理
- 序号的递增关系

## 核心对象

stream reassembler 将**带索引的字节流碎片**重组成有序的字节流

## 发送方

`_outstanding` 用于跟踪所有已发送但还未被确认的TCP段

## 接收方

## 三次握手

1. TCP三次握手中的ACK标志位：
- 第一次：客户端发送 SYN
- 第二次：服务器回复 SYN+ACK
- 第三次：客户端发送 ACK

## 字节流服务

Bytestream在网络编程和TCP概念中通常称为"Byte Stream Service"（字节流服务）或"Octet Stream Service"（八位字节流服务）。

在TCP协议特性中有个重要概念：

- TCP提供"Stream-oriented Service"（面向流的服务）
- 相对于UDP的"Message-oriented Service"（面向消息的服务）

TCP的Byte Stream的主要特点：

1. Stream Abstraction (流抽象)：
- 把数据看作是连续的字节流
- 没有消息边界的概念
- 发送方写入多少字节，接收方就必须读取多少字节

1. In-Order Delivery (顺序传递)：
- 保证字节按照发送顺序到达
- 维护字节的序列号
- 处理乱序到达的数据

1. Reliability (可靠性)：
- 确保所有字节都能正确传递
- 使用确认机制确保数据完整性
- 丢失数据会被重传

CS144中的ByteStream就是在实现这种TCP的基本抽象，它是理解和实现TCP协议的基础。所以Lab 0从ByteStream开始，是为后续实现完整的TCP协议打基础。

## 序列号

在TCP中，wrap和unwrap操作与**序列号的回绕(Sequence Number Wrapping)**和**序列号空间(Sequence Number Space)**的概念相关。

具体来说：

1. wrap 操作：
- TCP协议中称为序列号回绕(Sequence Number Wrapping)
- 将64位的绝对序列号转换为32位的相对序列号
- 因为TCP头部字段只有32位来存储序列号

1. unwrap 操作：
- 在TCP中没有专门的术语，通常称为序列号重建(Sequence Number Reconstruction)
- 将32位的TCP序列号还原成64位的绝对序列号
- 用于处理序列号回绕问题

这个机制存在的原因：

- TCP序列号字段只有32位
- 高速网络中32位空间可能很快用完需要回绕
- 需要正确处理序列号回绕以维持传输的可靠性和有序性

举个例子：

- 如果发送速率是1Gbps
- 32位序列号空间约4GB
- 大约34秒就会用完需要回绕
- 因此需要wrap/unwrap机制来正确处理序列号

这就是为什么CS144要实现这两个函数，它们是TCP协议正确处理序列号空间的基础。

### ISN

 **TCP序列号不从零开始**：为了提高安全性和避免不同连接之间的混淆，TCP试图确保序列号不能被猜到，而且不太可能重复。因此，一个流的序列号不从零开始。流中的第一个序列号通常是一个随机的32位数字，称为初始序列号（ISN）。这是代表SYN（流的开始）的序列号。其余的序列号在这之后表现正常，例如，第一个字节的数据将有ISN+1的序列号，第二个字节的数据将有ISN+2，等等。

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

## 优雅关闭

用微信举个例子来解释：

1. 你和朋友聊天结束了，双方都说了"拜拜"
2. 关闭聊天的方式有两种：
    - 果断关闭：直接退出聊天界面
    - 优雅关闭：等待一会儿（看看对方是否还有话说），如果确实没有新消息了才关闭

这种机制的目的是：

1. 确保所有数据都完整传输了
2. 避免过早关闭导致丢失最后的几条消息
3. 但也不能一直等待，所以设置了超时时间（10 * 重传超时时间）

就像微信视频通话结束时：

- 双方都说了再见
- 但还会等待几秒钟
- 确认对方真的没有其他话要说
- 然后才真正挂断

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
