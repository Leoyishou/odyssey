---
comment_id: 3d5a8c01
date created: 2025-04-12
date modified: 2025-04-12
draw: null
title: Transformer 模型实践教程— PyTorch Lightning版
---
下面给出 **原教程**（纯 PyTorch / Hugging Face Trainer 方式）与 **Lightning 版**（使用 PyTorch Lightning）在代码结构上的**关键差异**，也就是哪些地方被“简化”或“改动”了。简单来说，Lightning 主要帮我们**“收拢”了训练循环**（forward / backward / 优化器更新 / 日志记录 等），所以凡是原教程里手写或借助其它手段做的地方，都在 Lightning 里被**集中**和**自动**处理了。

---

## 1. 训练循环的替换

**原教程**：

- 人工写 `for epoch in range(epochs):` 循环；
    
- 在循环里做 `model.train()`、`optimizer.zero_grad()`、`loss.backward()`、`optimizer.step()`；
    
- 有时还手动记录 loss、accuracy 等指标，用 `print` 或者手写 logger；
    
- 如果需要验证集/测试集，还要自己写 `with torch.no_grad():` 遍历验证集，然后计算指标。
    

**Lightning 版**：

- 使用 `Trainer.fit(...)`，不再手写 epoch / step 循环；
    
- 将前向逻辑放在 `training_step`，Lightning 会自动调用；
    
- Lightning 会自动帮你做 `.zero_grad()`、`.backward()`、`.step()`；
    
- 通过 `self.log("某指标", 值)` 来记录训练或验证过程中的指标，Lightning 会自动输出并写日志文件；
    
- 在 `validation_step` 里实现验证逻辑，Lightning 同样会自动调用。
    

因此，原教程里**“显式写循环 + 优化过程”**的代码，在 Lightning 里**全部被省略**：

```python
# (原生 PyTorch 示例)
for epoch in range(epochs):
    model.train()
    for batch in train_dataloader:
        optimizer.zero_grad()
        outputs = model(**batch)
        loss = outputs.loss
        loss.backward()
        optimizer.step()
    # validate...
```

在 Lightning 版就只剩下：

```python
def training_step(self, batch, batch_idx):
    outputs = self.model(**batch)
    loss = outputs.loss
    self.log("train_loss", loss)
    return loss
...
trainer.fit(model, train_dataloaders, val_dataloaders)
```

再也没有“手动写循环”与“手动 zero_grad / step”，一切都被 Lightning `Trainer` 管理了。

---

## 2. 优化器、学习率调度等配置的简化

**原教程**：

- 需要手动创建 `optimizer = torch.optim.AdamW(model.parameters(), lr=...)`；
    
- 如果用学习率调度器，还要自己写 `scheduler = get_linear_schedule_with_warmup(...)`，并在每个 epoch/step 后 `scheduler.step()`；
    
- 可能还要在训练日志中记录学习率变化。
    

**Lightning 版**：

- 在 `configure_optimizers` 函数中**一次性**返回优化器和调度器，Lightning 内部会自动在合适的时机调用它们：
    

```python
def configure_optimizers(self):
    optimizer = torch.optim.AdamW(self.parameters(), lr=self.lr)
    scheduler = get_linear_schedule_with_warmup(...)
    return [optimizer], [scheduler]
```

- 不再写手动 `scheduler.step()` 之类的逻辑。
    

---

## 3. 自动日志记录/打印

**原教程**：

- 常见写法：`print(f"epoch={epoch}, loss={loss.item()}")`；
    
- 或者自己集成 TensorBoard / WandB，需要手写 `writer.add_scalar(...)` / `wandb.log(...)`。
    

**Lightning 版**：

- 用 `self.log("metric_name", metric_value, prog_bar=True)`，Lightning 就会把该指标打印到控制台进度条，并可以自动写入日志文件 / TensorBoard / WandB（如果在 `Trainer` 里配好）。
    
- 无需自己写 `print` 或者手动对接 logger。
    

---

## 4. Hugging Face Trainer 被替换

