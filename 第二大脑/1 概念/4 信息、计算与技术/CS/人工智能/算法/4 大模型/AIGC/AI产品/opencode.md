---
uid: 47f774e1-47a2-4bda-b42f-7e7bd2b344fe
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡Summary

> [!Info]
> 一句话说清楚
>

# 🧩 Cues

Claude Code → OpenCode 迁移速查表

| 操作           | Claude Code                  | OpenCode                   |
| :------------- | :--------------------------- | :------------------------- |
| 启动 TUI       | `claude`                     | `opencode`                 |
| 指定项目       | `claude /path`               | `opencode /path`           |
| 继续上次会话   | `claude -c`                  | `opencode -c`              |
| 非交互执行     | `claude -p "任务"`           | `opencode run "任务"`      |
| 指定模型       | `claude --model`             | `opencode -m provider/model` |
| Web 界面       | 无                           | `opencode web`             |

会话内命令

| 操作       | Claude Code     | OpenCode        |
| :--------- | :-------------- | :-------------- |
| 清空对话   | `/clear`        | `/clear`        |
| 压缩上下文 | `/compact`      | `/compact`      |
| 初始化配置 | `/init`         | `/init`         |
| 切换模型   | 设置菜单        | `/models`       |
| 添加凭证   | 环境变量        | `/connect`      |
| 查看帮助   | `/help`         | `/help`         |
| 退出       | `/exit` 或 `Ctrl+C` | `/quit` 或 `Ctrl+C` |

配置文件

| 项目     | Claude Code                 | OpenCode                         |
| :------- | :-------------------------- | :------------------------------- |
| 全局配置 | `~/.claude.json`            | `~/.config/opencode/opencode.json` |
| 项目配置 | `.claude/settings.json`     | `opencode.json`                  |
| 项目说明 | `CLAUDE.md`                 | `AGENTS.md`                      |
| MCP 配置 | `~/.claude.json` 或 `.mcp.json` | `opencode.json` 中 `mcp` 字段    |

权限模式

| 模式     | Claude Code                    | OpenCode                                  |
| :------- | :----------------------------- | :---------------------------------------- |
| 全自动   | `--dangerously-skip-permissions` | 配置 `"permission": {"bash": "allow", ...}` |
| 单次自动 | `-y` / `--yes`                 | 无                                        |

快捷键（TUI）

| 操作 | Claude Code | OpenCode    |
| :--- | :---------- | :---------- |
| 提交 | `Enter`     | `Enter`     |
| 换行 | `Shift+Enter` | `Shift+Enter` |
| 中断 | `Esc`       | `Esc`       |
| 退出 | `Ctrl+C`    | `Ctrl+C`    |

────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
❯
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
⏵⏵ bypass permissions on (shift+Tab to cycle)

# 🪞Notes