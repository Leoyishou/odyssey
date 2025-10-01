---
date created: 2025-04-16
date modified: 2025-09-25
uid: d2c59329-438b-4509-afcb-772862a99b8c
---

# Summary

langgraph 的[[异步编排]]是 allof逻辑。

1. 持久化与回放 (Checkpointer & Resume) 通过 Checkpointer 可把执行状态写入文件 / DB，意外中断后继续执行。
2. 条件分支、动态路由 理论介绍  graph.add_conditional_edges(node, fn)` 允许根据 fn 返回的标签动态路由。常用模式：Router Node。
3. 状态管理、共享上下文  state

![CleanShot 2025-04-16 at 23.58.52@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_Ks1xGMBeXd%2F2025%2F04%2F16%2F23-59-17-0dd536cf395767f2b253a5bba5654551-CleanShot%202025-04-16%20at%2023.58.52-2x-fd572c.png)

https://langchain-ai.github.io/langgraph/llms-txt-overview AI 时代的文档



# Cues


# Python版本LangGraph更新历史

| 版本号    | 发布日期       | 主要更新内容                                                                   |
| ------ | ---------- | ------------------------------------------------------------------------ |
| 0.1.0  | 2024年10月   | 第一个稳定版本发布，确立了LangGraph作为独立于主LangChain包的框架，核心设计理念是帮助开发者在代理工作流中增加更好的精确性和控制 |
| 0.3.0  | 2025年2月26日 | 被撤回(yanked)的版本，原因是缺少对langgraph-prebuilt的依赖                               |
| 0.3.1  | 2025年2月27日 | 修复0.3.0中的依赖问题                                                            |
| 0.3.5  | 2025年3月5日  | 功能增强与bug修复                                                               |
| 0.3.10 | 2025年3月14日 | 框架稳定性提升                                                                  |
| 0.3.15 | 2025年3月18日 | 功能增强与性能优化                                                                |
| 0.3.20 | 2025年3月25日 | 框架功能扩展                                                                   |
| 0.3.25 | 2025年4月3日  | 提升GraphInterrupt在子图中的使用                                                  |
| 0.3.26 | 2025年4月8日  | 性能优化与bug修复                                                               |
| 0.3.28 | 2025年4月11日 | 允许传递重试策略列表                                                               |
| 0.3.30 | 2025年4月14日 | 当前最新版本，稳定性与功能更新                                                          |

# LangGraph平台

|部署选项|特点|
|---|---|
|Self-Hosted Lite|免费版(执行节点数量不超过1M)，有限功能版本的LangGraph平台，可在本地或自托管方式运行|
|Cloud SaaS|作为LangSmith的一部分完全托管和托管，自动更新和零维护|
|Bring Your Own Cloud (BYOC)|在您的VPC内部署LangGraph平台，作为服务提供和运行|
|Self-Hosted Enterprise|完全在您自己的基础设施上部署LangGraph|

# 核心概念

| concept |     | 解释                             |
| ------- | --- | ------------------------------ |
| node    |     | [[责任链模式]]中的 processor          |
| state   |     | 类似责任链模式中的 context，一个不断流传的共享记事本 |
| Edge    |     | 定义 node 之间的顺序，在责任链模式中就是一个单向链表  |

下面这张 LangGraph 高频 API Cheat Sheet 总结了在日常搭建 StateGraph/Agent 工作流时最常用、最易混淆的核心接口与工具。所有名称都按「创建 → 执行 → 维护」的典型步骤编排，方便快速查阅。

## LLM接入

```Java
from langchain_openai import ChatOpenAI
from langchain_deepseek import ChatDeepSeek
```

# ToolNode 与 Command

- 流程简单、重点在自动“选工具” → Tool‑Calling 风格即可，代码最少。
- 需要复杂状态机、循环、并行 → 用 `Command / goto` 或 `conditional_edges` 更直观。
- 需要兼顾 → 两者混搭：
    1. 前段用 `ToolNode` 让模型“想干啥就叫啥”；
    2. 工具内部或后段节点，再返回 `Command(...)` 进入严格控制区。


# State

[[MessageState、AgentState]]

## Node

[[node、agent]]

## Edge

| Edge                                       | 硬编码跳转、少分支                                                                          |
| -------------------------------------------- |:----------------------------------------------------------------------------------- |
| 固定顺序跳转（线性工作流）。| `graph_builder.add_edge("tools", "chatbot")`                                         |
| 条件分支路由；`cond_fn` 返回下一节点名或 END。| `graph_builder.add_conditional_edges("chatbot", tools_condition)`                    |
| 预构建条件函数：如消息包含 tool_calls → `"tools"`；否则 END。| `graph.add_conditional_edges("chatbot", tools_condition)`                            |
| Command                                    | prompt+goto 灵活跳转  多智能体、Human‑in‑the‑Loop、异常兜底                                      |
| Command = 跳下一个 node + 更新 state               | return Command(<br>        update={"report": report},<br>        goto="__end__"<br>) |
| 批量修改 State，含 `messages` 或任意自定义键          | {"messages":[ToolMessage(...)], "step":"done"}                                       |
| 跳转下一节点；覆盖 Edge 路由                        | `return Command(goto="__end__")`                                                     |
| 携带人类输入 恢复 `interrupt` 暂停                 | `return Command(resume=human)  `                                                     |

## 执行

[[graph 执行]]

[[memory]]

| 作用 & 典型场景                          | 常见关键参数 / 最小示例                                                                                                                                                                         |                                             |
| ---------------------------------- |:------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |:------------------------------------------ |
| 创建“状态驱动”有向图的入口类；每个节点读写同一份 State。| `graph_builder = StateGraph(State)`                                                                                                                                                   | ✅                                           |
|                                    |                                                                                                                                                                                       | ✅                                           |
| 向图中注册节点（可为函数、LC Runnable、子图_）。| `graph_builder.add_node("chatbot", chatbot_fn)`                                                                                                                                       | ✅                                           |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       | 开放式的chat 似乎不能有这种 interrupt，必须都经过 coordinate |
| 跨图导航：`Command.PARENT` 回到父图         | return Command(<br>        update={"score": 0.92},<br>        goto="reviewer",<br>        graph=Command.PARENT<br>)                                                                   |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
| 构建可执行图；可绑定持久化方案。| `graph = graph_builder.compile(checkpointer=MemorySaver())`                                                                                                                           |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
| Checkpoint 后端；让图在多轮对话或崩溃后“断点续跑”。| `graph = graph.compile(checkpointer=MemorySaver())`                                                                                                                                   |                                             |
| 逐事件/逐值执行；适合 CLI、WebSocket 流式输出。| `for ev in graph.stream({'messages':[...]}, cfg):...`                                                                                                                                 |                                             |
|                                    |                                                                                                                                                                                       |                                             |
|                                    |                                                                                                                                                                                       |                                             |
| 查询最新 / 全历史 checkpoint，支持“时间旅行”与分支。| `snapshot = graph.get_state(cfg)`                                                                                                                                                     |                                             |
| 手动覆写 State （热修复或人工干预）。| `graph.update_state(cfg, {"counter": 0})`                                                                                                                                             |                                             |
| 人工审阅、审批；在节点内部暂停并等待外部输入再 resume。| `python\nhuman = interrupt({'query': '确认？'})\nreturn Command(resume={'data': human})\n`                                                                                               |                                             |
|                                    |                                                                                                                                                                                       |                                             |
| 可视化执行图，便于调试与文档。| `graph.get_graph().draw_mermaid_png()`                                                                                                                                                |                                             |
| 可调式输出                              | 通过 `with_structured_output` + Pydantic Schema 直接拿到布尔/枚举字段，避免解析自然语言导致出错 [Medium](https://medium.com/%40vivekvjnk/the-command-object-in-langgraph-bc29bf57d18f?utm_source=chatgpt.com)。|                                             |

> 💡 使用顺序小贴士
>
> 1. 建图：`StateGraph → add_node/edge → (ToolNode + tools_condition)`
>
> 2. 持久化：`compile(checkpointer)`
>
> 3. 运行：`stream / invoke`
>
> 4. 回溯：`get_state_history` + `checkpoint_id` → “时间旅行”
>
> 5. 人工审核：节点内 `interrupt` → 外部 `Command(resume=...)`
>

把这张表收进常用笔记，就能在写 LangGraph 工作流时随手查询，不再频繁翻文档。祝开发顺利!

> 假设前置条件：读者对 Python 较为熟悉，了解基本的 LLM 概念，对「Agent + 工具调用」或「多步对话流程」等话题有初步了解即可。
>
> 关于环境：如果在 Colab 环境使用，请先 `!pip install langgraph openai`。
>
> 关于模型：本教程以 OpenAI GPT-3.5/4 为例，其他模型或私有 API 的使用方式类似，只需替换相关配置。
>
> 教学目标：熟悉 LangGraph 的核心理念和主流用法，能在 1~2 天内快速上手开发多步对话、Agent 工具链和可视化流程图等功能。

```mermaid
flowchart TB
    %% 定义节点
    A((Start)):::start
    P[planner_node]
    S[supervisor_node]
    C[coordinator_node]
    R[reporter_node]
    E((End)):::end

    %% 样式
    classDef start fill:#cfc,stroke:#090,stroke-width:2px
    classDef end fill:#fcc,stroke:#900,stroke-width:2px

    %% 链接关系
    A --> P
    P -->|正常执行| S
    P -->|JSON解析失败等异常| E

    S -->|下一步为 planner| P
    S -->|下一步为 coordinator| C
    S -->|下一步为 reporter| R
    S -->|FINISH| E

    C -->|若包含 "handoff_to_planner"| P
    C -->|否则| E

    R --> S