在原教程里，对一些任务（如BERT文本分类、GPT文本生成）我们直接用过 `transformers` 中的 `Trainer` + `TrainingArguments`：

```python
trainer = Trainer(model=model, args=training_args, ...)
trainer.train()
```

在 Lightning 版中，我们**不再使用** `Hugging Face` 的 `Trainer`，而是**将模型封装成 LightningModule** 并调用 `pytorch_lightning.Trainer`：

```python
class BertForClassificationLightning(pl.LightningModule):
    ...

trainer = pl.Trainer(...)
trainer.fit(lightning_module, train_loader, val_loader)
```

两者本质功能类似（都是封装训练循环），只不过**Lightning 也可更灵活地自定义**，并且统一了你自己写的模块/模型与训练过程。

---

## 5. 演示脚本 & 测试部分的简化

**原教程**常见的示例：

1. 做一个 `demo_x = torch.zeros(...)`，
    
2. 调用 `model(demo_x)`，
    
3. 然后再手写 `loss =...`、`loss.backward()` 以测试网络是否能跑通。
    

**Lightning 版**里：

- 我们往往只写一个小的 `training_step`，然后用 `trainer.fit(model,[demo_x])` 做 1 个 epoch，让它自行前向+反向，看是否报错；
    
- 不需要写多余的脚本去验证**是否能 backward**，因为 Lightning 会自动执行并抛出错误或完成训练。
    
- 类似地，若要做推断/测试，可直接写 `test_step` + `trainer.test(...)` 或者自己写一个函数 `model(x)` 并推断就行。
    

所以可以看到，**很多零散的测试脚本**被**Lightning**的训练/验证/测试三个阶段给统一收编了。

---

## 6. 知识蒸馏 & RL 环节中训练逻辑的精简

**原教程**里知识蒸馏时：

- 人工写 loop：先拿 teacher logits → softmax → student logits → KL 散度 → 反向更新。
    
- 还要自己管理 batch、device 等。
    

**Lightning 版**：

- 上述逻辑直接放进 `training_step` 一次搞定；
    
- 传入的 `batch` 由 DataLoader + Trainer 提供，自动在 GPU 上。
    
- 不用额外写循环+device分发+日志，只保留**核心公式**即可。
    

同理，如果要实现**RLHF**或**PPO**，也可以在 `training_step` 里写相应的采样 / 计算策略梯度 / 更新；Lightning 负责 epoch / batch / 分布式 / 日志等通用流程。

---

## 7. 设备管理 / 分布式 / 深度加速配置

**原教程**：

- 如果想要用多 GPU、或分布式，需要自己写 `DistributedDataParallel`、`DataParallel`、或者 DeepSpeed、Horovod 等；
    
- 手动管理 `device = torch.device("cuda" if torch.cuda.is_available() else "cpu")` 并把模型、数据 `.to(device)`；
    
- 可能还要写各种繁琐的分布式初始化代码。
    

**Lightning 版**：

- 只要在 `Trainer(accelerator="gpu", devices=...)` 等指定，就自动管理并行、device 放置、梯度聚合、通信；
    
- 如果想用 DeepSpeed / FSDP / etc.，只要在 `Trainer(strategy="deepspeed_stage_2")` 等启用即可；
    
- 无需自己写 “`model.to(device)`”，Lightning 会自动将模型和数据搬到对应设备。
    

---

## 小结

**被“简化”或“改动”的主要就是：**

1. **去掉了所有手写的训练循环**（包括 epoch、mini-batch、optimizer.step 等），改为 `Trainer.fit()` + `training_step`。
    
2. **去掉了手动日志打印和手动管理 TensorBoard**，改用 `self.log(...)` 由 Lightning 内部管理。
    
3. **去掉了手动管理 GPU / 分布式**，改用 `Trainer(accelerator=..., devices=..., strategy=...)`。
    
4. **去掉了 Hugging Face 的 `Trainer`**（如果原教程里使用它），改成 LightningModule + Lightning 的 `Trainer`。
    
5. **demo/测试脚本**（比如验证网络能否 forward+backward）也用 Lightning 的 fit 或 test 来做演示，而不手写 `for step in range(steps)`。
    

