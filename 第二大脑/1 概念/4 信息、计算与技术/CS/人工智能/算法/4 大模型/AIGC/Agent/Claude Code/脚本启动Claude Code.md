---
uid: 935752aa-65fc-42bb-9153-a75210d75b12
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

## 


## 


`claude --bare` 的意思不是“裸模型”，而是 Claude Code 的“最小化启动模式”。

官方定义里，`--bare` 会跳过自动发现这些东西：
- hooks
- skills / slash commands
- plugins
- MCP servers
- auto memory
- `CLAUDE.md`

这样做的效果是：
- 启动更快
- 更适合脚本 / CI
- 结果更可复现，不会吃到本机 `~/.claude` 或项目目录里的隐式配置

但它不是完全没能力。官方说明 bare 模式下仍然有：
- `Bash`
- 文件读取
- 文件编辑

你们这次卡住的关键点也和官方文档一致：`--bare` 会跳过 OAuth 和 keychain 读取，所以认证不能走平时的 Claude 登录态；Anthropic 直连时需要 `ANTHROPIC_API_KEY`，或者在 `--settings` 里提供 `apiKeyHelper`。这也解释了为什么你只有 `ANTHROPIC_AUTH_TOKEN=rly_...` 时，`--bare` 的 auth 路径会偏掉。

所以一句话总结：
- `--bare` = 关闭 Claude Code 的“环境自动注入层”
- 不是关闭工具，也不是只保留纯聊天
- 特别会影响你现在关心的两件事：配置来源和认证来源

参考：
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Run Claude Code programmatically / bare mode](https://code.claude.com/docs/en/headless)

结合你现在的现象，方向 2 的判断是合理的：不用 `--bare`，能先复用现有 proxy / auth 链路，最快验证问题是不是 bare 模式把认证和 beta/header 路径切坏了。
# 🧩 Cues



# 🪞Notes
