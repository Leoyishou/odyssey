---
date created: 2025-04-18
date modified: 2025-10-25
uid: 37099cfd-444f-4379-8fc3-e12e26592f2c
---

# Summary

本质是[[责任链模式]]中的全局共享变量，生命周期：从传给`.invoke(inputs)` 的初始 `state` 开始，到执行结束返回最终 state。

# Notes
## 一、有两种 State

结论：主要用`MessagesState`
- **`MessagesState`** → 最小可用对话状态_，适合“自己造轮子”或需要高度可定制。
- **`AgentState`** → _官方 React‑Agent 的“流水线状态”_，用框架即配即用，字段更多但可定制性相对低。

```Java
class State(MessagesState):
    """State for the agent system, extends MessagesState with next field."""
    # 原始的chat request
    chat_request: ChatRequest

    # =============== 前端参数 ===============
    invoke_source: str

    # 会话全局上下文
    transcript_context: TranscriptContext

    # 用户当前发送的消息内容
    human_input_content: str

    # Runtime Variables
    next: str
    full_plan: str
    conversation_id: str
    message_id: str
    workflow_id: str
    intent: str
    thinking: str
    error_info: Optional[Dict[str, Any]]
    # 入参
    request: ChatRequest
    trip_planner_get_sights_city: str
    route_id: str
    c: str
    common_param: Optional[Dict[str, Any]]
    user_location: Optional[Dict[str, Any]]
    drawer_chat_type: str
    qunar_info: Optional[Dict[str, Any]]
    skip_thinker: bool

    travel_route_planner_recommend_routes: Optional[Dict[str, Any]]

    # 规划结果
    # 二层酒店推荐接口返回的displayList原始数据
    trip_recommend_hotel_list: List[Dict[str, Any]]
    # 路线推荐接口返回的原始数据
    trip_recommend_route_list: List[Dict[str, Any]]
    # 机票和火车票推荐接口返回的原始数据
    trip_recommend_transport_list: Optional[Dict[str, Any]]
    trip_weather_list: List[Dict[str, Any]]

    debug: bool
    ext_map: Optional[Dict[str, Any]]

    transport_consult_sector_node_data: Optional[Dict[str, Any]]
    transport_consult_params: Optional[Dict[str, Any]]

    hotel_consult_sector_node_data: Optional[Dict[str, Any]]
    hotel_list_req: Optional[Dict[str, Any]]

    hotel_list_filters: Optional[Dict[str, Any]]
    hotel_list_filters_to_llm: Optional[Dict[str, Any]]
    max_recommend_hotel_size: int

    # 行程插入参数: 交通规划结果
    travel_planner_param_by_travel_transport_planner_node: Dict[str, Any]
    # 行程插入参数: 路线规划结果
    travel_planner_param_by_travel_route_planner_node: List[Dict[str, Any]]
    # 行程插入参数: 酒店规划结果
    travel_planner_param_by_travel_hotel_planner_node: List[Dict[str, Any]]

    # xhs导入旅行规划参数
    xhs_note_import_request: Dict[str, Any]
    trip_planner_get_route_request: Dict[str, Any]

    # 路线规划推荐元路线id
    travel_route_planner_recommend_meta_route_id: str

    # 路线生成来源
    route_type: str
    poi_list: List[Dict[str, Any]]

    # AI捏的路线
    generate_travel_guide_extract_route: Optional[Dict[str, Any]]
    # 生成路线亮点说明的参数
    route_recommendation_params: Optional[Dict[str, Any]]

    recall_route_result: Optional[Dict[str, Any]]
    travel_intention_data: POITravelIntention

    # reception结果
    reception_by_domain_result: Optional[str]
    reception_by_sector_result: Optional[str]
    reception_by_bert_result: Optional[str]
    reception_by_sector_node_default_domain: Optional[str]

    # 低价格机票搜索会话id
    low_price_flight_search_session_id: Optional[str]
    low_price_flight_search_next_page: bool

    # 低价格机票搜索价格趋势引导问题需要的出发日期
    first_flight_go_date: Optional[str]
    generate_route_use_embedding_xhs_note_result: Optional[Dict[str, Any]]
    recommend_route_list_result: Optional[Dict[str, Any]]
    generate_route_use_embedding_xhs_note_route_planner_route_display_list: Optional[List[Dict[str, Any]]]
    generate_route_use_embedding_xhs_note_route_planner_route_type: Optional[str]

    # 行程规划的目的地
    travel_plan_destination: Optional[str]
    structured_response: HotelResp  

    # ReAct必须字段
    remaining_steps: RemainingSteps

```
## 二、state_schema=State 是做什么的

```Java
  - 指定“图的状态结构”。告诉 LangGraph：状态里有哪些键、类型是什么。
  - 要求至少包含：
      - messages（对话消息）
      - remaining_steps（循环保护，见 graph/types.py:159）
  - 若你传了 response_format，且同时自定义了 state_schema，还必须在 State 中加上：
      - structured_response（类型可写成 HotelResp 或 dict），否则会报缺少必需键的错误。

```

### 两种用法
```Java
  - 保留自定义 State（强类型、字段受控）：
      - 在 graph/types.py 的 State 里新增一行：structured_response: HotelResp | dict（仅当你要用 response_format 时）
      - 调用：create_react_agent(llm, tools, prompt=prompt_fn, state_schema=State, response_format=HotelResp)
  - 不想改 State：
      - 不传 state_schema，让内部自动用带 structured_response 的内置 Schema：
      - create_react_agent(llm, tools, prompt=prompt_fn, response_format=HotelResp)
      - 你的自定义字段仍可随输入一起传入，但不做强类型校验。
```



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


    

希望这能帮你快速分清两者定位；如果还想看源码字段或具体示例，告诉我即可！
