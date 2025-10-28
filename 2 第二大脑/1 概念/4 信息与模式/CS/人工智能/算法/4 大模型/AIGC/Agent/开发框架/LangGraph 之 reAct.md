---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-25
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  



# 🧩 Cues



# 🪞Notes

## React 实现（以 Hotel Agent 为例）

### 1.1 情况一（标准版 n + 1 次 LLM）

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F10%2F25%2F01-42-56-d9045b3e8605671d93dea1df6b350406-202510250142729-6b7bba.png)
红色的部分重复 n 次，
```Java
call_model(LLM) → should_continue 判定 → 输出 “tool”     +     tool API
call_model(LLM) → should_continue 判定 → 输出 “tool”     +     tool API
call_model(LLM) → should_continue 判定 → 输出 “tool”     +     tool API
					           ……
					           
```
最后以 `should_continue=end` 收尾
```Java
call_model(LLM) → should_continue 判定 → 输出 “tool”     +     tool API
call_model(LLM) → should_continue 判定 → 输出 “end”。
```
总计 n + 1 次 LLM 调用

### 1.2 情况二（n + 1 + 1 次 LLM）

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F10%2F25%2F02-29-38-bca2530f375d0a2558e3781eadd4fa64-202510250229931-dc7252.png)

如图所示左边第一个红框内的调用，和情况一同样的循环，只是最后不输出"end"，而是输出'generate_structured_response'
```Java
LLM(Agent Prompt) → should_continue 判定 → 输出 “tool”     +     tool API
LLM(Agent Prompt) → should_continue 判定 → 输出 “tool”     +     tool API
LLM(Agent Prompt) → should_continue 判定 → 输出 “tool”     +     tool API
					           ……
LLM(Agent Prompt) → should_continue 判定 → 输出 “tool”     +     tool API
LLM(Agent Prompt) → should_continue 判定 → 输出 “generate_structured_response”。
```
如图所示左边第二个红框内的调用，再调用一次单独的 llm，这个 llm 的 prompt 和之前的是不一样的，
```Java
LLM(final_prompt)
```

实现方式
```python
class Schema(TypedDict):
	overall_summary: str                      

final_prompt = "30 个字内总结一下 agent 调用的最终结果，并给出下一步的建议"
agent_executor = create_react_agent(
	react_agent_llm,
	react_agent_tools,
	prompt=agent_prompt, 
	#   1. State里 必须新增 structured_response 
	state_schema=State,
	#  2， response_format 支持传一个二元组：(最终这一步要用的 final prompt, schema)
	#         final prompt 与 agent prompt 不同，没有 tools 定义，只是用来做最后总结
	#         schema 是用来约束最终这次 llm 调用的返回格式的
	response_format=(final_prompt, Schema), # 仅用于最后一次结构化调用
	version="v2",
	name="trip_react"
)
```


为什么需要这次“收尾 LLM”
  - 典型 ReAct要让模型把工具结果融入上下文，组织成对用户可读的回答；如果直接在 tools 后结束，用户拿到的只是 ToolMessage/原始数据。
  - 实现上，结束条件是“最后一条 AIMessage 无 tool_calls”。因此需要有一次 agent 调用产生“无 tool_calls 的消息”，才能判定 END 或转去 structured
  response。



### 1.3 情况三（n  次 LLM）

想跳过情况一中的“收尾 LLM”，让工具直接返回给用户

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F10%2F25%2F02-46-52-09f67334a24fb13f4152c923431b528d-202510250246557-8ea214.png)

相比情况一，少了 `call_model(LLM) → should_continue 判定 → 输出 “end”。`的一次 LLM 调用
```Java
call_model(LLM) → should_continue 判定 → 输出 “tool”     +     tool API
call_model(LLM) → should_continue 判定 → 输出 “tool”     +     tool API
call_model(LLM) → should_continue 判定 → 输出 “tool”     +     tool API （直接结束）
```

#### 如何实现

