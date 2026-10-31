---
uid: 7a616fc6-98cf-4518-8624-8c27954d27a0
date created: 2025-07-19
date modified: 2025-09-22
---
# Summary

1. **Triton GPU加速** (`test_rmsnorm.py`)
    - 自定义CUDA kernel实现
    - 块级并行计算优化
    - 内存访问模式优化
2. **分布式训练** (`test_ddp*.py`) [分布式数据并行](分布式数据并行.md)
    - 梯度桶化策略：减少通信开销
    - All-reduce同步：保证参数一致性
    - Gloo后端：CPU测试环境
3. **内存优化** (`test_sharded_optimizer.py`)
    - [优化器状态分片技术](优化器状态分片技术.md)存储
    - ZeRO-1级别优化实现
    - 多进程状态同步

# Cues

[[分布式数据并行]]



# Notes

- [测试驱动学习](#%E6%B5%8B%E8%AF%95%E9%A9%B1%E5%8A%A8%E5%AD%A6%E4%B9%A0)
	- [测试层次结构](#%E6%B5%8B%E8%AF%95%E5%B1%82%E6%AC%A1%E7%BB%93%E6%9E%84)
	- [测试重要性排序](#%E6%B5%8B%E8%AF%95%E9%87%8D%E8%A6%81%E6%80%A7%E6%8E%92%E5%BA%8F)
	- [当前通过情况](#%E5%BD%93%E5%89%8D%E9%80%9A%E8%BF%87%E6%83%85%E5%86%B5)
	- [Level 1: 原子级组件测试 (最小粒度)](#Level%201:%20%E5%8E%9F%E5%AD%90%E7%BA%A7%E7%BB%84%E4%BB%B6%E6%B5%8B%E8%AF%95%20(%E6%9C%80%E5%B0%8F%E7%B2%92%E5%BA%A6))
	- [Level 2: 组件级测试 (小型组合单元)](#Level%202:%20%E7%BB%84%E4%BB%B6%E7%BA%A7%E6%B5%8B%E8%AF%95%20(%E5%B0%8F%E5%9E%8B%E7%BB%84%E5%90%88%E5%8D%95%E5%85%83))
	- [Level 3: 模块级测试 (复杂组合组件)](#Level%203:%20%E6%A8%A1%E5%9D%97%E7%BA%A7%E6%B5%8B%E8%AF%95%20(%E5%A4%8D%E6%9D%82%E7%BB%84%E5%90%88%E7%BB%84%E4%BB%B6))
	- [Level 4: 系统级测试 (完整系统/管道)](#Level%204:%20%E7%B3%BB%E7%BB%9F%E7%BA%A7%E6%B5%8B%E8%AF%95%20(%E5%AE%8C%E6%95%B4%E7%B3%BB%E7%BB%9F/%E7%AE%A1%E9%81%93))
	- [关键技术点总结](#%E5%85%B3%E9%94%AE%E6%8A%80%E6%9C%AF%E7%82%B9%E6%80%BB%E7%BB%93)
- [与[A1 Basics](A1%20Basics.md)的关系](#%E4%B8%8E%5BA1%20Basics%5D(A1%2520Basics.md)%E7%9A%84%E5%85%B3%E7%B3%BB)
	- [常见疑问 Q&A](#%E5%B8%B8%E8%A7%81%E7%96%91%E9%97%AE%C2%A0Q&A)
	- [推荐做法](#%E6%8E%A8%E8%8D%90%E5%81%9A%E6%B3%95)
- [实现过程](#%E5%AE%9E%E7%8E%B0%E8%BF%87%E7%A8%8B)
	- [0. 总体目标——把模型“跑得快、跑得大、跑得省”](#0.%20%E6%80%BB%E4%BD%93%E7%9B%AE%E6%A0%87%E2%80%94%E2%80%94%E6%8A%8A%E6%A8%A1%E5%9E%8B%E2%80%9C%E8%B7%91%E5%BE%97%E5%BF%AB%E3%80%81%E8%B7%91%E5%BE%97%E5%A4%A7%E3%80%81%E8%B7%91%E5%BE%97%E7%9C%81%E2%80%9D)
	- [1. 单卡提速篇](#1.%20%E5%8D%95%E5%8D%A1%E6%8F%90%E9%80%9F%E7%AF%87)
		- [1.1 Profiling & Benchmarking —— 先找到“瓶颈螺丝”](#1.1%20Profiling%20&%20Benchmarking%20%E2%80%94%E2%80%94%20%E5%85%88%E6%89%BE%E5%88%B0%E2%80%9C%E7%93%B6%E9%A2%88%E8%9E%BA%E4%B8%9D%E2%80%9D)
		- [1.2 Mixed Precision —— 「半精度」= 更小 float、更多吞吐](#1.2%20Mixed%20Precision%20%E2%80%94%E2%80%94%20%E3%80%8C%E5%8D%8A%E7%B2%BE%E5%BA%A6%E3%80%8D=%20%E6%9B%B4%E5%B0%8F%20float%E3%80%81%E6%9B%B4%E5%A4%9A%E5%90%9E%E5%90%90)
		- [1.3 RMSNorm vs. LayerNorm —— 把「螺丝」换成定制件](#1.3%20RMSNorm%20vs.%20LayerNorm%20%E2%80%94%E2%80%94%20%E6%8A%8A%E3%80%8C%E8%9E%BA%E4%B8%9D%E3%80%8D%E6%8D%A2%E6%88%90%E5%AE%9A%E5%88%B6%E4%BB%B6)
	- [2. Triton Kernel 篇——把 Python 运算“熔进” GPU](#2.%20Triton%20Kernel%20%E7%AF%87%E2%80%94%E2%80%94%E6%8A%8A%20Python%20%E8%BF%90%E7%AE%97%E2%80%9C%E7%86%94%E8%BF%9B%E2%80%9D%20GPU)
		- [实施要点](#%E5%AE%9E%E6%96%BD%E8%A6%81%E7%82%B9)
	- [3. 分布式数据并行 (DDP) 篇](#3.%20%E5%88%86%E5%B8%83%E5%BC%8F%E6%95%B0%E6%8D%AE%E5%B9%B6%E8%A1%8C%20(DDP)%20%E7%AF%87)
		- [3.1 从 0 写一个“原始版 DDP”](#3.1%20%E4%BB%8E%200%20%E5%86%99%E4%B8%80%E4%B8%AA%E2%80%9C%E5%8E%9F%E5%A7%8B%E7%89%88%20DDP%E2%80%9D)
		- [3.2 两条提速线](#3.2%20%E4%B8%A4%E6%9D%A1%E6%8F%90%E9%80%9F%E7%BA%BF)
		- [3.3 多节点注意点](#3.3%20%E5%A4%9A%E8%8A%82%E7%82%B9%E6%B3%A8%E6%84%8F%E7%82%B9)
	- [4. Optimizer State Sharding (ZeRO Stage 1 简化版)](#4.%20Optimizer%20State%20Sharding%20(ZeRO%E2%80%AFStage%E2%80%AF1%E2%80%AF%E7%AE%80%E5%8C%96%E7%89%88))
	- [5. 交付物与建议](#5.%20%E4%BA%A4%E4%BB%98%E7%89%A9%E4%B8%8E%E5%BB%BA%E8%AE%AE)
		- [对 Java 背景同学的几条“踩坑预警”](#%E5%AF%B9%20Java%20%E8%83%8C%E6%99%AF%E5%90%8C%E5%AD%A6%E7%9A%84%E5%87%A0%E6%9D%A1%E2%80%9C%E8%B8%A9%E5%9D%91%E9%A2%84%E8%AD%A6%E2%80%9D)
	- [6. 学完 Lab 2 你将收获](#6.%20%E5%AD%A6%E5%AE%8C%20Lab%E2%80%AF2%20%E4%BD%A0%E5%B0%86%E6%94%B6%E8%8E%B7)

## 与[A1 Basics](A1%20Basics.md)的关系

Lab 2 的起点里 _应该_ 已经带着一份能跑通的 Lab 1 参考实现（staff solution），但你仍然要把 Lab 1 中的核心模块拷进去或重新整理，以便在 Lab 2 的系统级骨架里继续用。

| 目录                 | 主要角色                                            | 你要做的事                                                                                                                                             | 为什么要这样分                                                       |
| ------------------ | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| **cs336‑basics/**  | ✅ **Lab 1 完整参考实现**（或你自己写的、已通过所有 unit tests 的版本） | - 保留一份“最小可运行”代码，随时对照调试- 将来如果 Lab 2 改动出错，可以回退核对                                                                                                    | 把“数值正确的核心模型”单独封装，避免在 Lab 2 大改动时把 Lab 1 弄坏                     |
| **cs336‑systems/** | 🛠 **Lab 2 工作区**（训练管线、I/O、性能、分布式等系统层）| 1. **先复制/引用** Lab 1 里用得到的类（模型、优化器、tokenizer…）-<br>2. **再补充/重写** Lab 2 需要的新功能：  <br>• 数据流水线、BPE 训练   <br>• Checkpointing & CLI   <br>• 多 GPU、性能基准等 | 让你在“可依赖的模型”之上专注系统工程，不必从零再造 Transformer；同时容许你根据 Lab 2 需求重构代码结构 |
|                    |                                                 |                                                                                                                                                   |                                                               |

## 测试驱动学习

### 测试层次结构

- **系统优化测试** (`test_rmsnorm.py`)
    - 验证GPU加速计算正确性  [RMSNorm](RMSNorm.md)
- **分布式训练测试** (`test_ddp*.py`, `test_sharded_optimizer.py`)
    - 数据并行机制
    - 梯度同步策略
    - 优化器状态分片

### 测试重要性排序

1. 🔴 **最核心**: `test_rmsnorm.py` - GPU加速的基础，性能优化的关键
2. 🟠 **很重要**: `test_ddp.py` - 分布式训练的核心，没有这个无法多GPU训练
3. 🟡 **重要**: `test_sharded_optimizer.py` - 内存优化，大模型训练必需
4. 🟢 **次要**: `test_ddp_individual_parameters.py` - DDP的替代实现方式

### 当前通过情况

- ✅ **PyTorch实现**: 所有RMSNorm的PyTorch版本测试
- ⚠️ **GPU依赖**: Triton测试需要CUDA环境
- ✅ **分布式测试**: DDP和优化器分片测试


按照粒度从小到大排列的所有测试用例：

- **总测试文件数**: 4
- **总测试用例数**: 14
- **原子级测试**: 4个 (基础归一化操作)
- **组件级测试**: 4个 (前向/反向传播)
- **模块级测试**: 2个 (完整autograd功能)
- **系统级测试**: 4个 (分布式训练集成)

### Level 1: 原子级组件测试 (最小粒度)

| 测试文件              | 测试函数                                | 测试目的                              | 状态  |
|:---------------- |:---------------------------------- |:-------------------------------- |:-: |
| `test_rmsnorm.py` | `test_rmsnorm_forward_pass_pytorch` | 验证 [[RMSNorm]] 前向传播的[[PyTorch]]实现 |  ✅  |
| `test_rmsnorm.py` | `test_rmsnorm_forward_pass_triton`  | 验证 RMSNorm 前向传播的[[Triton]]加速实现    |  ✅  |
| `test_rmsnorm.py` | `test_rmsnorm_backward_x_pytorch`   | 验证对输入x的梯度计算(PyTorch)              |     |
| `test_rmsnorm.py` | `test_rmsnorm_backward_g_pytorch`   | 验证对缩放参数g的梯度计算(PyTorch)            |     |

### Level 2: 组件级测试 (小型组合单元)

| 测试文件                                | 测试函数                                                | 测试目的                                      | 状态  |
|:---------------------------------- |:-------------------------------------------------- |:---------------------------------------- |:-: |
| `test_rmsnorm.py`                   | `test_rmsnorm_backward_x_triton`                    | 验证Triton实现的输入梯度计算                         |     |
| `test_rmsnorm.py`                   | `test_rmsnorm_backward_g_triton`                    | 验证[[Triton]]实现的参数梯度计算                     |     |
| `test_sharded_optimizer.py`         | `_test_sharded_optimizer`                           | 验证[[优化器状态分片技术]]的单进程行为                     |  ✅  |
| `test_ddp_individual_parameters.py` | `_test_DistributedDataParallelIndividualParameters` | 验证逐参数同步的[分布式数据并行](分布式数据并行.md)实现 |  ✅  |

### Level 3: 模块级测试 (复杂组合组件)

| 测试文件              | 测试函数                                             | 测试目的                    | 状态  |
|:---------------- |:----------------------------------------------- |:---------------------- |:-: |
| `test_rmsnorm.py` | `test_rmsnorm_autograd_pytorch_forward_backward` | 验证完整的PyTorch [[自动微分]]集成 |     |
| `test_rmsnorm.py` | `test_rmsnorm_autograd_triton_forward_backward`  | 验证完整的Triton自动微分集成       |     |

### Level 4: 系统级测试 (完整系统/管道)

| 测试文件                                | 测试函数                                               | 测试目的                                                                                                  | 状态  |
|:---------------------------------- |:------------------------------------------------- |:---------------------------------------------------------------------------------------------------- |:-: |
| **分布式数据并行测试**                       |                                                    |                                                                                                       |     |
| `test_ddp.py`                       | `test_DistributedDataParallelCPU`                  | 验证[分布式数据并行](分布式数据并行.md)训练<br>- 测试梯度桶化策略(0.0016/0.0001/0.01 MB)<br>- 验证参数同步正确性<br>- 支持参数共享模型 |  ✅  |
| `test_ddp_individual_parameters.py` | `test_DistributedDataParallelIndividualParameters` | 验证无桶化的DDP实现<br>- 逐参数梯度同步<br>- 适用于小模型或调试                                                               |  ✅  |
| **优化器状态分片测试**                       |                                                    |                                                                                                       |     |
| `test_sharded_optimizer.py`         | `test_sharded_optimizer`                           | 验证[[ZeRO-1]] [优化器状态分片技术](优化器状态分片技术.md)<br>- AdamW优化器状态分布式存储<br>- 内存效率提升验证<br>- 10轮迭代数值一致性             |  ✅  |
| **参数化测试覆盖**                         |                                                    |                                                                                                       |     |
| -                                   | `ToyModel`测试                                       | 标准模型架构：FC1→[[ReLU]]→FC2→ReLU→FC3                                                                      |     |
| -                                   | `ToyModelWithTiedWeights`测试                        | 参数共享模型：FC1和FC3共享权重                                                                                    |     |

### 常见疑问 Q&A

|Q|A|
|---|---|
|**我能直接 import cs336‑basics 里的模块吗？**|可以，但课程通常建议把关键组件复制到 `cs336_systems/models/…`，再按系统层面的分包需求（例如 `io/`, `trainer/`）做适当重构，练习“工程化封装”。|
|**参考实现是不是 100 % 完整？**|通常只保证通过 Lab 1 的测试；到 Lab 2 可能还需你：• 改 I/O 接口（支持流式数据）• 加 Checkpoint Save/Load• 适配 AMP/FSDP 等新特性|
|**如果我 Lab 1 写得跟官方实现不一样？**|建议保留你自己的版本以防万一，但 Lab 2 项目最好以官方接口规范为准，避免在系统测试中因签名不一致报错。|
|**为什么不用 git submodule 直接引用？**|课程想让你体验“复制‑重构‑优化”的过程，顺带学会如何在大项目里整理依赖、拆分 package。|

### 推荐做法

1. **把 Lab 1 的 `model.py`, `optimizer.py`, `tokenizer/` 复制进 `cs336_systems/` 对应子包**。
2. **跑一遍 Lab 2 的已有 tests**，确认基础组件能通过，再开始写系统级新功能。
3. **所有改动先写集成测试**（训练 1‑2 step、保存‑加载、推理），确保不破原有行为。
4. **逐步替换成更工程化的结构**（参数文件、CLI 入口、日志系统等）。

这样既能利用老师给的“标准答案”加速，也能在 Lab 2 里练到代码重构与系统集成的能力。祝实验顺利!



## 实现过程
### 0. 总体目标——把模型“跑得快、跑得大、跑得省”

> **一句话**：Lab 2 要你把上一轮实现好的“小 Transformer”升级成一个在 **单卡高效 + 多卡并行 + 内存省** 的工程级版本。手册在第 1 页给出了四个核心实现项：
>
> 1. 基准测试脚本 2) 自定义 RMSNorm Triton 内核 3) 分布式数据并行 4) 优化器状态分片
>

---

### 1. 单卡提速篇

#### 1.1 Profiling & Benchmarking——先找到“瓶颈螺丝”

- **为什么要基准测试**  
    GPU 调用是异步的，`torch.matmul` 返回得很快，但真正计算还在卡上跑。必须在计时前后插一句 `torch.cuda.synchronize()` 才算数。
    
- **怎么测**
    
    - 用 `timeit` 记录 N 次前向 / 反向平均耗时
        
    - 手册第 2–4 页给出了五种模型尺寸（small→2.7B），以及 warm‑up 的写法
        
- **PyTorch Profiler**  
    像「体检报告」：会告诉你 ① 哪个 CUDA kernel 最耗时，② 每层各占多少。`record_function` 能把整段前向或反向包进来，方便总览。
    

> 🔧 **给 Java 工程师的类比**：  
> Baseline benchmarking 就像你在 JVM 里跑 JMH；Profiler 像 YourKit，先看热点再决定是换 HashMap 还是用并发包。

#### 1.2 Mixed Precision——「半精度」= 更小 float、更多吞吐

- A100 上 FP16/F32 理论算力约 312 TFLOPS: 19 TFLOPS。
    
- 用 `torch.autocast` 一键开混合精度，得配合 `GradScaler` 做 **loss scaling**，防止梯度因精度不足被截断。
    
- 手册在第 6 页举了 `ToyModel` 例子，说明 **LayerNorm 仍需 FP32**，因为求方差时对数值范围最敏感。
    

#### 1.3 RMSNorm vs. LayerNorm——把「螺丝」换成定制件

- RMSNorm 公式简单，没有减均值步骤，理论上更快。
    
- 手册让你写脚本对比 **PyTorch 自带 LayerNorm** 与 **自己写的 RMSNorm**，并在隐藏维度 1024→8192 逐级测。
    
- 结果一般是：隐藏维度越大，RMSNorm 速度优势越明显。
    

---

### 2. Triton Kernel 篇——把 Python 运算“熔进” GPU

> Triton = CUDA C 的“Python 宏”。你要写一个 **融合版 RMSNorm**：一次 kernel 里完成平方、求均值、归一化、乘权重。
> **Forward** 比较直观；**Backward** 需要手推雅可比向量积（手册第 12–14 页给了推导提示）。

#### 实施要点

1. **分配线程块**：每个程序实例负责一行（即一个 token 的隐藏向量）。
    
2. **重计算 (re‑compute)**：反向时别把前向中间值写回显存，反而重算，省内存 + 带宽。
    
3. **部分梯度缓冲**：∇g 需要 across‑row sum，可先写到临时矩阵，再 `torch.sum`。
    

> 🔧 **类比**：在 Java 世界里这是把一串 Stream 操作改写成单个 for 循环并用 SIMD；Triton 就是帮你写“SIMD + 并发”的那层汇编。

---

### 3. 分布式数据并行 (DDP) 篇

> **图示在第 19 页**：两台机器、各 4 GPU，展示了 global rank / local rank 的对应关系。

#### 3.1 从 0 写一个“原始版 DDP”

1. **初始化进程组**：`dist.init_process_group(backend="nccl")`
    
2. **把 batch 切 N 份**，各 GPU 前向 + 反向。
    
3. **逐参数 all‑reduce** 梯度，得到全局平均。
    
4. **各自做 optimizer.step()**——因为梯度同步了，参数还是一致的。
    

_问题_：参数多时 all‑reduce 调用次数 = 参数个数 → 大量 launch 开销。

#### 3.2 两条提速线

|方案|思路|难度|效果|
|---|---|---|---|
|**Flatten**|把所有梯度 flatten 成一个大张量，一次 all‑reduce|低|减少通信调用次数，但无法边算边传|
|**Overlap & Bucket**|把参数按「反向计算顺序」打成 N 个桶；某桶梯度齐了就 **异步 all‑reduce**，CPU 继续算下一层|中|计算和通信重叠，提升利用率|

> 🔧 **类比**：Flatten 像把多条 HTTP 请求合成一个 ZIP；Bucket + overlap 像 HTTP2 多路复用，边上传边下载。

#### 3.3 多节点注意点

- **Slurm / mpirun**: 设置 `MASTER_ADDR`, `MASTER_PORT`；避免用 `--gpus-per-task`，否则 NCCL 看不到整机拓扑。
    
- **带宽 vs. 延迟**：在单节点熟悉后，再测双节点，你会发现大张量（> 10 MB）时带宽占主导，小张量时延迟占主导。
    

---

### 4. Optimizer State Sharding (ZeRO Stage 1 简化版)

- AdamW 要为每个权重存两份状态 (m,v)，显存 ×3。
    
- **思路**：把参数分成 world_size 份；每张卡只保管自己份额的 m,v，并只给这部分参数做 `step()`；然后 **广播更新后的参数** 给其他卡。
    
- 结果：显存≈ 原来的 1/world_size + 参数本体，但多了一次广播通信。
    
- 手册第 28–29 页要求你实现 `ShardedOptimizer` 接口并量化 _前 / 后_ 峰值显存。
    

> 🔧 **和 ZeRO Stage 1 差别**：这里我们没有把 **梯度** 也分片；真正的 ZeRO‑DP 还会分片梯度与参数本体，通信模式更复杂。

---

### 5. 交付物与建议

|模块|你需要完成的核心代码|建议|
|---|---|---|
|**benchmarking_script.py**|标准计时 + profiler + 混合精度|每改一次 kernel 都跑，防回退|
|**rmsnorm_autograd_function_[pytorch|triton].py**|Forward & Backward|
|**ddp_[flat|bucketed].py**|Hook 注册 + 异步 all‑reduce|
|**sharded_optimizer.py**|继承 `torch.optim.Optimizer`|注意在 `add_param_group` 里重新分桶|

#### 对 Java 背景同学的几条“踩坑预警”

1. **GPU 空间 = JVM 堆**：小心“内存泄漏”——张量要么 `.detach()` 要么 `del` + `torch.cuda.empty_cache()`。
    
2. **异步语义** ≠ Java Future：NCCL 异步操作返回的是 **request handle**，别忘 `wait()`。
    
3. **类型坑**：FP16 ↔ FP32 互转易忘记；在 Triton 内显式 cast，省 NaN。
    
4. **整数除法**：Python 默认是真·除法，写 kernel 要 `//`。
    

---

### 6. 学完 Lab 2 你将收获

- **定位瓶颈 & 写自定义 [[GPU Kernel 内核]]** 的实战能力
    
- **理解并实现数据并行通信协议** 的全流程
    
- **显存账本**：知道模型 / 激活 / 优化器各占多少，以及如何 trade‑off
    
- 对日后做 **大模型推理加速 (Flash‑Attention、KV‑cache)** 和 **张量并行 / pipeline 并行** 打下基础
    

祝你愉快地把 Lab 2 跑通！有任何具体实现问题，随时再来聊。
