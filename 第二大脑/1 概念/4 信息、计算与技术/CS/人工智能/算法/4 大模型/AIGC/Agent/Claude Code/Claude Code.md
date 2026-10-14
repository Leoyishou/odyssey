---
uid: 12a6fc1a-9804-4eff-9288-1f96c29be787
date created: 2025-07-19
date modified: 2025-09-15
---
# Summary

> [!Info]
> When I review my CLAUDE.md command and agent files, it feels like reading my own software philosophy—a reflection of what I've learned, what I value, and how I think code should be built. If it doesn't resonate with you personally, it won't guide the AI effectively.

[[cc Task]]

# Cues

[[codex]]
[[脚本启动Claude Code]]
[[ Claude Code 最佳实践]]
[[Claude Code 插件]]
[[harness]]
[[Claude Code 自动记忆 auto memory]]
[[Agentic loop]]
[[Agent 定时任务]]
[[上下文工程 Context Engineering]]
[[多Agent，multi agents]]
1. [[SubAgent]]
2. [[【cc】Agent Team]]

# Notes


![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/20260413130704229.png)

## 一、核心对象
### 1.1 基座模型
1. haiku 最快
2. Sonnet 中庸
3. opus最牛逼

### 1.2 tools
内置工具
![CleanShot 2026-04-04 at 22.07.39@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2022.07.39%402x.png)

### 1.3 skills
[[skills]]继承


### 1.4 上下文
一个session里，因为有[[SubAgent]]的存在可能对应多套上下文

### 1.5 Session
一个session里可能由多套context，这是因为[[SubAgent]]的机制

```text
这些日志主要存放在用户主目录下的 **.claude 目录**里，尤其是 ~/.claude/projects 这一大树下面，每个项目路径对应一个子目录，里面放若干个 .jsonl 会话文件。 (PyPI) 新版本还会额外维护一个全局的 ~/.claude/history.jsonl 文件，把所有项目的会话汇总在一起，便于做全局历史浏览。 (Uncommon Stack)
```

```text
  工作流程：

  Claude 响应完成
      ↓
  Stop Hook 触发（从 stdin 读取 JSON）
      ↓
  保存 session 状态到 ~/.claude/sessions/{session_id}.json
      ↓
  launchd 每 5 分钟调用 session-scanner.py
      ↓
  发现不活跃 > 2h 的 session
      ↓
  生成 AI 总结 → 更新 Linear Issue → 标记 finalized

  关键修复： Hook 数据是通过 stdin 以 JSON 传入，不是环境变量。脚本已更新为 INPUT=$(cat) + jq 解析。

  从下一轮对话开始，Stop hook 会自动记录当前 session 状态。
```

### 1.6 Agent
1. [[SubAgent]]
2. [[【cc】Agent Team]]

### 1.7 云环境

Claude Code on the web 在研究预览中可供以下用户使用：
- **Pro 用户**
- **Max 用户**
- **Team 用户**
- **Enterprise 用户**（拥有高级席位或 Chat + Claude Code 席位）


## 二、作用域

如何配置：https://code.claude.com/docs/zh-CN/settings
### 2.1 settings.json 有三级覆盖机制。
1. 项目维度 `项目目录/.claude/`
2. 用户维度 `~/.claude/`
3. 操作系统维度 `/opt/homebrew/.claude/`

### 2.2 skills 有二级覆盖机制

| 级别 | 路径 | 作用范围 |
| :--- | :--- | :--- |
| 用户级 | ~/.claude/skills/xxx/SKILL.md | 所有项目可用 |
| 项目级 | 项目目录/.claude/skills/xxx/SKILL.md | 仅该项目可用 |

给 job-bu 装项目专属 skill
以刚才创建的 job-data-analysis 为例，如果想让它只在 job-bu 下生效：
从全局移到项目级
`mkdir -p /Users/bytedance/usr/personal-ai-company/job-bu/.claude/skills/`
`mv ~/.claude/skills/job-data-analysis job-bu/.claude/skills/`

这样这个 skill 只在 job-bu/ 目录下工作时才会被加载，其他事业部看不到。

同名覆盖
如果全局 and 项目级有同名 skill，用户级（~/.claude）优先级更高，会覆盖项目级。所以：
- 通用 skills（飞书、画图、部署等）→ 放 ~/.claude/skills/
- 项目专属 skills（如 job-bu 的数据分析口径）→ 放 项目/.claude/skills/
- 不要同名，否则全局的会盖掉项目的

### 2.1 全局级cc

