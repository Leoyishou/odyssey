---
date created: 2024-11-16
date modified: 2025-07-10
uid: ef8d8aea-a91b-4230-bb9a-0d45b309e5c4
---
1. 创建有状态的 thread、无状态的 assistant
2. 把 message 加到 thread 中
3. 选一个 assistant 来处理2 中的 thread，生成一个 run
4. 轮询当 run 的状态为完成时，从 thread 拿到新message

## Assistants Function Calling with Streaming

https://platform.openai.com/docs/assistants/tools/function-calling
