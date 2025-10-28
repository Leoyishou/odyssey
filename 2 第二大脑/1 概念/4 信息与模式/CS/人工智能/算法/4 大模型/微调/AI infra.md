---
date created: 2025-04-03
date modified: 2025-10-20
uid: ab3a9b67-cd0e-4ac1-b641-d8e5e61f1dbb
---
# Summary


例如分布式框架，GPU/CPU资源分配，通信，cuda算子，训练引擎(deepspeed, fsdp, megatron)和推理引擎(vLLM, sglang,...) 的具体原理（3D混合并行 + Sequence Parallel）



# Cues

https://www.zhihu.com/question/4023337465/answer/1950623317519242011?share_code=yu8djYzPBs2c&utm_psn=1950696270743201007


- 核心任务：
    
    - 硬件支持：管理GPU集群、云服务器（像AWS、GCP），让模型训练有足够算力。
        
    - 软件环境：搭建框架（TensorFlow、PyTorch）和工具链，让开发者和模型跑得顺畅。
        
    - 数据管道：把非结构化数据（比如你的Obsidian笔记）变成模型能用的格式，传输到训练环境。
        
    - 扩展性：确保系统能支持从“小作坊”（个人项目）到“大工厂”（企业级AI）的规模。
        
- 举个例子：你用5000小时的番茄工作法记录想训练一个效率分析模型，AI Infra 就是负责把你的日志数据存好、分配一台GPU跑训练、保证代码不崩的那群“幕后英雄”。
    



___


这里我主要想答一下 Al Infra这一块，如果想做Al Infra相关可以参考。回答内容偏工程基础，没涉及论文之类的。对于 Al lnfra这一块的话，主要分为训练和推理，

由于RL今年特别火，今年不少厂会存在把RL Infra单独拆开来看。下面分3块说一下。

1. RL Infra：RL的3个阶段既要训练又要推理，训练复用训练的框架，如megatron、fsdp，推理服用推理的框架，如vllm和sglang，RL本身会额外多一些训练和推理参数同步的一些技术，以及 RL本身需要的一些优化如异步、rollout优化等，说了这么多，总结一下，算力不充足的时候，RL Infra 可能比较难搞，你要真的搞，那就各种offload+了。从训练开始就offload，到推理继续 offload，你一个step可能需要几天时间。
2. 训练Infra：训练的一个核心技术在于各种并行，另外显存优化、算子优化等也是一个方面，还是那句话，如果算力首先，比如只有1张卡，那你也只能offload 了。
3. 推理 Infra：这个能做的事情就多了，首先offload 可以继续研究、其次，[[模型量化]]技术＋也是推理可以研究的，但是目前量化也比较的卷，稀疏化+也是可以去研究的，比如长序列算力受限的场景下去研究稀疏化。如果纯属爱好，那完全可以去写一个推理框架，把attention、w4a8、w8a8这种算子也去实现一遍，先去做一个单击的推理引擎。你的算力少的话，还可以做很复杂的推理系统，去手写一个PD分离框架，P放在N卡上，D放在 CPU上跑。你还可以去做AFD分离，attention放在CPU上跑、MOE放在GPU上跑。你甚至还可以做一个并行的投机采样，小模型放在CPU上跑、大模型放在GPU上跑。你可以用流水线并行 + offload 在 8G显存的芯片上跑Deepseek 671B。
___

ai infra 相对核心的在训推框架，调度就有点远了 不是有个名人说，这块要深入，要么深入到cuda层面结合硬件做算子优化，要么深入模型结构解决实际问题 我简单说下训练框架需要知道的 预训练 并行策略，dp，tp，Pp，Sp，Cp等 overlap，计算通信overlap怎么实现的 flashattention transformer和megatron框架需要看 后训练 lora，ppo* •grpo+ *，dpot等 verl框架+需要看 算子方面 写个gemm*知道gpu怎么运行就差不多了，如果走算子方向再深入 另外就是论文，所有大模型的技术报告及引用的论文 另外推荐猛猿的文章，全部吃透
