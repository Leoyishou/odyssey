---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-15
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

本质是一个[[Shell]]命令行工具，用来发 HTTP 请求。

```shell
curl --location --request POST 'http://localhost:8000/chat/stream' \
--header 'Content-Type: application/json' \
--data-raw '{
  "message": "帮我规划一个北京三日游",
  "session_id": "test-001"
}'
```

# Cues



# Notes

你这个问题，本质上是**“协议与数据格式”辨析的经典问题变体**。

---

## 拆开来看

1. **curl**
    
    - 是一个命令行工具，用来发 HTTP 请求。
        
2. **--location --request POST '[http://localhost:8000/chat/stream](http://localhost:8000/chat/stream)'**
    
    - 表示走 **HTTP 协议**，方法是 `POST`，目标是 `http://...`。
        
    - 所以传输协议是 **HTTP/1.1 或 HTTP/2**（由服务器和客户端协商）。
        
3. **--header 'Content-Type: application/json'**
    
    - 表示请求体里数据的格式是 **JSON**。
        
    - 这告诉服务器“我发给你的内容是 JSON，请按 JSON 来解析”。
        
4. **--data-raw '{... }'**
    
    - 实际上传输的数据内容，符合 JSON 语法。
        

---

## 结论

这个 curl 请求的本质是：

- **传输层面**：用 **HTTP 协议**（POST 方法）。
    
- **数据内容层面**：用 **JSON 格式**作为请求体。
    

换句话说：  
它不是一种新的协议，而是“HTTP 协议 + JSON 格式”的常见组合。

---

要不要我帮你画一个简单的分层图（从网络协议栈到底层 JSON 数据）来更直观理解？
