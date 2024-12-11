---
draw:
tags: []
title: Lab 1 MapReduce
date created: 2024-08-07
date modified: 2024-11-12
---

N 个人去一个有 m 个坑位的大公共卫生间做清洁，该如何调度？这个实验的目标是实现一个分布式的 MapReduce 系统，包括协调器和工作进程，并确保它能正确处理并行任务、容错等情况。

<!-- more -->

## 运行
```bash
rm mr-out*
go run -race mrcoordinator.go pg-*.txt
go run -race mrworker.go wc.so
cat mr-out-* | sort | more
```

下面是典型的运行与测试 Lab 1 的步骤（假设你已经根据指导编写好 `mr/coordinator.go`, `mr/worker.go` 并有对应的代码实现）：

1. **编译插件（应用代码）**  
    首先需要编译你要运行的 MapReduce 应用插件（比如 word count 程序）。进入 `6.824/src/main` 目录，执行：
    
    ```bash
    cd ~/6.824/src/main
    go build -race -buildmode=plugin ../mrapps/wc.go
    ```
    
    成功后会生成 `wc.so` 文件。
    
2. **清理旧的输出文件**  
    运行前先确保没有残留的输出文件：
    
    ```bash
    rm mr-out*
    ```
    
3. **启动 coordinator**  
    在同一个目录下运行 `mrcoordinator.go`，并传入输入文件列表（比如 `pg-xxx.txt` 文件集）：
    
    ```bash
    go run -race mrcoordinator.go pg-*.txt
    ```
    
    该命令会启动一个 coordinator 进程，并监听来自 worker 的 RPC 请求。
    
4. **启动一个或多个 worker**  
    在另一个终端窗口中，同样在 `src/main` 目录下，启动 worker 进程，并加载你之前编译好的插件 `wc.so`：
    
    ```bash
    go run -race mrworker.go wc.so
    ```
    
    你可以多开几个终端窗口再运行数个相同命令，以模拟多个并发 worker。如果你的实现正确，coordinator 会分配任务给这些 worker，它们会处理完任务并写出最终的 `mr-out-X` 文件。
    
5. **查看输出结果**  
    当任务全部完成后（coordinator 进程会自动退出），你可以查看输出结果：
    
    ```bash
    cat mr-out-* | sort | more
    ```
    
    结果应与 `mrsequential.go` 单机版的输出一致。
    
6. **运行测试脚本**（可选）  
    为了验证你的实现正确性，你还可以使用官方提供的测试脚本 `test-mr.sh`：
    
    ```bash
    bash test-mr.sh
    ```
    
    此脚本会依次测试 word-count、indexer、并行度测试以及崩溃恢复测试。如果全部通过，会显示：
    
    ```
    *** PASSED ALL TESTS
    ```
    

**总结**：  
要取得实际执行的效果，你需要先编译插件，然后先启动 coordinator（给出输入文件），再启动 worker（加载插件），让 worker 请求并执行任务。最后，查看 `mr-out-*` 文件或运行官方测试脚本来验证结果。

## 资料

[6.5840 Lab 1: MapReduce (mit.edu)](https://pdos.csail.mit.edu/6.824/labs/lab-mr.html)

## 思想

[MapReduce](MapReduce.md)

## 入参

一些文章

```
.
├── pg-being_ernest.txt
├── pg-dorian_gray.txt
├── pg-frankenstein.txt
├── pg-grimm.txt
├── pg-huckleberry_finn.txt
├── pg-metamorphosis.txt
├── pg-sherlock_holmes.txt
├── pg-tom_sawyer.txt
```

## 处理

协调器和工作者的"main"例程位于 main/mrcoordinator.go 和 main/mrworker.go 中;不要更改这些文件。您应该将您的实现放在 mr/coordinator.go 、 mr/worker.go 和 mr/rpc.go 中。

## 出参

这些文章里单词的出现次数


## 代码流程

让我按照执行顺序来解释 MapReduce 的整个流程：

1. **初始化阶段**:
```go
// 1. Coordinator 启动并初始化
func MakeCoordinator(files []string, nReduce int) *Coordinator {
    c := Coordinator{
        files:       files,    // 输入文件列表
        nReduce:     nReduce,  // reduce任务数量
        nMap:        len(files), // map任务数量
        // ... 初始化各种channel
    }
    c.server()         // 启动RPC服务
    go c.schedule()    // 启动调度器
    return &c
}
```

2. **Map 阶段**:
```go
// 2.1 Worker不断发送心跳请求任务
func Worker(mapF func(string, string) []KeyValue, reduceF func(string, []string) string) {
    for {
        response := doHeartbeat()  // 请求任务
        switch response.JobType {
        case MapJob:
            doMapTask(mapF, response)  // 执行Map任务
        // ...
        }
    }
}

// 2.2 执行Map任务
func doMapTask(mapF func(string, string) []KeyValue, response *HeartbeatResponse) {
    // a. 读取输入文件
    content, _ := ioutil.ReadAll(file)
    
    // b. 调用用户定义的Map函数(wc.go中的Map)
    kva := mapF(fileName, string(content))
    
    // c. 按照key的hash值将结果分成nReduce份
    intermediates := make([][]KeyValue, response.NReduce)
    for _, kv := range kva {
        index := iHash(kv.Key) % response.NReduce
        intermediates[index] = append(intermediates[index], kv)
    }
    
    // d. 将中间结果写入文件
    // 每个Map任务会生成nReduce个中间文件
}
```

3. **Reduce 阶段**:
```go
// 3.1 所有Map任务完成后，Coordinator切换到Reduce阶段
func (c *Coordinator) schedule() {
    // ... Map阶段完成后
    c.initReducePhase()
}

// 3.2 Worker执行Reduce任务
func doReduceTask(reduceF func(string, []string) string, response *HeartbeatResponse) {
    // a. 读取所有相关的中间文件
    for i := 0; i < response.NMap; i++ {
        // 读取每个Map任务为当前Reduce任务生成的中间文件
    }
    
    // b. 按key归组
    results := make(map[string][]string)
    for _, kv := range kva {
        results[kv.Key] = append(results[kv.Key], kv.Value)
    }
    
    // c. 对每个key调用用户定义的Reduce函数(wc.go中的Reduce)
    for key, values := range results {
        output := reduceF(key, values)
        // 写入最终结果
    }
}
```

4. **任务协调**:
```go
// Coordinator通过心跳机制分配任务
func (c *Coordinator) selectTask(response *HeartbeatResponse) bool {
    // 1. 检查是否有空闲任务
    // 2. 检查是否有超时需要重新执行的任务
    // 3. 分配任务给Worker
    // 4. 检查是否所有任务都完成
}
```

关键流程：
1. Coordinator 初始化并加载输入文件
2. Worker 通过心跳获取任务
3. Map 任务处理单个输入文件，产生中间文件
4. Reduce 任务处理所有相关的中间文件，生成最终输出
5. 整个过程通过 RPC 进行通信和协调

这个实现类似于 Google 的 MapReduce 论文，但简化了一些。


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

