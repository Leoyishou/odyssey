---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-08-10
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

PEFT: Parameter-Efficient Fine-Tuning

/peft/
# Cues



# Notes
## 对比表格

|维度|PEFT|LlamaFactory|推荐场景|
|---|---|---|---|
|**易用性**|需要写代码 ⭐⭐⭐|配置文件驱动 ⭐⭐⭐⭐⭐|LlamaFactory 更友好|
|**灵活性**|完全可控 ⭐⭐⭐⭐⭐|预设模板 ⭐⭐⭐|PEFT 更灵活|
|**调试**|可断点调试 ⭐⭐⭐⭐⭐|黑盒较多 ⭐⭐|PEFT 更透明|
|**性能**|手动优化 ⭐⭐⭐⭐|自动优化 ⭐⭐⭐⭐⭐|LlamaFactory 内置优化|
|**社区**|Hugging Face 官方|中文社区活跃|都很好|

## 我的代码用的是 PEFT，原因

1. **更适合 Kaggle 竞赛**：可以精确控制每个细节
2. **便于集成自定义逻辑**：如特殊的数据处理、loss 函数
