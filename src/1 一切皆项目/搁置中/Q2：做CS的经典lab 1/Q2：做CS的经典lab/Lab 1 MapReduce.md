---
draw:
tags: []
title: Lab 1 MapReduce
date created: 2024-08-07
date modified: 2024-11-12
---

N 个人去一个有 m 个坑位的大公共卫生间做清洁，该如何调度？这个实验的目标是实现一个分布式的 MapReduce 系统，包括协调器和工作进程，并确保它能正确处理并行任务、容错等情况。

<!-- more -->

## 资料

[6.5840 Lab 1: MapReduce (mit.edu)](https://pdos.csail.mit.edu/6.824/labs/lab-mr.html)

## 思想

[MapReduce](MapReduce.md)

## 文件解读

```sql

(base) liuyishou@MacBook-Pro-3 src % tree
.
├── go.mod
├── go.sum
├── kvraft
│   ├── client.go
│   ├── common.go
│   ├── config.go
│   ├── server.go
│   └── test_test.go
├── kvsrv
│   ├── client.go
│   ├── common.go
│   ├── config.go
│   ├── server.go
│   └── test_test.go
├── labgob
│   ├── labgob.go
│   └── test_test.go
├── labrpc
│   ├── labrpc.go
│   └── test_test.go
├── main
│   ├── diskvd.go
│   ├── lockc.go
│   ├── lockd.go
│   ├── pg-being_ernest.txt          0 pg*.txt是需要被统计单词数的原文件
│   ├── pg-dorian_gray.txt
│   ├── pg-frankenstein.txt
│   ├── pg-grimm.txt
│   ├── pg-huckleberry_finn.txt
│   ├── pg-metamorphosis.txt
│   ├── pg-sherlock_holmes.txt
│   ├── pg-tom_sawyer.txt
│   ├── wc.so                        0 单词计数(Word Count)的MapReduce应用,它被编译成了一个Go插件。
│   ├── mrsequential.go              0 运行的是一个单体（单机）版本的 MapReduce。
│   ├── mr-out-0                     0 是示例程序`go run mrsequential.go wc.so pg*.txt`的输出结果。

│   ├── mrcoordinator.go             1 多 worker 版本的 mapReduce
│   ├── mrworker.go
│   ├── pbc.go
│   ├── pbd.go
│   ├── test-mr-many.sh
│   ├── viewd.go
│   └── test-mr.sh                   2 打分的脚本，用来测试你的作业的完成情况
├── models
│   └── kv.go
├── mr
│   ├── coordinator.go
│   ├── rpc.go
│   └── worker.go
├── mrapps
│   ├── crash.go
│   ├── early_exit.go
│   ├── indexer.go
│   ├── jobcount.go
│   ├── mtiming.go
│   ├── nocrash.go
│   ├── rtiming.go
│   └── wc.go
├── porcupine
│   ├── bitset.go
│   ├── checker.go
│   ├── model.go
│   ├── porcupine.go
│   └── visualization.go
├── raft
│   ├── config.go
│   ├── persister.go
│   ├── raft.go
│   ├── test_test.go
│   └── util.go
├── shardctrler
│   ├── client.go
│   ├── common.go
│   ├── config.go
│   ├── server.go
│   └── test_test.go
└── shardkv
    ├── client.go
    ├── common.go
    ├── config.go
    ├── server.go
    └── test_test.go

```

## how to run

这个实验是关于实现 MapReduce 系统的。以下是运行这个实验的基本步骤：

1. 准备环境：
    - 确保已安装 Go 语言环境。
    - 克隆实验代码仓库：
        `git clone git://g.csail.mit.edu/6.5840-golabs-2024 6.5840 cd 6.5840`
        
2. 构建 MapReduce 应用：进入 src/main 目录，构建词频统计应用：
    `cd src/main go build -buildmode=plugin../mrapps/wc.go`
    
3. 运行普通的顺序执行的版本（用于对比）：
    `rm mr-out* go run mrsequential.go wc.so pg*.txt`  
    输出将保存在 mr-out-0 文件中。
    
4. 运行分布式版本：
    - 在一个终端窗口运行协调器（coordinator）：
        `rm mr-out* go run mrcoordinator.go pg-*.txt`
    - 在一个或多个其他终端窗口运行工作进程（worker）：
        `go run mrworker.go wc.so`
        
5. 检查结果：当所有进程完成后，检查 mr-out-* 文件的输出。可以使用以下命令查看排序后的结果：
    `cat mr-out-* | sort | more`
    
6. 运行测试脚本：使用提供的测试脚本来验证实现：
    `bash test-mr.sh`
    
