---
draw:
title: graph 执行
date created: 2025-04-18
date modified: 2025-04-18
---

| 执行                        |                                                  |
| ------------------------- |:----------------------------------------------- |
| 同步阻塞式一次性执行；返回最终 State 增量。| `result = graph.invoke({'messages':[...]}, cfg)` |
|                           |                                                  |

**Explaining invoke vs. astream_events**

I’m thinking about how to explain the difference between `Graph.invoke` and `Graph.astream_events`. `invoke` returns a final result synchronously (like a dict), while `astream_events` is asynchronous, offering live updates as a stream. I’ll touch on when to use each, typical parameters, and the need for additional logic with `astream_events` to capture final states or accumulate updates. A simple code example sounds helpful! Also, I shouldn’t forget best practices for error handling, debugging, and custom events. A table should make things clearer!

Thought for 26 seconds

下面用 **一句话对比** 开门见山：

> **`graph.invoke()` = “一口气跑完，直接给最终 State”；
> **`graph.astream_events()` = “边跑边播，任何子节点/LLM 的 token、调试信息全都实时推流回来”。**

---

## 快速对照表

|方法|主要用途|同/异步|返回值/迭代值|最常用参数|典型场景|
|---|---|---|---|---|---|
|`graph.invoke(input, **kw)`|一次性获取**最终输出**|同步|`dict` 或任意类型（取决于 `State` 定义）|`stream_mode`（默认为 `"values"`）、`output_keys`、`debug`|CLI 脚本、单元测试、需要阻塞拿到结果的后端调用|
|`graph.astream_events(input, **kw)`|**实时流式**地拿到 **所有** 事件（Start/Stream/End）|异步|`AsyncIterator[StreamEvent]`|`version="v2"`（默认）、`include_* / exclude_*` 过滤、`debug`|WebSocket 聊天、进度条、Token‑by‑Token 推流、细粒度监控/可观测性|

_官方签名与说明见 LangGraph API 参考_ citeturn9view0turn4view0

---

## `graph.invoke()`——阻塞式一次运行

```python
result = graph.invoke(
    {"question": "What is the capital of France?"},
    stream_mode="values"      # 默认为 "values"
)
print(result["answer"])       # => "Paris"
```

- **同步阻塞**：调用直到整张图跑完才返回。
    
- **`stream_mode`** 决定内部是如何“跑”的，但**最终仍只给你最后一次 State**（例如整条对话消息列表）。
    
- **可选快门**：
    
    - `output_keys=["answer"]` 只想要部分字段。
        
    - `debug=True` 会在控制台打印调试信息，等同于 `graph.stream(..., debug=True)`。
        
- **最适合**：写脚本、单元测试、任务队列 Worker 等“一问一答”场景。
    

---

## `graph.astream_events()`——实时事件流

```python
async for ev in graph.astream_events(
    {"question": "What is the capital of France?"},
    stream_mode="messages",           # v0.2.60+，专门推 token
    include_types=["llm", "tool"],    # 只关心 LLM & Tool 事件
):
    if ev["event"].endswith("_stream"):          # token 或自定义 stream
        print(ev["data"]["chunk"], end="", flush=True)
```

### 事件结构（v2 版默认）

```json
{
  "event": "on_llm_stream",
  "name": "ChatOpenAI",
  "run_id": "...",
  "parent_ids": ["..."],        // 调用链
  "tags": ["agent=a"],
  "metadata": {...},            // 例如模型名、温度
  "data": { "chunk": "Par" }    // token / 自定义数据
}
```

- **全链路**：不仅能看到最终 State，还可以捕获**每个节点**的 `start / stream / end`、自定义 `StreamWriter` 事件。
    
- **过滤器**：用 `include_names/types/tags` 或相应 `exclude_*` 精准订阅。
    
- **版本**：`version="v2"` 是 0.2.x 新增的重写版，字段更统一、效率更高；v1 将在 0.4.0 被移除 citeturn0search3。
    
- **拿最终输出**：
    
    - 监听 `event == "on_graph_end"`；
        
    - 或者自己累积 `updates`（当 `stream_mode="updates"`）。
        

---

## 选型建议

1. **Web/前端流式体验**
    
    - 用 `astream_events`，`stream_mode="messages"` + WebSocket，边打字边显 token。
        
2. **后端 API**
    
    - 若只需结果 ➜ `invoke`，更简单。
        
    - 需要进度/重试/检查点 ➜ `astream_events` + `debug` or `stream_mode="debug"`。
        
3. **离线批量**
    
    - `graph.batch()` / `graph.abatch()` 并行跑多份输入，再配合 invoke 或 astream_events。
        

---

## 与 `graph.stream / graph.astream` 的区别

- `stream/astream` 只按 **`stream_mode`** 推 **State 更新值**，事件粒度比 `astream_events` 粗。
    