```text
~/.claude/
├── settings.json          # 全局用户设置（model, hooks, permissions, env 等）
├── settings.local.json    # 本地用户设置（不会被任何地方共享）
├── commands/              # 全局自定义斜杠命令（所有项目可用）
│   ├── review.md          #   → /review
│   └── deploy.md          #   → /deploy
├── agents/                # 全局子 Agent 定义（所有项目可用）
│   ├── code-reviewer.md
│   └── security-auditor.md
├── skills/                # 全局技能（所有项目可用）
│   ├── pdf/
│   │   ├── SKILL.md       #   技能定义（必须叫 SKILL.md）
│   │   └── extract.py     #   附带的脚本
│   └── csv/
│       ├── SKILL.md
│       └── analyze.py
├── projects/              # 项目状态数据（CC 自动管理）
│   └── {project-hash}/
│       └── {session-id}.jsonl   # 会话 transcript
└── hooks/                 # Hook 脚本存放处（你的配置就是这样组织的）
    ├── file-protection/
    │   └── index.js
    ├── git-linear-sync/
    │   └── index.js
    └── ...
```

另外根目录还有：`~/.claude.json`（旧式配置文件，存 OAuth、主题偏好、MCP 服务器、per-project 信任状态等）

---

### 2.2 项目级CC

```text
<project>/
├── CLAUDE.md                  # 项目记忆/指令（根目录级，给整个项目）
├── .mcp.json                  # 项目级 MCP 服务器配置（可 commit 共享）
└── .claude/
    ├── settings.json          # 项目设置（团队共享，commit 到 git）
    ├── settings.local.json    # 本地项目设置（gitignore，个人用）
    ├── CLAUDE.md              # 也可以放这里（.claude/ 目录级指令）
    ├── commands/              # 项目级斜杠命令
    │   ├── fix-issue.md       #   → /fix-issue
    │   └── run-tests.md       #   → /run-tests
    ├── agents/                # 项目级子 Agent
    │   └── db-reader.md
    ├── skills/                # 项目级技能
    │   └── testing-patterns/
    │       ├── SKILL.md
    │       └── helpers/
    └── hooks/                 # 项目级 Hook 脚本
        └── check-style.sh
```

### 2.3 配置方式
[[claude code 配置]]
两个配置文件的区别

| 文件 | 作用 | MCP 配置生效？ |
| :--- | :--- | :--- |
| `~/.claude/settings.json` | 用户全局设置：hooks、默认模型、主题等 | ❌ MCP 似乎不从这里读取 |
| `~/.claude.json` | 项目级配置：每个项目目录有独立的配置块，包括 MCP、权限等 | ✅ MCP 从这里读取 |

### 2.4 各部分定位总结

| 组件 | 存放位置 | 定位 |
| :--- | :--- | :--- |
| **`CLAUDE.md`** | 项目根 / `~/.claude/` / `.claude/` / 父目录 | **记忆与指令**——启动时自动加载，告诉 Claude 项目上下文、编码规范、工作流等 |
| **`settings.json`** | `~/.claude/` / `.claude/` | **配置**——权限、hooks、环境变量、模型选择等机器行为 |
| **`commands/`** | `~/.claude/commands/` / `.claude/commands/` | **斜杠命令**——手动触发的可复用 prompt 模板，通过 `/命令名` 调用 |
| **`agents/`** | `~/.claude/agents/` / `.claude/agents/` | **子 Agent**——有独立 system prompt、工具限制和上下文窗口的专用 AI 助手 |
| **`skills/`** | `~/.claude/skills/` / `.claude/skills/` | **技能**——Claude 自动发现并按需加载的能力包，可捆绑脚本和资源文件 |
| **`hooks/`** | 任意位置（settings.json 中引用路径） | **Hook 脚本**——生命周期事件触发的自动化脚本 |
| **`.mcp.json`** | 项目根目录 | **MCP 服务器**——扩展 Claude 的外部工具集成 |
| **`~/.claude.json`** | 用户 home | **旧式配置**——OAuth、主题、通知偏好、MCP（user/local scope）、项目信任状态 |

## 三、机制理解
### 上下文加载
[[上下文工程 Context Engineering]]

#### rules
`~/.claude/rules/workflow.md` 是你用来**“调教”** Claude Code 的地方，专门用于存储你个人的**标准作业程序（SOP）**。
它告诉 Claude：“不管我在哪个项目里，只要是跟我协作，就必须遵守这一套工作流程。”这样做的好处是你不需要在每个项目的 `CLAUDE.md` 里重复写一遍相同的 Git 规范或思考模式要求

