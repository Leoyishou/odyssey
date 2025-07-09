---
draw:
title: Transformer_中英翻译任务
date created: 2025-03-29
date modified: 2025-03-29
---

transformer_learn.ipynb

https://colab.research.google.com/drive/1J3kj7cGJIFMurjnbW_Bphn65FILWDrMp#scrollTo=AN5oDdVXx5UT

下面给出一套**20 小时左右**、**Exercism 风格**的循序渐进教程，演示如何从零搭建一个用于**中英机器翻译**的 Transformer 模型，并在 PyTorch 中完成训练、推理和评估。该教程与前面英德翻译版本类似，但将数据管线改为**中文→英文**的场景，以帮助读者熟悉处理中文文本的流程。每个步骤包含核心概念的通俗介绍、相关代码实现以及简单的验证方式。请**按照顺序**逐步完成每个练习。

---

## 教程概要

1. **数据集准备**
    - 获取中英平行语料并进行分词、词表构建、DataLoader 封装
    - （可选）自定义小型 toy 数据或使用公开语料（如 WMT、IWSLT）
        
2. **位置编码与嵌入层**
3.
    - 实现位置编码（Positional Encoding）与词嵌入（Embedding），处理中文和英文词向量
        
3. **编码器模块**
    
    - 搭建多头自注意力 + 前馈网络 + 残差连接 + LayerNorm 的编码器层
        
4. **解码器模块**
    
    - 搭建解码器层（带掩码自注意力、交叉注意力等）
        
5. **组装完整 Transformer**
    
    - 将编码器、解码器堆叠，并加上词嵌入、位置编码和输出线性层
        
6. **损失函数和优化器**
    
    - 配置带标签平滑的交叉熵损失，忽略 `<pad>` 的部分；使用 Adam 等优化器
        
7. **训练与验证循环**
    
    - 使用 _teacher forcing_ 策略，编写多轮 Epoch 训练，监控验证集损失
        
8. **推理与生成翻译**
    
    - 使用贪心解码（Greedy Decoding）在测试时自回归生成英文翻译
        
9. **模型评估**
    
    - 通过 BLEU 分数等指标评估翻译质量
        
10. **模型保存与加载**
    

- 保存训练好的 Transformer 并加载进行实际翻译测试
    

---

以下演示基于 **PyTorch**、**torchtext**（或自定义数据加载）以及一些辅助工具库（如 `jieba` 或 Hugging Face `transformers` tokenizer 来切分中文）。如果你在 Google Colab 环境中运行，通常已经内置了 PyTorch，如未安装请先执行：

```bash
!pip install torch torchtext
```

中文分词可选择多种方案，示例中演示使用 `jieba`（或 `transformers` 中的 `BertTokenizer` 等）对中文进行分词。英文可以继续使用 `spacy` 或 `nltk`。如果不想依赖太多外部工具，也可以写一个非常简单的自定义分词函数（对中文按字切分等）。

在开始之前，请确保你对 PyTorch 的张量操作、自动微分、nn.Module、DataLoader 等基础知识已经熟悉（可参见你之前的 PyTorch 基础 8 步教程）。下面我们进入**分步练习**。

---

## 第1步：数据集准备（中英平行语料、分词与词表）

**理论介绍：**  

