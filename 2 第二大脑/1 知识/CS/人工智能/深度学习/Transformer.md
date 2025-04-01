---
draw:
title: Transformer
date created: 2024-07-17
date modified: 2025-03-29
---

在一段文本中，某个词元的真正语义，有可能要等到后续的所有内容出现了之后，才能完全确定。这种情况在英语中尤其常见，比如各种各样的从句，而在汉语中也常常会遇到这种情况。当然，还有的时候哪怕文本读完了都不能确定语义，但这一般就是病句或者碰到谜语人了。现在我们的目标是让模型能翻译一般的句子，解读谜语人有点太强模型所难了，我们暂且忽略这种情况。Transformer 的核心是自注意力机制*。而自注意力机制，就是这种能够完整利用整个句子的所有信息，来表达"某个词元（token）的在句子中语义"的结构。

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

___

好的，我将为你设计一套总时长约 20 小时、风格与前面一致的分步式 Transformer 教程，专注于机器翻译任务。教程将使用 PyTorch 的标准模块构建，涵盖编码器、解码器、注意力机制等核心内容，并包含训练、评估、推理与模型保存流程。我会尽快整理好教程结构并展示给你。

## 从零实现 Transformer：机器翻译分步教程

Transformers 是一种基于**自注意力 (self-attention)**机制的编码器-解码器神经网络架构，由 Vaswani 等人在 2017 年提出，用于解决机器翻译等序列到序列任务 ([Transformer (deep learning architecture) - Wikipedia](https://en.wikipedia.org/wiki/Transformer_\(deep_learning_architecture\)#:~:text=The%20transformer%20is%20a%20deep,important%20tokens%20to%20be%20diminished)) ([MultiheadAttention — PyTorch 2.6 documentation](https://pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html#:~:text=Method%20described%20in%20the%20paper%3A,Attention%20Is%20All%20You%20Need))。与传统的循环神经网络 (RNN) 不同，Transformer 摒弃了递归结构，转而在每一层通过多头自注意力机制建模序列全局信息。这种设计允许 Transformer 模型并行处理整个序列，高效捕捉长距离的依赖关系 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=,for%20training%20language%20translation%20models))。例如，Transformer 可以通过自注意力学习句子中单词之间的关系，关注关键信息而忽略不相关内容 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=,and%20to%20ignore%20irrelevant%20words))。Transformer 现已成为机器翻译领域的主流方法，并被广泛应用于各种自然语言处理任务。

([File:Transformer, full architecture.png - Wikimedia Commons](https://commons.wikimedia.org/wiki/Image:Transformer,_full_architecture.png)) _图1：Transformer 编码器-解码器架构示意图。左侧为编码器（多个相同的编码器层堆叠），右侧为解码器（多个相同的解码器层堆叠）。编码器层包含多头自注意力和前馈网络，解码器层在自注意力之外还包含“交叉”注意力，用于融合编码器输出信息。模型通过添加位置编码获取序列位置信息，并在解码端输出经过线性映射的预测概率分布。_

本教程将通过**Exercism 风格**的逐步练习，一步步从零实现一个用于英德机器翻译的 Transformer 模型。总共规划 10 个步骤，每步包含背景理论讲解、代码实现以及必要的验证。整个教程预计耗时约 **20 小时** 完成，读者将亲自动手准备数据、搭建模型、训练并评估翻译效果。请按照顺序逐步完成每个练习，充分理解每个模块的作用。

**教程步骤概览：**

1. **数据集准备**–获取英德平行语料并进行分词、词汇表构建，创建数据加载器
    
2. **位置编码和嵌入层**–实现位置编码 (Positional Encoding) 和词嵌入层
    
3. **编码器模块构建**–实现 Transformer 编码器层（多头自注意力、残差连接和前馈网络）
    
4. **解码器模块构建**–实现 Transformer 解码器层（包含目标序列的掩码、多头交叉注意力）
    
5. **组装完整 Transformer 模型**–将编码器、解码器堆叠，并整合嵌入层和输出层形成完整模型
    
6. **损失函数与优化器**–定义带标签平滑的交叉熵损失以及 Adam 优化器
    
7. **训练与验证循环**–编写训练(epoch)循环，采用 _teacher forcing_ 策略喂入目标序列，监控训练/验证损失
    
8. **推理与生成翻译**–实现贪心解码 (greedy decoding) 来生成翻译结果（并简要讨论 beam search）
    
9. **模型评估**–使用 BLEU 分数等指标评估翻译质量
    
10. **模型保存与加载**–保存训练好的模型权重，并加载模型对新输入进行翻译测试
    

在开始实际编码之前，请确保已经安装 Python 依赖库：**PyTorch** (版本1.10+), **torchtext**, 以及用于分词的 **SpaCy**（包括英文和德文模型）。如果使用本地环境运行代码，请执行：

```bash
pip install torch torchtext torchdata spacy
python -m spacy download en_core_web_sm
python -m spacy download de_core_news_sm
```

下面让我们按照步骤逐一实现 Transformer 翻译模型。

### 1. 数据集准备：英德翻译语料的处理

首先，我们需要准备平行语料数据集（例如 IWSLT 或 Multi30k 数据集的英文-德文翻译对）。本教程将使用 PyTorch 的 **torchtext** 库来加载 **Multi30k** 英文-德文数据集 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=from%20torchtext,from%20typing%20import%20Iterable%2C%20List)) ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=train_iter%20%3D%20Multi30k,ln%5D%20%3D%20build_vocab_from_iterator%28yield_tokens%28train_iter%2C%20ln%29%2C%20min_freq%3D1))。我们将对句子进行分词(tokenization)、建立单词到索引的词表(vocabulary)，并为模型添加特殊符号。特殊符号包括：未知词标记`<unk>`、填充标记`<pad>`、序列起始标记`<bos>`和序列结束标记`<eos>`。我们约定它们的索引分别为0、1、2、3 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=,eos))。在训练和翻译过程中，我们会在每个源句子和目标句子开头加入`<bos>`、结尾加入`<eos>`，并使用`<pad>`填充使一个 batch 中的序列等长。

**理论讲解：**机器翻译模型需要将单词映射为数字才能输入神经网络。这通常包括以下步骤：

- **分词(tokenization)**：将原始句子拆分为单词或子词序列。本教程使用 SpaCy 分词器对英文和德文句子分词。
    
- **构建词汇表(vocabulary)**：统计语料中的单词，建立映射表将每个词映射到唯一的数字ID。我们会限定词表大小或最低频率，以过滤极罕见词。未在词表中的词统一映射为`<unk>`。
    
- **序列数值化**：将分词后的句子按词汇表映射为索引序列，并在开头结尾添加`<bos>`和`<eos>`标记。
    
- **批处理填充**：由于每个句子长度不一，我们对一个批次中的序列填充`<pad>`到相同最大长度。填充不会传递有用信息，稍后我们会使用**填充掩码(mask)**使模型忽略填充的位置。
    

下面的代码将使用 **torchtext** 提供的 `Multi30k` 数据集构建英德双语的数据管道。其中，我们使用 SpaCy 分别对英文和德文进行分词，利用 `build_vocab_from_iterator` 创建词汇表，并定义用于数据加载的转换和`collate`函数。

```python
import torch
from torchtext.data.utils import get_tokenizer
from torchtext.datasets import Multi30k
from torchtext.vocab import build_vocab_from_iterator

# 指定源语言和目标语言
SRC_LANGUAGE = 'de'
TGT_LANGUAGE = 'en'

# 定义特殊符号及索引
UNK_IDX, PAD_IDX, BOS_IDX, EOS_IDX = 0, 1, 2, 3
special_symbols = ['<unk>', '<pad>', '<bos>', '<eos>']

# 初始化分词器（使用SpaCy）
token_transform = {}
token_transform[SRC_LANGUAGE] = get_tokenizer('spacy', language='de_core_news_sm')
token_transform[TGT_LANGUAGE] = get_tokenizer('spacy', language='en_core_web_sm')

# 构建词表的迭代器
def yield_tokens(data_iter, language):
    language_index = {SRC_LANGUAGE: 0, TGT_LANGUAGE: 1}
    for data_sample in data_iter:
        yield token_transform[language](data_sample[language_index[language]])

# 构建源语言和目标语言的词汇表
vocab_transform = {}
for ln in [SRC_LANGUAGE, TGT_LANGUAGE]:
    train_iter = Multi30k(split='train', language_pair=(SRC_LANGUAGE, TGT_LANGUAGE))
    vocab_transform[ln] = build_vocab_from_iterator(
        yield_tokens(train_iter, ln),
        min_freq=1,
        specials=special_symbols,
        special_first=True  # 将special symbols放在词表开头
    )
    vocab_transform[ln].set_default_index(UNK_IDX)  # 未登录词返回<unk>的索引

# 确认特殊标记索引
assert vocab_transform[SRC_LANGUAGE]['<pad>'] == PAD_IDX
assert vocab_transform[TGT_LANGUAGE]['<bos>'] == BOS_IDX
print("Vocab size (DE):", len(vocab_transform[SRC_LANGUAGE]), 
      "Vocab size (EN):", len(vocab_transform[TGT_LANGUAGE]))
# 输出示例：Vocab size (DE): 16000 Vocab size (EN): 10000 （具体大小视数据集而定）
```

