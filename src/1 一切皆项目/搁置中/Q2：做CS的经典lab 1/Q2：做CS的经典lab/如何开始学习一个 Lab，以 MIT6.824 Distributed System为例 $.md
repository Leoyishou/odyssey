---
draw:
tags: []
title: 如何开始学习一个 Lab，以 MIT6.824 Distributed System为例 $
date created: 2024-07-22
date modified: 2024-11-12
---

 [建立直觉](建立直觉)与代码落地相结合，尝试将[使用 “隐喻” 的方式帮你建立对 Raft 的直觉 | 木鸟杂记 (qtmuniao.com)](https://www.qtmuniao.com/2023/11/15/raft-explain/)中的故事作为注释标记到代码实现上。

<!-- more -->

## 前言

- 前置知识只有 [Git](Git.md) 一个，尤其是用好 Jetbrain 的 GUI
- 找到靠谱答案，去 [解耦学习](1%20一切皆项目/搁置中/Q2：做CS的经典lab%201/Q2：做CS的经典lab/解耦学习.md)

## 前期准备

思想性的，比如代码的实现思路，可以参考这个清华爷的笔记

- [6.824 分布式系统课程学习总结 - 谭新宇的博客 (tanxinyu.work)](https://tanxinyu.work/6-824/)

流程性的，比如每个 lab 需要哪些命令 run 啥的，都可以直接把官方文档丢给 AI，让他介绍

- ![image.png|500](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F22%2F20-44-20-f0f052bfa8ba4a920e9ae1b1a2a03da1-20240722204419-0473d4.png)
- [Claude](https://claude.ai/new)

语法性的，直接问 AI，然后让从 [Java@](Java@.md) 的角度去解释  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F22%2F21-15-45-bf9807da629dd1f451db91d4e59d154d-20240722211544-376838.png)

## 实践

1. 把 [github](2%20第二大脑/1%20节点/CS/编程语言/Go/github.md) 题解仓库克隆到本地
2. check out 到对应的 revision  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F22%2F23-03-42-238e1133e08385f20b438501eadec517-20240722230341-38ec0d.png)
3. 抄袭增量部分的代码，遇到不懂的问 [claude](2%20第二大脑/2%20飞轮/0%20工具系统/AI/claude.md) 这相当于 [Java@](Java@.md) 里面的啥
4. 琢磨这种解决方案背后的思想，尝试将[使用 “隐喻” 的方式帮你建立对 Raft 的直觉 | 木鸟杂记 (qtmuniao.com)](https://www.qtmuniao.com/2023/11/15/raft-explain/)中的故事作为注释标记到代码实现上。

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F24%2F23-53-14-ee45868ab8eddcb499b23b3b5dd3ecab-20240724235311-8c3a82.png)

通过这种抄作业的方式

- 一边学习语法，
- 一边从主方法开始，一步步把标红的方法补全，抄一段大概就明白这段在干嘛，然后加上关键性的梳理性的注释。慢慢就把整个项目吃透了

另外，也可以把好的文档给[claude](2%20第二大脑/2%20飞轮/0%20工具系统/AI/claude.md)，让他结合代码帮忙回答一些问题  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F08%2F14%2F12-02-28-77a02d3410d42a1641b69e8767c923e1-202408141202804-c40941.png)

## 课程视频部分

参考[如何畅游英语油管$](如何畅游英语油管$.md)

## 建立直觉与代码逻辑相结合

[一文搞懂Raft算法 - 割肉机 - 博客园 (cnblogs.com)](https://www.cnblogs.com/williamjie/p/11137140.html)
