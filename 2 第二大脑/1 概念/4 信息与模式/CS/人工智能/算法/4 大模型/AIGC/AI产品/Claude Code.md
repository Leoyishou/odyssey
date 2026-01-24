---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-15
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

![img_v3_02u6_a04bf4e6-75a8-4356-9ccd-84125e36668g.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fresources%2Fimages%2F2026%2F01%2F22%2F17-43-41-36c15ae148258b93193d419212138b30-img_v3_02u6_a04bf4e6-75a8-4356-9ccd-84125e36668g-c6f0e3.png)

记录了[[Claude Code]]的提示词的变化
https://github.com/marckrenn/claude-code-changelog

创建者 @bcherny

热度和活跃度更是高的异常，他仿佛住在 X 上，但代码提交完全不耽误且十分高效，这就是 CC 团队自身最佳实践的证明了。

| | | |
| :---------------------- | :----------------------- | :-- |
| Marketplace (市场/商店) | 就像 App Store 或 Google Play | |
| Plugins (插件) | 就像具体的 App（如微信、抖音） | |
| [[skills]] | | |
| [[MCP]] | 1. 发小红书 | |

## 配置维度

两个配置文件的区别

| 文件 | 作用 | MCP 配置生效？ |
| :---------------------- | :------------------------------------------------------- | :---------------------- |
| `~/.claude/settings.json` | 用户全局设置：hooks、默认模型、主题等 | ❌ MCP 似乎不从这里读取 |
| `~/.claude.json` | 项目级配置：每个项目目录有独立的配置块，包括 MCP、权限等 | ✅ MCP 从这里读取 |

## Claude Code 的 session 是按项目目录隔离的

Claude Code 的 session 是按项目目录隔离的，不是全局的。

存储结构如下：
```
  ~/.claude/projects/
  ├── -Users-liuyishou/                          # 在 ~ 目录启动的 sessions
  ├── -Users-liuyishou-usr-projects-inbox-daily-quote-app/   # 本项目的 sessions
  ├── -Users-liuyishou-usr-projects-published-vocab-highlighter/  # vocab-highlighter 的 sessions
```

## 数据存储方式

打 log 的信息存储方式

本质上就是 JSONL（JSON Lines）格式的日志文件，每一行是一条消息记录：
```
  ~/.claude/projects/-<escaped-path>/
  ├── sessions-index.json          # 索引：session ID、摘要、消息数等元数据
  ├── <session-id-1>.jsonl         # 完整对话记录，逐行追加
  └── <session-id-2>.jsonl
```

```
 通用字段

  每行都有：
  {
    "type": "system | user | assistant | progress | file-history-snapshot",
    "uuid": "消息唯一ID",
    "parentUuid": "父消息ID（形成对话树）",
    "sessionId": "session ID",
    "timestamp": "ISO时间戳",
    "cwd": "工作目录",
    "gitBranch": "当前分支",
    "version": "版本号",
    "isSidechain": false
  }

  按 type 分类
  ┌───────────────────────┬──────┬──────────────────────────────────────────┐
  │         type          │ 数量 │                   内容                   │
  ├───────────────────────┼──────┼──────────────────────────────────────────┤
  │ system                │ 3    │ 系统提示词，content 为字符串             │
  ├───────────────────────┼──────┼──────────────────────────────────────────┤
  │ user                  │ 29   │ 用户消息或工具结果                       │
  ├───────────────────────┼──────┼──────────────────────────────────────────┤
  │ assistant             │ 56   │ AI 回复（含 thinking + text + tool_use） │
  ├───────────────────────┼──────┼──────────────────────────────────────────┤
  │ progress              │ 2    │ 进度状态                                 │
  ├───────────────────────┼──────┼──────────────────────────────────────────┤
  │ file-history-snapshot │ 9    │ 文件变更快照                             │
  └───────────────────────┴──────┴──────────────────────────────────────────┘
  assistant 消息的 content 块

  {
    "type": "assistant",
    "message": {
      "role": "assistant",
      "model": "claude-opus-4-5-20251101",
      "content": [
        { "type": "thinking", "thinking": "..." },
        { "type": "text", "text": "..." },
        { "type": "tool_use", "name": "Bash", "id": "toolu_xxx", "input": {"command": "..."} }
      ]
    }
  }

  user 消息（含工具结果）

  {
    "type": "user",
    "message": {
      "role": "user",
      "content": [
        { "type": "tool_result", "tool_use_id": "toolu_xxx", "content": "命令输出..." }
      ]
    }
  }
```
特点：
- 追加写入（append-only），不会修改历史记录
- 每行一个 JSON 对象，包含角色（user/assistant）、内容、工具调用等
- 按项目目录隔离，路径转义后作为文件夹名
- sessions-index.json 维护轻量索引，方便 claude --resume 列出可恢复的 session

