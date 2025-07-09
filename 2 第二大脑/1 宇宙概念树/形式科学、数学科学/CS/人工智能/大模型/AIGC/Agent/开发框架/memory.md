---
comment_id: 082dd576
date created: 2025-04-18
date modified: 2025-04-18
draw: null
title: memory
---
**Searching for memory patterns**

I’m thinking of browsing the web to look up memory patterns in LangGraph and large language model (LLM) frameworks. I bet it’ll offer insights into how memory systems are integrated into these models. I’ll search for relevant information that covers how these systems handle and optimize memory—hopefully, it’ll clarify how I can improve how I manage information for users and become more efficient in those tasks. Let's find some good resources!

**Synthesizing memory concepts**

I’ve gathered information on how LangGraph handles memory, conversation history, and persistence, including long-term memory. I need to break down three memory levels: per-session, thread-level, and long-term. For thread-level, I’ll look into tools like MemorySaver and the use of checkpointers with options for Redis, SQLite, Pinecone, etc. I also want to explore methods for reducing memory size like using reducers to compress and summarize data, while also mapping it to session variables. I’ll aim to write clear sections and provide code examples.

Thought for 17 seconds

## 在 LangGraph 里，“Memory” 究竟指什么？

> **核心思路**：所有“记忆”都以 `State` 里的字段存在；
> **持久化**与**裁剪**都是在编译图 (`graph.compile`) 时挂载的附加功能。
>
> LangGraph 官方把记忆分成三层：

|层次|生命周期|关键词|典型实现|
|---|---|---|---|
|**短期（本次调用）**|只在一次 `invoke/astream_events` 中|`State` dataclass|普通 Python 变量|
|**线程级（Session/Conversation）**|多次调用、同一个 `thread_id`|`checkpointer`=MemorySaver/Redis/SQLite|自动序列化 State|
|**长期（跨线程）**|任意时刻可检索|`MemoryRetriever` / `VectorStore`|Pinecone、Weaviate…|

官方文档说明 short‑term ↔ long‑term 的界限及用法 citeturn0search0turn0search6

---

## 1 ️⃣ 短期记忆：纯粹靠 State

```python
from langgraph.graph import StateGraph
from typing import TypedDict

class ChatState(TypedDict):
    messages: list

g = StateGraph(ChatState)
# 在任意节点里：state["messages"].append(...)
```

- 每个节点收到的 `state` 仅存在于 **当前这一次** 调用。
    
- 如果你只做 _一次问答_，不需要持久化就够了。
    

---

## 2 ️⃣ 线程级记忆：`checkpointer` 让多个调用“接上话”

### 最简示例：内存 Saver

```python
from langgraph.checkpoint.memory import MemorySaver
checkpointer = MemorySaver()         # 线程安全的字典

graph = g.compile(checkpointer=checkpointer)

thread_id = "u123"                   # 同一次对话必须一致
graph.invoke({"messages": [...]}, config={"thread_id": thread_id})
graph.invoke({"messages": [...]}, config={"thread_id": thread_id})  # 自动续聊
```

- **原理**：每走完一条路径，LangGraph 把 `state` 序列化→存储；下次同 `thread_id` 会自动加载。
    
- 同一时刻可跑 **多个并发线程**，互不影响。
    
- 生产环境常用 **SQLiteStore / RedisStore**：
    

```python
from langgraph.checkpoint.sql import SqliteCheckpointer
checkpointer = SqliteCheckpointer("sqlite:///chat.db")
```

> `MemorySaver`、`SqliteCheckpointer` 等实现详见 persistence how‑to citeturn0search5

---

## 3 ️⃣ 长期记忆：写入向量库，随时召回

```python
from langgraph.memory import MemoryRetriever
from langchain_community.vectorstores import Pinecone

retriever = MemoryRetriever(
    vectorstore=Pinecone(index_name="long_mem"),
    namespace="user_profile"         # 可按主题/用户分命名空间
)

# 把 node 产出的摘要写入长期记忆
def write_long_term(state):
    retriever.write(state["summary"])

# 在主 LLM 节点前，先检索相关长期记忆
def recall(state):
    memories = retriever.retrieve(state["query"], k=5)
    state["memories"] = memories
    return state
```

- 与 LangChain `VectorStoreRetriever` 用法几乎一致。
    
- 可同时启用 thread‑level + long‑term；线程结束后仍可查到历史知识。
    

官方博客宣布长期记忆功能 citeturn0search6

---

## 4 ️⃣ 控制上下文窗口：裁剪与总结

> 对话越长，`messages` 越占 Token；**裁剪策略** 是 memory 的核心难点 citeturn0search1

常用模式：

|策略|代码/要点|适用场景|
|---|---|---|
|**固定窗口**|`state["messages"]= state["messages"][-N:]`|快速 Demo|
|**Token 计数器**|`langchain.text_splitter` 统计 GPT‑3.5 token；超限时 pop|小团队内部工具|
|**递增摘要**|新增 `summary` 字段；当 `len(messages)>k` → 调用 LLM 总结后替换前半段|客服、长对话|
|**向量化压缩**|旧消息写入向量库，仅保留最近 N 条；回答前检索|问答机器人|

