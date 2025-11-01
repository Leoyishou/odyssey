---
date created: 2024-11-25
date modified: 2025-11-01
uid: 330545a4-236b-4c57-ba2f-adda7a3d0d18
---
# Summary

为 LLM 寻找相关上下文就是"[[搜索]]"。这就是为什么向量数据库和 LLM 就像饼干和奶油一样完美搭配。

在**语义完整**（保留专业术语）、**格式统一**（去除冗余标签）和**检索效率**（控制Chunk大小）之间找到黄金平衡点。

当我刚加入公司时，向量数据库是热门；后来，RAG 成了焦点；现在，人们又开始讨论“上下文工程”（C ontext Engineering）。但无论概念如何演变，有一件事从未改变，那就是：我们必须找到最好的信息提供给大语言模型，才能让它给出最好的答案。而这，正是检索的永恒价值所在。

# Notes

## 评估指标

查询类型                      推荐指标
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
							[[准确率、召回率]]
导航查询 (单答案)       [[MRR（Mean Reciprocal Rank）- 平均倒数排名]]
信息检索 (多答案)       [[MAP@k]] + [[NDCG]]
推荐系统                      NDCG
问答系统                      MRR
电商搜索                      MAP + [[业务指标]]
				

 [[MRR（Mean Reciprocal Rank）- 平均倒数排名]]
```Java
✅ 适合 MRR：
- 问答系统："北京的首都是哪？" → 只有1个正确答案
- 导航查询："淘宝官网" → 用户只想要第一个链接
- 实体搜索："苹果公司CEO" → 明确的事实

❌ 不适合 MRR：
- 信息搜索："成都旅游攻略" → 可能有很多好答案
- 推荐系统：用户可能对多个结果都感兴趣
```


## 我在 Weaviate 工作两年学到的关于向量数据库和搜索的知识

今天我在庆祝自己在 Weaviate（一家向量数据库公司）工作两周年。为了庆祝这一时刻,我想反思一下在这段时间里学到的关于向量数据库和搜索的知识。以下是我学到的一些东西以及我看到的一些常见误解:

**向量数据库的主要应用不在生成式 AI,而在搜索。** 但为 LLM 寻找相关上下文就是"搜索"。这就是为什么向量数据库和 LLM 就像饼干和奶油一样完美搭配。

**[[BM25]] 是搜索的强大基准线。** 哈!你以为我会从向量搜索开始讲,而我却在这里讨论关键词搜索。这正是第一课:在转向向量搜索等更复杂的方法之前,先从 BM25 这样简单的东西开始。

**向量数据库中的向量搜索是近似的而非精确的。** 理论上,你可以使用精确的 k-最近邻算法(KNN)运行暴力搜索,计算查询向量与数据库中每个向量之间的距离。但这无法很好地扩展。这就是为什么向量数据库使用近似最近邻(ANN)算法,如 [[HNSW]]、IVF 或 ScaNN,来加速搜索,同时牺牲少量准确性。向量索引正是使向量数据库在大规模下如此快速的原因。

**向量数据库不仅仅存储嵌入向量。** 它们还存储原始对象(例如,生成向量嵌入的文本)和[[元数据]]。这使它们能够支持向量搜索之外的其他功能,如元数据过滤、关键词搜索和混合搜索。

**你必须指定要检索多少结果。** 回想起来,我几乎要笑了,因为当我意识到需要定义要检索的最大结果数时,这是一个很大的"顿悟"时刻。这有点过于简化了,但如果没有限制或 top_k 参数,向量搜索会返回数据库中存储的所有对象,按与查询向量的距离排序。

**有许多不同类型的嵌入向量。** 当你想到向量嵌入时,你可能会想象类似`[-0.9837, 0.1044, 0.0090, …, -0.2049]` 的东西。这被称为密集向量,是最常用的向量嵌入类型。但还有许多其他类型的向量,例如稀疏向量`([0, 2, 0, …, 1])`、二值向量`([0, 1, 1, …, 0])`和多向量嵌入`([[-0.9837, …, -0.2049],[0.1044, …, 0.0090], …,[-0.0937, …, 0.5044]])`,可用于不同的目的。

**优秀的嵌入模型以及在哪里找到它们。** 首选之地是大规模文本嵌入基准([[MTEB]])。它涵盖了嵌入模型的各种不同任务,包括分类、聚类和检索。如果你专注于信息检索,可能需要查看 BEIR:零样本信息检索模型评估的异构基准。

**MTEB 上的大多数嵌入模型都是英语的。** 如果你使用多语言或非英语语言,可能值得查看 MMTEB(大规模多语言文本嵌入基准)。

**向量嵌入的一点历史:** 在今天的上下文嵌入(例如 BERT)出现之前,有静态嵌入(例如 [[Word2Vec]]、GloVe)。它们是静态的,因为每个词都有固定的表示,而上下文嵌入根据周围的上下文为同一个词生成不同的表示。尽管今天的上下文嵌入更具表现力,但静态嵌入在计算资源受限的环境中可能有帮助,因为它们可以从预计算的表中查找。

**不要混淆稀疏向量和稀疏嵌入。** 我花了一段时间才理解[[稀疏向量]]可以通过不同方式生成:要么对词频应用统计评分函数(如 TF-IDF 或 BM25)(通常通过倒排索引检索),要么使用神经稀疏嵌入模型(如 SPLADE)。这意味着稀疏嵌入是稀疏向量,但并非所有稀疏向量都一定是稀疏嵌入。

**嵌入一切。** 嵌入不仅适用于文本。你可以嵌入图像、PDF 图像(参见 ColPali)、图形等。这意味着你可以对多模态数据进行向量搜索。这非常令人难以置信。你应该试试。

**向量嵌入的经济学。** 这不应该令人惊讶,但向量维度会影响所需的存储成本。因此,在选择具有 1536 维的嵌入模型而不是 768 维的模型并冒着存储需求翻倍的风险之前,请考虑是否值得。是的,更多维度可以捕获更多语义细微差别。但你可能不需要 1536 维来"与你的文档聊天"。一些模型实际上使用 Matryoshka 表示学习,允许你在计算资源较少的环境中缩短向量嵌入,性能损失最小。

**说到这里:"与你的文档聊天"教程是生成式 AI 的"Hello World"程序。**

**你需要大量调用嵌入模型。** 仅仅因为你在摄取阶段嵌入了文档,并不意味着你已经完成了对嵌入模型的调用。每次运行搜索查询时,查询也必须被嵌入(如果你不使用缓存)。如果你稍后添加对象,这些对象也必须被嵌入(和索引)。如果你更改嵌入模型,你还必须重新嵌入(和重新索引)所有内容。

**相似不一定意味着相关。** 向量搜索通过对象与查询向量的相似性返回对象。相似性是通过它们在向量空间中的接近程度来衡量的。仅仅因为两个句子在向量空间中相似(例如,"如何修理水龙头"和"在哪里购买厨房水龙头")并不意味着它们彼此相关。

**余弦相似度和余弦距离不是同一回事。** 但它们彼此相关。如果你愿意,距离和相似度是相反的:如果两个向量完全相同,相似度为 1,它们之间的距离为 0。

**如果你使用归一化向量,使用余弦相似度还是点积作为相似度度量并不重要。** 因为从数学上讲,它们是相同的。对于计算而言,点积更高效。

**常见误解:RAG 中的 R 代表"向量搜索"。** 不是的。它代表"检索"。检索可以通过许多不同的方式完成(见以下要点)。

**向量搜索只是检索工具箱中的一个工具。** 还有基于关键词的搜索、过滤和重排序。不是非此即彼。要构建出色的东西,你需要将它与不同的工具结合起来。

**何时使用基于关键词的搜索与基于向量的搜索:** 你的用例主要需要匹配语义和同义词(例如,"柔和的颜色"与"浅粉色")还是精确关键词(例如,"A字裙"、"荷叶边连衣裙")?如果两者都需要(例如,"柔和色彩的 A 字裙"),你可能会受益于结合两者并使用混合搜索。在某些实现中(例如 Weaviate),你可以直接使用混合搜索功能,然后使用 alpha 参数改变权重,从纯关键词搜索、两者混合到纯向量搜索。

**混合搜索可以是不同搜索技术的混合。** 通常,当你听到人们谈论混合搜索时,他们指的是基于关键词的搜索和基于向量的搜索的组合。但"混合"一词并未指定要组合哪些技术。因此,有时你可能会听到人们谈论混合搜索,意思是基于向量的搜索和对结构化数据的搜索(通常称为元数据过滤)的组合。

**误解:过滤使向量搜索更快。** 直觉上,你会认为使用过滤器应该加快搜索延迟,因为你减少了要搜索的候选数量。但实际上,预过滤候选可能会破坏 HNSW 中的图连接性,而后过滤可能会让你根本没有结果。向量数据库有不同的复杂技术来应对这一挑战。

**两阶段检索管道不仅用于推荐系统。** 推荐系统通常有第一个检索阶段,使用更简单的检索过程(例如向量搜索)来减少潜在候选的数量,然后是第二个检索阶段,使用计算密集型但更准确的重排序阶段。你也可以将此应用于你的 RAG 管道。

**向量搜索与重排序的区别。** 向量搜索从整个数据库返回一小部分结果。重排序接收项目列表并返回重新排序的列表。

**找到合适的块大小进行嵌入并非易事。** 太小,你会失去重要的上下文。太大,你会失去语义含义。许多嵌入模型使用平均池化将所有标记嵌入平均为块的单个向量表示。因此,如果你有一个具有大上下文窗口的嵌入模型,理论上你可以嵌入整个文档。我忘了是谁说的,但我喜欢这个类比:你可以把它想象成通过叠加电影中的每一帧来为电影制作海报。所有信息都在那里,但你不会理解电影的内容。

**向量索引库与向量数据库不同。** 两者在向量搜索方面都非常快。两者都非常适合在"与你的文档聊天"风格的 RAG 教程中展示向量搜索。然而,只有其中一个添加了数据管理功能,如内置持久化、CRUD 支持、元数据过滤和混合搜索。

**自第一个长上下文 LLM 发布以来,RAG 一直在"死亡"。** 每次发布具有更长上下文窗口的 LLM 时,就会有人声称 RAG 已死。但它从未死过……

**你可以丢弃 97% 的信息,仍然可以(在某种程度上)准确检索。** 这被称为向量量化。例如,通过二值量化,你可以将类似[-0.9837, 0.1044, 0.0090, …, -0.2049]的东西变成[0, 1, 1, …, 0](从 32 位浮点数到 1 位的 32 倍存储减少),你会惊讶地发现检索仍然工作得很好(在某些用例中)。

