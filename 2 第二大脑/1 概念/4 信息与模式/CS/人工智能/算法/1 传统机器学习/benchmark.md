---
date created: 2025-04-26
date modified: 2025-10-19
uid: 58cc2b72-9725-45c2-ad67-de83b74f732b
---
GLUE/SuperGLUE/SQuAD/XSum/XTREME/[[MMLU]]

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F26%2F14-50-18-7bc7850909a72ce492b0f19e3bdaa66b-202504261450301-a24328.png)

[[MTEB]]

BUA(浏览器使用智能体): 在Mind2Web榜单上，击败OpenAI、Anthropic等巨头，获得小模型第一
CUUA(计算机使用智能体): 在OS World榜单上，7B小模型超越Kimi等大模型，获得专项榜第一


| Benchmark              | 测试领域      | 测试内容                                                          | 当前领先者                             | 官方链接                                                    |
| ---------------------- | --------- | ------------------------------------------------------------- | --------------------------------- | ------------------------------------------------------- |
| **OSWorld**            | 桌面操作（全系统） | 369个跨应用任务，操作真实OS环境（Ubuntu/Windows/macOS）及本地软件（VS Code, GIMP等） | claude-sonnet-4-5                 | [os-world.github.io](https://os-world.github.io/)       |
| **WebVoyager**         | 浏览器操作     | 真实网站端到端任务（Amazon, Google Flights, arXiv等）                     | Agent框架 > 裸模型                     | [leaderboard.steel.dev](https://leaderboard.steel.dev/) |
| **SWE-bench Verified** | 代码实战      | 写代码+改文件+跑终端，经人工验证                                             | Claude 4.5 Opus medium (20251101) | [swebench.com](https://www.swebench.com/)               |

---

**快速选择指南：**

|你的需求|看这个榜单|
|---|---|
|评估模型操作复杂界面的底层能力|OSWorld|
|找最好用的浏览器自动化框架/工具|WebVoyager|
|评估 AI 写代码、改 Bug 的能力|SWE-bench|