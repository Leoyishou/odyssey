---
date created: 2024-07-17
date modified: 2025-09-19
uid: fd6ec90a-6faa-417a-b30a-d5347051d5ad
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

1. [[Transformer架构]]是 Google Brain + Google Research 的团队在 2017 年的论文《Attention Is All You Need》里提出的。
2. 这篇论文里其实就有了第一个实现类：机器翻译模型 “[[Transformer模型]]” 本身。它包括 Encoder-Decoder 两部分，直接在机器翻译任务（英-德、英-法）上做实验并成功超越了当时的 RNN/CNN 方法。
3. 除了 [[BERT]] (2018, Google) 之外，还有很多基于 Transformer 的模型，你可以理解为不同的“实现类”或“子类”：
	• GPT 系列 (2018起, OpenAI)：只用 Decoder 堆叠，专注生成。
	• Transformer-XL (2019, Google/CMU)：解决长序列问题。
	• XLNet (2019, Google/CMU)：结合自回归和自编码思想。
	• T5 (2019, Google)：把所有 NLP 任务统一成文本到文本的形式。
	• Vision Transformer (ViT, 2020, Google)：把 Transformer 用到图像。


![CleanShot 2025-07-13 at 16.49.12@2x.png|1300](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_Y6nyeM0LzS%2F2025%2F07%2F13%2F16-49-28-7521b68d4a02125259e675439149deaf-CleanShot%202025-07-13%20at%2016.49.12-2x-012daf.png)


最牛逼的点：
1. [[并行计算]]
2. 注意力机制

Transformer不是抽象的数学玩具，它是现代AI的“骨干”（backbone），支撑着像[[ChatGPT]]、[[BERT]]和[[DALL-E]]这样的产品。它让机器更好地理解、解释和生成人类语言，比之前的[循环神经网络RNN](循环神经网络RNN.md)/[长短期记忆网络LSTM](长短期记忆网络LSTM.md)模型快得多、

# Cues
[[Transformer 参数]]
[多头注意力机制](多头注意力机制.md)
[Positional Encoding 位置编码](Positional%20Encoding%20位置编码.md)
1. （[[自注意力机制 Self-attention]]）
2. Batch Norm & Layer Norm（批量标准化/层[[标准化]]）
3. ResNet（[[残差]]）
4. Subword Tokenization（子词分词法）
5. 组装：Transformer

[[dropout]]
[[涌现]]
[[词汇表]]
[[batch]]

