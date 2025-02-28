---
draw:
tags: []
title: Transformer
date created: 2024-07-17
date modified: 2025-01-29
---

3blue的transformer视频

Transformer 模型 Q 由 Vaswani 等人在 2017 年提出，最初用于机器翻译任务。与传统的 RNN（循环神经网络）和 LSTM（长短期记忆网络）不同，Transformer 完全基于自注意力机制（Self-Attention Mechanism）实现，并行处理能力更强，训练速度更快。Transformer 的出现彻底改变了 NLP 领域，使得任务的性能显著提升。

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F22%2F03-55-58-dd276138c27da251f8022778663c33e9-202409220355685-852643.png)

## 自注意力机制

以 d_model=1 为例

```Java
## Q K V
Q 是问题，K 是 key，V 是 value
三者相乘得到 Q 的回答 A
自注意力就是 Q、K、V 是相同的

## 输入
我: [1]
爱: [2]
你: [3]

## 注意力分数(Q × K^T)

[我对我的关注度, 我对爱的关注度, 我对你的关注度]
[爱对我的关注度, 爱对爱的关注度, 爱对你的关注度]
[你对我的关注度, 你对爱的关注度, 你对你的关注度]
    softmax处理
[0.6, 0.2, 0.2]
[0.2, 0.6, 0.2]
[0.2, 0.2, 0.6]


## 与V相乘得到输出

[0.6, 0.2, 0.2]   [1]     [1.4]  我视角里的我爱你
[0.2, 0.6, 0.2] × [2] =   [1.8]  爱视角里的我爱你
[0.2, 0.2, 0.6]   [3]     [2.2]  你视角里的我爱你

通过这种方式映射以后，每个词从自我为中心的基础上稍微关注窗口内其他词，用每个词的视角去解读了一遍整个句子，输入是n个词，那么输出就是n个视角里的这句话

```

上面的部分『我爱你』只是变成了序列无关的embedding，但是现实中「我爱你」和「你爱我」其实是不同的，所以在 embedding 的时候还要加入位置编码以反应单词出现顺序的信息

知名大学教授

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
如果发顶会了记得带我一个名字。

区别于 [Transformers](Transformers.md)
