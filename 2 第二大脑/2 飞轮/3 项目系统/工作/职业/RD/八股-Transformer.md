---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-01
aliases: [Summary]
linter-yaml-title-alias: Summary
---
本文讨论了面试中关于Transformer相关知识的考点解析，包括手撕代码、概念问答、模型参数量计算、解码方法等内容。关键要点包括：​1.手撕Transformer模块：给出了Transformer中MHA、FFN、Positional Embedding、Encoder&Decoder等模块的参考代码，还给出了Transformer整体的代码实现及测试代码。​2.手撕Attention：用numpy的两种不同方式（np.einsum和np.matmul）实现了Attention，并给出了测试脚本。​3.K、Q、V取值问题：K和Q使用不同的W_k, W_q计算，在不同空间投影，能增加表达能力和泛化能力；若K和Q相同，attention score矩阵对称，泛化能力差。​4.Decoder的Cross Attn Mask：使用padding mask屏蔽来自encoder的padding token，只用padding mask不会出现attn信息泄漏问题。​5.大模型输出解码方法：介绍了贪心搜索、随机采样、Top - K、Top - P、BeamSearch、Blockwise Parallel Decoding、Speculative Sampling、Contrastive Decoding等方法在解码速度、质量和多样性上的特点。​6.Prenorm与Postnorm区别：同一设置下，Pre Norm结构易训练，但最终效果通常不如Post Norm；Pre Norm在transfomer layers较多时会退化，Post Norm训练更难但不易收敛。​7.Attention缩放原因：若不对softmax输入做缩放，输入数量级大时，softmax梯度趋向于0，导致梯度消失。​
