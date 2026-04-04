---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡Summary

> [!Info]
> Run Claude on a schedule to automate work that repeats: morning PR reviews, overnight CI failure analysis, weekly dependency audits, or syncing docs after PRs merge.
> 定期运行 Claude，自动执行重复性工作：早晨的 PR 审查、夜间的 CI 失败分析、每周的依赖项审计，或在 PR 合并后同步文档。

## 定时任务在哪里？
对比：

| 方式                 |                 | 持久化          | 本地文件访问 | 离线补跑 |
| ------------------ | --------------- | ------------ | ------ | ---- |
| /loop              | 当前session内      | 否（session 内） | 有      | 无    |
| [[crontab]]（linux） | 本地os            | 是            | 有      | 无    |
| [[launchd]]（macOS） | 本地os            | 是            | 有      | 有    |
| /schedule          | claude提供的云端操作系统 | 是            | 无      | 有    |
![CleanShot 2026-04-04 at 16.56.25@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2016.56.25%402x.png)

要我帮你配一个本地定时任务吗？说下你想定时跑什么。

## 云端os

远程任务：在 Anthropic 管理的云端基础设施上运行。即使您的计算机处于关闭状态，它们也会继续运行，但它们是针对代码库的新克隆副本而非您的本地检出版本进行操作。


- [Cloud scheduled tasks](https://code.claude.com/docs/en/web-scheduled-tasks) run on Anthropic-managed infrastructure, so they keep running even when your computer is off. Create them from the web, the Desktop app, or by running `/schedule` in the CLI.
  云端定时任务运行在 Anthropic 托管的基础设施上，因此即使您的电脑处于关闭状态，它们也会继续运行。您可以通过网页端、桌面应用或在 CLI 中运行 `/schedule` 来创建它们。

## 本地os
- [Desktop scheduled tasks](https://code.claude.com/docs/en/desktop-scheduled-tasks) run on your machine, with direct access to your local files and tools
  桌面定时任务在您的机器上运行，可以直接访问您的本地文件和工具。
- [`/loop`](https://code.claude.com/docs/en/scheduled-tasks) repeats a prompt within a CLI session for quick polling
  `/loop` 在 CLI 会话中重复执行提示词，以便进行快速轮询。

# 🧩 Cues

# 🪞Notes