**向量搜索对拼写错误不具有鲁棒性。** 有一段时间,我认为向量搜索对拼写错误具有鲁棒性,因为这些大型文本语料库肯定包含很多拼写错误,因此有助于嵌入模型也学习这些拼写错误。但如果你仔细想想,一个词的所有可能拼写错误不可能在训练数据中以足够的数量反映出来。因此,虽然向量搜索可以处理一些拼写错误,但你不能真的说它对它们具有鲁棒性。

**知道何时使用哪个指标来评估搜索结果。** 有许多不同的指标来评估搜索结果。查看学术基准,如 BEIR,你会注意到 NDCG@k 很突出。但像精确率和召回率这样更简单的指标非常适合许多用例。

**精确率-召回率权衡通常用渔夫撒网的类比来描述,但这个电子商务类比让我更好地理解了:** 想象你有一个拥有 100 本书的网店,其中 10 本与机器学习相关。

现在,如果用户搜索与机器学习相关的书籍,你可以只返回一本机器学习书籍。太棒了!你有完美的精确率(在返回的 k=1 个结果中,有多少是相关的)。但这是糟糕的召回率(在存在的相关结果中,我返回了多少?在这种情况下,10 本相关书籍中的 1 本)。而且,这对你的业务也不太好。也许用户不喜欢你返回的那一本与机器学习相关的书。

另一个极端是如果你返回整个书籍选择。全部 100 本。未排序…… 这是完美的召回率,因为你返回了所有相关结果。只是你也返回了一堆不相关的结果,这可以通过精确率有多糟糕来衡量。

**有些指标包括顺序。** 当我想到搜索结果时,我会想象类似谷歌搜索的东西。因此,我自然认为搜索结果的排名很重要。但像精确率和召回率这样的指标不考虑搜索结果的顺序。如果搜索结果的顺序对你的用例很重要,你需要选择排名感知指标,如 [[MRR（Mean Reciprocal Rank）- 平均倒数排名]]@k、[[MAP@k]] 或 [[NDCG]]@k。

**分词器很重要。** 如果你在 Transformer 的泡沫中待得太久,你可能已经忘记了除了字节对编码(BPE)之外还存在其他分词器。分词器对于关键词搜索及其搜索性能也很重要。如果分词器影响基于关键词的搜索性能,它也会影响混合搜索性能。

**域外与词汇表外不同。** 早期的嵌入模型曾经在词汇表外术语上失败。如果你的嵌入模型从未见过或听说过"Labubu",它只会遇到错误。通过智能分词,看不见的词汇表外术语可以得到优雅处理,但问题是它们仍然是域外术语,因此,它们的向量嵌入看起来像一个合适的嵌入,但它们是无意义的。

**查询优化:** 你知道你是如何学会在谷歌搜索栏中输入"longest river africa"而不是"What is the name of the longest river in Africa?"的。你已经学会了为关键词搜索优化搜索查询(是的,我们知道谷歌搜索算法更复杂。我们能就这样继续吗?)。类似地,我们现在需要学习如何为向量搜索优化我们的搜索查询。

**向量搜索之后是什么?** 首先,有基于关键词的搜索。然后,机器学习模型实现了向量搜索。现在,具有推理能力的 LLM 实现了基于推理的检索。

**信息检索现在非常热门。** 我很幸运能在这个令人兴奋的领域工作。虽然现在使用 LLM 工作似乎是很酷的事情,但弄清楚如何为它们提供最佳信息同样令人兴奋。这就是检索领域。

**我在重复我的最后一点,但回顾过去两年,我感到很幸运能在这个领域工作。** 到目前为止,我只是触及了表面,还有很多东西要学。当我加入 Weaviate 时,向量数据库是热门新事物。然后是 RAG。现在,我们正在谈论"上下文工程"。但没有改变的是找到最佳信息提供给 LLM 以便它可以提供最佳答案的重要性。

这篇博客的灵感来自 Doug Turnbull 的博客文章《AI 工程师需要了解的搜索知识》。如果你喜欢这篇博客,你可能也会喜欢 Doug 的博客。




## R

RAG 中的 “R” 代表“检索”，”，而不仅仅是[[向量检索]] 一个非常普遍的误解是，RAG（Retrieval-Augment ed Generation）中的“R” 代表向量搜索（Vector Search）。事实并非如此，

“R”代表的是“检索”（R etrieval)。向量搜索只是整个检索工具箱中的一件工具。一个强大的检索系统，需要将
1. 关键字搜索、
2. 元数据过滤、
3. 重排序（reranking）等多种工具结合起来。它们不是相互替代的关系，而是相辅相成的。想要构建卓越的应用，你必须学会组合使用这些工具。在这些工具中，过滤器的行为方式尤其出人意料。










> 简而言之：ingest 是“把原始数据摄入系统”的行话。
  - 在数据/搜索/ML 里，ingest 常指“取数/导入/采集”：读取原始数据、解析为内部结构、做极轻量规范化，供后续清洗/分块/索引使用。它比单
  纯的 import/load 更强调面向流水线的“数据进入系统”的过程。
  - 在本项目里，rag/ingest.py 做的就是：
      - 从 TSV 读取每行
      - 解析 source_data.title/content
      - 轻量 normalize（去零宽空格、压缩空白）
      - 组装成内部的 Post 对象
  - 对照关系：
      - ingest = 取数/导入/采集
      - clean = 清洗/去重
      - chunk = 分块
      - index = 建索引
      - retrieve = 召回/检索
      - generate = 生成行程



Pipeline：切分→索引→检索→重排→生成

#第一性原理  最关键的，你得有套评估体系。
找个你身边的场景，哪怕就是把你电脑里存的几百篇PDF论文或者笔记整理一下，搭一个最简单的RAG出来。当你真的能用自然语言跟你自己的知识库对话，并且得到靠谱答案的时候，那种感觉，比你在学术榜单上刷高0.1个点要爽得多。


![CleanShot 2025-04-16 at 23.15.49@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_O3jP7AdX37%2F2025%2F04%2F16%2F23-16-02-ef5fe329d3cc4d7db1a070e6f38b3a72-CleanShot%202025-04-16%20at%2023.15.49-2x-30531a.png)


标题: LLM RAG值得做吗？

看了一个月论文代码开始怀疑人生，这太正常了。当年我刚从推荐转过来搞NLP的时候，也是天天看谷歌、Meta那些神仙打架的论文，动不动就上千块GPU跑实验，感觉自己要是不租个服务器集群，都不配搞这行。
但咱得想明白一件事，学术圈发论文，跟咱们在公司里做项目，完全是两码事。
RAG这东西，到底值不值得做？我的答案是：非常值得，而且很可能是接下来几年里，绝大多数公司能把大模型用起来、做出东西、看到回头钱的最靠谱的一条路。
但一定要搞清楚自己做 RAG 是为了什么。别一上来就想着拉满配置去对标大厂的全量检索，不然很容易就是钱花了、机器烧了，效果还不稳。
我这几年看下来，RAG 的玩法大概两种路子。
1. 一种是“全域型”的，追求把所有能找到的语料都塞进去，比如 Wikipedia、新闻、论坛、公司文档全混一起。这种确实够广，也看着“厉害”，但硬件管着你脖子，动辄几十亿条记录，embedding计算、索引构建、更新维护都要人盯着。我们团队以前做过一次类似的，大概 8 亿条数据，用ES+Faiss跑了快一个月才搞定，embedding 一更新又得重来一次，人手不够直接炸锅，这种规模真不是普通团队能那么轻松玩的。
2. 另一种是聚焦垂直领域的，这类反而存活率高、效果稳定。比如法律、医疗、金融，语料的量级是几十万到几百万条，而且比较稳定，不用频繁更新。embedding 模型还能微调到领域里，检索更准，硬件压力也小的多。我自己给一家证券公司做过一个内部 RAG，数据才 500 万条不到，用单机加两张高性能卡，200ms 内出结果，用户反馈很稳，这种项目性价比特别高。
你之所以纠结，就是被学术圈那种玩法给绕进去了。他们搞研究，追求的是在公开数据集上把指标刷到最高，这样才能发顶会，才能证明自己的方法牛逼。所以他们当然要用维基百科、C4这种巨无霸语料库，不然怎么体现出模型的“知识渊博”？这就像是为了测试发动机的极限，非得把车开到德国不限速高速上，往死里踩油门。

可咱们在公司里干活，不是为了破纪录，是为了解决实际问题。没人关心你的系统知不知道八百年前的罗马史，大家只关心它能不能从公司那一堆乱七八糟的文档里，找到解决当前问题的答案。

**RAG的价值不在“大”，恰恰在“小”和“专”**

我给你讲个我前两年带队做的一个项目，你就明白了。

我们给一个做重型机械的老牌国企做内部的专家问答系统。他们的知识库是啥？根本不是什么互联网，就是内部文件服务器上存了几十年的各种技术文档、维修报告、客户现场问题记录单，格式五花八门，PDF、Word、甚至还有扫描版的图片。这些东西加起来，清理干净了也就几十个G。

你想想这个场景：一个年轻的售后工程师，在客户的工地上，对着一台停工的机器满头大汗。以前怎么办？翻纸质手册，或者用他那个破笔记本连内网，用巨慢的搜索引擎搜关键词，下载一堆PDF挨个看，再不行就硬着头皮打电话问快退休的老师傅。

我们做的RAG系统上线后，他直接掏出手机，对着App说：“503型号掘进机液压系统压力异常，错误码E-45，去年在山西项目好像有过类似问题，怎么处理？”

我们的系统这时候干了啥？
它压根就不会去互联网上瞎搜，而是直接在你说的这几十个G的“私有知识”里做检索。它会很快地把最相关的几段内容给捞出来，比如：

《503型号掘进机维修手册》里关于E-45错误码的那一章。

一份word文档，标题是《关于处理山西XX煤矿503掘进机液压故障的报告》。

几条老师傅们在内部论坛里零散的讨论记录。

然后，系统把这些捞出来的“证据”塞给大模型，告诉它：“你现在是这个厂里最有经验的维修专家，根据下面这些资料，给现场的工程师一个清晰、能直接上手的操作步骤。”

最后，那个年轻工程师在手机上看到的，就是一段非常具体的指导，甚至会提到上次在山西处理这个问题的王工的一些特别注意的事项，比如“先别急着换泵，先检查一下三号滤芯是不是被污染了，上次就是这个问题”。

