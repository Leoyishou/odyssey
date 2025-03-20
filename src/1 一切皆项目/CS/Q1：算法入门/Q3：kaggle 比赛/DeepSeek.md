---
draw:
title: DeepSeek
date created: 2025-02-23
date modified: 2025-03-13
---

Deepseek v3 和 llama-like 的模型有很大的区别，infra 层做了很多优化。但是 infra 厂商不一定会愿意

 llama-like 是 dense decoder，大部分显卡适配这种  
Deepseek 是 MOE，对硬件厂商来说有点冒险，在买模型彩票，让专用的芯片过度绑定于特定模型而不是通用模型的话

满血版需要 4台八卡h100

https://github.com/huggingface/open-r1

这个 **open-r1** 项目是 **DeepSeek-R1** 的一个完全开源的复现版本。其目标是**重现并构建 DeepSeek-R1 训练管线的缺失部分**，以便所有人都可以复现并在其基础上进行构建和优化。

[[SFT]]
