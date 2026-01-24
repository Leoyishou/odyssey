---
uid: 380eab47-b6a0-4af7-8c25-ab9a55401abd
date created: 2025-08-09
date modified: 2025-10-18
---

# Summary

q「好莱坞电影推荐」分词得到 多个Q「好莱坞」「电影」「推荐」

 q 与doc 的相关性计算方式是，

 q 里每个 Q 在该 doc 的 TF词频计算结果的和，但是是通过 IDF 加权后的结果



**Term Frequency–Inverse Document Frequency**。它的直译是：**词频–逆文档频率**。

IDF：：用文档d的长度（记作la）对词频做归一化。

场景：想象你要从一堆文章中找出每篇文章最重要的词：
	- "西湖"在杭州游记里出现10次 → 应该很重要
	- 但问题来了："的"、"了"也出现很多次，它们重要吗？❌
	- "的"、"了" → 每篇文章都有，太普通了，扣分！
	- "西湖" → 只在杭州相关文章出现，很特别，加分！
**
解决：**TF-IDF = TF × IDF**
	- 既要在本文中频繁出现（TF高）
	- 又要在其他文章中很少见（IDF高）
	- 这样的词才是真正的"关键词"


**举例：**

- "西湖"：杭州游记里常出现 ✓，其他城市游记很少提 ✓ → **高分！**
- "的"：到处都是 ✓，但太普通 ✗ → 低分
- "景点"：本文偶尔提 ✗ → 低分

**用途：** 搜索引擎、文章推荐、关键词提取都靠它！


TF‑IDF



TF-IDF 是一种把文本转换成数字的方法，让机器能够理解文本。它会计算每个词/字符组合的重要性。

具体在做什么：

1. 字符级特征提取

`analyzer="char_wb", ngram_range=(3, 5)`
- 把文本拆成3-5个连续字符的组合
- 比如 "hello" 会被拆成: "hel", "ell", "llo" (3字符), "hell", "ello" (4字符), "hello" (5字符)
- 这能捕捉拼写错误、俚语、特殊用词

2. 词级特征提取

`analyzer="word", ngram_range=(1, 2)`
- 把文本拆成单词和相邻两个词的组合
- 比如 "I love coding" 会被拆成: "I", "love", "coding", "I love", "love coding"
- 这能理解词汇含义和短语搭配

3. 特征合并

`Xtr = hstack([Xtr_c, Xtr_w])`  # 横向拼接
- 把字符特征和词特征并排放在一起
- 就像把两张表格左右拼接成一张大表
- 这样模型既能看到字符模式，又能理解词汇意义

4. 内存优化

`del Xtr_c, Xva_c, Xte_c, Xtr_w, Xva_w, Xte_w`
`gc.collect()`
- 删除中间变量，释放内存
- 因为文本特征矩阵很大，不删除会占用双倍内存

为什么要这样做？
- 字符级特征：抗拼写错误、捕捉细节
- 词级特征：理解语义、上下文
- 两者结合：让模型更准确地判断帖子是否违规

我来举例说明 fit_transform 前后的变化：

输入 (Input) - X_train

原始文本列表，比如：
```Java
X_train = [
    "[RULE] No spam [SUB] python [TEXT] Check out my website!",
    "[RULE] Be respectful [SUB] gaming [TEXT] You're an idiot",
    "[RULE] Stay on topic [SUB] cooking [TEXT] Great recipe, thanks!"
]

```

输出 (Output) - Xtr_c

一个稀疏矩阵（数字表格），每行代表一个文本，每列代表一个字符组合的TF-IDF值：

```Java
        "Che"  "hec"  "eck"  "You"  "ou'"  "u'r"  "Gre"  "rea"  "eat" ...
文本1:  0.23   0.18   0.21   0      0      0      0      0      0    ...
文本2:  0      0      0      0.31   0.28   0.25   0      0      0    ...
文本3:  0      0      0      0      0      0      0.19   0.22   0.20 ...
```

具体转换过程：

1. 提取3-5字符组合
  - 文本1: "Check" $\rightarrow$["Che", "hec", "eck", "Chec", "heck", "Check"]
  - 文本2: "You're" $\rightarrow$["You", "ou'", "u'r", "You'", "ou'r", "You'r"]
2. 计算TF-IDF值
  - TF (词频): 这个组合在当前文本中出现了多少次
  - IDF (逆文档频率): 这个组合有多独特（越少见越重要）
  - TF-IDF = TF $\times$ IDF
3. 生成稀疏矩阵
  - 大部分值是0（因为每个文本只包含少数字符组合）
  - 非0值表示该字符组合在该文本中的重要性

实际例子：
# 假设只有3个字符组合被提取
特征列:["eck", "You", "eat"]

文本1 "Check out...":[0.21, 0,    0]# 只有"eck"出现
文本2 "You're...":[0,    0.31, 0]# 只有"You"出现  
文本3 "Great...":[0,    0,    0.20]# 只有"eat"出现

这样，文本就从字符串变成了机器学习模型能理解的数字向量！
