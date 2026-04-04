---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-01
aliases: [Summary]
linter-yaml-title-alias: Summary
---
本文讨论了面试中关于Transformer相关知识的考点解析，包括手撕代码、概念问答、模型参数量计算、解码方法等内容。关键要点包括：​1.手撕Transformer模块：给出了Transformer中MHA、FFN、Positional Embedding、Encoder&Decoder等模块的参考代码，还给出了Transformer整体的代码实现及测试代码。​2.手撕Attention：用numpy的两种不同方式（np.einsum和np.matmul）实现了Attention，并给出了测试脚本。​3.K、Q、V取值问题：K和Q使用不同的W_k, W_q计算，在不同空间投影，能增加表达能力和泛化能力；若K和Q相同，attention score矩阵对称，泛化能力差。​4.Decoder的Cross Attn Mask：使用padding mask屏蔽来自encoder的padding token，只用padding mask不会出现attn信息泄漏问题。​5.大模型输出解码方法：介绍了贪心搜索、随机采样、Top - K、Top - P、BeamSearch、Blockwise Parallel Decoding、Speculative Sampling、Contrastive Decoding等方法在解码速度、质量和多样性上的特点。​6.Prenorm与Postnorm区别：同一设置下，Pre Norm结构易训练，但最终效果通常不如Post Norm；Pre Norm在transfomer layers较多时会退化，Post Norm训练更难但不易收敛。​7.Attention缩放原因：若不对softmax输入做缩放，输入数量级大时，softmax梯度趋向于0，导致梯度消失。​


| **问题**                             | **技术核心回答 (关键词)**                                                                            | **小白通俗解释**                                                        |
| ---------------------------------- | ------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| **1. [[Self-Attention 计算过程]]**     | $Q \times K^T$ (相似度) $\to$ $/ \sqrt{d_k}$ (缩放) $\to$ Softmax (归一化) $\to$ $\times V$ (加权求和)。 | 像是查字典：拿着问题 (Q) 去找匹配的目录 (K)，算出匹配度 (权重)，最后把对应的内容 (V) 拼凑出来。          |
| **2. [[为什么要除以 d_k]]                | **防止梯度消失**。避免点积结果过大导致 Softmax 进入饱和区 (导数趋近 0)。                                               | 假如不除，分差会拉得极其大（比如一个 99 分，其他 0 分），模型就变得“偏执”，学不进新东西了。                |
| **3. [[Multi-Head Attention 优势]]** | **多子空间表征**。将 $d_{model}$ 拆分为多个 head，独立关注不同位置和语义特征 (如语法、指代)。                                 | 就像读一本书，原本是一个人读，现在是 8 个专家同时读，有的看语法，有的看剧情，最后汇总意见。                   |
| **4. 为什么需要位置编码 (PE)？**             | Self-Attention 是**集合操作 (Set)**，具有排列不变性。需注入绝对/相对位置信息 ($sin/cos$)。                            | Transformer 眼里“我爱你”和“你爱我”是一样的词堆。必须给每个词贴上“1号”、“2号”标签才能分清顺序。        |
| **5. Transformer 整体架构**            | **Encoder-Decoder**。Encoder (自注意力 + FFN) 提取特征；Decoder (Masked + Cross Attention) 生成序列。      | Encoder 是“阅读者”，负责把文章读懂并压缩成精华；Decoder 是“写作者”，根据精华逐字写出译文。           |
| **6. LayerNorm vs BatchNorm**      | **LN 对样本归一化** (适合变长序列 NLP)；**BN 对批次归一化** (依赖 Batch Size，适合 CV)。                             | BN 是按全班平均分调整你的成绩（受别人影响大）；LN 是按你自己各科平均分调整你的偏科（只看你自己，适合写文章这种个性化任务）。 |
| **7. 残差连接 (Residual) 作用**          | $x + SubLayer(x)$。**解决退化问题**，允许梯度直接流向浅层，构建更深网络。                                             | 像是给信息传递修了一条“高速直达通道”。就算中间的处理层把信息搞乱了，原始信息还能顺着通道传下去，保底不亏。            |
| **8. BERT vs GPT 区别**              | **BERT**: 双向 Encoder (完形填空)，懂语义；**GPT**: 单向 Decoder (接龙)，懂生成。                               | BERT 是“英语老师”，擅长做阅读理解和改错；GPT 是“小说家”，擅长根据上文续写下文。                    |
| **9. 计算复杂度 (Self-Attention)**      | **$O(n^2 \cdot d)$**。序列长度 $n$ 的平方级复杂度，长文本处理成本极高。                                            | 每个人都要和其他所有人握手一次。如果只有 10 个人还好，要是有 1 万个人，握手次数就是天文数字。                |
| **10. 为什么 Transformer 可并行？**       | **无时序依赖**。即 $t$ 时刻计算不依赖 $t-1$ 时刻状态 (对比 RNN)。                                                | RNN 像是接力赛，必须等上一棒跑完；Transformer 像是大合唱，所有人拿着乐谱同时开口唱。                |