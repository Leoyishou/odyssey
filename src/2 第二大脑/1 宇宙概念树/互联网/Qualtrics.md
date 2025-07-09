---
comment_id: de21c75a
date created: 2025-05-28
date modified: 2025-05-29
draw: null
title: Qualtrics
---
**Qualtrics 就是一套把「收集反馈 → 解析洞察 → 触发行动」做成“流水线”的体验管理（Experience Management, XM）平台。**你可以把它想成专门处理“人的体验数据”的全栈数据平台：前端像问卷 / 聊天机器人在不停采集信号，后端一堆 AI 引擎在实时分析，最后再把结果推给客服、营销或 HR 系统去执行。
## 为啥会有这玩意儿？

企业做客诉、员工敬业度、产品可用性测试时，常常碰到三个痛点：

1. **数据分散**：问卷在 SurveyMonkey，呼叫录音在呼叫中心，App 留评在商店 → 汇总麻烦
2. **只听不动**：大批 CSV 报表发出来没人看，更别说自动修 bug / 发优惠券
3. **缺乏场景化** **AI**：通用 LLM 懂语义，但不知道“餐饮门店 NPS 掉了 5 分要拉店长开会”这种业务逻辑
Qualtrics 把这三块打包做成 SaaS，主张“体验也要像运营数据一样闭环管理”。

## 和传统「调研工具」有什么区别？

