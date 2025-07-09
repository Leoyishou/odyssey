---
draw:
title: LLM大模型
date created: 2024-10-21
date modified: 2025-07-03
---

#最佳实践   大语言模型系统化学习只需要看斯坦福三节课就够了：CS25: 通识类的整体介绍 CS224N: NLP+深度神经网络+Transformer CS336: 大语言模型从头实现，尤其是会介绍最新的进展（比如为什么现在的LLM都用SwiGLU）如果深度了解机器学习基础，可上CS229机器学习以及CS230深度学习。以上所有课程均Online+有视频+随业界进展实时更新。

想学习大语言模型(LLM)，应该从哪个开源模型开始？- 三风的回答 - 知乎

https://www.zhihu.com/question/608820310/answer/3267334734

大语言模型所有算子逻辑 - 骑虎南下的文章 - 知乎

https://zhuanlan.zhihu.com/p/1909996866432668841

| 侧重点  | 技术/方法                                | 简要目标                                   | 常见实现要点                                 | 类比说明                 |
| ---- | ------------------------------------ | -------------------------------------- | -------------------------------------- | -------------------- |
| 任务适配 | [[微调]] (Supervised Fine-Tuning, SFT) | 在小规模高质量的标注数据上继续监督学习，使 LLM 聚焦特定任务/领域/指令 | 重新训练全部或部分模型权重；数据通常为输入-输出对              | 司机拿到驾照后再上城市驾驶/卡车驾驶进修 |
| 人类对齐 | RLHF (基于人类反馈的强化学习)                   | 让模型输出符合人类偏好、价值观与安全准则                   | 标注员对多个候选答案排序 → 训练奖励模型 → 用 PPO 等算法优化主模型 | 教练给出评分，奖励安全/文明驾驶     |
|      | DPO (直接偏好优化)                         | 不训练单独奖励模型，直接用人类排序信息更新 LLM              | 把“好 vs. 差”配对当作目标分布，最小化 KL-divergence   | 教练实时指出更好做法，直接修正      |
|      | GRPO                                 |                                        |                                        |                      |
| 推理增强 |                                      | 提升多步逻辑推理、分步解题与因果分析能力                   | 采用带链式思考（CoT）标注或多步骤解答数据集进行再训练           | 练习长途路线规划与危险预判        |
| 效率优化 | 量化                                   | 降低权重精度，减小模型体积、提升推理速度                   | FP32→INT8/INT4；对称或非对称量化                | 给汽车轻量化轮胎、部件          |
|      | 修剪                                   | 删除不重要连接，压缩计算                           | 结构化/非结构化剪枝；迭代稀疏化                       | 去掉用不到的零件             |
|      | 知识蒸馏                                 | 教大模型知识给小模型，保持效果同时减小规模                  | Teacher-Student 训练；软标签/中间特征对齐          | 大师傅带徒弟               |
|      | 参数高效微调 (PEFT，如 LoRA)                 | 只训练少量新增/重参数层，以极低成本适配场景                 | 插入 LoRA/IA3 等小矩阵；冻结原权重                 | 给发动机加外挂调校而不拆主机       |

下面给出一条 “概念树最短路径”，只保留真正“断不开”且彼此递进的核心节点。沿着这条主干，你能在最短时间里形成对 LLM 技术的整体理解；之后有兴趣再向两侧分枝补充细节即可。

```Java
多层感知机 (MLP)
  └─ 反向传播 (Back‑prop)                —— 让神经网络真正可训练
      └─ 序列建模需求 (自然语言/时间序列)
          └─ 循环神经网络 RNN           —— 把状态带进下一步
              └─ 长依赖难题 (梯度消失/爆炸)
                  └─ LSTM / GRU        —— 门控缓解长依赖
                      └─ 注意力机制 (2014‑2016)
                          └─ Transformer (2017)
                              ├─ 自监督语言建模目标 (Next‑Token / MLM)
                              │    └─ 预训练 + 微调范式
                              └─ 大规模并行化 & Scaling Laws
                                  └─ GPT‑类 LLM
                                      ├─ 对齐技术 (RLHF / RLAIF / 指令微调)
                                      └─ 工具调用 & Agent 化
```

## 为什么这条路径才算“最短”？

#Roadmap