```

---

  

# LangGraph 基础教程

  

## 目录总览

  

1. [第1步：安装与快速体验](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step1)
    
2. [第2步：LangGraph 核心概念 —— 节点 (Node) 与边 (Edge)](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step2)
    
3. [第3步：构建基本流程图（Flow）](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step3)
    
4. [第4步：将 LangGraph 与 LLM 结合](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step4)
    
5. [第5步：Agent 与工具 (Tools) 的使用](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step5)
    
6. [第6步：多 Agent 协作与高级场景](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step6)
    
7. [第7步：调试与可视化](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step7)
    
8. [第8步：示例项目与简单评估](https://chatgpt.com/c/67ff2786-d0fc-8010-b8e7-ed22cc22e2e2#step8)
    

让我们开始吧！

  

---

  

## 第1步：安装与快速体验

  

### 理论介绍

[LangGraph](https://github.com/langgraph-ai/langgraph)（示例链接）是一个基于图（Graph）概念来组织语言模型对话和多步骤推理流程的框架。它允许你将对话步骤抽象为节点，并通过边来定义节点间的数据流；同时内置了对大语言模型和工具函数的调用支持，使得复杂的多步对话、Agent 推断、工具使用都能通过一个可视化或可编排的图形结构来实现。

  

LangGraph 的典型用法包括：

1. 多步对话流程：将对话的状态和逻辑拆分成若干节点，动态执行；
    
2. Agent + 工具调用：在节点中调用外部 API、检索数据库、执行 Python 函数等；
    
3. 可视化：以图的方式查看流程走向、步骤间数据传递；
    
4. 易于调试和维护：和传统的函数式或流水线式组织相比，图结构更具灵活性。
    

  

本练习会先让你安装 LangGraph，并快速体验一个最小示例：让一个节点调用 OpenAI 模型回答简单问题。

  

### 编程任务

  

1. 安装 LangGraph（如未安装）：`pip install langgraph openai`
    
2. 导入并初始化：填写自己的 OpenAI API Key；
    
3. 创建一个简单的单节点图：该节点调用 GPT-3.5/4 返回答案；
    
4. 执行图并查看输出结果。
    

  

示例代码（假设在本地或 Colab 中运行）：

  

```Python
# 第1步：检查安装并导入
# 若在本地： pip install langgraph openai
import os
import openai

