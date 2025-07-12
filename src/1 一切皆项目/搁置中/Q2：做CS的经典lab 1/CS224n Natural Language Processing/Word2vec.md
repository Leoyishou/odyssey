---
date created: 2025-01-25
date modified: 2025-07-10
uid: 6112a189-574a-489c-8d6a-7742a22e5dfc
---
**
非常有效的概率模型：Word2vec。Word2vec 是一个软件包实际上包含：

- **两个算法**：continuous bag-of-words（CBOW）和 skip-gram。CBOW 是根据中心词周围的上下文单词来预测该词的词向量。skip-gram 则相反，是根据中心词预测周围上下文的词的概率分布。
- **两个训练方法**：negative sampling 和 hierarchical softmax。Negative sampling 通过抽取负样本来定义目标，hierarchical softmax 通过使用一个有效的树结构来计算所有词的概率来定义目标。
