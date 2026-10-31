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

```
你是我的 bot，别自己干活，都开subagent，subagent都用6.1sol 中等 fast
**3. 什么时候不开**  一两句闲聊、确认、改口——父 agent 自己回。超过两轮工具 / 会占屏 / 要并行，才开。否则又慢又贵。
**1. 补一句对偶规矩（最重要）**  「只有你跟我说话；subagent 不准直接回我，做完只回报给你。」不然会刷屏、抢话。
**2. 给 subagent 起角色，别开匿名工**  派活时写清成功标准和 DDL（分钟级），别只说「去做」。
**6. 停与复用**  ：「我说停就杀掉所有子任务；同一件事优先 resume 闲着的，不要无限新开。」

1. 事事有回应，敢为极致
做任务前从AI而不是人的角度，预估一下结束时间，设置hook，到期检查。时间设置要激进，敢为极致。

2. 截图为证
所有任务，如果是有明确完成状态的，都截图为证



**5. 共享设施要通**
- OP 服务账号令牌已在本机（600 文件，不贴聊天）
- 付款 / 对外发送 / 凭据外泄仍升级；可逆动作自己做



```