你看，整个过程，信息是闭环的，答案是可追溯的，内容是绝对专业的。这才是RAG在工业界真正的用武之地。它不是要造一个什么都懂的“博士”，而是要打造一个只懂你公司业务的“资深员工”。

这种系统，对算力的要求真的有那么夸张吗？几十个G的文档做向量化，一台好点的服务器就搞定了。成本的大头可能是大模型的调用费，但你想想，它给公司省下的时间、避免的错误、传承的经验，这点调用费算什么？

想明白了这个，你就知道劲儿该往哪儿使了。别老盯着那些最新的模型、最复杂的算法，RAG是个系统工程，很多时候成败在那些看起来“没技术含量”的地方。

首先是源头，也就是数据处理。垃圾进，垃圾出。你拿到的PDF是两栏的，里面的表格怎么抽？一个长句子被分页符硬生生切开了怎么办？这些东西处理不好，你后面用再牛的Embedding模型都没用。还有就是文本块（Chunk）切分，这是个大学问。按固定字数切是最傻的，效果也最差。能不能按章节、按段落，甚至是用AI模型去理解语义来切？这块的活儿很脏很累，但效果提上来了，比你换个模型参数的回报大得多。

然后是检索过程，这里面门道很多。Embedding模型不是越大越好，有时候一个在你的专业语料上finetune过的中等模型，效果比通用的大家伙好得多。现在比较火的像[[bge]]-large-zh-v1.5之类的，可以拿来当基线。检索也不光是向量检索一条路，很多时候混合搜索，也就是把向量和传统的BM25这种关键词搜索结合起来，效果会好一大截。因为像设备型号、人名这种东西，关键词匹配比向量模糊查找要准得多。

再往深了说，你还可以
1. 搞查询重写（Query Rewriting）。用户问的问题通常很随意，你可以先让大模型把用户的问题“翻译”成更适合机器检索的精准问题，再去搜。
2. 还有[精召回（精排）](精召回（精排）.md)重排（Reranking），就是向量检索初步召回20条结果，再用一个更复杂的模型对这20条结果进行精细化打分排序，把最好的那几条喂给大模型。这些都是能显著提升效果的“骚操作”。

**最关键的，你得有套评估体系。**

这是区分草台班子和正规军的地方。你得搞一批真实的问答数据，用它们来量化你的系统效果。比如召回的上下文准确率怎么样？大模型生成的答案有没有瞎编（幻觉）？答案本身有没有解决用户的问题？没有评估，你所有的优化都是凭感觉，今天调个参感觉变好了，明天换个模型又感觉变差了，这活儿就没法干了。

你现在犹豫，主要是两个点：

一个是规模。很多人觉得 RAG 必须上到全网检索的量才能叫“真 RAG”，其实完全没必要。你倒回去想，用户需求是不是全域都有？很多情况下就在一个小圈子里找信息，范围越精准，模型越好调。

另一个是 ROI。学术研究想发重量级论文，还想直接比拼大厂的性能，很可能从数据处理到硬件资源都会被卡死。但如果是为了产品落地，尤其是有明确领域和用户群的，那 RAG 在体验提升上的回报是非常明显的。

所以我的建议是，不要把一开始的精力全压在规模和硬件上，先跑通一个小规模高质量的端到端流程，把检索、模型融合、响应延迟先调顺。后面的优化可以通过改检索策略、做 re-ranking、embedding 微调来拉效果，而不是一味堆数据和算力。

对了，你要是真想学习和落地，可以先摸几个靠谱的工具，像Haystack这种工业级框架，LangChain搭Milvus也挺成熟，另外ColBERT和一系列 LLM reranker 的新方法，可以在没有大规模扩容的情况下显著提效果。

RAG这摊子东西，变化很快，但核心思想还是信息检索（IR）那一套。想搞明白，底子得扎实。

如果你想搞清楚大模型怎么跟外部知识库结合、怎么在实际业务里用起来，字节RAG实践手册就是很好的参考。

下载地址：字节跳动6万字RAG 实践手册流出...（纯free，自由获取）

它不是只讲概念，还从工程实践的角度拆了整个流程：数据怎么准备、知识库怎么构建、检索和生成怎么配合、性能和延迟怎么优化。

不要被那种“我需要跟 GPT-4RAG 一样牛”的心态绑住，大多数真实场景根本用不到那个规模，但一个小范围做得有深度的 RAG，照样能把体验拉得比单纯调用 LLM 强很多。

RAG是现在这个阶段，能把大模型的能力快速、低成本、安全可控地应用到具体业务里最好的方式。这个领域坑很多，但也意味着机会很多。你现在进来，正是时候。


动手去干吧。



___

本文的核心观点总结如下：

1. **传统RAG存在的问题：**
    
    - **知识结构空心化**：传统RAG依赖于向量相似度检索，无法有效处理未直接提及关键词但存在关联的知识片段，导致知识结构空心化，难以有效建立知识之间的复杂联系。
    - 例如，与高血压治疗密切相关但未直接提及关键词的文本难以被检索。
2. 逻辑推理链条断裂：
    
    - 传统RAG仅依靠文本相似度，缺乏逻辑推理和链式推理能力，无法进行深度逻辑推导。
3. TopK截断问题：
    
    - RAG通常只保留TopK个最相似片段，可能遗漏重要的但相似度稍低的关键信息，无法提供完整视角和综合分析。
4. 为什么引入知识图谱？
    
    - 知识图谱可以捕捉实体之间的复杂关系，提供立体化的知识结构。
    - 提高跨领域知识调用、逻辑推理和聚合统计的能力，弥补RAG的缺陷。
    - 例如，医疗领域中疾病、症状、治疗之间的关系，或电影领域演员与作品之间的关系，都能借助图谱高效推理。
5. 图谱+RAG需要重点关注的几个问题：
    
    - 如何在LLM辅助下人为可控地构建知识图谱，避免数据冗余和噪音；
    - 如何处理知识实体的重复（如同义词）问题；
    - 考虑知识图谱的适用规模、领域和成本问题。
6. RAGFlow中图谱+RAG的实践路径：
    
    - RAGFlow通过GraphRAG框架，将知识图谱的结构性优势与RAG相结合，显著提升知识检索的准确性和智能化程度；
    - 提供了灵活的嵌入模型、分块策略、页面排名、召回增强（RAPTOR）等机制来提升检索质量。

总而言之，文章强调了传统RAG存在的局限性，提出通过引入知识图谱构建更深入的知识理解与推理能力，以实现更加智能、精准的AI知识应用。

___

LightRAG、GraphRAG和普通RAG系统在多个方面存在显著差异：

# 知识表示

**普通RAG**：使用简单的向量存储来索引和检索信息，通常将文档分割成块并存储在向量数据库中[4]。

**GraphRAG**：利用知识图谱来表示信息，捕捉实体之间的复杂关系和语义连接[4]。

**LightRAG**：结合了知识图谱和嵌入式检索系统的优点，使用图结构和双层检索范式[5]。

# 检索效率

**普通RAG**：在处理大规模数据集时可能面临效率和可扩展性问题[4]。

**GraphRAG**：通过图结构提高了检索效率，但社区遍历可能仍然耗时[5]。

**LightRAG**：采用双层检索策略，显著提高了检索速度和效率，同时降低了计算成本[1][5]。

# 上下文理解

**普通RAG**：可能难以处理需要复杂推理的查询[4]。

**GraphRAG**：通过知识图谱提供更丰富的上下文理解[2]。

**LightRAG**：结合局部和全局检索，提供全面的上下文理解[1]。

# 可解释性

**普通RAG**：通常缺乏可解释性[2]。

**GraphRAG**：提供更好的可解释性，可以追踪信息来源[4]。

**LightRAG**：保留了GraphRAG的可解释性优势[5]。

# 适应性和更新

**普通RAG**：更新知识库可能需要重新索引整个数据集。

**GraphRAG**：更新知识需要重建整个图结构，耗时较长[3]。

**LightRAG**：支持增量更新，可以快速适应新数据，无需大规模重建索引[1][3]。

# 查询处理

**普通RAG**：主要依赖于相似性搜索。

**GraphRAG**：可以处理更复杂的查询，支持多跳推理[4]。

**LightRAG**：通过双层检索提供多样化的答案，能更好地处理复杂查询[3][5]。

总体而言，LightRAG和GraphRAG相比普通RAG系统在处理复杂查询、提供上下文理解和可解释性方面表现更佳。而LightRAG在效率、适应性和成本效益方面又优于GraphRAG，使其成为一个强大而灵活的RAG解决方案[1][3][5]。

Citations:  
[1]https://aidisruptionpub.com/p/lightrag-is-open-sourced-lightweight  
[2]https://www.crestdata.ai/blogs/unveiling-the-power-of-graph-rag-revolutionizing-conversational-ai-and-knowledge-retrieval  
[3]https://hackernoon.com/lightrag-is-it-a-simple-and-efficient-rival-to-graphrag  
[4]https://www.falkordb.com/blog/what-is-graphrag/  
[5]https://learnopencv.com/lightrag/

# 动态章节检索

我们很高兴介绍一种新的检索增强生成（RAG）技术——动态章节检索💫——这种技术确保您可以从文档中检索完整的连续章节，而不仅仅是简单的碎片化块。

这是我们社区在多文档RAG挑战中听到的一个主要问题——传统的RAG方法返回的是没有考虑到文档整体背景的碎片化上下文。我们的方法允许您从一个"简单"的分块技术开始（例如按页面），然后进行后处理流程来添加章节/子章节的元数据。

之后，您可以像GraphRAG那样进行检索（两阶段检索）：首先检索块，然后查找附加的章节元数据，最后执行第二次检索，返回所有匹配该章节ID的块。

这种方法有效解决了RAG系统在处理大文档时，由于将文档分割成小块进行检索导致的上下文断裂问题，实现了基于文档结构的智能检索，提供了更为完整的语义上下文。

我将为您调研目前主流的大模型 RAG（Retrieval-Augmented Generation）技术的最新趋势、开源的最佳实践方案、不同大模型（如 GPT-4、Claude、Gemini 等）的对比分析，以及知识库 AI 问答的最佳实践和实际落地效果。

研究内容将涵盖：

- RAG 技术的发展趋势及最新进展
- 目前主流的 RAG 框架、向量数据库（如 FAISS、Milvus、Weaviate）及其最佳实践
- 主流大模型在 RAG 任务中的对比（准确性、性能、可扩展性）
- 知识库 AI 问答的开源方案（LlamaIndex、LangChain、Haystack等）
- 最佳实践，包括数据存储、检索、生成的全流程及实际落地案例

