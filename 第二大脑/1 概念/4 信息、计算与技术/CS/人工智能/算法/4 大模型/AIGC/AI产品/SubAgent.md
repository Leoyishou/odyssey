---
uid: 24f60ffc-ab96-4060-9337-391cc1da8dae
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡Summary

> [!Info]
> 1. 有向无环图的任务，subagent 才有意义
> 2. 一个进程、一个session、两个context

# 🧩 Cues

# 🪞Notes

## 一、subagent 的本质

1. 进程层面：subagent 是一个独立的推理过程，有自己的 context window，独立读文件、做搜索。但它不是 claude -p 那样的 CLI 子进程，而是在同一个 Claude Code 进程内部，通过 API 调用创建的一个并行推理流。
2. Session 层面：subagent 不会创建新的 session ID。它的生命周期完全在主 session 内，结束后上下文就释放了。
3. Context 层面：subagent 有独立的 context window（这是关键）。它读的文件、中间推理都不占主 context。只有最终返回的摘要文本进入主 context。

| | Subagent（Agent 工具） | claude -p 子进程 |
| :--- | :--- | :--- |
| 进程 | 同进程内 | 新进程 |
| Session | 无独立 session | 有独立 session ID |
| Context | 独立 window，结果回传 | 完全独立 |
| 可 resume | 不可 | 可（--resume） |
| 访问主对话历史 | 不能 | 不能 |

简单说：subagent 像是"脑子里开了个分线程想问题"，而 claude -p 是"叫了另一个人来干活"。

![CleanShot 2026-04-03 at 12.17.03@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-03%20at%2012.17.03%402x.png)

## 二、Agent 如何定义

[[【cc】Agent]]
agent 定义在` ~/.claude/agents/` 目录下，每个 agent 一个 .md 文件。

```text
  例如：                                                                                                                                             
  - ~/.claude/agents/business-unit.md
  - ~/.claude/agents/ceo-assistant.md                                                                                                                
  - ~/.claude/agents/investment-bu.md 
```

## 三、如何 new 一个 Agent 去干活
- **父 agent 自动 spawn** — Claude 看到任务匹配某个 agent 的 description，自动委派
- **你手动调用** — 对话里直接说"用 `research-department` 去调研这个问题"
- **快速切换模型** — 注意你的配置里 `product-department` 和 `research-department` 用的是 **opus**，重型任务专用；`ops-department` 用 **haiku**，轻量快速


除了这些内置子代理外，您还可以使用
1. 内置调用，自动出发 plan subagent、explore subagent
2. 自定义提示词、
3. 工具限制、权限模式、钩子和技能来创建自己的子代理。以下章节将介绍如何入门和自定义子代理。