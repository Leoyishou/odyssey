---
date created: 2025-01-25
date modified: 2025-07-10
uid: 575fd053-3a1f-4fdf-89e1-3f37c75e4ac0
---

下面给你一个简要的「**任务清单**」，帮助你快速了解 **Assignment #4** 的所有要求，并知道需要在哪些文件里完成哪些功能，以及有哪些书面回答的题目要写在 Written 作业里并提交。

---

## Assignment 概览

本次作业分为两大部分：

1. **Neural Machine Translation with RNNs (45 分)**  
    以**西语 → 英语**的翻译为例，使用 **双向 LSTM Encoder + 单向 LSTM Decoder**，以及 **multiplicative attention** 来实现一个 Seq2Seq 的 NMT 系统。
    
    - 你需要**在代码中实现**各个模块（embedding、encode、decode、attention 等），并进行训练和测试，最终得到一个 BLEU 分数。
    - 其中还包含两个**简短的书面分析问题**（(g) 关于 mask 的作用，(j) 关于不同 attention 机制的优劣）。
2. **Analyzing NMT Systems (30 分)**  
    主要是**书面分析题**，包括：
    
    - 给出已有模型输出的翻译错误，并说明**错误类型**、**原因**以及**改进方法**；
    - 手动计算 BLEU 分数，比较不同参考译文数量时 BLEU 分数的差异，以及讨论 BLEU 指标 vs. 人工评测的优缺点。

这两部分**相互独立**：如果你在第一部分遇到瓶颈，仍然可以先做第二部分的**written**分析题。

---

## Part 1: Neural Machine Translation with RNNs (45 分)

在此部分，你需要主要在 **`nmt_model.py`** 和 **`model_embeddings.py`**（以及一些辅助文件，如 `utils.py`）里完成对应的函数。完成后，在你的代码环境里跑 `sh run.sh train_local` 或者在 Azure VM 上跑 `sh run.sh train` 进行训练（大约需 4 小时 GPU 时间）。

### (a) (2 分) 在 `utils.py` 里实现 `pad_sents` 函数

- **功能**：对一个 batch 中的句子进行补齐（padding），保证它们有相同长度，方便后续张量运算。
- 你只需在 `pad_sents` TODO 处完成并返回补齐后的结果。

### (b) (3 分) 在 `model_embeddings.py` 里实现 `__init__` 函数

- **功能**：分别创建**源语言（source）**和**目标语言（target）**的嵌入层 (Embedding layer)。
- 作业要求**不直接使用** `nn.Embedding` 的高层封装，而是要自己写或至少自己初始化相关参数。

### (c) (4 分) 在 `nmt_model.py` 的 `__init__` 中初始化各层

- **功能**：
    - 建立 NMT 模型需要的层（**LSTM、投影层、dropout**等）
    - 调用 `ModelEmbeddings` 来创建 embeddings
    - 这些层会在后面的 `encode()`、`decode()`、`step()` 等函数里被调用。

### (d) (8 分) 在 `nmt_model.py` 中实现 `encode()` 函数

- **功能**：
    1. 把补齐后的源语句转换为张量 X；
    2. 送入双向 LSTM（Bi-LSTM），得到 encoder hidden states hench_{enc}；
    3. 线性映射 encoder 最后的 hidden state/cell state 用于初始化 decoder 的 hdec0h_{dec}^0 和 cdec0c_{dec}^0；
- 可以用 `python sanity_check.py 1d` 做一个初步测试。

### (e) (8 分) 在 `nmt_model.py` 中实现 `decode()` 函数

- **功能**：
    1. 把目标语言词向量（以及前一步的输出向量）拼接，形成 decoder 的输入；
    2. 多步（time step）调用 `step()` 函数，直到处理完目标句所有 token；
    3. 收集 decoder 的输出，用来计算预测分布等。
- 可以用 `python sanity_check.py 1e` 做初步测试。

### (f) (10 分) 在 `nmt_model.py` 中实现 `step()` 函数

- **功能**：
    1. Decoder LSTM 单步更新 (h_t, c_t)
    2. 计算对所有 encoder hidden states 的 attention 分数 e_t，并做 softmax 得到 attention 分布 α_t
    3. 根据 α_t 做加权求和得到 context 向量 a_t
    4. 将 a_t 和 h_t 拼接、通过一个线性 + tanh + dropout 得到 combined output o_t
    5. 用 `o_t` 最终预测一个词分布