上面的代码构建了德语和英语的词汇表，并确保了特殊符号的索引顺序。在我们的设定中，`<unk>=0`，`<pad>=1`，`<bos>=2`，`<eos>=3`。现在我们定义文本转换管道和 `DataLoader`。文本转换包括：先分词，再映射为索引序列，最后添加 `<bos>` 和 `<eos>` 标记并转换为张量。`collate_fn` 则会被 `DataLoader`调用，用于将每个批次的可变长序列张量填充对齐。

```python
from torch.nn.utils.rnn import pad_sequence

# 将一系列转换函数串联为一个变换
def sequential_transforms(*transforms):
    def func(txt_input):
        for transform in transforms:
            txt_input = transform(txt_input)
        return txt_input
    return func

# 在序列两端添加BOS/EOS标记并转换为张量
def tensor_transform(token_ids: list[int]):
    return torch.cat((
        torch.tensor([BOS_IDX]),
        torch.tensor(token_ids),
        torch.tensor([EOS_IDX])
    ))

# 构建用于源语句和目标语句的文本转换pipeline
text_transform = {}
for ln in [SRC_LANGUAGE, TGT_LANGUAGE]:
    text_transform[ln] = sequential_transforms(
        token_transform[ln],        # 分词
        vocab_transform[ln],        # 映射为索引
        tensor_transform           # 添加BOS/EOS并转换为tensor
    )

# 定义collate_fn，将一个batch的样本列表转换为批量张量
def collate_fn(batch):
    src_batch, tgt_batch = [], []
    for src_sample, tgt_sample in batch:
        # 应用文本转换（字符串->张量）
        src_batch.append(text_transform[SRC_LANGUAGE](src_sample.rstrip("\n")))
        tgt_batch.append(text_transform[TGT_LANGUAGE](tgt_sample.rstrip("\n")))
    # 按当前批次中最长序列进行填充
    src_batch = pad_sequence(src_batch, padding_value=PAD_IDX)  # shape: [src_len, batch_size]
    tgt_batch = pad_sequence(tgt_batch, padding_value=PAD_IDX)  # shape: [tgt_len, batch_size]
    return src_batch, tgt_batch

# 构建数据迭代器和DataLoader
from torch.utils.data import DataLoader
train_iter = Multi30k(split='train', language_pair=(SRC_LANGUAGE, TGT_LANGUAGE))
train_dataloader = DataLoader(train_iter, batch_size=32, collate_fn=collate_fn)
```

在上面的代码中，我们使用 `pad_sequence` 函数对批次中的句子进行填充 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=src_batch.append%28text_transform%5BSRC_LANGUAGE%5D%28src_sample.rstrip%28))。现在，数据集已经准备就绪：`train_dataloader` 将在迭代时返回形状为 `(src_len, batch)` 的源张量和 `(tgt_len, batch)` 的目标张量。下面我们测试一个小示例，确保文本转换效果符合预期。

```python
# 从数据加载器取出一个小批次进行测试
src_sample, tgt_sample = next(iter(train_dataloader))
print("Sample batch shape:", src_sample.shape, tgt_sample.shape)
# 检查每个序列是否以BOS开头、EOS结尾（非pad位置）
assert src_sample[0,0].item() == BOS_IDX and src_sample[-1,0].item() == EOS_IDX
assert tgt_sample[0,0].item() == BOS_IDX and tgt_sample[-1,0].item() == EOS_IDX
print("BOS/EOS tokens check passed. Example src indices:", src_sample[:,0])
```

我们已经成功准备了训练数据：对每个批次，`src_sample` 和 `tgt_sample` 张量分别包含填充对齐的源句子和目标句子序列。接下来，我们将开始实现 Transformer 模型所需的各个模块，从位置编码开始。

### 2. 实现位置编码 (Positional Encoding) 和嵌入层

Transformer 模型没有像RNN那样的顺序处理机制，因此需要一种方式让模型知晓序列中单词的位置信息。**位置编码**通过在词向量中添加按位置变化的固定向量来注入位置信息 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=den%20%3D%20torch.exp%28,2))。原论文使用了基于正弦和余弦函数的位置编码：对于位置 `pos` 和维度索引 `i`，位置编码定义为：

- $PE(pos,,2i) = \sin!\Big(pos / 10000^{2i/d_{\text{model}}}\Big)$
    
- $PE(pos,,2i+1) = \cos!\Big(pos / 10000^{2i/d_{\text{model}}}\Big)$
    

也就是说，位置编码在偶数维度为正弦函数，奇数维度为余弦函数，具有不同频率 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=pos_embedding%20%3D%20torch.zeros%28%28maxlen%2C%20emb_size%29%29%20pos_embedding,2))。这样处理后，不同位置的编码在各维度上呈现规律变化，模型可以通过学习对不同频率的响应来推断相对位置信息。此外，我们还将实现**词嵌入层**(Embedding)，用于将单词索引映射为给定维度的向量表示。在Transformer中，通常会将词嵌入乘以 $\sqrt{d_{\text{model}}}$ 的因子，以保持数值幅度稳定 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=super%28TokenEmbedding%2C%20self%29,emb_size%20%3D%20emb_size))。

**理论讲解：**词嵌入层可以直接使用 PyTorch 的 `nn.Embedding` 实现，将词ID映射为稠密向量。位置编码有两种常见选择：**固定位置编码**（如上述正弦余弦函数）或**可学习的位置编码**（将位置也当作参数学习）。本文采用固定位置编码，其优点是无需学习参数且在长度范围外也能外推。位置编码向量的维度与词嵌入维度相同，可以直接相加到词嵌入向量中。在实现时，我们会创建一个 `PositionalEncoding` 模块，在初始化时计算好足够长的正弦余弦位置矩阵并缓存。前向传播时，根据输入序列长度切片出对应长度的位置编码，加到输入的词向量上即可 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=def%20forward,token_embedding.size%280%29%2C))。我们也会添加 `nn.Dropout` 层防止过拟合。

下面实现位置编码模块 `PositionalEncoding` 以及词嵌入模块 `TokenEmbedding`：

```python
import math
import torch.nn as nn

class PositionalEncoding(nn.Module):
    def __init__(self, d_model: int, dropout: float = 0.1, max_len: int = 5000):
        super().__init__()
        self.dropout = nn.Dropout(dropout)
        # 创建位置编码矩阵 [max_len, d_model]
        pe = torch.zeros(max_len, d_model)
        position = torch.arange(0, max_len).unsqueeze(1)  # [max_len, 1]
        # 计算每个偶数维度的指数因子: 10000^{-2i/d_model}
        div_term = torch.exp(torch.arange(0, d_model, 2) * -(math.log(10000.0) / d_model))
        pe[:, 0::2] = torch.sin(position * div_term)  # 偶数维使用sin
        pe[:, 1::2] = torch.cos(position * div_term)  # 奇数维使用cos
        pe = pe.unsqueeze(1)  # 增加batch维度：[max_len, 1, d_model]
        # 将位置编码矩阵注册为buffer，不作为模型参数
        self.register_buffer('pe', pe)
    
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        """
        参数 x: 张量，形状 [seq_len, batch_size, d_model]
        返回: 加上位置编码的张量 (相同形状)
        """
        seq_len = x.size(0)
        # 将位置编码加到输入上，并应用Dropout
        x = x + self.pe[:seq_len]  # 广播机制将 pe[:seq_len] 加到每个batch的embedding上
        return self.dropout(x)

class TokenEmbedding(nn.Module):
    def __init__(self, vocab_size: int, emb_size: int):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, emb_size)
        self.emb_size = emb_size
    
    def forward(self, tokens: torch.Tensor) -> torch.Tensor:
        # 输出embedding并乘以sqrt(emb_size)
        return self.embedding(tokens) * math.sqrt(self.emb_size)
```