我将整理这些信息并尽快提供调研报告。

# RAG 技术综述与最新进展

**检索增强生成**（**RAG**，Retrieval-Augmented Generation）是近年来大模型应用的热门技术，用于将大型语言模型（LLM）的强大生成能力与**外部知识检索**相结合。其核心思想是在生成回答时，先根据用户查询从**知识库**中检索相关信息，将检索结果作为上下文提供给模型，从而提高回答的准确性和时效性 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Lewis%20and%20colleagues%20developed%20retrieval,in%20the%20latest%20technical%20details)) ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Retrieval,That%20builds%20trust))。下面我们将分别介绍RAG的最新研究趋势、主流开源工具和实践、大模型在RAG任务中的表现对比，以及知识库问答的最佳实践和实际应用案例。

## RAG 技术的最新趋势

**1. 减少幻觉，增强可控性：** RAG 的提出源于大型模型在知识密集任务上的局限。早在 2020 年，Lewis 等人发表论文提出将预训练生成模型与可检索的非参数知识结合，即在模型推理时动态检索外部文档 ([[2005.11401] Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks](https://arxiv.org/abs/2005.11401#:~:text=remain%20open%20research%20problems.%20Pre,We%20compare%20two%20RAG))。这样做一方面让模型能够引用最新的精确资料，避免仅依赖参数中固化的旧知识；另一方面还能为模型回答提供出处，增加结果的**可验证性**和用户信任 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Retrieval,That%20builds%20trust))。RAG 被证明在开放域问答等任务上显著减少了模型"胡编"现象（幻觉），其生成的答案更具体、有根据 ([[2005.11401] Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks](https://arxiv.org/abs/2005.11401#:~:text=formulations%2C%20one%20which%20conditions%20on,only%20seq2seq%20baseline))。Meta的博客指出，开发者只需少量代码就能实现基本的RAG管道，比起重新训练模型更快捷低成本 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Another%20great%20advantage%20of%20RAG,as%20five%20lines%20of%20code))。这使得RAG在工业界迅速流行，成为许多生成式AI应用的默认范式之一 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Patrick%20Lewis%2C%20lead%20author%20of,the%20future%20of%20generative%20AI))。

**2. 检索与推理的新研究：** 随着技术发展，RAG 本身也在不断演进，近期有几个显著趋势：

- **多跳检索与推理：** 传统RAG多用于单轮检索回答单一问题，但现实中许多查询需要**多跳推理**——结合多条证据链回答复杂问题。最新研究发现，现有RAG系统在多跳问答上表现不佳 ([MultiHop-RAG: Benchmarking Retrieval-Augmented Generation for Multi-Hop Queries | OpenReview](https://openreview.net/forum?id=t4eB3zYWBK#:~:text=mitigating%20LLM%20hallucinations%20and%20enhancing,an%20English%20news%20article%20dataset))。为此有学者构建了专门的基准（如 _MultiHop-RAG_ 数据集）来评测当需要跨文档、多步推理时RAG的能力，并指出当前方法在检索相关文档和综合推理方面都有不足 ([MultiHop-RAG: Benchmarking Retrieval-Augmented Generation for Multi-Hop Queries | OpenReview](https://openreview.net/forum?id=t4eB3zYWBK#:~:text=mitigating%20LLM%20hallucinations%20and%20enhancing,an%20English%20news%20article%20dataset)) ([MultiHop-RAG: Benchmarking Retrieval-Augmented Generation for Multi-Hop Queries | OpenReview](https://openreview.net/forum?id=t4eB3zYWBK#:~:text=of%20MultiHop,benchmarking%20code%20publicly%20available%20via))。这一方向促使研发更智能的检索策略，例如让LLM迭代询问或分解问题检索，或引入**推理链+工具检索(ReAct)**的Agent方法 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=process%20blog))。
    
- **融合结构化知识：** 将**知识图谱**等结构化知识融入RAG也是新趋势之一。例如领英（LinkedIn）提出了将历史客服工单构造成知识图谱，再结合RAG进行问答的方法 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Instead%20of%20treating%20the%20corpus,segmentation%20and%20improves%20retrieval%20accuracy))。用户提问时，系统先从知识图谱中解析并提取相关子图，然后再生成答案。这种**图谱增强的RAG**缓解了纯文本检索的片段化问题，提高了检索准确率 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=account%20intra,segmentation%20and%20improves%20retrieval%20accuracy))。据报道，该方案已部署于领英客服系统，将单个工单的解决时间中位数降低了28.6% ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Instead%20of%20treating%20the%20corpus,segmentation%20and%20improves%20retrieval%20accuracy))。微软研究等也提出了**GraphRAG**等框架，将文档内容抽取成语义网络后再检索，以实现更深层的关联回答 ([Project GraphRAG - Microsoft Research](https://www.microsoft.com/en-us/research/project/graphrag/#:~:text=GraphRAG%20,network%20analysis%2C%20and%20LLM)) ([Welcome - GraphRAG](https://microsoft.github.io/graphrag/#:~:text=The%20GraphRAG%20process%20involves%20extracting,these%20communities%2C%20and%20then))。
    
- **长上下文 vs. 检索：** 一个引人关注的发展是LLM上下文长度的大幅提升。有观点探讨在上下文窗口超大的模型中，是否可以直接将整个知识库嵌入上下文，减少实时检索步骤。例如谷歌的 Gemini 1.5 模型据称支持**多达200万Token**的上下文 ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=contexts,of%20documents%3B%20in%20this%20scenario))——相当于可直接容纳数百篇文档 ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=We%20designed%20our%20evaluation%20suite,context%20models%20really%20replace%20retrieval))。在这种情况下，某些规模适中的知识库可以预先填入模型上下文中供其直接生成答案，理论上**无需查询索引** ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=boast%20a%20context%20length%20of,context%20models%20really%20replace%20retrieval))。近期有论文提出了"缓存增强生成（Cache-Augmented Generation, **CAG**）"范式，利用长上下文模型预加载所有相关知识，将检索转换为一次性的上下文缓存，从而**免除每次查询时的检索延迟** ([Don’t Do RAG: When Cache-Augmented Generation is All You Need for Knowledge Tasks](https://arxiv.org/html/2412.15605v1#:~:text=Retrieval,the%20model%20utilizes%20these%20preloaded)) ([Don’t Do RAG: When Cache-Augmented Generation is All You Need for Knowledge Tasks](https://arxiv.org/html/2412.15605v1#:~:text=runtime%20parameters,superior%20results%20with%20reduced%20complexity))。实验表明，在知识规模有限的场景下，CAG 能取得与传统RAG相当的效果，同时简化系统架构 ([Don’t Do RAG: When Cache-Augmented Generation is All You Need for Knowledge Tasks](https://arxiv.org/html/2412.15605v1#:~:text=runtime%20parameters,superior%20results%20with%20reduced%20complexity))。不过，对于大规模知识库，动态检索仍不可或缺；而且目前长上下文模型往往在**推理准确性**上稍逊于结合检索的传统方案 ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=Gemini%201,up%20to%202%20Million%20Tokens))。例如，Databricks 的内部基准比较了 OpenAI 和 Google 新一代长上下文模型，发现**Gemini 1.5** 即使将超长上下文填满，其在复杂金融问答等任务上的准确率仍低于需检索的 GPT-4 系统 ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=Although%20the%20overall%20answer%20correctness,4o%20models%20up%20to))。由此业界倾向于将长上下文视为RAG的补充：即尽可能利用大模型的上下文容量一次提供更多检索结果，但当知识规模远超上下文窗口时，检索机制仍是必要且高效的。
    
- **更广泛的应用场景：** RAG 的应用已从早期的开放域问答扩展到各行各业。几乎任何需要将专有数据与LLM结合的场景都在探索RAG ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=For%20example%2C%20a%20generative%20AI,assistant%20linked%20to%20market%20data))。从医疗助手结合最新医学文献、金融分析助手关联市场数据，到企业将内部政策、产品文档构建知识库供客服和员工查询 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=For%20example%2C%20a%20generative%20AI,assistant%20linked%20to%20market%20data))。这也催生了云服务商提供RAG即服务的方案，例如 AWS、Azure 等均推出了让大模型安全访问企业内部数据的功能 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=The%20broad%20potential%20is%20why,and%20Pinecone%20are%20adopting%20RAG))。总的来看，RAG 正从学术概念走向工业实践的中心舞台，相关论文已达数百篇，许多商业服务围绕这一思想构建，可以说RAG代表了生成式AI的重要发展方向 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Patrick%20Lewis%2C%20lead%20author%20of,the%20future%20of%20generative%20AI))。
    

## 主流 RAG 开源工具与实践

RAG 技术的实现涉及到**向量检索**、**存储**和**大模型调用**等多个环节。得益于开源社区的努力，如今从底层向量数据库到高层应用框架，都有成熟的工具可用。下面介绍一些主要的开源组件和实践建议：

### 向量数据库与检索引擎

大规模**向量相似度检索**是 RAG 的基础。目前最常用的开源向量数据库/引擎包括：

- **Faiss：**由 Facebook AI Research 开发的高效相似度搜索库 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=match%20at%20L133%20Faiss%20is,contains%20supporting%20code%20for%20evaluation))。Faiss 提供多种近似最近邻（ANN）搜索算法，支持亿级向量检索，并针对GPU做了优化 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=match%20at%20L133%20Faiss%20is,contains%20supporting%20code%20for%20evaluation)) ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=%E2%80%A2%20Faiss%20is%20widely%20used,support%20in%20the%20AI%20community))。需要注意它本身是一个库而非完整的数据库系统，主要负责建立和查询向量索引，数据持久化和分布式管理需要结合其他工具 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=match%20at%20L141%20Note%3A%20FAISS,to%20achieve%20fast%20similarity%20search))。由于性能强劲，Faiss 被广泛应用于学术研究和工业研发中，常作为许多向量库的核心引擎或底层模块。
    
- **Milvus：**由 Zilliz 主导的开源向量数据库，定位于**分布式大规模**向量数据处理 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Milvus%20is%20an%20open%20source,integrated%20with%20various%20ML%20frameworks))。Milvus 原生支持向量的创建、读写和近似搜索，可无缝扩展到多节点以处理海量数据 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=%E2%80%A2%20Good%20scalability%3A%20supports%20distributed,scale%20data))。它支持多种索引类型（如 HNSW、IVF 等）和 GPU 加速，提供类似SQL的查询接口，易于与机器学习框架集成 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Milvus%20is%20an%20open%20source,integrated%20with%20various%20ML%20frameworks))。Milvus 注重**高可用性和伸缩性**，适合企业在生产环境中部署大规模向量检索服务。
    
