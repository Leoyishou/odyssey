---
date created: 2023-11-23
date modified: 2025-10-25
uid: ef5d9acb-a676-4e3e-868d-e402443c0181
---
## 一、Agent 核心要素

1. 环境
2. [[LLM大模型]]
3. 循环、[[harness]]、[[magpie]]
	1. [[goal driven]]
	2. [[ralph loop]]
4. [[Agent tools]]
	1. [[MCP]]
	2. [[browse Use]]
	3. [[phone Use]]
5. [[记忆(Agent的)]]
6. [[多Agent，multi agents]]
	1. [[SubAgent]]
7. [[Agent IM]]交互，多端同步，交互流程，通信流程还是个挺复杂的工程和产品问题


## 二、Harness 最终一定是中心化的

harness 是 OpenAI 的，环境、上下文和 UX 是我们的


## 三、人与 Agent

### 人参与 loop 时

1. 人的[[认知带宽]]是有限的，和 Agent 协作的时候同时只做三件事最佳


### 人不参与 loop 时

1. 北极星指标：谁能消耗更多 agent？
2. 两个关键要素：
	1. Goal 目标达成的状态是否能清晰描述
		1. **告诉 AI 要做什么（What），而不是怎么做（How）**
		    1. 感受一下 loop 全程自己不参与，但是又能获得高于自己能力的作品的感受。
		    2. 最偷懒的做法是，给现成的别人优质结果让 AI 复现，最省事的定义 what 的方式。
	2. Context 是否充分
		1. **人最大程度从 loop 中撤退**：真的非我不可吗？
		    1. 离开机器不中断：锁屏不中断，电脑、手机都做一些特殊设置
		    2. 比如跨设备的 apple 账号登录，人其实做的就是眼睛从 a 设备看到验证码，再输入 b 设备，连指纹和脸部生物识别都不需要，那么这件事就是可以 AI 闭环的。

## 思考

1. 上下文大小不够是第一矛盾，在这个矛盾下，会有一段时间的[[管理学@]]红利，但是会随着模型提升下降。
2. 今日 vibe 感想：我认为无论在什么行业，知识工作基本上分为三类。第一是创意（发散），第二是分解与执行（收敛），第三是一致性确认（对齐）。三者循环一次就是一个 agentic loop
3. 对软件的影响
	1. 软件的需求并没有变，不管开发了多少APP，能装在用户手机上的始终只有那么多
	2. 自己当热源 还是找一个热源系统  老的好的软件已经充分吸热了 所以要在优秀软件上做和自己的适配化而不是重新写一个 不要会抄而是要会搬


## 应用

1. [[个人Agent]]
2. [[理想态的 Agent]]
# Cues

[[AI@]]








> [!info]
> LLM + 一个图灵完备的机器 = Agent，**智力资源像水一样，**变得成了便宜稳定的供给
> 

KOL：@yetone

**日常的****ai****充其量的效果就是雇了个保姆** **找村口老李问了问建议**  **之后背后千百个****agent****被组织起来工作** **才能每天消耗一亿****token**     **中国起飞或者工程师红利就得益于恢复高考后的全民总****token****输出提高**   **给你一千万花完很难给你一亿****token****花完也很难**



Agent 是面向结果交付的，所以**和** **Agent** **打交道的时候，先无脑说出自己想要的最终效果，等发现他少什么图，再补就好。**
[[图灵完备]]

1. [[bash]]强的可怕，因为 LLM 训练数据里有海量的 Linux 操作记录，它天生就会用 Linux。
2. [[本地 Agent $]]
3. [[browse Use]]
4. [[phone Use]]
5. [[human in the loop]]
6. 

## 好用的资源

https://github.com/AccumulateMore/CV


![CleanShot 2026-08-05 at 19.26.07@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-08-05%20at%2019.26.07%402x.png)


## Mac OS

`security` 命令，就是 macOS 专门提供给开发者，用来在命令行里操作 Keychain 的内置系统级指令。




## 最佳实践
### Thariq Shihipar 的 Workshop 核心（“8点心得”深度解析）

这个视频名为《Claude Agent SDK [Full Workshop]》，Thariq 在其中手把手演示了如何用 Claude Agent SDK 构建一个 Agent。X 上流传的“8点心得”通常涵盖以下核心逻辑：


**Agent Loop 的本质（The Loop）**

- **流程：** 所有的 Agent 最终都简化为一个死循环： `Context（环境）` -> `Thought（思考）` -> `Action（行动/工具调用）` -> `Observation（观察执行结果）` -> `回到 Context`。
    
- **简单性：** 不要设计复杂的图（Graph），先跑通这个最简单的单线程循环。


1. Claude Agent SDK + 提示词编写初版 Agent，然后根据效果去细化提示词指令，再把确定性部分尽可能抽离成 Agent Skill 和配套脚本。
2. 如果满足不了需求，再考虑更细粒度的 Multi-Agent 框架比如 CrewAI、AutoGen。
3. 再满足不了就自己 LangChain/LangGraph/纯 OpenAI SDK 手撸或者拿 OpenCode 等框架去改。


## 全自动模式很重要


## Agent 设计

1. 对于 Manus、Devin、Claude Code 这些产品来说，Model（模型）是水电煤，Orchestration Code（调度层代码）才是真正的发动机。
2. - 调度层代码 (Orchestration)：这是包含了人类工程师无数次 Debug 经验的结晶。它把一个“会说话的 AI”变成了一个“会干活的 Agent”。


你提到的 Claude Code、Gemini CLI、Codex (Copilot) 的核心 Agent 调度逻辑目前都是闭源的。它们把这部分代码视为核心商业机密。



|                  |                                               |
| ---------------- | --------------------------------------------- |
| 编排层orchestration | 1. 先用 Claude Code，<br>2. 然后基于 OpenCode 试试慢慢自研 |
| 给 AI 用的 Tool 的设计 | 1. 模拟用户阅读小红书，可以在选项卡问用户平常是否喜欢参考社交媒体里的攻略<br>2.  |
| 沙箱环境             | 1. 先用 docker                                  |
| 上下文              | 1. 潜台词，用户浏览记录、籍贯                              |
| 后置 hook 做最终的结果约束 | 出一份agent的结果 全部使用schema                        |



## 核心调度循环

```
// 核心调度循环
while (true) {
  // 1. 调用 LLM 流式输出
  const stream = await LLM.stream(streamInput)
  
  for await (const value of stream.fullStream) {
    // ... 处理 token 流 ...
    
    // 2. 自动检测 "Doom Loop" (死循环)
    // 如果连续 3 次调用同一个工具且参数完全一样，立刻暂停并询问用户
    if (lastThree.length === DOOM_LOOP_THRESHOLD && lastThree.every(isSameTool)) {
       await PermissionNext.ask({ permission: "doom_loop", ... })
    }
  }

  // 3. 执行工具 (Tool Execution)
  // ...

  // 4. 自动快照 (Snapshot)
  // 每次执行完工具，都会对文件系统做一次快照，记录变更 (Diff)
  snapshot = await Snapshot.track()
  
  // 5. 上下文压缩 (Context Compaction)
  // 如果 Token 溢出，触发压缩逻辑
  if (needsCompaction) return "compact"
}
```



2. 把chat 和agent两个事分开，one by one 或者 one by n
3. 







## 2）在 Claude Code 上做扩展（plugins / slash commands / agents / hooks / MCP）——这是官方鼓励的路