我们在 `PositionalEncoding` 初始化时预先计算最大长度 `max_len` 的位置编码矩阵，这样每次前向计算时直接索引即可，避免重复开销 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=den%20%3D%20torch.exp%28,2)) ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=pos_embedding%20%3D%20torch.zeros%28%28maxlen%2C%20emb_size%29%29%20pos_embedding,2))。`TokenEmbedding` 则简单包装了 `nn.Embedding`，并在返回前乘以 $\sqrt{d_{\text{model}}}$ ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=super%28TokenEmbedding%2C%20self%29,emb_size%20%3D%20emb_size))。接下来，我们对这两个模块进行简单测试：

```python
# 测试 PositionalEncoding 和 TokenEmbedding
d_model = 16
pos_enc = PositionalEncoding(d_model, dropout=0.0, max_len=100)
token_emb = TokenEmbedding(vocab_size=100, emb_size=d_model)
# 构造一个长度为4的序列(batch_size=1)全为0的嵌入输出
x = torch.zeros(4, 1, d_model)
# 将 x 加上位置编码
y = pos_enc(x)
# 位置0的编码在偶数维应为0，奇数维应为1（因为 sin(0)=0, cos(0)=1）
assert abs(y[0, 0, 0].item() - 0.0) < 1e-6  # 偶数维0 -> sin(0)
assert abs(y[0, 0, 1].item() - 1.0) < 1e-6  # 奇数维1 -> cos(0)
# 测试输出形状匹配
assert y.shape == x.shape
print("PositionalEncoding correct for pos=0. Output shape:", y.shape)
# 测试 TokenEmbedding
idx = torch.tensor([2, 5, 10, 3])  # 一个示例序列 (含BOS=2, EOS=3等索引)
emb = token_emb(idx.unsqueeze(1))  # [seq_len, batch, emb_size]
print("TokenEmbedding output shape:", emb.shape)
```

当我们将 `PositionalEncoding` 应用于零张量时，输出的第0位置确实只有奇数维为1，偶数维为0，这验证了位置编码的计算正确性。现在，我们已经拥有**词嵌入**和**位置编码**两部分模块，可以将它们组合用于序列标识表示。接下来，将构建 Transformer 的核心：编码器和解码器层。

### 3. 构建编码器层：多头自注意力 + 前馈网络

Transformer **编码器层**的结构包括：多头自注意力 (Multi-Head Self-Attention)、位置前馈网络 (Position-wise Feed-Forward Network)，以及在每个子层之后的残差连接和层归一化 (Layer Normalization) ([Build your own Transformer from scratch using Pytorch | by Arjun Sarkar | TDS Archive | Medium](https://medium.com/data-science/build-your-own-transformer-from-scratch-using-pytorch-84c850470dcb#:~:text=The%20Multi,aspects%20of%20the%20input%20sequence))。编码器层接受输入序列的表示（例如词嵌入+位置编码相加的结果），通过自注意力机制让序列中的每个位置**关注**序列中其他位置的内容，从而获得更丰富的表示。多头注意力包含 $h$ 个平行的注意力头，每个头在不同的子空间执行缩放点积注意力，然后将结果拼接 ([MultiheadAttention — PyTorch 2.6 documentation](https://pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html#:~:text=Multi))。这种机制允许模型从不同角度考虑序列内部不同位置之间的相关性 ([MultiheadAttention — PyTorch 2.6 documentation](https://pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html#:~:text=Allows%20the%20model%20to%20jointly,information%20from%20different%20representation%20subspaces))。

**理论讲解：** 注意力机制通过**查询(query)**、**键(key)**和**值(value)**三个矩阵来计算加权和。对于自注意力，序列的每个位置都会同时作为查询、键和值。我们以向量形式表示，第 $i$ 个位置的查询向量 $Q_i$、键向量 $K_j$ 和值向量 $V_j$，则**缩放点积注意力**计算如下：首先计算注意力得分 $e_{ij} = Q_i \cdot K_j^T / \sqrt{d_k}$（点积除以$\sqrt{d_k}$缩放避免大值 ([Build your own Transformer from scratch using Pytorch | by Arjun Sarkar | TDS Archive | Medium](https://medium.com/data-science/build-your-own-transformer-from-scratch-using-pytorch-84c850470dcb#:~:text=class%20MultiHeadAttention%28nn,must%20be%20divisible%20by%20num_heads))），然后对所有 $j$ 应用 softmax 得到归一化权重 $\alpha_{ij}$，最后输出为加权和 $Z_i = \sum_j \alpha_{ij} V_j$ ([Build your own Transformer from scratch using Pytorch | by Arjun Sarkar | TDS Archive | Medium](https://medium.com/data-science/build-your-own-transformer-from-scratch-using-pytorch-84c850470dcb#:~:text=The%20Multi,aspects%20of%20the%20input%20sequence))。在实现中，我们使用 PyTorch 内置的 `nn.MultiheadAttention` 模块，它已优化实现了多头注意力计算 ([MultiheadAttention — PyTorch 2.6 documentation](https://pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html#:~:text=,when%20possible))。MultiheadAttention 模块允许我们传入**注意力掩码(attention mask)**来遮挡不需要看的位置。在编码器中，我们主要使用**填充掩码**，以避免在注意力计算时将 `<pad>` 等填充位计算在内。

一个标准的编码器层执行以下步骤：

1. **多头自注意力子层**：输入 $X$ 经过多头自注意力得到 $X_{\text{attn}}$。然后施加残差连接：$X' = \text{LayerNorm}(X + X_{\text{attn}})$ ([Build your own Transformer from scratch using Pytorch | by Arjun Sarkar | TDS Archive | Medium](https://medium.com/data-science/build-your-own-transformer-from-scratch-using-pytorch-84c850470dcb#:~:text=class%20MultiHeadAttention%28nn,must%20be%20divisible%20by%20num_heads))。
    
2. **前馈网络子层**：$X'$ 经过两层线性变换的前馈网络（通常在中间有 ReLU 激活）得到 $X_{\text{ffn}}$ ([Build your own Transformer from scratch using Pytorch | by Arjun Sarkar | TDS Archive | Medium](https://medium.com/data-science/build-your-own-transformer-from-scratch-using-pytorch-84c850470dcb#:~:text=class%20MultiHeadAttention%28nn,must%20be%20divisible%20by%20num_heads))。再应用残差和层归一化：输出 $Y = \text{LayerNorm}(X' + X_{\text{ffn}})$。
    

前馈网络通常对每个位置独立地应用相同的两层全连接网络（隐藏层维度一般取 $4 \times d_{\text{model}}$，例如 $d_{\text{model}}=512$ 则隐藏维度 $2048$）。残差连接和层归一化有助于缓解深层网络的梯度消失，并稳定训练过程。

下面我们实现编码器层模块 `EncoderLayer`。我们将使用 `nn.MultiheadAttention` 实现多头自注意力，并使用两层全连接实现前馈网络。注意在前向过程中传入可选的 `src_mask`（序列间的注意力掩码，一般编码器不用）和 `src_key_padding_mask`（填充掩码）。

```python
class EncoderLayer(nn.Module):
    def __init__(self, d_model: int, nhead: int, dim_feedforward: int = 2048, dropout: float = 0.1):
        super().__init__()
        # 多头自注意力
        self.self_attn = nn.MultiheadAttention(d_model, nhead, dropout=dropout)
        # 前馈网络两层全连接
        self.linear1 = nn.Linear(d_model, dim_feedforward)
        self.dropout = nn.Dropout(dropout)
        self.linear2 = nn.Linear(dim_feedforward, d_model)
        # 两个LayerNorm层，对应注意力子层和前馈子层
        self.norm1 = nn.LayerNorm(d_model)
        self.norm2 = nn.LayerNorm(d_model)
        # 两个dropout层
        self.dropout1 = nn.Dropout(dropout)
        self.dropout2 = nn.Dropout(dropout)
    
    def forward(self, src: torch.Tensor, src_mask: torch.Tensor = None, src_key_padding_mask: torch.Tensor = None) -> torch.Tensor:
        # src: [seq_len, batch_size, d_model]
        # 自注意力：Q=K=V=src
        attn_output, _ = self.self_attn(src, src, src, attn_mask=src_mask,
                                       key_padding_mask=src_key_padding_mask)
        # 残差连接 + LayerNorm (注意，这里的LayerNorm是对最后一维d_model归一化)
        src = self.norm1(src + self.dropout1(attn_output))
        # 前馈网络
        ff_output = self.linear2(self.dropout(torch.relu(self.linear1(src))))
        # 残差连接 + LayerNorm
        src = self.norm2(src + self.dropout2(ff_output))
        return src
```

我们创建了 `EncoderLayer` 类，其中包含必要的子模块。需要注意，PyTorch 的 `MultiheadAttention` 默认期望输入形状为 `(seq_len, batch, d_model)`，与我们准备数据时的张量维度匹配。填充掩码 `src_key_padding_mask` 的形状应为`[batch_size, seq_len]`，元素为布尔值，`True`表示对应位置是填充，需要被注意力忽略 ([MultiheadAttention — PyTorch 2.6 documentation](https://pytorch.org/docs/stable/generated/torch.nn.MultiheadAttention.html#:~:text=,For%20a%20binary))。在上面的实现中，残差连接和 LayerNorm 是按 **后规范化 (post-norm)** 的方式，即先残差相加再规范化（与原论文一致）。

现在，我们测试编码器层的输出维度是否正确，以及掩码是否正确地应用：

```python
# 测试 EncoderLayer
encoder_layer = EncoderLayer(d_model=16, nhead=4, dim_feedforward=64, dropout=0.0)
# 构造一个长度为5、batch为2的随机输入
sample_src = torch.randn(5, 2, 16)
# 构造一个填充掩码：假设对于batch中第2个序列，后两项是填充
sample_pad_mask = torch.tensor([[False, False, False, False, False],   # 第一个序列无填充
                                [False, False, False, True, True]])    # 第二个序列后两项为填充
output = encoder_layer(sample_src, src_mask=None, src_key_padding_mask=sample_pad_mask)
# 输出应与输入形状相同
assert output.shape == sample_src.shape
print("EncoderLayer output shape:", output.shape)
```

如果实现正确，`EncoderLayer` 会将输入 `(seq_len, batch, d_model)` 映射到相同形状的输出，同时会忽略掉填充位置的影响（可以进一步检查填充位置输出是否未更新，这里不展开）。编码器层已就绪，接下来我们实现解码器层。

### 4. 构建解码器层：带掩码的自注意力 + 交叉注意力

Transformer **解码器层**与编码器层类似，也包含多头注意力、前馈网络和残差连接，不同之处在于解码器有两个多头注意力子层：

- 第一个是**掩码多头自注意力**，作用于解码器输入（已经生成的目标序列部分），通过掩码来防止当前位置看到后面的单词 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=def%20generate_square_subsequent_mask%28sz%29%3A%20mask%20%3D%20%28torch,0%29%29%20return%20mask))。
    
- 第二个是**多头交叉注意力**，它的查询来自解码器当前层的输入，键和值来自编码器的输出，这使得解码器能够关注源序列的信息。
    

掩码自注意力保证了模型在预测下一个词时**不会偷看未来的词**。通常使用一个**下三角的布尔掩码**矩阵 `tgt_mask` 来实现：大小为 `(tgt_len, tgt_len)`，下三角（包括对角线）为 False，上三角为 True 或 `-inf`（表示禁止关注）([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=def%20generate_square_subsequent_mask%28sz%29%3A%20mask%20%3D%20%28torch,0%29%29%20return%20mask)) ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=mask%20%3D%20%28torch,0%29%29%20return%20mask))。这样softmax计算注意力权重时，未来位置被屏蔽为负无穷权重，从而不起作用。

**理论讲解：** 解码器层的运算步骤：

1. **掩码多头自注意力 (Masked MHA)**：输入为当前时间步之前的目标序列表示 $T$，通过注意力获取每个位置对已生成部分的依赖。掩码保证第 $i$ 个位置只能关注 $1$ 到 $i$ 的位置（包括自身），不能看$i+1$到$tgt_len$的位置 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=def%20generate_square_subsequent_mask%28sz%29%3A%20mask%20%3D%20%28torch,0%29%29%20return%20mask))。然后执行残差和规范化：$T' = \text{LayerNorm}(T + \text{MHA}_{\text{masked}}(T))$。
    
