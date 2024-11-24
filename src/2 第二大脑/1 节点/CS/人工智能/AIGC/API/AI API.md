
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

**project knowledge**  还没有提供 API