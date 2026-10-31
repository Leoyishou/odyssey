---
uid: ebde7c52-535a-4beb-b9c5-b98488906b88
date created: 2026-10-26
date modified: 2026-10-26
---
#  💡Summary  

> [!Info]
> 1. 生成记忆本质是对 session log 的索引，不同的技术代表[[索引Index]]不同
> 2. 回忆的本质是[[搜索]]
> 3. 更新方式很重要，而且其中一定需要人的参与
> 

1. 记忆分三层：核心区：AI 生成区：归档区。 个人 Agent 的核心区记忆是不是应该自己参与维护
2. 不同的索引方式体现了业务主张。然后我觉得这个东西是可以被评测，或者说是离线的 Agent 不断去发现出来的。比如说用户的哪些 query 其实是和之前的 history 有关联的，用哪个信息的话可以辅助 query 的解决。然后通过这个方式慢慢的去发现出来那些有用的、值得被特殊缓存的信息。
3. 记忆的第二核心是人机协同，因为 agent 是为人服务的，人的目标在人的脑海中，agent 不完全知道，所以一定需要人机协同.每个 agent 要有独立的 bank 的记忆不要混

# 🧩 Cues

 1. mem0
 2. claude-mem
 3. [[hindsight]]
 4. llm wiki

[[grep]]

# 🪞Notes

## hindsight

1. 打标存储
	1. 写入时，先由大模型把原文提取成**事实、实体、关系和时间**，再建立检索结构。（[写入机制](https://hindsight.vectorize.io/developer/retain)）
2. 构建四路索引
3. 搜索（多路召回 → 按排名融合 → 模型重新排序）
	1. [[reranker 重排序]]默认是 `cross-encoder/ms-marco-MiniLM-L-6-v2`
4. 新增数据
	1. 构建四路索引
	2. 归纳：后台将多条事实归纳成持续更新的 observations。例如，从多次交流归纳出“用户以前偏爱 React，现在转向 Vue”。

|                | 检索结构    | 作用                  |
| -------------- | ------- | ------------------- |
| 向量 embedding   |         | 找语义相近的事实，支持换种说法     |
| [[BM25]]       | 关键词全文索引 | 找具体人名、产品名、术语        |
|                | 关系图     | 沿实体、因果等关联找到间接信息     |
| 类似于先打标然后做元数据搜索 | 时间字段与关联 | 找“去年”“入职前”等时间范围内的信息 |



```rich-table
---
version: 2
title: 康奈尔笔记
columns:
  - id: c_cue
    name: 线索/问题
    width: 219
  - id: c_notes
    name: 笔记要点
    width: 708
rows:
  - id: r_0
    cells:
      c_cue: 什么是康奈尔笔记法？
      c_notes: 康奈尔大学 Walter Pauk 提出的笔记法，把页面分成线索、笔记、总结三块区域
  - id: r_1
    cells:
      c_cue: 三个区域分别记什么？
      c_notes: 笔记区记课上/阅读时的主体内容；线索区课后补关键词和问题，用于自测；总结区读完用一两句话概括
  - id: r_2
    cells:
      c_cue: 为什么要把线索和笔记分开？
      c_notes: 逼着自己做"记录—提炼—总结"的主动回顾，而不是记完就扔在一边
  - id: r_3
    cells:
      c_cue: 复习时怎么用？
      c_notes: 遮住笔记区，只看线索区的问题，试着口头回答，再对照笔记区检查有没有漏
  - id: r_4
    cells:
      c_cue: 适合什么场景？
      c_notes: 听课、读书、开会记录都适用，尤其适合之后要复习或考试的内容
  - id: r_5
    cells: {}
  - id: r_summary
    cells:
      c_cue: 总结：康奈尔笔记法通过"线索-笔记-总结"三栏结构，把被动记录变成主动复习的工具。
merges:
  - anchor: r_summary.c_cue
    end: r_summary.c_notes
styles:
  - target: c_cue
    bg: "#FFF7D6"
  - target: r_summary
    bg: "#E8F0FE"
    bold: true
  - target: header
    bold: true
---
```