---
comment_id: dfb720e7
date created: 2025-03-24
date modified: 2025-07-04
draw: null
title: Unsloth
---
Unsloth 是一个开源工具，专门用来加速大语言模型（LLMs）的微调过程。https://github.com/unslothai/unsloth

- 高效微调：Unsloth 的微调速度比传统方法快 2-5 倍，内存占用减少 50%-80%。这意味着你可以用更少的资源完成微调任务。
    
- 低显存需求：即使是消费级 GPU（如 RTX 3090），也能轻松运行 Unsloth。例如，仅需 7GB 显存就可以训练 1.5B 参数的模型。
    
- 支持多种模型和量化：Unsloth 支持 Llama、Mistral、Phi、Gemma 等主流模型，并且通过动态 4-bit 量化技术，显著降低显存占用，同时几乎不损失模型精度。
    
- 开源与免费：Unsloth 提供免费的 Colab Notebook，用户只需添加数据集并运行代码即可完成微调。