除了上述变化，**模型的原理、网络结构、注意力/FFN 等组件**都**与原教程一致**。你只需把它们写进 LightningModule，剩下的训练与验证流程就不用再手写了。这样代码更统一、更短、也更易扩展到分布式大模型场景。

---

希望这能帮助你快速了解：**Lightning 版 与 原教程版** 的主要“简化”之处究竟在哪里。祝学习顺利！

下面是一份**基于原教程结构**、使用 **PyTorch Lightning** 来组织训练流程的示例版本。为方便理解，本文在保留原教程的主要内容与思路的同时，会重点演示如何在关键步骤中使用 PyTorch Lightning 的 **LightningModule**、**Trainer** 等接口，让训练、验证以及部署流程更加简洁与标准化。由于教程内容非常丰富，以下示例代码不会一字不漏地替换每行原生 PyTorch 代码，但会展示最核心的 Lightning 使用方式。读者可结合此思路，将原教程中的其他部分迁移为 Lightning 风格。

---

## 六周 Transformer 模型实践教程（PyTorch Lightning 版）

|周数 (Week)|目标 (Goal)|
|---|---|
|第一周|搭建环境，了解 Transformer 架构和自注意力机制，完成第一个文本生成示例。|
|第二周|理解 Transformer 核心组件：词嵌入、位置编码、多头注意力、前馈网络等。|
|第三周|组装简化版 Transformer 块，并通过微调预训练模型完成文本分类。|
|第四周|高级应用：文本生成、模型压缩、提示工程与评估指标。|
|第五周|更高级技巧：高效训练策略、模型伦理与公平、多模态模型、优化部署等。|
|第六周|了解最新研究：稀疏化（MoE）、强化学习对齐（RLHF）、自主 AI Agent 原理等。|

---

### ❤️ 第一周：基本概念 (Lightning 版)

#### 步骤1：环境搭建（Python/PyTorch/Hugging Face Transformers + PyTorch Lightning）

与原教程相同，先在 Google Colab 或本地环境中安装必要包：

```bash
!pip install torch torchvision transformers pytorch-lightning --upgrade
```

然后验证环境，示例代码可直接使用原教程中的方式（比如打印版本信息和 `torch.cuda.is_available()`）。

#### 步骤2：使用 Lightning 体验预训练模型的前向传播

下面我们先演示**不**使用 LightningModule，而只是结合 Lightning 的 Trainer 来做一个简单推理，帮助大家熟悉 Lightning 的基本调用方式。在真正写 LightningModule 之前，我们可以像原生 PyTorch 一样加载模型，只是把 “推理” 或 “验证” 的过程托管给 Lightning 的 Trainer。

```python
import torch
import pytorch_lightning as pl
from transformers import AutoTokenizer, AutoModel

# 加载DistilBERT分词器与模型
tokenizer = AutoTokenizer.from_pretrained('distilbert-base-uncased')
model = AutoModel.from_pretrained('distilbert-base-uncased')

text = "Hello world"
inputs = tokenizer(text, return_tensors="pt")

# 由于这里只是做一次前向推理，不定义训练过程，所以可以使用Lightning的LightningModule空壳
class InferenceModule(pl.LightningModule):
    def __init__(self, model):
        super().__init__()
        self.model = model

    def forward(self, **inputs):
        return self.model(**inputs)

inference_module = InferenceModule(model)

# 我们用 Trainer 来做一个测试步骤
trainer = pl.Trainer(
    max_epochs=1,
    logger=False,
    enable_checkpointing=False,
    enable_model_summary=False
)

# 直接手动调用 forward 看输出
outputs = inference_module(**inputs)
print("输出张量维度:", outputs.last_hidden_state.shape)
```

这里因为没有真实的训练或验证集，所以我们用 `Trainer` 没有什么实际意义，但你可以通过这样的方式，日后把训练/验证/测试等过程放到 Lightning 中统一管理。

---

#### 步骤3：自注意力机制（Scaled Dot-Product Attention）