openai.api_key = "YOUR_OPENAI_API_KEY"  # TODO: 替换为你的OpenAI Key

from langgraph import Graph, LLMNode

# 创建一个空图
graph = Graph(name="MyFirstLangGraph")

# 创建一个调用 OpenAI LLM 的节点
llm_node = LLMNode(
    name="simple_gpt_node",
    prompt_template="你是谁？",  # 一个非常简单的 Prompt
    openai_model_name="gpt-3.5-turbo",
    max_tokens=50
)

# 将节点添加到图中
graph.add_node(llm_node)

# 运行图
outputs = graph.run()
print("Graph outputs:", outputs)
```

  

### 验证

  

如果一切顺利，你应该在控制台或 Notebook 中看到类似：

```Java
Graph outputs: {
  "simple_gpt_node": "我是OpenAI的语言模型……"  # 由 GPT 返回的文本
}
```

  

可以用一个简易断言来判断是否收到了回答：

  

```Python
answer_str = outputs.get("simple_gpt_node", "")
assert len(answer_str) > 0, "未能收到 LLM 的回答，请检查 API Key 和网络连接。"
print("第1步通过：LangGraph 安装并进行最小示例调用成功。")
```

  

当输出“第1步通过”时，说明你已成功安装并能使用 LangGraph 创建图、执行节点并获取 LLM 响应。

  

---

  

## 第2步：LangGraph 核心概念——节点 (Node) 与边 (Edge)

  

### 理论介绍

在 LangGraph 中，一切都围绕图 (Graph) 来构建。

  

- Node（节点）：对应一个可执行的步骤或算子，比如「LLM 调用」、「Python 函数执行」、「Tool 调用」等；
    
- Edge（边）：表示节点之间的数据流，通常包含「源节点输出 -> 目标节点输入」的映射关系；
    
- Graph（图）：由若干节点与边组成，定义了数据在流程中的流向。执行图时，LangGraph 会根据数据依赖关系来依次或并行执行节点，并将各节点输出汇总到最后结果。
    

  

一个最常见的情形是：某些节点接收用户输入，处理后输出结果给下游节点，继续处理或调用工具，直到得到一个最后的回答。

  

### 编程任务

  

1. 创建一个简单示例，包含两个节点：
    
    1. 第一个节点将输入文本转换成大写；
        
    2. 第二个节点调用 GPT 进一步做改写。
        
2. 使用 add_edge() 将这两个节点连接起来，使第一个节点的输出作为第二个节点的输入。
    
3. 运行并查看结果。
    

  

示例代码：

```Python
from langgraph import Graph, PythonNode, LLMNode

