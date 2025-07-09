---
comment_id: 1b9d5a2d
date created: 2025-04-18
date modified: 2025-04-18
draw: null
title: MessageState、AgentState
---
Thought for 5 seconds

下面用一句话先概括，然后再分开解释：

> **在 LangGraph 里，`MessagesState` 和 `AgentState` 都是“**状态容器**”（`TypedDict`），用来把**对话上下文**随节点在图中流动；前者是最精简的通用版本，后者是给官方预制 Agent 执行器（如 `create_react_agent`）准备的“加强版”。**

---

## 1 · MessagesState

|要点|说明|
|---|---|
|**定义位置**|`from langgraph.graph import MessagesState`|
|**本质**|`TypedDict`，只有一个键：`messages: list[BaseMessage]`（`BaseMessage` 是 LangChain 的统一消息抽象）|
|**设计目的**|给你一个**最小工作示例**：只要把「对话消息列表」放进来，图里每个节点都能读/写同一份历史记录|
|**典型用法**|`python<br>class State(MessagesState):<br> TEAM_MEMBERS: list[str]# 常量<br> next: str # 运行时字段<br> full_plan: str # …扩展字段<br>`→ 先继承，再按自己需要加字段|
|**适用场景**|你想自己完全掌控状态结构；或需要往状态里加很多自定义字段（正如 Taurus 项目里那样）|

---

## 2 · AgentState

|要点|说明|
|---|---|
|**定义位置**|`from langgraph.prebuilt.chat_agent_executor import AgentState`|
|**本质**|也是 `TypedDict`，字段比 `MessagesState` 丰富：- `messages`（同上）- `accumulated_response`（LLM 流式增量时的缓冲）- `intermediate_steps`（工具调用结果等）- ……可能还有 `next`、`plan` 等内部字段|
|**设计目的**|服务于官方“预制执行器”——例如 `create_react_agent` 或 `ChatAgentExecutor`。这些执行器会在运行时自动维护上述额外字段，因此你**无需手动写节点逻辑**就能得到带记忆、带工具调用链路的 Agent。|
|**典型用法**|当你调用：`python<br>agent = create_react_agent(llm, tools, …)`LangGraph 内部就把状态初始化为 `AgentState`，后续步骤都按它的字段约定读写。|
|**适用场景**|想省事，直接用官方封装好的 React‑style Agent；或对内部状态结构无需自定义。|

---

## 3 · 何时用哪个？

|需求|建议使用|
|---|---|
|**只要“消息列表”+ 少量自定义字段**，自己写节点路由|继承 `MessagesState` 手动扩展|
|**想快速拉起一个带工具调用、自动思考链的聊天 Agent** （例如 `create_react_agent`）|直接用框架自带的 `AgentState`|
|**已有复杂业务状态**，官方字段装不下|自定义 `State(MessagesState)`，完全掌控字段|

---

### 小结

- **`MessagesState`** → _最小可用对话状态_，适合“自己造轮子”或需要高度可定制。
    
- **`AgentState`** → _官方 React‑Agent 的“流水线状态”_，用框架即配即用，字段更多但可定制性相对低。
    

希望这能帮你快速分清两者定位；如果还想看源码字段或具体示例，告诉我即可！
