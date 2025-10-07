---
uid: 13741f0c-e7cd-4354-b0b8-9ed9f56beaee
date created: 2025-07-19
date modified: 2025-08-31
---
旋转位置编码（Rotary Position Embedding，RoPE）是论文[ROFORMER: ENHANCED TRANSFORMER WITH ROTARY POSITION EMBEDDING](https://link.zhihu.com/?target=https%3A//arxiv.org/pdf/2104.09864.pdf) 提出的一种能够将相对位置信息依赖集成到 [self-attention](https://zhida.zhihu.com/search?content_id=601987732&content_type=Answer&match_order=1&q=self-attention&zhida_source=entity) 中并提升 transformer 架构性能的位置编码方式。而目前很火的 LLaMA、GLM 模型也是采用该位置编码方式。

# 数学对于深度学习的重要性

没有数学就没有RoPE，你现在只能用[ALiBi](https://zhida.zhihu.com/search?content_id=668828424&content_type=Answer&match_order=1&q=ALiBi&zhida_source=entity)

没有数学也不会有VAE，你现在只能用[GAN](https://zhida.zhihu.com/search?content_id=668828424&content_type=Answer&match_order=1&q=GAN&zhida_source=entity)





您图片中展示的函数 f(t) 的定义方式是：

- 如果 d=2i，那么 f(t)=sin(100002⋅i/D1​⋅t)
    
- 如果 d=2i+1，那么 f(t)=cos(100002⋅i/D1​⋅t)
    

这种特定的正弦型位置编码方案，其中正弦和余弦函数应用于不同的维度（基于索引 d，通常对应于词元嵌入的不同维度），并且正弦/余弦的参数涉及 t（表示位置）以及一个由 1/100002i/D 导出的频率项，**确实是 Transformer 论文“Attention Is All You Need”中使用的原始位置编码的核心思想。**

**然而，它并不是特指“RoPE”（旋转位置嵌入）。**

这里是它们之间的区别：

- **原始位置编码（如您图片所示）：** 这种方法在输入嵌入进入 Transformer 层之前，向其添加一个固定的正弦信号。它通过将其与词元嵌入相加来直接提供位置信息。您展示的公式正是原始 Transformer 论文中的那个。
    
- **RoPE (Rotary Positional Embedding - 旋转位置嵌入)：** RoPE 是一种**不同类型**的位置编码，它**对注意力机制中的查询（queries）和键（keys）进行旋转**。RoPE 不是将位置信息添加到嵌入中，而是直接将相对位置信息编码到注意力机制的点积中。它使用一个依赖于词元之间相对位置的旋转矩阵。虽然 RoPE 也使用正弦和余弦函数，但它们在注意力机制中的应用和目的与您图片中所示的加性位置编码有着根本的区别。
    

**总结：**

您提供的公式是原始 Transformer 模型中使用的**加性正弦位置编码**。虽然它使用了正弦和余弦，但它**不是** RoPE。RoPE 是一种不同且更近期的新型位置编码方法，它利用旋转变换。