| 节点                  | 必不可少的独特贡献     | 如果跳过会发生什么？|                                       |
| ------------------- | ------------- | ---------------------- | ------------------------------------- |
| MLP + Back‑prop | 神经网络与可微分优化的基础 | 不知道梯度训练为何可行            | 这是神经网络的基础，引入了非线性激活函数和反向传播算法           |
| RNN → LSTM/GRU  | 第一次解决「序列依赖」问题 | 很难理解 Transformer 诞生动机  | 解决了序列数据处理和长期依赖问题                      |
| 注意力机制           | 把“对齐”信息显式建模   | 无法解释 Transformer 的核心优势 |                                       |
| Transformer     | 网络拓扑彻底去循环，可并行 | 不明白大模型为什么能快速训练         | 完全基于自注意力机制，去除了RNN的顺序依赖性，实现并行计算        |
| 自监督预训练          | 海量无标注文本→泛化能力  | 不知道大模型为何需要超大语料         | - 从ELMo到BERT(编码器)和GPT(解码器)的发展，引入自监督学习 |
| Scaling Laws    | “更大=更好”经验规律   | 不理解参数/数据/算力扩张的价值       |                                       |
| 对齐技术            | 从“能说”到“说得合人意” | 难以解释 ChatGPT 等系统效果     |                                       |
| RLHF(基于人类反馈的强化学习)   |               |                        |                                       |
| 工具调用/Agent      | LLM 向通用智能的延伸  | 无法展望未来应用形态             |                                       |

> 你的原路径 MLP → Transformer → LLM 省略了  
> - 序列建模阶段（RNN/LSTM）：Transformer 的创新点正是弥补 RNN 在长依赖上的缺陷；知らずにだと“为什么 transformer 重要”就成了空洞口号。
> - 自监督预训练 + Scaling：没有这一步，Transformer 只是一个“结构”，而不是“会说话的大模型”。
> - 对齐层：决定了模型从“技术 demo”变成“可交互产品”的最后一公里。

## 学习建议（沿主干快速过一遍即可）

1. 30 分钟 读懂 MLP + 反向传播（任何深度学习教材第一章）。
2. 1 小时 浏览 RNN ➜ LSTM 的梯度消失故事，理解“记忆门”。
3. 2 小时 仔细研读 Attention Is All You Need，只抓三个公式：Scaled Dot‑Product Attention、Multi‑Head Attention、Position Encoding。
4. 1 小时 看两篇综述：BERT（双向 MLM）和 GPT‑3（纯右向 LM + Scale）。
5. 1 小时 扫一眼 OpenAI/DeepMind/Anthropic 的 RLHF/RLAIF 过程示意图，知道“人类反馈”插在哪。
6. 1 小时 玩一次 LangChain / LangGraph 或 OpenAI Functions，感受 LLM 调用外部工具的范式。

合计 ~6 小时 即可完成“概念树主干”速通，后续再按兴趣深入各子枝（多模态、MoE、检索增强、可解释性等）。这样既避免信息洪流的迷失，又保证技术脉络清晰连贯。祝学习顺利!

1.关键词是什么 2.how to send the context 是prompt或者 AI 时代的最关键命题

technical probpem是最容易解决的

business conrext 过于个性化甚至是不可言说之物。

