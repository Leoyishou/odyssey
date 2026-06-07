---
uid: 8484a1df-11c9-4957-9c22-00649e85db13
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 



# 🧩 Cues

[[【cc】Agent Team]]

# 🪞Notes

## 如何新建

1. agent 定义的 frontmatter
	1. name、
	2. description、
	3. model、
	4. color、
	5. memory 等。

## 不能直接在 agent 定义里指定可用的 skills。

  但有几种间接方式：

  2. 在 system prompt 里引导：在 agent 的 markdown 正文里写明"你应该使用
   XXX skill"，agent 会优先调用
  ## 可用工具
  - 数据分析时优先使用 `lakehouse` skill 查询本地 DuckDB
  - 需要查 Supabase 时使用 `chat-to-supabase` skill
  2. frontmatter 的 tools 字段：可以限制 agent
  能用哪些工具（Read、Write、Bash 等），但这是底层工具，不是 skills
  tools:
    - Read
    - Bash
    - Glob
    - Grep
  3. Skills 自身的触发机制：skills 的 triggers
  关键词匹配是全局生效的，agent 在对话中提到相关关键词时会自动加载对应
  skill

  所以实际做法是：在 agent 正文里明确写清楚它该用哪些 skill，靠 prompt
  引导而不是硬限制。