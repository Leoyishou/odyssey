---
date created: 2024-11-25
date modified: 2025-10-31
uid: 5537f688-37bd-4195-8235-31e549ab6dbe
---

```http
curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gpt-4-turbo-preview",
    "messages": [
      {
        "role": "system",
        "content": "**参数提取指南**:\n- departure: 从用户消息中提取，或使用默认值\"上海\"\n- arrival: 必须从用户消息中提取目的地（这是核心参数）\n- days: 提取天数，未提及则默认3天\n- start_date: 提取出发日期，未提及则使用明天的日期\n- select_tags/trip_style: 提取用户偏好（如\"亲子游\"、\"文化深度游\"）\n\n**示例触发语句**:\n- \"我想去北京玩三天\"\n- \"下周去杭州，帮我安排行程\"\n- \"国庆想带孩子去成都\"\n\n### 2. adjust_trip_route - 调整现有行程\n**触发条件**（满足任一即调用）:\n- 对话中已经有生成的行程，用户要求修改\n- 用户说\"把XX景点换成YYY\"、\"帮我第二天的XX\"\n- 用户说\"增加/添加XX景点\"\n- 用户说\"行程太紧张，帮我放松点\"\n- 用户说\"行程可以帮把到别天吗？\"\n- 用户说\"行程可以帮加到5天吗？\"\n\n### 3. import_trip_route_note - 导入已有攻略\n**触发条件**（满足任一即调用）:\n- 用户直接粘贴了一段完整的旅游攻略/行程\n- 用户说\"帮我整理一下这个行程\"并附上文字\n- 用户分享了小红书/其他平台的行程内容\n- 内容包含明确的目地、景点、时间安排\n\n### 4. 直接回答（无需调用工具）\n仅在以下情况直接回答:\n- 旅游常识性问题（如\"XX景点门票多少钱\"）\n- 目的地咨询（如\"推荐一个适合亲子游的城市\"）\n- 旅行建议（如\"去云南什么季节最好\"）\n- 用户只是询问，没有明确的行程规划需求\n\n## 重要提醒:\n1. **当用户明确表达了去某地旅游的意图时，必须立即调用ai_generate_trip_route，不要只是归头回答**\n2. 如果信息不足（如没说目的地），先简短询问缺失的关键信息，获得后立即调用工具\n3. 参数优先从用户消息中提取，缺失的使用合理默认值，不要反复追问\n4. 保持回复简洁友好，重点是快速帮用户生成可用的行程"
      },
      {
        "role": "user",
        "content": "南京"
      },
      {
        "role": "assistant",
        "content": "AI开启行程"
      },
      {
        "role": "user",
        "content": "西班牙"
      },
      {
        "role": "assistant",
        "content": "AI开启行程"
      },
      {
        "role": "user",
        "content": "行程可以帮加到3天吗"
      }
    ],
    "temperature": 1,
    "max_tokens": 4096
  }'



{
  "messages": [
    {"role": "system", "content": "..."},
    {"role": "user", "content": "南京"},           // 历史消息 1
    {"role": "assistant", "content": "AI开启行程"},  // 历史消息 2
    {"role": "user", "content": "西班牙"},          // 历史消息 3
    {"role": "assistant", "content": "AI开启行程"},  // 历史消息 4
    {"role": "user", "content": "行程可以帮加到3天吗"} // ← 这是最新的
  ]
}
```

# Request body