| 周次     | 内容主题 | 学习时长 | 具体内容                                                                                                      |
| ------ | ---- | ---- | --------------------------------------------------------------------------------------------------------- |
| ❤️ 第一周 | 基本概念 | 4小时  | 环境搭建（[[Python]]/[[PyTorch]]）、[[Transformer]]架构简介、[[自注意力机制]]机制、基础文本生成（Hello World示例）|
| ❤️ 第二周 | 核心组件 | 5小时  | [词向量、词嵌入](词向量、词嵌入.md)嵌入（Word Embeddings）、位置编码（Positional Encoding）、多头注意力（Multi-Head Attention）、前馈网络（Feed-Forward）|
| ❤️ 第三周 | 实践入门 | 5小时  | 实现简单Transformer模块、微调预训练模型（如BERT）、文本分类任务（基于Hugging Face库）|
| ❤️ 第四周 | 进阶应用 | 6小时  | 生成任务（GPT文本生成）、模型压缩技术（知识蒸馏）、Prompt Engineering、模型评估（BLEU/ROUGE指标）|
| ❤️ 第五周 | 高级主题 | 18小时 | 预训练与微调策略（ZeRO/DeepSpeed）、模型伦理与偏见、多模态大模型（CLIP/DALL·E）、模型部署（ONNX/TensorRT）|
| ❤️ 第六周 | 前沿探索 | 38小时 | 大模型优化（MoE架构）、强化学习对齐（RLHF）、Agent开发（AutoGPT）|

  大模型必看博主/机构

  2. 👨🏻‍💻 李沐（《动手学深度学习》作者）
  3. 👨🏻‍💻 Andrej Karpathy（OpenAI研究员，技术解读深入）
  4. 👨🏻‍💻 Hugging Face团队（开源模型库与教程

  
  5. 日常自学神仙网站
  6. 🚀 新手：Hugging Face Courses（免费实战教程）
  7. 🚀 进阶：Papers With Code（论文+代码复现）
  8. 🚀 刷题：LeetCode（算法与模型优化题）
  9. 🚀 项目：GitHub（开源大模型项目库）
  
  
  10. 📚 学习物料
  11. 🛠️ 工具包推荐 PyTorch/TensorFlow
  12. 安装包 Jupyter Notebook
  13. 环境配置 Hugging Face Transformers 库
  14.
  15.
  16.
  17. 📖 经典书籍与论文《Attention Is All You Need》（Transformer原论文）《深度学习》（花书，理论基石）《Natural Language Processing with Transformers》（Hugging Face实战指南）📝 笔记与课程《李宏毅深度学习笔记》（大模型章节）《斯坦福CS224n（NLP公开课）》《动手学大模型》系列
  18.
  
  19. 🌱 附加资源 Kaggle大模型竞赛案例 OpenAI官方技术博客 ArXiv最新论文跟踪 [#大模型](https://www.xiaohongshu.com/search_result?keyword=%25E5%25A4%25A7%25E6%25A8%25A1%25E5%259E%258B&type=54&source=web_note_detail_r10) [#人工智能](https://www.xiaohongshu.com/search_result?keyword=%25E4%25BA%25BA%25E5%25B7%25A5%25E6%2599%25BA%25E8%2583%25BD&type=54&source=web_note_detail_r10) [#](https://www.xiaohongshu.com/search_result?keyword=AI%25E5%25B7%25A5%25E5%2585%25B7&type=54&source=web_note_detail_r10)

## 历史

以下是从过去到现在，按时间顺序梳理的LLM（大型语言模型）关键技术发展节点：

- 2012年：AlexNet赢得ImageNet比赛，开启深度学习热潮。
- 2013年：Google提出Word2Vec，推动[自然语言处理 NLP](自然语言处理%20NLP.md)进入[[深度学习]]时代。
- 2014年：Seq2Seq与Attention机制问世，显著提升机器翻译能力。
- 2017年6月：Transformer架构发表（论文《Attention Is All You Need》），NLP进入Transformer时代。
- 2018年6月：OpenAI推出GPT-1，开启预训练生成模型时代。
- 2018年10月：Google提出BERT，建立双向预训练新标准。
- 2019年6月：Meta推出RoBERTa，优化BERT架构，推动预训练模型研究深入。
- 2020年5月：OpenAI发布GPT-3（1750亿参数），展示大规模模型惊人的语言生成能力。
- 2022年11月：OpenAI发布ChatGPT，推动LLM首次大众化，引发全球关注。
- 2023年至今：GPT-4及各类LLM广泛普及与产业落地，标志着大型语言模型技术进入产业化新时代。

## 两个问题

### 大模型幻觉

  " 大模型幻觉 " 是指人们在与大语言模型交互时可能产生的一种错觉，即错误地认为这些模型具有真正的理解能力、意识或感知。这种幻觉通常源于模型生成的文本质量非常高，能够流畅、连贯地回答问题或进行对话，使人们感觉就像是在与一个真实的、有理解力的人类交谈。

  举一个论坛上的例子，怎么写出更好的 prompt？回答 A 介绍了一堆循序渐进的步骤，但是高赞的回答 B 说到，这一切的前提首先是它是个 prompt 专家！那么它是吗？

  

### 提示词工程有必要吗？

应用开发中是有用的！以 json 输出为例  
个人应用中性价比不高，另外对于一个黑箱，大量场景无法测试收益，设置为 template 嵌入到工具中更高效，核心是人设 + 结果约束

- 问题中加上一句我是十岁小孩
- 问题中加上一句，不确定的不要瞎编

## 两类场景

### AI 决策

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F21%2F14-37-37-fa295975d703324c629da687b0903cef-202410211437508-d16c73.png)

 在旅行规划场景，这是大模型幻觉吗？这个决策过程是科学有效的吗？用户真的去玩了七天后，回来会满意吗？这其实是一个未验证的能力，也可能是未来 AI 发展的魅力所在，因为我们人脑可能也没有那么强的逻辑，解决一个问题时似乎也是产生一个模糊的似是而非的经验性的解决步骤。

 

### 英语对话场景

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F21%2F14-37-55-78121c9177f8221dfbda8620fc7a9768-202410211437113-a2a447.png)

- 这个工具能够奏效的逻辑起点是：对于中文，gpt 只会说没道理的话，但是不会产生不通顺的话，而通顺正是这个场景下用户的使用目的。
- 只是复盘的工具，解决不了沟通的欲望。市面上一些其他的口语 AI 应用，无法让人产生持久的交流欲望  
	是人类的工具，但是无法提供情感支持