- 可以用 `python sanity_check.py 1f` 做初步测试。

### (g) (3 分) （书面问题）关于 `generate_sent_masks()` 生成的 `enc_masks`

- 题目让你用**三四句话**说明：
    1. mask 在 attention 计算中起到什么作用？
    2. 为什么在计算注意力分数时，必须这样用 mask 去屏蔽 padding token？

这些答案写在**Assignment 4 [written]**提交的报告中。

### (i) (4 分) 训练完成后，测试并报告 BLEU 分数

- **做法**：
    1. 在 Azure VM 上完整训练（`sh run.sh train`），大约 4 小时；
    2. 训练完后执行 `sh run.sh test` 计算 BLEU；
    3. 在报告中**写出最终的 BLEU**，需要**大于 21**；
- 这部分也记得写在**Written**报告中。

### (j) (3 分) （书面问题）比较 dot-product、multiplicative、additive attention 的优劣

- **要求**：
    1. 说出 dot-product 相比 multiplicative 的优点和缺点（各 1 点）；
    2. 说出 additive 相比 multiplicative 的优点和缺点（各 1 点）。

把这些回答写在**Written**报告里。

---

## Part 2: Analyzing NMT Systems (30 分)

这一部分**完全是书面分析题**。重点是观察某些翻译错误、分析原因和改进思路，以及手动计算 BLEU。

### (a) (12 分) 给定 6 个**已知翻译错误**的例子

- 对每个例子：
    1. **指出** NMT 翻译出现了什么错误；
    2. **可能的原因**（模型不足 or 特定语言现象）是什么；
    3. **提出一个改进方案**（可以是模型结构、超参数调整、更多训练数据、改 attention 等）。

### (b) (4 分) 观察你自己模型在 test set 上的输出

- 你在 part 1 训练后，生成的翻译保存在 `outputs/test_outputs.txt`。
- 需要**找出 2 处翻译错误**（并且和 part (a) 里给出的例子类型不同），对每个错误：
    1. 写下**源句**（在 `test.es`）、**参考译文**（在 `test.en`）、**模型译文**；
    2. 分析错误类型、可能原因、改进方案。

### (c) (14 分) 手动计算 BLEU 的题目

- 让你根据给出的公式(Modified n-gram precision, brevity penalty 等)对 1 个句子进行 BLEU 计算；
- 分别在有两个参考译文 vs. 只有一个参考译文时，对比两个候选翻译 c1、c2 的 BLEU 分数；
- 问你是否同意 BLEU 的判断、为什么单一参考翻译有局限、BLEU 相比人工评测的优缺点等：

1. (i) (5 分) **同时有两条参考译文** r1,r2r_1, r_2，计算 c1、c2 的 BLEU；谁得分更高，是否符合直觉？
2. (ii) (5 分) **只剩下一条**参考译文r1r_1，再次算 c1、c2 的 BLEU；结果如何，对比之前有何变化？
3. (iii) (2 分) **为何仅有单一参考**对 NMT 评价可能存在问题？
4. (iv) (2 分) BLEU vs 人工评测：**列举 2 个优点和 2 个缺点**。

将这些**详细计算过程与答案**写在**Written**报告里。

---

## 最终提交

作业需要**两次**提交到 GradeScope：

1. **Assignment 4[coding]**
    
    - 在你的 Azure VM 上执行 `collect_submission.sh`，会打包出 `assignment4.zip`；
    - 下载到本地后上传到 Gradescope。
    - 里面应该包含你写的**`nmt_model.py`、`model_embeddings.py`、`utils.py`** 等代码。
2. **Assignment 4[written]**
    
    - 把**所有文字分析回答**（包括 Part 1(g), Part 1(j), Part 2(a)(b)(c)）整理成一个 PDF（或学校要求的排版），上传到相应的 Written 作业入口。
    - 在 Gradescope 上"**tag 对应页面**"时，记得把每个题目对应到 PDF 中正确的页码，否则可能被扣分。

完成以上流程，你就完成了 CS224n Assignment 4。祝你一切顺利，得到理想的 BLEU 分数并顺利完成作业！
