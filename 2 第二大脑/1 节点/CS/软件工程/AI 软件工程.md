---
draw:
title: AI 软件工程
date created: 2025-01-23
date modified: 2025-02-06
---

mlops来自与devopst：devops包括了CI/CD*，监控告警，开发迭代效率等，而mlops针对AI有多了数据集管理、训练代码与任务管理、模型版本、自动化评估、模型部署与上线等工作，一般会把这套东西做成web页面，供算法人员使用。

mlsys：系统架构师有些接近，针对hpc+又需要特殊的技能，包括异构计算与显存，高性能网络互眹（ib/roce/rdma+），高速卡间互联（nvsiwitch/ink），cuda 软件栈（runtime/算子库＋/通信库），ai编译器，aiill推框架，要求特别高；

ai infra*：互联网的infra包括了各种存储系统（文件/对象/块存储*）/数据库（关系型、nosql，htap）/k8st/各种消息缓存中间件/大数据治理体系（batch*、stream、数据仓库、数据湖＋）/rpc 框架及可观测性，混沌工程*，全链路压测，多活，优化性能降本增效，系统SLA*，ai的infra主要针对训练推理框架做优化，比如千卡万卡集群训练优化和SLA，IIm*/多模态/视频生成的，需要懂模型算法，优化计算算子/通信，zero3/tppp分布式方案（pytorch&deepspeed& fsdp &megatront），跟mlsys技能有重叠更懂模型和框架，比如deepseek的ai infra 业界top级。
