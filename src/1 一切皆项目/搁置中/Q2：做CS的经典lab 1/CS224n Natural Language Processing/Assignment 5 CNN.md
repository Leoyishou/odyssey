---
date created: 2025-01-26
date modified: 2025-07-10
uid: 8f8e4e27-7ac0-4310-80c2-b950a9abb58d
---

这项实验主要让你设计、实现并训练一个基于字符级别的神经机器翻译系统，具体来说包括以下几个方面：

1. **字符级卷积编码器**：你需要用字符级的卷积神经网络（CNN）来替代传统的词汇查找方式生成词嵌入，这样可以更好地捕捉词内部的形态信息，并处理未登录词（）的问题。
    
2. **字符级LSTM解码器**：当词级解码器生成时，你将通过字符级的长短时记忆网络（LSTM）逐字符生成正确的目标词，从而提高翻译质量，特别是对于稀有词或形态变化丰富的语言。
    
3. **模型集成与训练调试**：你需要将上述模块集成到完整的NMT系统中，从数据预处理、模型搭建、训练到测试进行全面的调试和验证，包括在小数据集上验证模型过拟合以确认实现正确，再在实际数据上进行训练（可能需要数小时）。
    
4. **实验分析**：通过对比字符级模型和传统词级模型的输出及嵌入空间（例如利用TensorFlow的Embedding Projector），你将分析两种方法在处理词形变化和语义捕捉上的不同表现。
    

总的来说，这个实验旨在让你深入理解并实践如何利用子词（字符）信息来构建更健壮的机器翻译系统，从而应对词汇覆盖率、未登录词等问题，同时体验模块化设计、调试和大规模训练的全过程。

下面给你一个「**任务概览**」，帮助你理解 **CS224n: Assignment #5[updated]** 的整体要求，包括代码部分（coding）需要实现的功能，以及书面部分（written）需要回答的分析问题。和之前的作业类似，你将分别在 Gradescope 上提交 **Assignment 5[coding]** 和 **Assignment 5[written]**。注意，这个作业编码量相对大，并且我们提供的辅助检查（sanity-check）非常有限，你需要投入更多时间自测和调试。

---

## 作业整体简介

在这次作业中，你会在 **Assignment #4** 的基础上，改进并扩展 NMT（神经机器翻译）系统，实现**字符级（subword-level）**处理。具体来说，你会让模型在以下两处使用字符级信息：

1. **Character-based CNN Encoder**（Part 1）
   取代之前对词的简单 lookup embedding，改用一个卷积网络（Conv1D）来结合字符嵌入，从而得到每个单词的向量表示。
   - 模型结构包含：字符嵌入、卷积 + max-pooling、highway network、最后 dropout，得到单词级向量。
   - 理想情况下，这可以让模型在遇到罕见词、未登录词（OOV）时表现更好，因为可以学习到字符层面的拼写信息。

2. **Character-based LSTM Decoder**（Part 2）
   当主力的**词级解码器**输出 `<UNK>` 时，转而调用一个**字符级 LSTM** 来"拼出"目标词。
   - 训练时，字符解码器会对所有目标词进行训练（而非只对 `<UNK>` 词）。
   - 推断时，如果词解码器产生 `<UNK>`，就用字符解码器根据之前解码器的隐藏状态，逐字母生成目标词。

最后在 **Part 3** 里，你会对自己训练好的模型做一些分析，包括比较字符级嵌入和传统 Word2Vec 的差异，以及观察字符解码器在实际翻译时的表现。

注意：  

- 这次作业的**代码提示更少**，需要你自己做更多自测。
- 最后完整模型（Part 2 结束后）在 Azure GPU 上要训练 8～12 小时左右。
- 提交时一定要保留 `outputs/test_outputs.txt` 等文件，不要篡改它们；因为提交后自动评分会用到这些文件来验证你跑出的 BLEU 分数。

---

## 目录结构

从官方提供的代码看，主要涉及以下文件/模块：  