|                   | 参数                      | 类型                                 | 可选性/默认值                          | 描述                                                                                                 |
| :---------------- | :---------------------- | :--------------------------------- | :------------------------------- | :------------------------------------------------------------------------------------------------- |
|                   | `model`                 | string                             | **Required**                     | 用于生成响应的模型 ID，如 `gpt-4o` 或 `o3`。                                                                    |
|                   | `messages`              | array                              | **Required**                     | 构成迄今为止对话的消息列表。根据您使用的模型，支持不同的消息类型（模态），如文本、图像和音频。                                                    |
| 上下文限制             | `max_completion_tokens` | integer or null                    | Optional                         | 补全可生成的令牌数量上限，包括可见输出令牌和推理令牌。                                                                        |
|                   | `audio`                 | object or null                     | Optional                         | 音频输出的参数。当请求音频输出时（`modalities:["audio"]`）为必需。                                                       |
| [[Function Call]] | `tool_choice`           | string or object                   | Optional                         | 控制模型调用哪个（或哪些）工具。`none` 表示不调用，`auto` 表示模型自行选择，`required` 表示必须调用。                                    |
|                   | `tools`                 | array                              | Optional                         | 模型可能调用的工具列表。您可以提供自定义工具或函数工具。                                                                       |
| ✅                 | `frequency_penalty`     | number or null                     | Optional<br>Defaults to `0`      | 介于 -2.0 和 2.0 之间的数字。正值会根据新令牌在文本中已有的频率对其进行惩罚，从而降低模型逐字重复同一行的可能性。                                     |
| [[temperature]]   | `temperature`           | number                             | Optional<br>Defaults to `1`      | 使用的采样温度，介于 0 和 2 之间。                                                                               |
| [[top_p]]         | `top_p`                 | number                             | Optional<br>Defaults to `1`      | 一种替代温度采样的核采样方法。                                                                                    |
| ✅                 | `presence_penalty`      | number or null                     | Optional<br>Defaults to `0`      | 介于 -2.0 和 2.0 之间的数字。正值会根据新令牌是否已在文本中出现来对其进行惩罚，从而增加模型谈论新话题的可能性。                                      |
|                   | `logit_bias`            | map                                | Optional<br>Defaults to `null`   | 修改指定令牌在补全中出现的可能性。接受一个 JSON 对象，将令牌（由其在分词器中的令牌 ID 指定）映射到 -100 到 100 的关联偏置值。                          |
|                   | `logprobs`              | boolean or null                    | Optional<br>Defaults to `false`  | 是否返回输出令牌的对数概率。如果为 `true`，则返回 `message` 内容中每个输出令牌的对数概率。                                             |
|                   | `metadata`              | map                                | Optional                         | 可附加到对象的 16 个键值对集合。                                                                                 |
|                   | `modalities`            | array                              | Optional                         | 您希望模型生成的输出类型。默认为 `["text"]`。`gpt-4o-audio-preview` 模型可使用 `["text", "audio"]`。                      |
|                   | `n`                     | integer or null                    | Optional<br>Defaults to `1`      | 为每个输入消息生成多少个聊天补全选项。                                                                                |
|                   | `parallel_tool_calls`   | boolean                            | Optional<br>Defaults to `true`   | 在工具使用期间是否启用并行函数调用。                                                                                 |
|                   | `prediction`            | object                             | Optional                         | 预测输出的配置，当模型响应的大部分内容预先知道时，可以显著改善响应时间。                                                               |
|                   | `prompt_cache_key`      | string                             | Optional                         | OpenAI 用于缓存相似请求的响应以优化缓存命中率。取代 `user` 字段。                                                           |
|                   | `reasoning_effort`      | string                             | Optional<br>Defaults to `medium` | 约束推理模型的推理工作量。支持 `minimal`, `low`, `medium`, `high`。                                                |
|                   | `response_format`       | object                             | Optional                         | 指定模型必须输出的格式的对象。例如，`{ "type": "json_object" }` 或 `{ "type": "json_schema", "json_schema": {...} }`。 |
|                   | `safety_identifier`     | string                             | Optional                         | 一个稳定的标识符，用于帮助检测可能违反 OpenAI 使用策略的应用程序用户。                                                            |
|                   | `service_tier`          | string                             | Optional<br>Defaults to `auto`   | 指定用于处理请求的服务层级。可以是 `auto`, `default`, `flex`, 或 `priority`。                                         |
|                   | `stop`                  | string / array / null              | Optional<br>Defaults to `null`   | API 将停止生成更多令牌的最多 4 个序列。最新的推理模型 `o3` 和 `o4-mini` 不支持。                                               |
|                   | `store`                 | boolean or null                    | Optional<br>Defaults to `false`  | 是否存储此聊天补全请求的输出，用于模型蒸馏或评估产品。                                                                        |
|                   | `stream`                | boolean or null                    | Optional<br>Defaults to `false`  | 如果设置为 `true`，模型响应数据将在生成时使用服务器发送事件（server-sent events）流式传输到客户端。                                     |
|                   | `stream_options`        | object                             | Optional<br>Defaults to `null`   | 流式响应的选项。仅在设置 `stream: true` 时设置此项。                                                                 |
|                   | `top_logprobs`          | integer                            | Optional                         | 一个介于 0 和 20 之间的整数，指定在每个令牌位置返回的最可能令牌的数量。                                                            |
|                   | `verbosity`             | string                             | Optional<br>Defaults to `medium` | 约束模型响应的详细程度。支持 `low`, `medium`, `high`。                                                            |
|                   | `web_search_options`    | object                             | Optional                         | 此工具在网络上搜索相关结果以用于响应。                                                                                |
|                   | `function_call`         | **Deprecated**<br>string or object | Optional                         | 已弃用，推荐使用 `tool_choice`。控制模型调用哪个（或是否调用）函数。                                                          |
|                   | `functions`             | **Deprecated**<br>array            | Optional                         | 已弃用，推荐使用 `tools`。模型可能为其生成 JSON 输入的函数列表。                                                            |
|                   | `seed`                  | **Deprecated**<br>integer or null  | Optional                         | **此功能处于 Beta 阶段。** 如果指定，系统将尽力进行确定性采样。                                                              |
|                   | `max_tokens`            | **Deprecated**<br>integer or null  | Optional                         | 聊天补全中可生成的最大令牌数。此值现已弃用，推荐使用 `max_completion_tokens`，且与 o-series 模型不兼容。                              |
|                   | `user`                  | **Deprecated**<br>string           | Optional                         | 此字段正在被 `safety_identifier` 和 `prompt_cache_key` 取代。                                                |

