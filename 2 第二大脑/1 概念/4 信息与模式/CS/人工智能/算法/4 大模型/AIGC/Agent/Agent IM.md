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

内容 → 飞书消息类型

| 内容类型 | 飞书格式 | 样式 |
| :--- | :--- | :--- |
| 纯文字（无 markdown） | text | 普通文本 |
| 含代码块/表格（≤5个表格） | interactive card (schema 2.0) | 结构化卡片，代码高亮 |
| 多段落内容（≥2个空行） | post 富文本 | 段落排版 |
| 其他 markdown | post md 模式 | md 渲染 |

实时进度卡片里的颜色编码（就是你截图里那种）

| 内容 | 标签颜色 | 样式 |
| :--- | :--- | :--- |
| 💭 Thinking | 灰色 div | 小字，notation size |
| 工具调用（Tool Use） | 蓝色 <text_tag color='blue'> | 工具名 + 入参代码块 |
| 工具结果（Tool Result） | 青绿色 <text_tag color='turquoise'> | exit code + 输出 |
| 错误（Error） | 红色 <text_tag color='red'> | 错误信息 |

判断逻辑（buildReplyContent）：
无 markdown → text
有代码块/表格 → interactive card
多段落 → post
其他 → post(md)

# 🧩 Cues

# 🪞Notes

## 如何分配session
⏺ 不是绑死一个，是按"聊天上下文"分配 session。从数据里可以直接看出来：

mapping 逻辑

飞书 chat_id + user_id → cc-connect session (s1~s6) → CC session_id (UUID)

看实际数据：
oc_9e7c48b53... (群A) + ou_762... → s6 → 2b41cea4
oc_ed648df8b... (群B) + ou_762... → s5 → (空)
oc_ed648df8b... (群B) + ou_75275... → s2 → 47e1654d
oc_0aa249d9a... (群C) + ou_762... → s4 → 17fe4f04

本质是：
- 每个 (群聊 + 用户) 组合 → 独立 session
- 同一个群里不同用户各自有独立的 CC session
- 同一个用户在不同群里也是独立 session

session 生命周期：
- 首次发消息 → 新建 cc-connect session → 启动新 Claude Code 进程（新 session_id）
- 后续发消息 → 找到对应 cc session → 用 claude -r <session_id> resume 同一个 CC session
- 所以同一个群聊里和 bot 的对话，历史是完全连续的

不是"一个 bot 绑一个 session"，而是"每个独立的对话上下文绑一个 CC session"——跟你在 Claude.ai 网页上每个对话窗口是一个 conversation 是一个概念。

## 如何新建session
源码里 `/new` 应该做的三件事：

1. 清空旧 session 的 agent_session_id（置为空字符串）
2. 清空 cc-connect 这层的对话历史
3. 新建一个 cc-connect session（s7、s8...）

下次发消息时，cc-connect 发现 agent_session_id="" → 启动全新 claude 进程 → 生成新的 CC session UUID。这才是真正的上下文重置。

---