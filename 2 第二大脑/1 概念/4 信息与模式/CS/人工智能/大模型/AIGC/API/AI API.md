---
date created: 2024-11-25
date modified: 2025-07-10
uid: 5537f688-37bd-4195-8235-31e549ab6dbe
---

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

## OpenAI

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
| Best practices | [Prompt](Prompt.md)         |                                                                                                                                                                                                                                                                                                                         |
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

### 限流

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F27%2F04-27-30-a4b9a2b46b404be9adbfdd83d2876a2f-202410270427685-ad1693.png)

### 结果约束

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

## Anthropic

**project knowledge** 还没有提供 API