- 如果只是想要 **状态快照**，用它们就够；若要 **更底层的 LLM Token / 调用树**，才用 `astream_events`。
    

---

### 小贴士

- **自定义事件**：在节点里注入 `from langgraph.types import StreamWriter`，调用 `writer({...})` 就能让前端实时可视。
    
- **异常管理**：在 `include_types=["error"]` 时可以实时捕获栈追踪并回传。
    
- **性能**：`astream_events` 如果同时开启 `stream_mode="debug"`，调试信息非常多，线上慎用。
    

---

这样就能快速区分什么时候用 `invoke`、什么时候用 `astream_events`，并且掌握核心参数与最佳实践。祝你在 LangGraph 的流式 & 同步调用之间游刃有余！

**Looking at the code snippet**

The user asked how `Graph.astream_events()` is used in their code. To answer, I’ll need to review the code they’ve already provided. I’ll look at the context and see how this function is being applied. Once I’ve identified where and how it’s used, I'll provide a clear explanation of its role in their code, focusing on its purpose and functionality.

Thought for 6 seconds

在 **Taurus** 里，`Graph.astream_events()` 只在一个地方出现——`app/services/workflow_service.py` 中的 `run_agent_workflow()`。

下面把调用语境拆开讲，帮你迅速看懂「**是谁** 在什么时机 **怎么用** 它」。

---

## 1 调用位置

```python
# app/services/workflow_service.py
async for event in graph.astream_events(
    {
        # Constants（写死不变）
        "TEAM_MEMBERS": TEAM_MEMBERS,

        # Runtime variables（每次请求动态注入）
        "messages": user_input_messages,
        "deep_thinking_mode": deep_thinking_mode,
        "search_before_planning": search_before_planning,
    },
    version="v2",          # ✅ LangGraph 事件流新格式
):
    ...
```

- `graph` 是全局提前 `graph = build_graph()` 得到的 **已编译工作流**。
    
- 传进去的是 **初始状态字典**；等价于给节点的 `state` 参数。
    
- `version="v2"` 表示使用第二代事件协议（事件字段更统一）。
    

---

## 2 循环体在干什么

```python
kind     = event.get("event")      # 事件类别  e.g. on_chain_start
name     = event.get("name")       # 触发者   e.g. planner
metadata = event.get("metadata")   # 附带元数据
node     = metadata.get("checkpoint_ns", "").split(":")[0]
```

> **核心思路**：把 LangGraph 的“内部事件”**翻译**成前端或 SSE 能理解的“外部事件”。

1. **根据 `kind` 和 `name` 分类**
    
    - `on_chain_start / end` → 调用 `EventHandler.handle_chain_start/end`
        
    - `on_chat_model_*`      → 映射为 “start_of_llm / end_of_llm / message / thinking”
        
    - `on_tool_*`            → 变成 “tool_call / tool_call_result”
        
2. **把翻译后的结果 `ydata` 通过 `yield` 搬出去**  
    这就是 `EventSourceResponse`（SSE）或测试里 `async for` 观察到的实时流。
    
3. **特殊收尾**
    
    - 如果协调者（Coordinator）判定“handoff”场景，循环结束后再补一个 `end_of_workflow`。
        

---

## 3 宏观执行流程图

```Java
FastAPI / 测试代码
     │
     │ 1. await run_agent_workflow(...)
     ▼
run_agent_workflow()
 ├─ build_graph()           # 生成编译好的 LangGraph
 ├─ EventHandler(...)       # 负责格式转换
 └─ async for event in graph.astream_events(state):
        ├─ 对不同 kind 调用 EventHandler
        └─ yield 外部事件  ────────► SSE / 调用方实时收到
```

---

## 4 关键信息点

|细节|说明|
|---|---|
|**为什么用 `astream_events` 而不是 `ainvoke`**|需要“边运行边推送”——每个 LLM token、工具调用、节点开始/结束都能实时流给前端|
|**`streaming_llm_agents`**|只对 `TEAM_MEMBERS + planner + coordinator` 这些节点的 LLM 事件做转换，别的节点默认忽略|
|**EventHandler 的作用**|把 LangGraph 原生事件 → 业务自定义 JSON（`start_of_agent`、`message`、`tool_call`…）|
|**最终返回**|一个 async generator，FastAPI 用 `EventSourceResponse` 把它当 **SSE** 流返回给浏览器|

---

### TL;DR

`Graph.astream_events()` 在 Taurus 中就是 **工作流驱动器** + **事件泵**：

- 它负责按 LangGraph 图的定义依序执行节点；
    
- 每走一步就抛出一条结构化事件，让外层 `run_agent_workflow()` 能把这些事件实时转发给前端或测试代码。若想调试或扩展，只要在 `run_agent_workflow()` 的事件分支里加/改处理逻辑即可。