- **Weaviate：**一款云原生的向量数据库，特色是支持 **GraphQL 查询** 和融合非结构化/结构化数据的**混合搜索** ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Weaviate%20is%20a%20GraphQL,It%20seamlessly%20stores%20not)) ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=For%20semantic%20and%20hybrid%20search%3A,both%20structured%20and%20unstructured%20data))。Weaviate 不仅存储向量，还可以存储原始对象和其属性，允许在语义向量搜索的同时结合结构化过滤和知识图谱等操作 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Weaviate%20is%20a%20GraphQL,It%20seamlessly%20stores%20not))。它内置了文本、图像等多模态向量化模块，开箱即用地支持语义搜索和问答，非常适合需要**语义理解**和**知识推理**的应用场景 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=%E2%80%A2%20Weaviate%20is%20an%20open,knowledge%20graphs%20and%20semantic%20search))。
    
- **ChromaDB：**新兴的开源向量数据库，主打易用性和与Python生态的集成。Chroma 提供简单的Python接口，可方便地将嵌入向量存储、查询，并支持按元数据过滤。它在文档问答类应用中颇受欢迎，也是LangChain的默认向量存储后端之一。虽然Chroma目前主要作为本地嵌入数据库使用，但其设计也考虑了分布式扩展的能力。
    
- **Qdrant：**用Rust实现的高性能向量数据库，强调**高并发和低延迟**。Qdrant支持HNSW等ANN算法，提供gRPC/REST接口，也带有Web管理界面，便于监控和调优。由于Rust天生的性能优势，Qdrant在延迟敏感的在线服务中表现出色，社区反馈其在十亿级向量上仍能保持检索性能。
    

_（以上工具均在近期评选的"2024年Top5开源向量数据库"榜单中有所提及 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Discover%20the%20top%205%20open,Explore%20their%20features%20and%20benefits))。）_

**最佳实践：** 在使用向量检索时，需根据场景选择合适的方案。一般来说，**向量索引**可以采用扁平（Flat）索引获得最高准确率，但在数据规模很大时检索速度可能不足，这时HNSW、IVF等近似方法可大幅加速查询 ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=%E2%80%A2%20Faiss%20is%20widely%20used,support%20in%20the%20AI%20community))。像 Milvus、Weaviate 这样的系统已经集成多种索引类型，可根据数据规模和查询QPS需求调优。另外，应充分利用元数据：比如为每段文本嵌入向量附加来源、主题标签、时间戳等，在查询时通过元数据过滤可以提高相关性和新鲜度（例如只检索最新的文件）。在构建知识库时，预先对文档进行**分块**和清理也很关键，这一点在后文知识库QA最佳实践部分详述。

### 知识库 RAG 框架与管道

在底层存储确定后，还需要将**LLM、检索、数据**串联成完成任务的流水线。为此，业界涌现了多个开源框架，封装了常用的RAG流程，方便开发者快速搭建**知识库问答**等应用：

- **LangChain：**最受欢迎的LLM编排框架之一。LangChain提供了丰富的组件来"链式"调用大模型和各类工具 ([chatbot - Differences between Langchain & LlamaIndex - Stack Overflow](https://stackoverflow.com/questions/76990736/differences-between-langchain-llamaindex#:~:text=LangChain))。针对RAG，LangChain可以方便地集成向量数据库、检索器和LLM，将用户问题经过嵌入查询后与检索到的文档一起传给模型回答 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=generation%20combines%20LLMs%20with%20embedding,models%20and%20vector%20databases))。由于组件丰富，LangChain易于构建复杂应用（如在检索基础上再调用计算工具等）。许多厂商的参考方案（如NVIDIA的RAG Blueprint）也使用了LangChain来组织检索流程 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=generation%20combines%20LLMs%20with%20embedding,models%20and%20vector%20databases))。LangChain的优点是**通用性**强、生态活跃，但其抽象层次较高，对简单场景来说可能略显繁重。
    
- **LlamaIndex：**原名 GPT Index，是专为**本地知识检索**设计的轻量框架。LlamaIndex聚焦于**数据的摄取、索引构建和查询**，提供了简洁的接口将自定义数据接入LLM ([chatbot - Differences between Langchain & LlamaIndex - Stack Overflow](https://stackoverflow.com/questions/76990736/differences-between-langchain-llamaindex#:~:text=LlamaIndex%2C%20,of%20custom%20data%20into%20LLMs)) ([chatbot - Differences between Langchain & LlamaIndex - Stack Overflow](https://stackoverflow.com/questions/76990736/differences-between-langchain-llamaindex#:~:text=One%20key%20feature%20of%20LlamaIndex,be%20easily%20queried%20like%20so))。它内置多种索引结构（如向量索引、列表索引、树索引、知识图谱索引等），方便针对不同数据类型优化检索效果。与LangChain相比，LlamaIndex更专注于**检索和文档管理**，对接各种向量库也更直接。如果应用需要复杂的LLM对话和工具组合，仍可将LlamaIndex与LangChain配合使用：前者管数据索引，后者管对话流程。有开发者评价LlamaIndex在需要高效索引和检索的场景下更为简洁易用 ([chatbot - Differences between Langchain & LlamaIndex - Stack Overflow](https://stackoverflow.com/questions/76990736/differences-between-langchain-llamaindex#:~:text=It%20should%20be%20clear%20to,LLMs%20stick%20with%20only%20LangChain)) ([chatbot - Differences between Langchain & LlamaIndex - Stack Overflow](https://stackoverflow.com/questions/76990736/differences-between-langchain-llamaindex#:~:text=your%20specific%20use%20case,LLMs%20stick%20with%20only%20LangChain))。
    
- **Haystack：**由 deepset 开源的成熟 NLP 问答框架。Haystack 最早（2017年）用于搭建经典的**检索-阅读器**架构，即先用关键词/向量等检索文档，再用一个阅读器模型（如BERT问答）从文档中提取答案。随着LLM兴起，Haystack也增加了生成式模型支持，能让检索结果通过LLM生成答案。Haystack的特色在于其**模块化和成熟度**：组件包括多种文档存储（Elasticsearch、FAISS 等）、检索器和读者/生成模型，支持REST API部署和流水线可视化。其社区和文档也相对完善，对企业级应用的支持出色 ([LlamaIndex vs LangChain vs Haystack | by Hey Amit - Medium](https://medium.com/@heyamit10/llamaindex-vs-langchain-vs-haystack-4fa8b15138fd#:~:text=While%20not%20as%20large%20as,level%20search))。如果需要构建**可定制、可扩展且易调试**的问答系统（尤其是结合传统搜索的应用），Haystack 是一个稳健的选择。
    
- **其余框架和工具：** 除上述外，还有例如 **GPT-Index Hub**（LlamaHub，提供各种数据加载插件）、**Prompt-Engineer** 类库（帮助设计检索后提示模板）以及许多在线服务（如 OpenAI 的 Retrieval Plugin、Cohere Rerank API 等）可供使用。选型时应考虑团队技术栈和项目需求，例如纯Python环境下Chroma+LlamaIndex可能最快上手，而已有ElasticSearch部署的则可直接用其新推出的dense vector功能配合Haystack等。
    

### 知识库问答的全流程实践

构建一个知识库问答（例如企业文档问答Chatbot）通常需要如下几个主要步骤：

1. **数据处理与存储：**将原始知识库的数据（如文档、网页、PDF、Wiki等）进行提取清洗，转换成纯文本格式。然后根据内容语义将文本**拆分成合适的段落块**（chunks），每块几百字为宜，以避免超出模型上下文长度 ([Tutorial: ChatGPT Over Your Data](https://blog.langchain.dev/tutorial-chatgpt-over-your-data/#:~:text=1,select%20the%20most%20relevant%20chunks))。拆分时尽量在语义断点处分段，并可设置一定重叠以防止信息割裂。实际项目中，经常需要处理各种复杂格式和跨文件引用，这是个不小的工程。例如 RBC（加拿大皇家银行）的团队提到，他们构建内部政策RAG系统时，需要解析分散于网页、PDF、表格中的政策文件并拆成片段，如何高效解析和分块是一大挑战 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=and%20providing%20information%20sources))。完成拆分后，将每个文本块存入向量数据库并保存原始来源信息等元数据备用。
    
2. **向量化与索引构建：**为每个文本块生成向量表示（文本嵌入）。可使用预训练的Embedding模型，例如 OpenAI 的 _text-embedding-ada-002_，或开源的 _SentenceTransformers_、_InstructorXL_ 等，根据精度需求和成本选择。生成向量后，写入向量索引以支持相似度检索。向量库会根据选择的索引类型（如 HNSW）建立内部的数据结构便于后续快速查找。比如 Thomson Reuters 在其客户支持RAG系统中，就将知识库文章切分后嵌入向量，并存入向量数据库以待查询 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=The%20system%20uses%20embeddings%20to,to%20get%20the%20best%20matches))。
    
3. **查询检索：**当用户提出问题时，首先对**查询进行向量化**（用同一Embedding模型）。然后在向量数据库中执行相似度搜索，找到最相关的几个文本块作为**检索结果** ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=The%20system%20uses%20embeddings%20to,to%20get%20the%20best%20matches))。通常会取前 _k_ 篇文档片段（例如 top-5），必要时可以增加**重排序**步骤：例如采用一个跨编码器（Cross-Encoder）对候选片段重新打分排序，确保与查询高度相关的内容排在前列。检索阶段也可以结合业务规则过滤，例如只从特定类别文档中检索，或优先最新更新的内容等。
    
4. **生成回答：**将检索到的文档片段与用户问题一道，构造成提示（Prompt）发送给LLM，让其据此生成最终答案。通常的提示格式会说明"以下是知识库信息"和"用户提问"，要求模型只依据提供的信息回答、不编造额外内容，并可能在答案末尾附上引用来源。大型模型（如GPT-4）在有充分上下文时，能够**综合多文档信息**并给出连贯答案。例如在Thomson Reuters的案例中，检索到的内容传递给序列到序列的生成模型后，模型对答案进行了润色组织，并**结合检索知识定制回答**，极大提高了准确性，同时大幅减少了幻觉 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=to%20get%20the%20best%20matches))。很多实现中还会让模型输出答案时标注引用片段的编号，便于后续展示来源。
    
5. **后处理与质量控制（可选）：**为了提高答案可靠性，生产系统中常加入**结果校验和防护**机制。例如 DoorDash 的RAG客服机器人引入了"LLM Guardrail"和"LLM Judge"两个模块 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Delivery%20support%20chatbot))：前者在生成前约束模型行为（避免不当内容，限定回复格式等），后者在生成后审查回答质量。如果Judge模型发现回答与事实不符或不满足要求，可以拒绝或让主模型二次尝试。这种"双模型"架构在需要高可靠性的场景下很有价值。此外，在开放域场景可能还需实时检测敏感信息泄露、执行模糊匹配确保答案包含必要关键词等。通过这些后处理，可以将RAG系统输出的错误率降到更低。
    