2. **多头交叉注意力 (Cross MHA)**：以编码器输出 $M$ 作为键和值，$T'$ 作为查询计算注意力，即对每个解码端位置，让它从源序列的编码表示中选择相关信息 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=memory%20%3D%20model,max%28prob%2C%20dim%3D1))。输出经过残差和规范化：$U = \text{LayerNorm}(T' + \text{MHA}_{\text{cross}}(T', M))$。
    
3. **前馈网络**：与编码器层相同，对 $U$ 逐位置经过两层线性网络，得到 $U_{\text{ffn}}$，再残差规范化输出：$Y = \text{LayerNorm}(U + \text{FFN}(U))$。
    

下面实现解码器层 `DecoderLayer`：

```python
class DecoderLayer(nn.Module):
    def __init__(self, d_model: int, nhead: int, dim_feedforward: int = 2048, dropout: float = 0.1):
        super().__init__()
        # 掩码自注意力
        self.self_attn = nn.MultiheadAttention(d_model, nhead, dropout=dropout)
        # 交叉注意力（encoder-decoder attention）
        self.multihead_attn = nn.MultiheadAttention(d_model, nhead, dropout=dropout)
        # 前馈网络
        self.linear1 = nn.Linear(d_model, dim_feedforward)
        self.dropout = nn.Dropout(dropout)
        self.linear2 = nn.Linear(dim_feedforward, d_model)
        # 三个LayerNorm，针对 自注意力子层、交叉注意力子层 和 前馈子层
        self.norm1 = nn.LayerNorm(d_model)
        self.norm2 = nn.LayerNorm(d_model)
        self.norm3 = nn.LayerNorm(d_model)
        # 对应的dropout
        self.dropout1 = nn.Dropout(dropout)
        self.dropout2 = nn.Dropout(dropout)
        self.dropout3 = nn.Dropout(dropout)
    
    def forward(self, tgt: torch.Tensor, memory: torch.Tensor,
                tgt_mask: torch.Tensor = None, memory_mask: torch.Tensor = None,
                tgt_key_padding_mask: torch.Tensor = None, memory_key_padding_mask: torch.Tensor = None) -> torch.Tensor:
        # tgt: [tgt_len, batch, d_model], memory: [src_len, batch, d_model]
        # 1) 掩码多头自注意力 (注意 query=key=value=tgt)
        attn_output, _ = self.self_attn(tgt, tgt, tgt, attn_mask=tgt_mask,
                                        key_padding_mask=tgt_key_padding_mask)
        tgt = self.norm1(tgt + self.dropout1(attn_output))
        # 2) 交叉注意力 (query来自tgt，key和值来自encoder的memory)
        attn_output2, _ = self.multihead_attn(tgt, memory, memory, attn_mask=memory_mask,
                                             key_padding_mask=memory_key_padding_mask)
        tgt = self.norm2(tgt + self.dropout2(attn_output2))
        # 3) 前馈网络
        ff_output = self.linear2(self.dropout(torch.relu(self.linear1(tgt))))
        tgt = self.norm3(tgt + self.dropout3(ff_output))
        return tgt
```

`DecoderLayer` 与 `EncoderLayer` 十分相似，不同点在于我们增加了一个 `multihead_attn` 层来处理来自编码器的 `memory`。这里参数命名与 PyTorch `nn.TransformerDecoderLayer` 一致：`tgt`是当前解码层输入，`memory`是编码器输出。`tgt_mask` 是我们提供的未来位置掩码矩阵，`memory_key_padding_mask` 是对编码器输出的填充掩码。

我们来测试解码器层的维度和掩码：

```python
# 测试 DecoderLayer
decoder_layer = DecoderLayer(d_model=16, nhead=4, dim_feedforward=64, dropout=0.0)
# dummy输入：tgt序列长为6，src序列长为5，batch为2
sample_tgt = torch.randn(6, 2, 16)
sample_mem = torch.randn(5, 2, 16)
# 准备掩码：目标序列掩码 (6x6 下三角False)
tgt_mask = torch.triu(torch.ones((6, 6)), diagonal=1).bool()
# 填充掩码：假设第2个序列 tgt 后两步、memory后两步为填充
tgt_padding_mask = torch.tensor([[False]*6, [False, False, False, True, True, True]])
mem_padding_mask = torch.tensor([[False]*5, [False, False, False, False, True]])
out = decoder_layer(sample_tgt, sample_mem, tgt_mask=tgt_mask,
                    tgt_key_padding_mask=tgt_padding_mask, memory_key_padding_mask=mem_padding_mask)
assert out.shape == sample_tgt.shape
print("DecoderLayer output shape:", out.shape)
```

我们构造了一个简单的掩码（其中 `tgt_mask` 上三角为 True 表示要屏蔽）。通过断言输出形状与输入 `tgt` 相同，基本验证了解码器层的流程。注意：更多测试例如检查掩码是否生效（比如确保输出在被掩码的位置没有利用信息）比较复杂，在此不详细测试。至此，我们已经实现了 Transformer 的基本组成单元：**编码器层**和**解码器层**。

### 5. 组装完整 Transformer 模型

有了编码器层和解码器层，我们就可以堆叠它们来构建完整的 Transformer 编码器和解码器，并整合前面的嵌入层和位置编码。典型地，Transformer 使用 $N$ 层编码器和 $N$ 层解码器（原论文中 $N=6$）。在我们的实现中，我们会让层数可配置。编码器由多个 `EncoderLayer` 堆叠而成，每层接受前一层输出（第一个编码器层接受词嵌入+位置编码作为输入）。解码器类似，同时每个解码器层都会接收编码器最终的输出作为额外输入。

完整的 Transformer 模型 (Seq2Seq 模型) 包括：

- **源嵌入层 + 源位置编码**：将源句子词ID张量转换为词向量，并加上位置编码。
    
- **目标嵌入层 + 目标位置编码**：将目标序列（训练时使用强制教学的前缀）转为词向量并加位置编码。
    
- **编码器**：若干层 `EncoderLayer`，将源序列的表示编码为更高层语义表示（输出 memory）。
    
- **解码器**：若干层 `DecoderLayer`，每层接受上一层输出和编码器的输出 memory，逐步融合源信息来编码目标序列。
    
- **输出线性层**：将解码器最后一层每个位置的输出向量映射到目标词表大小的 logits 分布。每个位置的 softmax 概率分布代表下一个预测词的概率。
    

在实现中，我们将上述组件组合到一个 `TransformerModel` (`Seq2SeqTransformer`) 类中。需要注意以下几点：

- 在训练时，**目标序列输入**通常是完整目标序列右移一位（即开头有`<bos>`，去掉末尾`<eos>`），模型输出与之对应的预测序列（应对齐原目标序列除`<bos>`部分，即第一个实际词到`<eos>`）。这种对齐方式将通过数据喂入来控制，模型本身不处理序列对齐。
    
- **掩码的准备**：编码器的 `src_mask` 在本例中不需要（填充掩码已经处理需要忽略的位置，自注意力不需要未来掩码，因为可以看全序列），我们传入 `None` 或全 False 矩阵即可 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=tgt_mask%20%3D%20generate_square_subsequent_mask,bool))。解码器的 `tgt_mask` 我们会传入下三角 False 掩码矩阵，`memory_mask`通常不使用（我们不遮盖编码器输出，除非做类似于BERT那种双向遮挡）。另外还需传入前面计算的 `src_key_padding_mask` 和 `tgt_key_padding_mask` 以及 `memory_key_padding_mask`（通常与 `src_key_padding_mask`相同），以在注意力中忽略填充 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=tgt_mask%20%3D%20generate_square_subsequent_mask,bool))。
    

