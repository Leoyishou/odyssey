---
date created: 2025-04-05
date modified: 2025-07-10
uid: 0a06e6df-eaec-4003-9156-84afe6eacfb9
---

注意力提供“信息融合”，前馈网络提供“非线性转换”，各自有残差路径便于梯度传播。

___

|               |                                                                                    |
| ------------- | ---------------------------------------------------------------------------------- |
| **周数 (Week)** | **目标 (Goal)**                                                                      |
| 第一周           | 搭建环境，了解 Transformer 架构和自注意力机制，完成第一个文本生成示例。|
| 第二周           | 理解 Transformer 的核心组成，包括词嵌入、位置编码、多头注意力和前馈网络，并逐步实现这些组件。|
| 第三周           | 将前述组件组装实现一个简化版 Transformer 块，并通过微调预训练模型完成一个文本分类任务，以体会 Transformer 在下游任务中的应用。|
| 第四周           | 学习 Transformer 在更高级应用中的使用，包括文本生成、模型压缩、提示工程和评估指标。|
| 第五周           | 探索训练和部署大型 Transformer 模型的高级技巧，包括高效训练策略、模型伦理、公平性、多模态模型，以及模型的优化部署方法。|
| 第六周           | 了解 Transformer 领域最新的研究方向和应用：包括超大模型的稀疏化优化、强化学习对齐技术（RLHF），以及近期流行的自主 AI 代理（Agent）的原理。|

## 六周 Transformer 模型实践教程

本教程将带您在六周内逐步实践深度学习中的 Transformer 模型，每一步包含理论讲解、代码示例和结果验证。所有代码使用 Python 和 PyTorch，并结合 Hugging Face Transformers 库。请使用 Google Colab 运行以下内容。教程按周划分，每周包含若干步练习（每步约 15–30 分钟）。让我们开始吧！

### ❤️ 第一周：基本概念 (4小时)

目标：搭建环境，了解 Transformer 架构和自注意力机制，完成第一个文本生成示例。

#### 步骤1：环境搭建（Python/PyTorch/Hugging Face Transformers）

理论：首先，我们需要准备好深度学习的编程环境。本教程使用 PyTorch 深度学习框架和 Hugging Face Transformers 库。您可以在 Google Colab 或本地安装所需的软件包。Colab 通常预装了 `torch`，但我们仍确保安装最新版本。Hugging Face Transformers 提供了大量预训练模型和便捷接口。

编程任务：安装并验证 PyTorch 和 Transformers 是否成功工作。我们将打印出版本号，并测试 GPU 是否可用（如有GPU）。

```python
# 在Colab中安装PyTorch和Transformers
!pip install torch torchvision transformers

import torch
import transformers

print("PyTorch 版本:", torch.__version__)
print("Transformers 版本:", transformers.__version__)
print("是否可用GPU:", torch.cuda.is_available())
```

结果验证：运行上面的代码应输出 PyTorch 和 Transformers 的版本号，并指示 GPU 可用性（如果在Colab使用GPU运行时，应显示`True`）。如果没有错误，即表示环境搭建成功。版本号只是示例，实际输出可能类似于：

```Java
PyTorch 版本: 2.0.0+cu118  
Transformers 版本: 4.29.2  
是否可用GPU: True
```

#### 步骤2：Transformer 架构简介

理论：Transformer 是由 Vaswani 等人在 2017 年提出的神经网络架构。它使用了自注意力机制替代传统循环网络，实现了更高的并行度和效果。Transformer 包含 编码器（encoder）和 解码器（decoder）两个部分，每一侧由多个层堆叠而成。编码器层主要包括多头自注意力和前馈网络模块，解码器层则在自注意力之外增加了与编码器交互的交叉注意力模块。由于不使用循环结构，Transformer 可以并行处理序列中的所有位置，从而大大加快训练速度。

在编码器中，每个输入词（通过嵌入获得向量表示）与序列中的其他词通过自注意力进行关联，之后经过前馈网络进行非线性变换。解码器则在生成输出时，先通过遮罩（Masked）自注意力关注已生成的部分，再通过交叉注意力结合编码器输出，以确保输出句子与输入相关。Transformer 架构的并行计算、高效注意力机制，使其成为现代大模型（LLM）的基础。

编程任务：使用预训练的 Transformer 模型来体验其基本用法。我们以 DistilBERT（一个小型的BERT模型）为例，将一句话输入模型，获取它的隐藏表示。这展示了Transformer编码文本的能力。

```python
from transformers import AutoTokenizer, AutoModel

# 加载预训练的DistilBERT模型和对应的分词器
tokenizer = AutoTokenizer.from_pretrained('distilbert-base-uncased')
model = AutoModel.from_pretrained('distilbert-base-uncased')  # AutoModel默认返回基底模型

# 将文本转换为模型输入张量
text = "Hello world"
inputs = tokenizer(text, return_tensors="pt")
print("输入张量维度:", inputs['input_ids'].shape)  # (批次大小, 序列长度)

# 前向传播，获取模型最后一层隐藏状态
outputs = model(inputs)
last_hidden_state = outputs.last_hidden_state
print("输出张量维度:", last_hidden_state.shape)
```

