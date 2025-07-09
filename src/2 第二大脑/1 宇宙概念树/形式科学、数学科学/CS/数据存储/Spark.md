---
comment_id: 3fadad0e
date created: 2025-02-24
date modified: 2025-02-24
draw: null
title: Spark
---
Spark：更快、更灵活的干活工具

[[MapReduce]] 虽然能干活，但有点慢，因为它每次计算完都要把结果写回硬盘，再读出来做下一步。Spark 就厉害了，它是个更快的计算引擎，把数据尽量放在内存里跑，速度比 MapReduce 快很多。

Spark 不光快，还灵活。它能干批处理（一次处理一大堆数据）、流处理（实时处理流水一样的数据）、甚至机器学习。Spark 可以跟 Hadoop 搭档，用 HDFS 存数据，但它自己也能独立跑，不一定非要靠 Hadoop。

比喻一下，Spark 就像一台新款跑车，速度快、功能多，而 MapReduce 是老款货车，稳但慢。
