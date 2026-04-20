---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
#  💡Summary  

> [!Info]
> `claude -p "任务" --dangerously-skip-permissions` 是最牛逼的
> 

根据文档，要让 Claude Code 完全不问任何问题、直接执行所有操作，有几种组合方式：

## 核心方式

**`bypassPermissions` 模式**（即 `--dangerously-skip-permissions`）是最彻底的，但有两个"例外"仍会弹提示：

- `.git`、`.vscode`、`.idea`、`.husky` 目录的写入
- `.claude` 目录的写入（除了 commands/agents/skills 子目录）

所以它并非"零提示"，只是接近。

---

## 真正做到"零问题、直接交付"的完整组合

### 方案一：`bypassPermissions` + 非交互模式

```bash
claude -p "你的任务描述" --dangerously-skip-permissions
```

`-p` 是非交互（headless）模式，直接输入任务、拿结果退出。两者结合：**没有 UI 交互、没有权限弹窗**，唯一的限制是上述受保护目录。

### 方案二：`auto` 模式（更推荐，更安全）

```bash
claude --permission-mode auto --enable-auto-mode
```

`auto` 模式由后台分类器替代人工确认，**用户完全感知不到权限提示**，同时保留安全检查。但有前提：

- 需要 Team / Enterprise / API 计划
- 必须用 Claude Sonnet 4.6 或 Opus 4.6
- 管理员需在后台开启

### 方案三：`dontAsk` + 预定义允许规则

```bash
claude --permission-mode dontAsk
```

配合 `settings.json` 预先配置好所有 allow 规则：

```json
{
  "permissions": {
    "defaultMode": "dontAsk",
    "allow": [
      "Bash(*)",
      "Write(*)"
    ]
  }
}
```

这样只允许你明确白名单的工具，完全非交互——适合 CI/CD 场景。

---

## 关键细节对比

|方案|真正零提示？|有安全检查？|适用场景|
|---|---|---|---|
|`bypassPermissions`|几乎是（受保护目录除外）|❌ 无|隔离容器/VM|
|`auto`|✅ 是|✅ 有分类器|日常长任务|
|`dontAsk` + 白名单|✅ 是|仅限白名单|CI/脚本自动化|

---

## 实用结论

**最彻底**：在隔离容器里用 `claude -p "任务" --dangerously-skip-permissions`

**最安全且无感**：`auto` 模式（需要符合计划要求）

文档原话也提示：`bypassPermissions` 对 prompt injection 没有防护，如果不是完全隔离的环境（容器/VM），建议用 `auto` 代替。

# 🧩 Cues



# 🪞Notes

## 你的文件中已使用的配置项（4 个）

| 配置项                   | 你的用法                                                                                                                              |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `model`               | `"opus"`                                                                                                                          |
| `hooks`               | 配置了 9 种生命周期钩子（UserPromptSubmit, PreToolUse, PostToolUse, Notification, Stop, SessionEnd, PreCompact, SubagentStart, SubagentStop） |
| `feedbackSurveyState` | 内部状态，记录反馈调查的展示时间                                                                                                                  |
| `enabledPlugins`      | 启用了 `frontend-design@claude-plugins-official`                                                                                     |

---



## 权限问题

在 setting.json 中配置
根据官方文档，bypassPermissions 模式下所有 subagent 自动继承这个模式，所以只要主配置开了 bypass，subagent 执行任务时也不会弹权限提示。这个方案在权限这个问题上是最省心的。

超级最高权限的配置方案
```
  "permissions": {
    "skipDangerousModePermissionPrompt": true,
    "defaultMode": "bypassPermissions",
    "allow": [
      "Bash(*)",
      "Read(**)",
      "Write(**)",
      "Edit(**)",
      "MultiEdit(**)",
      "Glob(**)",
      "Grep(**)",
      "WebFetch",
      "WebSearch",
      "AskUserQuestion",
      "Agent(*)",
      "mcp__*"
    ]
  },
  "hooks": {
    "PermissionRequest": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\":{\"hookEventName\":\"PermissionRequest\",\"permissionDecision\":\"allow\",\"permissionDecisionReason\":\"auto-approved\"}}'",
            "statusMessage": "Auto-approving..."
          }
        ]
      }
    ],```