# 1) 定义图
graph = Graph("TwoNodesDemo")

# 2) 定义节点
uppercase_node = PythonNode(
    name="uppercase_node",
    python_callable=lambda text: text.upper()  # 简单把文本转大写
)

llm_node = LLMNode(
    name="llm_node",
    prompt_template="对以下文本进行简短润色：{{input_text}}",
    openai_model_name="gpt-3.5-turbo",
    max_tokens=50
)

# 3) 添加节点到图
graph.add_node(uppercase_node)
graph.add_node(llm_node)

# 4) 用边将第一个节点的输出映射到第二个节点的输入
graph.add_edge(
    source_node=uppercase_node, 
    target_node=llm_node, 
    mapping={"return_value": "input_text"}
)

# 5) 设置图的输入，并执行
inputs = {"uppercase_node": {"text": "hello langgraph"}}
outputs = graph.run(inputs)
print("Graph outputs:", outputs)
```

  

### 验证

  

- 执行后，`uppercase_node` 会先将 "hello langgraph" 转成 "HELLO LANGGRAPH"，再传给 `llm_node` 的 `input_text`。
    
- `llm_node` 根据提示模板对这段大写文本进行润色或改写，最终输出到 graph 的结果。
    
- 你应该看到一段 GPT 生成的文本。
    

  

可以做一个简单检查，确认最初的输入是 "hello langgraph"，最终 LLM 节点处理的是大写文本：

  

```Python
assert "HELLO LANGGRAPH" in outputs.get("llm_node", ""), "LLM 输出不包含大写文本，可能未正确连接。"
print("第2步通过：节点与边的基本用法已掌握。")
```

  

当看到“第2步通过”并输出合乎逻辑的文本，表示你已经理解了 LangGraph 中的 Node/Edge 概念及最简单的数据流设置。

  

---

  

## 第3步：构建基本流程图（Flow）

  

### 理论介绍

  

LangGraph 的核心是一张可执行的流程图 (Flow / Graph)。通过编程或可视化界面（如果 LangGraph 提供 GUI）来组合节点、配置输入/输出映射、控制执行顺序。当我们执行 `graph.run()` 时，LangGraph 会自动按照数据依赖顺序执行各个节点。

  

在实际场景中，一个流程图可能包含：

- 多个 PythonNode 做文本预处理；
    
- 一个或多个 LLMNode 做语言模型调用；
    
- ToolNode (假设 LangGraph 提供) 调用外部搜索 / 数据库 / 文件读写；
    
- 以及多路分支、条件判断等。
    

  

### 编程任务

  

1. 创建一个更复杂的流程图，比如包含三到四个节点：
    
    1. Node A：对输入文本做分句处理；
        
    2. Node B：对每个句子调用 GPT 进行翻译；
        
    3. Node C：汇总翻译结果；
        
    4. （可选）Node D：再次调用 GPT 对汇总做润色。
        
    
2. 连接这些节点形成一个多步管线。
    
3. 输入一些文本并执行图，查看最终结果。
    

  

简化示例（仅供参考）：

  

```Python
import re
from langgraph import Graph, PythonNode, LLMNode

def split_sentences(text):
    return re.split(r'[。！？]', text)

def join_sentences(sentences):
    return " / ".join(sentences)

graph = Graph("TranslationFlow")