6. **持续更新迭代：**知识库并非一成不变。最佳实践是设计**定期更新流程**：如每日/每周抓取新的文档，增量地生成嵌入并更新向量索引 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=The%20company%20shared%20how%20it,removed%20from%20their%20source%20location))。对于删除或过期的内容也要从索引中移除，保持知识库的新鲜和精确。系统还需要定期评估性能，监控回答的准确率和用户反馈，并据此调整。例如可以构造一批测试问答对，用GPT-4等作为评审参考来给出正确率、引用正确性等指标 ([LlamaIndex: RAG Evaluation Showdown with GPT-4 vs. Open-Source Prometheus Model — LlamaIndex - Build Knowledge Assistants over your Enterprise Data](https://www.llamaindex.ai/blog/llamaindex-rag-evaluation-showdown-with-gpt-4-vs-open-source-prometheus-model-14cdca608277#:~:text=source%20Prometheus%20model%20has%20recently,alternative%20for%20such%20evaluation%20tasks)) ([LlamaIndex: RAG Evaluation Showdown with GPT-4 vs. Open-Source Prometheus Model — LlamaIndex - Build Knowledge Assistants over your Enterprise Data](https://www.llamaindex.ai/blog/llamaindex-rag-evaluation-showdown-with-gpt-4-vs-open-source-prometheus-model-14cdca608277#:~:text=1,the%20answer%20to%20the%20query))。许多团队会持续收集真实用户提问和模型响应，人工标注后用于分析哪些问答不佳，找出改进方向（如需要增加哪些知识、调整提示模板或升级模型等）。通过这种闭环迭代，知识库问答系统的效果会越来越好。
    

## 不同大模型在 RAG 任务中的表现对比

将RAG应用于具体任务时，不同的底层大模型（LLM）在**准确性、上下文处理能力、推理性能**等方面存在差异。以下对主流的一些大模型在RAG场景下的表现进行分析：

- **GPT-4（OpenAI）：** GPT-4 被普遍认为是当前综合能力最强的通用LLM，在RAG任务中也表现出色。它拥有卓越的语言理解和逻辑推理能力，因而给定检索文档后，GPT-4 往往能够**准确整合信息并生成高质量答案**。在Databricks的长期上下文RAG基准测试中，一个优化推理版的GPT-4（GPT-4o）在多领域问题上取得了最高的答案正确率，平均分数领先于同类模型 ([Long Context RAG Performance of LLMs | Databricks Blog](https://www.databricks.com/blog/long-context-rag-performance-llms#:~:text=gpt))。即使将上下文扩展到上万Token，GPT-4的表现依然稳健 ([Long Context RAG Performance of LLMs | Databricks Blog](https://www.databricks.com/blog/long-context-rag-performance-llms#:~:text=0)) ([Long Context RAG Performance of LLMs | Databricks Blog](https://www.databricks.com/blog/long-context-rag-performance-llms#:~:text=0))。相对而言，GPT-4 的主要局限在于**上下文窗口**（标准版约8K，扩展版也仅32K左右）和调用成本较高。这意味着对于超长文档集，GPT-4需要分批检索、多轮交互，工程上稍复杂。但在准确性要求高的场景，GPT-4结合RAG几乎是"金标准"。许多评测显示，GPT-4 + 检索的方案能接近甚至超过人类专家水平 ([Llama 2 vs. GPT-4: Nearly As Accurate and 30X Cheaper](https://www.anyscale.com/blog/llama-2-is-about-as-factually-accurate-as-gpt-4-for-summaries-and-is-30x-cheaper#:~:text=%2A%20%60gpt))。综合考虑，GPT-4 适合用于追求最佳效果的RAG应用，比如法律、医学等对正确率要求极高的领域。
    
- **Claude 2 / Claude 100K（Anthropic）：** Claude 是Anthropic推出的对话大模型，以**超长上下文**能力见长。最新的Claude 2支持高达100K Token的上下文窗口，这在实践中允许一次提供非常多的检索文本给模型，从而减少分段提问的需要。在RAG任务中，Claude的**理解和综合能力**也相当强劲。据Databricks报告，Claude-3.5（Claude 2 的一个版本）在中等长度上下文下的问答准确率与GPT-4相当，甚至某些情况下略高 ([Long Context RAG Performance of LLMs | Databricks Blog](https://www.databricks.com/blog/long-context-rag-performance-llms#:~:text=claude))。当上下文增加时，Claude的性能也能较平稳地扩展，只是在逼近其最大上下文（如100K）时准确率略有下降 ([Long Context RAG Performance of LLMs | Databricks Blog](https://www.databricks.com/blog/long-context-rag-performance-llms#:~:text=0))。Claude 的一个特点是对指令和对话的顺从度高，生成内容风格上更倾向于详细解释，这在知识库QA中有利于提供**包含背景的友好回答**。另外，Claude对**语境的记忆和一致性**处理很好，适合多轮深入的知识对话。在企业应用中，如果需要让模型一次性"读"大量材料再回答，Claude是理想选择。不过Claude目前获取渠道有限，需要通过API且费用不低。
    
- **Gemini（Google）：** Gemini 是谷歌推出的新一代多模态大模型系列，其中以超大的上下文窗口和强化推理能力为卖点。据报道，Gemini 1.5版就支持最高2百万Token的上下文长度 ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=contexts,of%20documents%3B%20in%20this%20scenario))；后续的Gemini 2据称在部分版本中也提供了100万Token级别的窗口 ([Gemini 2.0 Flash vs GPT 4o: Which is Better? - Analytics Vidhya](https://www.analyticsvidhya.com/blog/2023/12/chatgpt-vs-gemini-a-clash-of-the-titans-in-the-ai-arena/#:~:text=Vidhya%20www,offering%20enhanced%20capacity%20for))。如此巨大的上下文使其在RAG场景下有了不同的用武之地：**小型语料可直接放入上下文**供模型读取，几乎不需要传统的检索步骤 ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=boast%20a%20context%20length%20of,context%20models%20really%20replace%20retrieval))。然而，需要注意的是，上下文容量不等于理解能力。Databricks的测试显示，Gemini 1.5 在包含金融文档和程序文档的复杂问答（需要跨文档推理）中，正确率明显低于OpenAI的新模型系列和Anthropic的Claude ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=Although%20the%20overall%20answer%20correctness,4o%20models%20up%20to))。不过在开放域的自然问题（如NaturalQuestions数据集）上，Gemini与GPT-4等表现旗鼓相当，正确率均在0.8以上 ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=match%20at%20L298%20On%20Databricks,5%20models%20don%E2%80%99t%20have%20a))。这表明Gemini擅长存储海量信息，但在**特定领域知识和推理**上还有提升空间。随着Google不断优化模型（Gemini据称融合了AlphaGo式的推理训练），其未来版本有望在保持长上下文优势的同时，缩小与GPT-4等在推理准确度上的差距。目前来看，如果应用场景涉及**极长文档**（如整本书、全年日志）且希望尽量少分批检索，Gemini是值得关注的选择。
    
- **开源大模型（Llama 2, Mistral等）：** 开源社区的LLM近年来发展迅速，一些中大型开源模型在结合RAG后也能实现与闭源模型相媲美的效果 ([Llama 2 vs. GPT-4: Nearly As Accurate and 30X Cheaper](https://www.anyscale.com/blog/llama-2-is-about-as-factually-accurate-as-gpt-4-for-summaries-and-is-30x-cheaper#:~:text=,turbo)) ([Llama 2 vs. GPT-4: Nearly As Accurate and 30X Cheaper](https://www.anyscale.com/blog/llama-2-is-about-as-factually-accurate-as-gpt-4-for-summaries-and-is-30x-cheaper#:~:text=%2A%20%60gpt))。例如，Meta的Llama 2 70B在无检索时知识覆盖不如GPT-4，但如果配套一个高质量的知识库进行RAG，其在事实问答的准确性可逼近GPT-4水平（有实验对新闻摘要事实核对表明Llama2-70B的准确率达到81.7%，而GPT-4约85.5% ([Llama 2 vs. GPT-4: Nearly As Accurate and 30X Cheaper](https://www.anyscale.com/blog/llama-2-is-about-as-factually-accurate-as-gpt-4-for-summaries-and-is-30x-cheaper#:~:text=%2A%20%60gpt))）。考虑到开源模型可部署在本地且成本更低（Llama2-70B运行费用约为GPT-4 API的1/30 ([Llama 2 vs. GPT-4: Nearly As Accurate and 30X Cheaper](https://www.anyscale.com/blog/llama-2-is-about-as-factually-accurate-as-gpt-4-for-summaries-and-is-30x-cheaper#:~:text=,turbo))），在某些对隐私、成本敏感的场景，它们配合RAG是具吸引力的方案。又比如只有70亿参数的_Mistral 7B_模型，是最近开源的小型模型新秀。单靠参数知识它难以回答复杂问题，但社区实践证明，通过RAG赋能，Mistral-7B在特定领域的问答中也能给出可靠答案 ([RAG Pipeline With Mistral 7B Instruct Model: A step-by-step guide](https://blog.gopenai.com/rag-pipeline-with-mistral-7b-instruct-model-a-step-by-step-guide-138df378a0c2#:~:text=RAG%20Pipeline%20With%20Mistral%207B,as%20a%20reasoning%20model%2C))。开发者使用LangChain+Chroma等搭建了Mistral的RAG管道，实现了对自定义文件的问答 ([RAG Implementation with Mistral 7B and ChromaDB - GitHub](https://github.com/mickymultani/RAG-ChromaDB-Mistral7B#:~:text=RAG%20Implementation%20with%20Mistral%207B,Mistral%207B%20model%20for))。小模型的优势在于**部署灵活、响应速度快**，可以在浏览器、本地服务器甚至移动设备上运行。这使得一些边缘场景（如离线知识库查询、私有部署的智能助手）成为可能。当然，小模型的语言理解和生成能力相对有限，需要精心设计提示并严格约束在给定资料范围内回答，以避免出错。总体而言，开源模型经过RAG"增强"后，在**垂直领域知识问答**中能够达到实用的准确度，其性价比和可控性是独有的优势。
    

_(注：除了上述模型，一些新兴的定制模型专门针对RAG优化，例如VECTARA推出的"Mockingbird"声称在RAG回答质量和防幻觉上超越GPT-4和Gemini 1.5 ([Mockingbird is a RAG Specific LLM that Beats GPT 4, Gemini 1.5 ...](https://www.vectara.com/blog/mockingbird-is-a-rag-specific-llm-that-beats-gpt-4-gemini-1-5-pro-in-rag-output-quality#:~:text=Mockingbird%20is%20a%20RAG%20Specific,and%20autonomous%20agent%20use%20cases))。这些定制模型通常融入了检索相关的微调或架构改进，但多为商业产品，尚未有公开的独立评测佐证。)_

## 知识库 AI 问答的开源方案与成功案例

借助RAG技术，各行业已经构建出众多**知识库问答**应用，涵盖客服、搜索、办公助手等场景。这里我们介绍一些**开源方案**和**最佳实践**，并列举实际成功案例，以展示RAG在知识库问答中的落地效果。

### 开源方案与实践要点

**1. 框架选择与架构设计：** 开源工具方面，前文提到的 LangChain、LlamaIndex、Haystack 等都是常用的知识库QA框架。在实际项目中，往往会将它们与向量数据库结合使用，形成**端到端的流水线**。例如，可以使用 Haystack 提供的 Pipeline 接口，将 Milvus 作为文档存储，SentenceTransformer 作为嵌入模型，Flan-T5 等本地模型作为生成器，快速拼装出一个问答系统。同时需要根据需求决定架构：是走经典的"检索-阅读器"提取式问答，还是直接"检索-生成"让LLM自由生成答案（大多数现在选择后者，因为生成式更灵活）。对于要求实时性的场景，也可以考虑用 faster transformer 或量化模型来加速推理，用多线程/异步提高并发。

**2. 提示模板与上下文封装：** 在RAG模式下，设计合适的Prompt非常关键。通常会有一个**模板**将检索到的文档片段插入，并提示模型回答时参考这些资料。一个典型模板可能是：

```Java
根据以下提供的文档摘录回答问题。若无法从中找到答案，请告知无法回答，不要编造。  
文档摘录1: "...(内容)..."  
文档摘录2: "...(内容)..."  
...  
问题: <用户问题>  
回答:
```

这样明确要求模型只基于提供的内容回答，从而减少幻觉和跑题风险。在多轮对话情境下，还需要考虑**对话历史**对提示的影响，常用的方法是将历史对话压缩总结，或者让模型把新问题与历史结合后重述为独立问题 ([Tutorial: ChatGPT Over Your Data](https://blog.langchain.dev/tutorial-chatgpt-over-your-data/#:~:text=1,relevant%20documents%20for%20the%20answer))。这避免了长期对话时无关背景干扰检索。提示中也可以加入格式要求，例如让模型在每个句子后标注信息来源编号，以方便用户追溯答案出处。这些在LangChain等框架中都可以通过 PromptTemplate 模块实现定制。

**3. 文本匹配与缓存策略：** 对于某些高频问答或大文档，构建**缓存**可以提高效率。例如可以记录下用户提问到检索结果的映射，下次遇到相似问题直接复用答案或结果（需注意判断相似度阈值）。还有一种方法是对长文档预先生成分层摘要索引（如LlamaIndex的树索引），当提问涉及该文档时，先检索到摘要，再深入检索细节，提高速度的同时减少不相关段落干扰。针对实时更新的知识库，则需要有**失效缓存**机制：文档一旦更新或被移除，其相关缓存也要同步失效，确保不会返回过期答案。

**4. 评估指标与反馈学习：** 部署后的知识库QA系统需要持续评估和改进。可以引入**自动化评估**：如用一个强大的LLM充当"评分员"来评估回答的正确性和引用可靠性 ([LlamaIndex: RAG Evaluation Showdown with GPT-4 vs. Open-Source Prometheus Model — LlamaIndex - Build Knowledge Assistants over your Enterprise Data](https://www.llamaindex.ai/blog/llamaindex-rag-evaluation-showdown-with-gpt-4-vs-open-source-prometheus-model-14cdca608277#:~:text=1,the%20answer%20to%20the%20query))。LlamaIndex等提供了 RetrieverEvaluator，可以对给定问答对计算检索相关性得分 ([LlamaIndex: RAG Evaluation Showdown with GPT-4 vs. Open-Source Prometheus Model — LlamaIndex - Build Knowledge Assistants over your Enterprise Data](https://www.llamaindex.ai/blog/llamaindex-rag-evaluation-showdown-with-gpt-4-vs-open-source-prometheus-model-14cdca608277#:~:text=source%20Prometheus%20model%20has%20recently,alternative%20for%20such%20evaluation%20tasks))。除了离线评测，也应收集**用户反馈**，当用户对答案点击"有用"或"无用"时，将这些标注数据用于微调检索模块（比如训练一个查询改写或更好的rerank模型），或用于训练回答生成的奖励模型（RLHF）。一些开源项目开始探索让系统自我提升，例如当回答被判定不满足要求时，利用链式思考让模型自己再检索新的信息尝试改进。这些技术仍在发展中，但都是为了让知识库AI更精准可靠。

**5. 安全和隐私：** 在企业知识库应用中，往往涉及内部机密或用户隐私数据，必须确保**数据安全**。如果采用云端大模型API，需要做好数据脱敏或选择私有部署的模型。向量数据库也要支持访问控制，防止未经授权的查询。生成阶段还要注意避免泄露敏感信息——LLM可能在回答中无意透露超出权限的内容，必要时要设定内容过滤或规则约束模型输出。此外，RAG系统作为决策支持工具，其答案可能被人直接采用，所以在法律、医疗等高风险场景下，一定要提供**免责声明**并鼓励用户核实提供的来源。如果条件允许，可以在LLM给出答案后，自动匹配答案中的关键事实与原始资料进行二次校验，发现不一致就提醒用户慎重。

### 实际落地案例

RAG 在知识库问答方面的价值已在许多真实案例中得到验证。以下列举几个具有代表性的应用场景和成功案例：

- **客户支持 Chatbot：**外卖平台 DoorDash 开发了基于RAG的配送员支持聊天机器人。当配送员（称为Dasher）报告问题时，系统会先整理上下文对话，再从知识库中检索相关政策和流程信息，提供给LLM生成解决方案 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Delivery%20support%20chatbot))。DoorDash 的方案由三部分组成：RAG 系统负责检索答案，LLM _Guardrail_确保输出符合客服政策，LLM _Judge_则在生成后评估回答质量 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Delivery%20support%20chatbot))。这一闭环使自动回复的准确率和安全性显著提高，减少了人工客服介入的频率。
    
- **技术支持问答：**领英（LinkedIn）面临海量用户技术问题咨询，他们引入了RAG+知识图谱的方法优化客服答疑 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Instead%20of%20treating%20the%20corpus,segmentation%20and%20improves%20retrieval%20accuracy))。具体做法是将历史工单和解决方案构建成一个知识图谱（节点表示问题要素，边表示关联关系），用户提问时先解析意图，从图谱中检索相关"子图"（类似找出相关的问题案例），然后结合检索到的图谱信息由LLM生成解答 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=account%20intra,segmentation%20and%20improves%20retrieval%20accuracy))。这种融合图谱的检索缓解了纯文本检索可能遗漏上下文关系的问题。据领英披露，该系统已部署给客服团队使用，使工单解决的中位时间下降了28.6% ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Instead%20of%20treating%20the%20corpus,segmentation%20and%20improves%20retrieval%20accuracy))。这一成果表明RAG不仅能用于文档，还能与结构化知识结合，提升问答效率。
    
- **内部知识库助手：**大型企业常有繁杂的内部政策和流程文档。加拿大皇家银行（RBC）开发了名为"Arcane"的RAG问答系统，供内部员工查询公司政策 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=The%20Royal%20Bank%20of%20Canada,productivity%20and%20streamline%20customer%20support))。员工在聊天界面提出政策相关问题，Arcane 就在银行内部网页、知识库中导航检索出相关条款和说明，提炼后返回给员工 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Here%E2%80%99s%20how%20the%20Arcane%20experience,format%20and%20providing%20information%20sources))。有了这个工具，新员工也能迅速定位所需的规章，极大提升了生产力和客户响应速度。RBC 团队分享道，构建Arcane最大的难点在于处理多源异构的数据并对其分段索引，这一经验对类似有大量非结构化内部文档的企业很有借鉴意义 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=and%20providing%20information%20sources))。
    
