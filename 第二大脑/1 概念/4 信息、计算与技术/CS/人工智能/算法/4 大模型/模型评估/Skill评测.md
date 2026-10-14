---
uid: b297cf93-cc92-4c15-aa37-7aac798a9a82
date created: 2026-09-30
date modified: 2026-09-30
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 



# 🧩 Cues



# 🪞Notes

OpenAI发布了一套方法论，主要是关于如何评测skill效果的。对于做skill的人还是值得一看，而且github也找到一个理念很相近的skill评测器：agent-skills-eval，不过是个人维护的，非官方。 先定义成功，再写 skill。OpenAI把检查项分为了这四个点： Outcome（任务完成没）、Process（调了预期的 skill/步骤没）、Style（产物符合团队规范没）、Efficiency（有没有瞎折腾/烧 token）。 没这个定义，skill 就只是个更长的 prompt；有了它，才能进版本管理 + 回归测试 + CI。 负样本比正样本更关键。博客里 test-04：用户只是想给现有项目"加 Tailwind"，skill 却新建了一整个 demo app。这种误触发比不触发更危险——因为它会动你的工作区。这是 agent 产品里最容易被忽略的验收口径。 Trace 是第一性证据。用 codex exec --json 抓 JSONL 事件流，确定性检查（跑没跑 npm install？package.json 生成没？）直接读事件，快、可解释、能进 CI；模型裁判（rubric grading）只做第二层"质量"补充，不替代规则。 小样本（10–20 条）起步：显式触发 / 隐式触发（不提 skill 名，靠 description 命中）/ 上下文触发 / 负向控制，各放几条。 它的主张我认为非常好：Agent 开发正从 prompt craft 走向 behavior engineering——skill 不该只是写给模型看的说明书，而该是可测、可评分、可回归的类似一个Agent的可复现测试的单元。