# Node A: 分句处理
split_node = PythonNode(
    name="split_node",
    python_callable=split_sentences
)

# Node B: 对每个句子调用 GPT 进行翻译（可简化为拼接后统一翻译）
translate_node = LLMNode(
    name="translate_node",
    prompt_template="请把下面的文本翻译成英文：{{chinese_text}}",
    openai_model_name="gpt-3.5-turbo",
    max_tokens=100
)

# Node C: 汇总翻译结果
join_node = PythonNode(
    name="join_node",
    python_callable=join_sentences
)

# 将节点加到图中
graph.add_node(split_node)
graph.add_node(translate_node)
graph.add_node(join_node)

# 边1: split_node -> translate_node
graph.add_edge(
    source_node=split_node,
    target_node=translate_node,
    mapping={"return_value": "chinese_text"}
)

# 边2: translate_node -> join_node
graph.add_edge(
    source_node=translate_node,
    target_node=join_node,
    mapping={"return_value": "sentences"}  # 这里假设 GPT 输出若干句子，可以进一步解析
)

# 输入示例
inputs = {
    "split_node": {"text": "你好！这是一次 LangGraph 测试。现在开始吧。"}
}

outputs = graph.run(inputs)
print("Graph outputs:", outputs)
```

  

### 验证

  

如果流程正确：

- 第一个节点输出的应该是一个句子列表，如 `["你好", " 这是一次 LangGraph 测试", " 现在开始吧", ""]`；
    
- 第二个节点应将这些句子拼接后翻译成英文；
    
- 第三个节点再将翻译结果整理成字符串或进行合并处理。
    

  

你应该能看到最终的输出是一个汇总的英文翻译结果。根据具体 Prompt 模板及 GPT 的回复，结果可能略有不同。

  

可以简单断言输出中包含一些英文单词：

  

```Python
final_output = outputs.get("join_node", "")
assert any(word in final_output.lower() for word in ["langgraph", "hello", "test"]), "翻译结果似乎不包含期望的英文内容。"
print("第3步通过：成功构建并运行一个基础多节点流程图。")
```

  

当输出“第3步通过”时，表示你已经能够在 LangGraph 中构建多个节点并将其组织成数据流执行。

  

---

  

## 第4步：将 LangGraph 与 LLM 结合

  

### 理论介绍

  

在上一步我们其实已经用到 LLMNode，但这一节更系统地介绍如何管理 LLM 配置。LangGraph 通常提供：

- LLMNode：通用 GPT 调用节点；
    
- 系统级参数：如 OpenAI key、model 名称、max_tokens、temperature；
    
- Prompt 模板：可使用 Jinja2 风格的 `{{var}}` 占位符拼接上下文。
    

  

你还可以自定义PromptNode或LLMNode来对 Prompt 进行更多细化控制，比如：system prompt、user prompt、few-shot example 等。

  

### 编程任务

  

1. 创建一个自定义的 LLMNode，在其中指定 system prompt、user prompt，或 few-shot example；
    
2. 将其整合进一个小的 graph；
    
3. 调试 temperature、max_tokens 等参数，观察回答风格变化。
    

  

示例：

  

```Python
from langgraph import Graph, LLMNode

graph = Graph("CustomLLMConfig")

custom_llm_node = LLMNode(
    name="custom_llm_node",
    openai_model_name="gpt-3.5-turbo",
    max_tokens=100,
    temperature=0.8,
    # 下面这段 prompt_template 可以包含更多上下文
    prompt_template="""以下是一些示例对话：
1) 问：今天天气怎么样？ 答：晴转多云。
2) 问：你好！你是谁？ 答：我是一个语言模型助手。

现在请回答我的问题：
{{user_input}}
""",
)

graph.add_node(custom_llm_node)

