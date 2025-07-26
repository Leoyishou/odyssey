---
date created: 2024-12-16
date modified: 2025-07-25
uid: 016c8fd1-d449-40f9-9761-132df63789ba
---
# UC Berkeley CS186 （Introduction to Database Systems）主要 Lab / Project 一览

| Lab / Project                            | 主题                       | 关键任务 & 实践点                                                                                             | 收获技能                               |
| ---------------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------ | ---------------------------------- |
| **Project 1：SQL on SQLite**              | 用 Lahman ⚾ 棒球数据集写 SQL 查询 | - 在 `proj1.sql` 中完成 4 组查询：基础筛选、名人堂院校、Sabermetrics 指标、球员薪资- 通过 SQLite 和 Python 测试脚本验证结果                 | 熟练掌握选择 / 聚合 / 嵌套子查询与视图的写法，理解真实数据模式 |
| **Project 2：B+ Tree Index**              | 在 RookieDB 中实现持久化 B+ 树   | - 补全 `LeafNode/InnerNode` 的 get/put/remove- 支持范围扫描、Bulk Load- 读懂元数据与磁盘页序列化                             | 数据页格式、索引插入 & 分裂、批量构建等底层索引实现        |
| **Project 3：Joins & Query Optimization** | 高效连接算法 + 基本优化器           | - Part 1：实现 Block Nested Loop、Grace Hash、Sort‑Merge Join 等- Part 2：为 `QueryPlan` 加成本估计与搜索，自动重排 Join 顺序 | I/O‑aware 算法、外排序、统计代价模型、启发式优化      |
| **Project 4：Concurrency**                | 多粒度锁与锁管理器                | - Part 1：实现底层 `LockManager`（互斥队列、死锁检测）<br>- Part 2：实现 `LockContext` 与 `LockUtil`，支持意向锁 & 递归加锁          | 两阶段锁、IX/IS/X/S 意向层次、并发事务调度         |
| **Project 5：Recovery (ARIES)**           | 写前日志 & 崩溃恢复              | - 实现 WAL：日志格式、事务状态记录、Savepoint/Checkpoint- 完成 Analysis‑Redo‑Undo 三阶段重启流程                               | ARIES 协议、脏页表 / 事务表维护、日志回放与补偿记录     |

## 使用提示

1. **语言与环境**
    
    - 项目代码为 **Java 17**，配套测试用 **Python 3**。
        
    - VS Code / IntelliJ + Dev Container 可一键启动。
        
2. **完成顺序**
    
    - Project 0 为环境搭建，可跳过介绍直接动手。
        
    - 建议按 1 → 2 → 3… 顺序循序渐进；后续 Lab 会复用前面实现的组件。
        
3. **时间投入（官方给出的估计）**
    
    - SQL ≈ 5‑10 h → B+ Tree ≈ 20‑25 h → Join ≈ 25 h → Concurrency ≈ 20 h → Recovery ≈ 30 h。
        
4. **评分比例**
    
    - 每个 Project 占期末总评 5‑8 %，全部完成即可覆盖 CS186 核心：**查询语言 → 存储/索引 → 执行/优化 → 并发 → 持久化**。
        

完成以上几个 Lab，基本就亲手“搭”了一套迷你版关系数据库的核心子系统。祝你玩得尽兴、代码顺利！
