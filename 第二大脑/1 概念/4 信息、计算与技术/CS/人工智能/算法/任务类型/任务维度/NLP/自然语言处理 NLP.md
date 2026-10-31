---
aliases: [NLP]
date created: 2024-06-15
date modified: 2025-11-01
uid: 0e5dd3ef-b0b4-4f18-bf82-cbfa1dd68e62
---
# Summary

真正NLP发展的历史：
1. 2017年，Google发布[Transformer](Transformer.md)（attention is all you need）大家：我操，牛逼，一个伟大的时代到来了。
2. 2018年，Google发布encoder only BERT发布，OpenAI发布decoder only GPT。大家：我操，更牛逼了！一年内，Google官宣：Google搜索里面已经强力嵌入[[BERT]]了，你们每一次搜索都是BERT的结果！
3. 2019年，大家就知道了，CV已经快死了，想做新东西只能沿着NLP开始做，于是2019年成了BERT大灌水元年，OpenAI发布了GPT-2，2019年，Google发布了当时巨无霸T5，当时人们惊呼，Google发布这么大的model，你不要命了？其实当年T5最大的也只有11b parameters，最小才76 million，跟今天比，简直小太多了；同年Meta也发布了Megatron。
4. 2020年，OpenAI发布了GPT-3，我清楚记得当年那个震撼的视频，随手拉个表格，一大堆公司的股价和信息自动补全，当时整个科技圈彻底沸腾了，一群人抢着要GPT-3的内测资格，同时Meta也发布了更大号的Megatron。
5. 2021年，一群人意识到了时代要来临了，开始抢占话语权了，开始发明新概念“foudation model”（基础模型），山雨欲来风满楼，大家都知道革命要出现了，各种任务和benchmark像下饺子一样出现了。
6. 2022年年底，ChatGPT发布。
最初是基于规则，做语法分析、语义分析，遇到了很大的瓶颈。70 年代以后，转向了基于统计的思路，获得了极大成功。

莱特兄弟发明飞机并不是靠仿生学，而是靠空气动力学。


# Notes

对话与交互
- [[任务型对话]]（NLU→[[对话状态跟踪 DST]]→[[Policy]]→NLG）
- 开放域闲聊/检索增强对话
- 对话状态跟踪（[[对话状态跟踪 DST]]）
- 多轮检索式问答/澄清式交互
- 对话安全/内容审控/用户建模

检索与排序（IR/语义检索）
- 关键词检索（[倒排索引](倒排索引.md)/[[BM25]]）
- 密集检索（双塔）与向量索引[embedding](embedding.md)
- 交互式重排（Cross-Encoder/Rerank）
- 检索式问答组件（Retriever for QA）

信息抽取（IE）
- [命名实体识别 NER](命名实体识别%20NER.md)
- [[实体链接, 挂靠]]/消歧（EL/NEL）
- 关系抽取（RE）
- 事件抽取（EE：触发词/论元/时间）
- 时间/数量表达抽取与归一化（TIMEX）
- 关键词/术语/关键短语抽取
- [[Open IE]]（开放域三元组）
- 文档级抽取/版面理解（Doc IE，Layout）

[[文本分类]]
- [[主题分类]]/多标签分类
- [[情感分析]]与情绪（含[方面级情感分析](方面级情感分析.md) ABSA）
- 自然语言推断（NLI/Entailment）
- 立场/观点/仇恨言论/毒性/垃圾
- [意图识别](意图识别.md)（Intent）/场景分类
- 语言识别
- 作者/风格/体裁/讽刺反语检测

语音与语言（边界学科）
- [[语音识别]] 自动语音识别（语音→文本）
- [[TTS]] 语音合成（文本→语音）
- 语音理解/说话人/情感识别

LLM 时代的工作流（现代范式）
- [Prompt](Prompt.md) 设计（CoT/少样本/结构化输出）
- 指令微调（[监督微调](监督微调.md)）
- 偏好对齐（[[RLHF]]/[[DPO]]）
- [[RAG]] 管线（切分→索引→检索→重排→生成）
- 工具/函数调用[Function Call](Function%20Call.md)与代理（Agents/Tool Use）
- 安全与治理（越狱/偏见/隐私/红队）
- 系统评测（MT-Bench/HELM/人评）

数据与评测
- [[标注]]（指南/一致性/质控/众包）
- 数据合成/过滤/去重/去毒
- 指标（BLEU/ROUGE/chrF/BERTScore/COMET/Acc/[[F1]]/MRR）
- 基准[benchmark](benchmark.md)（GLUE/SuperGLUE/SQuAD/XSum/XTREME/[[MMLU]] 等）

语料与预处理
- 分词/断句/[[Tokenization 分词]]
- 词形还原/Lemmatization，词干化
- 子词/字节切分（BPE/WordPiece）
- 清洗/去重/对齐/数据均衡

基础语言分析（语言学驱动）
- 词性标注（POS）
- 句法分析（成分/依存）
- 语义角色标注（SRL）
- 语义解析（AMR/逻辑式/UCCA/SDP）
- 指代消解/共指（Coreference）

序列标注/片段抽取
- 分块/短语切分（Chunking）
- 槽位标注（SLU）
- 片段/跨度抽取（Span Extraction）

问答与阅读理解（QA/RC）
- 抽取式 QA（Span-based）
- 生成式 QA（闭卷/开卷）
- 多跳/多文档/表格/图表 QA
- 知识库问答（KBQA）

文本生成（NLG）
- 机器翻译（MT：单/双向、多语、低资源）
- 摘要（抽取式/生成式/多文档）
- 改写/释义生成/风格迁移
- 数据到文本（Data-to-Text）
- 问题生成/标题生成/报告生成
- 代码相关（代码生成/注释/解释）
- 受控生成（长度/语气/术语约束）

知识与语义
- 知识库构建（实体/关系/事件灌库）
- 本体/术语库/对齐与映射
- 链接预测/知识补全（KBC）
- 语义相似度/释义判定（STS）
- 词义消歧（WSD）

篇章/话语层
- 篇章结构/话语解析（RST 等）
- 篇章连贯性/一致性建模
- 指代/省略恢复/语用现象（讽刺、隐喻）

多模态与文档智能
- 图文：图像描述、VQA、多模态检索
- 文档理解（[[OCR]]+Layout+表格/表单/图表）
- 视频-语言（时序定位/视频问答）