Claude Code 的官方定位就是“终端里的可组合、可脚本化工具”，并且支持用 MCP 接外部工具、用 插件系统做自定义命令/专用 agent。([Claude Code](https://code.claude.com/docs/en/overview "Claude Code overview - Claude Code Docs"))

对“行程规划 agent”来说，最合理的做法是：不要把 Claude Code 当成旅游 App，而是当成你的 agent 运行壳，把旅行能力放在“可控的工具层”。

---

## 2.1 工具层

Claude Code 能通过 `.mcp.json` 配置 MCP server，并且有清晰的作用域/优先级（本地/项目/企业）与环境变量注入，适合放 API Key。([Claude Code](https://code.claude.com/docs/en/mcp "Connect Claude Code to tools via MCP - Claude Code Docs"))

你可以做一个 `travel-mcp`，提供工具（Tools）比如：

- `search_poi(query, city)`：POI 搜索/去重/打标签
    
- `route_time(from, to, mode)`：交通时间（步行/公共交通/驾车）
    
- `open_hours(poi, date)`：营业/预约/季节性关闭
    
- `weather(city, date_range)`：天气 + “雨天替代方案”触发
    
- `budget_estimate(itinerary)`：粗略预算校验（门票/交通/餐饮）
    

这样模型就不会靠“脑补”，而是每次规划都查工具。

> `.mcp.json` 结构示意（最小形态）：

```json
{
  "mcpServers": {
    "travel": {
      "type": "http",
      "url": "${TRAVEL_MCP_URL}",
      "headers": { "Authorization": "Bearer ${TRAVEL_MCP_KEY}" }
    }
  }
}
```

（环境变量展开是官方支持的。）([Claude Code](https://code.claude.com/docs/en/mcp "Connect Claude Code to tools via MCP - Claude Code Docs"))

## B. 用插件/自定义 slash command 做入口

Claude Code 支持自定义 slash commands，且有命名空间等机制，适合做 `/trip`、`/replan`、`/day2` 这种交互入口。([Claude Code](https://code.claude.com/docs/en/slash-commands?utm_source=chatgpt.com "Slash commands - Claude Code Docs"))  
插件市场/分发也有官方文档路径（你可以把自己的插件打包给团队装）。([Claude Code](https://code.claude.com/docs/en/plugin-marketplaces?utm_source=chatgpt.com "Create and distribute a plugin marketplace"))

你最终的用户体验可以是：

- `/trip 上海 3天 预算2000 喜欢citywalk 住静安`
    
- Claude Code 调 MCP：拉 POI → 算动线 → 查天气 → 输出行程（Markdown + JSON）
    
- `/replan day2 雨天`：只重排第二天，并保持预算与强约束
    

## Hooks

Claude Code 有 hooks 机制（从 CHANGELOG 可见 hooks 已作为特性加入并持续演进），适合做：

- 输出后自动跑一个 `validate_itinerary.py`：检查每日时间总和、跨城不合理、预算超标、POI 重复等
    
- 校验不过就让 Claude 自动修（给出具体错误列表）
    

（hooks 的存在与演进在官方 changelog 中可以看到。）([GitHub](https://raw.githubusercontent.com/anthropics/claude-code/refs/heads/main/CHANGELOG.md?utm_source=chatgpt.com "https://raw.githubusercontent.com/anthropics/claud..."))

## 上下文

你可以把这个“行程规划”当成一个 repo/项目：

- `CLAUDE.md` 写产品规则：必须输出结构化 JSON、必须给备选、必须标注风险点
    
- `profiles/` 放不同用户画像模板（亲子/摄影/老年/特种兵）
    

Claude Code 本来就擅长在“项目上下文”里工作。([Claude Code](https://code.claude.com/docs/en/overview "Claude Code overview - Claude Code Docs"))

---

# 什么时候“不该用 Claude Code”而该用别的？

如果你要把它做成面向 C 端的产品（网页/小程序/APP），Claude Code 会显得别扭：它是开发者工具壳。官方也明确有 Claude Agent SDK（用于真正搭 agent 应用），更适合产品化落地。([Anthropic](https://www.anthropic.com/engineering/building-agents-with-the-claude-agent-sdk?utm_source=chatgpt.com "Building agents with the Claude Agent SDK"))

---

# 最小落地路线（你今天就能开工的那种）

1. 先写一个 `travel-mcp`：哪怕只实现 `search_poi` + `route_time` 两个工具
    
2. 在 Claude Code 里配 `.mcp.json` 接上它([Claude Code](https://code.claude.com/docs/en/mcp "Connect Claude Code to tools via MCP - Claude Code Docs"))
    
3. 做一个 `/trip` 命令：输入城市/天数/预算/偏好，输出 Markdown + JSON([Claude Code](https://code.claude.com/docs/en/slash-commands?utm_source=chatgpt.com "Slash commands - Claude Code Docs"))
    
4. 加一个校验脚本 + hook：保证“可执行性”底线([GitHub](https://raw.githubusercontent.com/anthropics/claude-code/refs/heads/main/CHANGELOG.md?utm_source=chatgpt.com "https://raw.githubusercontent.com/anthropics/claud..."))
    

如果你告诉我你希望 MCP 里接的“真实数据源”是哪类（OSM/Google Places/高德/携程或你们内部 POI 库/天气/交通），我可以直接给你一套 travel-mcp 的工具接口定义（tool schema） + 行程 JSON schema + 校验规则清单，让你能快速把评测/可执行性做起来。



- 粗略公式
      - Agent ≈ n 次 [[RunnableSequence]]（每次=提示拼装 → 1 次 LLM）+ 中间穿插的工具执行（不算在 RunnableSequence 里）+ 可选 1 次 response_format 的最终 LLM。
  - 计数
      - LLM 次数 = n + (response_format? 1: 0)
      - Tool API 次数 = 各轮产生的 tool_calls 之和（每轮可能为 0、1、多个）
  - 退出机制
	  - 退出条件 = 某次 RunnableSequence 的输出不再包含 tool_calls（或达到步数上限）

  所以，用通俗口径说“n × runnableSequence + response_format 就是 agent”基本对，但要记得工具执行是独立于 RunnableSequence 的插入步骤，不计入 LLM
  次数。





以前模型能力不行，我们要在 prompt 里写死每一步："先做 A，再做 B，然后做 C"；    现在模型更聪明了，我们只需要说"这是你的目标，这些是你的工具，自己看着办"，让 Agent 自主决策。  里面的关键是做好四件事：写清晰的 prompt、设计高效的工具、用即时检索代替预加载、管理好有限的 context 窗口（通过压缩、笔记或拆分  sub agent 等方式）。


[[Parlant]]


https://github.com/shareAI-lab/analysis_claude_code

Agent 有两个变量，
1. 一个是控制任务走向的 workflow 工作流，
2. 一个是控制内容生成的 context 上下文。1）

分类
1. 如果 workflow 和 context 的确定性都很高，这类任务就容易被自动化，类似传统 RPA，比如在处理发票处理、表单填报任务时，Al 更多是粘合剂，发挥空间比较有限。
2. 如果 workflow 确定但 context 不确定，也就是流程固定但输入多变，就需要 Agent 在语义和理解上补全，比如客服问答、合同解析，需要通过外部检索、知识图谱等工具来弥补信息的缺口，让推理结果更符合预期。
3. 如果 workflow 不确定但 context 确定，也就是输入清晰但走法多样，Agent就要去自主规划路径，例如市场分析报告生成、个性化推荐等，大多数 End-to-End RL Agent都擅长做这类任务，因为它们在训练阶段就习得了大量的路径规划和解题思路。
4. 而当 workflow 和 context 都不确定时，就是最复杂的场景了，既要推理也要探索，像创新方案设计、跨部门信息收集等，这类更偏向于通用型 Agent，它的执行效果，取决于给它配备的工具丰富度，尤其是编程能力要最大化开放，例如让它学会去 Github 找仓库克隆并修改代码来解决问题，让它像人一样干活儿。所以，要把 Agent 做好，首先要明确场景。本质上，自动化解决的是“确定性”问题，而智能化解决的是“不确定性”问题。


![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F08%2F30%2F23-51-58-360bc9dde2b6feb83f301c48dcfc5317-202508302351637-377e9d.png)



![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F08%2F30%2F23-52-12-477cb357833210197ff64dede3f161c4-202508302352401-153775.png)
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F08%2F30%2F23-53-00-dd2f64833f3be50b306014db84702ef8-202508302353778-15b409.png)

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F06%2F22%2F17-24-01-c5c45b18b04df68c4875acad4a0681fb-202506221724165-718f08.png)

突然有个思考 agent的不确定来自于需求的不确定，刚性需求其实全部接口化了。  所以更好的模式是工作台。比如spring 比如gammas

关于 Agent 项目，抛开眼花缭乱的词汇，解决方案其实就三个方向：

1. Prompt engineering + Workflow（如 ReAct）
2. "协议token" + 工具调用接口 + GRPO 微调
3. RLVR + 系统级可靠性验证 在一般情况下，难度、可靠性、潜在价值 1 < 2 < 3。

你这段话非常精炼，已经准确划分了当前 Agent 项目的三种主流实现路径。针对你问的后两种：

---

# ✅ 方向 2：「协议 token + 工具调用接口 + GRPO 微调」

这是增强工具使用能力的 Agent架构，关键在于 明确意图、准确调用工具，是当前最具实用价值的路径之一。

## 🔍 核心构成解释

|概念|说明|
|---|---|
|协议 token|类似一种“约定俗成的格式化提示语”或“语义标签”。例如：在对话中出现 `[search]东京天气`、`<calc>1+2`，这些都是让 Agent 识别并调用对应工具的“标记符”或“触发信号”。|
|工具调用接口|Agent 可以调用 API（如搜索引擎、数据库、日历等），执行具体任务。这部分类似“插件”机制。|
|GRPO 微调|是指 对大模型进行轻量微调（如LoRA），以便它更擅长处理工具调用任务。GRPO 可被理解为 “Generalized Retrieval-Planning-Observation”，是一种让 Agent 更有效在“检索-规划-执行观察反馈”循环中发挥作用的微调范式。|

👉 典型代表：LangChain + Function Calling / OpenAI Toolformer / HuggingGPT 等。

---

# ✅ 方向 3：「RLVR + 系统级可靠性验证」

这是最复杂、最可靠也最前沿的 Agent 实现方向，强调模型在真实任务中的长期表现和鲁棒性。

## 🔍 核心构成解释

|概念|说明|
|---|---|
|RLVR|Reinforcement Learning with Verified Rewards，意为“带验证奖励的强化学习”。它改进了传统 RLHF（人类反馈强化学习），避免奖励漂移或不一致，通过规则验证系统来判断 Agent 行为的对错。|
|系统级可靠性验证|Agent 不再依赖简单的 prompt 和即时反馈，而是要在 多轮决策、多工具调用、复杂状态演化中表现出稳定性和正确性。这涉及任务评估指标设计、测试用例库、错误回滚机制等系统级手段。比如 Agent 是否在处理旅行规划、医疗问答、复杂路由优化中始终靠谱。|

👉 典型代表：AutoGPT、Voyager、Meta的反事实反馈框架、Anthropic 的 Constitutional AI 等。

---

# 📊 三者对比总结

|方向|实现难度|稳定性|应用门槛|代表模型或工具|
|---|---|---|---|---|
|1. Prompt+Workflow|⭐|⭐|低|ReAct, Tree of Thought|
|2. 协议+工具+GRPO微调|⭐⭐|⭐⭐|中|LangChain, Toolformer|
|3. RLVR+系统级验证|⭐⭐⭐|⭐⭐⭐|高|AutoGPT, Voyager|

---

如你正在落地 Agent 项目，第 2 种方式是性价比最高的；第 3 种适合走在前沿或探索高度可靠系统（如金融、医疗、工业自动化等）。

如果你想结合场景细化方案，我可以帮你出一版适配框架。

___

好的岗位分工是概念抽象的好，好的框架是概念抽象的好，好的系统架构也是



#最佳实践

https://docs.llamaindex.ai/en/stable/getting_started/concepts/

https://docs.llamaindex.ai/en/stable/understanding/

|教程部分|子主题|
|---|---|
|Using LLMs|Getting started with LLMs|
||Utilizing supported LLMs (via remote API calls or local execution)|
|Building Agents|Building a single agent|
||Using existing tools (LlamaHub registry)|
||Maintaining state|
||Streaming output and events|
||Human in the loop|
||Multi-agent systems with AgentWorkflow|
|Building Workflows|Building a simple workflow|
||Visualizing workflows|
||Looping and branching|
||Concurrent execution|
||Streaming events|
||Multi-agent systems from scratch|
|Adding RAG to Your Agents|Loading & Ingestion|
||Indexing and Embedding|
||Storing|
||Querying|
||Putting it all together|
||Tracing and debugging (observability)|
||Evaluating|

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F16%2F11-43-20-3f1097d801711236dcb377a7c45680c7-202504161143562-db4734.png)

# 2025 AI Agents Infrastructure Stack

![image.png|1200](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F12%2F15-08-54-c5c45b18b04df68c4875acad4a0681fb-202504121508750-d88b7a.png)

|                                        |                                        |                                                                                                                                                                     |             |
| -------------------------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| 大标题                                    | 类别                                     | 工具                                                                                                                                                                  | 最佳实践？|
| PLATFORM                           | PaaS/BaaS                              | fly.io, griptape, Render, netlify, NEON, supabase, Vercel, Railway, AXIOM                                                                                           |             |
|                                        | Logging                                | Grafana, Datadog, RAYGUN                                                                                                                                            |             |
|                                        | Observability, Tracing, and Evaluation | AgentOps, Metoro, LangSmith, Langfuse, braintrust, Patronus AI, COVAL, Oplik                                                                                        |             |
|                                        | Agent Frameworks                       | AgentStack, LangGraph, AG, Boundary, PG2, CAMEL AI, ControlFlow, Llamindex, Prefleon, emergegenie, OpenAI Agents SDK, Microsoft                                     |             |
| ORCHESTRATION                      | Persistence                            | inngest, hatchet, trigger.dev                                                                                                                                       |             |
|                                        | Agent Routing                          | Temporal, LangGraph, octo.ai                                                                                                                                        | LangGraph   |
|                                        | Model Routing                          | Letta, Marteen, acreo.ai, Modal                                                                                                                                     |             |
| DATA                               | Memory                                 | cognee, mem, zep                                                                                                                                                    |             |
|                                        | Storage                                | NEON, supabase, Pinecone, chroma, Weaviate, mongoDB, Fireproof, MotherDuck, tinybird, drant, neo4j                                                                  | neo4j       |
|                                        | ETL(Extract-Load-Transform)            | LlamaIndex, reducto, DATAVOLO, verodat                                                                                                                              | LlamaIndex  |
| AGENTS AS A SERVICE (TOOLS)        | Search                                 | exa, serper, glean, meilisearch, Search1API, Tavily                                                                                                                 |             |
|                                        | Data Extraction                        | Parallel, Firecrawl, TINY FISH, Browse AI, oxylabs, NIMBLE, bright data                                                                                             |             |
|                                        | UI Automation                          | Browser Use, Browserbase, bytebot, LaVague, AGI.inc, notte, OS-ATLAS, Open Interpreter, HyperWrite, Anthropic Computer Use, OpenAI Operator, Google Project Mariner |             |
|                                        | Payments                               | Open Commerce, payman, Skyfire, protege, Stripe Agent SDK                                                                                                           |             |
| SECURE TOOL USAGE & AUTHENTICATION | Secure Tool Usage                      | composio, ARCADE, LAYER, Paragon, mcprun, Unified, wildcard, Toolhouse, glean                                                                                       |             |
|                                        | Auth                                   | Auth0, clerk, ANON, okta, OpenFGA, outhzed                                                                                                                          |             |
| BROWSER INFRASTRUCTURE & SANDBOXES | Browser Infrastructure                 | Browserbase, Browser Use, Browsersless, APIFY, Cloudflare, platform.sh                                                                                              | Browser Use |
|                                        | Sandboxes                              | E2B, MODAL, CodeSandbox, Pig, SCRAPYB ARA, Cloudflare, RIZA, ForeverVM, Daytona, WebContainers                                                                      | 阿里云的无影云手机   |
| AGENTS                             | Next Gen Copilots                      | perplexity, gradial, Cleric, glean                                                                                                                                  |             |
|                                        | Agent Teammates                        | Astral, bolt.ax, DEVIN, Dropzone AI                                                                                                                                 | DEVIN       |
|                                        | Agent Swarms                           | aaru, SOCIETIES, Canopy, CommonRoom                                                                                                                                 |             |

# 九天菜菜-大模型与Agent开发实战

## 大模型与Agent开发实战【新课】

### 1-在线大模型部署调用

- DSPy优化器优化LLM程序.ts
- DSPy：斯坦福20k+星标项目 - 革命性的LLM优化框架，从评估到自动优化你的LLM系统（如RAG系统）.ts
- GLM 4 工具调用从入门到精通与CogVideoX 文本生成视频 API 全流程.ts
- GLM Assistant API集成与FastAPI服务复刻实战指南.ts
- GLM模型：流式处理的工具调用、多模态应用及模型微调实战指南.ts
- Claude 系列
  - 【Claude】01_了解Claude的所有特质.MP4
  - 【Claude】02_Claude的注册流程.ts
  - 【Claude】03_API_KEY领取及调用.ts
  - 【Claude】04_API必须参数揭秘,多轮对话机器人速成指南_x264.ts
  - 【Claude】05_API多模态的最佳实践.ts
  - 【Claude】06_API生成定制化与提示工程实战长文档.mp4
  - 【Claude】07_tool use全流程讲解与wiki搜索实战.ts
  - 【Claude】08_tool_choice与定制Json输出及tool_use大实战.ts
  - 【Claude】09_揭秘prompt Cache提示缓存：节省成本加快响应的绝技.ts
  - 【Claude】10_stream函数及生成器本质讲解.ts
  - 【Claude】11_stream流式概念与使用方法讲解.ts
  - 【Claude】12_异步的使用原因与基本语法.ts
  - 【Claude】13_异步的API调用的讲解与实践.ts
- Gemini 系列
  - 【Gemini】01_Gemini注册指南-最新模型免费API调用.ts
  - 【Gemini】02_生态体验攻略与微调实战.ts
  - 【Gemini】03_解锁 Gemini API-开启文本生成之旅.ts
  - 【Gemini】04_掌握原生generate_content函数多轮对话技巧.ts
  - 【Gemini】05_API内置简化多轮对话的start_chat函数与ChatSessi.ts
  - 【Gemini】06_API精细化控制生成之System Instruction.ts
  - 【Gemini】07_API精细化控制生成之generation_config与max_t.ts
  - 【Gemini】08_API精细化控制生成之Temperature与Stop Sequen.ts
  - 【Gemini】09_提示工程之长文档实战.ts
  - 【Gemini】10_API精细化控制生成结构化与Json参数.ts
- GLMAssistantAPI 系列
  - 【GLMAssistantAPI】01_调用assistantAPI流程与效果.ts
  - 【GLMAssistantAPI】02_多个assistant演示解析使用.ts
  - 【GLMAssistantAPI】03_将智能体集成Agent类使用.ts
  - 【GLMAssistantAPI】04_构建fastapi的Web项目.ts
  - 【GLMAssistantAPI】05_复刻AssistantAPI.ts
- GLM 系列
  - 【GLM】01_GLM大模型API调用介绍与领取APIKEY.ts
  - 【GLM】02_GLM模型API初次调用与响应解析.ts
  - 【GLM】03_大模型API调用的消息和构建多轮对话机器人.ts
  - 【GLM】04_十分钟速通GLM的可选参数.ts
  - 【GLM】05_GLM的stream流式响应讲解.ts
  - 【GLM】06_tooluse上-一次api调用的tooluse.ts
  - 【GLM】07_tooluse下-两次api调用的tooluse.ts
  - 【GLM】08_tooluse接入维基百科搜索实战.ts
  - 【GLM】09_CogVideoX视频生成大模型.ts
- GPT4o 系列
  - 【GPT4o】01_4o文本生成API基本讲解与调用.ts
  - 【GPT4o】02_2个核心必需参数详解与对话消息本质剖析.ts
  - 【GPT4o】03_多轮对话历史及面向对象多轮对话机器人实战.ts
  - 【GPT4o】04_精细控制生成可选参数系列讲解.ts
  - 【GPT4o】05_4o多模态图片的文本生成API调用详解.ts
  - 【GPT4o】06_3步的 tool use基础讲解与【实战】社交媒体情绪数据分析.ts
  - 【GPT4o】07_5步的 tool use完整的调用流程与汇总.ts
  - 【GPT4o】08_tool use 维基百科实时写作助手【实战】.ts
  - 【GPT4o】09_并行的函数调用与实时股票价格、历史走势绘图与分析【实战】.ts
  - 【GPT4o】10_输出生成结构化的3种方式（JSON结构化）.ts
  - 【GPT4o】11_首token时延与提示缓存原理与最佳实践.ts
  - 【GPT4o】12_Streaming流式响应详解.ts
  - 【GPT4o】13_函数调用中的Streaming流式响应.ts
  - 【GPT4o】14_风格微调（上）创建数据集与微调作业.ts
  - 【GPT4o】15_风格微调（下）微调作业运行与应用评估.ts
- GPTo1 系列
  - 【GPTo1】01_o1大模型原理介绍 从4o到o1实际上发生了什么.ts
  - 【GPTo1】02_o1模型的API调用详解与总结.ts
  - 【GPTo1】03_通过COT制作4o-mini版本的o1模型【实战】.ts
  - 【GPTo1】04_2种加载模型o1没有的Json结构功能与【实战】.ts
  - 【GPTo1】05_用o1蒸馏模型实战（上）.ts
  - 【GPTo1】06_用o1蒸馏模型实战（下）.ts
- GPT 系列
  - 【GPT】01_OPENAI的账号注册速通.ts
  - 【GPT】02_OPEN AI官网及开发者平台使用指南.ts
  - 【GPT】03_虚拟信用卡开通与升级GPT_Plus会员.ts
  - 【GPT】04_API账户充值及领取API_KEY及首次调用API.ts
- 提示工程全解指南.ts

### 2-开源大模型部署调用

- Cursor&Coder高效低代码开发办法.ts
- Glm-Edge-v-5B模型介绍与本地部署.ts
- Marco-o1模型介绍与本地部署.ts
- Ollama最新版本保姆级指南.ts
- Ollama最新版本启动多模态模型Llama3.2Vision全流程[Linux环境].ts
- Qwen2--5Coder本地部署使用全流程.ts
- Qwen2--5Coder：32B基本介绍和测试.ts
- Qwen2VL多模态模型参数介绍&部署使用全流程.ts
- ChatGLM3-6B 系列
  - 【ChatGLM3-6B】01_Ubuntu双系统安装.MP4
  - 【ChatGLM3-6B】02_Ubuntu环境下ChatGLM3-6b命令行安装办法.MP4
  - 【ChatGLM3-6B】03_服务器环境下部署ChatGLM3-6b.ts
  - 【ChatGLM3-6B】04_ChatGLM3-6b的多种启动方法.ts
  - 【ChatGLM3-6B】05_AutoDL快速部署ChatGLM3-6b办法.ts
  - 【ChatGLM3-6B】06_Git方法简洁下载ChatGLM3-6b.ts
  - 【ChatGLM3-6B】07_以ChatGLM3-6b为例的大模型更新办法.ts
  - 【ChatGLM3-6B】08_单机多卡启动大模型办法.ts
  - 【ChatGLM3-6B】09_LoRA原理浅析.ts
  - 【ChatGLM3-6B】10_LoRA微调实战训练.ts
  - 【ChatGLM3-6B】11_DeepSpeed原理浅析.ts
  - 【ChatGLM3-6B】12_PEFT高效微调原理浅析.ts
  - 【ChatGLM3-6B】13_P-Tuning V2微调实战.ts
  - 【ChatGLM3-6B】14_LangChian体系详解.ts
  - 【ChatGLM3-6B】15_LangChain0.3安装部署与调用方法.ts
- 【Cursor】在Windows环境下调用QWQ线上API实现辅助编程方法.ts
- 【Cursor】调用QWQ实现辅助编程--调用本地模型全流程.ts
- GLM4-9B 系列
  - 【GLM4-9B】01_模型基本介绍.ts
  - 【GLM4-9B】02_安装部署流程展示.ts
  - 【GLM4-9B】03_多种启动方式流程展示.ts
  - 【GLM4-9B】04_vLLM介绍和部署应用.ts
  - 【GLM4-9B】05_WSL部署流程.ts
  - 【GLM4-9B】06_Git方法简洁下载ChatGLM3-6b.ts
  - 【GLM4-9B】07_以ChatGLM3-6b为例的大模型更新办法.ts
  - 【GLM4-9B】08_单机多卡启动大模型办法.ts
  - 【GLM4-9B】09_LoRA原理浅析.ts
  - 【GLM4-9B】10_LoRA微调实战训练.ts
  - 【GLM4-9B】11_DeepSpeed原理浅析.ts
  - 【GLM4-9B】12_PEFT高效微调原理浅析.ts
  - 【GLM4-9B】13_P-Tuning V2微调实战.ts
  - 【GLM4-9B】14_LangChian体系详解.ts
  - 【GLM4-9B】15_LangChain0.3安装部署与调用方法.ts

### 3-大模型微调实战

- Ch 1.1 通用模型发展与微调前景.ts
- Ch 1.2 微调与其他技术的区别和关联.ts
- Ch 1.3 微调步骤及技术栈.ts
- Ch 1.4 高效微调及范围.ts
- Ch 1.5 强化学习RHLF介绍.ts
- Ch 1.6 主流微调框架.ts
- Ch 2.1 Lora原理详解.ts
- Ch 2.2 LLama_factory 介绍.ts
- Ch 2.3 LLama_factory 安装部署及验证.ts
- Ch 2.4 实战Qwen2.5微调.ts
- Ch 2.5 配置TensorBoard.ts
- Ch 3.1 如何获取数据源及使用解析.ts
- Ch 3.2 llamafactory中如何使用和注册数据集.ts
- Ch 3.3 模型微调及参数详解.ts
- Ch 3.4 数据验证&deepspeed微调.ts
- Ch 4.1 win&Qlora&qwen2.5_Qlora能节约多少内存.ts
- Ch 4.2 Qlora核心概念.ts
- Ch 4.3 如何在windows配置环境.ts
- Ch 4.4 Qlor微调实战与验证.ts
- Ch 5.1PPO&llama3.2_什么是PPO算法.ts
- Ch 5.2 如何理解PPO.ts
- Ch 5.3 各个模型PPO算法的协作关系.ts
- Ch 5.4 PPO模型与数据的选择.ts
- Ch 5.5 PPO微调实操.ts
- Ch 5.6 PPO源码解读.ts
- Ch 6.1 RLHF&Qwen2.5,RLHF背景与意义.ts
- Ch 6.2 RLHF流程拆解与核心.ts
- Ch 6.3 奖励模型原理.ts
- Ch 6.4 奖励模型训练流程.ts
- Ch 6.5 RLHF全流程微调.ts
- Ch 6.6 RLHF挑战与改进方向.ts
- DPO微调Qwen2.5.ts
- LLama_factory&Lora微调Qwen2.5做NL2sql.ts
- LLama_Factory+LORA大模型微调.ts
- PPO微调Llama-3.2.ts
- window系统微调QLORA.ts

### 5-RAG项目实战企业篇

- week_1_1_part_1_课程说明及大模型问答功能复现.mp4
- week_1_1_part_2_本地私有化部署glm4-9b-chat模型.mp4
- week_1_1_part_3_调用本地启动的glm4模型的方法.mp4
- week_1_2_part_1_GLM在线API调用方法.mp4
- week_1_2_part_2_Langchain应用框架整体介绍.mp4
- week_1_2_part_3_FastAPI项目介绍.mp4
- week_2_1_part_1_本周开发任务说明.mp4
- week_2_1_part_2_Naive RAG介绍及借助LangChain实现RAG流.mp4
- week_2_1_part_3_RAG集成历史对话信息及SqlalChemy持久化存储的使用.mp4
- week_3_1_part_1_企业级RAG的构建架构和思路.mp4
- week_3_1_part_2_结合Faiss向量数据的RAG Indexing构建流程.mp4
- week_3_1_part_3_百万量级的wiki公有语料构建方法实践.mp4
- week_3_1_part_4_yolox + ocr 识别私有PDF语料的方案实践.mp4
- week_3_2_part_1_本地RAG知识库问答功能链路详解与复现.mp4
- week_3_2_part_2_RAG评估框架-TruLens实践.mp4
- week_3_2_part_3_RAG评估框架-Ragas的使用及借助GLM4生成评估数据.mp4
- week_4_1_part_1_自定义RAG评估Pipeline-构建统一的数据集格式.mp4
- week_4_1_part_2_自定义RAG评估Pipeline-构建统一的提示模板.mp4
- week_4_1_part_3_自定义RAG评估Pipeline-整体逻辑实现及细节说明.mp4
- week_4_1_part_4_自定义RAG评估Pipeline-评估指标及评估流程实现.mp4
- week_4_1_part_5_自定义RAG评估Pipeline-ReRanker原理及实践.mp4
- week_4_2_part_1_Serper API联网检索数据预处理过程拆解.mp4
- week_4_2_part_2_基于Serper实现系统架构下的联网实时检索功能.mp4
- week_5_part_1_大模型融入推荐系统一种思路_1.MP4
- week_5_part_2_大模型用于特征工程及实现推荐系统的整体逻辑代码.MP4
- week_5_part_3_基于Langchain实现基于ReAct的代码实践及TOT提示.MP4
- week_5_part_4_项目整体框架下实现基于Agent的问答流程代码实践.ts
- week_5_part_5_Docker介绍及借助Docker打包部署完整项目的流程.mp4

### 6-直播回放篇

#### 加餐-前沿技术追更

- 【加餐】01 Computer Use API 详解（1）准备工作.ts
- 【加餐】02 Computer Use API 详解（2）对话函数上.ts
- 【加餐】03 Computer Use API 详解（3）对话函数下.ts
- 【加餐】04 Computer Use API 详解（4）多轮对话及tool use.ts
- 【加餐】05 Computer Use API 详解（5）全组件总结.ts
- 【加餐】06 Computer use实战初窥.ts
- 【加餐】07 Computer Use docker安装部署全流程.ts
- 【加餐】08 Computer Use项目启动&实战案例.ts
- 【加餐】09 Computer Use源码详解.ts

#### 直播回放

- ChatGPT的注册与ChatGPT API充值.ts
- GLM-4-Video-9b 介绍与部署流程.ts
- GPT4o的API基本调用、精细生成控制与多模态实操.ts
- GPT4o的函数调用实战与结构化生成回答与提示缓存原理.ts
- LangGraph 中 Human-in-the-loop 应用实战.ts
- LangGraph 实现自治循环代理（ReAct）及事件流的应用.ts
- LangGraph 长短期记忆实现机制及检查点的使用.ts
- LangGraph中State状态模式与LangSmith基础使用入门.ts
- Llama 3.1&2部署微调.ts
- Llama 3.1介绍与部署流程.ts
- o1模型详解与模型蒸馏实战.ts
- OPENAI的Streaming流式响应与API微调双实战，Claude computer use接入自己的电脑.ts
- Qwen2.5-Coder&Math介绍与部署.ts
- Qwen2.5介绍与部署流程.ts

### PART4：DeepSeekV3?(5小节)

- 【DeepSeekv3】1 整体架构与分布式基础概念.mp4
- 【DeepSeekv3】2 分布式嵌入层与投影层.mp4
- 【DeepSeekv3】3 MLA潜在注意力 (1) 从自回归掩码看KV缓存机制.mp4
- 【DeepSeekv3】4 MLA潜在注意力 (2) 针对KV缓存的改进.mp4
- 【DeepSeekv3】5 DeepSeekMOE的MOE架构与创新.mp4

---

## 大模型与Agent开发实战课件

### Agent开发实战-课件

- 01_大模型应用发展及Agent前沿技术趋势.ipynb
- 02_AI Agent应用类型及Function Calling开发实战.ipynb
- 04_OpenAI Assistant API 基本理论与入门实战.ipynb
- 05_OpenAI Assistant API 进阶应用.zip
- 07_LangGraph底层原理解析与基础应用入门.ipynb
- 08_LangGraph中State状态模式与LangSmith基础使用入门(1).ipynb
- 09_单代理架构在 LangGraph 中构建复杂图的应用.ipynb
- 10_LangGraph 实现自治循环代理（ReAct）及事件流的应用.ipynb
- 11_LangGrah 长短期记忆实现机制及检查点的使用.ipynb
- 12_LangGraph 中 Human-in-the-loop 应用实战.ipynb
- 13_LangGraph Multi-Agent Systems 开发实战.ipynb
- 16_MicroSoft AutoGen 代理对话与人机交互源码解析.ipynb
- 03_ReAct Agent 基础理论与项目实战
  - 03_ReAct Agent 基础理论与项目实战.ipynb
  - ReAct_AI_Agent.zip

### 在线大模型部署与调用-课件

#### 【Claude 3.5 系列】

- 【Claude3.5】part1-Claude介绍及注册流程与API领取.pdf
- 【Claude3.5】part2-API从入门到精通.zip
- 【Claude3.5】part3_tool_use工具调用全流程与2参数详解.ipynb
- 【Claude3.5】part4_prompt cache 提示缓存与streaming流式输出.ipynb

#### 【DSPy】

- 【DSPy】part1_核心理念DSPy的核心理念（配置LM、签名、构建模块）.ipynb
- 【DSPy】part2_DSPy的评估体系（Example数据、Metric度量指标、评估）.ipynb
- 【DsPy】part3_实战优化_GSM8K数据集下模块的表现(优化篇).ipynb
- 【DsPy】part4_推文生成与优化实战.ipynb

#### 【Gemini 系列】

- 【Gemini】part1_Gemini生态之最新模型、免费策略、注册指南、网页端微调实战一网打尽.md
- 【Gemini】part1_甄嬛Gemini微调语料500条.xlsx
- 【Gemini】part2_API文本生成调用入门到构建多轮对话机器人.ipynb
- 【Gemini】part3_API精细化控制生成参数与长文档实战与生成结构化.ipynb

#### 【GLM 系列】

- 【GLM】part0_智谱清言GLM账号注册及APIKEY领取.md
- 【GLM】part1_GLM-4文本对话大模型API调用指南.ipynb
- 【GLM】part2_Streaming流式响应调用API全解.ipynb
- 【GLM】part3_GLM_函数调用讲解与实战.ipynb
- 【GLM】part4_智谱CogVideoX 文生视频.ipynb

#### 【OpenAI GPT 4o 系列】

- 【GPT 4o】part1_文本生成API调用入门与精细化控制.ipynb
- 【GPT 4o】part2_文本生成API的多模态调用.zip
- 【GPT 4o】part3_tool_use工具调用与实战.ipynb
- 【GPT 4o】part4_生成结构化输出.ipynb
- 【GPT 4o】part5_首token时延与提示缓存.ipynb
- 【GPT 4o】part6_Streaming流式响应调用API全解.ipynb
- 【GPT 4o】part7_微调实战初窥之写作风格.zip
- 【GPT 4o】part8_微调实战进阶之tool_use微调.zip

#### 【OpenAI GPT o1 系列】

- 【GPT o1】part1_o1大模型与推理token简介.md
- 【GPT o1】part2_o1的API调用与总结.ipynb
- 【GPT o1】part3_通过COT制作4o-mini版本的o1模型与其局限性.ipynb
- 【GPT o1】part4_使用o1模型生成诺贝尔奖得主的JSON格式数据分析.zip
- 【GPT o1】part5_o1蒸馏4o-mini模型实战.zip

#### 【OpenAI 账号注册与环境配置】

- 【GPT】Part 1. OpenAI的账号注册速通.md
- 【GPT】Part 2. OpenAI官网及开发者平台使用指南.md
- 【GPT】Part 3. 开通虚拟信用卡与Plus会员订阅与API账户的充值与领取API_KEY.md
- 【GPT】Part 4. 在Jupyter中首次调用OpenAI API.ipynb

#### 提示工程

- 【提示工程】part1_提示工程指南(上).md
- 【提示工程】part2_提示工程指南(下).ipynb
- 使用 FastAPI 构建后端的 API 服务.pdf

### 大模型微调-课件

- CH 1 论文.zip
- Ch 1. 大模型微调.ipynb
- Ch 2. LLama_Factory+LORA大模型微调.ipynb
- Ch 3. LLama_factory&Lora微调Qwen2.5做NL2sql.ipynb
- Ch 4. Window微调Qlora.ipynb
- Ch 5. PPO微调LLama-3.2.ipynb
- Ch 6. RHLF微调Qwen2.5.ipynb
- Ch 7. DPO微调Qwen2.5.ipynb
- Ch 8 PEFT&医疗领域模型微调实践.ipynb

### 开源大模型部署与调用-课件

- 02_【GLM4-9B-Chat】GLM4-9b部署&vLLM推理.ipynb
- 03_【Llama3.1】Llama 3.1介绍与部署流程.ipynb
- 04_【Llama3.2】Llama3.2介绍与部署流程.ipynb
- 05_【Qwen2.5】Qwen2.5介绍与部署流程.ipynb
- 06_【Qwen2.5】Qwen2.5-Coder&Math介绍部署.ipynb
- 08_【Ollama】最新版本从入门到进阶攻略.ipynb
- 09_【Marco-01】推理大模型参数介绍与本地部署.ipynb
- 10_【GLM-Edge-V】端侧多模态模型参数介绍与部署流程.ipynb
- 11_【Qwen2VL】多模态视觉识别模型参数介绍与部署.ipynb
- 12_【Llama.cpp】开源推理框架量化模型使用指南.ipynb
- 13_【GLM4-Voice】语音模型参数介绍与部署.ipynb
- 14_【Cursor】以调用QWQ推理大模型为例实现辅助编程.ipynb

#### 01_ChatGLM3-6B

- 【ChatGLM3-6B】Part1_Ubuntu双系统安装教程.ipynb
- 【ChatGLM3-6B】Part2_Ubuntu环境ChatGLM3-6B安装办法.ipynb
- 【ChatGLM3-6B】Part3.1_服务器场景下ChatGLM3-6B部署与多种启动方式.ipynb
- 【ChatGLM3-6B】Part3.2_在AutoDL中快速部署ChatGLM3-6B模型.pdf
- 【ChatGLM3-6B】Part4_以ChatGLM3为例实现本地大模型更新.ipynb
- 【ChatGLM3-6B】Part5_单机多卡启动大模型办法.ipynb
- 【ChatGLM3-6B】Part6_LORA微调原理&实战.ipynb
- 【ChatGLM3-6B】Part7_PEFT主流高效微调方法介绍&实战.ipynb
- 【ChatGLM3-6B】Part8_Langchain体系详解与v0.3安装部署.ipynb

以下是图片中列出的各项内容整理成的 Markdown 表格：

---

以上内容根据图片信息整理而成，涵盖了图片中全部涉及的工具和服务。

https://flowithai.feishu.cn/docx/I5J6dQZt9opp2Rxhdi2c0JnrnKc

[[langmanus]]

---

1. 不一定要从ota的视角出发，而是从AI本身出发，然后再带入ota的框架可能更好，就像互联网对旅行社的冲击一样。AI的本质是信息的流动速度会加剧，可能无网络时代-网络时代-到微软GUI-到现在AI
	1. 异地出行会面对信息爆炸，横向对比产品，票-酒-poi-饮食的联动是刚需（票酒在前是我们的优势）
	2. 过去移动端主流，屏占比太小，结构化的呈现方式导致一些功能被深埋了，比如去哪儿里可以打车，但是以后在大chat交互上都会被打平。会随着chat的进度自然涌现出满足需求的功能。每个APP有自己主打的服务，会把主打的服务放到注意力最集中的地方。
2. 所以大chat呼之欲出，竞品也都有了，然后对于大chat的交互，当务之急都不是创新，而是把目前所有的app能提供的tool整合好，比如携程目前的bug。
3. 下一步就是运营问题了，这不只是一个技术问题，运营也很重要。运营和细分的策略，就像小红书一样，能post东西的平台很多，为啥它能最后火起来 而不是单纯的技术问题
	1. 对于旅游领域的信息如果给以更重的运营，可能是携程原有生态位向上突破的渠道，票+酒是所有出行中的大头，其他一切的场景都可以由这个大头展开，比如去一个地方最先打开的肯定不是这个地方的外卖，而是这个地方的机票和酒店，这在大 chat的场景下能做的事情很多

---

1. 从ota具体到q的话, 如果q继续用追Cbeat的逻辑，生态位稳固，那么体验lose其实不会有大的影响，因为会被作为比价工具，只是作为成交前交易辅助工具的地位一定会进一步下降
2. 具体到c的话，对c来说就非常关键了

---

MCP 生态和以太坊，瘦LLM，供应链会进一步渗透到社会的各个角落，比互联网的触角更深更广，像毛细血管一样

---

Agent

- 从简单排序，到让雷军给我造个汽车品牌然后给我
- Long horizon task planning + [[多模态]] + [Function Call](Function%20Call.md)  

---

Agent = Long horizon task planning + [[多模态]] + [Function Call](Function%20Call.md)  
Agent 的爆火会导致推理阶段的 token 消耗上涨一百倍都不止，因为多轮+推理的高时长

人类目前的 Agent 在哪里？在微信里, 在飞书里。比如代购、闲鱼找人帮忙订酒店之后加微信、比如给下属一个任务，去完成一根柱子。本质是可以把复杂度通过一个人给封装好。

人类目前的 Agent 是怎么完成任务的？假设给他们装个监控的话能拍到什么？

- 最简单的任务，帮我排序一份文档，可能微信中某个人帮你用 python 或者 excel 处理一下。
- 中等的，帮我写个背单词APP，他可能会翻翻 github，打开 IDE，从 UI、前端、后端、算法一路搞下去。
- 最复杂的，加了雷军的微信，他是我的 Agent，让他帮我造一辆车，打造一个品牌。监控里的他发挥了管理才能、撬动软硬件资源、整合供应链，耗时几年，把小米公司交付给你。

链路拆解一下：

人 - 软件 - 人 - 人- 软件 - 供应链

Agent 的发展面临两个坎：

1. 穿透所有软件层，意味着类似 MCP 这样的生态，打通目前各个孤岛的接口、数据。可能不融入就会死掉。
2. 穿透所有供应链层，本质上美团已经穿透到供应链层了，他们面向骑手、商家的调度就是一种控制。我派人去点一杯奶茶，他点点 APP 就可以调度供应链了。当然还有一些很小规模的供应链没有整合到线上（过去可能体量太小不足以平台化）

顺着这个逻辑，其实对于人类社会的增量有两点：

1. tools, 第一点小的就是人类社会的各种生产力彻底的线上化、接口化（其实本质逻辑就是过去二十年互联网发展的衍生），这部分其实增量有限，只是触角伸的更长而已。另外人在完成任务的时候，人脑的内存是很小的，但是 LLM 很大，这意味着过去普通的 GUI 的交互是满足不了AI Agent 的需求的，需要更开放的 GUI，MCP
2. planning 的平权。举两个极端点的例子，在大清，可能只有小企业主有一个能做排序任务的 Agent，但是如今几乎所有会用大模型的普通人都已经有了这样的 Agent。在现在可能还没人能有雷军这样的 Agent，直接帮你造一个品牌出来，但是未来的未来，可能没那么难了。本质是大模型能力如果能让 planning 的每一步都足够精确，搭配 1 中的 tool 实现比较高的交付质量，那么会有更多人的创造力、聪明才智被这种杠杆落地，从而用更快的速度改造整个社会的方方面面，这是第二部分增量的本质来源。

终局可能是一个类似微信一样的东西，里面有通用 Agent，也有一堆更精的 Agent，或者直接一个分发路由的秘书。

OTA 目前的生态位是提供酒店供应链的软件服务，获取佣金。（会随着酒店集团化降低议价权，参考中国目前 40%酒店集团化，美国 70%）。不管 Agent 发展成啥样，中间的软件服务是肯定需要的，但是空间会面临来自供应链的集团化的自有软件的挤压，还有来自入口被流量把持的挤压。

未来的终局就是没有app store 只有微信小程序的agent版本入口。但是都在微信还是一部分在微信一部分在飞书就不好说了

___

打败一个东西的 不是按照这个东西价值体系认为牛的东西 比如优酷视频 那会儿他们的危机可能来自于某个ip某个影视没签下来 根本想不到抖音这种UGC的东西会席卷他们 UGC视频为啥会爆发 是因为拍摄平权了

可以想一下自己雇了一个活人Agent，给他布置一个任务，想象给他装一个监控，他会怎么利用世界上的各种资源去实现？监控里拍的内容就是 AI Agent 需要做的，复杂度的封装。

首先agent是啥意思 比如北美的房产经纪人。本质是能屏蔽细节 把很多函数接口收束为一个。那么就剩两件事了 决策和做动作。

就像我点外卖一样，希望的是点对点的服务，脑子里有个想吃啥的想法，然后门口的位置就出现一份餐食。

决策的增量在点一杯咖啡。到。渴了。之间强化的决策空间。这个能力能达到什么程度 就是agent的想象空间。这个地方一定需要积累用户历史数据做分析

做动作的话是一个协议性质的问题。functioncall是我们从大模型拿到参数后自己实现。但是MCP出现试图一统江湖。主流信息世界 github这种纯信息的接入快一点 信息系统加供应链的可能慢一点 比如瑞幸 我们。

agent的商业价值 在于能缩短用户意图 到信息世界 到供应链世界的链路 比如我有一个意图 卖房子 到银行卡里多一笔钱 中间的整个链条就是房产agent的空间

让我纠正一下，您希望次数列反向排序，但数量列保持与原始次数的对应关系不变。

| 次数    | 数量      |
| ----- | ------- |
| 20次以上 | 515,316 |
| 20次   | 18,270  |
| 19次   | 17,954  |
| 18次   | 18,824  |
| 17次   | 19,620  |
| 16次   | 20,468  |
| 15次   | 20,302  |
| 14次   | 22,267  |
| 13次   | 29,758  |
| 12次   | 25,292  |
| 11次   | 27,293  |
| 10次   | 34,620  |
| 9次    | 30,082  |
| 8次    | 33,686  |
| 7次    | 43,901  |
| 6次    | 51,819  |
| 5次    | 102,951 |
| 4次    | 83,063  |
| 3次    | 92,570  |
| 2次    | 20,474  |
| 1次    | 5,817   |

#第一性原理  
聪明你的旅行

1. r1 展现出的逻辑能力是合理的。  q 有小红书这种内容平台没有的价格和同质化数据。两个结合起来就是我们的优势 zeng
2. 为什么要engineer推这个事 因为ai应用是从IDE爆发的  作为搞技术的人 对模型的能力边界也更理解
3. 住对面全季和住橘子其实是没区别的对用户来说 但是对我们来说佣金是有区别的

值班 Agent：

借这个机会把各个组的值班方法论串起里更好

文档：

https://hf7l9aiqzx.feishu.cn/docx/Y4mEdaLMCop8b4xnW6zc3YvvnBd?from=from_copylink

# 一、对 AI 的认知

1. 大模型的优势：
    
    1. 非结构化问题解决
        
        1. 以往需要人工理解和处理的非结构化流程，如自然语言指令，现通过 AI 模型能够直接调用相应函数，实现自动化操作。
            
    2. 结构化任务提效
        
        1. 针对那些虽然可以通过编程实现但成本较高的结构化任务，AI 交互可以大幅降低实现难度与成本，使得简单的对话就能完成原先复杂的代码编写工作。
            
    3. 思考力资源、注意力资源
        
        1. 只需依赖电力，AI 就能完成大量过去依赖人脑集中注意力才能实现的认知工作，从而实现人力资源的合理分配和优化。
            
    

2. 什么是 Agent？
    
    1. 交互友好，体验像人类助理一样
        
    2. 利用大模型三种优势，能在特定的场景下，完成有一定复杂度的任务。

# 对 AI 的认知

AI 的核心能力是：

1. 解决过去的一些非结构化的过程，比如随便说一段自然语言就能调用对应的函数
2. 解决过去可以结构化，但是写代码成本比较高的工作，现在简单的 AI 交互就可以实现了
3. 注意力资源，只需要电力就可以完成一定的过去需要人脑的认知工作

# 场景挖掘

思路：  
	 1. 用户在使用软件的过程中，有哪些脏活累活（无论是结构化的还是非结构化的）？  
		 1. 同 sku 跨平台比价，打开去哪儿、再打开美团，找到同一个 sku 去比价  
		 2. 同平台内跨酒店的相似类型的 sku 的比较，比如去沈阳，在 L 页找到五家酒店，然后横向对比一些产品  
	 2. 开发人员在值班查问题的时候有很多脏活累活（结构化、非结构化都有）  
		 1. 调很多接口，看很多数据  
		 2. 很多同类型的问题反复出现，要走同样的流程，但是专门写一套代码又不太值当  
	 3. 哪些地方需要更低成本的 70 分的人脑？对应 1.3  
		 1. 一些核心运营指标可能值得更多注意力资源的投入，比如产品力的一些指标

| 场景                 | 实现难度   | 优势                 | 劣势                  | 可能收益                                                               | 可能风险 |
| ------------------ | ------ | ------------------ | ------------------- | ------------------------------------------------------------------ | ---- |
| 1. 购物车比价比质         | 一般     |                    | 需要先等类似 C 的跨酒店的购物车上线 | 1. 需要携程购物车使用率数据，如果入口流量大，那么这个 Agent 应该有一定收益<br>2. 作为 Agent 功能获取用户好感 |      |
| 2. 整个酒店的值班 Agent   | 一般     | 每个组之前的值班已有很多观测性的接口 |                     | 1. 人效<br>2. 解放时间专注更复杂的疑难杂症                                         |      |
| 3. 酒店业务知识库 + PMO   | 简单<br> | 前期已经有大量文档沉淀，做遴选即可  |                     | 1. 未来的业务分享，同样的时间投入能有更好的沉淀<br>2. 新人学习                               |      |
| 4. 核心业务指标的运营 Agent | 较困难    | 产品力基建比较成熟          | 推理模型的认知能力有待提升       | 1. 发现新的业务机会                                                        |      |

# 示意图

https://app.eraser.io/workspace/nnosGsLNbXMRLqJQ3rcB