1. **全渠道**：不仅是问卷，电话、社媒、App 端埋点都能接。
2. **行业语义模型**：Clarabridge 收购带来的 150+ 细分场景 NLP，比分词 + 情感分析准确。[Qualtrics](https://www.qualtrics.com/customer-experience/?utm_source=chatgpt.com)
3. **实时闭环**：新一代 Experience Agents 直接在客服/电商界面“代为解决”，不只是报表。[Business Insider](https://www.businessinsider.com/agentic-ai-improve-qualtrics-company-customer-communication-data-collection-2025-5?utm_source=chatgpt.com)
4. **低代码 / 无代码**：产品经理就能拖拽出“CSAT <70 → 创建 Zendesk 工单 → GPT 写安抚信”的工作流。

| 阶段          | 你能做的事                             | Qualtrics 提供的工具                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ----------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **采集**      | 出问卷、网站弹窗、App SDK、呼叫中心转写、社媒评论抓取    | Survey Builder、Website/App Feedback、Contact Center转写                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| **解析**      | 把文本归类、情感打分、找关键驱动、做预测模型            | Text iQ、Stats iQ、Driver iQ、Predict iQ ([Qualtrics](https://www.qualtrics.com/?utm_source=chatgpt.com "Qualtrics XM: The Leading Experience Management Software"), [Qualtrics](https://www.qualtrics.com/customer-experience/?utm_source=chatgpt.com "Customer Experience (CX) Management Software - Qualtrics"))                                                                                                                                                                                           |
| **呈现**      | NLG 一句话总结、自动生成图表，支持自然语言提问         | Qualtrics Assist（仪表盘聊天式问答）([Qualtrics](https://www.qualtrics.com/blog/x4-2025-innovations-showcased/?utm_source=chatgpt.com "AI-powered Innovations for the XM Platform Showcased at X4 2025"))                                                                                                                                                                                                                                                                                                            |
| **行动**      | 触发工单 / 邮件 / Slack 通知，甚至让 AI 直接回客户 | xFlow Workflows +「Experience Agents」([LinkedIn](https://www.linkedin.com/pulse/road-report-qualtrics-x4-2025-stacia-sherman-garr-p84lc?utm_source=chatgpt.com "Road Report: Qualtrics X4 2025 - LinkedIn"), [Business Insider](https://www.businessinsider.com/agentic-ai-improve-qualtrics-company-customer-communication-data-collection-2025-5?utm_source=chatgpt.com "Qualtrics' president of product has a vision for agentic AI in the workplace: 'We're going to operate in a multiagent world'")) |
| **预测 / 分群** | 把人按流失风险、忠诚度、价格敏感度自动分桶             | Experience iD (XiD) Auto-Segmentation ([Qualtrics](https://www.qualtrics.com/news/x4-2025-recap-qualtrics-redefined-the-future-of-experience-management-with-experience-agents/?utm_source=chatgpt.com "X4 2025 Recap: Qualtrics redefined the future of experience ..."))                                                                                                                                                                                                                                 |

下面把 Qualtrics 在体验管理流水线里的 **主要 AI 能力** 按层级汇总成一张速查表（从采集到行动再到预测）。

|层级|AI 功能模块|典型能力 / 作用|关键技术 / 接口|参考|
|---|---|---|---|---|
|**采集**|AI Survey Builder & Conversational Feedback|LLM 一键生成问卷、基于实时回答自动追问|Generative AI (GPT-4o) + `/survey/generate` API|([SiliconANGLE](https://siliconangle.com/2024/05/01/qualtrics-generative-ai-gives-surveys-real-time-twist/?utm_source=chatgpt.com "Qualtrics generative AI gives surveys a real-time twist - SiliconANGLE"), [SiliconANGLE](https://siliconangle.com/2024/03/28/qualtrics-delighted-subsidiary-folds-generative-ai-survey-design-service/?utm_source=chatgpt.com "Qualtrics' Delighted subsidiary folds generative AI into survey design ..."))|
||ExpertReview (Quality iQ)|检测逻辑冲突、偏见措辞、题量过多并给改进建议|规则引擎 + 微调 GPT|([Qualtrics](https://www.qualtrics.com/support/survey-platform/survey-module/survey-checker/quality-iq-functionality/?utm_source=chatgpt.com "ExpertReview Functionality - Qualtrics"))|
||Clarabridge 语音转写 & 情绪识别|呼叫录音秒级转写并标注情绪、意图、努力度|ASR + 150+ 行业 NLU 模型|([Qualtrics](https://www.qualtrics.com/clarabridge/?utm_source=chatgpt.com "Clarabridge - Qualtrics"), [SiliconANGLE](https://siliconangle.com/2021/07/29/qualtrics-buys-clarabridge-1-125b-pair-survey-feedback-sentiment-analysis/?utm_source=chatgpt.com "Qualtrics buys Clarabridge for $1.125B to pair survey feedback with ..."))|
|**分析**|Text iQ|主题抽取、情感/情绪/意图打分|多语 BERT + 自定义分类器 API|([Qualtrics](https://www.qualtrics.com/support/survey-platform/data-and-analysis-module/text-iq/text-iq-best-practices/?utm_source=chatgpt.com "Text iQ Best Practices - Qualtrics"), [Qualtrics](https://www.qualtrics.com/clarabridge/?utm_source=chatgpt.com "Clarabridge - Qualtrics"))|
||Stats iQ|自动选检验、回归、聚类并用 NLG 解释|AutoML + 统计启发式|([Qualtrics](https://www.qualtrics.com/support/stats-iq/getting-started-with-stats-iq/overview-stats-iq/?utm_source=chatgpt.com "Stats iQ Basic Overview - Qualtrics"))|
||Driver iQ|量化各属性对 NPS、CSAT 等指标贡献|回归 + 树模型 ensemble|([basecamp.qualtrics.com](https://basecamp.qualtrics.com/using-stats-iq-to-identify-key-drivers?utm_source=chatgpt.com "Using Stats iQ to Identify Key Drivers - XM Basecamp"))|
||Predict iQ|预测客户流失、复购或员工离职概率|深度 NN + 回归模型并行|([Qualtrics](https://www.qualtrics.com/support/survey-platform/data-and-analysis-module/predict-iq/?utm_source=chatgpt.com "Predict iQ - Qualtrics"))|
|**洞察呈现**|Qualtrics Assist|在仪表盘自然语言提问即返图表与行动建议|向量检索 + GPT-4o|([Qualtrics](https://www.qualtrics.com/support/employee-experience/creating-ee-project/dashboards-tab/qualtrics-assist-ex/?utm_source=chatgpt.com "Qualtrics Assist (EX)"))|
||Support AI Assistant|24×7 技术文档/工单智能问答|文档检索 + GPT|([Qualtrics Community](https://community.qualtrics.com/product-release-notes-96/meet-the-new-qualtrics-support-ai-assistant-27869?utm_source=chatgpt.com "Meet the new Qualtrics Support AI Assistant \| XM Community"))|
|**行动**|xFlow Workflows + GPT Task|触发事件→GPT 生成邮件/摘要→创建工单等|低代码流程 + OpenAI Task|([Qualtrics](https://www.qualtrics.com/support/survey-platform/actions-page/tasks/integration-tasks/gpt-task/?utm_source=chatgpt.com "OpenAI Tasks - Qualtrics"))|
||Experience Agents (Agentic AI)|多智能体协作，实时为客户/员工解决问题|Agent Router + 领域知识库 + 执行工具|([Qualtrics](https://www.qualtrics.com/platform/experience-agents/?utm_source=chatgpt.com "Experience Agents: AI Powered Experience Management (XM)"), [Business Insider](https://www.businessinsider.com/agentic-ai-improve-qualtrics-company-customer-communication-data-collection-2025-5?utm_source=chatgpt.com "Qualtrics' president of product has a vision for agentic AI in the workplace: 'We're going to operate in a multiagent world'"))|
|**预测 & 分群**|Experience iD Auto-Segmentation|基于亿级 XM 数据自动发现高风险/高价值人群|聚类 + 特征工程 + Webhook 事件|([Qualtrics](https://www.qualtrics.com/support/survey-platform/actions-page/events/experience-id-segments-events/?utm_source=chatgpt.com "Experience ID Segments Event - Qualtrics"))|

> **怎么用？**
>
> - 每个模块都有 REST 或 SDK 接口，可嵌入自家数据湖与模型；
> - Text iQ/Predict iQ 支持上传自定义数据与特征；
> - xFlow Workflows + OpenAI Task 让你“零代码”把 GPT 接到工单、CRM、Slack；
> - Experience Agents SDK 正在公测，想玩多智能体可申请内测。




-
2. question editor **写题（区域 1）** → 题目被放进 **Block** 并在 **区域 2** 的 _Show Block_ 处引用。
3. experimental flow **区域 2** 用 **Embedded Data** 接收上一题回答，再通过 **Web Service** 把这些变量连同 prompt 发给 LLM，实现 AI 生成追问 / 摘
4. participants view **区域 3** 让你即时看到这些动态逻辑的效果——例如回答完第一题后是否能顺利拿到 LLM 回包并显示下一题。

![CleanShot 2025-05-28 at 09.59.51@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_vK5Dsl2AHm%2F2025%2F05%2F28%2F10-00-04-337d289d8f24452ca4f645dc74753dd1-CleanShot%202025-05-28%20at%2009.59.51-2x-72df56.png)

```json
[ "role": "system",
"content": "Summarize the following passage, which describes a conspiratorial belief, in a single sentence. Do not nention that k is a conspiracy theory, or a belief, or provide any kind of normative judgment. Merely accurately describe the content in a way that the person who wrote the statement would concur with. Frame it as an assertion. If the statement is already short, no need to change it very much. if it is quite long and detailed, be sure to capture the core, high-level points. Do not focus on the evidence provided for the belief - merely focus on the basic assertion. For your context, the passage was written by a participant in an online academic survey in response to the following two questions: 
< Q1: Throughout history, various theories have emerged that suggest certain significant events or situations are the result of secret plans by individuals or groups. These theories often offer alternative explanations for events than those that are widely accepted by the public or presented by official sources. Some people call these "conspiracy theories". Reflecting on his, are there any specific such theories that you find particularty credible or compelling? If so, please describe one belovi and share your reasons for finding it compelling, If not, please explain that you do nor believe in any such theories. You wit stil be able to proceed with the experiment. > c Q2: Could you share more about what led you to find this theory compelling? For instance, are there specific pieces of evidence, events, sources of internation, or personal experiences that have particularly influenced your perspective? Please describe these in as much detai, as you feel comfortable. > 

Again, your role is to summarize the overarching beef captured by this response. Your summary will be shown to the participant, their belief in the summary will be used as an outcome variable in a scientific experiment." ，

"role": 'user. "content: *Question 1 Response: S(e:lField/UserConspiracy_Parti), Question 2 Response: S(e:WField/UserConspiracy_Part2y"
```
### the main question

 Throughout history, various theories have emerged that suggest certain significant events or situations are the result of secret plans by individuals or groups. These theories often offer alternative explanations for events than those that are widely accepted by the public or presented by official sources. Some people call these "conspiracy theories". Reflecting on this, are there any specific such theories that you find particularly credible or compelling? Please describe one below and share your reasons for finding it compelling.

### Elaboration

On the previous question, you wrote: *"S(q://QID18/ChoiceTextEntryValue)"* Could you share more about what led you to find this theory compelling? For instance, are there specific pieces of evidence, events, sources of information, or personal experiences that have particularly influenced your perspective? Please describe these in as much detail as you feel comfortable.