### hook切面时机
![CleanShot 2026-02-03 at 11.08.17@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_7PAHBomwLY%2F2026%2F02%2F03%2F11-08-38-de3122b120008306cd06f004428a7ef2-CleanShot%202026-02-03%20at%2011.08.17-2x-0fe6bd.png)

### 数据存储方式
打 log 的信息存储方式
本质上就是 JSONL（JSON Lines）格式的日志文件，每一行是一条消息记录：

```text
  ~/.claude/projects/-<escaped-path>/
  ├── sessions-index.json          # 索引：session ID、摘要、消息数等元数据
  ├── <session-id-1>.jsonl         # 完整对话记录，逐行追加
  └── <session-id-2>.jsonl
```

```text
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
```

| type | 数量 | 内容 |
| :--- | :--- | :--- |
| system | 3 | 系统提示词，content 为字符串 |
| user | 29 | 用户消息或工具结果 |
| assistant | 56 | AI 回复（含 thinking + text + tool_use） |
| progress | 2 | 进度状态 |
| file-history-snapshot | 9 | 文件变更快照 |

```text
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

### 权限控制
[[classifier]]
[[Claudecode 权限]]

### 插件
[[Claude Code 插件]]

## 四、常用命令

```bash
## 无人值守模式
claude --dangerously-skip-permissions -p "你的任务描述"
```

| 动作 | 何时使用 |
| :--- | :--- |
| /clear | 相当于 new session，session ID 也会变 |
| /compact | 对话过长时压缩，保留关键信息 |
| /rewind | 发现走错方向时回退 |

Command 机制 它其实相当于就是一个预制的 prompt，通过一个简短的指令注入到上下文

```text
  你输入: /code-review

  Claude Code 做的事:
  1. 查找 ~/.claude/commands/code-review.md
  2. 读取文件内容
  3. 把内容注入当前对话作为指令
  4. Claude 按照这个指令执行