原教程中写了一个简化注意力的函数 `scaled_dot_product_attention(Q, K, V)`. 在 Lightning 中，这段逻辑通常写在 LightningModule 内部或者封装成一个独立的 nn.Module。下面给出一个**独立模块**示例，保留原公式，方便后续被 LightningModule 调用。

```python
import torch
import torch.nn.functional as F

class ScaledDotProductAttention(torch.nn.Module):
    def __init__(self):
        super().__init__()

    def forward(self, Q, K, V):
        d_k = Q.size(-1)
        scores = torch.matmul(Q, K.transpose(-2, -1)) / (d_k ** 0.5)
        weights = F.softmax(scores, dim=-1)
        output = torch.matmul(weights, V)
        return output, weights
```

后续在真正的 LightningModule 中，可以直接实例化并调用 `ScaledDotProductAttention` 来做自注意力计算。

---

#### 步骤4：基础文本生成（Lightning 版）

让我们快速演示一下**文本生成**的 Lightning 用法——实际上，Hugging Face 提供的 `generate()` 方法并不需要 Lightning 的训练循环。但如果你想要在 Lightning 的范式下做**训练+生成**，则可以把生成逻辑写在 `validation_step` 或 `test_step` 里。下面演示一个最基本的**LightningModule + Trainer**做文本生成推理：

```python
from transformers import AutoModelForCausalLM, AutoTokenizer
import pytorch_lightning as pl

class GPT2InferenceModule(pl.LightningModule):
    def __init__(self, model_name="distilgpt2"):
        super().__init__()
        self.save_hyperparameters()
        self.tokenizer = AutoTokenizer.from_pretrained(model_name)
        self.model = AutoModelForCausalLM.from_pretrained(model_name)

    def forward(self, input_ids, max_length=30):
        # 仅做生成
        generated_ids = self.model.generate(
            input_ids=input_ids, max_length=max_length
        )
        return generated_ids

# 初始化
gpt2_module = GPT2InferenceModule("distilgpt2")
trainer = pl.Trainer(logger=False, enable_checkpointing=False)

# 推断
prompt = "Hello, my name is"
input_ids = gpt2_module.tokenizer(prompt, return_tensors='pt').input_ids
generated_ids = gpt2_module(input_ids)
output_text = gpt2_module.tokenizer.decode(generated_ids[0], skip_special_tokens=True)
print("生成文本:", output_text)
```

这样虽然有点“多此一举”，但展示了可以用 LightningModule 包装预训练模型，便于后续扩展训练环节。

---

### ❤️ 第二周：核心组件 (Lightning 版)

下面展示如何把**词嵌入、位置编码、多头注意力、前馈网络**等核心组件封装进一个 LightningModule 或者普通的 PyTorch Module。然后 LightningModule 在 `training_step` 中调用这些模块进行前向传播，并依赖 Trainer 的循环机制来完成训练。

#### 步骤5：Word Embedding（词嵌入）+ Lightning 用法

Lightning 对“组件”层面没有特别要求，与普通 PyTorch nn.Module 相同。只要在 `LightningModule` 的 `forward` 内部正确调用即可。示例：

```python
import torch
import torch.nn as nn
import pytorch_lightning as pl

class EmbeddingDemo(pl.LightningModule):
    def __init__(self, vocab_size=10, embed_dim=4):
        super().__init__()
        self.embedding = nn.Embedding(num_embeddings=vocab_size, embedding_dim=embed_dim)
        # 为了让Lightning能训练，需要一个优化器，哪怕只是演示
        self.lr = 1e-3

    def forward(self, x):
        return self.embedding(x)

    def configure_optimizers(self):
        return torch.optim.Adam(self.parameters(), lr=self.lr)

    def training_step(self, batch, batch_idx):
        # 假设 batch 就是单词索引
        embedded = self.forward(batch)
        loss = embedded.mean()  # 随便定义一个loss演示
        self.log("train_loss", loss)
        return loss

# 演示：训练一个 epoch
demo_model = EmbeddingDemo()
trainer = pl.Trainer(max_epochs=1, logger=False, enable_checkpointing=False)
fake_data = torch.randint(0, 10, (16,))  # 假设batch=16
trainer.fit(demo_model, train_dataloaders=[fake_data])
```

