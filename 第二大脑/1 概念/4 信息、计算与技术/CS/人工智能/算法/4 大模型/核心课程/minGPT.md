---
uid: a588c176-ff5a-4835-bb93-efa582beaf53
date created: 2025-07-19
date modified: 2025-10-13
---
# Summary

[[Andrej Karpathy]] 2019 年发布，用最小代码（几百行）展示 GPT 的核心机制（Transformer + 自回归语言建模）。

 1. 本质：GPT is not a complicated model and this implementation is appropriately about 300 lines of code (see `mingpt/model.py`)
 2. 就是 Transformer：**All that's going on** is that a sequence of indices feeds into a Transformer, and a probability distribution over the next index in the sequence comes out.      `def forward(self, idx, targets=None):` 中体现
 3. 高效batching是重点：The majority of the complexity is just being clever with batching (both across examples and over sequence length) for efficiency.
 
# Cues

[[Transformer]]
[[自回归]]
[[temperature]]
[[随机种子]]
# Notes

- [一、项目结构](#%E4%B8%80%E3%80%81%E9%A1%B9%E7%9B%AE%E7%BB%93%E6%9E%84)
- [二、本质是 Transformer](#%E4%BA%8C%E3%80%81%E6%9C%AC%E8%B4%A8%E6%98%AF%20Transformer)
	- [1.1 Temperature](#1.1%20Temperature)
	- [1.2 top_k采样](#1.2%20top_k%E9%87%87%E6%A0%B7)
- [三、训练过程](#%E4%B8%89%E3%80%81%E8%AE%AD%E7%BB%83%E8%BF%87%E7%A8%8B)
	- [3.1 训练参数](#3.1%20%E8%AE%AD%E7%BB%83%E5%8F%82%E6%95%B0)
	- [3.2 训练过程](#3.2%20%E8%AE%AD%E7%BB%83%E8%BF%87%E7%A8%8B)
	- [3.3 正则化与优化器配置](#3.3%20%E6%AD%A3%E5%88%99%E5%8C%96%E4%B8%8E%E4%BC%98%E5%8C%96%E5%99%A8%E9%85%8D%E7%BD%AE)
		- [效果](#%E6%95%88%E6%9E%9C)
- [四、实践](#%E5%9B%9B%E3%80%81%E5%AE%9E%E8%B7%B5)
	- [4.1 示例感受](#4.1%20%E7%A4%BA%E4%BE%8B%E6%84%9F%E5%8F%97)
		- [任务类型](#%E4%BB%BB%E5%8A%A1%E7%B1%BB%E5%9E%8B)
		- [训练结果](#%E8%AE%AD%E7%BB%83%E7%BB%93%E6%9E%9C)
		- [损失变化（Loss）](#%E6%8D%9F%E5%A4%B1%E5%8F%98%E5%8C%96%EF%BC%88Loss%EF%BC%89)
		- [准确率](#%E5%87%86%E7%A1%AE%E7%8E%87)
		- [验证](#%E9%AA%8C%E8%AF%81)
	- [4.2 实践项目 (动手训练模型)](#4.2%20%E5%AE%9E%E8%B7%B5%E9%A1%B9%E7%9B%AE%C2%A0(%E5%8A%A8%E6%89%8B%E8%AE%AD%E7%BB%83%E6%A8%A1%E5%9E%8B))
		- [4.2.1 加法器任务`adder.py`](#4.2.1%20%E5%8A%A0%E6%B3%95%E5%99%A8%E4%BB%BB%E5%8A%A1%60adder.py%60)
		- [4.2.2 文本生成任务 `chargpt`](#4.2.2%20%E6%96%87%E6%9C%AC%E7%94%9F%E6%88%90%E4%BB%BB%E5%8A%A1%20%60chargpt%60)
- [五、待尝试](#%E4%BA%94%E3%80%81%E5%BE%85%E5%B0%9D%E8%AF%95)

## 一、项目结构

1. GPT 模型训练和推理 - 支持从头训练或加载预训练模型
2. 多种模型规格 - 从 nano (3层) 到 xl (48层)
3. 字符级和 [[BPE]] 分词 - 支持文本生成任务

| 优先级   | 文件                        | 作用                                                                                                                   |
| ----- | ------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| 🔴 阅读 | `model.py`                | GPT 模型定义                                                                                                             |
| 🔴 阅读 | `trainer.py`              | 训练流程<br>- 训练循环的标准流程<br>- [[优化器]] 配置（权重衰减策略）<br>- Callback 机制                                               |
| 🟢 阅读 | `bpe.py`                  | 文本编码<br>- [[BPE]] (Byte Pair Encoding) 如何将文本转换为 token 序列<br>- 这是 GPT-2 使用的同款编码器，<br>    GPT-2 使用 50,257 个 token 的词汇表 |
| 🟡 阅读 | `utils.py`                | 配置管理<br>- `CfgNode` [[超参数]]配置系统（灵感来自 [[YACS]]）<br>- 如何从命令行或字典更新配置                                                    |
| 🟡 动手 | `projects/adder/adder.py` | 完整训练示例                                                                                                               |
|       |                           |                                                                                                                      |

## 二、本质是 Transformer

**All that's going on** is that a sequence of indices feeds into a Transformer, and a probability distribution over the next index in the sequence comes out.      `def forward(self, idx, targets=None):` 中体现

```Java
输入 → Norm → Multi-Head Attention → Add → Norm → FFN → Add → 输出
```


1. `idx` 就是输入的索引序列，形状是 `(batch_size, sequence_length)`
2. Token Embedding: 将索引转换为向量
3. Position Embedding: 添加位置信息
4. Transformer Blocks: 多层自注意力和前馈网络处理
	1. 都是 attn 和 mlpf 都是preNorm[[正则化]]
5. `lm_head` 将隐藏状态映射到词汇表大小, 得到 logits，表示每个位置对下一个 token 的未归一化概率
6. 归一化后得到真正的概率分布
7. 从分布中采样得到下一个索引 `idx_next`
8. 将新索引拼接回序列，[[自回归]]继续生成

```shell

GPT类
├── 初始化: 构建 Transformer 架构
│   ├── wte: 词嵌入层
│   ├── wpe: 位置嵌入层
│   ├── Block x N层: Transformer块
│   │   ├── LayerNorm1 $\rightarrow$ 自注意力 (CausalSelfAttention)
│   │   └── LayerNorm2 $\rightarrow$ 前馈网络 (MLP)
│   └── lm_head: 输出投影层
│
├── forward(): 前向传播
│   ├── 输入token $\rightarrow$ 词嵌入 + 位置嵌入
│   ├── 通过N个Transformer块
│   └── 输出logits (+ 可选loss计算)
│
└── generate(): 文本生成
    └── 自回归预测下一个token
    
    
def forward(self, idx, targets=None):
    # 1. idx 就是输入的索引序列，形状是 (batch_size, sequence_length)
	device = idx.device
	b, t = idx.size()
	pos = torch.arange(0, t, dtype=torch.long, device=device).unsqueeze(0) # shape (1, t)

	# forward the GPT model itself
	tok_emb = self.transformer.wte(idx) # 2. Token Embedding: 将索引转换为 Embedding
	pos_emb = self.transformer.wpe(pos) # 3. Position Embedding: 添加位置信息 Embedding
	x = self.transformer.drop(tok_emb + pos_emb)
	for block in self.transformer.h:  # 4. Transformer Blocks: 多层自注意力和前馈网络处理
		x = x + self.attn(self.ln_1(x))  # pre Norm
		x = x + self.mlpf(self.ln_2(x))
	x = self.transformer.ln_f(x)
	# 5. lm_head 将隐藏状态映射到词汇表大小, 得到 logits，表示每个位置对下一个 token 的未归一化概率
	logits = self.lm_head(x)         

	loss = None
	if targets is not None:
		loss = F.cross_entropy(logits.view(-1, logits.size(-1)), targets.view(-1), ignore_index=-1)
	return logits, loss
	
	
@torch.no_grad()
def generate(self, idx, max_new_tokens, temperature=1.0, do_sample=False, top_k=None):
	for _ in range(max_new_tokens):
		idx_cond = idx if idx.size(1) <= self.block_size else idx[:, -self.block_size:]
		logits, _ = self(idx_cond)
		logits = logits[:, -1, :] / temperature
		if top_k is not None:
			v, _ = torch.topk(logits, top_k)
			logits[logits < v[:, [-1]]] = -float('Inf')
		# 6. 归一化后得到真正的概率分布
		probs = F.softmax(logits, dim=-1)
		# 7. 从分布中采样得到下一个索引 idx_next
		if do_sample:
			idx_next = torch.multinomial(probs, num_samples=1)
		else:
			_, idx_next = torch.topk(probs, k=1, dim=-1)
		# 8. 将新索引拼接回序列，自回归继续生成
		idx = torch.cat((idx, idx_next), dim=1)

	return idx
```

### 1.1 LLM decoding

这里的`generate`函数比较简单，进一步可以看[[Transformers]]库的generate函数的所有参数的作用（了解了就知道推理的各种方法了，什么[[temperature]]，[[重复性惩罚]]，top-k, top-p, [[Beam search]], group_beam_search, 避免[[N-gram]]重复），有助于深入理解[[OpenAI API]]

#### Temperature

Temperature 通过缩放 logits 来控制概率分布的"尖锐度"：
`logits = logits[:, -1,:]/ temperature`
1. temperature = 1.0（默认值）
	- 不改变 logits，保持原始概率分布
	- 平衡的随机性
2. temperature < 1.0（例如 0.5）
	- logits 被放大（除以小于1的数）
	- 经过 softmax 后，概率分布更加尖锐/集中
	- 模型更倾向选择最可能的 token
	- 输出更确定、保守、重复性高
3. temperature > 1.0（例如 1.5）
	- logits 被缩小（除以大于1的数）
	- 经过 softmax 后，概率分布更加平滑/均匀
	- 模型会考虑更多可能性较低的 token
	- 输出更多样、创造性、但可能不连贯

#### top_k采样
1. 是一种采样策略，相当于从 n 个 logits 候选人里截取了 k 个
2. 另一种是 top_p - [[核采样]]（0-1），默认 1.0，类似 top_k，但基于累积概率。例如 top_p=0.9 表示只考虑累积概率达到 90% 的 token。[[OpenAI API]]

回顾 `model.py` 中的这些部分：
- **Line 29-71**: `CausalSelfAttention`
  - 注意 `masked_fill` 如何实现因果掩码
  - Multi-head 的拆分和合并
  
- **Line 215-258**: `configure_optimizers`
  - Weight Decay 的选择性应用策略
  - 哪些参数需要正则化，哪些不需要

## 三、训练过程

`trainer.py` 是一个通用的训练框架，与 GPT 本身无关，可以用于任何 PyTorch 模型。

[[BPE]] `bpe.py`

### 3.1 训练参数

```@staticmethod
def get_default_config():
    C = CN()
    C.device = 'auto'           # GPU or CPU
    C.num_workers = 4           # 数据加载的进程数
    C.max_iters = None          # 最大训练步数，训练循环由 max_iters 控制，而不是 epoch 数
							      这是 GPT 训练的常见做法（不强调 "epoch" 概念）
								  实现无限数据流，不依赖 epoch 概念
    C.batch_size = 64           # 批次大小，背一本书，一次背几页
    C.learning_rate = 3e-4      # 学习率（GPT-3 论文中的值），在山上每一步跨多大
    C.betas = (0.9, 0.95)       # Adam 优化器的 beta 参数     [[优化器 optimizer]]
    C.weight_decay = 0.1        # 权重衰减（GPT-3 风格）
    C.grad_norm_clip = 1.0      # 梯度裁剪阈值      [[梯度范数]]
    return C
```


#### 正则化

##### Pre Norm
[[正则化]]
```python
class Block(nn.Module):
    """ an unassuming Transformer block """
    def forward(self, x):
        x = x + self.attn(self.ln_1(x))
        x = x + self.mlpf(self.ln_2(x))
        return x
```

##### 优化器配置

问题：模型在训练集上表现很好，但在测试集上表现差（过拟合）
解决方案：[[正则化]] = 限制模型复杂度，防止过拟合
1. [[权重衰减]]是一种正则化方法，它惩罚过大的权重值。
2. 哪些层需要正则化，哪些层不需要？
	1. Bias通常不正则化，它只是一个偏移量（标量或向量），数量少，影响小，不会导致过拟合。
	2. Linear 层是模型的主要学习参数，数量多、维度高，容易过拟合。
	3. LayerNorm 的 weight 和 bias 是缩放和平移参数，它们的作用是归一化，不是学习特征，通常接近1和 0，不会导致过拟合。
	4. Embedding是查找表，不是参数化的变换，每个token 对应一个向量。Weight decay 会让所有embedding 向量缩小，破坏表示

```shell
def configure_optimizers(self, train_config):
	decay = set()
	no_decay = set()
	whitelist_weight_modules = (torch.nn.Linear, )
	blacklist_weight_modules = (torch.nn.LayerNorm, torch.nn.Embedding)
	for mn, m in self.named_modules():
		for pn, p in m.named_parameters():
			fpn = '%s.%s' % (mn, pn) if mn else pn 
			if pn.endswith('bias'):
				# 1. Bias通常不正则化，它只是一个偏移量（标量或向量），数量少，影响小，不会导致过拟合
				no_decay.add(fpn)
			elif pn.endswith('weight') and isinstance(m, whitelist_weight_modules):
				# 2. 这些是模型的主要学习参数数量多、维度高，容易过拟合
				decay.add(fpn)
			elif pn.endswith('weight') and isinstance(m, blacklist_weight_modules):
				# 3. LayerNorm 的 weight 和 bias 是缩放和平移参数 
				#    它们的作用是归一化，不是学习特征，通常接近1和 0，不会导致过拟合
				# 4. Embedding是查找表，不是参数化的变换 每个token 对应一个向量
				#     Weight decay 会让所有embedding 向量缩小，破坏表示
				no_decay.add(fpn)
				
	optimizer = torch.optim.AdamW(optim_groups, lr=train_config.learning_rate, betas=train_config.betas)
	return optimizer
```

效果
```Java
# 假设模型结构如下
model = nn.Sequential(
    nn.Linear(10, 20),      # 有 weight 和 bias
    nn.LayerNorm(20),       # 有 weight 和 bias
    nn.Linear(20, 5),       # 有 weight 和 bias
)

# 运行 configure_optimizers 后的分组：

decay = {
    '0.weight',    # Linear 的 weight ✅
    '2.weight',    # Linear 的 weight ✅
}

no_decay = {
    '0.bias',      # Linear 的 bias ❌
    '1.weight',    # LayerNorm 的 weight ❌（特殊！）
    '1.bias',      # LayerNorm 的 bias ❌
    '2.bias',      # Linear 的 bias ❌
}
```


### 3.2 训练过程

1. 拿到1个 batch 的训练数据，放到 GPU 中
2. 前向传播
3. 反向传播，更新参数
4. 回调一些动作
5. 终止条件

``` shell
def run(self):
	model, config = self.model, self.config

	# setup the optimizer
	self.optimizer = model.configure_optimizers(config)

	# setup the dataloader
	train_loader = DataLoader(
		self.train_dataset,
		sampler=torch.utils.data.RandomSampler(self.train_dataset, replacement=True, num_samples=int(1e10)),
		shuffle=False,
		pin_memory=True,
		batch_size=config.batch_size,
		num_workers=config.num_workers,
	)

	model.train()
	self.iter_num = 0
	self.iter_time = time.time()
	data_iter = iter(train_loader)
	while True:

		# 1. 拿到1个 batch 的训练数据，放到 GPU 中
		try:
			batch = next(data_iter)
		except StopIteration:
			data_iter = iter(train_loader)
			batch = next(data_iter)
		batch = [t.to(self.device) for t in batch]
		x, y = batch

		# 2. 前向传播
		logits, self.loss = model(x, y)

		# 3. 反向传播，更新参数
		model.zero_grad(set_to_none=True)
		self.loss.backward()
		## 梯度裁剪，限制梯度的全局范数不超过 1.0，GPT-3 论文的标准做法
		torch.nn.utils.clip_grad_norm_(model.parameters(), config.grad_norm_clip)
		self.optimizer.step()
		
		# 4. 回调一些动作
		self.trigger_callbacks('on_batch_end')
		self.iter_num += 1
		tnow = time.time()
		self.iter_dt = tnow - self.iter_time
		self.iter_time = tnow

		#  5. 终止条件
		if config.max_iters is not None and self.iter_num >= config.max_iters:
			break
```


回调就是[[观察者模式]]：
1. 训练日志记录：不仅记录 loss，还要记录时间、学习率等，[[wandb]]
2. 模型验证与评估：对于生成任务，定期采样查看质量，定期检查模型在验证集上的表现
3. 保存表现最好的模型，防止过拟合：保存[[checkpoint]]时包含 optimizer 状态
4. 记录 iter_num 以便续训


## 四、实践


### 4.1 直观例子

| 文件名                  | 描述                                          |
|:------------------- |:------------------------------------------ |
| `simple_generate.py` | 只是在一个随机参数的模型上演示推理流程（不训练），会生成乱码，**证明训练的重要性** |
| `generate_script.py` | 加载预训练 GPT-2 模型并生成文本，类似于调用一个开源的 Qwen         |
| `demo_script.py`     | 最简单的排序任务示例，自己感受完整的训练流程                      |

#### 任务类型

排序任务`demo_script.py`
```Java
input sequence  : [[0, 0, 2, 1, 0, 1]]      ← 输入的原始序列
predicted sorted: [[0, 0, 0, 1, 1, 2]]      ← 模型预测的排序结果
gt sort         : [0, 0, 0, 1, 1, 2]        ← 真实的正确答案（Ground Truth）
matches         : True                       ← 预测是否与真值匹配
```

#### 训练结果

- 只用了 10000 个样本
- 只训练了 2000 次迭代（~1 分钟）
- 模型只有 0.09M 参数（相比 GPT-2 的 124M）

```shell
(venv) (base) liuyishou@MacBook-Pro-3 minGPT % python demo_script.py
Creating datasets...

Example data point:
1 -1
1 -1
0 -1
2 -1
0 -1
1 0
0 0
0 1
1 1
1 1
1 2

Creating model...
number of parameters: 0.09M

Setting up trainer...
running on device cpu

Starting training...
/Users/liuyishou/usr/projects/learn/minGPT/venv/lib/python3.12/site-packages/torch/utils/data/dataloader.py:684: UserWarning: 'pin_memory' argument is set as true but not supported on MPS now, then device pinned memory won't be used.
  warnings.warn(warn_msg)
  
iter_dt 0.00ms; iter 0: train loss 1.06646
iter_dt 19.26ms; iter 100: train loss 0.13984
iter_dt 19.41ms; iter 200: train loss 0.05240
iter_dt 18.38ms; iter 300: train loss 0.01114
iter_dt 20.08ms; iter 400: train loss 0.05151
iter_dt 19.12ms; iter 500: train loss 0.02871
iter_dt 18.89ms; iter 600: train loss 0.08735
iter_dt 20.64ms; iter 700: train loss 0.00715
iter_dt 17.61ms; iter 800: train loss 0.01766
iter_dt 19.13ms; iter 900: train loss 0.02940
iter_dt 19.60ms; iter 1000: train loss 0.01233
iter_dt 19.74ms; iter 1100: train loss 0.02167
iter_dt 18.90ms; iter 1200: train loss 0.01647
iter_dt 17.48ms; iter 1300: train loss 0.01915
iter_dt 20.00ms; iter 1400: train loss 0.00056
iter_dt 16.95ms; iter 1500: train loss 0.02157
iter_dt 19.06ms; iter 1600: train loss 0.00426
iter_dt 17.79ms; iter 1700: train loss 0.03464
iter_dt 16.83ms; iter 1800: train loss 0.00071
iter_dt 16.56ms; iter 1900: train loss 0.00308

Evaluating model...
train final score: 5000/5000 = 100.00% correct
test final score: 5000/5000 = 100.00% correct

Testing with a specific sequence...
input sequence  : [[0, 0, 2, 1, 0, 1]]
predicted sorted: [[0, 0, 0, 1, 1, 2]]
gt sort         : [0, 0, 0, 1, 1, 2]
matches         : True
```

#### 损失变化（Loss）

```Java
iter 0:    1.06646  ← 初始：完全随机
iter 1000: 0.01233  ← 中期：已经学得很好
iter 1900: 0.00308  ← 末期：几乎完美
```
说明：模型从"随机猜测"进化到"精确预测"

#### 准确率

```Java
# 训练集 100% 正确 → 模型学会了
train final score: 5000/5000 = 100.00% correct  ✅

# 测试集也 100% 正确 → 模型没有死记硬背，而是真正理解了排序算法
test final score:  5000/5000 = 100.00% correct  ✅

# 如果过拟合，测试集会明显低于训练集。这里两者都是 100%，说明模型学到了通用规律
```

#### 验证

```Java
输入：     [0, 0, 2, 1, 0, 1]
模型预测： [0, 0, 0, 1, 1, 2]  ✅
正确答案： [0, 0, 0, 1, 1, 2]                      (Ground Truth（真实值/标准答案/真值）)
matches: True
```

说明：模型在从未见过的具体序列上也能正确排序


### 4.2 实践项目 (动手训练模型)

####  4.2.1 加法器任务`adder.py`

A. projects/adder/ - 推荐首先尝试！

- 任务：训练 GPT 学习加法（如 123+456=579）

- 难度：⭐⭐

- 收获：理解 GPT 如何学习算术逻辑，体会位置编码和注意力机制的作用

- 这是 GPT-3 论文中提到的经典实验

这个项目会训练一个 GPT 模型来学习 n 位数的加法运算。例如：85 + 50 = 135
可视化理解模型如何学习算术规则

**a) 加法器项目（最简单）**
```bash
projects/adder/adder.py
```


#### 4.2.2 文本生成任务 `chargpt`

B. projects/chargpt/ - 更有趣但更复杂
- 任务：训练字符级语言模型（比如莎士比亚文本生成）
- 难度：⭐⭐⭐
- 收获：理解语言模型如何学习文本风格和结构


**b) 字符级语言模型**
```bash
projects/chargpt/chargpt.py
```
- 在文本上训练字符级 LM
- 可以下载 tiny-shakespeare 数据集测试


## 五、待尝试

1. **修改超参数**：尝试改变 `n_layer`、`n_head`、`n_embd` 观察效果
2. **自定义数据集**：用自己的数据训练模型
3. **添加功能**：尝试实现 README 中的 todos
4. 阅读 README 中引用的论文（GPT-1/2/3）
5. **Debug & Visualize**：
   - 打印 attention weights 可视化注意力模式
   - 观察不同层学到了什么
6. 如果想要更现代的实现，可以看 [[nanoGPT]]（作者的新版本）
	- - 对比 minGPT vs nanoGPT 的设计差异
	- nanoGPT 更注重性能和可复现性