以下是对这些 LLM（大型语言模型）相关参数和选项的解释与介绍：

1. **Model（模型）**：
    指定所使用的语言模型版本或类型。不同的模型在训练数据、参数规模、性能和特点方面可能有所差异。选择不同模型可以影响回答的质量、语气和速度。
    
2. **Token count（令牌计数）**：
    令牌（Token）是模型处理文本的基本单位，一般由词语、子词或字符片段组成。**Token count** 通常指已经输入或产生的文本长度，模型对令牌的数量有一定限制。较大的令牌上限允许模型产生更长的输出文本，但处理时间和费用也会提高。
    
3. **Temperature（温度）**：
    温度是一个用于控制模型回答的"随机性"或"创造性"的参数。
    - 当温度接近 0 时，模型更倾向于选择概率最高的词语，回答会相对保守、确定性较高且重复率可能较高。
    - 当温度较高时，模型回答会更具多样性和创意，可能更有"发散思维"，但同时不确定性也增加，回答可能显得更不稳定。
4. **Tools（工具）/ Structured output（结构化输出）/ Code execution（代码执行）/ Function calling（函数调用）**：
    这些选项主要用于增强 LLM 的功能和输出格式。
    
    - **Structured output**：可用来指定输出的格式或结构（例如 JSON 格式、列表等），方便后续处理。
    - **Code execution（代码执行）**：在有些环境中，模型可能集成了执行特定代码片段的能力，可根据用户指令进行运算或调用外部 API。
    - **Function calling（函数调用）**：允许模型以受控的方式调用已定义的函数，从而实现更好的工具使用能力，比如调用计算器函数、数据库查询函数等，让回答更具实用性和可控性。
5. **Advanced settings（高级设置）/ Safety settings（安全设置）**：
    
    - **Safety settings**：用于控制模型回答中的内容安全，例如过滤敏感信息、不适宜内容。可根据策略调整这些设置以确保回答符合道德规范和平台政策。
6. **Add stop sequence（添加停止序列）**：
    停止序列是一段特殊的文本（如某些标记符号），在生成文本时，如果模型输出该序列，就会停止继续生成。这有助于控制回答的长度或防止无穷延续。
    
7. **Output length（输出长度）**：
    指定模型回答的最大长度（以令牌计）。如果设定此值，模型在达到该限制时将停止生成更多文本。较大的输出长度允许更详细的回答，但可能增加冗余。
    
8. **Top P（核采样）**：
    与 Temperature 类似，Top P（又称 nucleus sampling）控制采样的随机性方式不同。
    
    - 模型会从概率分布中选取一组累计概率达到 P 的词语作为候选，然后从这些候选中随机选择下一个词。当 P 接近 1 时，模型有更多候选可选，因此回答更具发散性。较低的 P 值则让模型只从最有可能的一小部分词中进行选择，回答更加保守和可控。

通过调节上述参数，可以在回答的准确度、创造性、长度、格式、安全性等多个维度对模型的输出进行微调和优化。

# OpenAI