# 为了让外部输入直接进 user_input
inputs = {
    "custom_llm_node": {
        "user_input": "请介绍一下你自己，并说明LangGraph的特点。"
    }
}
outputs = graph.run(inputs)
print("Graph outputs:", outputs)
```

  

### 验证

  

如果执行正常，你应该看到回答中既有自我介绍，也提到了 LangGraph 的一些特点（如果上下文或 ChatGPT 不知道，会编造一些合理的解释）。

可以做个简单断言，保证回答非空：

  

```Python
assert len(outputs.get("custom_llm_node", "")) > 0, "LLM 回答为空，请检查 Prompt 或 API Key。"
print("第4步通过：灵活定制 LLM 节点与 Prompt 模板已完成。")
```

  

当输出“第4步通过”并能看到带有示例格式或多句回答，说明你已经掌握了如何在 LangGraph 中自定义 LLM 调用方式与参数。

  

---

  

## 第5步：Agent 与工具 (Tools) 的使用

  

### 理论介绍

  

很多基于 LangChain、LangGraph 等框架的核心是Agent概念：让语言模型在对话中能“思考”，并在需要时调用外部工具 (Tool) 来完成特定任务（例如搜索、数据库查询、计算等）。LangGraph 也可能提供AgentNode或者ToolNode来支持：

  

1. AgentNode：在对话过程中动态决定是否调用工具；
    
2. Tool：可以是 Python 函数、API、数据库查询等；
    
3. 工具调用过程：语言模型会根据上下文和提示判断何时调用工具，如何使用其输出，再给出最终回答。
    

  

本节示例：我们构建一个 Agent 节点，配备一个或多个 Tools，让用户提问时 Agent 决定是否调用 Tool 来获取答案。

  

### 编程任务

  

1. 定义一个简单的 Tool：例如做四则运算，或者获取当前时间；
    
2. 定义一个 AgentNode，将该 Tool 注册进去；
    
3. 构建一个 Graph，让用户输入一个问题（比如要求计算某些数值），看看 Agent 是否会自动调用 Tool 并返回正确结果。
    

  

示例（示意）：

  

```Python
from langgraph import Graph, AgentNode, Tool

# 定义一个简单的Tool: 进行加法运算
def add_numbers(x, y):
    return x + y

add_tool = Tool(
    name="add_tool",
    description="进行加法运算, 参数: x, y",
    func=add_numbers
)

# 定义AgentNode: 允许调用 add_tool
agent_node = AgentNode(
    name="math_agent",
    tools=[add_tool],
    # 一个简易 prompt，引导Agent何时调用工具
    system_prompt="你是一个擅长数学计算的助手，必要时可调用工具来计算。",
    openai_model_name="gpt-3.5-turbo"
)

graph = Graph("AgentToolsDemo")
graph.add_node(agent_node)

# 输入一个需要计算的问题
inputs = {
    "math_agent": {
        "user_input": "请帮我计算 12345 加上 67890 等于多少？"
    }
}
outputs = graph.run(inputs)
print("Graph outputs:", outputs)
```

  

### 验证

  

若 Agent 逻辑正常，它会“思考”要进行加法运算，然后调用 add_tool，再将结果整合到回答中。输出格式视具体实现而定，可能类似：

  

```Java
"math_agent": "我调用了 add_tool(12345, 67890) 得到结果 80235。答案是 80235。"
```

  

可对回答做一个简单的字符串判断，也可解析回答是否包含正确的数字：

  

```Python
pred = outputs.get("math_agent", "")
assert "80235" in pred, "Agent 工具调用结果不正确，可能工具未被正确调用。"
print("第5步通过：Agent + 工具的简单演示完成。")
```

  

当输出“第5步通过”且返回包含正确的加法结果，即说明你已经了解了 LangGraph 中 Agent/Tool 的基本用法。

  

---

  

## 第6步：多 Agent 协作与高级场景

  

### 理论介绍

  

多 Agent 协作指在同一个对话或流程中，存在多个 Agent 节点，各自有不同的工具或专长，彼此通过图的边进行信息交互，协同完成更复杂的任务。例如：

  

- Agent A 专精数学计算工具；
    
- Agent B 专精外部 API 请求或数据库查询；
    
- 通过图结构，将 Agent A 的输出传给 Agent B；
    
- 或者并行地让两个 Agent 去处理不同子问题，最后整合答案。
    

  

LangGraph 的图式结构让此类场景更易实现：每个 AgentNode 都是一个节点，边则定义了它们如何交换信息。

  

### 编程任务

  

1. 定义两个 AgentNode，各自包含不同的 Tools
    
2. 在 Graph 中将它们串联或者并行，然后再由一个 PythonNode 汇总它们的结果；
    
3. 模拟一个问题，观察多 Agent 之间如何协作。
    

  

示例（思路演示）：

  

```Python
from langgraph import Graph, AgentNode, Tool, PythonNode

# Tool 1: 数学加法
def add_numbers(x, y):
    return x + y

math_tool = Tool(name="add_tool", func=add_numbers, description="数学加法")

# Tool 2: 一个简单翻译函数 (纯 Python 演示)
def simple_translate(text):
    return "Translated:" + text  # 伪翻译

translate_tool = Tool(name="translate_tool", func=simple_translate, description="翻译文本")