现在，实现 Transformer 完整模型类：

```python
class TransformerModel(nn.Module):
    def __init__(self, src_vocab_size: int, tgt_vocab_size: int,
                 d_model: int = 512, nhead: int = 8,
                 num_encoder_layers: int = 6, num_decoder_layers: int = 6,
                 dim_feedforward: int = 2048, dropout: float = 0.1):
        super().__init__()
        # 嵌入和位置编码
        self.src_tok_emb = TokenEmbedding(src_vocab_size, d_model)
        self.tgt_tok_emb = TokenEmbedding(tgt_vocab_size, d_model)
        self.positional_encoding = PositionalEncoding(d_model, dropout=dropout)
        # 编码器和解码器堆叠
        self.encoder = nn.ModuleList([EncoderLayer(d_model, nhead, dim_feedforward, dropout) 
                                      for _ in range(num_encoder_layers)])
        self.decoder = nn.ModuleList([DecoderLayer(d_model, nhead, dim_feedforward, dropout) 
                                      for _ in range(num_decoder_layers)])
        # 输出线性层
        self.generator = nn.Linear(d_model, tgt_vocab_size)
    
    def forward(self, src: torch.Tensor, tgt: torch.Tensor,
                src_mask: torch.Tensor = None, tgt_mask: torch.Tensor = None,
                src_key_padding_mask: torch.Tensor = None,
                tgt_key_padding_mask: torch.Tensor = None,
                memory_key_padding_mask: torch.Tensor = None) -> torch.Tensor:
        # 1. 嵌入 + 位置编码
        src_emb = self.positional_encoding(self.src_tok_emb(src))   # [src_len, batch, d_model]
        tgt_emb = self.positional_encoding(self.tgt_tok_emb(tgt))   # [tgt_len, batch, d_model]
        # 2. 编码器：逐层传递
        memory = src_emb
        for layer in self.encoder:
            memory = layer(memory, src_mask=src_mask, src_key_padding_mask=src_key_padding_mask)
        # 3. 解码器：逐层传递，注意传入编码器输出 memory
        output = tgt_emb
        for layer in self.decoder:
            output = layer(output, memory, tgt_mask=tgt_mask,
                           tgt_key_padding_mask=tgt_key_padding_mask,
                           memory_key_padding_mask=memory_key_padding_mask)
        # 4. 输出线性层映射到词表维度
        out_logits = self.generator(output)  # [tgt_len, batch, tgt_vocab_size]
        return out_logits
```

在上述实现中，我们用 Python 列表手动堆叠了多层编码器和解码器（也可以使用`nn.TransformerEncoder`等高级API，这里为了清晰起见直接堆叠）。编码器对 `src_emb` 连续应用 `num_encoder_layers` 次编码器层，得到编码后的 `memory` 表示。解码器则对 `tgt_emb` 连续应用多层解码器层，在每一层都使用相同的 `memory`（即最后一层编码器输出）。最后通过 `generator` 将每个位置的隐状态映射为词表大小的分数向量。

在构造模型后，我们通常需要对模型参数进行初始化。原论文使用了基于均匀分布的 Xavier 初始化来随机初始化权重 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=for%20p%20in%20transformer,1%3A%20nn.init.xavier_uniform_%28p))。我们也采用类似方式对线性层和嵌入层权重进行初始化（PyTorch 默认embedding已是Uniform初始化）。下面示范构建一个小Transformer模型并初始化参数，然后进行一次前向传递测试输出维度：

```python
# 实例化一个小规模 Transformer 模型用于测试
temp_model = TransformerModel(src_vocab_size=len(vocab_transform[SRC_LANGUAGE]),
                              tgt_vocab_size=len(vocab_transform[TGT_LANGUAGE]),
                              d_model=32, nhead=4, num_encoder_layers=2, num_decoder_layers=2,
                              dim_feedforward=64, dropout=0.1)
# 参数初始化：对多维权重采用Xavier均匀分布初始化
for p in temp_model.parameters():
    if p.dim() > 1:
        nn.init.xavier_uniform_(p)
# 准备一个 dummy 输入：src_len=3, tgt_len=4, batch_size=2
dummy_src = torch.randint(0, len(vocab_transform[SRC_LANGUAGE]), (3, 2))
dummy_tgt = torch.randint(0, len(vocab_transform[TGT_LANGUAGE]), (4, 2))
# 生成掩码（这里简单采用全False的mask，填充mask留空表示无填充）
out_logits = temp_model(dummy_src, dummy_tgt, src_mask=None, tgt_mask=None,
                        src_key_padding_mask=None, tgt_key_padding_mask=None,
                        memory_key_padding_mask=None)
print("Transformer output shape:", out_logits.shape)
# 验证输出维度：应为 [tgt_len, batch, tgt_vocab_size]
assert out_logits.shape[0] == dummy_tgt.shape[0] and out_logits.shape[1] == dummy_tgt.shape[1]
assert out_logits.shape[2] == len(vocab_transform[TGT_LANGUAGE])
print("Output vocab size:", out_logits.shape[2], "Matches target vocab:", len(vocab_transform[TGT_LANGUAGE]))
```

以上测试构造了一个较小的 Transformer 模型并进行了随机输入的前向计算，验证输出张量的形状为 `(tgt_len, batch_size, tgt_vocab_size)`，这和我们的预期一致。我们已经成功搭建了 Transformer 模型的结构，接下来为模型配置合适的**损失函数**和**优化器**，以便开始训练。