这里 `fake_data` 只是一个纯数字张量充当索引，实际中你会用 DataLoader 产出真正的 token id。Lightning 会自动帮你做前向、反向和优化步骤。

---

#### 步骤6：Positional Encoding（位置编码）

可以依旧定义一个独立的 `PositionalEncoding` 模块，然后在 LightningModule 中初始化和调用：

```python
import math

class PositionalEncoding(nn.Module):
    def __init__(self, d_model, max_len=5000):
        super().__init__()
        pe = torch.zeros(max_len, d_model)
        position = torch.arange(0, max_len, dtype=torch.float).unsqueeze(1)
        div_term = torch.exp(torch.arange(0, d_model, 2).float() * (-math.log(10000.0) / d_model))
        pe[:, 0::2] = torch.sin(position * div_term)
        pe[:, 1::2] = torch.cos(position * div_term)
        pe = pe.unsqueeze(0)  # shape [1, max_len, d_model]
        self.register_buffer('pe', pe)

    def forward(self, x):
        # x shape: [batch_size, seq_len, d_model]
        seq_len = x.size(1)
        x = x + self.pe[:, :seq_len, :]
        return x
```

使用时只需在 LightningModule 的 `forward` 中调用：

```python
class TransformerEmbeddingsLightning(pl.LightningModule):
    def __init__(self, vocab_size=100, d_model=32, max_len=128):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, d_model)
        self.pos_encoding = PositionalEncoding(d_model, max_len)
        self.lr = 1e-3

    def forward(self, x):
        embed = self.embedding(x)
        out = self.pos_encoding(embed)
        return out

    def configure_optimizers(self):
        return torch.optim.Adam(self.parameters(), lr=self.lr)

    def training_step(self, batch, batch_idx):
        out = self(batch)
        # 伪造一个loss
        loss = out.mean()
        self.log("train_loss", loss)
        return loss
```

---

#### 步骤7：Multi-Head Attention（多头注意力）+ Lightning

在 Lightning 中使用多头注意力，与普通 PyTorch 几乎相同，只是在训练循环中更简洁。我们可以直接用 `nn.MultiheadAttention` 或自己实现都行。下面演示**自定义**一个 LightningModule，包含多头注意力进行前向。训练数据依然是伪造的。

```python
class MultiHeadAttentionDemo(pl.LightningModule):
    def __init__(self, embed_dim=8, num_heads=2):
        super().__init__()
        self.mha = nn.MultiheadAttention(embed_dim, num_heads, batch_first=True)
        self.lr = 1e-3

    def forward(self, x):
        # x shape: [batch, seq_len, embed_dim]
        attn_out, attn_weights = self.mha(x, x, x)
        return attn_out, attn_weights

    def configure_optimizers(self):
        return torch.optim.Adam(self.parameters(), lr=self.lr)

    def training_step(self, batch, batch_idx):
        x = batch
        out, w = self(x)
        loss = out.mean()
        self.log("loss", loss)
        return loss

# 伪造一个 x: batch=2, seq_len=5, embed_dim=8
dummy_x = torch.randn(2, 5, 8)
demo_mha = MultiHeadAttentionDemo()
trainer = pl.Trainer(max_epochs=1, logger=False, enable_checkpointing=False)
trainer.fit(demo_mha, train_dataloaders=[dummy_x])
```

---

#### 步骤8：Feed-Forward 模块（前馈网络）+ Lightning

同理，前馈网络可以是一个普通 `nn.Sequential`，在 LightningModule 中做前向和训练：

