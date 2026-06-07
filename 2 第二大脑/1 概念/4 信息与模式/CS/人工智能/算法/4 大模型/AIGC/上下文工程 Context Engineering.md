---
uid: dd202de6-9249-4518-ad40-6fc4f6b1aa2c
date created: 2025-07-19
date modified: 2025-10-19
---
# Summary

1. 去掉什么？ 本质是context的30% 阈值（约 60k/200k）是业界常见的"显著衰减点"
2. [ ] 增加什么？更好的上下文给模型更多参考



最佳实践：https://code.claude.com/docs/en/context-window 这个网站可以交互式地看到[[Claude Code]] 的上下文加载顺序
![CleanShot 2026-04-04 at 21.50.27@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2021.50.27%402x.png)

**只有大模型的交互方式还是 input 带来 output，那么 input 一定非常关键**
1. 潜台词加入：浏览足迹本质是端内的context 其实也是上下文工程
2. 废话过滤：所有和当前 LLM 节点决策无关的数据都可以过滤

围棋需要的智力很高，但是context是全的。所以尽管智力门槛高任然可以快速被ai取代
# Cues



# Notes

`/context` 命令查看当前 session 的上下文情况
![CleanShot 2026-04-04 at 21.54.18@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2021.54.18%402x.png)
