---
uid: a99e04a0-fcb1-40d2-9ac5-88a1f0b0b0e4
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

tmux 面板也已同步显示各 agent 的实时输出，可以 tmux attach -t agent-team 查看。需要深入某个方向可以继续问。


## tmux 简介

**tmux**（Terminal Multiplexer，终端复用器）是一个在终端中运行的工具，让你可以在一个窗口里管理多个终端会话。

---

### 核心概念

tmux 有三层结构：

```
Session（会话）
  └── Window（窗口）
        └── Pane（窗格）
```

- **Session**：一个独立的工作环境，可以后台运行，断开连接后不会消失
- **Window**：类似浏览器的标签页，一个 session 可以有多个 window
- **Pane**：将一个 window 分割成多个区域，同时显示多个终端

---

### 为什么用 tmux？

1. **会话持久化** — SSH 断线后，服务器上的程序继续运行，重连后还能恢复
2. **分屏操作** — 一个终端同时看代码、日志、命令行
3. **多任务管理** — 在不同 window/session 之间快速切换

# 🧩 Cues



# 🪞Notes