### 6. 损失函数与优化器：交叉熵+标签平滑 & Adam

对于序列到序列的词预测任务，通常使用**交叉熵损失**来训练模型。具体来说，我们会将模型输出的每个时间步的词概率分布与目标序列对应位置的真实词（标签）计算交叉熵。在实现中，可以使用 PyTorch 提供的 `nn.CrossEntropyLoss`。需要注意的是，我们在批处理中用 `<pad>` 填充的部分不应产生梯度影响，因此会将这些位置的损失忽略。PyTorch 的交叉熵支持参数 `ignore_index` 来忽略某个类别的损失计算 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=loss_fn%20%3D%20torch))。我们会将 `<pad>` 的索引 (1) 传给 `ignore_index`，这样计算损失时填充部分的预测将被跳过。

此外，Transformer 论文中引入了**标签平滑(label smoothing)**技巧 ([What is the formula for cross entropy loss with label smoothing?](https://discuss.pytorch.org/t/what-is-the-formula-for-cross-entropy-loss-with-label-smoothing/149848#:~:text=What%20is%20the%20formula%20for,label%20smoothing%20concept))。标签平滑指在训练时不使用完全“硬”的 one-hot 标签分布，而是将一小部分概率分给其他非正确词，以防止模型过度自信过拟合。通常做法是在正确标签上保留 `1 - \epsilon` 的概率，在其余类别上均匀分配 $\epsilon$ 的概率总和。论文中 $\epsilon=0.1`，即把10%的概率均匀给错词。这一技巧可通过 PyTorch 的` CrossEntropyLoss(label_smoothing=0.1)` 参数直接实现 ([What is the formula for cross entropy loss with label smoothing?](https://discuss.pytorch.org/t/what-is-the-formula-for-cross-entropy-loss-with-label-smoothing/149848#:~:text=What%20is%20the%20formula%20for,label%20smoothing%20concept))。

**理论讲解：** 交叉熵损失定义为 $-\sum_{i} y_i \log p_i$，其中 $y_i$ 是目标分布，$p_i$ 是预测分布的概率。对于one-hot标签，正确类别$y_{true}=1$其余为0，损失就是 $-\log p_{true}$。标签平滑将标签分布调整为非零的更平滑分布，从而惩罚模型过度将概率集中在单一类别。适度的平滑可以提升模型泛化能力。

**优化器**方面，Transformer 原论文使用了基于 Adam 的自适应优化器以及自定义的学习率调度（warm-up策略）。在本教程中，我们使用 Adam 优化器即可 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=loss_fn%20%3D%20torch))。我们可以使用与论文类似的超参数，如学习率 1e-4，$\beta_1=0.9, \beta_2=0.98$，以及$\epsilon=1e-9$ ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=loss_fn%20%3D%20torch))。如果时间允许，可进一步实现学习率随训练过程的调整策略（warmup + 下降），但这里不作展开，默认使用固定学习率即可取得不错效果。

下面配置损失函数和优化器：

```python
import torch.optim as optim

# 定义损失函数 (交叉熵)，忽略<PAD>标签，使用标签平滑
pad_idx = PAD_IDX  # PAD_IDX = 1
loss_fn = nn.CrossEntropyLoss(ignore_index=pad_idx, label_smoothing=0.1)
# 定义优化器 (Adam)
optimizer = optim.Adam(temp_model.parameters(), lr=1e-4, betas=(0.9, 0.98), eps=1e-9)
```

此处我们将 `ignore_index` 设为 `PAD_IDX`，这样计算损失时，目标为 `<pad>` 的位置其损失将被忽略，不影响梯度。([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=loss_fn%20%3D%20torch))设置了 `label_smoothing=0.1` 来平滑标签分布。如果不想使用平滑，可将其设置为0。在优化器方面，我们使用了较小的学习率 1e-4 以及稍大的 $\beta_2$（0.98），这些设置在 Transformer 中较常见，被认为可以稳健训练 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=loss_fn%20%3D%20torch))。

我们可以做一个简单测试：假设模型预测完全正确，损失应当很低；如预测和目标完全相同则BLEU为1但交叉熵接近0。这里简单用已构造的 `temp_model` 来计算一次损失：

```python
# 生成一些模拟输出和目标来测试loss的忽略填充效果
logits = torch.randn(5, 3, len(vocab_transform[TGT_LANGUAGE]))  # 假设输出分布
target = torch.randint(0, len(vocab_transform[TGT_LANGUAGE]), (5, 3))
# 强制将某些位置设为PAD以模拟填充
target[2:, :] = PAD_IDX
loss_val = loss_fn(logits.view(-1, logits.size(-1)), target.view(-1))
print("Sample loss (w/ random logits):", loss_val.item())
```

如上，`loss_fn` 会自动忽略 `target` 中值为 `PAD_IDX` 的那些元素的损失。现在，我们已经准备好模型和优化器，可以进入训练过程。

### 7. 模型训练与验证：Teacher Forcing 教师强制策略

在训练序列到序列模型（如翻译）时，常用**教师强制 (teacher forcing)**策略 ([What is Teacher Forcing for Recurrent Neural Networks? - MachineLearningMastery.com](https://www.machinelearningmastery.com/teacher-forcing-for-recurrent-neural-networks/#:~:text=Teacher%20forcing%20is%20a%20method,prior%20time%20step%20as%20input))：在每一步预测下一个词时，模型的输入使用**真实的先前目标词**，而不是模型上一步预测的词。Teacher forcing 可以加速模型收敛 ([What is Teacher Forcing for Recurrent Neural Networks? - MachineLearningMastery.com](https://www.machinelearningmastery.com/teacher-forcing-for-recurrent-neural-networks/#:~:text=Teacher%20forcing%20is%20a%20method,prior%20time%20step%20as%20input))。不过需要注意的是，训练阶段模型总是看到正确的前文，但推理阶段不会有参考答案，可能造成所谓“曝光偏差”。尽管如此，大多数序列训练还是采用教师强制来提高效率。

在我们的训练循环中，对于每个批次的数据，我们将**目标序列**按如下方式拆分：

- `tgt_input`: 目标序列去掉最后一个词 (即不包含 `<eos>`)。开头仍有 `<bos>`，表示模型的起始输入。
    
- `tgt_output`: 目标序列去掉第一个词 (即不包含 `<bos>`)，这样对应的位置就是模型需要预测的下一个词序列，包括最终的 `<eos>`。
    

例如，真实目标序列是「`<bos>` A B C `<eos>`」，则 `tgt_input` 是「`<bos>` A B C」；而 `tgt_output`（模型应预测）是「A B C `<eos>`」。模型在时间步1输入`<bos>`，应输出`A`；输入`A`输出`B`，...，输入`C`输出`<eos>`。

**数据准备和掩码：** 在每个迭代中，我们从 `DataLoader` 取出 `(src, tgt)`：形状分别为 `[src_len, batch]` 和 `[tgt_len, batch]`，其中 `tgt` 包含`<bos>... <eos>`。我们构造 `tgt_input = tgt[:-1]`，`tgt_output = tgt[1:]`。随后，我们需要为当前批次动态生成掩码：

- `src_mask`: 编码器注意力掩码，这里不需要未来信息遮挡，可用全 False 矩阵或 None ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=tgt_mask%20%3D%20generate_square_subsequent_mask,bool))。
    
- `tgt_mask`: 大小为 `(tgt_len-1, tgt_len-1)` 的后向掩码矩阵，屏蔽序列中后面的信息 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=def%20generate_square_subsequent_mask%28sz%29%3A%20mask%20%3D%20%28torch,0%29%29%20return%20mask))。
    
- `src_key_padding_mask`: 大小 `[batch, src_len]` 的布尔矩阵，指示源句填充部分 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=src_mask%20%3D%20torch))。
    
- `tgt_key_padding_mask`: 大小 `[batch, tgt_len-1]`，指示目标输入序列填充部分。
    

我们将编写一个辅助函数 `create_mask` 根据批次的 `src` 和 `tgt_input` 生成上述掩码张量。然后进入训练循环，对每个批次前向计算损失，反向传播并更新参数。我们也会定义验证 (evaluation) 函数，在每个epoch结束时评估模型在验证集上的损失，以跟踪训练进展，防止过拟合。