[https://zhuanlan.zhihu.com/p/1903109025303883861?share_code=sxLzOfpNsGww&utm_psn=1931375190878717429](https://zhuanlan.zhihu.com/p/1903109025303883861?share_code=sxLzOfpNsGww&utm_psn=1931375190878717429)

# Notes

![image.png|500](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F08%2F06%2F14-40-44-f3308b31f76faee043eb101f1ee36d65-202508061440113-03443a.png)


## 真正的历史

### 主要脉络

1. 为什么要“干掉 RNN”
    - [循环神经网络RNN](循环神经网络RNN.md)/LSTM/GRU 速度慢（时序依赖无法并行）＋长程记忆差（梯度消散、遗忘）。
    - Google Brain 团队的原始动机：找一种完全并行、无递归的新架构。
        
2. Attention 已有雏形，但只是配角
    - 2014 年 Bahdanau 等人把 Attention 嵌进 Seq2Seq，缓解信息瓶颈；仍依赖 RNN。
    - 2016 年 Bengio 组的 “Self‑Attention” 用作句子 pooling，三合一 Q/K/V，输出固定长度——本质上是一种加性注意力池化，并非今天的 MHA。
        
3. Google Brain 的多轮失败与关键转折
    - 团队最早尝试改造 Bengio 版思路，效果平平。
    - Noam Shazeer 通过密集消融实验，逐步确定了三个关键要素：
        ① 点积而非加性 ② 缩放除√d ③ 多头并保持维度一致。
        
4. Transformer Block 的定型
    - 有了 MHA + 前馈 + 残差 + [Positional Encoding 位置编码](Positional%20Encoding%20位置编码.md)，完全摆脱递归，计算可全并行。
    - “Transformer” 名字更多是情怀梗（变形金刚）+“层层变换”双关。
        
5. 后续演化的“流水”与“MHA”这根铁钉
    - 交叉注意力、前馈结构、位置编码、归一化方式不断改良。
    - 唯独 MHA 基本不动——成为所有大模型（BERT、GPT、T5…）的共同核心。
        

---

### 本质观点

> Transformer 的破局点不是“有了注意力”，而是“把注意力锻造成可堆叠、可并行、可扩展的多头缩放点积自注意力”，并用系统化实验确立其地位。

文章通过讲动机、试错与“炼丹”过程，想让读者明白两件事：

1. 技术替代的驱动力：并行效率与长程建模需求促使 RNN 让位。
    
2. 架构创新的方式：大量消融实验 + 关键细节（缩放、多头、维度匹配）才能把一个点子真正落地并长期存活。
    

因此，这不仅是 Transformer 诞生的回顾，也是一次关于 “工程化实验如何塑造 AI 重大突破” 的案例分享。



```python
    def forward(
        self,
        input_ids: Int[Tensor, "batch seq"],
        weights: dict[str, Tensor],
    ) -> Float[Tensor, "batch seq vocab_size"]:
        """Forward pass through transformer LM."""
        
        # Token embeddings
        x = embedding(weights["token_embeddings.weight"], input_ids)

        # Pass through transformer blocks
        for i in range(self.num_layers):
            block_weights = {
                key.replace(f"layers.{i}.", ""): value
                for key, value in weights.items()
                if key.startswith(f"layers.{i}.")
            }
            x = self.blocks[i](x, block_weights)

        # Final layer norm
        x = rmsnorm(weights["ln_final.weight"], x, eps=1e-5)

        # Language model head
        # 就是把d model 维的潜空间的向量转化为词汇表中每个词的得分（logits）
        # 就是我们使用大模型中的 top k 参数要控制的东西
        logits = x @ weights["lm_head.weight"].T

        return logits
```
## 学习方法

[@ProfTomYeh](https://x.com/ProfTomYeh)  
 并重新计算查看变化！

千万不要搁那研究 k 是建值，q 是查询，v 是值，如果你看到这种讲解，基本就别看了，那作者自己也没搞明白。
信我一句，把 transformer 和 [GNN](GNN.md)，[GCN](GCN) 放在一起学，你会看到更加本质的东西。
这样你就能理解位置嵌入，不管是正弦还是可学习的嵌入，不管是时间嵌入还是其他先验嵌入。
进而理解什么 [autoformer](https://www.zhihu.com/search?q=autoformer&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D)，ltransformer，[itransformer](https://www.zhihu.com/search?q=itransformer&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D)，graphformer，这样你就会看到 [transformer](https://www.zhihu.com/search?q=transformer&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D) 在多元时序和图上的应用（二者本就一样）
然后你就能明白只要改动注意力计算的方式就能造一个新的 transformer，至于 [多头](https://www.zhihu.com/search?q=%E5%A4%9A%E5%A4%B4&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D) 和单头，就非常容易理解。而至于什么 [多模态cross attention](https://www.zhihu.com/search?q=%E5%A4%9A%E6%A8%A1%E6%80%81cross%20attention&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D)，那也就更加显而易见了。
而 [残差](https://www.zhihu.com/search?q=%E6%AE%8B%E5%B7%AE&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D) 和 norm 只是模型的一种技巧，虽然是小技巧，但实际很有用。
那个 [ffn](https://www.zhihu.com/search?q=ffn&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D)，则更是不值一提。你就算用 [CNN](https://www.zhihu.com/search?q=CNN&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D) 去平替，在小问题上也毫无压力。
而至于在 [cv](https://www.zhihu.com/search?q=cv&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D) 上的使用，其实就是变着法把图像信息变成 [token序列](https://www.zhihu.com/search?q=token%E5%BA%8F%E5%88%97&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D)。
后续的自监督方式，则是另一个内容了。
从可解释性上可以试着考虑一下流形，信息论，
encoder 和 decoder 看着玄学，实则简单，建议多往 [流形](https://www.zhihu.com/search?q=%E6%B5%81%E5%BD%A2&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3272070260%7D) 和信息论上靠一靠。

3blue的transformer视频

[👨🏻‍💻王木头学科学](👨🏻‍💻王木头学科学.md)
## 基本框架



>  类比一个英国人想成为一个中文作家，中文真实含义 = 词典（[[潜空间]]中的[[词向量]]们）+ 选哪些词（[注意力机制](注意力机制.md)）+ 行文顺序[Positional Encoding 位置编码](Positional%20Encoding%20位置编码.md)
1. 是掌握基本的外语词汇量，[嵌入 embedding](嵌入%20embedding.md)解决单个词，单个语义的问题，相当于是建立了一个[[潜空间]]的翻译方式
2. 是选哪些词，[注意力机制](注意力机制.md)解决的是许多词组合在一起之后，整体体现出来的那个语义。
3. 是行文顺序，[[Positional Encoding 位置编码]]


	
![CleanShot 2025-07-13 at 17.50.41@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_yMH68e78vg%2F2025%2F07%2F13%2F17-51-29-373cf532ca9cdd32593698ba31b040b8-CleanShot%202025-07-13%20at%2017.50.41-2x-4d05a3.png)

![CleanShot 2025-07-20 at 15.36.29@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_UXFigip7eR%2F2025%2F07%2F20%2F15-36-45-35124c64da2f64f9c1df3e1fbe58e461-CleanShot%202025-07-20%20at%2015.36.29-2x-38302e.png)

1. 左边是编码过程，右边是解码过程
2. [[掩码]]是为了在[[解码器]]生成答案的时候屏蔽本词后面的词
3. [[Add&Norm]]

### 训练过程



![CleanShot 2025-07-13 at 19.01.19@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_oRGEW1Swk3%2F2025%2F07%2F13%2F19-01-28-9d0f0a980ba551addaf2cf4a7b3e1601-CleanShot%202025-07-13%20at%2019.01.19-2x-2e0514.png)

图中需要注意的几个点
1. 图中的 Nx 代表可以叠加很多层
2. [[损失函数]]：好久不见和 long time no see在[潜空间](潜空间.md)中两个[词向量](词向量.md)的差异

### 推理过程

好久不见 + `<BOS>` 通过[交叉注意力](交叉注意力.md) 得到-> long
![CleanShot 2025-07-19 at 22.22.34@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_KROJ0Olrj1%2F2025%2F07%2F19%2F22-22-43-a909fa57a90eb3256fd455c836c0d6d9-CleanShot%202025-07-19%20at%2022.22.34-2x-f560ff.png)


![CleanShot 2025-07-13 at 19.06.27@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_k0iKGKnI52%2F2025%2F07%2F13%2F19-07-14-b0446b19745382d9e4d2d7cf1c554bf9-CleanShot%202025-07-13%20at%2019.06.27-2x-e0c9e3.png)

好久不见 + `<BOS>` -> long
`<BOS>` + long  -> time
`<bos>` + long + time -> no
`<bos>` + long + time + no -> see
`<bos>` + long + time + no + see -> `<eos>`

所谓 GPT 的 Decoder-only，的核心特征：因果掩码

![CleanShot 2025-07-19 at 22.24.32@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_CXtkQ6xhf3%2F2025%2F07%2F19%2F22-24-43-127512b96b7b5fa1063714e51b05d0dc-CleanShot%202025-07-19%20at%2022.24.32-2x-d6ecbe.png)

## 发展脉络

![CleanShot 2025-07-13 at 16.49.12@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_Y6nyeM0LzS%2F2025%2F07%2F13%2F16-49-28-7521b68d4a02125259e675439149deaf-CleanShot%202025-07-13%20at%2016.49.12-2x-012daf.png)

### Decoder-Only

![CleanShot 2025-07-13 at 19.14.39@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_06KynsXg3C%2F2025%2F07%2F13%2F19-15-06-78e189d2bf6a1a0608590ef8542d9151-CleanShot%202025-07-13%20at%2019.14.39-2x-c8fa06.png)

Decoder-Only：它不是让模型从O开始去生成，而是把你输入的内容，先输入到模型里面，让模型把你的内容看作是它已经生成过的内容，它只需要接着继续生成后续的内容。



好久不见 + `<BOS>` -> long
`<BOS>` + long  -> time
`<bos>` + long + time -> no
`<bos>` + long + time + no -> see
`<bos>` + long + time + no + see -> `<eos>`


## 参数介绍
### 1. vocab_size = 50：词汇表大小

- 表示模型能识别的不同token（词）的总数
- 比如：词汇表可能包含["我", "爱", "你", "他", "她",...]共50个不同的词
- 每个词有一个唯一的ID，范围是 0-49

### 2. d_model = 64：模型维度（潜空间维度）

- 这才是每个词在模型内部的向量表示维度
- 每个词都会被编码成一个64维的向量

## transformer 和 RNN 的区别

这句话可以拆成两部分来理解：

|关键词|含义|
|---|---|
|长依赖（long-range dependency）|在一段文本或序列里，前面元素对很后面元素产生影响的情况。例如一句话里开头出现的主语，要到句尾才和谓语动词配对。|
|弥补 RNN 的缺陷|传统循环神经网络（RNN）要一个字一个字按顺序处理，时间跨度一长，早期信息容易在梯度传播中衰减（梯度消失/爆炸），导致它“记不住”很久以前的内容。|

### 1. RNN 为何难处理长依赖？
- 串行处理：RNN 必须一步步地把前一时刻的隐藏状态传到下一时刻，序列越长，信息“搬运”次数越多。
- 梯度衰减：反向传播时，梯度在长链条上传递会指数级缩小或放大，模型学不到真正有用的远距离关联。
- 记忆容量有限：即便改进的 LSTM/GRU 加入门控机制，也只能缓解而非根治问题。
    

### 2. Transformer 的创新点

- 自注意力（Self-Attention）
    每个位置都直接“抬头”看整条序列，对任意两位置计算注意力权重，一步就把远距离信息“拉近”——再长的间隔也只是一次矩阵乘法的事。
- 并行计算  
    不用按时间展开，整段序列同时送进模型，显存允许时还能一次性看更长上下文。
- 多头机制  
    多组注意力在不同的“子空间”里各自捕捉语法、实体、位置等关系，信息更加丰富。

### 3. 形象比喻
- RNN 像接力赛：每位选手（时间步）必须把 baton（隐藏状态）交到下一位，路程越长越容易掉棒。
- Transformer 像电话会议：所有队员直接接入同一个会议室（自注意力矩阵），想联系谁就拨分机号，距离再远也一秒可达。
    
### 4. 举个具体例子

句子：“The book that you gave me yesterday is amazing.”

|模型|需要做什么|难点|
|---|---|---|
|RNN/LSTM|把 “book” 的信息一路传到 “is” 才知道主语|经过 6-7 个词，梯度已衰减，可能忘记了 “book”|
|Transformer|“is” 直接对整句做注意力，立刻发现与 “book” 最相关|无论中间插入多少定语从句，都只是一次注意力操作|

总结  
“Transformer 的创新点正是弥补 RNN 在长依赖上的缺陷”这句话就是说：Transformer 用自注意力机制，让模型能一次性、并行地捕捉序列中任意距离的关联，避免了 RNN 逐步传递、易丢失远距离信息的问题，从而大幅提高了对长文本或长序列的理解与生成能力。




## Transformer模型的发明历史

Transformer模型是现代AI（尤其是自然语言处理和大型语言模型如ChatGPT的核心架构）的一个革命性发明。它不是凭空出现的，而是基于前人工作逐步演化而来。下面我基于可靠来源（如Wikipedia、NVIDIA博客和学术论文）来通俗解释它的发明过程。

### 1. 背景：为什么需要Transformer？
   - 在2017年之前，处理序列数据（如文本翻译、语音识别）的常见方法是循环神经网络（RNN）和长短期记忆网络（LSTM）。这些模型像“一步一步走”一样处理数据：一个词一个词地读，容易遗忘远距离信息（比如句子前面的词对后面的影响），而且计算慢，无法充分利用GPU的并行能力。
   - 2014年，深度学习先驱Yoshua Bengio在一篇论文中引入了“注意力机制”（attention mechanism），这是一种能让模型“关注”序列中重要部分的技术。但那时注意力只是RNN的“辅助工具”。
   - Google Brain团队看到机会：如果完全抛弃RNN，只用注意力机制，能不能更快、更准地处理序列数据？这就是Transformer的起源。

### 2. 发明过程：2017年的突破
   - 时间和团队：2017年，Google的8位研究者（Ashish Vaswani、Noam Shazeer、Niki Parmar、Jakob Uszkoreit、Llion Jones、Aidan N. Gomez、Łukasz Kaiser 和 Illia Polosukhin）在论文《Attention is All You Need》中首次提出Transformer架构。
   - 关键创新：他们设计了一个完全基于“多头注意力”（multi-head attention）的模型，包括编码器（encoder）和解码器（decoder）。这允许模型并行处理整个序列（如一次性看完整句话），而不是顺序处理。结果？训练速度飞升，性能碾压RNN。
   - 最初应用：论文聚焦机器翻译（如英译法），但很快扩展到其他领域。论文预印本发布后，迅速流行，因为它简单高效。
   - 后续发展：2018年，OpenAI基于Transformer推出GPT-1（Generative Pre-trained Transformer），这是第一个生成式预训练模型。后来GPT系列（到GPT-4）主导了AI聊天机器人。BERT（2018年，Google）则是另一个变体，专注于理解任务。

Transformer的发明标志着AI从“递归时代”进入“注意力时代”，开启了生成式AI的爆炸式增长。现在，它被用于文本生成、图像处理、蛋白质预测等，几乎无处不在。

### 3. 有趣的事儿（Fun Facts）

Transformer的诞生不乏有趣的轶事，这些让它听起来更“人性化”：

- 标题的灵感来源：论文标题“Attention is All You Need”（注意力就是一切）其实是致敬披头士乐队的歌曲“All You Need Is Love”（爱就是一切）。据合著者Llion Jones透露，这是他们故意玩的梗，体现了团队的幽默感。
- 提前“玩”生成内容：在论文正式发布前的2017年春天，一位合著者就用Transformer的“decoder-only”变体（纯解码器版本）生成假的维基百科文章！这预示了后来ChatGPT之类的生成AI。
- 与GPU的“天作之合”：Transformer的并行设计正好赶上GPU（如NVIDIA的芯片）大爆发，导致AI训练成本暴降。但也引发了全球芯片短缺——2023年，AI热潮让NVIDIA市值破万亿美元，而训练一个大模型需要数千块GPU跑几个月！
- 品牌争议：OpenAI在2018年发明了“GPT”这个词（基于Transformer），但到2023年，他们开始“护犊子”——更新服务条款，禁止其他公司用“GPT”命名产品，甚至雇品牌管理公司发通知。结果，一些插件（如Google Workspace的“GPT for Sheets”）被迫改名。
- 意外的跨界应用：Transformer本是为翻译设计的，但现在用于预测蛋白质结构（AlphaFold，2020年），帮科学家加速药物开发。甚至在游戏开发和机器人控制中都有身影，堪称“万金油”。

总之，Transformer的发明是AI史上的一次“优雅革命”，从解决翻译痛点起步，意外点燃了生成式AI的火种。
