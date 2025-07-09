---
comment_id: 4e733ee6
date created: 2025-01-25
date modified: 2025-02-06
draw: null
title: Assignment 4 RNNs
---
- [实操](#%E5%AE%9E%E6%93%8D)
	- [1. 本地开发和调试](#1.%20%E6%9C%AC%E5%9C%B0%E5%BC%80%E5%8F%91%E5%92%8C%E8%B0%83%E8%AF%95)
	- [2. 在 Azure GPU 虚拟机上部署和训练](#2.%20%E5%9C%A8%20Azure%20GPU%20%E8%99%9A%E6%8B%9F%E6%9C%BA%E4%B8%8A%E9%83%A8%E7%BD%B2%E5%92%8C%E8%AE%AD%E7%BB%83)
	- [3. 模型测试和后续工作](#3.%20%E6%A8%A1%E5%9E%8B%E6%B5%8B%E8%AF%95%E5%92%8C%E5%90%8E%E7%BB%AD%E5%B7%A5%E4%BD%9C)
	- [4. 小贴士](#4.%20%E5%B0%8F%E8%B4%B4%E5%A3%AB)
- [理论](#%E7%90%86%E8%AE%BA)
	- [Assignment 概览](#Assignment%20%E6%A6%82%E8%A7%88)
	- [Part 1: Neural Machine Translation with RNNs (45 分)](#Part%201:%20Neural%20Machine%20Translation%20with%20RNNs%20(45%20%E5%88%86))
		- [(a) (2 分) 在 `utils.py` 里实现 `pad_sents` 函数](#(a)%20(2%20%E5%88%86)%20%E5%9C%A8%20%60utils.py%60%20%E9%87%8C%E5%AE%9E%E7%8E%B0%20%60pad_sents%60%20%E5%87%BD%E6%95%B0)
		- [(b) (3 分) 在 `model_embeddings.py` 里实现 `__init__` 函数](#(b)%20(3%20%E5%88%86)%20%E5%9C%A8%20%60model_embeddings.py%60%20%E9%87%8C%E5%AE%9E%E7%8E%B0%20%60__init__%60%20%E5%87%BD%E6%95%B0)
		- [(c) (4 分) 在 `nmt_model.py` 的 `__init__` 中初始化各层](#(c)%20(4%20%E5%88%86)%20%E5%9C%A8%20%60nmt_model.py%60%20%E7%9A%84%20%60__init__%60%20%E4%B8%AD%E5%88%9D%E5%A7%8B%E5%8C%96%E5%90%84%E5%B1%82)
		- [(d) (8 分) 在 `nmt_model.py` 中实现 `encode()` 函数](#(d)%20(8%20%E5%88%86)%20%E5%9C%A8%20%60nmt_model.py%60%20%E4%B8%AD%E5%AE%9E%E7%8E%B0%20%60encode()%60%20%E5%87%BD%E6%95%B0)
		- [(e) (8 分) 在 `nmt_model.py` 中实现 `decode()` 函数](#(e)%20(8%20%E5%88%86)%20%E5%9C%A8%20%60nmt_model.py%60%20%E4%B8%AD%E5%AE%9E%E7%8E%B0%20%60decode()%60%20%E5%87%BD%E6%95%B0)
		- [(f) (10 分) 在 `nmt_model.py` 中实现 `step()` 函数](#(f)%20(10%20%E5%88%86)%20%E5%9C%A8%20%60nmt_model.py%60%20%E4%B8%AD%E5%AE%9E%E7%8E%B0%20%60step()%60%20%E5%87%BD%E6%95%B0)
		- [(g) (3 分) （书面问题）关于 `generate_sent_masks()` 生成的 `enc_masks`](#(g)%20(3%20%E5%88%86)%20%EF%BC%88%E4%B9%A6%E9%9D%A2%E9%97%AE%E9%A2%98%EF%BC%89%E5%85%B3%E4%BA%8E%20%60generate_sent_masks()%60%20%E7%94%9F%E6%88%90%E7%9A%84%20%60enc_masks%60)
		- [(i) (4 分) 训练完成后，测试并报告 BLEU 分数](#(i)%20(4%20%E5%88%86)%20%E8%AE%AD%E7%BB%83%E5%AE%8C%E6%88%90%E5%90%8E%EF%BC%8C%E6%B5%8B%E8%AF%95%E5%B9%B6%E6%8A%A5%E5%91%8A%20BLEU%20%E5%88%86%E6%95%B0)
		- [(j) (3 分) （书面问题）比较 dot-product、multiplicative、additive attention 的优劣](#(j)%20(3%20%E5%88%86)%20%EF%BC%88%E4%B9%A6%E9%9D%A2%E9%97%AE%E9%A2%98%EF%BC%89%E6%AF%94%E8%BE%83%20dot-product%E3%80%81multiplicative%E3%80%81additive%20attention%20%E7%9A%84%E4%BC%98%E5%8A%A3)
	- [Part 2: Analyzing NMT Systems (30 分)](#Part%202:%20Analyzing%20NMT%20Systems%20(30%20%E5%88%86))
		- [(a) (12 分) 给定 6 个**已知翻译错误**的例子](#(a)%20(12%20%E5%88%86)%20%E7%BB%99%E5%AE%9A%206%20%E4%B8%AA**%E5%B7%B2%E7%9F%A5%E7%BF%BB%E8%AF%91%E9%94%99%E8%AF%AF**%E7%9A%84%E4%BE%8B%E5%AD%90)
		- [(b) (4 分) 观察你自己模型在 test set 上的输出](#(b)%20(4%20%E5%88%86)%20%E8%A7%82%E5%AF%9F%E4%BD%A0%E8%87%AA%E5%B7%B1%E6%A8%A1%E5%9E%8B%E5%9C%A8%20test%20set%20%E4%B8%8A%E7%9A%84%E8%BE%93%E5%87%BA)
		- [(c) (14 分) 手动计算 BLEU 的题目](#(c)%20(14%20%E5%88%86)%20%E6%89%8B%E5%8A%A8%E8%AE%A1%E7%AE%97%20BLEU%20%E7%9A%84%E9%A2%98%E7%9B%AE)
	- [最终提交](#%E6%9C%80%E7%BB%88%E6%8F%90%E4%BA%A4)

## 实操

### 1. 本地开发和调试

**(1) 设置本地虚拟环境**

- 确保你已经安装了 Anaconda（或 Miniconda）。
- 在项目根目录下执行下面的命令来创建并激活本地虚拟环境：

    ```bash
    conda env create --file local_env.yml
    conda activate <your_env_name>  # 替换 <your_env_name> 为 env.yml 中指定的环境名称
    ```

**(2) 生成词汇表**

- 在终端中运行下面的命令来生成所需的词汇表文件：

    ```bash
    sh run.sh vocab
    ```

    该命令会根据数据预处理，生成训练和测试过程中需要用到的词汇表。

**(3) 本地调试训练代码**

- 为了避免在 GPU 上浪费宝贵的时间，先在本地运行简化版的训练脚本。运行：

    ```bash
    sh run.sh train_local
    ```

- 观察程序运行至少到 iter 10 或 iter 20，确保代码能够顺利执行且不会崩溃。如果在这一阶段发现问题，可以及时调试修改。

---

### 2. 在 Azure GPU 虚拟机上部署和训练

**(4) 准备并上传代码到 Azure VM**

- 按照 CS224n Azure Guide 和 "Managing Code Deployment to a VM" 指南配置你的 Azure 虚拟机（确保分配了 GPU 资源）。
- 使用 `scp` 或其它文件传输工具将你本地测试通过的代码上传到 VM 上。

**(5) 安装 GPU 版依赖**

- 在 VM 上进入你的项目目录，运行以下命令安装 GPU 环境所需的依赖包：

    ```bash
    pip install -r gpu_requirements.txt
    ```

**(6) 利用 tmux 运行长时间训练任务**

- 为防止 SSH 连接断开导致训练中断，建议使用 tmux。首先创建一个 tmux 会话：

    ```bash
    tmux new -s nmt
    ```

- 在 tmux 会话内，启动正式的训练（大约需要 4 小时）：

    ```bash
    sh run.sh train
    ```

- 如果需要临时退出 tmux 会话而不终止任务，可以按下 `Ctrl+B` 后再按 `D`（或者直接输入 `tmux detach`）。

**(7) 监控训练进程**

- 你可以随时通过下面的命令重新连接到该会话以查看训练日志：

    ```bash
    tmux attach -t nmt
    ```

---

### 3. 模型测试和后续工作

**(8) 测试模型**

- 当训练完成后（约 4 小时后），在同一个 tmux 会话或新的终端中运行：

    ```bash
    sh run.sh test
    ```

- 该命令会加载训练好的模型，对测试数据进行翻译，并计算 BLEU 分数。翻译结果一般会保存在 `outputs/test_outputs.txt` 文件中。

**(9) 分析和撰写报告**

- 根据测试结果，查阅输出文件，回答 Assignment #4 中关于 NMT 系统分析的书面题目。
- 确保代码部分和书面答案部分都按照要求打包提交到 GradeScope。

---

### 4. 小贴士

- **本地开发先行**：在正式上传到 VM 前，务必在本地充分调试代码，确保基本功能（如 pad_sents、encode、decode、step 等函数）正确实现。
- **合理管理资源**：在不使用 VM 时及时关闭以节省 GPU 使用时间和费用。
- **参考文档**：如果遇到任何问题，建议查阅 CS224n 提供的 Azure Guide、Practical Guide to VMs，以及课程网站上提供的相关文档。

按照以上步骤，你就可以一步步地跑起 Assignment #4 了。祝你训练顺利，取得好成绩！

## 理论

### Assignment 概览

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

### Part 1: Neural Machine Translation with RNNs (45 分)

在此部分，你需要主要在 **`nmt_model.py`** 和 **`model_embeddings.py`**（以及一些辅助文件，如 `utils.py`）里完成对应的函数。完成后，在你的代码环境里跑 `sh run.sh train_local` 或者在 Azure VM 上跑 `sh run.sh train` 进行训练（大约需 4 小时 GPU 时间）。

#### (a) (2 分) 在 `utils.py` 里实现 `pad_sents` 函数

- **功能**：对一个 batch 中的句子进行补齐（padding），保证它们有相同长度，方便后续张量运算。
- 你只需在 `pad_sents` TODO 处完成并返回补齐后的结果。

#### (b) (3 分) 在 `model_embeddings.py` 里实现 `__init__` 函数

- **功能**：分别创建**源语言（source）**和**目标语言（target）**的嵌入层 (Embedding layer)。
- 作业要求**不直接使用** `nn.Embedding` 的高层封装，而是要自己写或至少自己初始化相关参数。

#### (c) (4 分) 在 `nmt_model.py` 的 `__init__` 中初始化各层

- **功能**：
    - 建立 NMT 模型需要的层（**LSTM、投影层、dropout**等）
    - 调用 `ModelEmbeddings` 来创建 embeddings
    - 这些层会在后面的 `encode()`、`decode()`、`step()` 等函数里被调用。

#### (d) (8 分) 在 `nmt_model.py` 中实现 `encode()` 函数

- **功能**：
    1. 把补齐后的源语句转换为张量 X；
    2. 送入双向 LSTM（Bi-LSTM），得到 encoder hidden states hench_{enc}；
    3. 线性映射 encoder 最后的 hidden state/cell state 用于初始化 decoder 的 hdec0h_{dec}^0 和 cdec0c_{dec}^0；
- 可以用 `python sanity_check.py 1d` 做一个初步测试。

#### (e) (8 分) 在 `nmt_model.py` 中实现 `decode()` 函数

- **功能**：
    1. 把目标语言词向量（以及前一步的输出向量）拼接，形成 decoder 的输入；
    2. 多步（time step）调用 `step()` 函数，直到处理完目标句所有 token；
    3. 收集 decoder 的输出，用来计算预测分布等。
- 可以用 `python sanity_check.py 1e` 做初步测试。

#### (f) (10 分) 在 `nmt_model.py` 中实现 `step()` 函数

- **功能**：
    1. Decoder LSTM 单步更新 (h_t, c_t)
    2. 计算对所有 encoder hidden states 的 attention 分数 e_t，并做 softmax 得到 attention 分布 α_t
    3. 根据 α_t 做加权求和得到 context 向量 a_t
    4. 将 a_t 和 h_t 拼接、通过一个线性 + tanh + dropout 得到 combined output o_t
    5. 用 `o_t` 最终预测一个词分布
- 可以用 `python sanity_check.py 1f` 做初步测试。

#### (g) (3 分) （书面问题）关于 `generate_sent_masks()` 生成的 `enc_masks`

- 题目让你用**三四句话**说明：
    1. mask 在 attention 计算中起到什么作用？
    2. 为什么在计算注意力分数时，必须这样用 mask 去屏蔽 padding token？

- **mask 的作用**：在 attention 计算中，mask 用于屏蔽输入中那些仅用于填充、没有实际语义信息的位置，确保模型在计算注意力分布时只关注有效的词汇。
- **屏蔽 padding token 的必要性**：因为 padding token 并不携带实际信息，如果不屏蔽，这些位置可能会获得不合理的注意力得分，导致 softmax 后分配非零的概率，从而干扰上下文向量的计算和后续预测。因此，通过将 padding token 对应的注意力得分置为负无穷，保证它们在 softmax 后的概率为 0，使模型只聚焦于真正有用的输入信息。

#### (i) (4 分) 训练完成后，测试并报告 BLEU 分数

- **做法**：
    1. 在 Azure VM 上完整训练（`sh run.sh train`），大约 4 小时；
    2. 训练完后执行 `sh run.sh test` 计算 BLEU；
    3. 在报告中**写出最终的 BLEU**，需要**大于 21**；
- 这部分也记得写在**Written**报告中。

#### (j) (3 分) （书面问题）比较 dot-product、multiplicative、additive attention 的优劣

- **要求**：
    1. 说出 dot-product 相比 multiplicative 的优点和缺点（各 1 点）；
    2. 说出 additive 相比 multiplicative 的优点和缺点（各 1 点）。

把这些回答写在**Written**报告里。

---

### Part 2: Analyzing NMT Systems (30 分)

这一部分**完全是书面分析题**。重点是观察某些翻译错误、分析原因和改进思路，以及手动计算 BLEU。

#### (a) (12 分) 给定 6 个**已知翻译错误**的例子

- 对每个例子：
    1. **指出** NMT 翻译出现了什么错误；
    2. **可能的原因**（模型不足 or 特定语言现象）是什么；
    3. **提出一个改进方案**（可以是模型结构、超参数调整、更多训练数据、改 attention 等）。

#### (b) (4 分) 观察你自己模型在 test set 上的输出

- 你在 part 1 训练后，生成的翻译保存在 `outputs/test_outputs.txt`。
- 需要**找出 2 处翻译错误**（并且和 part (a) 里给出的例子类型不同），对每个错误：
    1. 写下**源句**（在 `test.es`）、**参考译文**（在 `test.en`）、**模型译文**；
    2. 分析错误类型、可能原因、改进方案。

#### (c) (14 分) 手动计算 BLEU 的题目

- 让你根据给出的公式(Modified n-gram precision, brevity penalty 等)对 1 个句子进行 BLEU 计算；
- 分别在有两个参考译文 vs. 只有一个参考译文时，对比两个候选翻译 c1、c2 的 BLEU 分数；
- 问你是否同意 BLEU 的判断、为什么单一参考翻译有局限、BLEU 相比人工评测的优缺点等：

1. (i) (5 分) **同时有两条参考译文** r1,r2r_1, r_2，计算 c1、c2 的 BLEU；谁得分更高，是否符合直觉？
2. (ii) (5 分) **只剩下一条**参考译文r1r_1，再次算 c1、c2 的 BLEU；结果如何，对比之前有何变化？
3. (iii) (2 分) **为何仅有单一参考**对 NMT 评价可能存在问题？
4. (iv) (2 分) BLEU vs 人工评测：**列举 2 个优点和 2 个缺点**。

将这些**详细计算过程与答案**写在**Written**报告里。

---

### 最终提交

作业需要**两次**提交到 GradeScope：

1. **Assignment 4[coding]**
    - 在你的 Azure VM 上执行 `collect_submission.sh`，会打包出 `assignment4.zip`；
    - 下载到本地后上传到 Gradescope。
    - 里面应该包含你写的**`nmt_model.py`、`model_embeddings.py`、`utils.py`** 等代码。
2. **Assignment 4[written]**
    - 把**所有文字分析回答**（包括 Part 1(g), Part 1(j), Part 2(a)(b)(c)）整理成一个 PDF（或学校要求的排版），上传到相应的 Written 作业入口。
    - 在 Gradescope 上"**tag 对应页面**"时，记得把每个题目对应到 PDF 中正确的页码，否则可能被扣分。

完成以上流程，你就完成了 CS224n Assignment BLEU 分数并顺利完成作业！