- **智能客服与信息检索：**老牌信息提供商 Thomson Reuters 也利用RAG改善了客服流程 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Executive%20customer%20support))。他们构建了一个客服支持系统，面向客服坐席人员：坐席在与客户沟通时，可以通过聊天界面询问内部知识库某问题的解答。系统会将坐席的提问向量化，检索内部的法规、资料库文档片段，然后由生成模型整理成易于转述给客户的答案 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=The%20system%20uses%20embeddings%20to,to%20get%20the%20best%20matches))。该方案采用嵌入+向量数据库实现高速检索，并由seq2seq模型润色答案 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=to%20get%20the%20best%20matches))。这样一来，一线客服无需逐字翻阅冗长文档，就能得到准确且最新的答复，提高了客户满意度和问题解决率 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Support%20Using%20RAG%20at%20Thomson,com))。值得一提的是，Thomson Reuters 的系统在回答时还提供引用来源，确保客服人员可以点开查看原始内容以核实，增强了内部信任度。
    
- **其他行业应用：**除了上述案例，RAG 正被广泛应用于各种场景。例如，东南亚超级应用 **Grab** 将RAG驱动的LLM用于自动生成内部分析报告和执行反欺诈调查等任务，代替人工汇总数据，提升了运营效率 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=%EF%B8%8F%E2%80%8D%E2%99%80%EF%B8%8F%20Analytical%20fraud%20reports))。图片社交平台 **Pinterest** 则将RAG用于辅助数据科学家查询内部数据表结构：系统会根据用户的分析问题检索潜在相关的数据库表并推荐给用户，大幅减少了人工查找时间 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=for%20users))。金融科技公司 **Ramp** 利用RAG改进客户行业分类：将客户资料向量化匹配NAICS行业标准向量库，再由LLM生成分类建议，从而纠正了过去人工分类不一致的问题 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=To%20solve%20this%20problem%2C%20Ramp,Here%E2%80%99s%20how%20it%20works))。学术领域，哈佛商学院的教授开发了课程助手ChatLTV，预先索引了课程案例和教材，学生提问时ChatLTV检索相关内容并回答，不仅减轻教师答疑负担，还能在回答后注明出处供学生自行研读 ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=ChatLTV%20was%20trained%20on%20the,in%20private%20and%20public%20modes)) ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=To%20respond%20to%20a%20student%27s,and%20generate%20a%20quality%20score))。这些案例证明，无论是客服、数据分析、金融合规还是教育培训，RAG+大模型的组合都展现出变革传统知识获取流程的巨大潜力。
    