```python
class FFNLightning(pl.LightningModule):
    def __init__(self, embed_dim=8, hidden_dim=16):
        super().__init__()
        self.ffn = nn.Sequential(
            nn.Linear(embed_dim, hidden_dim),
            nn.ReLU(),
            nn.Linear(hidden_dim, embed_dim)
        )
        self.lr = 1e-3

    def forward(self, x):
        return self.ffn(x)

    def configure_optimizers(self):
        return torch.optim.Adam(self.parameters(), lr=self.lr)

    def training_step(self, batch, batch_idx):
        out = self(batch)
        loss = out.mean()
        self.log("loss", loss)
        return loss

dummy_x = torch.randn(2, 5, 8)
ffn_module = FFNLightning()
trainer = pl.Trainer(max_epochs=1, logger=False, enable_checkpointing=False)
trainer.fit(ffn_module, train_dataloaders=[dummy_x])
```

---

### ❤️ 第三周：实践入门 (Lightning 版)

#### 步骤9：实现简化版 TransformerBlock

结合前面组件：多头自注意力 + 前馈网络 + LayerNorm + 残差，做成一个 **nn.Module**，再被 LightningModule 调用。

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
        self.norm1 = nn.LayerNorm(d_model)
        self.norm2 = nn.LayerNorm(d_model)

    def forward(self, x):
        attn_out, _ = self.self_attn(x, x, x)
        x = self.norm1(x + attn_out)
        ffn_out = self.ffn(x)
        x = self.norm2(x + ffn_out)
        return x

# 将其封装到 LightningModule 中，以便训练
class SimpleTransformerLightning(pl.LightningModule):
    def __init__(self, d_model=4, nhead=1, dim_ff=4):
        super().__init__()
        self.block = TransformerBlock(d_model, nhead, dim_ff)
        self.lr = 1e-3

    def forward(self, x):
        return self.block(x)

    def configure_optimizers(self):
        return torch.optim.Adam(self.parameters(), lr=self.lr)

    def training_step(self, batch, batch_idx):
        out = self(batch)
        loss = out.mean()
        self.log("loss", loss)
        return loss

# 测试
demo = SimpleTransformerLightning()
dummy_x = torch.zeros(2,3,4)
trainer = pl.Trainer(max_epochs=1, logger=False, enable_checkpointing=False)
trainer.fit(demo, train_dataloaders=[dummy_x])
```

此时就能复现原教程中对 TransformerBlock 的测试方式，只不过我们把训练循环用 Lightning 接管了。

---

#### 步骤10 & 11：微调预训练模型（BERT）+ 文本分类 (Lightning 版)

重点来了：使用 Lightning 来微调 BERT。我们可以通过 `transformers` 提供的 `AutoModelForSequenceClassification` 进行初始化，然后在 LightningModule 中实现 `forward`、`training_step`、`validation_step` 等。

1. **准备数据**：与原教程一样使用 `datasets` 加载 IMDB，tokenize 后得到 DataLoader。
    
2. **LightningModule**：包装预训练模型 + 优化器 + 训练/验证逻辑。
    

示例如下：

```python
!pip install datasets evaluate

import torch
import torch.nn as nn
import torch.nn.functional as F
import pytorch_lightning as pl

from datasets import load_dataset
from transformers import AutoTokenizer, AutoModelForSequenceClassification

# 加载IMDB子集
dataset_train = load_dataset("imdb", split="train[:2000]")
dataset_val = load_dataset("imdb", split="test[:500]")

model_name = "bert-base-uncased"
tokenizer = AutoTokenizer.from_pretrained(model_name)

def tokenize_batch(batch):
    return tokenizer(batch["text"], padding=True, truncation=True, max_length=128)

dataset_train = dataset_train.map(tokenize_batch, batched=True)
dataset_val = dataset_val.map(tokenize_batch, batched=True)

dataset_train = dataset_train.remove_columns(["text"])
dataset_val = dataset_val.remove_columns(["text"])
dataset_train.set_format("torch")
dataset_val.set_format("torch")

# 建立 DataLoader
def collate_fn(batch):
    return {
        "input_ids": torch.stack([x["input_ids"] for x in batch]),
        "attention_mask": torch.stack([x["attention_mask"] for x in batch]),
        "labels": torch.tensor([x["label"] for x in batch])
    }