机器翻译依赖**平行语料**（源语言与目标语言句子一一对应）。对中文→英文，我们可以使用公开[[数据集]]（如 [WMT](https://huggingface.co/datasets/wmt19) 或 [IWSLT](https://huggingface.co/datasets/iwslt2017) 的中-英子集），也可自己收集小规模数据进行实验。本示例为了简单，假设我们已经有一小部分中英句子对。数据处理包含：

1. **分词**：将中文句子用 `jieba`（或其他工具）分词，把英文句子用 `spacy` 分词（或自行分割）。
2. **词表(vocab)**：统计语料中的词频，建立索引映射，包括 `<unk>`、`<pad>`、`<bos>`、`<eos>` 等特殊符号。
3. **数值化**：将分词后的 token 转成数字 ID。训练时用 `<bos>` 和 `<eos>` 标记句子起止，用 `<pad>` 填充批次中较短序列。
4. **DataLoader**：将中英句子打包成 `Dataset`，并按批次填充成相同长度张量（[seq_len, batch_size]）。

为什么需要这些步骤？

1. 从 token 到 ID：
    - 机器学习模型（如 Transformer）无法直接处理文本，需要将 token 转换为数字 ID。
    - tokenizer_zh.encode 和 tokenizer_en.encode 完成了这一转换。
2. 添加 `<bos>` 和 `<eos>`：
    - 在机器翻译任务中，`<bos>`（开始标记）和 `<eos>`（结束标记）告诉模型句子的边界。
    - 例如，`<bos> 我 爱 北京 天安门 <eos>` 明确了句子的起止。
3. Padding：
    - 批处理时，模型要求输入的序列长度一致。
    - pad_sequence 用 `<pad>`（ID=1）补齐短序列，确保批次中的所有序列长度相同。

| 标记      | 全称                          | 通俗含义                            | 作用场景                 |
| ------- | --------------------------- | ------------------------------- | -------------------- |
| UNK_IDX | Unknown Index               | “不知道这个词”的编号，用来表示词汇表里没有的生僻词或新词。| 遇到不在词汇表中的词时，替换成这个标记。|
| PAD_IDX | Padding Index               | “填充物”的编号，用来把短句子补齐到固定长度，就像塞个占位符。| 批处理时让所有句子长度一致。|
| BOS_IDX | Beginning of Sentence Index | “句子开头”的编号，告诉模型这是句子的起点，像个起跑线。| 生成任务中标记句子开始。|
| EOS_IDX | End of Sentence Index       | “句子结尾”的编号，告诉模型句子到此结束，像个终点旗。| 生成任务中标记句子结束。|

下面的代码示例演示构建一个最小的中英 toy 数据集，然后对其进行分词和词表创建。实际项目中，你需要改写成从文件或更大数据集中读取。

```python
import torch
from torch.utils.data import DataLoader, Dataset
import jieba  # 用于中文分词
import spacy  # 用于英文分词
from tokenizers import Tokenizer, models, pre_tokenizers, trainers  # 使用 Hugging Face 的 tokenizers

# 安装依赖 (若在 Colab，需要先执行安装)
# !pip install jieba spacy tokenizers

# 下载并加载 spacy 英文分词模型 (只需执行一次)
# !python -m spacy download en_core_web_sm
en_nlp = spacy.load("en_core_web_sm")

# 定义特殊标记及其索引
UNK_IDX, PAD_IDX, BOS_IDX, EOS_IDX = 0, 1, 2, 3
special_tokens = ["<unk>", "<pad>", "<bos>", "<eos>"]

# 假设我们有这样一个非常小的 toy 数据集 (仅为演示)
toy_data = [
    ("我 爱 北京 天安门", "I love Beijing Tiananmen"),
    ("北京 是 中国 的 首都", "Beijing is the capital of China"),
    ("你 好 世界", "Hello world"),
    ("今天天气 不错", "The weather is nice today"),
]

# 分词函数：中文用 jieba，英文用 spacy
def tokenize_zh(text):
    return list(jieba.cut(text))  # jieba.cut 返回生成器，这里转列表

def tokenize_en(text):
    return [tok.text for tok in en_nlp(text)]

# 准备数据对：分词后的中文、英文
tokenized_pairs = []
for zh_sentence, en_sentence in toy_data:
    zh_tokens = tokenize_zh(zh_sentence)
    en_tokens = tokenize_en(en_sentence)
    tokenized_pairs.append((zh_tokens, en_tokens))

# 使用 tokenizers 构建中文和英文的 tokenizer
def build_tokenizer(tokenized_data, special_tokens, language="zh"):
    # 初始化一个 WordLevel 模型（基于词级别）
    tokenizer = Tokenizer(models.WordLevel(unk_token="<unk>"))

    # 设置预分词器：这里我们已经用 jieba/spacy 分好词了，所以直接用空格分隔
    tokenizer.pre_tokenizer = pre_tokenizers.Whitespace()

    # 创建训练器，添加特殊标记
    trainer = trainers.WordLevelTrainer(
        special_tokens=special_tokens,
        min_frequency=1  # 最低词频设为 1，与原代码一致
    )

    # 准备训练数据：将分词后的 token 列表转为字符串列表（tokenizers 需要字符串输入）
    training_data = [" ".join(tokens) for tokens, _ in tokenized_data] if language == "zh" else [" ".join(tokens) for _, tokens in tokenized_data]

    # 训练 tokenizer
    tokenizer.train_from_iterator(training_data, trainer=trainer)

    return tokenizer

# 构建中文和英文的 tokenizer
tokenizer_zh = build_tokenizer(tokenized_pairs, special_tokens, language="zh")
tokenizer_en = build_tokenizer(tokenized_pairs, special_tokens, language="en")

# 获取词汇表大小
vocab_size_zh = tokenizer_zh.get_vocab_size()
vocab_size_en = tokenizer_en.get_vocab_size()

print("中文词表大小:", vocab_size_zh)
print("英文词表大小:", vocab_size_en)

# 打印词汇表（可选）
print("中文词汇表:", tokenizer_zh.get_vocab())
print("英文词汇表:", tokenizer_en.get_vocab())
```

上面我们仅演示了**toy 示例**，实际可替换为更大规模的中英语料（比如 IWSLT zh-en、WMT zh-en）。得到 `vocab_zh` 和 `vocab_en` 后，我们需要在训练、推理时对文本执行统一的**数值化**流程：分词 → 映射ID → 添加 `<bos>`、`<eos>` → 转为张量。并将较短句子用 `<pad>` 填充以适配批量处理。

下面封装成一个 `MyZhEnDataset`，并在 `__getitem__` 中返回 `(src_tensor, tgt_tensor)`。同时展示 `collate_fn` 如何对批次的张量进行 `pad_sequence` 填充。

```python
import math
import torch.nn as nn
import torch.nn.functional as F
from torch.nn.utils.rnn import pad_sequence

class MyZhEnDataset(Dataset):
    def __init__(self, data_pairs, vocab_zh, vocab_en):
        self.data = data_pairs
        self.vocab_zh = vocab_zh
        self.vocab_en = vocab_en
    
    def __len__(self):
        return len(self.data)
    
    def __getitem__(self, idx):
        zh_tokens, en_tokens = self.data[idx]
        # 转ID并加<BOS>,<EOS>
        zh_ids = [BOS_IDX] + [self.vocab_zh[tok] for tok in zh_tokens] + [EOS_IDX]
        en_ids = [BOS_IDX] + [self.vocab_en[tok] for tok in en_tokens] + [EOS_IDX]
        return torch.tensor(zh_ids, dtype=torch.long), torch.tensor(en_ids, dtype=torch.long)

def collate_fn(batch):
    # batch: List[(zh_ids, en_ids), (zh_ids, en_ids), ...]
    src_batch = []
    tgt_batch = []
    for (src_ids, tgt_ids) in batch:
        src_batch.append(src_ids)
        tgt_batch.append(tgt_ids)
    # pad_sequence的结果形状 [max_len, batch_size]
    src_batch = pad_sequence(src_batch, padding_value=PAD_IDX)
    tgt_batch = pad_sequence(tgt_batch, padding_value=PAD_IDX)
    return src_batch, tgt_batch

# 构建Dataset和DataLoader
dataset = MyZhEnDataset(tokenized_pairs, vocab_zh, vocab_en)
dataloader = DataLoader(dataset, batch_size=2, shuffle=True, collate_fn=collate_fn)
for src_sample, tgt_sample in dataloader:
    print("src shape:", src_sample.shape, "tgt shape:", tgt_sample.shape)
    break
```

这一步完成后，我们就能获得形状为 `[seq_len, batch_size]` 的源、目标张量，供后续 Transformer 模型使用。请自行扩充到更大规模的中英语料，以获得更实际的翻译效果。**第1步完成**，我们成功构建了中英数据处理与加载管线。

---

## 第2步：位置编码 (Positional Encoding) 与嵌入层

这部分与之前的英德示例完全相同，原理是：Transformer 不使用 RNN，所以需要**位置编码**来让模型获知序列位置。我们依旧使用正弦余弦函数形式。词嵌入层直接用 `nn.Embedding`。

请参考以下实现并简单测试。代码与之前一致，只需记得词表大小换成我们当前的 `vocab_zh` 和 `vocab_en` 大小即可：

```python
import math
import torch.nn as nn

class PositionalEncoding(nn.Module):
    def __init__(self, d_model: int, dropout: float = 0.1, max_len: int = 5000):
        super().__init__()
        self.dropout = nn.Dropout(dropout)
        pe = torch.zeros(max_len, d_model)
        position = torch.arange(0, max_len).unsqueeze(1)
        div_term = torch.exp(torch.arange(0, d_model, 2) * -(math.log(10000.0) / d_model))
        pe[:, 0::2] = torch.sin(position * div_term)
        pe[:, 1::2] = torch.cos(position * div_term)
        pe = pe.unsqueeze(1)  # [max_len, 1, d_model]
        self.register_buffer('pe', pe)
    
    def forward(self, x: torch.Tensor) -> torch.Tensor:
        seq_len = x.size(0)
        x = x + self.pe[:seq_len]
        return self.dropout(x)

class TokenEmbedding(nn.Module):
    def __init__(self, vocab_size: int, emb_size: int):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, emb_size)
        self.emb_size = emb_size
    
    def forward(self, tokens: torch.Tensor) -> torch.Tensor:
        return self.embedding(tokens) * math.sqrt(self.emb_size)

# 测试
d_model = 8
pos_enc = PositionalEncoding(d_model, dropout=0.0, max_len=20)
emb = TokenEmbedding(10, d_model)
x = torch.zeros(5, 1, d_model)
y = pos_enc(x)
assert y.shape == (5, 1, d_model)
print("PositionalEncoding test passed. shape:", y.shape)
```

我们确认位置编码效果无误后，就可以在后续 Transformer 中使用。**第2步完成**。

---

## 第3步：构建编码器模块 (EncoderLayer)

Transformer 编码器层由**多头自注意力**与**前馈网络**组成，并带有残差连接和 LayerNorm。与前面示例基本一致，只是变量命名从德语→中文不影响实现。核心依旧是 `nn.MultiheadAttention` 和两层线性层。可复用之前的代码。

```python
class EncoderLayer(nn.Module):
    def __init__(self, d_model: int, nhead: int, dim_feedforward: int = 2048, dropout: float = 0.1):
        super().__init__()
        self.self_attn = nn.MultiheadAttention(d_model, nhead, dropout=dropout)
        self.linear1 = nn.Linear(d_model, dim_feedforward)
        self.dropout = nn.Dropout(dropout)
        self.linear2 = nn.Linear(dim_feedforward, d_model)
        self.norm1 = nn.LayerNorm(d_model)
        self.norm2 = nn.LayerNorm(d_model)
        self.dropout1 = nn.Dropout(dropout)
        self.dropout2 = nn.Dropout(dropout)
    
    def forward(self, src, src_mask=None, src_key_padding_mask=None):
        attn_output, _ = self.self_attn(src, src, src, attn_mask=src_mask,
                                        key_padding_mask=src_key_padding_mask)
        src = self.norm1(src + self.dropout1(attn_output))
        ff_output = self.linear2(self.dropout(torch.relu(self.linear1(src))))
        src = self.norm2(src + self.dropout2(ff_output))
        return src

# 测试
layer = EncoderLayer(d_model=8, nhead=2)
dummy_src = torch.randn(5, 2, 8)  # seq_len=5, batch=2
out = layer(dummy_src)
print("EncoderLayer output shape:", out.shape)
assert out.shape == dummy_src.shape
```

**第3步完成**，编码器层已准备就绪。

---

## 第4步：构建解码器模块 (DecoderLayer)

解码器层除了自注意力，还有**交叉注意力**(encoder-decoder attention)，以融合编码器输出的 memory 信息。在实现中新增一个 `multihead_attn` 和相应的残差 + LayerNorm。并且第一步自注意力需传入**序列掩码**（下三角）以禁止访问后面词。

```python
class DecoderLayer(nn.Module):
    def __init__(self, d_model: int, nhead: int, dim_feedforward: int = 2048, dropout: float = 0.1):
        super().__init__()
        self.self_attn = nn.MultiheadAttention(d_model, nhead, dropout=dropout)
        self.cross_attn = nn.MultiheadAttention(d_model, nhead, dropout=dropout)
        self.linear1 = nn.Linear(d_model, dim_feedforward)
        self.dropout = nn.Dropout(dropout)
        self.linear2 = nn.Linear(dim_feedforward, d_model)

        self.norm1 = nn.LayerNorm(d_model)
        self.norm2 = nn.LayerNorm(d_model)
        self.norm3 = nn.LayerNorm(d_model)

        self.dropout1 = nn.Dropout(dropout)
        self.dropout2 = nn.Dropout(dropout)
        self.dropout3 = nn.Dropout(dropout)
    
    def forward(self, tgt, memory, tgt_mask=None, memory_mask=None,
                tgt_key_padding_mask=None, memory_key_padding_mask=None):
        # 1) 自注意力 (masked)
        attn_output, _ = self.self_attn(tgt, tgt, tgt,
                                        attn_mask=tgt_mask,
                                        key_padding_mask=tgt_key_padding_mask)
        tgt = self.norm1(tgt + self.dropout1(attn_output))
        # 2) 交叉注意力
        attn_output2, _ = self.cross_attn(tgt, memory, memory,
                                          attn_mask=memory_mask,
                                          key_padding_mask=memory_key_padding_mask)
        tgt = self.norm2(tgt + self.dropout2(attn_output2))
        # 3) 前馈网络
        ff_output = self.linear2(self.dropout(torch.relu(self.linear1(tgt))))
        tgt = self.norm3(tgt + self.dropout3(ff_output))
        return tgt

# 测试
decoder_layer = DecoderLayer(d_model=8, nhead=2)
dummy_tgt = torch.randn(6, 2, 8)
dummy_mem = torch.randn(5, 2, 8)
out = decoder_layer(dummy_tgt, dummy_mem)
assert out.shape == dummy_tgt.shape
```

**第4步完成**，解码器层就绪。

---

## 第5步：组装完整 Transformer 模型

有了编码器、解码器单层后，我们堆叠多层形成完整模型，并在最前面加**源/目标词嵌入 + 位置编码**，最后一层加**线性输出**映射到英文词表大小。与之前的英德实例相同，只是语言改为中英：

```python
class TransformerModel(nn.Module):
    def __init__(self,
                 src_vocab_size: int,
                 tgt_vocab_size: int,
                 d_model: int=512,
                 nhead: int=8,
                 num_encoder_layers: int=6,
                 num_decoder_layers: int=6,
                 dim_feedforward: int=2048,
                 dropout: float=0.1):
        super().__init__()
        # 嵌入层
        self.src_tok_emb = TokenEmbedding(src_vocab_size, d_model)
        self.tgt_tok_emb = TokenEmbedding(tgt_vocab_size, d_model)
        self.pos_encoding = PositionalEncoding(d_model, dropout=dropout)
        # 编码器堆叠
        self.encoder_layers = nn.ModuleList([
            EncoderLayer(d_model, nhead, dim_feedforward, dropout)
            for _ in range(num_encoder_layers)
        ])
        # 解码器堆叠
        self.decoder_layers = nn.ModuleList([
            DecoderLayer(d_model, nhead, dim_feedforward, dropout)
            for _ in range(num_decoder_layers)
        ])
        # 输出映射
        self.generator = nn.Linear(d_model, tgt_vocab_size)
    
    def forward(self,
                src,
                tgt,
                src_mask=None,
                tgt_mask=None,
                src_key_padding_mask=None,
                tgt_key_padding_mask=None,
                memory_key_padding_mask=None):
        # [seq_len, batch] -> [seq_len, batch, d_model]
        src_emb = self.pos_encoding(self.src_tok_emb(src))
        tgt_emb = self.pos_encoding(self.tgt_tok_emb(tgt))

        # 编码器 (multi-layer)
        memory = src_emb
        for layer in self.encoder_layers:
            memory = layer(memory, src_mask=src_mask, src_key_padding_mask=src_key_padding_mask)
        
        # 解码器 (multi-layer)
        output = tgt_emb
        for layer in self.decoder_layers:
            output = layer(output, memory, tgt_mask=tgt_mask,
                           tgt_key_padding_mask=tgt_key_padding_mask,
                           memory_key_padding_mask=memory_key_padding_mask)
        
        # 映射到词表分布
        logits = self.generator(output)
        return logits

# 简单测试
test_model = TransformerModel(src_vocab_size=len(vocab_zh),
                              tgt_vocab_size=len(vocab_en),
                              d_model=32, nhead=4,
                              num_encoder_layers=2, num_decoder_layers=2,
                              dim_feedforward=64)
dummy_src = torch.randint(0, len(vocab_zh), (5, 2))  # [seq_len=5, batch=2]
dummy_tgt = torch.randint(0, len(vocab_en), (6, 2))
out = test_model(dummy_src, dummy_tgt)
print("Transformer output shape:", out.shape)  # [tgt_len, batch, tgt_vocab_size]
assert out.shape[0] == dummy_tgt.shape[0] and out.shape[1] == dummy_tgt.shape[1]
assert out.shape[2] == len(vocab_en)
```

**第5步完成**，我们的中英 Transformer 结构已经搭建好。

---

## 第6步：损失函数与优化器（标签平滑）

对于翻译任务，通常使用**交叉熵损失**并忽略 `<pad>` 位置。PyTorch 的 `nn.CrossEntropyLoss` 中可以通过 `ignore_index=PAD_IDX` 达成这一效果。还可加 `label_smoothing=0.1` 做标签平滑，以提高泛化。

同时我们用 Adam 优化器，学习率可设为 `1e-4` 或按原论文策略做 warmup。此处演示简单写法：

```python
import torch.optim as optim

pad_idx = PAD_IDX
criterion = nn.CrossEntropyLoss(ignore_index=pad_idx, label_smoothing=0.1)
optimizer = optim.Adam(test_model.parameters(), lr=1e-4, betas=(0.9,0.98), eps=1e-9)
```

**第6步完成**。

---

## 第7步：训练与验证循环 (Teacher Forcing)

跟之前类似，我们编写 `train_epoch` 和 `evaluate` 函数，对每个批次做前向、反向、更新。为了让解码器无法“偷看”后面词，需要构造**下三角掩码** `tgt_mask`；为了忽略填充位置，需要 `src_key_padding_mask`、`tgt_key_padding_mask` 以及 `memory_key_padding_mask`。主要区别在于我们这里是**中文→英文**。

```python
def generate_square_subsequent_mask(sz):
    # 下三角为False, 上三角为True
    mask = torch.triu(torch.ones(sz, sz) == 1, diagonal=1)
    return mask

def create_mask(src, tgt_in):
    src_seq_len = src.size(0)
    tgt_seq_len = tgt_in.size(0)
    # 源端不需要未来掩码
    src_mask = torch.zeros((src_seq_len, src_seq_len), dtype=torch.bool)
    # 目标端下三角掩码
    tgt_mask = generate_square_subsequent_mask(tgt_seq_len)
    # 填充mask: True表示是<PAD>
    src_key_padding_mask = (src == pad_idx).transpose(0,1)
    tgt_key_padding_mask = (tgt_in == pad_idx).transpose(0,1)
    return src_mask, tgt_mask, src_key_padding_mask, tgt_key_padding_mask

def train_epoch(model, optimizer, dataloader, device):
    model.train()
    total_loss = 0
    for src, tgt in dataloader:
        src = src.to(device)
        tgt = tgt.to(device)

        # teacher forcing: 输入序列是 tgt[:-1], 需要预测 tgt[1:]
        tgt_input = tgt[:-1, :]
        tgt_out   = tgt[1:, :]

        src_mask, tgt_mask, src_kpm, tgt_kpm = create_mask(src, tgt_input)

        optimizer.zero_grad()
        logits = model(src, tgt_input,
                       src_mask=src_mask, tgt_mask=tgt_mask,
                       src_key_padding_mask=src_kpm, tgt_key_padding_mask=tgt_kpm,
                       memory_key_padding_mask=src_kpm)
        
        loss = criterion(logits.reshape(-1, logits.size(-1)), tgt_out.reshape(-1))
        loss.backward()
        optimizer.step()

        total_loss += loss.item()
    return total_loss / len(dataloader)

def evaluate(model, dataloader, device):
    model.eval()
    total_loss = 0
    with torch.no_grad():
        for src, tgt in dataloader:
            src, tgt = src.to(device), tgt.to(device)
            tgt_input = tgt[:-1, :]
            tgt_out   = tgt[1:, :]
            src_mask, tgt_mask, src_kpm, tgt_kpm = create_mask(src, tgt_input)
            logits = model(src, tgt_input,
                           src_mask=src_mask, tgt_mask=tgt_mask,
                           src_key_padding_mask=src_kpm, tgt_key_padding_mask=tgt_kpm,
                           memory_key_padding_mask=src_kpm)
            loss = criterion(logits.reshape(-1, logits.size(-1)), tgt_out.reshape(-1))
            total_loss += loss.item()
    return total_loss / len(dataloader)
```

然后我们就可以**训练若干 epoch**，观察损失下降情况。由于我们这里只是 toy 数据集，训练不会有太大意义，实际请用上千、上万句中英平行句子。示例：

```python
device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
model = test_model.to(device)

EPOCHS = 10
for epoch in range(1, EPOCHS+1):
    train_loss = train_epoch(model, optimizer, dataloader, device)
    val_loss = evaluate(model, dataloader, device)  # 暂无单独验证集就用同数据
    print(f"Epoch {epoch}, Train Loss: {train_loss:.4f}, Val Loss: {val_loss:.4f}")
```

**第7步完成**。当我们看到训练损失持续下降，即表明模型渐渐学习到从中文序列映射到英文序列的规律。

---

## 第8步：推理与翻译生成 (Greedy Decoding)

训练完后，需要用**自回归**方法来生成译文：从 `<bos>` 开始，每次解码一个词，把这个预测词拼到序列末尾，再次输入解码器，直到生成 `<eos>` 或超长。贪心解码就是每一步都选概率最高的词，效率较高，但有时不如束搜索效果好。

示例函数：

```python
def greedy_decode(model, src, max_len, start_symbol=BOS_IDX):
    model.eval()
    src = src.to(device)
    src_mask = torch.zeros((src.size(0), src.size(0)), dtype=torch.bool, device=device)
    # 编码器输出
    with torch.no_grad():
        memory = src
        for layer in model.encoder_layers:
            memory = layer(memory, src_mask=src_mask)
    
    ys = torch.tensor([[start_symbol]], dtype=torch.long, device=device)  # <bos>
    for i in range(max_len-1):
        tgt_mask = generate_square_subsequent_mask(ys.size(0)).to(device)
        out = ys
        for layer in model.decoder_layers:
            out = layer(out, memory, tgt_mask=tgt_mask)
        out = model.generator(out)  # [seq_len, batch, vocab_en]
        next_token = out[-1, 0].argmax(-1).item()
        ys = torch.cat([ys, torch.tensor([[next_token]], device=device)], dim=0)
        if next_token == EOS_IDX:
            break
    return ys

def translate(model, src_sentence):
    model.eval()
    # 分词 -> ID -> 加<BOS>,<EOS>
    zh_tokens = tokenize_zh(src_sentence)
    zh_ids = [BOS_IDX] + [vocab_zh[tok] for tok in zh_tokens] + [EOS_IDX]
    src_tensor = torch.tensor(zh_ids, dtype=torch.long).unsqueeze(1)  # [seq_len, 1]
    tgt_indices = greedy_decode(model, src_tensor, max_len=50)
    tgt_indices = tgt_indices.flatten().cpu().numpy()
    # 转回字符串，去掉<BOS>, <EOS>
    en_tokens = [vocab_en.lookup_token(idx) for idx in tgt_indices]
    # 过滤特殊符号
    en_tokens = [t for t in en_tokens if t not in ["<bos>", "<eos>", "<pad>"]]
    return " ".join(en_tokens)
```

现在我们可以尝试翻译。例如，训练过的模型输入“我 爱 北京”，看看输出：

```python
test_sentence = "我 爱 北京"
print("SRC (ZH):", test_sentence)
print("PRED (EN):", translate(model, test_sentence))
```

如果数据和训练足够，可能输出 “I love Beijing” 等（toy 数据可能也能拼对几个词）。真实大规模训练后，这里可以产生更完整的英语句子。**第8步完成**。

---

## 第9步：模型评估 (BLEU)

对机器翻译最常用的自动评价指标是 **BLEU**。我们可以使用第三方工具包，如 `sacrebleu` 或 `nltk.translate.bleu_score`。示例用 `sacrebleu`：

```python
!pip install sacrebleu

import sacrebleu

def compute_bleu_score(model, dataset):
    model.eval()
    refs = []
    hyps = []
    for i in range(len(dataset)):
        zh_ids, en_ids = dataset[i]
        # 还原英文参考
        en_tokens = [vocab_en.lookup_token(idx) for idx in en_ids.tolist()]
        en_tokens = [t for t in en_tokens if t not in ["<bos>", "<eos>", "<pad>"]]
        refs.append(" ".join(en_tokens))

        # 源句翻译
        zh_tokens = [vocab_zh.lookup_token(idx) for idx in zh_ids.tolist()
                     if idx not in [BOS_IDX, EOS_IDX, PAD_IDX]]
        zh_sent = "".join(zh_tokens)  # 或者带空格 ' '.join(zh_tokens), 取决于训练方式
        pred = translate(model, zh_sent)
        hyps.append(pred)

    bleu = sacrebleu.corpus_bleu(hyps, [refs])
    return bleu.score

bleu_score = compute_bleu_score(model, dataset)
print(f"BLEU score on toy dataset = {bleu_score:.2f}")
```

对于我们极小的 toy 数据，BLEU 可能无意义或易过拟合到 100。真实任务可以在千句以上的测试集中得到更可靠分数。**第9步完成**，我们学会用 BLEU 评估中英翻译。

---

## 第10步：模型保存与加载

与 PyTorch 通用流程相同，将模型状态字典 `state_dict()` 保存即可。下次加载时需构造相同结构的模型实例，然后 `load_state_dict()` 并 `eval()`。

```python
torch.save(model.state_dict(), "transformer_zh2en.pth")
print("Model saved.")

model_loaded = TransformerModel(len(vocab_zh), len(vocab_en),
                                d_model=32, nhead=4, num_encoder_layers=2, num_decoder_layers=2,
                                dim_feedforward=64)
model_loaded.load_state_dict(torch.load("transformer_zh2en.pth"))
model_loaded = model_loaded.to(device)
model_loaded.eval()
print("Model loaded and ready.")

# 测试一下
test_sentence = "北京 是 中国 的 首都"
print("SRC (ZH):", test_sentence)
print("Loaded model translation:", translate(model_loaded, test_sentence))
```

**第10步完成**，实现了模型的保存和后续推断加载。

---

## 总结

恭喜你！通过以上**10 大步骤**，你已从头搭建了一个**中文→英文 Transformer** 模型，并完成了数据处理、模型定义、训练、推理与评估全流程。该教程围绕**Exercism 风格**：每一步有理论介绍、代码实现和验证，有助于加深对 Transformer 架构和机器翻译实际操作的理解。

在**真实项目**中，你可以：

- 使用**更大规模**的中英平行数据（如 WMT、IWSLT），并做更完善的**中英文分词**（如字级切分、BPE/字节对编码、或使用 Hugging Face `Tokenizer`）。
    
- 增大模型超参（`d_model=512`、`num_layers=6`）或实现**学习率调度**(warm-up) 等，以得到更好的翻译效果。
    
- 使用**束搜索 (beam search)** 替代贪心解码，提升翻译质量。
    
- 在评估时配置**更严格**的测试集并查看 BLEU、TER、ROUGE 等多种指标。
    
- 加入如**多 GPU 训练**、**Mixed Precision** 训练等进一步加速大模型训练。
    

希望这个教程能够为你搭建**中英翻译的 Transformer** 提供清晰的思路和实践指引。祝你在深度学习与自然语言处理的道路上不断进步、取得好成绩！
