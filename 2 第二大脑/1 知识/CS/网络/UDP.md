---
draw:
tags: []
title: UDP
date created: 2024-09-23
date modified: 2024-11-12
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F23%2F19-21-44-078f6caf9c31fe593b5afbab70dab4ae-202409231921831-b11d0c.png)

TCP 是⾯向连接的，连接是双方达成三个信息的共识。
1、彼此知道Socket：IP 地址+端⼝号  
2、序列号：保证顺序和可靠，是面向字节流的  
3、窗⼝⼤⼩：⽤来做拥塞控制和流量控制

UDP 是不需要连接，所以  
1、可以单播、多播、广播  
2、UDP是面向应用层报文的，是有边界的，但可能会丢包和乱序。
3、没有拥塞控制和流量控制，即使⽹络⾮常拥堵了，也不会影响 UDP 的发送速率

应用场景：
TCP：FTP⽂件传输、HTTP / HTTPS  
UDP：一对多--广播，不可靠--实时的视频音频、DNS