总而言之，Retrieval-Augmented Generation 正在从实验室走向各行各业的生产环境。通过结合向量检索和大模型，企业能够构建出**懂业务、可查证、可持续学习**的智能问答系统，极大降低了获取专业知识的门槛。当今，越来越多的公司（如 AWS、IBM、Google、Microsoft 等）都在其产品中融入了RAG理念 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=The%20broad%20potential%20is%20why,and%20Pinecone%20are%20adopting%20RAG))；开源社区的工具和模型也使构建专属的知识AI助手变得前所未有地容易。一系列成功实践表明：RAG 不仅显著提升了问答的准确性和实用性，还为企业带来了实实在在的效率收益（如响应时间缩短、人工成本降低等）。可以预见，随着研究的推进和应用的积累，RAG 将在**智能客服、知识管理、决策支持**等领域发挥更加关键的作用，成为未来生成式AI应用的标准范式之一 ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Patrick%20Lewis%2C%20lead%20author%20of,the%20future%20of%20generative%20AI))。

**参考资料：**

1. Patrick Lewis et al. _"Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks."_ NeurIPS 2020. ([[2005.11401] Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks](https://arxiv.org/abs/2005.11401#:~:text=%3E%20Abstract%3ALarge%20pre,augmented%20generation%20%28RAG%29)) ([[2005.11401] Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks](https://arxiv.org/abs/2005.11401#:~:text=pre,only%20seq2seq))
    
2. NVIDIA Blog. _"What Is Retrieval-Augmented Generation, aka RAG?"_ (2023 updated 2025). ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Retrieval,That%20builds%20trust)) ([What Is Retrieval-Augmented Generation aka RAG | NVIDIA Blogs](https://blogs.nvidia.com/blog/what-is-retrieval-augmented-generation/#:~:text=Another%20great%20advantage%20of%20RAG,as%20five%20lines%20of%20code))
    
3. Yixuan Tang, Yi Yang. _"MultiHop-RAG: Benchmarking Retrieval-Augmented Generation for Multi-Hop Queries."_ OpenReview 2024. ([MultiHop-RAG: Benchmarking Retrieval-Augmented Generation for Multi-Hop Queries | OpenReview](https://openreview.net/forum?id=t4eB3zYWBK#:~:text=mitigating%20LLM%20hallucinations%20and%20enhancing,an%20English%20news%20article%20dataset)) ([MultiHop-RAG: Benchmarking Retrieval-Augmented Generation for Multi-Hop Queries | OpenReview](https://openreview.net/forum?id=t4eB3zYWBK#:~:text=experiment%2C%20we%20examine%20the%20capabilities,code%20publicly%20available%20via%20GitHub))
    
4. LinkedIn Engineering Blog. _"Retrieval-Augmented Generation with Knowledge Graphs for Customer Service Question Answering."_ (2023). ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Instead%20of%20treating%20the%20corpus,segmentation%20and%20improves%20retrieval%20accuracy))
    
5. Quinn Leng et al. (Databricks). _"The Long Context RAG Capabilities of OpenAI o1 and Google Gemini."_ (Oct 2024). ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=We%20designed%20our%20evaluation%20suite,context%20models%20really%20replace%20retrieval)) ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=Gemini%201,up%20to%202%20Million%20Tokens))
    
6. Xiang Zhao et al. _"Don't Do RAG: When Cache-Augmented Generation is All You Need for Knowledge Tasks."_ (arXiv 2023). ([Don’t Do RAG: When Cache-Augmented Generation is All You Need for Knowledge Tasks](https://arxiv.org/html/2412.15605v1#:~:text=Retrieval,the%20model%20utilizes%20these%20preloaded)) ([Don’t Do RAG: When Cache-Augmented Generation is All You Need for Knowledge Tasks](https://arxiv.org/html/2412.15605v1#:~:text=runtime%20parameters,superior%20results%20with%20reduced%20complexity))
    
7. GPU-mart Blog. _"Top 5 Open Source Vector Databases in 2024."_ ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Discover%20the%20top%205%20open,Explore%20their%20features%20and%20benefits)) ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=match%20at%20L133%20Faiss%20is,contains%20supporting%20code%20for%20evaluation))
    
8. GPU-mart Blog. _"Top 5 Open Source Vector Databases in 2024."_ (Faiss & Milvus). ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=match%20at%20L141%20Note%3A%20FAISS,to%20achieve%20fast%20similarity%20search)) ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Milvus%20is%20an%20open%20source,integrated%20with%20various%20ML%20frameworks))
    
9. GPU-mart Blog. _"Top 5 Open Source Vector Databases in 2024."_ (Weaviate). ([Top 5 Open Source Vector Databases in 2024](https://www.gpu-mart.com/blog/top-5-open-source-vector-databases-2024/#:~:text=Weaviate%20is%20a%20GraphQL,It%20seamlessly%20stores%20not))
    
10. Stack Overflow. _"Differences between LangChain & LlamaIndex."_ (answered Oct 2023). ([chatbot - Differences between Langchain & LlamaIndex - Stack Overflow](https://stackoverflow.com/questions/76990736/differences-between-langchain-llamaindex#:~:text=LlamaIndex%2C%20,of%20custom%20data%20into%20LLMs)) ([chatbot - Differences between Langchain & LlamaIndex - Stack Overflow](https://stackoverflow.com/questions/76990736/differences-between-langchain-llamaindex#:~:text=One%20key%20feature%20of%20LlamaIndex,be%20easily%20queried%20like%20so))
    
11. Medium @heyamit10. _"LlamaIndex vs LangChain vs Haystack."_ (2023). ([LlamaIndex vs LangChain vs Haystack | by Hey Amit - Medium](https://medium.com/@heyamit10/llamaindex-vs-langchain-vs-haystack-4fa8b15138fd#:~:text=While%20not%20as%20large%20as,level%20search))
    
12. LangChain Blog. _"Tutorial: ChatGPT Over Your Data."_ (Feb 2023). ([Tutorial: ChatGPT Over Your Data](https://blog.langchain.dev/tutorial-chatgpt-over-your-data/#:~:text=1,select%20the%20most%20relevant%20chunks)) ([Tutorial: ChatGPT Over Your Data](https://blog.langchain.dev/tutorial-chatgpt-over-your-data/#:~:text=1,relevant%20documents%20for%20the%20answer))
    
13. LlamaIndex Blog. _"RAG Evaluation Showdown: GPT-4 vs. Prometheus (open-source) Model."_ (Dec 2023). ([LlamaIndex: RAG Evaluation Showdown with GPT-4 vs. Open-Source Prometheus Model — LlamaIndex - Build Knowledge Assistants over your Enterprise Data](https://www.llamaindex.ai/blog/llamaindex-rag-evaluation-showdown-with-gpt-4-vs-open-source-prometheus-model-14cdca608277#:~:text=source%20Prometheus%20model%20has%20recently,alternative%20for%20such%20evaluation%20tasks)) ([LlamaIndex: RAG Evaluation Showdown with GPT-4 vs. Open-Source Prometheus Model — LlamaIndex - Build Knowledge Assistants over your Enterprise Data](https://www.llamaindex.ai/blog/llamaindex-rag-evaluation-showdown-with-gpt-4-vs-open-source-prometheus-model-14cdca608277#:~:text=1,the%20answer%20to%20the%20query))
    
14. DoorDash Engineering. _"Path to High-quality LLM-based Dasher Support Automation."_ (2023). ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Delivery%20support%20chatbot))
    
15. Evidently AI Blog. _"10 RAG examples and use cases from real companies."_ (Feb 2025). ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=Instead%20of%20treating%20the%20corpus,segmentation%20and%20improves%20retrieval%20accuracy)) ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=The%20Royal%20Bank%20of%20Canada,productivity%20and%20streamline%20customer%20support))
    
16. Thomson Reuters (Medium). _"Better Customer Support Using RAG at Thomson Reuters."_ (2023). ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=The%20system%20uses%20embeddings%20to,to%20get%20the%20best%20matches)) ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=to%20get%20the%20best%20matches))
    
17. Databricks Blog. _"Long Context RAG Performance of LLMs."_ (Sep 2024). ([Long Context RAG Performance of LLMs | Databricks Blog](https://www.databricks.com/blog/long-context-rag-performance-llms#:~:text=gpt)) ([Long Context RAG Performance of LLMs | Databricks Blog](https://www.databricks.com/blog/long-context-rag-performance-llms#:~:text=claude))
    
18. Anyscale Blog. _"Llama 2 is about as factually accurate as GPT-4 for summaries and is 30X cheaper."_ (Aug 2023). ([Llama 2 vs. GPT-4: Nearly As Accurate and 30X Cheaper](https://www.anyscale.com/blog/llama-2-is-about-as-factually-accurate-as-gpt-4-for-summaries-and-is-30x-cheaper#:~:text=,turbo)) ([Llama 2 vs. GPT-4: Nearly As Accurate and 30X Cheaper](https://www.anyscale.com/blog/llama-2-is-about-as-factually-accurate-as-gpt-4-for-summaries-and-is-30x-cheaper#:~:text=%2A%20%60gpt))
    
19. Databricks Blog. _"The Long Context RAG Capabilities of OpenAI o1 and Google Gemini."_ (Oct 2024). ([The Long Context RAG Capabilities of OpenAI o1 and Google Gemini | Databricks Blog](https://www.databricks.com/blog/long-context-rag-capabilities-openai-o1-and-google-gemini#:~:text=Although%20the%20overall%20answer%20correctness,4o%20models%20up%20to))
    
20. Evidently AI Blog. _"10 RAG examples and use cases from real companies."_ (Feb 2025). ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=%EF%B8%8F%E2%80%8D%E2%99%80%EF%B8%8F%20Analytical%20fraud%20reports)) ([10 RAG examples and use cases from real companies](https://www.evidentlyai.com/blog/rag-examples#:~:text=for%20users))