1. **`highway.py`**：你需要自己创建 `Highway` 模块（nn.Module）。
2. **`cnn.py`**：你需要自己创建 `CNN` 模块（nn.Module），完成字符嵌入经卷积和 max-pooling 的过程。
3. **`model_embeddings.py`**：你要在 `ModelEmbeddings` 中整合 CNN + Highway + Dropout 等操作，并在 forward() 中对一个 batch 的字符级输入完成处理。
4. **`char_decoder.py`**：字符级 LSTM 解码器的核心文件，包括 forward()、train_forward()、decode_greedy() 等函数。
5. **`nmt_model.py`**：最后将字级 encoder 和字级 decoder 集成到 NMT 模型结构里，一部分地方需要你改动（例如 Part 1(i)）
6. 其他：`vocab.py`、`utils.py`、`run.sh`、若干脚本和数据文件夹等。

---

## Part 1: Character-based Convolutional Encoder for NMT (36 points)

### 1(a)～1(d)：理论与书面问题 (8 分)

1. **(a)** 1D 卷积网络对于可变长度输入的适用性？为什么 RNN 与 CNN 在这方面有何不同？（1 分）
2. **(b)** 在给定 kernel size = 5 时，决定 1D convolution 的 padding 大小，并简要说明理由。（2 分）
3. **(c)** Highway 网络的作用及初始 bias 的设定。（3 分）
4. **(d)** Transformer encoder 相对于 LSTM encoder 的两个优势。（2 分）

这些都是**简答题**，写在 Written 提交文件里即可。

### 1(e)～1(j)：编码实现 (28 分)

1. **(e) (4 分)** 实现 `to_input_tensor_char()`（在 `vocab.py` 中）的功能：
   - 把一个 batch 的句子转换成字符级索引，并对每个单词 pad 到同样长度（batch 内最長）。
   - 输出的张量尺寸要是 `(max_sentence_length, batch_size, max_word_length)`.  
   - 利用 `words2charindices()` 和 `pad_sents_char()` 来实现。
   - `python sanity_check.py 1e` 做初步检查。

2. **(f) (4 分) + (g) (4 分)** 分别实现 `Highway` 和 `CNN` 两个模块（都是 `nn.Module`）：
   - **`highway.py`**：需要写 `__init__()` 和 `forward()`，用 2 个线性层（投影 + gate），然后作 \(x_{highway} = x_{gate} \circ x_{proj} + (1 - x_{gate}) \circ x_{conv\_out} \)。
   - **`cnn.py`**：需要写 `__init__()` 和 `forward()`，使用 PyTorch 的 `nn.Conv1d`（kernel_size=5，padding=1）并后接 max-pooling。
   - 题目要求你自己写一些**测试**来验证功能，然后在报告中**说明你测试了哪些情况**及**为什么足够证明**模块正确（每个模块 4 分主要看你写的测试思路描述）。

3. **(h) (10 分)** 在 `model_embeddings.py` 中完成 `ModelEmbeddings` 类：
   - 整合上述 CNN、Highway 等到一起，把输入的 `(sentence_length, batch_size, max_word_length)` 转成 `(sentence_length, batch_size, e_word)` 的词向量。
   - 记得加 dropout(0.3)。
   - 你可以用 `python sanity_check.py 1h` 做基本检查。

4. **(i) (4 分)** 在 `nmt_model.py` 中，把原来的词表 lookup embedding 替换成**字符级** `ModelEmbeddings`，使 forward() 使用你的新编码方式。
5. **(j) (2 分)** 本地上用脚本 `sh run.sh train_local_q1` + `sh run.sh test_local_q1` 确认小规模数据可 100% 拟合（loss→0，BLEU→99+）。若达不到则需要 debug。

---

## Part 2: Character-based LSTM Decoder for NMT (26 points)

这里，你会在**解码器**层面，如果产生 `<UNK>` 时，就调用一个字符级 LSTM 解码器来"逐字母"生成目标语言的单词：

1. **(a)** (4 分) 在 `char_decoder.py` 里写 `forward()`：
   - 输入字符索引序列 `[x1,..., x_n]`，和初始 (h0, c0)（来自主模型的 combined output），输出各时刻的 logits `[s1,..., s_n]`（对应 `V_char` 大小），以及最终 (h_n, c_n)。
   - `python sanity_check.py 2a` 测试。