[How Assistants work - OpenAI API](https://platform.openai.com/docs/assistants/how-it-works)

| 主分类            | 子项目                       |                                                                                                                                                                                                                                                                                                                         |
| -------------- | ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Get started    | Overview                  |                                                                                                                                                                                                                                                                                                                         |
|                | Quickstart                |                                                                                                                                                                                                                                                                                                                         |
|                | Models                    |                                                                                                                                                                                                                                                                                                                         |
|                | Changelog                 |                                                                                                                                                                                                                                                                                                                         |
|                | Terms and policies        |                                                                                                                                                                                                                                                                                                                         |
| Capabilities   | Text generation           |                                                                                                                                                                                                                                                                                                                         |
|                | Image generation          |                                                                                                                                                                                                                                                                                                                         |
|                | Vision                    |                                                                                                                                                                                                                                                                                                                         |
|                | Audio generation          |                                                                                                                                                                                                                                                                                                                         |
|                | Text to speech            |                                                                                                                                                                                                                                                                                                                         |
|                | Speech to text            |                                                                                                                                                                                                                                                                                                                         |
|                | Vector embeddings         |                                                                                                                                                                                                                                                                                                                         |
|                | Moderation                |                                                                                                                                                                                                                                                                                                                         |
|                | Reasoning                 |                                                                                                                                                                                                                                                                                                                         |
| Guides         | Function calling          | 让 AI 生成入参调本地函数                                                                                                                                                                                                                                                                                                          |
|                | Structured outputs        | 借助 [Pydantic](Pydantic.md)                                                                                                                                                                                                                                                                                              |
|                | Evaluations               |                                                                                                                                                                                                                                                                                                                         |
|                | Fine-tuning               | 使用您的数据将模型适应您的特定用例                                                                                                                                                                                                                                                                                                       |
|                | Distillation              | 使用生产日志评估和微调模型                                                                                                                                                                                                                                                                                                           |
|                | Realtime API              | 原生语音转语音而不是TTS                                                                                                                                                                                                                                                                                                           |
|                | Batch API                 | 批处理，适合那种一两个小时内不需要结果的任务，放在 openai 那里慢慢做，收费比正常接口低                                                                                                                                                                                                                                                                         |
| Assistants     | Overview                  | 助手可以并行访问**多种工具**。这些可以是 OpenAI 托管的工具——像[代码解释器](https://platform.openai.com/docs/assistants/tools/code-interpreter)和[文件搜索](https://platform.openai.com/docs/assistants/tools/file-search) ——或者您构建/托管的工具(通过[函数调用](https://platform.openai.com/docs/assistants/tools/function-calling))。相当于是 OpenAI 的函数+本地函数，其实就是我之前做的 viva |
|                | Quickstart                |                                                                                                                                                                                                                                                                                                                         |
|                | Deep dive                 |                                                                                                                                                                                                                                                                                                                         |
|                | Tools                     |                                                                                                                                                                                                                                                                                                                         |
|                | What's new?               |                                                                                                                                                                                                                                                                                                                         |
|                | Migration guide           |                                                                                                                                                                                                                                                                                                                         |
| ChatGPT        | Actions                   |                                                                                                                                                                                                                                                                                                                         |
|                | Release notes             |                                                                                                                                                                                                                                                                                                                         |
| Best practices | [Prompt](Prompt.md)       |                                                                                                                                                                                                                                                                                                                         |
|                | Production best practices |                                                                                                                                                                                                                                                                                                                         |
|                | Safety best practices     |                                                                                                                                                                                                                                                                                                                         |
|                | Prompt caching            | Openai 侧的缓存，降低成本                                                                                                                                                                                                                                                                                                        |
|                | Model selection           |                                                                                                                                                                                                                                                                                                                         |
|                | Latency optimization      |                                                                                                                                                                                                                                                                                                                         |
|                | Accuracy optimization     |                                                                                                                                                                                                                                                                                                                         |
|                | Advanced usage            |                                                                                                                                                                                                                                                                                                                         |
| Resources      | Libraries                 |                                                                                                                                                                                                                                                                                                                         |
|                | Prompt examples           |                                                                                                                                                                                                                                                                                                                         |
|                | Rate limits               |                                                                                                                                                                                                                                                                                                                         |
|                | Prompt generation         |                                                                                                                                                                                                                                                                                                                         |
|                | Error codes               |                                                                                                                                                                                                                                                                                                                         |
|                | Deprecations              |                                                                                                                                                                                                                                                                                                                         |

## 限流

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F27%2F04-27-30-a4b9a2b46b404be9adbfdd83d2876a2f-202410270427685-ad1693.png)

## 结果约束

JSON mode和Structured Outputs的主要区别在于：

1. JSON mode（基础版）:
- 只确保输出是有效的JSON格式
- 不保证输出符合任何特定的数据结构
- 没有数据验证和类型检查
- 你需要自己处理数据验证和错误情况

1. Structured Outputs（进阶版）:
- 不仅确保输出是有效的JSON格式
- 还能确保输出严格匹配你指定的schema（数据结构）
- 提供了完整的数据验证功能
- 支持复杂的数据类型定义和嵌套结构
- 自动进行类型检查和验证

举个例子来说：

假设你想要获取一个用户的信息：

使用JSON mode时，模型可能返回：

```json
{
  "name": "张三",
  "age": "25岁"  // 注意这里age是字符串
}
```

而使用Structured Outputs时，如果你定义了schema要求age必须是数字类型：

```json
{
  "name": "张三",
  "age": 25  // 这里一定是数字类型
}
```

所以Structured Outputs提供了更严格的数据控制，让你的应用程序能够更可靠地处理模型输出。这也是为什么在可能的情况下，建议使用Structured Outputs而不是基础的JSON mode。

# Anthropic

**project knowledge** 还没有提供 API
