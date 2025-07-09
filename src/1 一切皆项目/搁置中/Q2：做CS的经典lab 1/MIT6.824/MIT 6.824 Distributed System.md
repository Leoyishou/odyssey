---
comment_id: c8301fbc
date created: 2024-07-11
date modified: 2025-04-08
draw: null
title: MIT 6.824 Distributed System
---
计算机硕士开题想基于MIT 6.824做一个分布式存储引擎，请问想法是否可行？- 知乎  
https://www.zhihu.com/question/53471701

Lab：

https://www.youtube.com/watch?v=IdCbMO0Ey9I&feature=youtu.be

学习分布式系统需要怎样的知识？- 量子位的回答 - 知乎  
https://www.zhihu.com/question/23645117/answer/1053242777

- LEC 1: Introduction
    - 主题: MapReduce
    - 准备: 阅读 [MapReduce](MapReduce.md) (2004)
    - 作业: [Lab 1 MapReduce](1%20一切皆项目/搁置中/Q2：做CS的经典lab%201/MIT6.824/Lab%201%20MapReduce.md)
- LEC 2: RPC and Threads
    - 示例: crawler.go, kv.go, vote
    - 准备: 完成在线 [Golang](Golang.md) 教程
- LEC 3: (因暴风雪取消)
    - 作业: [Lab 3 Key Value server](Lab%203%20Key%20Value%20server.md)
- LEC 4: Consistency and Linearizability
    - 准备: Linearizability Testing
- LEC 5: Go patterns (嘉宾讲座 - Russ Cox from Google/Go)
    - 准备: 阅读 The Go Programming Language and Environment
- LEC 6: GFS
    - 准备: 阅读 GFS (2003)
- LEC 7: Fault Tolerance: Raft (1)
    - 准备: 阅读 Raft (extended) (2014), 到第 5 节结束
- LEC 8: Fault Tolerance: Raft (2)
    - 准备: 阅读 Raft (extended) (2014), 第 7 节到结尾 (不包括第 6 节)
    - 作业: Final Project 分配
- LEC 9: Zookeeper
    - 准备: 阅读 ZooKeeper (2010)
- LEC 10: Q&A Lab 3A+B
    - 作业: [Lab 4: KV Raft](Lab%204:%20KV%20Raft)
- LEC 11: Verified Primary/Backup
    - 准备: 阅读 Grove (2023), 重点关注第 1, 2, 7 节
- LEC 12: Distributed Transactions
    - 准备: 阅读 6.033 Chapter 9, 仅 9.1.5, 9.1.6, 9.5.2, 9.5.3, 9.6.3 节
- LEC 13: Spanner
    - 准备: 阅读 Spanner (2012)
    - 作业: Lab 5: Sharded KV
- LEC 14: Chardonnay
    - 准备: 阅读 Chardonnay (2023)
- LEC 15: Optimistic Concurrency Control
    - 准备: 阅读 FaRM (2015)
- LEC 16: Amazon DynamoDB (虚拟讲座 by Doug Terry)
    - 准备: 阅读 DynamoDB (2022)
- LEC 17: Ray
    - 准备: 阅读 Ray (2021)
- LEC 18: Cache Consistency: Memcached at Facebook
    - 准备: 阅读 Memcached at Facebook (2013)
- LEC 19: AWS Lambda (虚拟讲座 by Marc Brooker)
    - 准备: 阅读 On-demand Container Loading (2023)
- LEC 20: Boki
    - 准备: 阅读 Boki (2021)
- LEC 21: Fork Consistency, SUNDR
    - 准备: 阅读 SUNDR (2004) (到 3.3.2 节结束)
- LEC 22: Byzantine Fault Tolerance
    - 准备: 阅读 Practical BFT (1999)
- LEC 23: Peer-to-peer: Bitcoin
    - 准备: 阅读 Bitcoin (2008) 和总结
- LEC 24: (取消)
- LEC 25: Project demos
