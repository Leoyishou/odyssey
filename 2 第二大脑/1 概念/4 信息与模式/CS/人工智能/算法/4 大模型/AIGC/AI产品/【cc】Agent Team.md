---
uid: 8f73f65a-bb00-43c3-81b1-84500bd77468
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
>  和[[SubAgent]]的本质区别在于通信，|通信|仅向主 agent 报告结果|队友之间直接互相消息|
> 

参考文档：https://code.claude.com/docs/en/agent-teams


Use subagents when you need quick, focused workers that report back. Use agent teams when teammates need to share findings, challenge each other, and coordinate on their own.  
当你需要快速、专注的执行者并向你汇报时，请使用子代理（subagents）。当团队成员需要分享发现、互相质疑并自主协调时，请使用代理团队（agent teams）。

![CleanShot 2026-04-04 at 22.39.19@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2022.39.19%402x.png)

1. Agent teams let you coordinate multiple Claude Code instances working together. One session acts as the team lead, coordinating work, assigning tasks, and synthesizing results. Teammates work independently, each in its own context window, and communicate directly with each other.  
2. Unlike [[SubAgent]], which run within a single session and can only report back to the main agent, you can also interact with individual teammates directly without going through the lead.

## 收益

1. 并行模式，人多力量大。例如：评审 PR、调研库或调查 Bug等任务
2. 对抗模式，防止陷入单一思维
	1. 观点的思辨，多个独立agent
	2. 一个agent写代码，一个做测试
# 🧩 Cues



# 🪞Notes

## 一、如何启动

### 1.1 配置打开


### 1.2 自然语言启动

启动以后张这个样子，注意和[[SubAgent]]区分开
![CleanShot 2026-04-04 at 23.30.32@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2023.30.32%402x.png)

## 二、如何通信：**Mailbox  邮箱**


**Teammate messaging:  队友消息：**

- 私聊：**message**: send a message to one specific teammate  消息：向特定队友发送消息
- 广播：**broadcast**: send to all teammates simultaneously. Use sparingly, as costs scale with team size.  广播：同时发送给所有团队成员。请谨慎使用，因为成本会随团队规模增加。

An agent team consists of:  
一个智能体团队由以下部分组成：

| Component  组件        | Role  角色                                                                                                                           |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| **Team lead  团队负责人** | The main Claude Code session that creates the team, spawns teammates, and coordinates work  <br>创建团队、生成团队成员并协调工作的主要 Claude Code 会话 |
| **Teammates  团队成员**  | Separate Claude Code instances that each work on assigned tasks  <br>各自负责执行分配任务的独立 Claude Code 实例                                  |
| **Task list  任务列表**  | Shared list of work items that teammates claim and complete  <br>团队成员认领并完成的工作项共享列表                                                 |
| **Mailbox  邮箱**      | Messaging system for communication between agents  <br>用于代理之间通信的消息系统                                                               |

See [Choose a display mode](https://code.claude.com/docs/en/agent-teams#choose-a-display-mode) for display configuration options. Teammate messages arrive at the lead automatically.





##  三、权限继承

Teammates start with the lead’s permission settings. If the lead runs with `--dangerously-skip-permissions`, all teammates do too. After spawning, you can change individual teammate modes, but you can’t set per-teammate modes at spawn time.  
队友初始继承负责人的权限设置。如果负责人以 `--dangerously-skip-permissions` 模式运行，所有队友也会如此。启动后，您可以更改单个队友的模式，但无法在启动时为每个队友单独设置模式。

## 四、显示方式

1. 一个窗口，手动切换默认的进程内模式适用于任何终端。
2. 分屏模式需要 [[tmux]] 或 iTerm2：


