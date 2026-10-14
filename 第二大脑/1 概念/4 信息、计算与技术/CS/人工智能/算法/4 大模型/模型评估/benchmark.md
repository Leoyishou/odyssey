---
date created: 2025-04-26
date modified: 2025-10-19
uid: 58cc2b72-9725-45c2-ad67-de83b74f732b
---
这是一个**“看到一堆 benchmark 名字，不知道各自在测什么”**的经典问题变体。

| Benchmark 名字         | 通俗解释                                                   |
| -------------------- | ------------------------------------------------------ |
| [[HLE]]              | HLE 本身是 2,500 道专家出的闭卷型题，覆盖数学、自然科学、医学、计算机、人文社科等 100+ 方向 |
| SWE-bench Pro        | 测模型像软件工程师一样修真实代码仓库里的复杂 bug，偏“高级编程实战”。                  |
| SWE-bench Verified   | 也是测修 bug，但题目筛得更严、更干净，结果通常更可靠，偏“标准版真实修 bug”。            |
| Terminal-Bench 2.0   | 测模型会不会在终端里干活，比如敲命令、查文件、跑程序、改配置，偏“命令行实操能力”。             |
| Humanity’s Last Exam | 测特别难、跨学科的问题，像“博士级综合大题”，看模型能不能做深度推理。                    |
| BrowseComp           | 测模型会不会自己上网搜资料、追线索、翻网页，最后把答案找出来，偏“代理式搜索能力”。             |
| MCP-Atlas            | 测模型在很多外部工具之间协同工作的能力，像“会不会调各种工具来完成任务”。                  |
| OSWorld-Verified     | 测模型会不会像人一样操作电脑界面，比如点按钮、填表、切窗口，偏“用电脑办事”。                |
| Finance Agent v1.1   | 测模型处理金融分析任务的能力，比如读数据、做判断、给分析结论。                        |
| CyberGym             | 测模型在网络安全场景里复现漏洞、理解攻击链或安全问题的能力。                         |
| GPQA Diamond         | 测非常难的研究生级科学问题，偏“高难知识推理”，很多题不是死记硬背能做出来的。                |
| CharXiv Reasoning    | 测模型看图、看图表、看视觉材料后进行推理的能力，偏“视觉理解 + 推理”。                  |
| MMLU                 | 测多学科知识问答能力，像一套覆盖很多科目的通识考试。                             |

你这张图里左侧那些大类，其实可以再压缩成一句话理解：

- **Coding 类**：会不会写代码、修代码、在终端里干活
    
- **Reasoning 类**：会不会做难题、跨学科推理
    
- **Search / Tool / Computer Use 类**：会不会自己找资料、调工具、操作电脑
    
- **Finance / Cybersecurity 类**：会不会处理专业领域任务
    
- **Visual / Multilingual 类**：会不会看图推理、会不会多语言答题
    

如果你愿意，我下一条可以把这个表再补一列：**“更像考学生，还是更像考打工能力”**。


GLUE/SuperGLUE/SQuAD/XSum/XTREME/[[MMLU]]

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F26%2F14-50-18-7bc7850909a72ce492b0f19e3bdaa66b-202504261450301-a24328.png)

[[MTEB]]

BUA(浏览器使用智能体): 在Mind2Web榜单上，击败OpenAI、Anthropic等巨头，获得小模型第一
CUUA(计算机使用智能体): 在OS World榜单上，7B小模型超越Kimi等大模型，获得专项榜第一


|           | Benchmark              | 测试领域 | 测试内容                                                          | 当前领先者                             | 官方链接                                                    |
| --------- | ---------------------- | ---- | ------------------------------------------------------------- | --------------------------------- | ------------------------------------------------------- |
| 桌面操作（全系统） | **OSWorld**            |      | 369个跨应用任务，操作真实OS环境（Ubuntu/Windows/macOS）及本地软件（VS Code, GIMP等） | claude-sonnet-4-5                 | [os-world.github.io](https://os-world.github.io/)       |
| 浏览器操作     | Mind2Web               |      | 更学术化，关注"AI 能不能理解网页结构并做出正确动作"                                  |                                   |                                                         |
|           | WebVoyager             |      | 真实网站端到端任务（Amazon, Google Flights, arXiv等）                     | Agent框架 > 裸模型                     | [leaderboard.steel.dev](https://leaderboard.steel.dev/) |
|           | **SWE-bench Verified** | 代码实战 | 写代码+改文件+跑终端，经人工验证                                             | Claude 4.5 Opus medium (20251101) | [swebench.com](https://www.swebench.com/)               |
|           |                        |      |                                                               |                                   |                                                         |

---

**快速选择指南：**

|你的需求|看这个榜单|
|---|---|
|评估模型操作复杂界面的底层能力|OSWorld|
|找最好用的浏览器自动化框架/工具|WebVoyager|
|评估 AI 写代码、改 Bug 的能力|SWE-bench|