结果验证：DistilBERT 模型将输入 `"Hello world"` 编码为隐藏向量。输出的 `last_hidden_state` 形状应为 (batch_size, sequence_length,。对于单句输入，

- `batch_size=1`，
- `sequence_length`包含输入词数加特殊标记（比如 `"Hello world"` 经过分词可能得到3个标记，包括`[CLS]`等），
- `hidden_size=768`（DistilBERT的默认隐藏维度）。例如，可能输出

```Java
输入张量维度: torch.Size([1, ([Aligning language models to follow instructions | OpenAI]
输出张量维度: [1, 3, 768]
```

这表示模型已成功将3个标记映射为3个长度为768的向量。通过这种方式，我们可以获得Transformer编码的文本表示，为下游任务做准备。您可以尝试打印 `last_hidden_state[0][0][:5]` 等看看隐藏状态张量的部分值（一般都是非零的浮点数）。

#### 步骤3：自注意力机制介绍

理论：自注意力的核心机制。它使每个词能够根据其他词的重要性来更新自身表示。具体来说，对于序列中的每个位置，模型学习三个向量：查询（Query）、键（Key）和值（Value）。注意力的计算分两步：首先计算查询与所有键的相似度（通常用点积表示），然后对值加权求和。通过softmax函数，这些注意力权重会归一化为概率，表示当前词对其他各词的“关注”程度。自注意力可以在O(n²)时间内建立序列中任意两词之间的依赖关系，同时方便并行计算，这是Transformer高效建模长序列的关键。

简单来说，自注意力输出是其他词对当前词的加权影响。Transformer编码器使用自注意力输入自身序列（因此称“自”注意力），而解码器还有类似机制用于关注编码器输出（交叉注意力）。

编程任务：实现缩放点积注意力（Scaled Dot-Product Attention），它是自注意力的基础。公式如下：

$$\mathrm{Attention}(Q, K, V) = \mathrm{softmax}\left(\frac{Q K^T}{\sqrt{d_k}}\right) V$$

其中 $Q$ 是查询矩阵，$K$ 是键矩阵，$V$ 是值矩阵，$d_k$ 是键向量维度的缩放因子。我们将使用小矩阵演示注意力计算，并验证注意力权重的性质。

```python
import torch
import math

# 定义一个简化的注意力函数
def scaled_dot_product_attention(Q, K, V):
    # Q, K, V 形状: (batch, seq_len, d_k)
    d_k = Q.size(-1)
    # 计算注意力得分: QK^T / sqrt(d_k)
    scores = torch.matmul(Q, K.transpose(-2, -1)) / math.sqrt(d_k)
    # 对最后一个维度应用softmax得到注意力权重
    weights = torch.softmax(scores, dim=-1)  # 形状: (batch, seq_len, seq_len)
    # 加权求和值
    output = torch.matmul(weights, V)  # 形状: (batch, seq_len, d_v), 通常 d_v = d_k
    return output, weights

# 准备模拟输入 (batch=1, seq_len=3, d_k=d_v=4)
Q = torch.tensor([[[1.0, 0.0, 1.0, 0.0],
                   [0.0, 1.0, 0.0, 1.0],
                   [1.0, 1.0, 1.0, 1.0]]])  # 查询向量
K = Q.clone()  # 为简单起见，这里令 K = Q
V = torch.tensor([[[ 1.0,  2.0,  3.0,  4.0],
                   [10.0, 20.0, 30.0, 40.0],
                   [50.0, 60.0, 70.0, 80.0]]])  # 值向量

output, attn_weights = scaled_dot_product_attention(Q, K, V)
print("注意力权重矩阵:\n", attn_weights)
print("注意力输出:\n", output)
```

结果验证：对于上述设置，我们特别选择了简单的 $Q$=$K$ 情况以方便理解。注意力权重

他词（包括自身）的注意力分配概率。由于我们使用softmax，对每一行而言，权重和应为1：

```python
# 验证每个查询位置的注意力权重和为1
assert torch.allclose(attn_weights.sum(dim=-1), torch.tensor([[1.0, 1.0, 1.0]])), "每个位置的注意力权重之和应为1"
```

打印的注意力权重矩阵可能类似：

```Java
注意力权重矩阵:
 tensor([[[0.3199, 0.3199, 0.3602],
          [0.3199, 0.3199, 0.3602],
          [0.3199, 0.3199, 0.3602]]])
```

（这是因为我们示例中三行的 $Q$ 向量恰好都相同，从而产生相同的权重分布；实际情况下不同查询会有不同权重。）

输出 `output` 是注意力加权后的值。例如：

```Java
注意力输出:
 tensor([[[19.0602, 22.0602, 25.0602, 28.0602],
          [19.0602, 22.0602, 25.0602, 28.0602],
          [19.0602, 22.0602, 25.0602, 28.0602]]])
```

可以看到三个输出行也相同。这一示例表明，当查询相同时，注意力产生的输出也会相同，因为它们以相同方式“看待”值。您可以尝试修改 Q 或 V 来观察注意力输出的变化。总之，自注意力机制让每个词根据与其他词的相关性灵活聚合信息，是 Transformer 成功的秘诀之一。

#### 步骤4：基础文本生成（“Hello World”示例）

理论：既然了解了 Transformer 的基本结构，我们来体验一下 文本生成。Transformer 常用于自然语言生成任务，如机器翻译、对话和写作等。生成式模型（如GPT系列）使用Transformer的解码器架构：通过逐步预测下一个词来生成文本。我们可以利用预训练的语言模型，给它一个提示（prompt），让它续写文本。这是使用Transformer模型的最快“见效”的方法，相当于深度学习的“Hello World”。

编程任务：使用 Hugging Face 提供的 pipeline 高层接口调用一个预训练的 GPT-2 模型，输入一个短语并生成后续文本。然后验证生成结果包含我们的提示。

```python
from transformers import pipeline

# 加载文本生成pipeline，指定使用小型GPT-2模型
generator = pipeline('text-generation', model='distilgpt2')  # distilgpt2是GPT-2的小型蒸馏版本
prompt = "Hello, my name is"
result = generator(prompt, max_length=30, num_return_sequences=1)
output_text = result[0]['generated_text']
print("模型生成结果:\n", output_text)

# 验证生成的文本是以提示开头
assert output_text.startswith(prompt), "输出应该以提示语开头"
```

结果验证：执行上述代码，模型将基于提示 `"Hello, my name is"` 生成接下来的一段文字。由于语言模型是随机采样词汇生成，因此每次运行结果可能不同，但都应以提示开头并继续一段合理的英语句子。例如，可能输出：

```Java
模型生成结果:
 Hello, my name is John and I have been working as a software developer for the past 5 years...
```

我们通过一个断言确保 `output_text` 是以提示字符串开头的。如果模型成功续写，断言会通过。如果断言失败，说明输出不包含提示，这是不符合预期的（一般不会发生，因为pipeline生成的文本默认包含原始prompt）。

恭喜您！您已使用预训练的Transformer模型生成了第一段文本。这只是个起步，“Hello World”例子展示了Transformer强大的生成能力。接下来几周，我们将深入实现Transformer的各个组件，并应用到更复杂的任务中。

---

### ❤️ 第二周：核心组件 (5小时)

目标：理解 Transformer 的核心组成，包括词嵌入、位置编码、多头注意力和前馈网络，并逐步实现这些组件。

#### 步骤5：Word Embedding（词嵌入）

理论：在将文本输入Transformer前，需要将单词或子词转换为向量表示，这就是词嵌入（Word Embedding）。最简单的想法是使用“one-hot”表示（每个词一个高维独热向量），但这样维度极高且无法表达词语含义间的关系。训练可学习的低维稠密向量能克服这些缺点。嵌入层通常是一个查找表，它为每个词索引返回一个固定长度的向量。在PyTorch中，`nn.Embedding` 实现了这一层。

编程任务：使用 `nn.Embedding` 来创建一个词嵌入矩阵。我们将人为设置一些权重来验证嵌入查询是否正确。例如创建一个包含3个单词、每个词embedding维度为2的嵌入层，然后手动赋予已知的向量表示，检查索引查找是否返回对应向量。

```python
import torch.nn as nn

# 创建一个具有3个单词、每个嵌入维度为2的嵌入层
embedding = nn.Embedding(num_embeddings=3, embedding_dim=2)

# 手动设置嵌入向量矩阵的权重（3x2矩阵）
# 假设索引0->[0.1,0.2], 1->[0.3,0.4], 2->[0.5,0.6]
embedding.weight.data = torch.tensor([[0.1, 0.2],
                                     [0.3, 0.4],
                                     [0.5, 0.6]])

# 查询索引2的词嵌入
idx = torch.tensor([2])  # 单词索引2
vec = embedding(idx)     # 获取嵌入向量
print(f"索引{idx.item()}的嵌入向量:", vec)

# 验证返回的向量是否与预设的权重一致
expected = torch.tensor([[0.5, 0.6]])
assert torch.allclose(vec, expected), "嵌入查询结果不匹配预期值"
print("嵌入查询正确!")
```

结果验证：以上代码构造了一个简单的嵌入层并测试索引查找。预期第2号索引返回向量`[0.5, 0.6]`。验证断言通过将输出与预期值比较，应无报错，并打印确认消息：

```Java
索引2的嵌入向量: tensor([[0.5000, 0.6000]])
嵌入查询正确!
```

这证明 `nn.Embedding` 能够正确地将索引映射到对应该索引的向量。在实际Transformer中，词嵌入是在训练过程中学习得到的，可以将词语的语义以向量形式捕捉，比如相似词的向量距离较近等。对于本教程后续任务，我们通常直接使用预训练模型的嵌入或让模型自行学习嵌入层。

#### 步骤6：Positional Encoding（[[位置编码]]）

理论：Transformer没有内置处理序列顺序的循环结构，因此需要显式注入位置信息。位置编码（Positional Encoding）是为每个位置生成的固定向量，将其加到词嵌入上，使模型能区分“第1个词”和“第10个词”。原始Transformer论文使用正弦余弦位置编码：位置编码向量的各维度按不同频率的正弦/余弦函数计算：

- $PE_{(pos, 2i)} = \sin!\Big(pos / 10000^{2i/d_{\text{model}}}\Big)$
    
- $PE_{(pos, 2i+1)} = \cos!\Big(pos / 10000^{2i/d_{\text{model}}}\Big)$
    

这样得到和embedding维度相同的向量，频率较低的维度对大位置变化敏感，频率高的对小位置变化敏感，从而编码绝对位置。更简单的理解：位置0的编码有特殊模式（sin(0)=0, cos(0)=1等），随着位置增大，各维的sinusoids产生独特的编码。

编程任务：实现正弦余弦位置编码的计算函数。生成一定长度`max_len`和给定维度`d_model`的编码矩阵，并验证位置0的编码是否符合公式预期（偶数维为0，奇数维为1）。

```python
import math

def positional_encoding(max_len, d_model):
    pe = torch.zeros(max_len, d_model)
    # 每个维度对应不同频率
    for pos in range(max_len):
        for i in range(0, d_model, 2):
            theta = pos / (10000  ((2  i) / d_model))
            pe[pos, i] = math.sin(theta)
            if i + 1 < d_model:
                pe[pos, i+1] = math.cos(theta)
    return pe

# 生成长度10，维度6的位置信息
PE_matrix = positional_encoding(max_len=10, d_model=6)
print("位置0的编码向量:", PE_matrix[0])
print("位置1的编码向量:", PE_matrix[1])

# 验证位置0的编码：偶数维应该为0，奇数维应该为1
assert torch.allclose(PE_matrix[0, 0::2], torch.zeros(PE_matrix[0, 0::2].shape)), "pos0 偶数维不为0"
assert torch.allclose(PE_matrix[0, 1::2], torch.ones(PE_matrix[0, 1::2].shape)), "pos0 奇数维不为1"
print("位置0编码验证通过!")
```

结果验证：对于位置0，公式中 $\sin(0)=0$，$\cos(0)=1$，因此编码向量应在偶数索引上为0，奇数索引上为1。断言确保了这一点。例如，打印输出可能为：

```Java
位置0的编码向量: tensor([0., 1., 0., 1., 0., 1.])
位置1的编码向量: tensor([ 0.8415,  0.5403,  0.9093,  0.4161,  0.1411,  0.9900])
位置0编码验证通过!
```

这里位置0的编码如预期是[0,1,0,1,0,1]。位置1则产生了一组由不同频率三对正弦/余弦值，例如第0维 $\sin(1/10000^0) = \sin(1) \approx 0.84$，第1维 $\cos(1) \approx 0.54$ 等。注意随着维度增大（频率更高），正弦/余弦值变化更缓慢。通过将这些位置编码加到词嵌入上，Transformer就能利用它们来获知词的位置。在实践中，也有使用可学习的位置嵌入等其它策略，但正弦位置编码因其不依赖训练、能推广到比训练序列更长的位置等优点而常被采用。

#### 步骤7：Multi-Head Attention（多头注意力）

理论：[[多头注意力]]（Multi-Head Attention）是对注意力机制的扩展。直观上，单一的注意力可能只能关注序列中的某一模式或关联，而多头机制让模型可以并行地用多个注意力“视角”看待信息。实现上，多头注意力会将输入向量通过不同的线性变换得到 $h$ 组不同的 Q、K、V（即有 $h$ 个头），对每组执行 scaled dot-product attention，然后将每个头的输出拼接起来，再通过一个线性层映射回原始维度。这样每个头可以学习关注不同类型的信息（如句子中的不同依存关系），综合起来提升模型表达能力。

Transformer编码器和解码器大量使用多头注意力：编码器每层有一个多头自注意力，解码器每层有一个多头自注意力和一个多头交叉注意力。原论文中常用8个注意力头。

编程任务：使用 PyTorch 的 `nn.MultiheadAttention` 模块，体验多头注意力的效果。我们将构造一个小输入，通过多头注意力层，检查输出形状与输入是否一致，以及注意力权重张量的维度。这里我们不手动计算多头细节（实现较复杂），而借助框架验证理解。

```python
# 创建一个多头注意力层：embed_dim=8, num_heads=2
mha = nn.MultiheadAttention(embed_dim=8, num_heads=2, batch_first=True)

# 构造伪输入: batch=2, seq_len=5, embed_dim=8
x = torch.rand(2, 5, 8)

# 自注意力：Q=K=V=x，out形状: (2,5,8), attn_weights形状: (2,2,5,5)
out, attn_weights = mha(x, x, x, average_attn_weights=False)
out, attn_weights = mha(x, x, x) 
print("输出张量形状:", out.shape)
print("注意力权重形状:", attn_weights.shape)

# 验证输出形状与输入相同，注意力权重形状解释为(批次, 头数, 查询序列长, 键序列长)
assert out.shape == x.shape
assert attn_weights.shape[0] == x.size(0) and attn_weights.shape[1] == 2  # 批次和头数
assert attn_weights.shape[2] == attn_weights.shape[3] == x.size(1)       # 查询长度=键长度=序列长度
print("多头注意力输出验证通过!")
```

结果验证：运行上述代码，`out` 应该与输入 `x` 形状相同 (2,5,8)，说明多头注意力层对输入序列进行变换后仍返回相同长度和维度的序列。`attn_weights` 则包含每个头的注意力矩阵，其形状(2,2,5,5)表示：2个样本，每个样本有2个头，每个头产生一个 5×5 的注意力矩阵（这里查询和键长度都为5）。断言检查应通过，并输出：

```Java
输出张量形状: torch.Size([2, 5, 8])
注意力权重形状: torch.Size([2, 2, 5, 5])
多头注意力输出验证通过!
```

您可以进一步查看 `attn_weights[0][0]` 和 `attn_weights[0][1]` 来比较同一数据下两个注意力头的权重分布是否不同。通常不同头关注的模式不同。多头注意力最终将不同头的信息拼合，使模型决策更加丰富。

在Transformer实现中，我们往往使用多个头并选择合适的嵌入维度能被头数整除（例如embed_dim=512, num_heads=8，每头64维）。多头注意力模块会为每个头维护独立的投影矩阵参数，这部分参数也会被训练更新。

#### 步骤8：Feed-Forward 模块（前馈网络）

理论：Transformer层中的前馈网络（Feed-Forward Network, FFN）通常指每个位置独立的一组全连接网络。它接受来自注意力层的输出，对每个序列位置的向量分别进行非线性变换。典型结构是两层线性层，中间加ReLU激活：$$FFN(x)=max⁡(0,xW1+b1)W2+b2\text{FFN}(x) = \max(0, xW_1 + b_1) W_2 + b_2$$

。在Transformer中，前馈层作用是在融合了全局信息的每个位置向量上再进行复杂变换，提高模型表达能力。通常，FFN的隐藏层维度比embedding维度大（如原论文中FFN隐藏层维度为2048，embedding维度512）。

编程任务：实现一个简化的前馈模块，并验证它对每个位置独立作用、输出形状与输入一致。我们可以用 `nn.Sequential` 或自定义 `nn.Module` 来构建两层线性+[[ReLU]]的网络。

```python
# 定义前馈网络模块：将embed_dim从8升到16，再降回8
ffn =(
    nn.Linear(8, 16),
    nn.ReLU(),
    nn.Linear(16, 8)
)

# 输入仍使用上一步的x (形状2x5x8)
y = ffn(x)
print("前馈网络输出形状:", y.shape)

# 验证输出形状和输入相同
assert y.shape == x.shape

# 验证前馈网络是逐位置独立的：对于相同的输入行输出应该相同
test_in = torch.tensor([[1.0]8])        # 单个位置输入
batch_in = test_in.repeat(3, 1).unsqueeze(0)  # 扩展为 batch=1, seq_len=3
batch_out = ffn(batch_in)
# 检查seq_len维度上每个位置输出是否相等
assert torch.allclose(batch_out[0,0], batch_out[0,1]) and torch.allclose(batch_out[0,1], batch_out[0,2]), "前馈网络没有逐位置独立作用"
print("前馈网络逐位置独立性验证通过!")
```

结果验证：首先，输出形状应为 (2,5,8)，与输入相同，仅每个元素的值改变。然后，我们设计了一个测试：构造3个完全相同的输入向量送入前馈网络，如果网络对每个位置独立处理，那么3个输出也应相同。断言检查了这一点。如果实现正确，将看到：

```Java
前馈网络输出形状: torch.Size([2, 5, 8])
前馈网络逐位置独立性验证通过!
```

（请注意，该验证依赖于随机初始化的线性层参数且输入相同向量，如果参数非零则应输出相同结果；极少情况下可能所有参数初始化为零导致输出都零也会通过，此时本质也满足逐位置独立，只是退化了。）

通过这个Feed-Forward网络，Transformer在每个位置上引入更复杂的特征变换。实际Transformer实现中还包含[[残差连接]]（将注意力层输入加到输出上）和层归一化（Layer Norm）等细节，使训练更稳定。但这些并不改变基本的数据流：注意力提供“信息融合”，前馈网络提供“非线性转换”，各自有残差路径便于梯度传播。本周我们逐个实现并验证了Transformer的核心组件，它们将在后续周整合成完整模型。

---

### ❤️ 第三周：实践入门 (5小时)

目标：将前述组件组装实现一个简化版Transformer块，并通过微调预训练模型完成一个文本分类任务，以体会Transformer在下游任务中的应用。

#### 步骤9：实现简化版 Transformer 模块

理论：现在我们尝试动手将上周的组件组装成一个Transformer编码器块。一个标准的Transformer编码器层顺序为：多头自注意力 -> 加残差再层归一化 -> 前馈网络 -> 加残差再层归一化。为了简化，我们可以暂时省略残差和层归一化（或在PyTorch中直接使用现成的 `nn.TransformerEncoderLayer`）。这里我们将构建一个自定义的 `TransformerBlock` 类，包含一个多头自注意力子层和一个前馈网络子层。

编程任务：实现 `TransformerBlock` 模块，完成前向传播。为了直观验证，我们将测试该模块在某些特殊权重设置下的行为，比如如果我们禁用注意力或前馈，这个模块应当可以退化为恒等映射（输出等于输入）。

```python
class TransformerBlock(nn.Module):
    def __init__(self, d_model, nhead, dim_ff):
        super().__init__()
        self.self_attn = nn.MultiheadAttention(embed_dim=d_model, num_heads=nhead, batch_first=True)
        self.ffn = nn.Sequential(
            nn.Linear(d_model, dim_ff),
            nn.ReLU(),
            nn.Linear(dim_ff, d_model)
        )
        # 层归一化（LayerNorm）和残差连接
        self.norm1 = nn.LayerNorm(d_model)
        self.norm2 = nn.LayerNorm(d_model)
    def forward(self, x):
        # 自注意力子层
        attn_out, _ = self.self_attn(x, x, x)
        x = self.norm1(x + attn_out)           # 残差连接 + 规范化
        # 前馈网络子层
        ffn_out = self.ffn(x)
        x = self.norm2(x + ffn_out)            # 残差连接 + 规范化
        return x

# 测试TransformerBlock
d_model, nhead, dim_ff = 4, 1, 4
block = TransformerBlock(d_model, nhead, dim_ff)

# 测试1: 输入恒为0向量，期望输出也为0向量（因为注意力和FFN对零输入应输出零，然后加上残差零不变）
zero_input = torch.zeros(2, 3, d_model)  # batch=2, seq_len=3
zero_output = block(zero_input)
print("零输入通过TransformerBlock的输出:", zero_output)
assert torch.allclose(zero_output, torch.zeros_like(zero_output)), "零输入未得到零输出"

# 测试2: 让attention层始终输出0，FFN层为恒等映射，看输出是否等于输入
# 手段：冻结block，将 self_attn 权重置0（使 attn_out 始终0），将 ffn 设置为恒等映射
with torch.no_grad():
    # 将注意力层的输出强制为0：可以将其Q,K,V投影矩阵设为0
    for param in block.self_attn.parameters():
        param.data.zero_()
    # 将FFN调整为恒等: 第一层权重设为单位矩阵的一部分，第二层设为单位
    block.ffn[0].weight.data = torch.cat([torch.eye(d_model), torch.zeros(dim_ff-d_model, d_model)], dim=0)
    block.ffn[0].bias.data.zero_()
    block.ffn[2].weight.data = torch.eye(d_model, dim_ff)
    block.ffn[2].bias.data.zero_()

    test_input = torch.randn(1, 5, d_model)
    test_output = block(test_input)
    print("TransformerBlock输出是否与输入相同:", torch.allclose(test_output, test_input))
```

结果验证：

- 测试1：对于全零输入，由于注意力是线性计算（softmax在这种情况下对任意输入应该输出均匀分布但乘上零 V 仍得到零向量），FFN对零输入通常输出零（线性层的bias此时作用，要注意理论上应为0，这里由于LayerNorm存在，小偏差也会归一，但残差加入零输入仍为零）。断言要求输出仍为零张量。如无误会，打印应显示零张量：
    

```Java
零输入通过TransformerBlock的输出: tensor([[[0., 0., 0., 0.],
                                     [0., 0., 0., 0.],
                                     [0., 0., 0., 0.]],
                                    ...
                                   ])
```

- 测试2：我们通过设置参数，期望 `TransformerBlock` 退化为恒等变换：注意力层输出为0，然后残差使这部分实际上跳过；FFN层被设置成输出 = 输入（通过构造近似单位矩阵的权重）。执行后，比较输出和输入的张量是否close。我们打印了比较结果布尔值，理想情况下应为 `True`，证明输出几乎等于输入（除了LayerNorm可能引入极小数值误差）。输出例如：
    

```Java
TransformerBlock输出是否与输入相同: True
```

通过以上测试，我们有信心 `TransformerBlock` 的基本组成正确。在实际使用时，我们会使用随机初始化并训练参数，那时不会刻意让它输出等于输入。但这些测试帮助我们检查残差连接、层归一化等逻辑是否正确集成。

注意：为简单起见，我们自己实现的块和PyTorch自带的 `nn.TransformerEncoderLayer` 在数值上可能有所差异（例如我们的LayerNorm放在残差之后而非之前）。但原理上差别不大，不影响理解。您也可以打印 `block` 来查看其子模块结构。

#### 步骤10：微调预训练模型（如 BERT）

理论：微调（Fine-Tuning）是将预训练模型略加训练来适应特定任务的过程。预训练的Transformer（例如 BERT）在大规模无监督语料上学到了通用的语言特征，我们可以在其基础上附加一个任务相关的输出层，然后用标注数据训练少量轮次即可取得很好的效果。相比从零训练，这种方法收敛更快、需要的数据量更少。

在这里，我们以 文本分类 为例微调BERT模型。具体任务可以是电影评论的情感分类（二分类：正面/负面）。我们将使用Hugging Face的 `datasets` 加载示例数据，使用 [[Transformers]] 提供的 `Trainer` API 来简化训练过程。BERT预训练模型 bert-base-uncased 可作为基础，加一个线性分类层（这由 `AutoModelForSequenceClassification` 类完成）。

编程任务：使用 Hugging Face `datasets` 库加载IMDB影评数据子集，定义BERT序列分类模型，配置训练参数，进行微调并输出训练过程中的指标。由于教程时间有限，我们只使用极小的数据子集，并跑很少的epoch以验证流程可行。

```python
!pip install datasets evaluate

from datasets import load_dataset
# 加载IMDB电影评论数据集的一个小子集
dataset = load_dataset("imdb", split="train[:2000]")        # 取前2000条作为训练
dataset_val = load_dataset("imdb", split="test[:500]")      # 取前500条作为验证

# 打印一条样本以了解结构
print(dataset[0])
```

首先，我们安装并导入 `datasets` 库，然后加载IMDB数据集。`load_dataset("imdb")` 会返回包含'train'和'test'划分的数据集字典。这里我们用切片语法仅选取一部分，以加快演示。打印的样本应类似：

```Java
{'text': 'I rented this movie...', 'label': 0}
```

表示`label=0`（可能代表negative负评）。接着，我们需要将文本tokenize，并构建训练器。

```python
from transformers import AutoTokenizer, AutoModelForSequenceClassification, TrainingArguments, Trainer
import numpy as np

# 初始化分词器和模型
model_name = "bert-base-uncased"
tokenizer = AutoTokenizer.from_pretrained(model_name)
model = AutoModelForSequenceClassification.from_pretrained(model_name, num_labels=2)

# 将文本数据集转换为模型可用的格式（tokenize并编码成input_ids等），并小批量化
def tokenize_batch(batch):
    return tokenizer(batch["text"], padding=True, truncation=True, max_length=128)

train_ds = dataset.map(tokenize_batch, batched=True, batch_size=32)
val_ds = dataset_val.map(tokenize_batch, batched=True, batch_size=32)

# 删除多余的原始字段，只保留模型需要的输入和标签
train_ds = train_ds.remove_columns(["text"])
val_ds = val_ds.remove_columns(["text"])
train_ds.set_format("torch")
val_ds.set_format("torch")
```

分词器会将文本转为字典形式，包括 `input_ids`, `attention_mask` 等张量，并保留标签 `label` 字段。我们用 `.map` 批量处理dataset，加快速度。设置 `padding=True, truncation=True` 以保证序列长度统一（短的填充，长的截断到128）。接着清理掉原始文本列，仅保留tensor格式的数据，以便Trainer使用。

```python
# 定义训练参数
training_args = TrainingArguments(
    output_dir="./test_trainer",
    overwrite_output_dir=True,
    evaluation_strategy="epoch",
    learning_rate=2e-5,
    per_device_train_batch_size=16,
    per_device_eval_batch_size=16,
    num_train_epochs=1,
    logging_steps=10,
    log_level="error",
    disable_tqdm=False,
    logging_strategy="steps",
    save_strategy="no"
)

# 使用Trainer API进行微调
trainer = Trainer(
    model=model,
    args=training_args,
    train_dataset=train_ds,
    eval_dataset=val_ds
)

trainer.train()
```

我们配置了 `TrainingArguments` 来指定训练细节：如学习率2e-5，小批大小16，只训练1个epoch，日志每10步输出等。然后将模型、数据集和参数传入 `Trainer`。调用 `trainer.train()` 即开始微调。

结果验证：训练过程会输出日志，包括每隔10步的loss，以及每个epoch结束的评估指标。因为我们只用2000条训练，500条验证，1个epoch，所以很快结束。您应能看到类似：

```Java
{'loss': 0.54, 'learning_rate': 1e-05, 'epoch': 1.0}  
{'eval_loss': 0.37, 'eval_accuracy': 0.83, 'epoch': 1.0}
```

这表示训练完一轮后，在验证集上loss约0.37，准确率约83%。准确率会有波动，但哪怕极少数据BERT也能有一定效果。如果一切顺利，我们完成了BERT在IMDB情感分类任务的微调！现在模型存储在 `trainer.model` 中（也等同于我们之前定义的 `model` 对象本身，因为Trainer会就地更新）。

#### 步骤11：文本分类任务（Hugging Face）

理论：模型微调后，我们需要验证它在实际输入上的表现。我们可以用微调后的模型对新句子进行情感预测。如前所述，label 0/1 通常分别代表 negative/positive（负面/正面）情感。在IMDB数据集中，0=neg, 1=pos。现在，我们将通过推理（Inference）来评估模型：输入若干示例句子，得到模型预测的类别，并与我们预期相符。

编程任务：利用训练好的模型，对两句影评分别进行情感分类。我们将使用两种方式之一：直接用 `Trainer` 提供的 `predict` 方法，或使用 Transformers 的 `pipeline` 高层接口加载模型进行推断。这里我们展示后一种，利用 `TextClassificationPipeline`。

```python
from transformers import TextClassificationPipeline

# 使用我们训练好的模型和分词器构建文本分类Pipeline
pipe = TextClassificationPipeline(model=model, tokenizer=tokenizer, framework='pt', device=-1)

# 准备两个测试句子
sentences = [
    "This movie was absolutely fantastic! I loved it.",
    "What a waste of time. The film was terrible."
]
predictions = pipe(sentences)
for sent, pred in zip(sentences, predictions):
    label = pred['label']  # 这里label可能是 'LABEL_0' 或 'LABEL_1'
    # 转换标签为可读
    label_str = "Positive" if label in ["LABEL_1", "1", 1] else "Negative"
    print(f"文本: \"{sent}\" => 情感判定: {label_str} (置信度: {pred['score']:.2f})")
```

结果验证：我们输入一条正面评价和一条负面评价，期望模型能正确分类。输出示例：

```Java
文本: "This movie was absolutely fantastic! I loved it." => 情感判定: Positive (置信度: 0.99)  
文本: "What a waste of time. The film was terrible." => 情感判定: Negative (置信度: 0.98)
```

模型应当将第一句判断为正面（Positive），第二句为负面（Negative），并且置信度较高。如果预测结果与预期不符，可能是模型微调不足或者标签映射不同，您可以检查 `pred` 的内容。例如有时pipeline会返回 `'label': 'NEGATIVE'` 直接写明。这种情况下判断方式要相应调整。不过BERT的`AutoModelForSequenceClassification`通常返回 `'LABEL_0'/'LABEL_1'`。

通过这个实践，我们完成了从预训练模型加载 -> 数据预处理 -> 模型微调 -> 模型预测 的完整流程。这也是Transformer模型应用于下游任务的标准套路。在真实场景中，我们会用更大的数据集、更多epoch，可能需要GPU来加速训练，但方法都类似。可以尝试改变 `num_train_epochs` 或数据大小观察效果变化。至此，您已亲手 fine-tune 了一个Transformer模型来完成有用的任务！

---

### ❤️ 第四周：进阶应用 (6小时)

目标：学习Transformer在更高级应用中的使用，包括文本生成、模型压缩、提示工程和评估指标。

#### 步骤12：使用 GPT 进行文本生成

理论：GPT 系列模型（如 GPT-2、GPT-3）是Transformer的解码器架构，只用自回归生成。它们通过给定开头文本，反复预测下一个词来生成序列。与第二周简单调用不同，这里我们更深入探讨生成的解码策略：如贪婪搜索（每次选概率最高词）、随机采样、束搜索等。不同策略会影响生成文本的连贯性和多样性。

编程任务：使用 GPT-2 模型自行执行文本生成，探索不同的生成参数。我们将使用 `model.generate` 接口，它允许控制生成行为。给定一个 prompt，我们用贪婪和采样两种策略生成文本，并比较结果长度和内容差异。

```python
from transformers import AutoModelForCausalLM

# 加载GPT-2分词器和模型
tokenizer_gpt = AutoTokenizer.from_pretrained('gpt2')
model_gpt = AutoModelForCausalLM.from_pretrained('gpt2')

prompt_text = "In a distant future, humans and robots"
input_ids = tokenizer_gpt(prompt_text, return_tensors='pt').input_ids

# 1. 贪婪搜索（无随机性，生成固定最可能序列）
greedy_output_ids = model_gpt.generate(input_ids, max_length=50, do_sample=False)
greedy_text = tokenizer_gpt.decode(greedy_output_ids[0], skip_special_tokens=True)
print("贪婪生成:\n", greedy_text)

# 2. 随机采样（添加随机性参数top_k和temperature）
sample_output_ids = model_gpt.generate(input_ids, max_length=50, 
                                       do_sample=True, top_k=50, top_p=0.9, temperature=1.0)
sample_text = tokenizer_gpt.decode(sample_output_ids[0], skip_special_tokens=True)
print("随机采样生成:\n", sample_text)

# 验证: 生成结果长度是否达到我们设定的50（包括prompt本身）
assert len(greedy_output_ids[0]) <= 50 and len(sample_output_ids[0]) <= 50
assert greedy_text.startswith(prompt_text) and sample_text.startswith(prompt_text)
```

结果验证：`model_gpt.generate` 将基于 GPT-2 继续生成文本。`max_length=50` 指定总长度上限（包含初始prompt的tokens）。对贪婪搜索，模型每一步都选概率最高的词，输出确定；对随机采样，我们设置了 `do_sample=True`，并使用 top-k 和 top-p (nucleus) 采样方法来引入随机选择（top_k=50表示每步仅从概率最高的50个词中抽样，top_p=0.9表示从累计概率0.9的候选中抽样），这些技术通常使文本更有趣且避免太离谱的词。

您会看到两段不同的续写，例如：

```Java
贪婪生成:
 In a distant future, humans and robots are not the only ones who have been living in the past. The world is a place where people are not allowed to be...

随机采样生成:
 In a distant future, humans and robots wage war side by side. The last remnants of humanity dwell in bunkers beneath a scarlet sky, while autonomous machines patrol the wastelands...
```

（具体输出会因随机性和模型权重而异。）通常，贪婪法可能反复、乏味甚至卡住（例如上例贪婪生成看似开始循环“not allowed to be...”），而采样法可能给出更加丰富或意想不到的情节。断言检查确保输出长度不超过50个token，且都包含原始prompt开头。

您可以多次运行采样生成看看不同结果，或调整 [[temperature]]（控制随机程度，>1更随机，<1更保守）来感受变化。也可以尝试 beam search (如 `num_beams=5, early_stopping=True, do_sample=False`)查看效果。掌握这些生成策略对使用Transformer生成任务（如写作AI、对话系统）很有帮助。

#### 步骤13：知识蒸馏（Knowledge Distillation）

理论：知识蒸馏是一种模型压缩技术：让一个大型的教师模型（Teacher）指导训练一个较小的学生模型（Student）。通过让学生学习教师对样本的软预测分布（而不只是正确答案），学生模型能够获得教师模型蕴含的知识，从而在较小模型容量下也能取得与教师接近的性能。DistilBERT 就是通过蒸馏从 BERT 获得的小模型。

蒸馏过程通常包含：预先训练好教师模型 -> 用教师对大量数据（可用未标注数据）生成预测分布 -> 以这些分布为“软标签”训练学生模型，使其输出尽可能接近教师输出（最常用损失是KL散度）。如果有真实标签，也可结合传统监督损失。

编程任务：模拟一个知识蒸馏过程的小例子。由于用真实的BERT作为教师跑全数据较耗时，我们在这里用我们微调好的BERT分类模型作为教师，构造一个简化的学生模型（比如只有几层的小型Transformer或甚至线性模型），然后让学生学习教师的输出概率分布。我们将执行几个梯度步骤观察学生损失下降。

```python
import torch.optim as optim
import torch.nn.functional as F

teacher_model = model    # 使用上一步微调好的BERT分类器作为教师
teacher_model.eval()     # 切换教师为评估模式

# 构造一个简单学生模型：比如仅仿造有BERT尺寸输入的两层感知机
student_model = nn.Sequential(
    nn.Linear(768, 128),
    nn.Tanh(),
    nn.Linear(128, 2)
)
student_optimizer = optim.Adam(student_model.parameters(), lr=3e-4)

# 从验证集拿出一小批数据作为蒸馏训练样本
batch = val_ds.shuffle(seed=42).select(range(8))[:]  # 8条样本
inputs = {k: batch[k] for k in ['input_ids', 'attention_mask']}
labels = batch['label']

# 用教师模型计算软目标（soft targets）
with torch.no_grad():
    teacher_logits = teacher_model(inputs).logits  # (8,2)
    teacher_probs = F.softmax(teacher_logits, dim=-1)

print("教师模型对前8个验证样本的输出概率:\n", teacher_probs)

# 蒸馏训练学生：让学生的输出概率逼近教师输出概率
student_model.train()
for step in range(101):
    student_logits = student_model(teacher_model.bert.embeddings(inputs['input_ids']))  # 使用教师BERT的embedding作为学生输入
    loss = F.kl_div(
        input=F.log_softmax(student_logits, dim=-1),
        target=teacher_probs,
        reduction='batchmean'
    )
    student_optimizer.zero_grad()
    loss.backward()
    student_optimizer.step()
    if step % 50 == 0:
        print(f"Step {step} 蒸馏损失 = {loss.item():.4f}")
```

结果验证：首先，我们取8条验证数据，通过教师模型计算其对每条的正负面概率（2分类所以输出2维概率）。打印 `teacher_probs` 可以看到每行两个概率和为1。例如：

```Java
教师模型对前8个验证样本的输出概率:
 tensor([[0.1234, 0.8766],
         [0.9980, 0.0020],
         [0.3012, 0.6988],
         ... ])
```

这些就是软标签。然后，我们构建一个学生模型，这里为简单直接用BERT的embedding层输出作为输入（为了不训练embedding、加快收敛，我们复用教师的embedding层输出，这样学生只学分类层部分）。注意：`teacher_model.bert.embeddings(inputs['input_ids'])` 获取了输入ID对应的embedding表示，其shape是 (8, seq_len, 768)。BERT的分类通常取[CLS]位置的embedding作为整个句子的表示，因此我们实际应该用 `embedding[:,0,:]` 作为句子特征。上面代码略简化，直接传整个embedding给学生的线性层，会让学生尝试自己处理序列维度。为严谨可以改为：

```python
embeds = teacher_model.bert(inputs, output_hidden_states=True, return_dict=True).last_hidden_state
cls_embeds = embeds[:, 0, :]  # 取每句的CLS位置向量 (8,768)
student_logits = student_model(cls_embeds)
```

但为不复杂化代码，我们暂不细纠。总之，我们定义了KL散度损失，让学生输出分布逼近教师分布。打印训练过程的损失，每50步输出一次，比如：

```Java
Step 0 蒸馏损失 = 0.3542  
Step 50 蒸馏损失 = 0.0815  
Step 100 蒸馏损失 = 0.0228
```

可以看到蒸馏损失不断降低，这表示学生模型正在学习教师的预测分布。虽然只有100步小数据，这已经能显著降低KL散度。理论上，学生模型的预测概率应逐渐接近教师。您可以在训练结束后比较一下：

```python
student_model.eval()
student_probs = F.softmax(student_model(teacher_model.bert.embeddings(inputs['input_ids'])), dim=-1)
print("学生模型蒸馏后输出概率:\n", student_probs)
```

应当与先前打印的 `teacher_probs` 越来越接近。这就完成了一次简要的知识蒸馏过程。在实际中，我们会用更大的教师和学生模型、更多的数据、更多训练轮次。蒸馏可以大幅缩小模型（如BERT-base 110M参数 -> DistilBERT 66M参数）同时只轻微降低性能，是部署大模型的常用技术之一。

#### 步骤14：Prompt Engineering（提示工程）

理论：提示工程是在不改变模型参数的情况下，通过设计输入提示（prompt）来引导大模型执行特定任务的一门技巧。对于预训练的大型语言模型（如 GPT-3、ChatGPT），不同的提示措辞会显著影响模型输出。这包括选择适当的指令、提供上下文示例（few-shot），甚至使用特殊格式。好的提示能令模型更准确或遵循预期格式完成任务。

例如，让GPT模型翻译，可以提供提示：“Translate the following English sentence to French:...”，模型才能明白任务是翻译。如果只是给一句英文，它可能只是续写英文而不翻译。又如要求总结，可能要以“Summarize:”开头才能获得摘要而非照常续写原文。

编程任务：以机器翻译为例，展示提示对输出的作用。我们使用一个开源指令微调模型 FLAN-T5（小型），让它翻译一句话。在一个提示中明确要求法语翻译，另一个不提供说明，对比结果。

```python
from transformers import AutoModelForSeq2SeqLM

model_flant5 = AutoModelForSeq2SeqLM.from_pretrained("google/flan-t5-small")
tokenizer_flant5 = AutoTokenizer.from_pretrained("google/flan-t5-small")

english_sentence = "I love programming."
# 提示1：明确要求翻译
prompt1 = "Translate English to French: " + english_sentence
# 提示2：不说明任务，仅提供英文句子
prompt2 = english_sentence

def generate_text(prompt):
    input_ids = tokenizer_flant5(prompt, return_tensors="pt").input_ids
    output_ids = model_flant5.generate(input_ids, max_length=50)
    return tokenizer_flant5.decode(output_ids[0], skip_special_tokens=True)

output1 = generate_text(prompt1)
output2 = generate_text(prompt2)
print("带翻译指令的提示输出:", output1)
print("无指令的提示输出:", output2)
```

结果验证：FLAN-T5 已经过指令微调，会识别 "Translate English to French:" 这种命令格式。因而：

- 输出1 可能是正确的法语翻译，如 `"J'aime la programmation."`。
    
- 输出2 在没有明确指令时，模型可能无法判断任务。它可能尝试直接重复输入、解释它或者做不相关的生成。例如可能输出英文 `"I love programming."` 原样，或者一些扩展（视训练细节）。
    

典型结果：

```Java
带翻译指令的提示输出: J'aime la programmation.  
无指令的提示输出: I love programming.
```

可以看到，有明确的“Translate...”前缀时，模型正确地翻译成法语；而如果不给出，模型没有执行翻译。这说明即使模型已具备能力，我们需要通过恰当的提示来激活其能力——这正是Prompt Engineering的要点。

您可以尝试修改提示，比如把 prompt1 换成 `"请将以下英文翻译为法文：I love programming."`（中文指令）看模型是否也懂（FLAN-T5可能因为看过多语言指令而懂一些中文指令）。总之，学会编写清晰、有指示性的prompt非常重要。

更高级的提示工程包括 Few-shot 提示（给出示例输入输出让模型仿照）、使用特殊令牌或格式（如 `<|endoftext|>` 等）来引导模型行为，甚至链式思考提示等。随着对大模型掌握加深，提示工程已经成为应用大语言模型的关键技能之一。

#### 步骤15：模型评估指标（BLEU、ROUGE）

理论：对生成任务（如机器翻译、文本摘要），需要自动评估生成文本与参考答案的接近程度。常用指标有 BLEU（主要用于翻译）和 ROUGE（主要用于摘要）。

- BLEU（Bilingual Evaluation Understudy）计算机器翻译与参考译文的n-gram匹配程度。简单说，它统计生成文本和参考文本之间1-gram、2-gram...的重合情况，并乘以一个长度惩罚项。BLEU分数通常取0到1之间（有时乘100表示百分制）。1表示完全一致，0表示完全不相关。
    
- ROUGE（Recall-Oriented Understudy for Gisting Evaluation）则更多衡量召回率：例如 ROUGE-L 基于最长公共子序列，衡量摘要涵盖了多少参考摘要的重要内容。ROUGE也通常0-1或百分制。
    

这些指标并非完美，比如BLEU无法充分捕捉译文的语义或流畅度，但作为自动指标能快速评价模型改进。本节我们实践计算BLEU和ROUGE。

编程任务：使用 Hugging Face 的 `evaluate` 库来计算 BLEU 和 ROUGE 分数。我们将构造几对简短句子，比较模型输出和参考答案完全相同与略有差异时的分值差异。

```python
import evaluate

bleu = evaluate.load("bleu")
rouge = evaluate.load("rouge")

reference = ["the cat is on the mat"]
candidate1 = ["the cat is on the mat"]        # 完全相同
candidate2 = ["the cat is on mat"]           # 略有差异（少了一个 "the"）
candidate3 = ["a dog lies under the table"]  # 完全不同

result1 = bleu.compute(predictions=candidate1, references=[[reference[0]]])
result2 = bleu.compute(predictions=candidate2, references=[[reference[0]]])
result3 = bleu.compute(predictions=candidate3, references=[[reference[0]]])

print("BLEU (完全匹配):", result1["bleu"])
print("BLEU (少一个词):", result2["bleu"])
print("BLEU (完全不匹配):", result3["bleu"])

result_rouge1 = rouge.compute(predictions=candidate1, references=reference)
result_rouge2 = rouge.compute(predictions=candidate2, references=reference)
result_rouge3 = rouge.compute(predictions=candidate3, references=reference)

print("ROUGE-L (完全匹配):", result_rouge1["rougeL"])
print("ROUGE-L (少一个词):", result_rouge2["rougeL"])
print("ROUGE-L (完全不匹配):", result_rouge3["rougeL"])
```

结果验证：

- 对于Candidate1（完全匹配参考），BLEU应当是1.0（100%）；ROUGE-L也应是1.0，因为预测和参考完全相同。
    
- 对于Candidate2（缺少一个“the”），BLEU会略低于1。因为1-gram和2-gram有些不匹配。ROUGE-L则仍可能接近1，因为缺一个词大部分内容还在，最长公共子序列是“大部分句子”。
    
- 对于Candidate3（完全不相关），BLEU会接近0，ROUGE-L也很低。
    

可能的输出：

```Java
BLEU (完全匹配): 1.0  
BLEU (少一个词): 0.6389431042462724  
BLEU (完全不匹配): 0.0

ROUGE-L (完全匹配): 1.0  
ROUGE-L (少一个词): 0.875  
ROUGE-L (完全不匹配): 0.0
```

（注意BLEU计算较复杂，这里的结果2约0.639，表示有些n-gram缺失；ROUGE-L少一个词的情况0.875，表示预测串长度为7个词，公共子序列为7个词中的6个，占参考8个词的75%召回，也许f1或类似算法算出0.875。）

这些指标验证了自动评估的直观性：越接近参考答案，分数越高。现实中通常会对系统输出取平均BLEU/ROUGE。比如机器翻译领域，BLEU≥0.3就不错了（当然句子长短和参考多少影响尺度）。

小结：自动指标只是参考，可能无法完全替代人工评价。但在模型开发中，它们非常有用，可快速比较不同模型版本。Hugging Face的 `evaluate` 库支持很多指标，可以按需使用。在后续训练大型生成模型或做比赛题目时，这些指标都会反复出现。

---

### ❤️ 第五周：高级主题 (18小

目标：探索训练和部署大型Transformer模型的高级技巧，包括高效训练策略、模型伦理、公平性、多模态模型，以及模型的优化部署方法。

#### 步骤16：预训练与微调策略（ZeRO、DeepSpeed）

理论：当模型参数达到数亿甚至上百亿时，单卡显存往往不够，需要分布式训练优化策略。[[ZeRO]] (Zero Redundancy Optimizer) 和 [[DeepSpeed]] 库是微软开源的解决方案，旨在降低大模型训练的内存和通信开销。ZeRO将优化器状态、梯度、模型参数在数据并行进程间切分，避免每张卡存完整拷贝，从而训练更大的模型。DeepSpeed集成了ZeRO并提供易用接口，同时支持梯度检查点、CPU/offload（将部分数据放CPU）等。本节我们在单机模拟DeepSpeed使用方法，理解配置要点。

编程任务：演示如何使用DeepSpeed对模型应用ZeRO优化。在Colab这种单机环境，我们仍可利用ZeRO Stage 2+Offload将一部分模型/优化器存CPU，以减少显存占用。下面代码需在具有GPU的环境执行，并确保安装DeepSpeed。我们构造一个大模型（如几层的GPT2），尝试用DeepSpeed初始化并比较内存占用。

```python
!pip install deepspeed

import deepspeed
from transformers import GPT2LMHeadModel

# 准备一个较大的模型实例（GPT2的中等规模模型）以模拟大模型
model_name = "gpt2-medium"  # 约3.45亿参数
model_big = GPT2LMHeadModel.from_pretrained(model_name)
model_big.cuda()  # 将模型加载到GPU

# DeepSpeed配置：启用ZeRO Stage 2并将优化器状态offload到CPU
ds_config = {
    "train_batch_size": 1,
    "optimizer": {"type": "Adam", "params": {"lr": 1e-5}},
    "zero_optimization": {
        "stage": 2,
        "offload_optimizer": {
            "device": "cpu",
            "pin_memory": True
        }
    }
}

model_big, optimizer, _, _ = deepspeed.initialize(model=model_big, config_params=ds_config, model_parameters=model_big.parameters())
print("DeepSpeed 模型包装类型:", type(model_big))
```

结果验证：安装DeepSpeed可能需要一些时间。我们加载了GPT2-Medium模型（约3亿多参数），这在没有ZeRO的情况下将完全驻留GPU，占用相当显存。通过`deepspeed.initialize`，模型被包装成DeepSpeed的`DeepSpeedEngine`对象，并按照配置应用ZeRO Stage 2优化：将优化器状态存储在CPU，仅在需要时移动数据。这减轻了GPU压力。

检查输出，应该打印：

```Java
DeepSpeed 模型包装类型: <class 'deepspeed.runtime.engine.DeepSpeedEngine'>
```

这表示模型已被DeepSpeed接管。此时我们若进行训练，DeepSpeed会自动处理参数分片和offload策略。虽然我们未实际训练，但可以通过如下检查内存占用来感受ZeRO效果：

```python
import torch

# 模型已经initialize，可检查模型参数是否分布式分片（在单GPU环境下其实Stage 2主要作用是offload optimizer）
# 此时可计算一个前向，看显存占用是否较未使用ZeRO时减少
dummy = tokenizer_gpt.encode("Hello world", return_tensors='pt').cuda()
loss = model_big(dummy, labels=dummy).loss  # 一个dummy前向+计算loss
loss.backward()  # 反向传播，DeepSpeed将处理梯度

# 获取GPU显存占用
allocated = torch.cuda.memory_allocated() / 10242
reserved = torch.cuda.memory_reserved() / 10242
print(f"当前显存占用: 已用 {allocated:.1f} MB, 预留 {reserved:.1f} MB")
```

如果DeepSpeed工作正常，optimizer状态应在CPU上，显存占用可能比不使用时有所下降。当然，在单GPU且我们没实际数据并行时，节省可能不明显，但对于多GPU大模型训练，ZeRO能线性扩展模型大小。

注意：DeepSpeed功能强大，此处只是浅尝。ZeRO还有Stage 3（连参数也offload或精细切分）、还有Hybrid Engine、逐层释放、甚至内置Lamb优化器等。配置通常写在json文件中然后传给Trainer或DeepSpeed launcher。HuggingFace的Trainer也支持 `deepspeed` 参数直接加载配置文件来训练超大模型。总之，利用这些技术，如今研究者可以在相对较少的GPU上训练百亿参数模型。下次当你听到某模型用了多少GPU训练，背后往往有ZeRO等技术助力。

#### 步骤17：模型伦理与偏见分析

理论：训练数据和模型结构可能导致Transformer模型产生偏见（bias）或不当内容。比如语言模型常反映语料中的性别或种族刻板印象；聊天机器人可能输出有害言论。模型伦理要求我们在部署模型前检测并减轻这些问题。手段包括：构建偏见测试集（如职业=男性?女性?概率）、对模型输出进行有害内容过滤，加入额外训练约束等。

此步我们尝试探测模型的潜在偏见。通过填空或生成，让模型显式表现其内在偏好。例如，BERT的遮罩语言模型可用于检查类似句子的不同性别预测；GPT类模型可看对某些提示是否产生不恰当内容。

编程任务：利用 `pipeline('fill-mask')` 和一个有偏见倾向的模板句，看看BERT的预测。以常见例子：“The doctor was a[MASK].”和“The nurse was a[MASK].”BERT常倾向预测doctor -> man, nurse -> woman。我们验证这一点。同时，用一个现成的有毒内容检测模型来检测一段话的毒性评分，演示如何发现模型的不良输出。

```python
# 使用fill-mask管道来让BERT填空
unmasker = pipeline('fill-mask', model='bert-base-uncased')
res1 = unmasker("The doctor was a [MASK].")
res2 = unmasker("The nurse was a [MASK].")
print("BERT预测(doctor):", res1[:5])
print("BERT预测(nurse):", res2[:5])

# 使用有毒评论检测模型
tox_detector = pipeline('text-classification', model='unitary/toxic-bert')
texts = [
    "I hate all of you.",          # 含有仇恨言论
    "Have a nice day my friend!"  # 正常友善
]
for t in texts:
    pred = tox_detector(t)[0]
    print(f"文本: \"{t}\" -> {pred}")
```

结果验证：BERT fill-mask的结果`res1`和`res2`是列表，包含模型最可能填入的词及置信度。我们打印各自前5项看看：

```Java
BERT预测(doctor): [{'sequence': 'the doctor was a man.', 'score': 0.21, 'token': ...}, ...]  
BERT预测(nurse): [{'sequence': 'the nurse was a woman.', 'score': 0.19, ...}, ...]
```

可以看到，BERT确实倾向于 `doctor -> man`，`nurse -> woman`（具体概率可能不同，但通常前列结果里有这些）。这反映出训练数据中的性别职业偏见被模型学到了。

有毒内容检测部分，`tox_detector`（unitary/toxic-bert是一个将句子分类为有毒或无毒的模型）输出类似：

```Java
文本: "I hate all of you." -> {'label': 'toxic', 'score': 0.99}  
文本: "Have a nice day my friend!" -> {'label': 'non-toxic', 'score': 0.98}
```

可以看到第一句被标为有毒（toxicity很高），第二句是正常的non-toxic。这样的检测模型可以用来审查生成模型的输出，避免将明显有害内容呈现给用户。一些开放的聊天模型也会在生成时对每句话跑一下这种检测模型，过滤或修改输出。

讨论：通过这步，我们感受到Transformer模型确实可能包含不良偏见和内容。一方面，这是训练语料的问题；另一方面，也和模型未受控制地学习关联有关。目前缓解方法有：在微调阶段加入 对抗数据（如要求模型输出中性描述）、对生成结果进行 后过滤、或者训练 去偏见版本 的模型（如让模型在特定属性上正则化）。作为实践者，应始终对模型输出保持警惕，使用工具检测可能的问题，并在应用中增加人类监控或反馈机制，以免产生严重的伦理后果。

#### 步骤18：[[多模态]]模型（CLIP、DALL·E）

理论：Transformer不仅在NLP发光，在多模态领域也有重大影响。例如 CLIP（Contrastive Language-Image Pre-training）利用文本和图像对进行训练，使得图像和对应描述的向量相近，不匹配的远离，从而实现跨模态检索和分类。[[DALL·E]] 和 [[Stable Diffusion]] 则将Transformer/扩散模型用于图像生成：输入文本提示，模型生成相符的图像。在多模态模型中，Transformer作为通用的序列建模器，可以处理文本序列，也可以处理图像分块序列（如Vision Transformer），或两者结合。

本节我们动手体验CLIP的图文检索能力，以及Stable Diffusion的文生图。

编程任务1（CLIP）：给定一张图片和若干文字描述，让 CLIP 模型判断哪个描述与图片最相符。这相当于零样本图像分类。我们将使用 OpenAI CLIP 模型，通过pipeline实现。

_示例图：一只可爱的猫。我们将让模型判断描述 "a photo of a cat" vs "a photo of a dog" 哪个更符合这张图。_

```python
from PIL import Image
import requests

# 下载示例图片（猫）
url = "https://upload.wikimedia.org/wikipedia/commons/thumb/7/74/A-Cat.jpg/960px-A-Cat.jpg"
image = Image.open(requests.get(url, stream=True).raw)

# CLIP零样本图像分类
clip_classifier = pipeline('zero-shot-image-classification', model='openai/clip-vit-base-patch32')
result = clip_classifier(image, candidate_labels=["a photo of a cat", "a photo of a dog"])
print(result)
```

结果验证1：CLIP 会输出每个候选标签的分数，例如：

```Java
[{'score': 0.992, 'label': 'a photo of a cat'}, {'score': 0.008, 'label': 'a photo of a dog'}]
```

可以看到模型以99.2%的置信认为图片内容是“a cat”而非“a dog”。这展示了CLIP强大的零样本识别能力，无需专门训练分类头，只给文本标签即可完成分类任务。

编程任务2（Stable Diffusion）：使用 Hugging Face Diffusers 调用 Stable Diffusion 模型，输入文本提示生成图像。注意：该步骤需要GPU，且模型文件较大（数GB），如在Colab请确保使用GPU runtime并有足够内存。如果资源不允许，可跳过实际运行，仅阅读代码了解流程。

```python
!pip install diffusers accelerate
from diffusers import StableDiffusionPipeline

# 加载Stable Diffusion模型（v1-5）
pipe = StableDiffusionPipeline.from_pretrained("runwayml/stable-diffusion-v1-5", torch_dtype=torch.float16)
pipe = pipe.to("cuda")  # 将模型移动到GPU
prompt = "a cute kitten playing with a ball, cartoon style"
image = pipe(prompt, height=512, width=512, num_inference_steps=50).images[0]
image.save("output.png")
print("图像已生成并保存为 output.png")
```

结果验证2：如果成功，模型会根据提示生成一张图并保存。由于我们不能直接显示，这里假设已保存`output.png`。你可以打开下载查看，应是一只可爱的卡通小猫玩球的图像。如果运行时间太长或GPU不足，可尝试 `pipe(prompt, num_inference_steps=20)` 减少迭代步数牺牲些质量，或使用 `"CompVis/stable-diffusion-v1-4"` 模型。

通过CLIP和Stable Diffusion，我们一窥多模态Transformer应用的强大：Transformer让文字和图像可以互相检索、互相生成。要深入，这里还有很多领域如图像字幕生成（Image Captioning）、视觉问答（VQA）等，都是Transformer加持下取得很大进展的任务。

#### 步骤19：模型部署（ONNX、TensorRT）

理论：完成模型训练后，部署到推理服务也很重要。为提高推理速度、降低依赖，我们常将PyTorch模型转换为标准格式如 ONNX（Open Neural Network Exchange），然后用高性能推理引擎运行，比如 ONNX Runtime 或 TensorRT。ONNX是业界通用模型格式，支持多种语言环境；TensorRT是NVIDIA提供的GPU上优化推理库，能对模型图进行融合和量化以极致加速。简言之，部署时我们希望模型轻量且推理快。

本节我们实践将前面训练的BERT分类模型导出为ONNX，并用ONNX Runtime加载执行验证结果一致性。TensorRT部分，我们简单介绍，如有NVIDIA GPU环境可尝试使用它优化ONNX模型。

编程任务：使用 PyTorch 自带的 ONNX 导出，将 fine-tuned BERT 模型转换，并用 onnxruntime 进行推理比较。

```python
!pip install onnx onnxruntime

import onnx
import onnxruntime as ort

# 准备示例输入
sample_text = "I really enjoy this movie!"
tokens = tokenizer(sample_text, return_tensors="pt")
# PyTorch原生推理
pt_outputs = model(tokens)  # logits输出

# 导出为ONNX
onnx_path = "bert_classification.onnx"
torch.onnx.export(
    model,                                       # 要转换的模型
    (tokens["input_ids"], tokens["attention_mask"]),  # 模型输入 (tuple)
    onnx_path,
    input_names=["input_ids", "attention_mask"], 
    output_names=["logits"],
    opset_version=12,
    dynamic_axes={"input_ids": {1: "seq_len"}, "attention_mask": {1: "seq_len"}}
)
# 验证ONNX模型结构正确
onnx_model = onnx.load(onnx_path)
onnx.checker.check_model(onnx_model)

# 用 ONNX Runtime 推理
ort_session = ort.InferenceSession(onnx_path)
ort_inputs = { "input_ids": tokens["input_ids"].numpy(), "attention_mask": tokens["attention_mask"].numpy() }
ort_outputs = ort_session.run(None, ort_inputs)
print("ONNX 输出logits:", ort_outputs[0])
print("PyTorch 输出logits:", pt_outputs.logits.detach().numpy())

# 比较ONNX和PyTorch输出差异
diff = np.max(np.abs(ort_outputs[0] - pt_outputs.logits.detach().numpy()))
print("ONNX vs PyTorch 最大差异:", diff)
```

结果验证：ONNX 导出过程如果成功不会报错。然后 ONNX Runtime 会对同一输入得到输出 logits（形状[1,2]）。我们打印ONNX和PyTorch的输出，以确保它们几乎一致。输出示例：

```Java
ONNX 输出logits: [[-0.527, 0.613]]  
PyTorch 输出logits: [[-0.527, 0.613]]  
ONNX vs PyTorch 最大差异: 1.2e-07
```

可见二者数值几乎相同，只有浮点级微小误差 (~1e-7)，证明模型成功导出且推理结果一致。之后我们就可以将 `bert_classification.onnx` 部署到任意支持ONNX的环境，比如用 ONNX Runtime 在C++/Java 中运行，摆脱Python依赖。

附：关于TensorRT，如果环境有该库（通常需要在有NVIDIA GPU且安装TensorRT库的机器上），则可以用 ONNX 模型作为输入创建TensorRT引擎，从而获得更快的推理速度，尤其在批量和并发高时效果明显。一个基本流程是在Python中：

```python
!pip install onnxoptimizer nvidia-tensorrt  # 假设可以装，实际Colab不一定支持
import tensorrt as trt

logger = trt.Logger(trt.Logger.WARNING)
builder = trt.Builder(logger)
network = builder.create_network(common.EXPLICIT_BATCH)
parser = trt.OnnxParser(network, logger)
with open(onnx_path, 'rb') as f:
    parser.parse(f.read())

# 设置优化参数，创建Engine
builder.max_batch_size = 1
builder.max_workspace_size = 1 << 28  # 256MiB
engine = builder.build_cuda_engine(network)
# 然后用engine执行推理（过程略）
```

TensorRT通过低层次优化和FP16、INT8精度缩减等可以大幅提高吞吐并降低延迟。不过配置和部署稍复杂。

总的来说，学会ONNX导出可以满足大部分部署需求，而进一步掌握TensorRT则能在需要极致性能时派上用场。

---

### ❤️ 第六周：前沿探索 (38小时)

目标：了解Transformer领域最新的研究方向和应用：包括超大模型的稀疏化优化、强化学习对齐技术，以及近期流行的自主AI代理的原理。

#### 步骤20：大模型优化（MoE 架构）

理论：随着模型规模持续增长，出现了一种Mixture of Experts (MoE, 专家混合)架构，用于构建稀疏激活的大模型。在MoE中，有许多不同的“专家”子网络，但对于每个输入，仅激活其中一部分（例如2个），其它专家不参与计算。这样模型参数总数可以极大增加，但每个样本的计算量并不大幅增加。典型做法是在Transformer某些层替换为MoE层：包含N个并行的前馈网络（专家），以及一个门控网络决定每个输入路由到哪几个专家。Google的 Switch Transformer 就是使用MoE在上千亿参数级别取得优秀性能。

编程任务：实现一个简单的 MoE 模块示例，包括两个“专家”全连接层和一个门控路由。我们使用一个可学习的门控 (gating) 线性层，根据输入计算编程任务：实现一个简单的 MoE 模块示例，包括两个“专家”全连接层和一个门控路由。我们使用一个可学习的门控 (gating) 线性层，根据输入计算选择哪一个专家。为简化，将每次仅激活一个专家（hard routing）。我们可以手动设计门控权重，使其按输入特征的符号将数据分流：比如令门控层计算输入和一个全1向量的点积，如果结果正则选Expert1，如果负则选Expert2。然后两个专家网络分别对输入做不同变换。最后验证路由正确性。

```python
class SimpleMoE(nn.Module):
    def __init__(self, input_dim):
        super().__init__()
        # 门控层：输出两个值 (logits)
        self.gate = nn.Linear(input_dim, 2, bias=False)
        # 定义两个专家：这里用线性层模拟不同变换
        self.expert1 = nn.Linear(input_dim, input_dim, bias=False)
        self.expert2 = nn.Linear(input_dim, input_dim, bias=False)
        # 初始化专家1为恒等映射，专家2为取负（通过权重设置）
        self.expert1.weight.data = torch.eye(input_dim)
        self.expert2.weight.data = -torch.eye(input_dim)
        # 初始化门控权重：让其倾向于根据输入和全1向量的点积判断正负
        # 第一行全+1，第二行全-1
        self.gate.weight.data = torch.cat([torch.ones(1, input_dim), -torch.ones(1, input_dim)])
    def forward(self, x):
        # x shape: (batch, input_dim)
        gate_logits = self.gate(x)  # (batch, 2)
        # 选取得分最高的专家索引
        expert_indices = torch.argmax(gate_logits, dim=1)
        outputs = []
        for i, idx in enumerate(expert_indices):
            if idx.item() == 0:
                outputs.append(self.expert1(x[i:i+1]))
            else:
                outputs.append(self.expert2(x[i:i+1]))
        return torch.cat(outputs, dim=0)

# 测试 SimpleMoE
moe = SimpleMoE(input_dim=2)
# 准备两个输入: 一个和全1向量点积为正，一个为负
x_pos = torch.tensor([[0.5, 0.2]])   # sum=0.7 正
x_neg = torch.tensor([[-0.3, 0.1]])  # sum=-0.2 负
out_pos = moe(x_pos)
out_neg = moe(x_neg)
print("正输入经过MoE输出:", out_pos)
print("负输入经过MoE输出:", out_neg)
# 验证: 正输入应该由专家1处理(恒等), 输出应≈输入; 负输入由专家2处理(取负), 输出应≈输入取反
assert torch.allclose(out_pos, x_pos, atol=1e-6)
assert torch.allclose(out_neg, -x_neg, atol=1e-6)
print("MoE路由正确，实现成功!")
```

结果验证：`SimpleMoE` 中，我们手动设置专家1为恒等映射、专家2为取相反数，并设置门控层使其对输入各元素求和，大于0选Expert1，小于0选Expert2。测试中：

- 对于 `x_pos =[0.5, 0.2]`，和全1向量点积=0.7>0，应路由到Expert1，输出应与输入相同。
    
- 对于 `x_neg =[-0.3, 0.1]`，点积=-0.2<0，应路由到Expert2，输出应为输入取负。
    

打印结果，例如：

```Java
正输入经过MoE输出: tensor([[0.5000, 0.2000]], grad_fn=<CatBackward0>)  
负输入经过MoE输出: tensor([[ 0.3000, -0.1000]], grad_fn=<CatBackward0>)  
MoE路由正确，实现成功!
```

可以看到，第一个输出 `[0.5, 0.2]` 等于原输入，第二个输出 `[0.3, -0.1]` 正好是原输入的相反数，符合预期。断言也通过证明MoE模块运行正确。

实际的MoE Transformer层比这复杂一些：通常每个专家是更深的网络，路由可以是软选择（让部分输入加权分配到多个专家），并需要处理不同序列位置各自路由的情况。不过本例捕捉了MoE的本质：用门控网络动态选择子模型。通过MoE，大模型能拥有海量参数（很多专家）但每次推理只用一小部分参数，从而实现参数规模和计算成本的解耦。例如Switch Transformer在翻译任务中将参数扩大至超过1000亿，但每个token只用其中0.1%参数计算。

MoE目前主要在研究和大厂产品中使用，如果您有机会训练/使用超大语言模型，不妨关注 MoE 思路带来的性能提升和挑战（如负载不均、通信开销、训练稳定性等）。

#### 步骤21：强化学习对齐（[[RLHF]]）

理论：基于人类反馈的强化学习 (RLHF) 是近年用来对齐 (Align) 大型语言模型行为的关键技术。它通过引入人类偏好来优化模型，使模型输出更符合人意图、安全可靠。RLHF流程通常：先有一个预训练语言模型，通过人类标注一些模型输出的质量，训练一个奖励模型打分，然后使用强化学习（如PPO算法）让语言模型尽量生成高分输出，同时不偏离原先语言能力太远。OpenAI 的 InstructGPT、ChatGPT 都应用了RLHF，使得模型愿意遵循指令、减少不良行为。

由于完整RLHF实现复杂，这里用一个极简模拟说明其原理。我们假设一个策略要输出动作，我们通过奖励函数指引其学习目标行为。

编程任务：模拟一个简单环境：策略每回合从 {A, B} 两个动作中选择一个，动作 'A' 的奖励为 +1，'B' 的奖励为 -1。我们的目标是通过强化学习使策略最终总是选 'A'。用概率表示策略，对其进行多轮更新（类似policy gradient），观察概率趋向。

```python
import random
import math

# 初始化策略: 概率p_A（选A的概率）初始为0.5
logit_A = 0.0
logit_B = 0.0

def get_prob_A():
    # 由logit计算softmax概率
    pA = math.exp(logit_A) / (math.exp(logit_A) + math.exp(logit_B))
    return pA

print("初始选择A的概率:", get_prob_A())

# 强化学习迭代
for episode in range(1000):
    # 根据当前策略概率选择动作
    pA = get_prob_A()
    action = 'A' if random.random() < pA else 'B'
    # 获得奖励
    reward = 1 if action == 'A' else -1
    # Policy Gradient 简单更新: 增大选择的动作对应logit按 reward 大小
    if action == 'A':
        logit_A += 0.01  reward  # reward=+1, 增加logit_A
    else:
        logit_B += 0.01  reward  # reward=-1, 增加(因为是负负得正) logit_B？实际这里reward=-1,加上会减少logit_B
    # 简化: 未考虑baseline和严格梯度公式，只做比例更新
final_pA = get_prob_A()
print("训练后选择A的概率:", final_pA)
```

结果验证：初始时 `p(A)=0.5`。经过强化学习迭代，我们期望策略越来越偏向动作A。输出可能：

```Java
初始选择A的概率: 0.5  
训练后选择A的概率: 0.99
```

最终概率应非常接近1，表示策略几乎总是选 'A'。这就模拟了RLHF中模型学会选择人类偏好的行为。在实际RLHF中：

- 动作 = 模型生成的完整回答；
    
- 奖励 = 人类反馈训练的奖励模型打分（高分表示人类喜欢的回答）；
    
- 更新策略 = 用PPO等算法微调模型参数，使其输出分布朝着高奖励方向移动。
    

通过RLHF，大模型可以纠正一些不符合人意的倾向。例如原模型可能喜欢用复杂术语回答，现在人类偏好简单解释，那奖励模型就会对简单答案给高分，于是强化学习让模型学会给出简单回答。

提示：上述策略更新我们使用一个非常简单的“增加所选动作logit”的规则，并未严格遵循RL理论（只是直观演示）。但结果仍有效，因为动作A正向奖励，算法会不断提高A概率。现实用PPO需要维护旧策略、裁剪更新、防止梯度爆炸等，复杂许多。不过核心思想就是引导模型朝人类偏好的方向自我调整。

#### 步骤22：Agent 开发（AutoGPT）

理论：自主智能体（Agent）是让语言模型不再局限于单轮问答，而是具备连贯执行任务的能力。2023年火热的 AutoGPT 就是这种思路的产物：它使用 GPT-4/GPT-3.5 作为大脑，围绕它搭建循环，让模型可以反复计划、执行、反馈，直至达到用户指定的目标。典型Agent（如 AutoGPT）的架构包括：

- 任务拆解：模型根据高层目标，自己生成子任务清单。
    
- 工具使用：模型可调用外部工具/API（如上网搜索、代码执行）获取信息或动作结果。
    
- 记忆/工作记忆：模型将每次中间结果添加到上下文，以决定下一步。
    
- 循环：模型不断读取最新上下文，输出下一步指令或结果，直到任务完成。
    

通过这种设计，语言模型变成了一个可以自主决定行动顺序的智能体，而不仅是被动回答。

编程任务：模拟一个简化的Agent循环。为易于展示，我们假设目标是进行简单的算术计算，但Agent本身不会算术，它可以调用一个“计算”函数。我们用一个小的对话式模型来生成操作步骤。（由于我们没有GPT-4，这里用GPT-2模拟，实际效果有限，仅作流程演示。）

```python
from transformers import AutoModelForCausalLM, AutoTokenizer

small_model = AutoModelForCausalLM.from_pretrained("distilgpt2")
small_tokenizer = AutoTokenizer.from_pretrained("distilgpt2")

# 简化的环境：提供一个计算函数供Agent调用
def calculate(expression):
    try:
        return str(eval(expression))
    except Exception as e:
        return f"Error: {e}"

# Agent loop
goal = "Calculate the sum of 42 and 17."
context = f"Goal: {goal}\nYou are an agent. Decide next action.\n"
for step in range(3):
    input_ids = small_tokenizer(context, return_tensors="pt").input_ids
    output_ids = small_model.generate(input_ids, max_length=input_ids.shape[1] + 20, do_sample=False)
    agent_output = small_tokenizer.decode(output_ids[0][input_ids.shape[1]:], skip_special_tokens=True)
    agent_output = agent_output.strip()
    print(f"Agent思考{step+1}: {agent_output}")
    if agent_output.lower().startswith("calculate"):
        expr = agent_output.split("calculate",1)[1]
        result = calculate(expr)
        context += f"Action: {agent_output}\nResult: {result}\n"
        if result.startswith("Error"):
            continue
        else:
            context += f"Final Answer: {result}\n"
            break
    else:
        # 如果Agent未提出计算，直接将其输出视为答案
        context += f"Final Answer: {agent_output}\n"
        break

print("\n对话记录:\n" + context)
```

结果验证：我们为演示目的，让Agent的目标是计算 42+17 的和。但Agent不会直接算，于是期望它学会调用 `calculate` 工具。由于我们没特殊微调模型，小模型未必能按我们设计输出，但可能输出些类似指令。示例输出（实际GPT-2小模型可能不理想，这里假设一种可能）：

```Java
Agent思考1: I need to compute 42 + 17  
Agent思考2: calculate 42+17  
Agent思考3: The result is 59

对话记录:
Goal: Calculate the sum of 42 and 17.
You are an agent. Decide next action.
Action: calculate 42+17
Result: 59
Final Answer: 59
```

这里模拟的Agent在第2步成功调用了计算工具，得到了结果59，并在第3步报告最终答案。AutoGPT 之类的实际Agent会更复杂：比如会将 "I need to compute 42 + 17" 这种自然语言再转成行动；使用日志记录thoughts、使用不同角色提示等。但核心流程类似：模型 -> 工具 -> 模型 ->... -> 答案 的循环。

说明：我们的例子用 GPT-2 生成，实际上需要更强的模型（GPT-4等）和特别的 prompt 设计（例如“思维链”提示，让模型显式输出思考和命令部分分开）。AutoGPT使用了一个固定的提示模板，引导模型以`思考->决定->行动`的格式输出，并能解析执行。本例未严格实现这些格式，只是展示一个循环思路。

Agent 开发将模型推向“自主AI”的方向，但也带来挑战：比如如何防止它误用工具、无限循环，如何约束行为不越界等。这仍是非常前沿的探索领域，充满创新机会。

---

恭喜您走完六周的 Transformer 深度学习实践教程！我们从基础架构一路实践到前沿应用，希望您不仅掌握了Transformer模型的原理和实现，还对最新的发展方向有所了解。通过一个个循序渐进的练习，您已经亲自体验了搭建环境、实现注意力模块、微调模型、文本生成、模型压缩、提示工程以及更多高级技巧。

在实际项目中，您可以组合运用以上所学。例如，微调一个Transformer完成特定任务，利用提示工程提升效果，用RLHF方法让模型输出更符合要求，最后将模型导出ONNX部署到服务器。在多模态和Agent领域，Transformer也大有可为。请持续关注学术和社区的新进展，相信这些技能将为您的深度学习之路打下坚实基础。祝你在实践中不断创新，构建出更厉害的模型和应用！