**示例节点**（摘要）：

```python
def maybe_summarize(state):
    if len(state["messages"]) > 20:
        summary = summarizer(state["messages"][:-10])   # 调 LLM
        state["messages"] = [{"role": "system", "content": summary}] + state["messages"][-10:]
    return state
```

---

## 5 ️⃣ 多 Agent 场景的记忆隔离与共享

1. **各 Agent 单独的 State 字段**
    
    ```python
    class TeamState(TypedDict):
        coordinator: list
        planner: list
        reporter: list
    ```
    
2. **共享知识库**
    
    - 用同一个 `checkpointer` + 不同 **namespace**：`coordinator.history` / `planner.history`
        
3. **消息总线**
    
    - 在中心节点做 _merge_ / _broadcast_，决定哪些消息同步到其他 Agent。
        

---

## 6 ️⃣ 常见坑 & 最佳实践

|问题|解决方案|
|---|---|
|**历史爆炸**：Context 超限、成本飙升|审计 Token → 摘要 or Sliding‑Window|
|**并发覆盖**：多个同 `thread_id` 并发写|为写操作加 `asyncio.Lock` 或用 Redis 原子操作|
|**持久层崩溃**：SQLite 锁、Redis 丢失|生产用外置 DB＋定期备份／监控|
|**隐私合规**：长期记忆含 PII|加密字段、设置 TTL、按用户 namespace 清除|

---

### 结论

- **记忆 = State + Checkpointer +（可选）Retriever**。
    
- 先确定 **对话生命周期**，再选 **短期 / 线程级 / 长期** 组合。
    
- 任何时候都要准备 **裁剪策略**，防止上下文失控。
    

有了这些模板，你就能在 LangGraph 中灵活地“记住 & 忘掉”信息，既保证对话连贯，又控制成本与隐私。祝你构建的多 Agent / Workflow 写入&读取记忆都稳如老狗！

Thought for a couple of seconds

### “线程”在 LangGraph/对话系统里的意思（通俗版）

> **一句话**：这里的 **线程 (thread)** 不是操作系统里的“CPU 线程”，而是指 **一次连续的对话或任务会话**——给它起个 `thread_id`，LangGraph 就能把同一次会话的“记忆”串起来。

---

#### 为什么叫线程？

- 想像在 **论坛** 里发帖：
    
    - 一篇主帖下面所有回复都属于同一个“帖（Thread）”。
        
    - 你可以随时回来接着聊，论坛会把前面的内容都找回来。
        
- LangGraph 借用了同样的词：
    
    - 你第一次调用 `graph.invoke(..., thread_id="abc")` 就像“开帖”。
        
    - 以后任何时候再次传入同样的 `thread_id="abc"`，就像“在同一个帖下回帖”。
        
    - 框架会自动把之前保存的 **State（记忆）** 载入，让对话“连得上”。
        

---

#### 它和“操作系统线程”有啥区别？

|LangGraph 线程|OS 线程|
|---|---|
|**概念**：一段对话 / 工作流会话|CPU 并行执行的最小调度单元|
|**粒度**：逻辑层面，决定上下文|系统层面，决定并行度|
|**作用**：找回记忆、隔离不同聊天|提升并行计算性能|
|**冲突**：同 `thread_id` 并发写入需注意锁|CPU 资源竞争、同步问题|

> 换句话说：LangGraph 用“线程”来**标识会话**；操作系统用“线程”来**跑代码**。两者只是名字撞车了，本质完全不同。

---

#### 你什么时候需要关心 `thread_id`？

1. **多轮聊天机器人**
    
    - 为每位用户分配独一无二的 `thread_id`；下次用户再来，记忆自动续接。
        
2. **Agent Workflow 多步任务**
    
    - 比如 ReAct：思考 → 调用工具 → 回复，人类每输入一次就产生一次调用，但同一个工单必须共享上下文。
        
3. **批量评测 / A/B 测试**
    
    - 同一输入在不同模型上跑对比，用同一个 `thread_id` 却配不同 `config`，可保持历史一致性。
        

---

#### 实操小提示

```python
thread_id = f"user_{user_id}"          # 每位用户一个
config = {"thread_id": thread_id}

# 第一次
graph.invoke({"msg": "你好"}, config=config)

# 第二次：LangGraph 自动把上次 State 带回来
graph.invoke({"msg": "继续刚才的话题"}, config=config)
```

- **不传 `thread_id`** → LangGraph 会随机生成，等于每次“开新帖”，记忆不会延续。
    
- **存储介质**：默认内存 (`MemorySaver`)；生产推荐 **SQLite / Redis** checkpointer。
    
- **并发调用**：如果多个请求 _同时_ 使用同一 `thread_id`，需要加锁或用 Redis 的原子操作，避免“回帖撞车”。
    

---

#### 一句话收尾

> 在 LangGraph 里，“线程”=“对话/任务的专属编号”，**用来串起往返的上下文**；和操作系统层面的并行线程完全不是一回事，只是沿用了论坛式的命名。希望这样解释后，你再看到 `thread_id` 就不会混淆啦！
