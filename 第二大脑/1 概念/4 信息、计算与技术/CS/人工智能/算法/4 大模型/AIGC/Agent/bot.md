---
uid: a09036f6-ef45-417f-a034-f436a2ebab94
date created: 2026-10-28
date modified: 2026-10-28
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

1. 感觉 bot 的本质是 agent上面封装一层记忆系统，实现不切 session 的无限上下文的单 agent 的使用体验
2. 规则
	1. 做事前之前自己一个 DDL 并 hook 出一个后面的回看任务
3. 启动 [[subAgent]] 时候用的命令
	1. codex --dangerously-bypass-approvals-and-sandbox
	2. dsh 把「沙箱模式」和「审批策略」分成两个旋钮：沙箱有 `workspace-write`、`danger-full-access` 等，审批有 `ask`（要确认）和 `never`（不确认）。权限预设插件 `@deepseek-ai/dsh-permission-presets` 默认就带两档：`

# 🧩 Cues

[[Agent@]]

# 🪞Notes

