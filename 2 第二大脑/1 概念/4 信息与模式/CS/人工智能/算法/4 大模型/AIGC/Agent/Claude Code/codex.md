---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases:
  - Summary
linter-yaml-title-alias: Summary
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

截至 **2026-04-19**，如果你想把 `Claude Code` 的概念迁移到 `Codex`，可以先用这张“心智模型对照表”来记：

| Claude Code | Codex | 怎么理解 |
|---|---|---|
| `CLAUDE.md` | `AGENTS.md` | 都是“长期指令/项目约定”文件 |
| `.claude/skills/.../SKILL.md` | `SKILL.md` / skills | 都是把一套可复用流程封装成“技能” |
| 旧的 `/commands` / slash commands | Codex 里更接近 skills / 内置工具 | Claude 更强调 `/xxx` 入口；Codex 更像“技能 + 工具调用” |
| `/permissions`、permission modes | approval policy + sandbox mode + plan mode | 都是在控制“能不能执行命令/改文件/要不要先问你” |
| MCP servers | MCP / apps / connectors | 这块两边非常像，都是给代理接外部系统 |
| subagents / agent teams | sub-agents / delegated workers | 都有“拆分任务给多个代理并行做”的思路 |
| hooks | 没有一个完全同名的一对一物件；更接近 skills、脚本化工具链、自动化 | Claude 的 hooks 更像“动作前后插钩子” |
| Routines / scheduled tasks | automations / heartbeat | 都能做定时或持续跟进任务 |
| auto memory | memory / 线程上下文 / 技能沉淀 | 都有“让代理别每次都从零开始”的方向，但形态不完全一样 |

更口语一点的类比：

- `CLAUDE.md` ≈ `AGENTS.md`
- `slash command` ≈ `Codex skill` 的入口感
- `permissions mode` ≈ `Codex` 的审批/沙箱策略
- `MCP` ≈ `MCP`，这块几乎就是同一个生态概念
- `Routines` ≈ `automations`
- `subagents` ≈ `spawn/delegate agents`

有两个地方要特别注意，不是严格 1:1：

1. **技能/命令**
Claude 官方现在把 custom commands 基本并到 skills 体系里了；而 Codex 这边你更适合把它理解成“技能包 + 工具能力”，不一定执着于 `/命令` 这个交互壳。

2. **hooks**
Claude Code 的 hooks 是很明确的产品概念；Codex 里更常见的是通过技能、脚本、MCP 工具、自动化去实现类似效果，所以是“能力等价”，不是“名词等价”。

如果你本来就熟悉 Claude Code，我建议你这样迁移记忆：

- “项目规则”看成 `CLAUDE.md -> AGENTS.md`
- “复用工作流”看成 `slash commands/skills -> skills`
- “权限控制”看成 `permission modes -> approval/sandbox`
- “外部集成”看成 `MCP -> MCP`
- “定时跑任务”看成 `Routines -> automations`

参考：
- [Claude Code overview](https://code.claude.com/docs/en)
- [Claude Code skills](https://code.claude.com/docs/en/slash-commands)
- [Claude Code permissions](https://code.claude.com/docs/en/permissions)
- [OpenAI Codex docs home](https://developers.openai.com/)
- [Codex use cases](https://developers.openai.com/codex/use-cases)
- [Codex AGENTS.md docs pointer](https://github.com/openai/codex/blob/main/docs/agents_md.md)

如果你愿意，我下一条可以直接给你一版：
**“Claude Code 用户迁移到 Codex 的速查表”**，按日常操作习惯来对照。

# 🧩 Cues

[[codex最佳实践]]



# 🪞Notes
