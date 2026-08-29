---
date created: 2025-04-06
date modified: 2025-07-10
uid: 43b7e414-eda5-4024-a7e6-8264e8a53a97
---

ZeRO (Zero Redundancy Optimizer) 和 DeepSpeed 库是微软开源的解决方案，旨在降低大模型训练的内存和通信开销。ZeRO将优化器状态、梯度、模型参数在数据并行进程间切分，避免每张卡存完整拷贝，从而训练更大的模型。DeepSpeed集成了ZeRO并提供易用接口，同时支持梯度检查点、CPU/offload（将部分数据放CPU）等。本节我们在单机模拟DeepSpeed使用方法，理解配置要点。