所以它确实就是 structured logging，没有数据库，没有复杂的状态管理，纯文件系统。好处是简单可靠、可以直接 cat 或 jq
查看内容，坏处就是你刚遇到的——跨项目不互通。

___

### 2026年真实社区共识 Top 3 保命组合拳（每天必用）

1. 上下文快满 → 先 `/context` 看情况
2. 还行就 `/compact`，不行就直接 `/clear` + 让Claude写总结
3. 大项目全程 → 每完成一个小阶段 `/clear` + 用 `/agents` 创建子代理继续

### 额外高频但不是内置的（自定义Slash Command）

很多人会自己做这些，频率其实比一部分内置还高：

- `/plan` → 做详细计划
- `/commit` 或 `/commit-push-pr` → 一键提交+PR
- `/review` 或 `/code-review` → 代码审查
- `/test` → 跑测试并报告

**一句话总结目前2026年1月的vibe：** 「会用 `/clear` + `/context` + `/compact` + 子代理 的，才算真正会玩Claude Code」 其他都是锦上添花，自定义命令看个人/团队习惯。

**Claude Code（claudecode）里最高频使用的 /命令**（基于2025-2026年社区真实使用频率排序，来自Boris等官方人员、活跃重度用户、日本/中文圈大V反馈） 这些是真正每天被用到爆的，基本属于「不离手」级别：

| 排名 | 命令 | 使用频率 | 主要作用与实用场景 | 谁最爱用 / 典型说法 |
| :-- | :------------------------------------ | :---- | :------------------------------------------------------- | :----------------------------------- |
| 3 | **/context** | ★★★★ | 实时查看当前上下文占用情况、哪些文件/MCP/历史在吃token，一看就知道谁是罪魁祸首 | 重度用户标配「先/context再决定/clear还是/compact」 |
| 1 | **/clear** | ★★★★★ | 直接清空当前会话历史，保留项目文件上下文，瞬间释放大量token，最常用保命/续命手段 | 几乎所有人，尤其是长任务时「每做完一件事就/clear」 |
| 2 | **/compact** | ★★★★☆ | 手动触发上下文压缩（比auto-compact更可控），保留关键信息，适合不想彻底清空时用 | Boris本尊、上下文管理党「/compact > auto」 |
| 4 | **/help** | ★★★★ | 显示所有可用命令（包括你自己加的自定义命令），新手/忘了命令时必点 | 所有人，新功能上线后狂点 |
| 5 | **/agents** | ★★★☆ | 管理/查看/创建子代理（Sub-Agent），2026年最火的上下文解决方案核心入口 | Sub-Agent党「不装子代理的都out了」 |
| 6 | **/bug** | ★★★ | 快速把当前会话打包发给Anthropic报告bug（尤其是奇怪的崩溃/卡死/乱压缩时） | 遇到诡异问题时的救命稻草 |
| 7 | **/init** | ★★☆ | （部分老版本/场景）初始化项目记忆或读取CLAUDE.md等关键文件，很多教程里推荐 | 日本/中文圈早期用户较多，现在逐渐被自定义取代 |
| 8 | **/permissions** 或 **/allowed-tools** | ★★☆ | 查看/修改工具权限（尤其是bash危险命令），安全党常用 | 企业用户、安全意识强的开发者 |