tool 的最后加一行 `hotel_list.return_direct = True`
```Java
def create_trip_hotel_list_tool(state: State):
    """创建酒店列表查询工具的工厂函数，用于捕获当前 state"""
    # 读取可配置的简短描述
    tool_description = common_config.get_config_instance().get_str(
        "trip_react_hotel_list_tool_description",
        "查询指定城市、日期的酒店列表，支持按价格、评分、距离等排序。返回包含酒店信息的列表"
    )

    # 读取可配置的参数描述
    param_description_by_arr_city = common_config.get_config_instance().get_str(
        "trip_react_hotel_list_param_description_by_arr_city",
        "目标城市名称，例如：'北京'、'上海'"
    )
	…………………………

    @tool(description=tool_description)
    def hotel_list(
            arr_city: Annotated[str, param_description_by_arr_city],
            from_date: Annotated[str, param_description_by_from_date] = "",
            to_date: Annotated[str, param_description_by_to_date] = "",
            constraints: Annotated[str, param_description_by_constraints] = "",
            sort_type: Annotated[str, param_description_by_sort_type] = ""
    ) -> dict:
        try:
            # 从 state 中获取必要参数
            c = state.get("c") or {}

            # 处理日期默认值
            if not from_date:
                from_date = datetime.now().strftime("%Y-%m-%d")
            if not to_date:
                # 解析入住日期并加1天作为离店日期
                from_date_obj = datetime.strptime(from_date, "%Y-%m-%d")
                to_date_obj = from_date_obj + timedelta(days=1)
                to_date = to_date_obj.strftime("%Y-%m-%d")

            logger.info(
                f"trip_consult_sector_hotel_list - 酒店列表查询调用，城市: {arr_city}, "
                f"入住日期: {from_date}, 离店日期: {to_date}, 筛选条件: {constraints}, 排序方式: {sort_type}"
            )


            # 调用酒店列表接口
            result = http_request_service.get_hotel_list(hotel_list_req)

            QMonitor.record_one("hotel_list_tool_success")
            return {
                "success": True,
                "message": f"成功获取到酒店列表",
                "data": result
            }

        except Exception as e:
            logger.error(f"trip_consult_sector_hotel_list - 酒店列表查询失败: {str(e)}", exc_info=True)
            return {
                "success": False,
                "message": f"酒店查询服务暂时不可用: {str(e)}",
                "data": []
            }
            
    hotel_list.return_direct = True
    
    return hotel_list

```

create_react_agent 会在构图时收集所有 return_direct=True 的工具，工具一执行完就立刻END，不再回到 LLM 收尾。



## 其他

`remaining_steps` 是一次执行期间的“剩余图步骤数”（受 LangGraph 运行时管理），所有节点执行都会消耗步数
  - 对应关系：
  
标准 ReAct（情况一）：
      - 一轮“思考+工具”≈消耗 2 步（assistant 1 步 + tools 若干步，v2 下每个 tool_call 各记 1 步；
      - 随后再回到assistant 收尾 1 步）。

情况二：
      - 有 response_format：工具循环结束后再加 1 次仅 LLM 的结构化收尾（+1 步）。

情况三：
      - 有 return_direct=True：命中的那轮工具执行后直接 END，通常省去“收尾 LLM”那 1 步。


  - 代码里的保护逻辑要点（基于你贴的实现）：
      - 若本轮产出 tool_calls 且 remaining_steps < 2，会放弃继续走工具（因为至少还需要“tools + 回到 LLM”两步），直接产出一条
  普通 AIMessage 让路由走 END。
      - 若本轮所有工具都是 return_direct 且 remaining_steps < 1，同理不会再发起工具。
  - 实务结论：
      - remaining_steps 控制的是“还能跑多少节点”的预算；n（有工具的思考轮数）≈取决于预算除以每轮实际消耗（受并行工具数、是否
  直出、是否有最终结构化收尾影响），不是严格等于它。
      - 想要更多循环，就提高执行的步数预算；想更早结束，就降低它或使用 return_direct。



## 实现方式



```Java
# 角色
你是一名专业的交通出行咨询助手，能够根据用户的出行需求智能选择并调用合适的工具，提供最优交通建议与查询结果。

# 任务目标
1. 理解用户需求，识别其关注的交通方式（如飞机、火车、高铁、出行路线、票价趋势等）。
2. 在满足需求的前提下调用最合适的工具（见下方工具说明），返回简明、准确、专业的结果。
3. 回答风格：简洁明了、逻辑清晰、专业可靠，避免冗余解释。

# 工具调用选择逻辑
- 若用户问题仅涉及一种交通方式，则直接调用相应工具。
- 若用户提及多个可能方式（如“去冰岛坐飞机还是火车”），优先调用能到达目的地的方式（如国际航线 → flight_search）。
- 若用户仅问票价走势/趋势，则调用 flight_price_trend。
- 若用户仅提供模糊地点（如“去北欧”“去海边”），可调用 flight_search 并传入模糊参数（fuzzy_departure / fuzzy_arrival）。
- 若问题非工具类（如政策咨询、儿童票规则），直接由你回答，不调用工具。

# 回答要求
- 若调用工具，请明确工具用途与查询目标。
- 若返回结果来自工具调用，请基于结果生成自然语言总结，不输出原始结构化字段。
- 若工具不可用或参数缺失，请明确提示用户补充必要信息（如“请提供出发城市”）。

# 追问 
最后不在需要调用 tools call 的时候除了要返回用户需要的信息外，还要做进一步的追问
可以是：要不要规划游玩路线，要不要帮忙找酒店，要不要帮忙找合适的交通报价
```