train_loader = torch.utils.data.DataLoader(dataset_train, batch_size=16, shuffle=True, collate_fn=collate_fn)
val_loader = torch.utils.data.DataLoader(dataset_val, batch_size=16, shuffle=False, collate_fn=collate_fn)


# LightningModule 封装微调逻辑
class BertForClassificationLightning(pl.LightningModule):
    def __init__(self, model_name="bert-base-uncased", num_labels=2, lr=2e-5):
        super().__init__()
        self.save_hyperparameters()
        self.model = AutoModelForSequenceClassification.from_pretrained(model_name, num_labels=num_labels)

    def forward(self, input_ids, attention_mask, labels=None):
        return self.model(input_ids=input_ids, attention_mask=attention_mask, labels=labels)

    def training_step(self, batch, batch_idx):
        outputs = self(
            input_ids=batch["input_ids"],
            attention_mask=batch["attention_mask"],
            labels=batch["labels"]
        )
        loss = outputs.loss
        self.log("train_loss", loss, prog_bar=True)
        return loss

    def validation_step(self, batch, batch_idx):
        outputs = self(
            input_ids=batch["input_ids"],
            attention_mask=batch["attention_mask"],
            labels=batch["labels"]
        )
        val_loss = outputs.loss
        preds = torch.argmax(outputs.logits, dim=1)
        acc = (preds == batch["labels"]).float().mean()
        self.log("val_loss", val_loss, prog_bar=True)
        self.log("val_acc", acc, prog_bar=True)
        return val_loss

    def configure_optimizers(self):
        return torch.optim.AdamW(self.parameters(), lr=self.hparams.lr)

# 训练
model_light = BertForClassificationLightning(model_name, num_labels=2, lr=2e-5)
trainer = pl.Trainer(max_epochs=1, accelerator="gpu" if torch.cuda.is_available() else "cpu")
trainer.fit(model_light, train_dataloaders=train_loader, val_dataloaders=val_loader)
```

这样就完成了**基于 Lightning** 的 BERT 分类微调。Lightning 的好处在于，如果你要加上 `learning_rate_scheduler`、`loggers`、`checkpoint` 等，都可以在 `Trainer` 中灵活配置，而不用手写太多循环代码。

**推理**也很简单，可以在 `validation_epoch_end` 或 `predict` 模式中进行，或者自己写个函数手动调用：

```python
def predict_sentiment(model, text: str):
    model.eval()
    inputs = tokenizer(text, return_tensors="pt")
    with torch.no_grad():
        outputs = model(input_ids=inputs["input_ids"], attention_mask=inputs["attention_mask"])
        logits = outputs.logits
        pred = torch.argmax(logits, dim=1).item()
    return "Positive" if pred == 1 else "Negative"

print(predict_sentiment(model_light, "This movie is fantastic, I love it!"))
```

---

### ❤️ 第四周：进阶应用 (Lightning 版)

在这一周，我们学习了**文本生成**、**知识蒸馏**、**提示工程**、**评估指标**等。在 Lightning 里，它们的核心原理并无变化，只是将训练循环迁移到 Lightning 中更简洁。下面挑**知识蒸馏**做个 Lightning 展示。

#### 知识蒸馏（Knowledge Distillation）+ Lightning

思路：**Teacher**（已经训练好的大模型）在推理模式下，给每个样本产生 logits → softmax → 作为 soft label；**Student** 用 KL 散度学习逼近 Teacher 的输出分布。

```python
import torch.nn.functional as F
import pytorch_lightning as pl

class DistillationLightning(pl.LightningModule):
    def __init__(self, teacher_model, student_model, lr=3e-4):
        super().__init__()
        self.teacher = teacher_model.eval()  # 冻结教师
        for p in self.teacher.parameters():
            p.requires_grad = False
        self.student = student_model
        self.lr = lr

    def forward(self, **inputs):
        # 学生前向
        return self.student(**inputs)

    def training_step(self, batch, batch_idx):
        # 1) 教师输出
        with torch.no_grad():
            teacher_out = self.teacher(
                input_ids=batch["input_ids"],
                attention_mask=batch["attention_mask"]
            )
            teacher_probs = F.softmax(teacher_out.logits, dim=-1)

        # 2) 学生输出
        student_out = self.student(
            input_ids=batch["input_ids"],
            attention_mask=batch["attention_mask"]
        )
        student_log_probs = F.log_softmax(student_out.logits, dim=-1)

        # 3) KL 散度
        loss = F.kl_div(student_log_probs, teacher_probs, reduction='batchmean')
        self.log("distill_loss", loss)
        return loss

    def configure_optimizers(self):
        return torch.optim.Adam(self.student.parameters(), lr=self.lr)

