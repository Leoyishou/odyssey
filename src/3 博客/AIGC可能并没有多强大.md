---
date created: 2023-12-04
date modified: 2025-07-10
uid: 11c747e1-7d50-456a-9d7a-63cd82482775
---
- [一、使用经历和分享内容](#%E4%B8%80%E3%80%81%E4%BD%BF%E7%94%A8%E7%BB%8F%E5%8E%86%E5%92%8C%E5%88%86%E4%BA%AB%E5%86%85%E5%AE%B9)
- [二、一些前置概念](#%E4%BA%8C%E3%80%81%E4%B8%80%E4%BA%9B%E5%89%8D%E7%BD%AE%E6%A6%82%E5%BF%B5)
	- [2.1 大模型幻觉](#2.1%20%E5%A4%A7%E6%A8%A1%E5%9E%8B%E5%B9%BB%E8%A7%89)
	- [2.2 大模型的本质是什么？](#2.2%20%E5%A4%A7%E6%A8%A1%E5%9E%8B%E7%9A%84%E6%9C%AC%E8%B4%A8%E6%98%AF%E4%BB%80%E4%B9%88%EF%BC%9F)
- [三、项目一：旅行助手](#%E4%B8%89%E3%80%81%E9%A1%B9%E7%9B%AE%E4%B8%80%EF%BC%9A%E6%97%85%E8%A1%8C%E5%8A%A9%E6%89%8B)
- [四、一些概念](#%E5%9B%9B%E3%80%81%E4%B8%80%E4%BA%9B%E6%A6%82%E5%BF%B5)
	- [4.1 AI决策](#4.1%20AI%E5%86%B3%E7%AD%96)
	- [4.2 上下文限制、embedding与向量数据库](#4.2%20%E4%B8%8A%E4%B8%8B%E6%96%87%E9%99%90%E5%88%B6%E3%80%81embedding%E4%B8%8E%E5%90%91%E9%87%8F%E6%95%B0%E6%8D%AE%E5%BA%93)
	- [4.3 prompt engineering有必要吗？](#4.3%20prompt%20engineering%E6%9C%89%E5%BF%85%E8%A6%81%E5%90%97%EF%BC%9F)
	- [4.4 结果约束](#4.4%20%E7%BB%93%E6%9E%9C%E7%BA%A6%E6%9D%9F)
	- [4.5 Streaming](#4.5%20Streaming)
	- [4.6 总结](#4.6%20%E6%80%BB%E7%BB%93)
- [五、项目二：口语复盘工具](#%E4%BA%94%E3%80%81%E9%A1%B9%E7%9B%AE%E4%BA%8C%EF%BC%9A%E5%8F%A3%E8%AF%AD%E5%A4%8D%E7%9B%98%E5%B7%A5%E5%85%B7)
	- [Openai的最新API](#Openai%E7%9A%84%E6%9C%80%E6%96%B0API)
	- [展示数据处理过程](#%E5%B1%95%E7%A4%BA%E6%95%B0%E6%8D%AE%E5%A4%84%E7%90%86%E8%BF%87%E7%A8%8B)
	- [总结](#%E6%80%BB%E7%BB%93)
- [六、避免大模型幻觉，反复确认它的真实本领](#%E5%85%AD%E3%80%81%E9%81%BF%E5%85%8D%E5%A4%A7%E6%A8%A1%E5%9E%8B%E5%B9%BB%E8%A7%89%EF%BC%8C%E5%8F%8D%E5%A4%8D%E7%A1%AE%E8%AE%A4%E5%AE%83%E7%9A%84%E7%9C%9F%E5%AE%9E%E6%9C%AC%E9%A2%86)
	- [6.1 第一类工具的范畴](#6.1%20%E7%AC%AC%E4%B8%80%E7%B1%BB%E5%B7%A5%E5%85%B7%E7%9A%84%E8%8C%83%E7%95%B4)
	- [6.2 第二类工具的范畴](#6.2%20%E7%AC%AC%E4%BA%8C%E7%B1%BB%E5%B7%A5%E5%85%B7%E7%9A%84%E8%8C%83%E7%95%B4)
- [七、总结：时刻用两类工具论审视AI发展](#%E4%B8%83%E3%80%81%E6%80%BB%E7%BB%93%EF%BC%9A%E6%97%B6%E5%88%BB%E7%94%A8%E4%B8%A4%E7%B1%BB%E5%B7%A5%E5%85%B7%E8%AE%BA%E5%AE%A1%E8%A7%86AI%E5%8F%91%E5%B1%95)
- [八、其他](#%E5%85%AB%E3%80%81%E5%85%B6%E4%BB%96)
	- [8.1 多模态的AI产品](#8.1%20%E5%A4%9A%E6%A8%A1%E6%80%81%E7%9A%84AI%E4%BA%A7%E5%93%81)
	- [8.2 怎么样快速上手一款AI产品](#8.2%20%E6%80%8E%E4%B9%88%E6%A0%B7%E5%BF%AB%E9%80%9F%E4%B8%8A%E6%89%8B%E4%B8%80%E6%AC%BEAI%E4%BA%A7%E5%93%81)
	- [8.3 怎么样获取最新的AI资讯](#8.3%20%E6%80%8E%E4%B9%88%E6%A0%B7%E8%8E%B7%E5%8F%96%E6%9C%80%E6%96%B0%E7%9A%84AI%E8%B5%84%E8%AE%AF)
		- [1. hugging face](#1.%20hugging%20face)
		- [2. youtuber：Matt Wolfe、assemblyAI](#2.%20youtuber%EF%BC%9AMatt%20Wolfe%E3%80%81assemblyAI)

## 一、使用经历和分享内容

Chatgpt、Midjourney、Elvenlabs、assemblyai  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213162208.png)

分享内容简单介绍

## 二、一些前置概念

### 2.1 大模型幻觉

  " 大模型幻觉 " 是指人们在与大语言模型交互时可能产生的一种错觉，即错误地认为这些模型具有真正的理解能力、意识或感知。这种幻觉通常源于模型生成的文本质量非常高，能够流畅、连贯地回答问题或进行对话，使人们感觉就像是在与一个真实的、有理解力的人类交谈。  
  举一个论坛上的例子，怎么写出更好的 prompt？回答 A 介绍了一堆循序渐进的步骤，但是高赞的回答 B 说到，这一切的前提首先是它是个 prompt 专家！那么它是吗？

  

### 2.2 大模型的本质是什么？

  是人类的工具，但是无法提供情感支持。  
  有两类工具，第一种是拿来就可以解决问题，实现目标的，比如战场上的一把枪，可以对赤手空拳的敌人降维打击；第二种是拿来可以提高我们自己解决问题的能力的，比如一个哑铃，我们可以通过它练习自己的力量，从而战胜力量不如我们的敌人。

## 三、项目一：旅行助手

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213150959.png)  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213151812.png)

langchain 只是将大模型开发的过程高度抽象化，最终 tools、agent 等都是基于老的 API 实现（在 playground 展示老的 api）

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213150655.png)  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213151952.png)

## 四、一些概念

### 4.1 AI 决策

	traditional deterministic code

 通过大量的 if-else 或者 switch 做出确定性的逻辑判断  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213153420.png)  
 这是大模型幻觉吗？这个决策过程是科学有效的吗？用户真的去玩了七天后，回来会满意吗？这其实是一个未验证的能力，也可能是未来 AI 发展的魅力所在，因为我们人脑可能也没有那么强的逻辑，解决一个问题时似乎也是产生一个模糊的似是而非的经验性的解决步骤。

 

### 4.2 上下文限制、embedding 与向量数据库

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213152540.png)

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213150325.png)

### 4.3 prompt engineering 有必要吗？

应用开发中是有用的！以 json 输出为例  
个人应用中性价比不高，另外对于一个黑箱，大量场景无法测试收益，设置为 template 嵌入到工具中更高效，核心是人设 + 结果约束  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213160747.png)

### 4.4 结果约束

展示项目中的 json 约束 prompt  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213141914.png)

### 4.5 Streaming

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213152831.png)

### 4.6 总结

- langchain 的本质：最终 tools、agent 等都是基于老的 API 实现（在 playground 展示老的 api），所有机制的实现都只是一段一段的 prompt。
- 最炫的点是通过自然语言调用本地函数

## 五、项目二：口语复盘工具

### Openai 的最新 API

使用开发者大会之后的新 api 开发（在 playground 展示新的 api）

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213151616.png)

### 展示数据处理过程

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213151451.png)  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213151144.png)  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213151049.png)

### 总结

- 这个工具能够奏效的逻辑起点是：对于中文，gpt 只会说没道理的话，但是不会产生不通顺的话，而通顺正是这个场景下用户的使用目的。
- 只是复盘的工具，解决不了沟通的欲望。市面上一些其他的口语 AI 应用，无法让人产生持久的交流欲望  
	是人类的工具，但是无法提供情感支持

## 六、避免大模型幻觉，反复确认它的真实本领

### 6.1 第一类工具的范畴

- 基本都是围绕着它的两个核心能力展开的，1: 语义理解 2: 一定能生成一个回答
- 检查错别字
- 降重改写
- 简单的信息检索
	- 1. 语义理解
	- 2. 适合个人对于一个新领域的冷启动，适合没有官方文档，或者官方文档不够完整或者不够有趣的前提下，提供一定的词汇扩展和交互性。
	- 3. 但是对于更细分更专门化的知识更好的选择是找到对应的群体  
![image.png|100](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213155245.png)  
![image.png|100](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213155322.png)

### 6.2 第二类工具的范畴

- 语言学习
- 对于一个新领域的冷启动，交互式初探

## 七、总结：时刻用两类工具论审视 AI 发展

	  有两类工具，第一种是拿来就可以解决问题，实现目标的，比如战场上的一把枪，可以对赤手空拳的敌人降维打击；第二种是拿来可以提高我们自己解决问题的能力的，比如一个哑铃，我们可以通过它练习自己的力量，从而战胜力量不如我们的敌人。
- 当 AI 作为第一类工具时，一定会被各种公司通过商业化制作成简单易上手的产品，对于个人来说，无需焦虑，只需享受，真的好用的 AI 产品一定会快速出圈！
- 当 AI 作为第二类工具时，更考验我们的自我规划，自我探索，热爱和自律，一个比较现实的切入点是，尽可能多得保留个人痕迹的数据。
- 信息检索和吸收能力 --> 认知能力
- 间隔重复，大量练习，时间的投入永远是个人能力的护城河
- 数据驱动&心态开放

## 八、其他

### 8.1 多模态的 AI 产品

文字、图片、视频、声音  
Midjourney  
Eleven labs

### 8.2 怎么样快速上手一款 AI 产品

1. playground 试接口（展示 [OpenAI](OpenAI.md)和、assemblyai、Eleven labs）
2. 遇到问题查 forum 或者 discord

### 8.3 怎么样获取最新的 AI 资讯

#### 1. hugging face

[Hugging Face – The AI community building the future.](https://huggingface.co/)  
![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213150520.png)

The platform where the machine learning community collaborates on models, datasets, and applications.

#### 2. youtuber：Matt Wolfe、assemblyAI

![image.png|1000](https://cdn.jsdelivr.net/gh/Leoyishou/imageHosting@main/img/20231213144248.png)