```python
def create_mask(src: torch.Tensor, tgt: torch.Tensor):
    src_seq_len = src.size(0)
    tgt_seq_len = tgt.size(0)
    # 编码器不需要mask未来词，创建全零mask
    src_mask = torch.zeros((src_seq_len, src_seq_len), device=src.device).bool()
    # 解码器的目标掩码：上三角为 True（需要屏蔽），下三角为 False（保留）
    tgt_mask = torch.triu(torch.ones((tgt_seq_len, tgt_seq_len), device=tgt.device), diagonal=1).bool()
    # 填充掩码：True表示对应位置是<PAD>
    src_padding_mask = (src == PAD_IDX).transpose(0, 1)   # [batch, src_len]
    tgt_padding_mask = (tgt == PAD_IDX).transpose(0, 1)   # [batch, tgt_len]
    return src_mask, tgt_mask, src_padding_mask, tgt_padding_mask

def train_epoch(model: nn.Module, optimizer: optim.Optimizer, dataloader: DataLoader, device: torch.device):
    model.train()
    total_loss = 0
    for src, tgt in dataloader:
        src = src.to(device)    # [src_len, batch]
        tgt = tgt.to(device)    # [tgt_len, batch]
        tgt_input = tgt[:-1, :]  # [tgt_len-1, batch]
        # 生成mask
        src_mask, tgt_mask, src_padding_mask, tgt_padding_mask = create_mask(src, tgt_input)
        # 前向计算
        logits = model(src, tgt_input, src_mask=src_mask, tgt_mask=tgt_mask,
                       src_key_padding_mask=src_padding_mask,
                       tgt_key_padding_mask=tgt_padding_mask,
                       memory_key_padding_mask=src_padding_mask)
        # 计算损失：将输出与 tgt 的下一个词比较
        tgt_out = tgt[1:, :]    # [tgt_len-1, batch]
        loss = loss_fn(logits.view(-1, logits.size(-1)), tgt_out.reshape(-1))
        # 反向传播和参数更新
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        total_loss += loss.item()
    return total_loss / len(dataloader)

def evaluate(model: nn.Module, dataloader: DataLoader, device: torch.device):
    model.eval()
    total_loss = 0
    with torch.no_grad():
        for src, tgt in dataloader:
            src = src.to(device)
            tgt = tgt.to(device)
            tgt_input = tgt[:-1, :]
            src_mask, tgt_mask, src_padding_mask, tgt_padding_mask = create_mask(src, tgt_input)
            logits = model(src, tgt_input, src_mask, tgt_mask,
                           src_key_padding_mask=src_padding_mask,
                           tgt_key_padding_mask=tgt_padding_mask,
                           memory_key_padding_mask=src_padding_mask)
            tgt_out = tgt[1:, :]
            loss = loss_fn(logits.view(-1, logits.size(-1)), tgt_out.reshape(-1))
            total_loss += loss.item()
    return total_loss / len(dataloader)
```

