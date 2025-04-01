---
draw:
tags: []
title: Prompt Caching
date created: 2024-11-25
date modified: 2024-12-27
---

OpenAI的Prompt Caching功能是自动启用的,无需开发者手动开启。以下是关于OpenAI Prompt Caching的主要信息:

1. 自动应用:  
    Prompt Caching自动应用于最新版本的GPT-4o、GPT-4o mini、o1-preview和o1-mini模型,以及这些模型的微调版本[
    

2. 触发条件:

- 缓存自动应用于长度超过1024个token的提示[
    

- 缓存以128个token为增量进行,从1024个token开始[
    

    2

    

](https://platform.openai.com/docs/guides/prompt-caching).

1. 价格优势:

- 缓存的提示比未缓存的提示更便宜,可获得50%的折扣[
    

    1

    

](https://cookbook.openai.com/examples/prompt_caching101)[

    

    4

    

](https://www.53ai.com/news/finetuning/2024100330976.html).

- 例如,GPT-4o模型的原价为$2.50/1K tokens,缓存价格为$1.25/1K tokens[
    

    4

    

](https://www.53ai.com/news/finetuning/2024100330976.html).

1. 性能提升:

- 可以减少延迟高达80%[
    

    1

    

](https://cookbook.openai.com/examples/prompt_caching101).

- 对于长提示(超过10,000个token),延迟减少尤为显著[
    

    1

    

](https://cookbook.openai.com/examples/prompt_caching101).

1. 使用建议:

- 将静态内容(如指令和示例)放在提示的开头,将可变内容(如用户特定信息)放在末尾[
    

    2

    

](https://platform.openai.com/docs/guides/prompt-caching).

- 这种结构可以增加缓存命中率,从而提高效率[
    

    6

    

](https://www.53ai.com/news/finetuning/2024100945782.html).

1. 监控使用情况:

- API响应中的'usage'字段包含'cached_tokens'值,显示有多少token被缓存[
    

    5

    

](https://openai.com/index/api-prompt-caching/).

1. 缓存持续时间:

- 缓存通常在5-10分钟不活动后清除,最长保留1小时[
    

    4

    

](https://www.53ai.com/news/finetuning/2024100330976.html)[

    

    5

    

](https://openai.com/index/api-prompt-caching/).

总之,开发者无需采取任何特殊操作来启用Prompt Caching。只要使用支持的模型并发送超过1024个token的提示,系统就会自动应用缓存,从而降低成本并提高性能

入参是 项目 id，截止时间，返回是[{docName：，lastCommit：，lastDiff}]

```Java
入参：
 groupId：str
 queryTime：str

出参：
[
  {
    "docName": "文档名称",
    "lastCommit": "最后commit的文本",
    "lastDiff": "最后 commit 和前一次的 diff 文本"
  }，
  {
    "docName": "文档名称",
    "lastCommit": "最后commit的文本",
    "lastDiff": "最后 commit 和前一次的 diff 文本"
  }，
]



```
