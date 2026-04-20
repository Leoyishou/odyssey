---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡Summary

> [!Info]
> 一句话说清楚

1. `script` 是原子能力，单一职责
2. skills 是面向场景的单一职责原则，才能解放模型 Agent 能力（本质是[[FaaS]]）
3. [[bu维度的skills隔离]]，面向场景能交付结果。（基于 Function 的灵活调用，涌现出智能结果）
  - 原子化：一个 Skill 只做一件事，做到不可再分
  - bu 是涌现的：不要提前设计 bu，让它从高频组合中自然涌现

三大核心组件定义：
- Commands = 轻量提示模板，注入现有上下文
- Skills = 可配置、预加载、支持上下文分叉
- Subagents = 独立隔离上下文的自主执行者

高价值建议（可能我们还没做的）：

1. 60k tokens 硬清除策略 — 上下文衰减是失败的首要原因，在 60k tokens 或 30% 阈值处强制 /clear
2. Plan-First 工作流 — 编码前必须完成规划，用 Plan Mode 产出结构化文档
3. 5个标准 Slash Commands：/custom-init（生成 CLAUDE.md）、/commit、/issue、/reviewpr、/test
4. 质量门禁 Hooks — 只做构建检查和测试，不做自动格式化（太耗 token）

反模式（要避免的）：
- 重 MCP 依赖（>20k tokens 会压垮上下文）
- 复杂多 Agent 系统（调试噩梦）
- 自动格式化 Hooks（消耗过多 tokens）

# 🧩 Cues

# 🪞Notes

## 六、最佳实践

2026年真实社区共识 Top 3 保命组合拳（每天必用）

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
| :--- | :--- | :--- | :--- | :--- |
| 3 | **/context** | ★★★★ | 实时查看当前上下文占用情况、哪些文件/MCP/历史在吃token，一看就知道谁是罪魁祸首 | 重度用户标配「先/context再决定/clear还是/compact」 |
| 1 | **/clear** | ★★★★★ | 直接清空当前会话历史，保留项目文件上下文，瞬间释放大量token，最常用保命/续命手段 | 几乎所有人，尤其是长任务时「每做完一件事就/clear」 |
| 2 | **/compact** | ★★★★☆ | 手动触发上下文压缩（比auto-compact更可控），保留关键信息，适合不想彻底清空时用 | Boris本尊、上下文管理党「/compact > auto」 |
| 4 | **/help** | ★★★★ | 显示所有可用命令（包括你自己加的自定义命令），新手/忘了命令时必点 | 所有人，新功能上线后狂点 |
| 5 | **/agents** | ★★★☆ | 管理/查看/创建子代理（Sub-Agent），2026年最火的上下文解决方案核心入口 | Sub-Agent党「不装子代理的都out了」 |
| 6 | **/bug** | ★★★ | 快速把当前会话打包发给Anthropic报告bug（尤其是奇怪的崩溃/卡死/乱压缩时） | 遇到诡异问题时的救命稻草 |
| 7 | **/init** | ★★☆ | （部分老版本/场景）初始化项目记忆或读取CLAUDE.md等关键文件，很多教程里推荐 | 日本/中文圈早期用户较多，现在逐渐被自定义取代 |
| 8 | **/permissions** 或 **/allowed-tools** | ★★☆ | 查看/修改工具权限（尤其是bash危险命令），安全党常用 | 企业用户、安全意识强的开发者 |