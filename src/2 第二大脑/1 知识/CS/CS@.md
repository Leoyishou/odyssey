---
draw: 
title: CS@
tags: []
date created: 2024-05-19
date modified: 2025-01-24
---

外表高大上，看着是宾利的壳，其实核心动力是几个老鼠在壳里面蹬三轮呢。只要你干一两年基本都懂了。

[胡适](胡适.md)说：怕什么真理无穷，**进一寸有进一寸的欢喜**。

<!-- more -->

[MIT 6.824 Distributed System](MIT%206.824%20Distributed%20System.md)  
[MIT 6.828 Operating System Engineering](MIT%206.828%20Operating%20System%20Engineering.md)  
[CS144 Computer Network](CS144%20Computer%20Network.md)  
[[UCB CS186 Introduction to Database System]]  
[[CS224n Natural Language Processing]]

## CS 心法

任何活儿，无论是存储、网络、[ai infra](https://www.zhihu.com/search?q=ai%20infra&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D)、编译器后端、cuda，等等，都非常简单。外表高大上，看着是宾利的壳，其实核心动力是几个老鼠在壳里面蹬三轮呢。只要你干一两年基本都懂了。

你能把活儿干出花来，能快速的超越自己的限制，最重要的是计算机各个领域的基本视野和概念。

你必须知道计算机科学几乎所有的领域基本的想法是什么。比如 tcp 怎么维护 [虚拟链接](https://www.zhihu.com/search?q=%E8%99%9A%E6%8B%9F%E9%93%BE%E6%8E%A5&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D) 传递信息，如何做微观的拥塞控制；编译器怎么 [parse](https://www.zhihu.com/search?q=parse&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D) 出来 ir，如何对 ir 优化；分布式算法如何协调工作；[流式计算](https://www.zhihu.com/search?q=%E6%B5%81%E5%BC%8F%E8%AE%A1%E7%AE%97&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D) 里面 unbounded data 怎么做 [window/trigger](https://www.zhihu.com/search?q=window%2Ftrigger&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D)；[机器学习](机器学习.md) 里面各种模型是如何拟合数据的，为什么需要不同的拟合方式；mysql 中事务怎么做，行锁怎么设计的；[linux kernel](https://www.zhihu.com/search?q=linux%20kernel&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D) 的进程调度怎么设计、[异步 io](https://www.zhihu.com/search?q=%E5%BC%82%E6%AD%A5%20io&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D) 怎么设计……

有了这些视野之后，你要把它们从具体的领域中抽象成概念。

千万不要把自己的精力放在 "tcp 包头几个字节都是什么 "、"[c++ 17](https://www.zhihu.com/search?q=c%2B%2B%2017&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D) 引入了什么 " 这种无聊的东西上。除非你决定把你的事业投入到上面。

概念就是 [计算机科学](https://www.zhihu.com/search?q=%E8%AE%A1%E7%AE%97%E6%9C%BA%E7%A7%91%E5%AD%A6&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D) 最精华的部分，你必须学会在不同的领域复用这些概念，这样你就是有 insight(洞察力、深刻理解) 的。

比如你去写一个最简单的业务，你一定会遇到事务、parse 配置规则、[限流控制](https://www.zhihu.com/search?q=%E9%99%90%E6%B5%81%E6%8E%A7%E5%88%B6&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3054501672%7D)、调度，是否有一些策略可以采样训练模型自动优化……你能否把学到的概念，结合实际应用起来，这个才是工程师的意义所在。

例如，理解了图论的核心概念后，你可能会在社交网络分析、交通路线优化、甚至是生物学中的蛋白质相互作用网络中看到相似的结构和问题。这种能力就体现了 insight。

什么时候能惊艳你的概念越来越少，到最后你看绝大部分东西的时候都能说出 " 就这 "，你就是合格的工程师了。
[缓存](缓存.md)

## 暴力

1. 深度学习的"矩阵乘法大法"

深度学习听起来很高大上，像是人工智能的终极形态。其实它的核心计算无非就是矩阵乘法的反复堆叠。训练神经网络时，你基本上就是在调整一堆参数（权重），让输出越来越接近目标值，而矩阵乘法正是实现这种参数调整的关键。某种程度上，深度学习就是"大规模线性代数运算的艺术"，并没有多少"智能"可言。要不怎么GPU（图形处理器）也能拿来训练神经网络呢？因为图形处理器的设计正好是用来加速矩阵运算的。

1. TCP协议中的"重传机制"

互联网最可靠的协议之一——TCP，被誉为保障网络传输稳定性的基石。它的魔力在于确保数据可靠传输，但其实它的实现手段相当粗暴。每当数据包发送出去，对方没有在规定时间内确认收到，TCP就会直接重传这份数据！没错，它就是这么简单粗暴——靠反复尝试，直到对方确实收到为止。用"拿不到确认我就一直发"的死磕态度，硬生生保障了网络的可靠性。

1. 图像去噪中的"中值滤波"

图像处理里有种叫做"中值滤波"的技术，用来去除图像中的噪点。听名字感觉很玄乎，其实就是在图像的每个像素点附近找一圈邻居，把这些邻居的颜色值按大小排序，然后取个中间值替换掉当前像素。虽然看起来像是复杂的数学运算，但本质上不过是"把周围人的意见平均一下"的笨方法，却能大幅度提升图像的质量。

1. AI游戏玩得好靠"暴力穷举"

看过AI击败人类顶级围棋选手的新闻吗？很多AI其实是在海量的计算资源上靠"暴力穷举"找到了"完美解法"。例如蒙特卡洛树搜索，就是先模拟大量随机的下棋路径，从中统计哪些路径胜率更高。听起来是"人工智能"的成果，但更像是"体力活"——用海量的数据堆出来的成功率。AI表面上是高手，背后却是用无数机器一起"傻干"。

1. 冷启动技术——缓存预热

互联网公司总会说"优化了冷启动时间"，让应用打开速度更快。看起来很高级的样子，实质上很多时候就是靠"缓存预热"。开发者提前把用户最可能需要的数据预先加载进内存，让它随时待命。当用户真正启动应用时，数据已经在那里等着了，就给人一种"启动超快"的错觉。相当于先悄悄做了准备工作，表面上看起来像"技术升级"。

1. 黑客攻击中的"字典攻击"

很多黑客攻击听起来高深莫测，比如破解密码的技术。但最常见的"字典攻击"其实特别直接，就是准备好一堆常见的密码，像"123456"，"password"，"qwerty"等，逐个尝试登录。虽然听起来像是初级黑客才会用的方法，却依然有很多成功的案例。人们太容易用弱密码，所以这种简单"蛮力破解"往往比复杂算法还要有效。

这些技术之所以高端，是因为它们在看似简单的原理上做了极致的优化和应用，最终达到令人印象深刻的效果。可以说，这些暴力原理+智慧巧思的组合才是真正的魔法所在。

### CS 之路

[北大CS自学指南 (csdiy.wiki)](https://csdiy.wiki/%E5%A5%BD%E4%B9%A6%E6%8E%A8%E8%8D%90/)

| 主题                | 子主题                                         |     |
| ----------------- | ------------------------------------------- | --- |
| 数学进阶              | - <br>- <br>- <br>- <br>- <br>-             |     |
| 编程入门              | - <br>- <br>- <br>- <br>- <br>- <br>- <br>- |     |
| 电子基础              | - <br>- <br>-                               |     |
| 数据结构与算法           | - <br>- <br>- <br>- <br>-                   |     |
| [软件工程@](软件工程@.md) | - <br>- <br>-                               |     |
| 计算机系统基础           | - <br>-                                     |     |
| 体系结构              | - <br>- <br>- <br>-                         |     |
| 操作系统              | - <br>- <br>- <br>-                         |     |
| 并行与分布式系统          | - <br>-                                     |     |
| 计算机系统安全           | - <br>- <br>- <br>- <br>- <br>-             |     |
| 计算机网络             | - <br>- <br>-                               |     |
| 数据库系统             | - <br>- <br>- <br>- <br>-                   |     |
| 编译原理              | - <br>- <br>- <br>-                         |     |
| 编程语言设计与分析         | - <br>- <br>- <br>-                         |     |
| 计算机图形学            | - <br>- <br>- <br>- <br>- <br>-             |     |
| Web开发             | - <br>- <br>- <br>-                         |     |
| 数据科学              | -                                           |     |
| 人工智能              | - <br>-                                     |     |
| 机器学习              | - <br>- <br>-                               |     |
| 机器学习系统            | - <br>- <br>-                               |     |
| 深度学习              | - <br>- <br>- <br>- <br>- <br>- <br>-       |     |
| 机器学习进阶            | - <br>- <br>- <br>- <br>-                   |     |

计算机基础学习顺序是什么？- 编程指北的回答 - 知乎  
https://www.zhihu.com/question/473267862/answer/2009648056

如果让你重新开始学计算机，你的学习路线会怎么选择? - 知乎  
https://www.zhihu.com/question/492545174/answer/2482876550

### 思维模型

计算机思维对我个人生活和做事影响还是蛮大的，编程能力最终的体现，仍然是计算思维的运用的能力，关于计算思维有很多类，下面是我控制chatgpt，整理出一些我觉得比较重要的思维方法，其中我认为非常典型的，在我看来包括递归、迭代、并行、状态机、缓存、贪心、分层、抽象

1. 递归思维（Recursive Thinking）递归是一种解决问题的方法，特别适用于自相似结构或问题的分解。递归思维强调将问题简化为更小的同类问题，直到达到可以直接求解的基础情况。
2. 迭代思维（Iterative Thinking）通过重复执行一个步骤序列来逼近解决方案。迭代思维在循环和优化算法中应用广泛，允许逐步改进或寻找最优解。
3. 并行思维（Parallel Thinking）这是对多个问题同时处理的思考方式，强调任务分割、同步和协调。并行思维在多线程、多进程或分布式计算中非常重要。
4. 状态机思维（State Machine Thinking）用于处理有状态的系统或问题，将问题视为一系列状态转换的过程。常用于控制系统、协议设计和游戏编程等领域。
5. 缓存思维（Caching Thinking）缓存思维关注如何通过存储中间结果，避免重复计算或提高效率。在大规模系统中，缓存优化可以大大提升性能。
6. 贪心思维（Greedy Thinking）在每一步都选择当前最优解，期望最终获得全局最优解。虽然不总能找到最优解，但在某些场景下（如贪心算法），这种思维是高效的。
7. 图论思维（Graph Theory Thinking）在面对节点和边的关系时，使用图论思维可以帮助解决网络、路径规划、拓扑结构等问题。广泛用于社交网络分析、路径优化和电路设计。
8. 模糊思维（Fuzzy Thinking）计算机科学中的模糊逻辑用于处理不确定性问题，这种思维方式可以帮助在信息不完整或模糊的情况下作出合理的判断。
9. 最优化思维（Optimization Thinking）最优化思维着眼于在有限资源下找到最优或次优的解决方案。这在资源分配、路径选择等问题中非常关键。
10. 局部最优思维（Local Optimization Thinking）在面对复杂问题时，局部最优思维尝试先在局部区域找到较好的解决方案，然后逐步扩展或组合以求全局最优。这在爬山算法、局部搜索算法中尤为常见。
11. 分层思维（Layered Thinking）分层思维的本质是将复杂的系统或问题按照功能、结构或逻辑划分为不同层次，每一层只关注其具体职责并与相邻层次进行交互。这种思维方式让问题更易于管理、维护和扩展。12. 抽象思维（Abstraction Thinking）抽象思维是忽略细节，专注于事物的本质特征和核心功能的思维方式。它通过提炼出问题的核心要素，屏蔽复杂实现，使得问题更加简洁易解。
