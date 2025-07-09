---
aliases:
- NLP
comment_id: 1a380ab5
date created: 2024-06-15
date modified: 2025-06-15
draw: null
title: 自然语言处理 NLP
---
真正NLP发展的历史：2017年，Google发布transformer（attention is all you need）大家：我操，牛逼，一个伟大的时代到来了。2018年，Google发布encoder only BERT发布，OpenAI发布decoder only GPT。大家：我操，更牛逼了！一年内，Google官宣：Google搜索里面已经强力嵌入BERT了，你们每一次搜索都是BERT的结果！2019年，大家就知道了，CV已经快死了，想做新东西只能沿着NLP开始做，于是2019年成了BERT大灌水元年，OpenAI发布了GPT-2，2019年，Google发布了当时巨无霸T5，当时人们惊呼，Google发布这么大的model，你不要命了？其实当年T5最大的也只有11b parameters，最小才76 million，跟今天比，简直小太多了；同年Meta也发布了Megatron，2020年，OpenAI发布了GPT-3，我清楚记得当年那个震撼的视频，随手拉个表格，一大堆公司的股价和信息自动补全，当时整个科技圈彻底沸腾了，一群人抢着要GPT-3的内测资格，同时Meta也发布了更大号的Megatron，2021年，一群人意识到了时代要来临了，开始抢占话语权了，开始发明新概念“foudation model”（基础模型），山雨欲来风满楼，大家都知道革命要出现了，各种任务和benchmark像下饺子一样出现了，2022年年底，ChatGPT发布。

| 模型类型            | 核心结构         | 典型模型      |
| --------------- | ------------ | --------- |
| 循环神经网络（RNN）| 循环结构         | LSTM、GRU  |
| 卷积神经网络（CNN）| 卷积结构         | TextCNN   |
| Transformer模型   | 自注意力机制       | BERT、GPT  |
|                 |              |           |

最初是基于规则，做语法分析、语义分析，遇到了很大的瓶颈。70 年代以后，转向了基于统计的思路，获得了极大成功。

莱特兄弟发明飞机并不是靠仿生学，而是靠空气动力学。