### Plugins (插件)

- **类比**：就像具体的 App（如微信、抖音）

```
/skills  查看 skills
```

Claude Code 逆向工程研究仓库
https://github.com/shareAI-lab/analysis_claude_code

When I review my [CLAUDE.md/slash](http://claude.md/slash) command and agent files, it feels like reading my own software philosophy—a reflection of what I've learned, what I value, and how I think code should be built. If it doesn't resonate with you personally, it won't guide the AI effectively.

1/ shift + tab 先切换到 plan mode, 别一来就猴急脱裤子 2/ 把想要解决的问题想清楚，按照how -> why -> any other better solutions 问个底朝天 3/ 画图，先别急着说 CC 画的图太烂，一图胜千言，画个十几张图架构就清晰了 4/ 千万不要用 YOLO mode, 要不然后面的屎山代码几个月都清理不干净

# Cues

# Notes

## 如何关闭内置的 shell

1. 按向下的方向键进入
2. kill 关闭

## 基础命令

### 项目管理

```bash
# 创建新项目
claude-code new <project-name>

# 在当前目录初始化项目
claude-code init

# 查看项目状态
claude-code status
```

### 代码操作

```bash
# 让 Claude 编写代码
claude-code write "实现一个排序算法"

# 让 Claude 审查代码
claude-code review <file>

# 让 Claude 重构代码
claude-code refactor <file>

# 让 Claude 修复 bug
claude-code fix <file>
```

## ⚠️ "危险"命令（需谨慎使用）

### 批量操作命令

```bash
# 批量修改多个文件
claude-code batch-edit "*.js" --prompt "添加错误处理"

# 自动应用所有建议（无需确认）
claude-code apply --all --no-confirm

# 递归处理整个目录
claude-code process . --recursive
```

### 系统级操作

```bash
# 执行 Claude 建议的系统命令
claude-code exec --allow-system

# 自动安装依赖
claude-code deps --auto-install

# 清理和重构整个项目
claude-code cleanup --aggressive
```

### 代码生成相关

```bash
# 完全重写文件
claude-code rewrite <file> --force

# 基于描述生成整个模块
claude-code generate-module "完整的用户认证系统"

# 删除"冗余"代码
claude-code optimize --remove-unused
```

## 安全建议

### 使用前备份

```bash
# 创建备份
git commit -am "Before Claude Code changes"

# 或使用内置备份
claude-code backup create
```

### 安全模式

```bash
# 开启安全模式（所有更改需确认）
claude-code config set safety-mode on

# 设置只读模式（只分析不修改）
claude-code --read-only
```

### 查看变更

```bash
# 预览将要进行的更改
claude-code <command> --dry-run

# 查看历史操作
claude-code history

# 撤销上次操作
claude-code undo
```

## 实用技巧

### 配置文件 (.claude-code.yml)

```yaml
# 设置默认行为
safety:
  confirm-changes: true
  backup-before-change: true

exclude:
  - node_modules/
  - dist/
  - "*.min.js"

limits:
  max-file-size: 100KB
  max-files-per-operation: 10
```

### 常用组合

```bash
# 安全地重构
claude-code refactor src/ --dry-run
# 确认后再执行
claude-code refactor src/

# 增量式改进
claude-code review --suggest-only
claude-code apply --interactive
```

## ⚠️ 特别注意

**绝对要避免的操作：**

1. 在生产代码上直接使用 `--force` 或 `--no-confirm`
2. 对关键系统文件使用 `rewrite` 或 `cleanup --aggressive`
3. 在没有备份的情况下批量修改
4. 给予过于宽泛的权限（如 `--allow-all`）

建议先在测试环境或个人项目上熟悉这些命令，充分了解每个命令的影响后再在重要项目中使用。

需要查看更详细的文档，可以访问：https://docs.anthropic.com/en/docs/claude-code
