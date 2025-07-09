---
aliases:
- Assistants Function Calling with Streaming
comment_id: 592ae7fe
date created: 2024-11-16
date modified: 2024-12-27
draw: null
linter-yaml-title-alias: Assistants Function Calling with Streaming
tags: []
title: Assistants Function Calling with Streaming
---
1. 创建有状态的 thread、无状态的 assistant
2. 把 message 加到 thread 中
3. 选一个 assistant 来处理2 中的 thread，生成一个 run
4. 轮询当 run 的状态为完成时，从 thread 拿到新message

## Assistants Function Calling with Streaming

https://platform.openai.com/docs/assistants/tools/function-calling