# 演示：teacher 用之前微调好的 model_light， student 用小模型 DistilBERT
from transformers import AutoModelForSequenceClassification
student_model = AutoModelForSequenceClassification.from_pretrained("distilbert-base-uncased", num_labels=2)

distill_module = DistillationLightning(model_light.model, student_model)
trainer = pl.Trainer(max_epochs=1)
trainer.fit(distill_module, train_loader)
```

---

### ❤️ 第五周：高级主题 (Lightning 版)

**分布式训练 (DeepSpeed / FSDP / ZeRO)**、**模型伦理**、**多模态（CLIP, DALL·E）**、**部署(ONNX/TensorRT)** 等，都可以按照 Lightning 方式来处理训练或推理循环。比如：

- **DeepSpeed + Lightning**: 只要在 `Trainer` 中指定 `strategy="deepspeed_stage_2"` 或者传递配置文件，即可把LightningModule的训练过程托管到 DeepSpeed。示例：
    
    ```python
    trainer = pl.Trainer(
        max_epochs=3,
        strategy="deepspeed_stage_2",
        devices=2,
        accelerator="gpu"
    )
    trainer.fit(model_light, train_dataloaders=train_loader, val_dataloaders=val_loader)
    ```
    
- **多模态**：跟前面类似，把 CLIP 或 Stable Diffusion 的模型写在 `LightningModule` 里，或者直接做推断。
    
- **ONNX 导出**：LightningModule 也只是 PyTorch 模型，你可以和原生 PyTorch 一样使用 `torch.onnx.export()`，只要把 `self.model` 取出来即可。
    

---

### ❤️ 第六周：前沿探索 (Lightning 版)

**MoE 架构**、**RLHF**、**Agent** 思路与本质实现也不冲突。LightningModule 可帮助你管理大规模训练、分布式等，但核心原理与原教程一致。比如：

- **MoE**：可在 LightningModule 构造函数中初始化多个专家子网络，forward 中写好门控逻辑，训练时定义好 `training_step`。
    
- **RLHF**：可把 **reward model** / **策略模型** / **算法** 整合在一个 LightningModule 或多个 LightningModule 中，更好地管理超参和日志。
    
- **Agent**：如果需要循环式地调用大模型、外部工具，可在 `validation_step` 或自定义的推理脚本中完成 agent 的思维过程；Lightning 主要帮助你管理模型的权重和训练流程，agent 的多轮 loop 需要自己写 Python 逻辑或借助 [Lightning Flow](https://lightning.ai/) 之类的生态，但那是更高层的工作流管理。
    

---

## 总结

通过以上示例，可以看到：

- **LightningModule** 主要解决了**训练循环**（前向、反向、优化器更新、日志记录等）的简化问题，与原教程中的**模型原理**、**注意力计算**、**微调思路**等完全兼容。
    
- 你可以把原教程中任何基于 `nn.Module` + 手写 `for epoch in range(epochs)` 的代码改写成 LightningModule + Trainer.fit() 的方式，从而获得更简洁的代码、自动断点续训、自动日志、分布式训练等优势。
    
- 其余原教程提到的**推理、评估、提示工程**等环节，不一定要用 Lightning，但如果你喜欢统一风格，也可以把推理过程写进 `test_step` 或专门函数里。
    

希望以上**PyTorch Lightning 版本**的示例，能帮助你在保留 Transformer 教程核心知识与实践流程的同时，享受到 Lightning 带来的更干净、更便捷的训练体验。祝你学有所成，玩转 Lightning 下的各种 Transformer 应用！