# Agent A: 擅长数学
agent_a = AgentNode(
    name="agent_a",
    tools=[math_tool],
    system_prompt="你擅长数学计算，有 add_tool 可使用。",
    openai_model_name="gpt-3.5-turbo"
)

# Agent B: 擅长翻译
agent_b = AgentNode(
    name="agent_b",
    tools=[translate_tool],
    system_prompt="你擅长翻译文本，有 translate_tool 可使用。",
    openai_model_name="gpt-3.5-turbo"
)

# PythonNode: 汇总
def combine_results(math_str, translate_str):
    return f"MathResult: {math_str}\nTranslateResult: {translate_str}"

summary_node = PythonNode(name="summary_node", python_callable=combine_results)

# 构建图
graph = Graph("MultiAgentDemo")
graph.add_node(agent_a)
graph.add_node(agent_b)
graph.add_node(summary_node)

# 让Agent A做个数学计算后，Agent B 也翻译一句话
# 这里我们通过 graph inputs 分别给 agent_a, agent_b
inputs = {
    "agent_a": {"user_input": "请计算 99 + 1。"},
    "agent_b": {"user_input": "翻译：你好世界"}
}

# 接下来，需要将两个Agent的输出都输入给 summary_node
graph.add_edge(
    source_node=agent_a,
    target_node=summary_node,
    mapping={"return_value": "math_str"}
)
graph.add_edge(
    source_node=agent_b,
    target_node=summary_node,
    mapping={"return_value": "translate_str"}
)

# 执行
outputs = graph.run(inputs)
print("Final Outputs:", outputs)
```

  

### 验证

  

- 运行后，`agent_a` 返回加法结果（如 "100"），`agent_b` 返回翻译结果（如 "Translated:你好世界"），`summary_node` 将这两者合并；
    
- 最终输出会在 `summary_node` 中看到一个带有两段信息的字符串。
    

  

可以断言一下：

  

```Python
summary_text = outputs.get("summary_node", "")
assert "MathResult" in summary_text and "TranslateResult" in summary_text, "多Agent结果汇总不完整"
print("第6步通过：多Agent协作与高级场景示例完成。")
```

  

当输出“第6步通过”并打印出合并后的信息，说明你已经了解了如何在图中创建多个 AgentNode 并让它们协作解决任务。

  

---

  

## 第7步：调试与可视化

  

### 理论介绍

LangGraph 的图结构为调试提供了优势：你可以查看每个节点的输入、输出、执行日志；如果它支持 GUI 或者导出到某些可视化工具（如 Graphviz 等），就能在图形界面直观地查看节点连接。

  

调试方法：

1. 日志：LangGraph 可能提供 debug 模式或返回详细的 node execution log；
    
2. 可视化：将图导出为.dot 文件或类似格式，用 graphviz、mermaid 或浏览器查看；
    
3. 中间结果检查：在执行后查看每个节点的 outputs，排查出错节点。
    

  

### 编程任务

  

1. 将之前的图启用 debug 级别日志（若 LangGraph 提供类似 `graph.run(debug=True)` 或 logger 设定）；
    
2. 打印每个节点的输入、输出；
    
3. 尝试导出图为可视化文件 (如 `.dot`) 并查看。
    

  

示例（视 LangGraph 具体API 而定，这里仅示意）：

  

```Python
# 假设在前面定义的 graph 基础上
outputs = graph.run(inputs, debug=True)

# 假设 LangGraph 内部会输出节点执行日志
# 或者可以获取 graph.execution_trace

print("Execution Trace:")
for step in graph.execution_trace:
    print(step)

