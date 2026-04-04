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

## Gateway
我之前是飞书消息到cc session，但是openclaw的设计其实是硬编码转发，这一层并没有llm驱动
`if 消息来了 → 查 session 表 → 拼 prompt → POST /v1/messages → 转发回复`

Gateway 本质上就是个路由器：
- 纯 if-else + HTTP 调用，没有任何智能。所有智能都在 API 端。
- 而你现在用的 feishu-channel，Claude Code 本身就是 LLM 在运行，消息进来直接在上下文里处理，不需要绕一圈调 API。Pro 订阅额度，不额外花钱，更适合你。

## Agent 层的for循环是自己写的
就这么个 while 循环。Claude Code 本质上也是这个循环，只不过 Anthropic 帮你把工具执行、权限系统、context 管理、MCP 集成全做好了。

OpenClaw 自己写这层，好处是完全可控（没有权限弹窗），代价是得自己维护工具实现、错误处理、上下文裁剪、token 计数这些脏活。

## tool sandbox
就是把原生的 bash read write fetch用代理模式包了一层，加了一些权限校验

```javascript
  // 伪代码，大意
  function executeTool(name, input) {
    // Bash：只允许白名单命令
    if (name === "bash") {
      if (input.command.startsWith("rm -rf /")) throw "禁止"
      if (input.command.includes("curl") && !allowNetwork) throw "禁止"
      return exec(input.command, { timeout: 30000, cwd: sandboxDir })
    }

    // 文件：只允许读写指定目录
    if (name === "write") {
      if (!input.path.startsWith(allowedDir)) throw "路径越界"
      return fs.writeFile(input.path, input.content)
    }
  }
```

## 记忆模块

| | Claude Code Memory | OpenClaw Memory |
| --- | --- | --- |
| 存储 | Markdown 文件 | SQLite + FTS5 |
| 检索 | 无检索，全量加载 | 关键词/向量搜索 |
| 容量上限 | ~200 行索引（实际靠 LLM 自己裁剪注意力） | 理论无限（检索 top-k） |
| 额外成本 | 0 | embedding API（如果开向量） |
| 智能程度 | 靠 LLM 自己判断相关性 | 靠算法预筛 |

chunks 表：

| id | text | embedding | ... |
| --- | --- | --- | --- |
| c_001 | 投资系统… | [0.0123, -0.0456, 0.0789, 0.0012, ...(1536个浮点数)] | path, start_line, ... |
| c_002 | 英语学习… | [-0.034, 0.067, 0.012, ...] | |

检索时就是暴力算余弦相似度：

-- 伪逻辑，SQLite 没有原生向量运算
-- 实际是在应用层用 JS/Python 算的

1. 用户消息 → 调 embedding API → 得到 query_vec
2. SELECT id, text, embedding FROM chunks
3. 逐行 cosine_similarity(query_vec, row.embedding)
4. 取 top-k 最相似的塞进 prompt

没用专门的向量数据库（Pinecone/Milvus），就是 SQLite 当存储 + 应用层暴力搜索。数据量小的时候够用，几万条 chunks 以内毫秒级。

embedding_cache 表则是避免同一段文字重复调 API：

"投资系统需要仓位管理" → hash: abc123 → 已缓存，直接用，不再花钱调 API

# 🧩 Cues

# 🪞Notes