2. **(b)** (5 分) 在 `char_decoder.py` 里写 `train_forward()`：
   - 计算字符级解码器的 cross-entropy loss，对应公式 \(\sum_{t=1}^n -\log p_t(x_{t+1})\)（对一个 batch 作和）。
   - 用 `nn.CrossEntropyLoss`。
   - `python sanity_check.py 2b` 测试。

3. **(c)** (8 分) 在 `char_decoder.py` 里写 `decode_greedy()`：
   - 实现算法 1(伪代码) 的贪心生成，但要支持 batch；可以一次循环到 max_length，再在事后截断 `<END>` 之后的字符。
   - `python sanity_check.py 2c` 测试。

4. **(d)** (3 分) 同样先本地做小规模测试 `sh run.sh train_local_q2` + `sh run.sh test_local_q2`，期望看到训练集和 dev 上的 loss→0、perplexity→1，测试集 BLEU>99。
5. **(e)** (6 分) 在 Azure VM 上完整训练 + 测试：
   - `sh run.sh train` 大概需要 8～12 小时。
   - `sh run.sh test` 输出最终的 BLEU 分数，你需要在报告里写下该数值。
   - BLEU < 35 则给 0 分，≥35 给至少 2 分，≥36 给满分 6 分。
   - 训练完后，记得保留 `outputs/test_outputs.txt` 交作业时要一并上传。

---

## Part 3: Analyzing NMT Systems (8 points)

### (a) (2 分)

观察西语动词 "traducir" (意为 "to translate") 的多种变形（比如 `traduzco`, `traduces`, `traduce`, `traduzca`, `traduzcas`等），查找哪些出现在 5 万词表（vocab.json）里。  

- 哪些子形态被包含？哪些没有？
- 为什么对纯词级 NMT 不利？
- 字符级 NMT 如何更好处理这个问题？

### (b) (4 分)

与 Assignment1/2 类似，这里让你对比**Word2Vec 词向量**与**CharCNN 的词向量**在词语最近邻检索时有何区别。

1. **(b)(i)** 在 TF Embedding Projector 上查看 Word2Vec All 预训练向量，对给定词(`financial`, `neuron`, `Francisco`, `naturally`, `expectation`)分别找到最近邻并截图。
2. **(b)(ii)** 用提供的 `character-embeddings.txt` + `metadata.txt` 上传到 TF Embedding Projector，并同样检索相同单词的最近邻。
3. **(b)(iii)**（3 分）对比两者：为什么 Word2Vec 更偏"语义相似"，CharCNN 可能更偏"形态 / 拼写 / 字符相似"？简要解释算法原理导致的区别。

### (c) (2 分)

对比你**Assignment4**里的纯词级模型翻译 `outputs/test_outputs_a4.txt` 与这次**字符级解码**模型翻译 `outputs/test_outputs.txt`：  

- 找到一处 `<UNK>` 被 CharDecoder 成功替换为合适的单词，以及一处 `<UNK>` 被不正确地替换。
- 写下原句（西语）、参考译文、Assignment4 的翻译（标出 `<UNK>`），Assignment5 的翻译（标出生成的单词），并简要分析该现象的可能原因。

---

## 最终提交

你需要**两次**提交到 Gradescope：

1. **Assignment 5[coding]**  
   1. 确保你目录里有这几个输出文件：
      - `outputs/test_outputs.txt`  
      - `outputs/test_outputs_local_q1.txt`  
      - `outputs/test_outputs_local_q2.txt`  
   2. 运行 `collect_submission.sh` 得到 `assignment5.zip`，上传到 Gradescope。
   3. 查看公共测试结果（BLEU 等），确认无问题。
2. **Assignment 5[written]**  
   - 把 Part 1(a)～(d), Part 2(e) 的 BLEU、Part 3(a)～(c) 等所有书面问题回答整理成一个 PDF 并上传。
   - 记得在 Gradescope 上给每个题目正确关联到 PDF 页码。

如此，你就完成了 **CS224n: Assignment #5** 的全部要求。**祝你一切顺利**，在字符级 NMT 的探索中获得理想 BLEU 分数和更多收获！
