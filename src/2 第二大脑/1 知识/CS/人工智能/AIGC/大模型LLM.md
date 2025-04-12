---
draw:
title: 大模型LLM
date created: 2024-10-21
date modified: 2025-04-05
---
1.关键词是什么 2.how to send the context 是prompt或者 AI 时代的最关键命题

technical probpem是最容易解决的

business conrext 过于个性化甚至是不可言说之物。

#Roadmap

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

- **2012年**：AlexNet赢得ImageNet比赛，开启深度学习热潮。
- **2013年**：Google提出Word2Vec，推动[自然语言处理 NLP](自然语言处理%20NLP.md)进入[[深度学习]]时代。
- **2014年**：Seq2Seq与Attention机制问世，显著提升机器翻译能力。
- **2017年6月**：Transformer架构发表（论文《Attention Is All You Need》），NLP进入Transformer时代。
- **2018年6月**：OpenAI推出GPT-1，开启预训练生成模型时代。
- **2018年10月**：Google提出BERT，建立双向预训练新标准。
- **2019年6月**：Meta推出RoBERTa，优化BERT架构，推动预训练模型研究深入。
- **2020年5月**：OpenAI发布GPT-3（1750亿参数），展示大规模模型惊人的语言生成能力。
- **2022年11月**：OpenAI发布ChatGPT，推动LLM首次大众化，引发全球关注。
- **2023年至今**：GPT-4及各类LLM广泛普及与产业落地，标志着大型语言模型技术进入产业化新时代。

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
