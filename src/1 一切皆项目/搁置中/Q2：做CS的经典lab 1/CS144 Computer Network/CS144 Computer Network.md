---
date created: 2024-09-21
date modified: 2025-07-10
uid: 3bfe53ea-8831-40c9-a306-1c1656aade5c
---
## 环境部署

Ubuntu 18.04 [BYO Linux installation (stanford.edu)](https://stanford.edu/class/cs144/vm_howto/vm-howto-byo.html)  
[CS144Winter2024--Chapter0--wsl2+docker环境搭建_cs144环境配置-CSDN博客](https://blog.csdn.net/weixin_73074012/article/details/135847082)

[2021 CS144 实验笔记 | 沧海月明 (inlighting.org)](https://www.inlighting.org/archives/2021-cs144-notes)

用 Clion 里的 remote 开发

## 参考资料

CS 自学指南  
https://github.com/PKUFlyingPig/cs-self-learning  
官网  
https://www.scs.stanford.edu/10au-cs144/  
小林 coding  
https://xiaolincoding.com/network/  
lab 思路  
[CS144 Lab0翻译 | Doraemonzzz](https://doraemonzzz.com/2022/01/30/2022-1-30-CS144-Lab0%E7%BF%BB%E8%AF%91/)  
https://www.cnblogs.com/kangyupl/p/stanford_cs144_labs.html  
[CS 144: Introduction to Computer Networking](https://cs144.github.io/)  
[CS144计算机网络 Lab0 | Kiprey's Blog](https://kiprey.github.io/2021/11/cs144-lab0/)  
[【计算机网络】Stanford CS144 Lab Assignments 学习笔记 - 康宇PL - 博客园 (cnblogs.com)](https://www.cnblogs.com/kangyupl/p/stanford_cs144_labs.html)  
lab 答案  
https://github.com/PKUFlyingPig/CS144-Computer-Network

## Lab

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F01%2F23%2F17-01-10-fe2b3215855e029d9038a0a0d12e3674-202501231701193-9c99f1.png)

|       | Lab                               | Tasks                                                                        | 认知                                       |
| ----- | --------------------------------- | ---------------------------------------------------------------------------- | ---------------------------------------- |
|       | [[Lab0 ByteStream]]               | • 实现字节流(ByteStream)<br>• 熟悉环境设置和基础工具<br>• 主要是热身练习，为后续实验打基础                   | ByteStream的本质是一个 char 的deque 的 service 层 |
| TCP 层 | [[数据流重组 Lab1 Stream Reassembler]] | • 实现TCP的流重组器(StreamReassembler)<br>• 处理乱序到达的数据并重组为有序字节流<br>• 模拟TCP如何处理失序的数据包 |                                          |
|       | [[Lab2 TCP Receiver]]             | • 实现TCP接收端<br>• 处理序列号(Sequence Numbers)<br>• 管理接收窗口(Receive Window)          |                                          |
|       | [[Lab3 TCP Sender]]               | • 实现TCP发送端<br>• 实现重传机制<br>• 实现流量控制                                           |                                          |
|       | [[Lab4 TCP Connection]]           | • 将之前的Sender和Receiver组合<br>• 实现完整的TCP连接<br>• 处理连接的建立和终止                      |                                          |
| IP 层  | [[Lab5 Network Interface]]        | • 实现网络接口层<br>• ARP (Address Resolution Protocol)协议<br>• 以太网帧的处理              |                                          |
|       | [[Lab6 Router]]                   | • 实现路由器的核心功能<br>• 路由表的查找和维护<br>• IP数据包的转发                                    |                                          |
| 额外    | 研究不过的 case，尝试 gdb 做 debug         |                                                                              |                                          |

## Shell 命令

```shell
# 1. 首先，回到项目根目录
cd /Users/liuyishou/usr/projects/CS144-Computer-Network

# 2. 创建（如果不存在）并进入 build 目录
mkdir -p build
cd build

# 3. 运行 cmake
cmake ..

# 4. 编译项目
make format (使编码风格正常化)
make        (确保代码可以编译)

# 5. 测试
make check_lab0 (确保自动测试通过)


# 5. 运行 webget
./apps/webget stanford.edu /class/cs144

```

## 思考

1. 做 lab 的过程和工作中开发需求很像，[TCP](TCP.md)、[IP](IP.md)的理论就像产品经理写的 PRD，lab 代码就像我们开发的需求代码。网上各种网络方面的技术文章就像其他开发人员做完需求写的分享，看他们的分享远不如自己做一遍需求理解的深刻。
2. 完成 Lab 的过程中，一开始可能缺乏语言基础或者不熟悉这种学习模式，这时候，深入理解 git 的各种版本切换就很重要，找一份优质 lab 作业，穿梭在他各个完成阶段中抄作业，就像学霸陪自己学习一样。等基础强化，熟悉模式后可以自己再尝试独立完成一些。
3. 工作原因，不一定能短期做完一套 lab，这时候代码的原子性、健壮性很重要，最好每次做完一个 lab 都把测试用例全部跑过，这样每个小阶段都能有一个清晰的起点和终点。
4. orbstack 演示

To succeed in the labs, you'll need to be comfortable with C++ and debugging tools1. CS144 is a demanding course that requires strong programming skills. If you're struggling with C++, consider reviewing the basics or seeking help from the TAs.

It's also important to remember that the lab assignments are not just specifications to be followed blindly7. They require critical thinking and problem-solving skills. You'll need to understand the concepts behind the code and apply them to different scenarios.

When working on the labs, pay close attention to the different types of sequence numbers used in TCP8. These sequence numbers are crucial for implementing TCP correctly and ensuring reliable data transmission. You'll also need to familiarize yourself with the tools used in the labs, such as the Minnow library4. These tools provide a framework for your implementations and simplify the development process.

Understanding the concept of "bytes in flight" is essential for grasping TCP flow control and congestion control9. This concept refers to the amount of data that has been sent but not yet acknowledged. Managing bytes in flight is crucial for preventing network congestion and ensuring efficient data transfer.

Finally, pay attention to the endianness of data in network programming10. Endianness refers to the order in which bytes are stored in memory. Different computer architectures may use different endianness, and it's crucial to handle this correctly when sending and receiving data over the network.

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F23%2F11-51-40-d83ae1e55a6a6ef17e6e6b169e730006-202409231151421-2ce186.png)