在 `train_epoch` 中，我们对每个批次执行前向、计算损失、反向传播、优化器更新，并积累损失。`evaluate` 则不更新模型，只计算平均损失。请注意，在计算 `logits` 时我们传入了同一个 `src_padding_mask` 作为 `memory_key_padding_mask`，因为编码器输出的填充部分需要在解码器的交叉注意力中被忽略 ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=src_mask%2C%20tgt_mask%2C%20src_padding_mask%2C%20tgt_padding_mask%20%3D,src%2C%20tgt_input)) ([Translating Languages using PyTorch Transformers | RipeSeed](https://ripeseed.io/blog/translating-languages-using-py-torch-transformers#:~:text=src_mask%2C%20tgt_mask%2C%20src_padding_mask%2C%20tgt_padding_mask%20%3D,src%2C%20tgt_input))。

接下来，我们准备训练若干 epoch，并在每个 epoch 后输出训练和验证集损失：

```python
# 假设已经创建了 train_dataloader 和 val_dataloader
device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
model = temp_model.to(device)  # 使用前面temp_model或新建TransformerModel
NUM_EPOCHS = 5
for epoch in range(1, NUM_EPOCHS+1):
    train_loss = train_epoch(model, optimizer, train_dataloader, device)
    # 假设我们也有 val_dataloader 类似于 train_dataloader
    # 这里为简洁起见，可用train数据代替验证或者实际构建val迭代器
    val_loss = evaluate(model, train_dataloader, device)  
    print(f"Epoch {epoch}: Training loss: {train_loss:.4f}, Validation loss: {val_loss:.4f}")
```

以上代码演示了一个训练循环。在实际使用中，请确保提供正确的验证集 DataLoader（例如 `val_dataloader`）。每个 epoch 后我们打印训练和验证损失，理想情况下，损失会逐渐下降。训练若干 epoch 后，模型应该能够从源语言生成对应的目标语言句子轮廓。

由于训练可能耗时，我们此处不打印具体输出示例。下一步，我们关注在训练完成后，如何使用模型进行翻译推理（即不再有参考答案，模型需自回归地生成整句翻译）。

### 8. 推理与生成翻译：Greedy 解码

训练完成后，我们将模型用于实际翻译任务。解码过程需要**自回归地生成序列**：模型一次预测一个词，随后将该预测作为输入继续预测下一个，直到生成终止符 `<eos>` 或达到最大长度。这里我们实现**贪心解码 (greedy decoding)**，每一步都选择概率最高的词作为输出。贪心策略速度快但可能错过全局最优翻译；更先进的方法是**束搜索 (beam search)**，保留 $k$ 个概率最高的部分翻译路径再扩展，可以提高翻译质量但实现较复杂。在本教程中我们先实现贪心解码。

**解码步骤**：

1. 对于给定源句子，首先用训练时相同的分词和数值化流程获得其索引张量，加上 `<bos>` 和 `<eos>`（或我们可以在生成过程中手动添加 `<bos>`）。
    
2. 将源序列通过编码器得到 `memory` 表示。
    
3. 初始化目标序列为只包含 `<bos>` 的张量 `ys`。
    
4. 迭代以下过程，直到生成 `<eos>` 或达到最大长度：
    
    - 使用当前的 `ys` 通过解码器计算输出 logits。
        
    - 取出最后一个时间步的预测分布，选择概率最大的词 `next_word`。
        
    - 将 `next_word` 附加到 `ys` 序列末尾。
        
5. 返回生成的完整序列。最后，可以通过 `vocab_transform` 将索引转回单词。
    

在实际实现时，需要注意将模型切换到评估模式 (`model.eval()`)，并且在推理时关闭梯度计算（用 `torch.no_grad()`）以提高效率。

我们先实现一个帮助函数 `greedy_decode` 来对单个句子的源序列tensor生成目标序列tensor：

```python
def greedy_decode(model, src, max_len, start_symbol):
    model.eval()
    src = src.to(device)
    src_mask = torch.zeros(src.shape[0], src.shape[0], device=device).bool()  # 不遮掩源序列
    # 编码器得到memory
    memory = src
    for layer in model.encoder:
        memory = layer(memory, src_mask=src_mask, src_key_padding_mask=None)
    # 解码过程
    ys = torch.tensor([[start_symbol]], device=device)  # 初始输出序列只包含<BOS>
    for i in range(max_len - 1):
        tgt_mask = torch.triu(torch.ones((ys.size(0), ys.size(0)), device=device), diagonal=1).bool()
        out = ys
        # 通过所有解码器层
        for layer in model.decoder:
            out = layer(out, memory, tgt_mask=tgt_mask, tgt_key_padding_mask=None, memory_key_padding_mask=None)
        out_logits = model.generator(out)  # 当前解码序列的输出分布
        next_token = out_logits[-1, 0].argmax(-1)  # 取最后一个时间步的输出，选概率最高的token
        next_token = next_token.unsqueeze(0).unsqueeze(0)  # shape (1,1)
        ys = torch.cat([ys, next_token], dim=0)  # 将新token添加到序列
        if next_token.item() == EOS_IDX:
            break
    return ys
```

函数 `greedy_decode` 返回生成的目标序列索引张量（包含 `<bos>` 和 `<eos>`）。接下来，我们编写一个 `translate` 函数，将原始源文本字符串翻译为目标文本字符串。该函数会对输入句子进行分词和数值化，然后调用 `greedy_decode` 得到翻译序列，再把序列索引转换回单词。

```python
def translate(model, src_sentence: str):
    model.eval()
    # 对输入句子进行和训练数据相同的预处理
    src_tensor = text_transform[SRC_LANGUAGE](src_sentence).view(-1, 1).to(device)
    # 使用贪心解码生成翻译序列索引（包括BOS和EOS）
    tgt_indices = greedy_decode(model, src_tensor, max_len=src_tensor.size(0)+20, start_symbol=BOS_IDX)
    tgt_indices = tgt_indices.flatten().cpu().numpy()
    # 映射回单词，并去除<BOS>和<EOS>
    tgt_tokens = vocab_transform[TGT_LANGUAGE].lookup_tokens(list(tgt_indices))
    # 移除可能存在的<BOS>和<EOS>
    tgt_tokens = [tok for tok in tgt_tokens if tok not in ("<bos>", "<eos>")]
    return " ".join(tgt_tokens)
```

现在，我们可以使用 `translate` 函数测试模型对德语句子的翻译。例如：

```python
# 示例翻译（需要使用训练好的模型，此处假设 model 已经训练好若干epoch）
test_sentence = "Ein kleines Mädchen spielt im Garten."
print("Source (DE):", test_sentence)
print("Translation (EN):", translate(model, test_sentence))
```

如果模型训练充分，输出可能是类似“**A little girl is playing in the garden.**”。在我们的设置中，由于模型容量和训练时间有限，翻译结果可能不完全准确，但应能生成合乎语法的句子。通过进一步训练更多轮次、使用更大的模型（更大的 `d_model` 和更多层）和束搜索解码，翻译质量可以提升。

_提示：_ 上述 `greedy_decode` 实现为了直观，直接在循环中每次通过所有解码层计算。如果需要提高解码效率，可以将解码器改写为每次只计算新添的最后一个词的增量表示（缓存先前计算结果），但实现较复杂，这里不深入。

### 9. 模型评估：计算 BLEU 分数

为了客观评估机器翻译模型的性能，常用评价指标是 **BLEU** (Bilingual Evaluation Understudy) 分数。BLEU 分数通过比较模型翻译输出与参考译文，在字串层面计算 n-gram 的匹配情况来衡量翻译质量 ([[PDF] BLEU: a Method for Automatic Evaluation of Machine Translation](https://aclanthology.org/P02-1040.pdf#:~:text=Translation%20aclanthology,For%20this%20reason%2C%20even))。其取值通常在0到100之间，值越高表示模型翻译与参考越接近；例如，完美匹配参考的翻译将得到 100 分 ([[PDF] BLEU: a Method for Automatic Evaluation of Machine Translation](https://aclanthology.org/P02-1040.pdf#:~:text=Translation%20aclanthology,For%20this%20reason%2C%20even))。

BLEU 的计算考虑了不同长度的 n-gram 精确匹配率，并通过惩罚过短翻译的**长度惩罚(brevity penalty)**来平衡翻译长度 ([[PDF] BLEU: a Method for Automatic Evaluation of Machine Translation](https://aclanthology.org/P02-1040.pdf#:~:text=Translation%20aclanthology,For%20this%20reason%2C%20even))。简单来说：

- 逐个比较机器翻译和参考翻译的 1-gram、2-gram...4-gram 重合情况，计算精确率。
    
- 将各n-gram精确率取几何平均，再乘以长度惩罚因子，即得到BLEU分数 ([[PDF] BLEU: a Method for Automatic Evaluation of Machine Translation](https://aclanthology.org/P02-1040.pdf#:~:text=Translation%20aclanthology,For%20this%20reason%2C%20even))。
    

在实践中，直接实现 BLEU 计算需要处理token匹配计数等细节。这里我们推荐使用现有工具，例如 **sacreBLEU** 库或 **NLTK** 中的 BLEU 实现。**sacreBLEU** 是一个标准且易用的工具，它能计算与学术报告一致的BLEU分数。

我们可以使用 sacreBLEU 来评价模型：对一批测试集句子，用模型生成翻译，然后将生成结果与参考翻译一起传给 sacreBLEU 计算分数。下面以简单示例演示如何使用 sacreBLEU：

```python
!pip install sacrebleu  # 如果未安装，需要先安装

import sacrebleu
# 假设我们有一些句子的参考翻译和模型翻译
references = ["The cat is on the mat.", "I am a student."]
hypotheses = ["The cat is on the mat.", "I am student."]
# sacrebleu期望输入：hypotheses列表，以及一个包含参考列表的列表（支持多参考翻译）
bleu = sacrebleu.corpus_bleu(hypotheses, [references])
print(f"BLEU score = {bleu.score:.2f}")
```

在上面的例子中，第一个句子完全匹配参考，第二个句子缺少一个单词“a”，所以整体BLEU会稍有降低。如果模型输出与参考完全一致，BLEU会是100；如果完全不相关则接近0。通常训练好的翻译模型在测试集上的 BLEU 可能在20-40左右（具体取决于训练数据量和难度）。

我们也可以对相同句子测试，以验证极端情况：

```python
# 简单验证: 完全相同的句子BLEU应很高，完全不同行的应很低
refs = ["hello world"]
hyps1 = ["hello world"]       # 完全相同
hyps2 = ["goodbye"]           # 完全不同
bleu1 = sacrebleu.corpus_bleu(hyps1, [refs])
bleu2 = sacrebleu.corpus_bleu(hyps2, [refs])
print("Identity BLEU:", bleu1.score)  # 接近100
print("Different BLEU:", bleu2.score)  # 接近0
assert int(bleu1.score) == 100 and bleu2.score < 1e-6
```

通过 BLEU 分数，我们可以评估模型在测试集上的整体翻译质量。需要注意，BLEU 作为自动评价指标并不完美，有时模型翻译与参考意思相近但用词不同，也可能导致BLEU较低。因此，可结合人工评估来更全面地判断翻译效果。

### 10. 模型保存与加载，测试实际翻译

最后，在训练完成后，我们应该将模型保存，以便后续加载使用或继续训练。PyTorch 推荐使用 `torch.save()` 保存模型的**状态字典(state_dict)**，然后使用相同模型结构加载该字典。保存状态字典比直接保存整个模型对象更灵活，也避免跨版本不兼容问题。

**模型保存：**

```python
MODEL_PATH = "transformer_model.pth"
torch.save(model.state_dict(), MODEL_PATH)
print("Model saved.")
```

这会将模型参数保存到指定路径（约几十MB到几百MB，取决于模型大小）。同时，我们也应保存词汇表 `vocab_transform` 等必要的信息，因为推理时需要用相同的词表进行数值映射。词表可以使用 `pickle` 保存，或者保存整个 `vocab_transform` 对象。

**模型加载：**

在需要使用模型时，先实例化相同结构的模型，然后加载状态字典：

```python
model_loaded = TransformerModel(src_vocab_size=len(vocab_transform[SRC_LANGUAGE]),
                                tgt_vocab_size=len(vocab_transform[TGT_LANGUAGE]),
                                d_model=32, nhead=4, num_encoder_layers=2, num_decoder_layers=2,
                                dim_feedforward=64, dropout=0.1).to(device)
model_loaded.load_state_dict(torch.load(MODEL_PATH, map_location=device))
model_loaded.eval()
print("Model loaded and set to eval mode.")
```

确保 `TransformerModel(...)` 的超参数与保存时的模型一致。成功加载后，`model_loaded` 的权重应与原模型相同。我​​们可以简单验证某几个参数是否一致：

```python
for name, param in model_loaded.named_parameters():
    orig_param = model.state_dict()[name]
    if param.requires_grad:
        assert torch.allclose(param, orig_param, atol=1e-6)
print("Loaded model parameters match original.")
```

此断言通过则说明模型权重完好无损地加载。现在我们可以用 `model_loaded` 来进行翻译。比如：

```python
test_sentence = "Ein Mann isst einen Apfel."
print("Source:", test_sentence)
print("Translation:", translate(model_loaded, test_sentence))
```

输出应是对德语句子的英语翻译。例如，若模型训练良好，可能得到**“A man is eating an apple.”**。

到这里，我们已经完成了从数据预处理、模型搭建、模型训练到推理评估的完整流程。您可以尝试用自己选择的英-德句子来测试模型翻译效果，并计算其 BLEU 分数。

_提示：_ 本教程使用了一个相对小规模的 Transformer 模型和数据集以便演示。如果追求更高翻译质量，可以增大 `d_model`、增加编码器/解码器层数，并使用更大的数据集（如 WMT）。训练这样的模型需要GPU和较长时间，但翻译效果也会显著提升。另外，可以尝试实现**束搜索**解码来替代贪心解码，从而可能得到更好的翻译结果。

**总结：**我们通过10个循序渐进的练习，从零实现了Transformer模型用于机器翻译任务。从数据处理、模型各组件实现，到训练策略和评估指标，每一步都进行了讲解和代码验证。希望通过本教程，读者对 Transformer 的工作原理和实现细节有了深入直观的理解，也为进一步研究更高级的序列建模奠定了基础。
