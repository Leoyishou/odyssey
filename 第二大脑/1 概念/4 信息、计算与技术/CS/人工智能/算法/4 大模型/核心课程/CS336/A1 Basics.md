---
uid: 78387fac-a272-4d00-8c76-bc5445749d4a
date created: 2025-07-19
date modified: 2025-07-21
---
- [测试用例总结](#%E6%B5%8B%E8%AF%95%E7%94%A8%E4%BE%8B%E6%80%BB%E7%BB%93)
	- [Level 1: 原子级组件测试 (最小粒度)](#Level%201:%20%E5%8E%9F%E5%AD%90%E7%BA%A7%E7%BB%84%E4%BB%B6%E6%B5%8B%E8%AF%95%20(%E6%9C%80%E5%B0%8F%E7%B2%92%E5%BA%A6))
	- [Level 2: 组件级测试 (小型组合单元)](#Level%202:%20%E7%BB%84%E4%BB%B6%E7%BA%A7%E6%B5%8B%E8%AF%95%20(%E5%B0%8F%E5%9E%8B%E7%BB%84%E5%90%88%E5%8D%95%E5%85%83))
	- [Level 3: 模块级测试 (复杂组合组件)](#Level%203:%20%E6%A8%A1%E5%9D%97%E7%BA%A7%E6%B5%8B%E8%AF%95%20(%E5%A4%8D%E6%9D%82%E7%BB%84%E5%90%88%E7%BB%84%E4%BB%B6))
	- [Level 4: 系统级测试 (完整系统/管道)](#Level%204:%20%E7%B3%BB%E7%BB%9F%E7%BA%A7%E6%B5%8B%E8%AF%95%20(%E5%AE%8C%E6%95%B4%E7%B3%BB%E7%BB%9F/%E7%AE%A1%E9%81%93))
- [项目结构](#%E9%A1%B9%E7%9B%AE%E7%BB%93%E6%9E%84)
- [项目代码执行流程](#%E9%A1%B9%E7%9B%AE%E4%BB%A3%E7%A0%81%E6%89%A7%E8%A1%8C%E6%B5%81%E7%A8%8B)
	- [1. UV 环境管理器启动](#1.%20UV%20%E7%8E%AF%E5%A2%83%E7%AE%A1%E7%90%86%E5%99%A8%E5%90%AF%E5%8A%A8)
	- [2 Pytest 初始化流程](#2%20Pytest%20%E5%88%9D%E5%A7%8B%E5%8C%96%E6%B5%81%E7%A8%8B)
	- [3 测试执行流程（以 test_model.py::test_linear 为例）](#3%20%E6%B5%8B%E8%AF%95%E6%89%A7%E8%A1%8C%E6%B5%81%E7%A8%8B%EF%BC%88%E4%BB%A5%20test_model.py::test_linear%20%E4%B8%BA%E4%BE%8B%EF%BC%89)
	- [完整调用栈示例](#%E5%AE%8C%E6%95%B4%E8%B0%83%E7%94%A8%E6%A0%88%E7%A4%BA%E4%BE%8B)
- [实现各组件的代码](#%E5%AE%9E%E7%8E%B0%E5%90%84%E7%BB%84%E4%BB%B6%E7%9A%84%E4%BB%A3%E7%A0%81)
- [训练数据](#%E8%AE%AD%E7%BB%83%E6%95%B0%E6%8D%AE)
- [test_transformer_lm 的流程](#test_transformer_lm%20%E7%9A%84%E6%B5%81%E7%A8%8B)
- [模型能做什么？](#%E6%A8%A1%E5%9E%8B%E8%83%BD%E5%81%9A%E4%BB%80%E4%B9%88%EF%BC%9F)
	- [1. 文本生成](#1.%20%E6%96%87%E6%9C%AC%E7%94%9F%E6%88%90)
	- [2. 文本补全](#2.%20%E6%96%87%E6%9C%AC%E8%A1%A5%E5%85%A8)
	- [3. 零样本学习 (经过大规模训练后)](#3.%20%E9%9B%B6%E6%A0%B7%E6%9C%AC%E5%AD%A6%E4%B9%A0%20(%E7%BB%8F%E8%BF%87%E5%A4%A7%E8%A7%84%E6%A8%A1%E8%AE%AD%E7%BB%83%E5%90%8E))
- [核心组件的作用](#%E6%A0%B8%E5%BF%83%E7%BB%84%E4%BB%B6%E7%9A%84%E4%BD%9C%E7%94%A8)
- [Assignment 1: Basics (GitHub: stanford-cs336/assignment1-basics) 通俗解释](#Assignment%201:%20Basics%20(GitHub:%20stanford-cs336/assignment1-basics)%20%E9%80%9A%E4%BF%97%E8%A7%A3%E9%87%8A)
	- [它干嘛的？](#%E5%AE%83%E5%B9%B2%E5%98%9B%E7%9A%84%EF%BC%9F)
	- [要实现的东西](#%E8%A6%81%E5%AE%9E%E7%8E%B0%E7%9A%84%E4%B8%9C%E8%A5%BF)
	- [训练 mini LM 的过程（通俗版）](#%E8%AE%AD%E7%BB%83%20mini%20LM%20%E7%9A%84%E8%BF%87%E7%A8%8B%EF%BC%88%E9%80%9A%E4%BF%97%E7%89%88%EF%BC%89)

# Summary

侧重于 [[Transformer]] 里所有组件的实现

Assignment 1: 你实现的是一个 GPT 风格的自回归语言模型，而不是 Seq2Seq！这个模型的意义：
你实现的是 **GPT-2 / GPT-3 的核心架构**！虽然你的模型规模很小，但其工作原理与当今最先进的大语言模型完全相同。
- ChatGPT 的基础就是这种 Decoder-only 架构。
- 模型通过“预测下一个词”这一简单任务来学习语言的复杂规律。
- 这个基础模型可以通过微调（fine-tuning）来适应各种特定的下游任务。

所以，Assignment 1 实际上是让你从零开始，亲手构建了一个迷你版的 GPT！🎉

# Cues

[[snapshot]]
[[token ID]]
[[词汇表]]
[[pt 文件]]
[[BPE]]

# Notes
## 测试驱动学习

测试层次结构：
```Java
├── Transformer功能测试 (test_model.py)  
│   └── 验证Transformer计算正确性
├── 基础设施测试 (其他所有测试)
│   ├── checkpointing机制
│   ├── 优化器功能  
│   ├── 数据加载
│   └── 工具函数
```

 测试重要性排序
  1. 🔴 最核心：test_model.py - 没有这个，模型根本跑不起来
  2. 🟠 很重要：test_nn_utils.py, test_optimizer.py - 训练必需的基础设施
  3. 🟡 重要：test_data.py, test_serialization.py - 实际训练时需要
  4. 🟢 次要：test_tokenizer.py, test_train_bpe.py - 可以用现成的分词器代替

  当前通过情况
  - ✅ 基本通过：前 4 个文件的大部分测试
  - ⚠️ 数值精度问题：test_model.py 中的 transformer_lm（误差 < 0.0002）
  - ❌ 未通过：test_tokenizer.py 和 test_train_bpe.py 的部分测试
  
按照粒度从小到大排列的所有测试用例：
- **总测试文件数**: 7  
- **总测试用例数**: 45  
- **原子级测试**: 7个 (基础数学运算和层)  
- **组件级测试**: 7个 (组合的小模块)  
- **模块级测试**: 10个 (复杂功能模块)  
- **系统级测试**: 21个 (完整功能和集成测试)  
### Level 1: 原子级组件测试 (最小粒度)  

| 测试文件             | 测试函数                         | 测试目的                                                     |     |
| ---------------- | ---------------------------- | -------------------------------------------------------- | --- |
| test_nn_utils.py | test_softmax_matches_pytorch | 验证 [[softmax]] 实现与 PyTorch 一致，包括数值稳定性                    | ✅   |
| test_nn_utils.py | test_cross_entropy           | 验证[交叉熵 cross_entropy](交叉熵%20cross_entropy.md)损失计算，包括溢出处理 | ✅   |
| test_nn_utils.py | test_gradient_clipping       | 验证[[梯度裁剪]]功能                                             | ✅   |
| test_model.py    | test_linear                  | 验证[[全连接层，线性层]]实现                                         | ✅   |
| test_model.py    | test_embedding               | 验证[embedding](embedding.md)实现                    | ✅   |
| test_model.py    | test_silu_matches_pytorch    | 验证 SiLU [[激活函数]]与 PyTorch 一致                             | ✅   |
| test_model.py    | test_rmsnorm                 | 验证 RMSNorm [[归一化]]层                                      | ✅   |

### Level 2: 组件级测试 (小型组合单元)  

| 测试文件              | 测试函数                                 | 测试目的                                                                   |     |
| ----------------- | ------------------------------------ | ---------------------------------------------------------------------- | --- |
| test_model.py     | test_swiglu                          | 验证 [[SwiGLU]] [[前馈网络]]组件                                               | ✅   |
| test_model.py     | test_rope                            | 验证旋转[Positional Encoding 位置编码](Positional%20Encoding%20位置编码.md) (RoPE) | ✅   |
| test_model.py     | test_scaled_dot_product_attention    | 验证 3D 缩放点积注意力机制                                                        | ✅   |
| test_model.py     | test_4d_scaled_dot_product_attention | 验证 4D 批次化[[注意力机制]]                                                     | ✅   |
| test_optimizer.py | test_adamw                           | 验证 AdamW 优化器实现                                                         | ✅   |
| test_optimizer.py | test_get_lr_cosine_schedule          | 验证余弦学习率调度器，带线性预热的[[余弦退火]]                                              | ✅   |
| test_data.py      | test_get_batch                       | 验证训练批次数据创建                                                             | ✅   |

### Level 3: 模块级测试 (复杂组合组件)  

| 测试文件              | 测试函数                                              | 测试目的                                                                                                   |     |
| ----------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | --- |
| test_model.py     | test_multihead_self_attention                     | 验证完整的多头自注意力模块                                                                                          | ✅   |
| test_model.py     | test_multihead_self_attention_with_rope           | 验证带位置编码的多头自注意力                                                                                         | ✅   |
| test_model.py     | test_transformer_block                            | 验证完整的 Transformer 块 (注意力+FFN+归一化)<br>注意只是随机生成几个矩阵跑了跑"，没有任何实际的NLP语义内容，只是用来验证 Transformer Block 的数值计算正确性 | ✅   |
| test_tokenizer.py | test_roundtrip_empty                              | 验证空字符串的编码解码往返                                                                                          |     |
| test_tokenizer.py | test_roundtrip_single_character                   | 验证单个 ASCII 字符的往返                                                                                       |     |
| test_tokenizer.py | test_roundtrip_single_unicode_character           | 验证单个 Unicode 字符的往返                                                                                     |     |
| test_tokenizer.py | test_roundtrip_ascii_string                       | 验证 ASCII 字符串的往返                                                                                        |     |
| test_tokenizer.py | test_roundtrip_unicode_string                     | 验证 Unicode 字符串的往返                                                                                      |     |
| test_tokenizer.py | test_roundtrip_unicode_string_with_special_tokens | 验证带特殊标记的 Unicode 字符串往返                                                                                 |     |
| test_tokenizer.py | test_overlapping_special_tokens                   | 验证重叠特殊标记的处理                                                                                            |     |

### Level 4: 系统级测试 (完整系统/管道)  

  1. test_tokenizer.py - 使用多个真实文本文件：
    - GPT-2 词汇表和合并文件：gpt2_vocab.json、gpt2_merges.txt
    - 测试文本：address.txt、german.txt、tinystories_sample.txt、tinystories_sample_5M.txt
    - 特殊标记测试文件
  2. test_train_bpe.py - 使用真实训练数据：
    - 英文语料库：corpus.en
    - 参考词汇表和合并文件：train-bpe-reference-vocab.json、train-bpe-reference-merges.txt
    - TinyStories 样本
  3. test_model.py - 使用预训练模型权重：
    - 模型文件：ts_tests/model.pt、ts_tests/model_config.json
    - 用于验证各个神经网络组件的实现

| 测试文件                  | 测试函数                                                     | 测试目的                                                                                   |     |
| --------------------- | -------------------------------------------------------- | -------------------------------------------------------------------------------------- | --- |
| **完整模型测试**            |                                                          |                                                                                        |     |
| test_model.py         | test_transformer_lm                                      | 验证完整的 Transformer 语言模型<br>注意只是随机生成几个矩阵跑了跑"，没有任何实际的NLP语义内容，只是用来测试计算逻辑是否正确，而不是问题本身是否有意义。| ✅   |
| test_model.py         | test_transformer_lm_truncated_input                      | 验证截断输入的模型处理                                                                            |     |
| **分词器兼容性测试**          |                                                          |                                                                                        |     |
| test_tokenizer.py     | test_empty_matches_tiktoken                              | 验证空字符串与 [[tiktoken]] 一致                                                                |     |
| test_tokenizer.py     | test_single_character_matches_tiktoken                   | 验证单字符与 tiktoken 一致                                                                     |     |
| test_tokenizer.py     | test_single_unicode_character_matches_tiktoken           | 验证 Unicode 字符与 tiktoken 一致                                                             |     |
| test_tokenizer.py     | test_ascii_string_matches_tiktoken                       | 验证 ASCII 字符串与 tiktoken 一致                                                              |     |
| test_tokenizer.py     | test_unicode_string_matches_tiktoken                     | 验证 Unicode 字符串与 tiktoken 一致                                                            |     |
| test_tokenizer.py     | test_unicode_string_with_special_tokens_matches_tiktoken | 验证特殊标记与 tiktoken 一致                                                                    |     |
| **文件处理测试**            |                                                          |                                                                                        |     |
| test_tokenizer.py     | test_address_roundtrip                                   | 验证地址文本的编码解码往返                                                                          |     |
| test_tokenizer.py     | test_address_matches_tiktoken                            | 验证地址文本与 tiktoken 一致                                                                    |     |
| test_tokenizer.py     | test_german_roundtrip                                    | 验证德语文本的往返                                                                              |     |
| test_tokenizer.py     | test_german_matches_tiktoken                             | 验证德语文本与 tiktoken 一致                                                                    |     |
| test_tokenizer.py     | test_tinystories_sample_roundtrip                        | 验证 TinyStories 样本的往返                                                                   |     |
| test_tokenizer.py     | test_tinystories_matches_tiktoken                        | 验证 TinyStories 与 tiktoken 一致                                                           |     |
| **特殊情况测试**            |                                                          |                                                                                        |     |
| test_tokenizer.py     | test_encode_special_token_trailing_newlines              | 验证尾部换行符的特殊标记编码                                                                         |     |
| test_tokenizer.py     | test_encode_special_token_double_newline_non_whitespace  | 验证双换行符非空白字符的处理                                                                         |     |
| **内存效率测试**            |                                                          |                                                                                        |     |
| test_tokenizer.py     | test_encode_iterable_tinystories_sample_roundtrip        | 验证迭代器编码的往返                                                                             |     |
| test_tokenizer.py     | test_encode_iterable_tinystories_matches_tiktoken        | 验证迭代器编码与 tiktoken 一致                                                                   |     |
| test_tokenizer.py     | test_encode_iterable_memory_usage                        | 验证迭代器编码的内存使用效率                                                                         |     |
| test_tokenizer.py     | test_encode_memory_usage                                 | 验证常规编码的内存使用                                                                            |     |
| **BPE 训练测试**          |                                                          |                                                                                        |     |
| test_train_bpe.py     | test_train_bpe                                           | 验证基本 [BPE](BPE.md) 训练算法                                                                | ✅   |
| test_train_bpe.py     | test_train_bpe_special_tokens                            | 验证带特殊标记的 BPE 训练                                                                        |     |
| test_train_bpe.py     | test_train_bpe_speed                                     | 验证 BPE 训练效率 (性能测试)                                                                     |     |
| **模型持久化测试**           |                                                          |                                                                                        |     |
| test_serialization.py | test_checkpointing                                       | 验证模型[checkpoint](checkpoint.md)检查点的保存和加载                                               | ✅   |

### test_transformer_block 的流程
```Java
输入: [batch, seq, d_model] 嵌入向量
  ↓
单个Transformer Block:
  - LayerNorm → MultiHead Attention → 残差连接
  - LayerNorm → SwiGLU FFN → 残差连接
  ↓  
输出: [batch, seq, d_model] 经过一层处理的向量
```

### test_transformer_lm 的流程
```Java
输入: [batch, seq] token IDs
  ↓
Token Embedding Layer: token ID → 嵌入向量
  ↓
多个Transformer Blocks (num_layers层):
  Block 1 → Block 2 → ... → Block N
  ↓
Output Layer: 向量 → 词汇表概率分布
  ↓
输出: [batch, seq, vocab_size] 每个位置的下个token概率
```

| 测试 | 测试范围 | 输入 | 权重 | 作用 |
|-----|---------|------|------|------|
| **test_transformer_block** | **单个**Transformer块 | `in_embeddings`<br/>(已嵌入的向量) | 单个block的权重 | 测试一层的逻辑是否正确 |
| **test_transformer_lm** | **完整**语言模型 | `in_indices`<br/>(token ID) | 整个模型的权重 | 测试端到端的模型是否正确 |

## 项目结构
  1. 测试文件 (tests/test_*.py) - 定义了期望的行为
  2. 适配器文件 (tests/adapters.py) - 只有函数签名，全是 raise NotImplementedError
  3. 空的实现目录 (cs336_basics/) - 基本是空的
  项目结构：
```Java
  ├── pyproject.toml          # 项目配置，定义依赖
  ├── tests/
  │   ├── fixtures/ts_tests  # 会放模型的权重和配置
  │   ├── conftest.py        # pytest 配置和 fixtures
  │   ├── adapters.py        # 测试适配器，连接测试和实现
  │   ├── test_*.py          # 具体测试用例
  │   └── _snapshots/        # 预期输出的参考值
  
  ├── cs336_basics/          # 我们的实现
  │   ├── nn.py             # 神经网络基础组件
  │   ├── attention.py      # 注意力机制
  │   ├── transformer.py    # Transformer 架构
  │   ├── training.py       # 训练工具
  │   └── tokenizer.py      # BPE 分词器
```

## 项目代码执行流程

### 1. UV 环境管理器启动

  `uv run pytest`

  当执行这个命令时：
  1. UV 读取 pyproject.toml（项目根目录）
    - 解析项目依赖
    - 创建或激活虚拟环境.venv/
    - 确保所有依赖已安装
  2. UV 在虚拟环境中执行 [[pytest]]
    - 实际执行：`.venv/bin/python -m pytest`

### 2 Pytest 初始化流程

2.1 配置加载
```Java
  # pyproject.toml 中的 pytest 配置
  [tool.pytest.ini_options]
  testpaths = ["tests"]
  pythonpath = ["."]
```

2.2 测试发现
```Java
  Pytest 扫描 tests/ 目录，找到所有 test_*.py 文件：
  tests/
  ├── test_model.py
  ├── test_nn_utils.py
  ├── test_optimizer.py
  ├── test_data.py
  ├── test_serialization.py
  ├── test_tokenizer.py
  └── test_train_bpe.py
```


### 3 测试执行流程（以 test_model.py::test_linear 为例）

| 测试文件 | 测试内容 | 预期功能 | 通俗解释 |
|:--- |:--- |:--- |:--- |
| `test_model.py` | - Linear 层<br>- Embedding 层<br>- RMSNorm<br>- SiLU/SwiGLU<br>- 注意力机制<br>- RoPE<br>- Transformer 块<br>- 完整语言模型 | 验证神经网络各组件的前向传播计算正确 | **测试"积木块"能否正确工作：**<br>• 矩阵乘法对不对<br>• 注意力有没有算错<br>• 位置编码是否正确<br>• 整个模型能否正常跑通 |
| `test_nn_utils.py` | - Softmax 函数<br>- 交叉熵损失<br>- 梯度裁剪 | 验证基础数学运算和训练工具的正确性 | **测试"数学公式"实现对不对：**<br>• 概率归一化是否正确<br>• 损失计算准不准<br>• 梯度会不会爆炸 |
| `test_optimizer.py` | - AdamW 优化器<br>- 余弦学习率调度 | 验证优化器更新权重和学习率变化的逻辑 | **测试"学习策略"：**<br>• 权重更新方向对不对<br>• 学习率是否按计划衰减 |
| `test_data.py` | - 批量数据采样<br>- 输入输出对齐 | 验证从数据集中正确采样训练批次 | **测试"喂数据"的方式：**<br>• 能否随机抽取连续文本<br>• 输入和标签是否错位一个token |
| `test_serialization.py` | - 保存检查点<br>- 加载检查点<br>- 恢复训练状态 | 验证模型和优化器状态的保存/恢复 | **测试"存档读档"：**<br>• 训练一半断电能否继续<br>• 模型参数有没有丢失 |
| `test_tokenizer.py` | - BPE 编码/解码<br>- 特殊标记处理<br>- Unicode 支持<br>- 与 tiktoken 对比 | 验证分词器能正确处理各种文本 | **测试"文字切分"：**<br>• 中文、emoji 能否处理<br>• 特殊符号会不会出错<br>• 编码解码是否可逆 |
| `test_train_bpe.py` | - BPE 训练算法<br>- 词表构建<br>- 合并规则学习 | 验证能从语料库训练出合理的分词器 | **测试"学习切词"：**<br>• 能否找出高频子词<br>• 词表大小是否符合预期<br>• 训练速度够不够快 |

  3.1 Fixture 加载
```python
  # tests/conftest.py:20-30
  @pytest.fixture
  def d_model():
      return 64

  @pytest.fixture  
  def vocab_size():
      return 10000
    # 这些 fixture 在测试函数需要时自动注入
```



  3.2 测试函数执行
```python
# tests/test_model.py:19-27
def test_linear(ts_state_dict, in_embeddings):
  # 1. 从 fixture 获取权重
  w1_weight = ts_state_dict[0][f"layers.0.ffn.w1.weight"]

  # 2. 调用 adapters.py 中的包装函数
  actual_output = run_linear(
	  d_in=d_model,
	  d_out=d_ff,
	  weights=w1_weight,
	  in_features=in_embeddings,
  )

```

  3.3 适配器调用
```python
# tests/adapters.py:24-51
def run_linear(
  d_in: int,
  d_out: int,
  weights: Float[Tensor, " d_out d_in"],
  in_features: Float[Tensor, " ... d_in"],
) -> Float[Tensor, " ... d_out"]:
  # 原本是: raise NotImplementedError
  # 现在是: 调用我们的实现
  return linear(weights, in_features)
```


  3.4 实际实现执行
```python
# cs336_basics/nn.py:10-15
def linear(
  weights: Float[Tensor, "d_out d_in"],
  in_features: Float[Tensor, "... d_in"],
) -> Float[Tensor, "... d_out"]:
  """Apply linear transformation: out = in @ weights.T"""
  return in_features @ weights.T  # 实际的矩阵乘法
```


  3.5 结果验证

  # tests/test_model.py:29
  numpy_snapshot.assert_match(actual_output, atol=1e-5)

  这会：
  5. 加载预期结果：tests/_snapshots/test_linear.npz
  6. 比较实际输出和预期输出
  7. 如果误差在容忍范围内，测试通过

## 完整调用栈示例

  以 test_scaled_dot_product_attention 为例：

  1. uv run pytest tests/test_model.py::test_scaled_dot_product_attention
     ↓
  2. pytest 加载 conftest.py 中的 fixtures
     ↓
  3. 执行 test_model.py:65 的测试函数
     ↓
  4. 调用 adapters.py:90 run_scaled_dot_product_attention()
     ↓
  5. 调用 cs336_basics/attention.py:10 scaled_dot_product_attention()
     ↓
  6. 执行实际计算:
     - scores = Q @ K.T / sqrt(d_k)  (第20行)
     - 应用 mask (第23-30行)
     - softmax (第33行)
     - 返回 attn_weights @ V (第36行)
     ↓
  7. numpy_snapshot.assert_match() 验证结果
     ↓
  8. 测试通过 ✓ 或失败 ✗

  9. 测试失败时的调试流程

  当测试失败时，pytest 会显示：
  E   AssertionError:
  E   Not equal to tolerance rtol=0.0001, atol=1e-06
  E   Array 'array' does not match snapshot for test_transformer_block
  E   Mismatched elements: 74 / 3072 (2.41%)
  E   Max absolute difference: 2.1338463e-05
  E    ACTUAL: array([[[0.058676,  1.071681, -0.696942,...
  E    DESIRED: array([[[0.058669,  1.071683, -0.696938,...

  这告诉我们：
  - 哪个测试失败了
  - 期望值 vs 实际值
  - 误差有多大
  - 在代码的哪一行


# 实现各组件的代码
  1. 在 cs336_basics/ 目录下创建了实现文件：
    - nn.py - 实现了 Linear, Embedding, RMSNorm, SiLU, SwiGLU
    - attention.py - 实现了 Scaled Dot-Product Attention, Multi-Head Self-Attention, RoPE
    - transformer.py - 实现了 Transformer Block, Transformer LM
    - training.py - 实现了训练相关工具
    - tokenizer.py - 实现了 BPE Tokenizer

| 名词                                     | 在模型/代码中的角色   | 通俗解释                                        |
| -------------------------------------- | ------------ | ------------------------------------------- |
| **Linear**                             | 最基本的全连接层     | 就像“乘矩阵 + 加偏置”的计算器，把输入维度线性变换到输出维度            |
| **Embedding**                          | 词向量查表        | 把单词（整数 id）映射成稠密向量，等于给“词典”每个词配一行向量           |
| **RMSNorm**                            | 归一化层         | 用均方根（RMS）而不是均值方差来做缩放，参数更少、更快                |
| **SiLU (Swish)**                       | 激活函数         | 公式 _x · σ(x)_，比 ReLU/GELU 更平滑，让梯度流动更顺       |
| **SwiGLU**                             | 前馈网络中的“激活+门” | 两个线性变换后，一路用 SiLU 当“闸门”乘到另一路，信息筛得更精细         |
| **Scaled Dot‑Product Attention**       | 注意力核心公式      | “点积相似度 / √d”再做 softmax，告诉模型“哪些词彼此相关”        |
| **Multi‑Head Self‑Attention**          | 多头注意力        | 把注意力拆成 h 份并行计算，小视角拼成大视角，学到多种关系              |
| **RoPE (Rotary Positional Embedding)** | 位置编码方式       | 用旋转矩阵给 Q/K 加“角度”，不新增参数却支持长序列外推              |
| **Transformer Block**                  | 网络基本单元       | “注意力层 + 残差 + Norm + MLP” 的组合砖，叠很多块就成大楼      |
| **Transformer Language Model**         | 整体架构         | 输入 token，层层 Transformer Block，最后预测下一个 token |
| **Softmax**                            | 概率归一化        | 把一串实数压成 0‑1 区间且总和=1，用来表示“谁的概率大”             |
| **Cross‑Entropy**                      | 训练损失         | 比较预测分布和真实分布差距，差距越大损失越高                      |
| **Gradient Clipping**                  | 梯度裁剪         | 当梯度爆炸时把其“限速”，防止训练数值不稳定                      |
| **Batch Sampling**                     | 小批量取样        | 一次喂 N 条数据给 GPU，既加速又让梯度估计更平滑                 |
| **Learning Rate Scheduling**           | 学习率调度        | 训练过程中动态调节步长，常见余弦、线性暖启等策略                    |
| **Save Checkpoint**                    | 保存权重         | 把当前模型参数写入文件，断电也能从这一步继续                      |
| **Load Checkpoint**                    | 加载权重         | 读回保存的参数，恢复模型“记忆”                            |
| **BPE Tokenizer**                      | 子词分词器        | 先把生僻词拆成常见“子词”，词表更小、覆盖率更高                    |

  2. 更新 `adapters.py`，把 raise NotImplementedError 替换成调用我实现的函数
  3. 逐个运行测试，根据测试失败的信息调整实现，直到测试通过

# 训练数据

得到训练样本的逻辑

```Java
# 从位置15开始取数据：

原始片段: [15, 16, 17, 18, 19, 20, 21, 22]

		↑—————————   7个  ————————↑    ↑

			       x (输入)         y的最后一位

  

x[0] = [15, 16, 17, 18, 19, 20, 21] # 输入序列

y[0] = [16, 17, 18, 19, 20, 21, 22] # 目标序列（向右偏移1位）


```

所以训练样本：

```Java
训练样本：

位置0: 输入=15, 目标=16 → "看到15，预测16"

位置1: 输入=16, 目标=17 → "看到16，预测17"

位置2: 输入=17, 目标=18 → "看到17，预测18"

位置3: 输入=18, 目标=19 → "看到18，预测19"

位置4: 输入=19, 目标=20 → "看到19，预测20"

位置5: 输入=20, 目标=21 → "看到20，预测21"

位置6: 输入=21, 目标=22 → "看到21，预测22"
```

# test_transformer_lm 的流程
``

> 输入文本 "Hello world"
>
> ↓
>
> **[Tokenizer]** → `[101, 7592, 2088]`
>
> ↓
>
> **[Embedding]** → 将 token ID 转换为词向量
>
> ↓
>
> **[位置编码 RoPE]** → 为词向量注入位置信息
>
> ↓
>
> **[N × Transformer Block]**
> ├── Multi-Head Attention (使用因果 Mask)
> ├── [[RMSNorm]]
> ├── [[SwiGLU]]
> └── 残差连接
>
> ↓
>
> **[Final RMSNorm]**
>
> ↓
>
> **[LM Head]** → 投影到词汇表大小，得到 logits
>
> ↓
>
> **[Softmax]** → 将 logits 转换为概率分布
>
> ↓
>
> **预测下一个 token**




```Java



```
```Java
输入 Token IDs
    ↓
Token Embedding (将ID转换为向量)
    ↓
TransformerBlock 0 (第1层)
    ├─ Multi-Head Self-Attention
    ├─ Add & Norm
    ├─ Feed Forward (SwiGLU)
    └─ Add & Norm
    ↓
TransformerBlock 1 (第2层)
    ├─ Multi-Head Self-Attention
    ├─ Add & Norm
    ├─ Feed Forward (SwiGLU)
    └─ Add & Norm
    ↓
Final Layer Norm
    ↓
Linear Projection (投影到词汇表大小)
    ↓
输出 Logits
```

# 在写什么？



**模型类型：Decoder-only Transformer（GPT 架构）**

- **输入**：一串 token
    ```python
    # 例如: "This is a"
    input_ids = [101, 2023, 2003, 1037]
    ```

- **输出**：在每个位置上，预测下一个 token 的概率分布
    ```python
    # logits 的形状: [batch_size, sequence_length, vocab_size]
    logits = model(input_ids)
    ```

- **如何生成下一个词**：取序列最后一个位置的 logits，并找到概率最高的 token
    ```python
    # 取最后一个位置的输出
    next_token_logits = logits[:, -1, :]
    
    # 找到概率最高的 token，例如 "test"
    next_token = torch.argmax(next_token_logits)
    ```

与 Seq2Seq 的区别

| 特性 | 你的模型 (GPT-style) | Seq2Seq |
|:--- |:--- |:--- |
| **架构** | 只有 Decoder | Encoder + Decoder |
| **任务** | 语言建模 (预测下一个词) | 翻译、摘要等 |
| **注意力** | 因果注意力 (只看左边) | 交叉注意力 (看整个输入) |
| **训练方式** | 自回归 (一个个预测) | 输入 → 输出映射 |
| **典型应用** | 文本生成、对话、续写 | 翻译、问答、摘要 |

## 模型能做什么？

### 1. 文本生成
通过不断预测下一个词并将其添加回输入，模型可以生成连贯的文本。
```python
prompt = "Once upon a time"
for _ in range(50):
    logits = model(tokenizer.encode(prompt))
    next_token = sample(logits[:, -1, :]) # 从概率分布中采样
    prompt += tokenizer.decode([next_token])

# 可能的输出: "Once upon a time, there was a little girl who..."
```

### 2. 文本补全
给定一段前文，模型会预测最可能的后续文本。
```Java
# 输入
text = "The capital of France is"

# 模型会预测: "Paris"
```

### 3. 零样本学习 (经过大规模训练后)
模型可以执行它没有被明确训练过的任务，只需给出正确的提示。
```Java
# 提示
prompt = "Translate to French: Hello world\nOutput:"

# 模型可能输出: "Bonjour le monde"
```

## 核心组件的作用

这是一个简化的数据流，展示了各个组件如何协同工作：




为什么你的模型不是 Seq2Seq？
1. **没有 Encoder**：你的实现中只有 Decoder 部分，没有一个独立的 Encoder 来处理源序列。
2. **没有交叉注意力**：模型中只有自注意力机制（Self-Attention），没有交叉注意力机制（Cross-Attention）来融合 Encoder 的信息。
3. **使用了因果 Mask**：注意力机制被强制要求只能关注当前位置及其左边的 token，无法“看到”未来的信息。这是通过一个上三角矩阵实现的：
    ```python
    # 你的注意力 mask 确保了位置 i 只能看到 0...i-1 的信息
    # 这使得模型在预测下一个词时，无法作弊
    mask = torch.triu(torch.ones(seq_len, seq_len) * float("-inf"), diagonal=1)
    ```




# 测试什么？


  这就像盖房子：
  - 地基和框架（model, nn_utils）必须稳固
  - 水电系统（optimizer, data）要正常工作
  - 装修细节（tokenizer）可以后期完善



  没有，Assignment 1 还没有完全通过所有测试。具体情况是：

  1. 总共有 48 个测试
  2. 核心功能大部分已实现：
    - ✅ 基础神经网络组件
    - ✅ 注意力机制
    - ✅ Transformer 架构
    - ✅ 训练工具
    - ✅ 模型保存/加载
    - ⚠️ BPE 分词器（基本实现但有问题）
  3. 测试通过情况：
    - 约 17-20 个核心测试通过
    - 3 个测试因为数值精度问题失败（误差 < 0.0002，实际上可以接受）
    - 剩余约 25-28 个测试失败，主要是 BPE 分词器相关的测试
  4. 主要问题：
    - Transformer 语言模型和 Transformer 块的数值精度差异（非常小，不影响实际使用）
    - BPE 分词器的特殊标记处理有问题
    - BPE 训练算法可能有 bug

  所以严格来说，Assignment 1 还没有完全通过，但核心的 Transformer 实现已经基本完成且功能正确。

___



___








## Assignment 1: Basics (GitHub: stanford-cs336/assignment1-basics) 通俗解释

这个作业是斯坦福 CS336 课程（Language Modeling from Scratch，春季 2025）的第一个任务，目的是让你从零开始搭建语言模型的基础。简单说，就是教你怎么让计算机“学习”文本，比如预测下一个词，就像 ChatGPT 的简化版。通过这个，你能搞懂语言模型的核心步骤：怎么处理文本、建模型、训练它变聪明。重点不是理论推导，而是动手写代码，实现关键部分，然后用小数据集训练一个 mini 语言模型（LM）。整个作业用 PyTorch，代码框架在 GitHub 上，你填空就好，还有 unit tests 来检查对不对。
### 它干嘛的？
- **整体目标**：构建一个简单的语言模型系统，从文本输入到预测输出。全过程包括数据处理、模型构建和训练优化。完成后，你能用它在小数据集上训练模型，比如用故事文本预测下一个词。
- **为什么重要**：这是课程的基础，后面的作业会基于这个扩展（如优化速度、处理大数据）。它帮你理解 Transformer 等模型的“从 scratch”实现，但这里可能是简化版（不一定是全 Transformer，可能从 GPT-like 开始）。

### 要实现的东西
你要填的代码主要是三个核心组件：tokenizer（分词器）、model（模型）和 optimizer（优化器）。这些是语言模型的“积木块”。作业提供 starter code，你在 `./tests/adapters.py` 里实现函数，跑 `uv run pytest` 测试通过就算 OK。<grok:render card_id="bd9f83" card_type="citation_card" type="render_inline_citation">
<argument name="citation_id">0</argument>
</grok:render>

- **Tokenizer（分词器）**：
  - **通俗解释**：文本是乱七八糟的字符串，计算机不懂。你需要写个工具，把文本拆成小块（像单词或字符），然后转成数字（tokens），模型才能处理。就像把句子切成词条，再编号。
  - **Input（输入）**：原始文本数据，比如故事集（从 TinyStories 或 OpenWebText 下载的小数据集，train 和 valid 分开）。
  - **Output（输出）**：tokens 序列（数字列表），比如 "Hello world" 可能变成[101, 202]这样的数值数组，便于模型计算。

- **Model（模型）**：
  - **通俗解释**：这是大脑部分。你要建一个 mini LM，能根据前面的词预测下一个词。比如输入 “The cat is”，它输出 “sleeping” 的概率最高。可能基于 Transformer 或简单 RNN，但课程偏 Transformer。
  - **Input（输入）**：从 tokenizer 来的 tokens 序列（一批文本片段）。
  - **Output（输出）**：下一个 token 的预测（概率分布），比如对词汇表里的每个词打分，选分数最高的作为下一个词。

- **Optimizer（优化器）**：
  - **通俗解释**：模型一开始预测乱七八糟，你需要写优化逻辑，让它通过比较预测和真实答案，调整内部参数（权重），越来越准。就像老师改作业，告诉模型“错哪儿了，下次改”。
  - **Input（输入）**：模型的预测结果 + 真实下一个 token（从数据集标签）。
  - **Output（输出）**：更新后的模型参数（权重矩阵），让 loss（错误率）下降。

### 训练 mini LM 的过程（通俗版）
1. 下载数据集：用 TinyStories（简单故事）或 OpenWebText（网页文本）的小样本。
2. 处理数据：tokenizer 把文本转 tokens。
3. 建模型：用你实现的 model，输入 tokens，输出预测。
4. 训练：喂数据给模型，算 loss，用 optimizer 更新。跑几轮 epoch，看 loss 降不降。
5. 测试：用 valid 数据检查模型准不准（e.g., perplexity 分数，低了好）。

整个作业强调 reproducibility（用 uv 工具设置环境），小数据集先在 CPU 调试，GPU 加速训练。完成后，你就有一个能跑的 mini LM，能在简单任务上工作。时间估计 4-6 小时，如果你 Transformer 基础好，很快。






以下表格按出现顺序逐个解释 CS336 作业中提到的关键名词，力求**通俗易懂**——看完就能大致知道“它干啥、在哪儿用”。



看这两个测试函数，它们测试的是**不同层次**的组件：

## 1. `test_transformer_block` - 测试单个Transformer块

```python:188-206:assignment1-basics/tests/test_model.py
def test_transformer_block(numpy_snapshot, ts_state_dict, in_embeddings, d_model, n_heads, d_ff, n_keys, theta):
    # 提取单个block的权重（第0层）
    block_weights = {k.replace("layers.0.", ""): v for k, v in ts_state_dict[0].items() if "layers.0." in k}

    actual_output = run_transformer_block(
        d_model=d_model,
        num_heads=n_heads,
        d_ff=d_ff,
        max_seq_len=n_keys,
        theta=theta,
        weights=block_weights,        # 只有一个block的权重
        in_features=in_embeddings,    # 输入是已经嵌入的向量
    )
```

## 2. `test_transformer_lm` - 测试完整语言模型

```python:134-154:assignment1-basics/tests/test_model.py
def test_transformer_lm(
    numpy_snapshot, vocab_size, n_keys, d_model, n_layers, n_heads, d_ff, theta, ts_state_dict, in_indices
):
    state_dict, _ = ts_state_dict

    actual_output = run_transformer_lm(
        vocab_size=vocab_size,
        context_length=n_keys,
        d_model=d_model,
        num_layers=n_layers,          # 多层
        num_heads=n_heads,
        d_ff=d_ff,
        rope_theta=theta,
        weights=state_dict,           # 完整模型的所有权重
        in_indices=in_indices,        # 输入是token ID
    )
```