# 导出可视化
graph.save_dot("my_langgraph.dot")
print("已导出到 my_langgraph.dot，可用 graphviz 查看。")
```

  

### 验证

  

- 检查在控制台里能看到每个节点的输入与输出；
    
- 查看 `.dot` 文件是否生成；
    
- 若有合适环境（graphviz 等），可以用 `dot -Tpng my_langgraph.dot -o out.png` 转成图片，看流程图是否符合预期。
    

  

若能成功导出并查看，就说明已掌握基本的调试和可视化方法：

  

```Python
import os
assert os.path.exists("my_langgraph.dot"), "可视化文件未生成，检查 save_dot 调用。"
print("第7步通过：调试与可视化的基础操作成功。")
```

  

---

  

## 第8步：示例项目与简单评估

  

### 理论介绍

  

到目前为止，你已经学会了：

  

1. 创建 Graph 并添加各类节点（PythonNode、LLMNode、AgentNode 等）；
    
2. 使用 Edge 在节点之间传递数据；
    
3. 配置 LLM、Agent、Tool 等核心功能。
    
4. 进行基本调试和可视化。
    

  

在实际项目中，你往往需要对 Agent 或流程进行评估：

  

- 功能性评估：看是否能正确调用工具、正确回答问题；
    
- 质量评估：与基准答案对比或计算相似度、拼写检查等；
    
- 人工审阅：对于生成式任务，还可人工标注、打分。
    

  

本节通过一个小“知识问答”项目示例，让你把整个流程串起来，并添加一个简单的自动化评估。

  

### 编程任务

  

1. 设计一个包含 AgentNode + 工具（可结合自定义检索函数、数据库查询或直接硬编码一个知识库）的流程；
    
2. 让用户提出问题后，Agent 调用该工具查询答案，再返回给用户；
    
3. 准备一些测试问答对进行自动化脚本测试，计算简单准确率；
    
4. 打印最终评估分数。
    

  

示例：

  

```Python
from langgraph import Graph, AgentNode, Tool

# 假设我们有个简单知识库
knowledge_db = {
    "LangGraph": "LangGraph 是一个用图结构来组织LLM逻辑的框架。",
    "Python": "Python是一种高层次解释型语言。",
}

def retrieve_answer(keyword):
    return knowledge_db.get(keyword, "未找到相关信息")

# 定义Tool
db_tool = Tool(
    name="db_retrieval",
    description="根据关键词检索简单知识库",
    func=retrieve_answer
)

# 定义AgentNode
agent_node = AgentNode(
    name="qa_agent",
    tools=[db_tool],
    system_prompt="你可以使用 [db_retrieval] 工具，根据关键词从知识库获取信息，然后回答用户问题。",
    openai_model_name="gpt-3.5-turbo"
)

graph = Graph("KnowledgeQADemo")
graph.add_node(agent_node)

# 测试问答对
test_data = [
    ("什么是LangGraph？", "LangGraph 是一个用图结构来组织LLM逻辑的框架。"),
    ("Python是什么？", "Python是一种高层次解释型语言。"),
]

def simple_eval(pred, gold):
    # 简单规则：如果 gold 子串出现在 pred 中，就给1分
    return 1.0 if gold in pred else 0.0

scores = []
for question, gold in test_data:
    outputs = graph.run({"qa_agent": {"user_input": question}})
    pred_ans = outputs.get("qa_agent", "")
    score = simple_eval(pred_ans, gold)
    scores.append(score)
    print(f"Q: {question}\nPred: {pred_ans}\nGold: {gold}\nScore: {score}\n---")

avg_score = sum(scores) / len(scores)
print("平均得分:", avg_score)
```

  

### 验证

  

若 Agent 能正常调用 `db_retrieval` 工具，并在回答中包含正确的知识点，你会看到较高的得分（例如 1.0 或 0.5 之类）。若 Agent 回答出现偏差，也能在这里反映出来。

可以加个断言，如果平均得分 < 0.5，可能说明工具调用或提示不够明确：

  

```Python
assert avg_score >= 0.5, "平均得分过低，Agent 未正确使用知识库或回答不准确。"
print("第8步通过：完整项目示例与简单评估流程完成。")
```

  

当输出“第8步通过”并得到合理的得分，你便完成了一个较完整的 LangGraph 基础练习。

  

---

  

# 总结

  

恭喜你！通过上述 8 个步骤的示例与练习，你已经从零起步，掌握了 LangGraph 的核心能力：

  

1. 安装与快速体验：创建最小图、调用 LLM；
    
2. Node 与 Edge：理解图中节点和边的本质；
    
3. 构建流程图：多节点串联或并行；
    
4. 整合 LLM：自定义 Prompt、调参；
    
5. Agent + Tools：让模型在多步对话中调用外部函数；
    
6. 多 Agent 协作：在一个图里并行或串联多个 Agent；
    
7. 调试与可视化：查看节点输入输出、导出.dot 文件；
    
8. 示例项目与评估：将知识库问答串联起来，做简单自动化测试。
    

  

通过这些学习，你能在 LangGraph 中灵活地组织对话逻辑、工具调用和高级推理流程，也能进行一定程度的调试与自动化测试。后续你可以探索更复杂的功能，包括条件分支、自定义执行调度、大型知识库检索、UI 集成等，甚至搭建真实的多轮对话应用。

  

祝你在多 Agent 和图式对话编排的道路上越走越远，玩转 LangGraph！