```

```text
  Claude Code (CC)
  ├── 💾 Memory 记忆系统
  │   ├── CLAUDE.md 层级
  │   │   ├── Enterprise (~/.claude/enterprise/CLAUDE.md)
  │   │   ├── User (~/.claude/CLAUDE.md)
  │   │   ├── Project (项目根目录/CLAUDE.md)
  │   │   └── Local (.claude/CLAUDE.md，gitignore)
  │   ├── 自动发现
  │   │   ├── 父目录递归查找
  │   │   └── 多文件合并加载
  │   └── /memory 命令
  │       ├── 添加记忆到 Project CLAUDE.md
  │       └── 添加记忆到 User CLAUDE.md
  │
  ├── ⚙️ Settings 配置系统
  │   ├── 配置层级
  │   │   ├── Enterprise Policy (强制/默认)
  │   │   ├── User (~/.claude/settings.json)
  │   │   ├── Project (.claude/settings.json)
  │   │   └── Local (.claude/settings.local.json)
  │   ├── 权限控制
  │   │   ├── allowedTools / disallowedTools
  │   │   ├── permissions (Bash/Edit/Write 规则)
  │   │   └── trustProjectSettings
  │   └── 行为配置
  │       ├── model (sonnet/opus/haiku)
  │       ├── contextCompaction
  │       └── thinkingMode
  │
  ✅├── 🔧 Tools 工具集
  │   ├── 文件操作
  │   │   ├── Read (读取文件/图片/PDF/Notebook)
  │   │   ├── Write (写入新文件)
  │   │   ├── Edit (精确字符串替换)
  │   │   ├── NotebookEdit (Jupyter 单元格编辑)
  │   │   ├── Glob (文件模式匹配)
  │   │   └── Grep (内容搜索)
  │   ├── 系统操作
  │   │   ├── Bash (命令执行)
  │   │   └── Task (启动子代理)
  │   ├── 网络操作
  │   │   ├── WebFetch (抓取网页)
  │   │   └── WebSearch (网络搜索)
  │   ├── 任务管理
  │   │   ├── TaskCreate / TaskUpdate / TaskList / TaskGet
  │   │   └── TodoWrite (待办事项)
  │   └── 交互操作
  │       ├── AskUserQuestion (多选问答)
  │       └── EnterPlanMode / ExitPlanMode
  │
  ✅├── 🤖 Subagents 子代理
  │   ├── 内置代理
  │   │   ├── Explore (代码库探索)
  │   │   ├── Plan (架构规划)
  │   │   ├── Bash (命令执行专家)
  │   │   └── general-purpose (通用任务)
  │   ├── 自定义代理 (agents.toml)
  │   │   ├── 工具限制配置
  │   │   ├── 模型选择
  │   │   └── 提示词模板
  │   └── 执行模式
  │       ├── 前台同步
  │       └── 后台异步 (run_in_background)
  │
  ✅├── ⚡ Skills 技能系统
  │   ├── 技能定义
  │   │   ├── ~/.claude/skills/<name>/skill.md
  │   │   └── 项目级 .claude/skills/
  │   ├── 技能组成
  │   │   ├── 名称和描述
  │   │   ├── 触发条件 (自动/手动)
  │   │   ├── 文件上下文
  │   │   └── 执行指令
  │   └── 调用方式
  │       ├── /skillname 斜杠命令
  │       └── 自然语言触发
  │
  ✅├── 🪝 Hooks 钩子系统
  │   ├── 触发时机
  │   │   ├── PreToolCall (工具调用前)
  │   │   ├── PostToolCall (工具调用后)
  │   │   ├── Notification (通知事件)
  │   │   └── Stop (会话结束)
  │   ├── 钩子类型
  │   │   ├── command (Shell 命令)
  │   │   └── mcp (MCP 工具调用)
  │   ├── 匹配器
  │   │   ├── toolName
  │   │   ├── toolInput (正则匹配)
  │   │   └── event
  │   └── 钩子行为
  │       ├── 修改工具输入
  │       ├── 阻止执行 (decision: deny)
  │       └── 注入额外上下文
  │
  ✅├── 🔌 MCP 协议集成
  │   ├── 配置位置
  │   │   ├── ~/.claude.json (全局)
  │   │   └── 项目/.mcp.json (项目级)
  │   ├── 传输方式
  │   │   ├── stdio (本地进程)
  │   │   └── sse (远程服务)
  │   ├── MCP 能力
  │   │   ├── Tools (工具扩展)
  │   │   ├── Resources (资源访问)
  │   │   └── Prompts (提示模板)
  │   └── 常用 MCP
  │       ├── filesystem
  │       ├── supabase
  │       ├── linear
  │       └── 自定义服务
  │
  ├── 🎛️ Interactive Mode 交互模式
  │   ├── 斜杠命令
  │   │   ├── /help - 帮助
  │   │   ├── /clear - 清除上下文
  │   │   ├── /compact - 压缩上下文
  │   │   ├── /memory - 管理记忆
  │   │   ├── /config - 配置设置
  │   │   ├── /mcp - MCP 管理
  │   │   ├── /cost - 查看花费
  │   │   └── /doctor - 诊断问题
  │   ├── 快捷键
  │   │   ├── Ctrl+C - 取消
  │   │   ├── Ctrl+L - 清屏
  │   │   ├── Escape - 中断
  │   │   └── Tab - 自动补全
  │   └── 输入模式
  │       ├── 单行输入
  │       ├── 多行输入 (Shift+Enter)
  │       └── 文件拖放 (@file)
  │
 ✅ ├── 💻 CLI 命令行
  │   ├── 启动模式
  │   │   ├── claude (交互式)
  │   │   ├── claude -p "prompt" (单次)
  │   │   ├── cat file | claude (管道)
  │   │   └── claude --resume (恢复会话)
  │   ├── 核心参数
  │   │   ├── --model (模型选择)
  │   │   ├── --allowedTools (工具白名单)
  │   │   ├── --permission-mode (权限模式)
  │   │   └── --output-format (输出格式)
  │   └── 输出格式
  │       ├── text (纯文本)
  │       ├── json (结构化)
  │       └── stream-json (流式)
  │
  └── 🧩 Plugins 插件系统
    │   ├── 插件组成
  │   │  ✅ ├── skills/ (技能)
  │   │   ├── commands/ (斜杠命令)
  │   │   ✅  ├── agents/ (子代理)
  │   │   ├── hooks/hooks.json (钩子)
  │   │   ├── .mcp.json (MCP 服务)
  │   │   └── .lsp.json (LSP 语言服务)
  │   ├── 安装方式
  │   │   ├── /plugin install (从 Marketplace)
  │   │   └── --plugin-dir (本地测试)
  │   └── 安装范围
  │       ├── User (全局可用)
  │       ├── Project (项目级)
  │       └── Local (仅本地)
```

## 五、生态

| | | |
| :--- | :--- | :--- |
| Marketplace (市场/商店) | 就像 App Store 或 Google Play | |
| Plugins (插件) | 就像具体的 App（如微信、抖音） | |
| [[skills]] | | |
| [[MCP]] | 1. 发小红书 | |

## 六、沙箱机制

https://code.claude.com/docs/zh-CN/sandboxing
文件系统隔离

## 七、超快模式

[[Groq]]等的思路，在 Agentic Loop 中，更快的响应速度很重要
https://code.claude.com/docs/zh-CN/fast-mode