## 两种工具

	  有两类工具，第一种是拿来就可以解决问题，实现目标的，比如战场上的一把枪，可以对赤手空拳的敌人降维打击；第二种是拿来可以提高我们自己解决问题的能力的，比如一个哑铃，我们可以通过它练习自己的力量，从而战胜力量不如我们的敌人。

### 第一类工具

- 基本都是围绕着它的两个核心能力展开的，1: 语义理解 2: 一定能生成一个回答
- 检查错别字
- 降重改写
- 简单的信息检索
	- 1. 语义理解
	- 2. 适合个人对于一个新领域的冷启动，适合没有官方文档，或者官方文档不够完整或者不够有趣的前提下，提供一定的词汇扩展和交互性。
	- 3. 但是对于更细分更专门化的知识更好的选择是找到对应的群体  
		

### 第二类工具

- 语言学习
- 对于一个新领域的冷启动，交互式初探

### 总结

- 当 AI 作为第一类工具时，一定会被各种公司通过商业化制作成简单易上手的产品，对于个人来说，无需焦虑，只需享受，真的好用的 AI 产品一定会快速出圈！
	- 生成 PPT、图片
- 当 AI 作为第二类工具时，更考验我们的自我规划，自我探索，热爱和自律，一个比较现实的切入点是，尽可能多得保留个人痕迹的数据。
	- 借助 AI 在新领域快速达到 70 分的水平，当这样的领域足够多时，会发生个人能力的『涌现』

## function call

是的,OpenAI确实支持function calling功能。这是一个强大的特性,允许模型与外部工具和系统进行交互。以下是关于OpenAI function calling的一些关键信息:

### 功能概述

Function calling允许您描述自定义函数或外部API给助手,使助手能够智能地调用这些函数,输出包含相关参数的JSON对象[

2

](https://help.openai.com/en/articles/9492280-function-calling-in-the-chat-playground)。

### 主要用途

Function calling适用于多种场景,包括:

1. 使助手能够获取数据,如从内部系统获取最新的客户数据。
2. 使助手能够执行操作,如根据用户偏好和日历可用性安排会议。
3. 使助手能够进行计算,如数学辅导助手执行数学计算。
4. 构建复杂的工作流,如数据提取管道。
5. 修改应用程序的UI,如根据用户输入在地图上渲染标记[
    

    3

    

](https://platform.openai.com/docs/guides/function-calling)。

### 实现方式

OpenAI提供了多种方式来实现function calling:

1. Chat Completions API: 这是最常用的方法,允许您在对话中使用function calling[
    

    3

    

](https://platform.openai.com/docs/guides/function-calling)。

1. Assistants API: 专门为构建AI助手设计的API,也支持function calling[
    

    4

    

](https://help.openai.com/en/articles/8555517-function-calling-in-the-openai-api)。

1. Batch API: 用于批量处理的API,同样支持function calling[
    

    3

    

](https://platform.openai.com/docs/guides/function-calling)。

### 特殊功能

1. 并行函数调用: 支持同时执行多个函数调用,提高效率[
    

    1

    

](https://learn.microsoft.com/pl-pl/azure/ai-services/openai/how-to/function-calling?tabs=non-streaming%2Cpython)[

    

    2

    

](https://help.openai.com/en/articles/9492280-function-calling-in-the-chat-playground)。

1. 结构化输出: 通过设置`strict: true`,确保模型生成的函数调用参数完全匹配您提供的JSON Schema[
    

    3

    

](https://platform.openai.com/docs/guides/function-calling)[

    

    4

    

](https://help.openai.com/en/articles/8555517-function-calling-in-the-openai-api)。

### 使用步骤

1. 选择您代码库中希望模型能够调用的函数。
2. 向模型描述您的函数,使其知道如何调用。
3. 调用Chat Completions API,包含您的函数和用户输入。
4. 使用模型的响应来调用您的API或函数。
5. 再次调用Chat Completions API,包含您函数的响应,以获得最终回答[
    

    3

    

](https://platform.openai.com/docs/guides/function-calling)。

### 注意事项

- 模型本身不会执行函数,它只会生成可用于调用函数的参数。您的应用程序始终保持完全控制[
    

    3

    

](https://platform.openai.com/docs/guides/function-calling)。

- 在Chat Playground中,您可以通过点击右侧配置面板中的"+ Add function"来添加新函数[
    

    2

    

](https://help.openai.com/en/articles/9492280-function-calling-in-the-chat-playground)。

总之,OpenAI的function calling是一个强大的功能,可以显著增强AI模型的能力,使其能够与外部系统和工具无缝集成,从而创建更智能、更实用的应用程序。
