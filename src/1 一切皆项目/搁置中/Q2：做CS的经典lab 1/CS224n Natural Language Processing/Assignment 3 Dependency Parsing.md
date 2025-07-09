---
comment_id: d912d79f
date created: 2025-01-24
date modified: 2025-05-13
draw: null
title: Assignment 3 Dependency Parsing
---
- [理论](#%E7%90%86%E8%AE%BA)
	- [Part 1: Machine Learning & Neural Networks（8 分）](#Part%201:%20Machine%20Learning%20&%20Neural%20Networks%EF%BC%888%20%E5%88%86%EF%BC%89)
	- [Part 2: Neural Transition-Based Dependency Parsing（44 分）](#Part%202:%20Neural%20Transition-Based%20Dependency%20Parsing%EF%BC%8844%20%E5%88%86%EF%BC%89)
		- [训练数据](#%E8%AE%AD%E7%BB%83%E6%95%B0%E6%8D%AE)
		- [2(a) 理解并列出完整的转移序列 (4 分)](#2(a)%20%E7%90%86%E8%A7%A3%E5%B9%B6%E5%88%97%E5%87%BA%E5%AE%8C%E6%95%B4%E7%9A%84%E8%BD%AC%E7%A7%BB%E5%BA%8F%E5%88%97%20(4%20%E5%88%86))
		- [2(b) 解析一个 n 词句子，需要多少步？(2 分)](#2(b)%20%E8%A7%A3%E6%9E%90%E4%B8%80%E4%B8%AA%20n%20%E8%AF%8D%E5%8F%A5%E5%AD%90%EF%BC%8C%E9%9C%80%E8%A6%81%E5%A4%9A%E5%B0%91%E6%AD%A5%EF%BC%9F(2%20%E5%88%86))
		- [2(c) 实现 `PartialParse` 的两个函数 (6 分)](#2(c)%20%E5%AE%9E%E7%8E%B0%20%60PartialParse%60%20%E7%9A%84%E4%B8%A4%E4%B8%AA%E5%87%BD%E6%95%B0%20(6%20%E5%88%86))
		- [2(d) 实现 `minibatch_parse` 函数 (8 分)](#2(d)%20%E5%AE%9E%E7%8E%B0%20%60minibatch_parse%60%20%E5%87%BD%E6%95%B0%20(8%20%E5%88%86))
		- [2(e) 训练并实现神经网络依存解析器 (12 分)](#2(e)%20%E8%AE%AD%E7%BB%83%E5%B9%B6%E5%AE%9E%E7%8E%B0%E7%A5%9E%E7%BB%8F%E7%BD%91%E7%BB%9C%E4%BE%9D%E5%AD%98%E8%A7%A3%E6%9E%90%E5%99%A8%20(12%20%E5%88%86))
		- [2(f) 错误解析分析 (12 分)](#2(f)%20%E9%94%99%E8%AF%AF%E8%A7%A3%E6%9E%90%E5%88%86%E6%9E%90%20(12%20%E5%88%86))
- [实操](#%E5%AE%9E%E6%93%8D)
	- [1. 准备代码目录](#1.%20%E5%87%86%E5%A4%87%E4%BB%A3%E7%A0%81%E7%9B%AE%E5%BD%95)
	- [2. 创建并激活 Python 环境](#2.%20%E5%88%9B%E5%BB%BA%E5%B9%B6%E6%BF%80%E6%B4%BB%20Python%20%E7%8E%AF%E5%A2%83)
		- [2.1 安装 Miniconda/Anaconda](#2.1%20%E5%AE%89%E8%A3%85%20Miniconda/Anaconda)
		- [2.2 创建虚拟环境 & 安装 PyTorch](#2.2%20%E5%88%9B%E5%BB%BA%E8%99%9A%E6%8B%9F%E7%8E%AF%E5%A2%83%20&%20%E5%AE%89%E8%A3%85%20PyTorch)
	- [3. 测试 & 开发：`parser_transitions.py`](#3.%20%E6%B5%8B%E8%AF%95%20&%20%E5%BC%80%E5%8F%91%EF%BC%9A%60parser_transitions.py%60)
	- [4. 训练模型：`parser_model.py` & `run.py`](#4.%20%E8%AE%AD%E7%BB%83%E6%A8%A1%E5%9E%8B%EF%BC%9A%60parser_model.py%60%20&%20%60run.py%60)
		- [4.1 模型实现](#4.1%20%E6%A8%A1%E5%9E%8B%E5%AE%9E%E7%8E%B0)
		- [4.2 开始训练](#4.2%20%E5%BC%80%E5%A7%8B%E8%AE%AD%E7%BB%83)
	- [5. 查看 & 提交结果](#5.%20%E6%9F%A5%E7%9C%8B%20&%20%E6%8F%90%E4%BA%A4%E7%BB%93%E6%9E%9C)
	- [6. 可能出现的问题与排查](#6.%20%E5%8F%AF%E8%83%BD%E5%87%BA%E7%8E%B0%E7%9A%84%E9%97%AE%E9%A2%98%E4%B8%8E%E6%8E%92%E6%9F%A5)
	- [7. 总结](#7.%20%E6%80%BB%E7%BB%93)
	- [训练过程](#%E8%AE%AD%E7%BB%83%E8%BF%87%E7%A8%8B)
		- [1. 初始化与数据加载](#1.%20%E5%88%9D%E5%A7%8B%E5%8C%96%E4%B8%8E%E6%95%B0%E6%8D%AE%E5%8A%A0%E8%BD%BD)
		- [2. 训练过程（TRAINING）](#2.%20%E8%AE%AD%E7%BB%83%E8%BF%87%E7%A8%8B%EF%BC%88TRAINING%EF%BC%89)
		- [3. 测试阶段（TESTING）](#3.%20%E6%B5%8B%E8%AF%95%E9%98%B6%E6%AE%B5%EF%BC%88TESTING%EF%BC%89)
		- [总结](#%E6%80%BB%E7%BB%93)

## 理论

### Part 1: Machine Learning & Neural Networks（8 分）

这是一些**基础理论题**，主要围绕 Adam 优化器和 Dropout 这两种神经网络技术展开。

1. **Adam Optimizer (4分)**  
	(a)(i) 关于动量的直觉:
	- 引入动量可以通过对梯度做"滑动平均",平滑掉单次梯度中的噪声,使参数更新更稳定。
	- 更稳定的更新方向让模型能沿着一致的方向前进,避免在优化过程中来回震荡,从而更容易找到最优解。
	

	(a)(ii) 关于自适应学习率的直觉:

	- 用 $\sqrt{v}$ 缩放是为了自动调节不同参数的更新步长,其中 $v$ 记录了梯度平方的滑动平均。
	- 梯度经常较小的参数会得到较大更新(因为除以小的 $\sqrt{v}$);梯度经常较大的参数会得到较小更新(因为除以大的 $\sqrt{v}$)。
	- 这种自适应机制让每个参数都能以合适的速度学习,避免某些参数更新过快或过慢,使训练更稳定高效。

2. **Dropout (4分)**  
	(b)(i) 系数关系:  
	$\gamma = \frac{1}{1 - p_{\text{drop}}}$  
	原因:为了保证经过dropout后的输出期望值等于原始输出,需要用这个系数来放大保留的神经元输出。

	(b)(ii) 训练与预测时的区别:

	- 训练时使用dropout是为了让网络学会在缺少部分信息时也能正常工作,防止过拟合。
	- 预测时不使用dropout是为了利用网络的全部能力,获得最稳定的结果。因为训练时已经通过缩放保证了输出期望不变,所以可以直接使用完整网络。

这部分你需要**写在书面报告（Assignment 3 [written]）里**，回答上述问题（只需简要解释直觉即可）。

下面给出对这两个问题的简明解答，用简单易懂的语言说明直觉和原理：

- **为什么引入 `m`(动量)可以减少更新的波动?**  
    在随机梯度下降中,每次计算的梯度都有一定的噪声,可能方向不稳定。通过对梯度做一个"滑动平均"(即用 `m` 来记录过去多个梯度的平均),单次梯度中的噪声就会被平滑掉,得到的平均梯度变化更平稳。
    
- **为什么这种"低方差"更新可以帮助训练更稳定地收敛?**  
    当更新的方向不再因为单个样本的噪声而剧烈波动时,模型参数在更新过程中就能沿着更一致的方向前进,避免在"崎岖"的损失曲面上跳来跳去,从而更稳定地接近最优解。

- **为什么用 $\sqrt{v}$ 来缩放更新?**  
    Adam 会维护一个 $v$ 来记录梯度平方的滑动平均,代表各个参数的梯度幅度。如果直接用梯度来更新,不同参数的更新步长可能差异很大。用 $\sqrt{v}$ 来缩放,每个参数的更新步长就会自动调节:
    - 对于梯度较大(变化快)的参数,$v$ 较大,因此更新步长就会缩小;
    - 对于梯度较小(变化平缓)的参数,$v$ 较小,因此更新步长相对较大。
- **哪些参数会得到更大或更小的更新?**
    - **较大更新:** 当一个参数的梯度长期都比较小,即 $v$ 小,这样除以 $\sqrt{v}$ 后,更新步长相对变大。
    - **较小更新:** 当一个参数的梯度经常很大,即 $v$ 大,那么更新时除以较大的 $\sqrt{v}$ 后,更新步长就会变小。
- **这样做对学习有什么好处?**  
    这种自适应的更新方式可以使每个参数都按照其"敏感性"进行调节:
    - 对于容易剧烈变化的参数,自动减小步长,避免"跳过"最优点;
    - 对于变化较平缓的参数,自动增大步长,帮助快速收敛。
        总的来说,这能使整个训练过程更稳定、更高效,避免某些参数更新过大或过小导致训练不平衡。
- **(b)(i) 求出系数 $\gamma$ 与丢弃概率 $p_{\text{drop}}$ 的关系**

	在 Dropout 中,我们对隐藏层 $h$ 的每个神经元随机生成一个 0/1 的掩码 $d$,其中每个元素为 1 的概率为 $1 - p_{\text{drop}}$,为 0 的概率为 $p_{\text{drop}}$。为了保证经过 Dropout 之后的输出 $h_{\text{drop}} = \gamma \, d \odot h$ 的期望值仍然等于原始的 $h$,我们要求:

	$\mathbb{E}[h_{\text{drop}}]= \gamma \, \mathbb{E}[d]\, h = h$

	

	而由于 $\mathbb{E}[d]= 1 - p_{\text{drop}}$,所以我们必须有:

	

	$\gamma \, (1 - p_{\text{drop}}) = 1 \quad \Longrightarrow \quad \gamma = \frac{1}{1 - p_{\text{drop}}}$

	
	**简要说明**:这意味着我们在训练时丢弃一部分神经元,为了保持整体输出水平不变,需要将剩下的神经元的输出放大 $\frac{1}{1 - p_{\text{drop}}}$ 倍。

- **(b)(ii) 为什么训练时用 Dropout,预测时不使用?**
	- **训练时使用 Dropout:**  
	    想象一下,训练时我们随机"关掉"部分神经元,这就像让网络中的每个神经元都不能总是依赖其它固定的神经元,而是需要学会在缺少一些信息的情况下也能工作。这种"打乱"会迫使网络学到更稳健、分布更均匀的特征,从而有效减少过拟合。
	    
	- **预测/推断时不使用 Dropout:**  
	    在预测时,我们希望利用整个网络的全部能力获得最稳定的结果,因此不再随机丢弃神经元,而是使用全部神经元进行计算。因为在训练期间我们已经通过缩放(即乘以 $\gamma = \frac{1}{1 - p_{\text{drop}}}$)来保证整体输出均值一致,预测时直接用全网络可以得到更准确、确定的结果。

	**形象的比喻**:
	- 训练时使用 Dropout 就像在足球比赛中让部分球员临时下场训练,这样其他球员就不得不学会独立作战,从而提高整体团队的应变能力;
	- 而在正式比赛(预测时),你当然希望所有球员都上场,发挥团队的全部实力来争取胜利。

---

### Part 2: Neural Transition-Based Dependency Parsing（44 分）

这一部分是整个作业的核心：**实现一个基于神经网络的依存句法解析器**，并分析一些错误解析案例。

- **输入特征改用嵌入向量** (embedding)。
    - 例如，把当前栈顶 2-3 个单词、缓冲区前面 2-3 个单词等，都变成对应的词向量。
    - 同时可能还会把它们的词性、已经建立好的依存标签等也映射成 embedding。
    - 把所有这些向量**拼接 (concatenate)** 在一起，作为神经网络的输入。
- **神经网络计算**
    - 常见做法：输入层 → 若干隐藏层 (多层感知器 MLP) → 输出层 (softmax)。
    - 输出层会给出对三个动作 (Shift / Left-Arc / Right-Arc) 以及其对应依存标签的概率分布。
    - 训练时，用交叉熵损失 (cross entropy) + 反向传播 (backpropagation) 来更新网络参数，包括嵌入矩阵和全连接层。
- **贪心解码**
    - 在真实解析时，我们从初始状态开始，**每一步都选置信度最高（[softmax](softmax.md)后的最大值）的动作**，直到到达终止状态，从而得到依存树。
    

#### 训练数据

- 第二列 FROM 就是原句子
- 第八列 DEPREL 就是专家标注出的金标准（**真实的依存树**）
- 我们的神经网络依存解析器的目标是预测出一系列状态机转换操作，并使得**预测的依存树**和真实的依存树之间的 loss 最小

```markdown
| ID | FORM      | LEMMA    | UPOS  | XPOS | FEATS | HEAD | DEPREL  | DEPS | MISC |
|----|-----------|----------|-------|------|-------|------|---------|------|------|
| 1  | I         | I        | PRON  | PRP  | _     | 2    | nsubj   | _    | _    |
| 2  | parsed    | parse    | VERB  | VBD  | _     | 0    | root    | _    | _    |
| 3  | this      | this     | DET   | DT   | _     | 4    | det     | _    | _    |
| 4  | sentence  | sentence | NOUN  | NN   | _     | 2    | dobj    | _    | _    |
| 5  | correctly | correctly| ADV   | RB   | _     | 2    | advmod  | _    | _    |
```

- **ID**：每个单词在句子中的位置编号。
- **FORM**：单词的实际形式。
- **LEMMA**：单词的基本形式（例如 "parsed" 的词元为 "parse"）。
- **UPOS**：通用词性标签，例如 PRON（代词）、VERB（动词）、DET（限定词）、NOUN（名词）、ADV（副词）。
- **XPOS**：细粒度词性标签（这里用英语常见标注，如 PRP, VBD, DT, NN, RB）。
- **FEATS**：形态学特征（此处没有额外信息，用 _ 表示）。
- **HEAD**：依存头的编号。例如，"I" 依附于编号为 2 的单词 "parsed"；"sentence" 是 "parsed" 的直接宾语；"this" 是 "sentence" 的限定词；"correctly" 修饰 "parsed"。
- **DEPREL**：依存关系标签，例如 nsubj（主语）、root（根）、det（限定词）、dobj（直接宾语）、advmod（状语）。
- **DEPS** 和 **MISC**：通常为空，用 _ 表示。

这种标注下，整句的依存树大致结构为：

- "parsed" 是句子的根节点；
- "I" 是 "parsed" 的主语；
- "sentence" 是 "parsed" 的直接宾语，其中 "this" 作 "sentence" 的限定词；
- "correctly" 作为副词修饰 "parsed"。

#### 2(a) 理解并列出完整的转移序列 (4 分)

给定示例句 "I parsed this sentence correctly"，以及一张目标依存树，你需要：

- **一步步** 写出 parser 的状态转移（stack、buffer、所用的 transition 以及新增的依存关系）。
- 作业里已经示例了前面三步，你需要把后续步骤完整列出来，直到解析结束。
写在了[依存句法分析](依存句法分析.md)

#### 2(b) 解析一个 n 词句子，需要多少步？(2 分)

- 用简洁的理由说明，对长度为 n 的句子，为什么需要特定数量的 SHIFT / LEFT-ARC / RIGHT-ARC 步骤才能完成依存解析。

> 因为每个词都得从 buffer 到 stack，然后又都得从 stack 到 arcs，所以 n × 2 = 2n 次

#### 2(c) 实现 `PartialParse` 的两个函数 (6 分)

- 你需要在 `parser_transitions.py` 里完成类 `PartialParse` 的 `init` 与 `parse_step` 方法，来实现"单步"转移的逻辑。
- 可以用 `python parser_transitions.py part_c` 进行基本测试（但并非完整测试）。

#### 2(d) 实现 `minibatch_parse` 函数 (8 分)

- 同样在 `parser_transitions.py` 里，按照给定算法"**小批量解析**"多句话。
- 你要用 `model`（提供的一个示例模型）去预测下一步应该做哪种转移，然后对这一小批的 partial parses 同步执行转移。
- 测试命令：`python parser_transitions.py part_d`。

#### 2(e) 训练并实现神经网络依存解析器 (12 分)

- 核心是 `parser_model.py` 和 `run.py` 文件中的若干 TODO：
    1. **`parser_model.py`**
        - `__init__`: 初始化网络，包括手写一个 **Embedding 层**、一个 **线性映射**层（hidden 层），再加一个 **输出层**（logits），以及相应的激活函数 ReLU 等。
        - `embedding_lookup`: 给定词索引列表，把它们映射成对应的 embedding 向量，并拼接（concatenate）。
        - `forward`: 前向传播，输出 logits，再用 softmax 得到预测。
    2. **`run.py`**
        - `train_for_epoch` 和 `train`: 完成训练流程，包括把数据分批、前向/反向传播、更新参数，循环多 epoch。
- 训练完成后，脚本会在 dev/test 数据集上计算 **UAS (Unlabeled Attachment Score)**，并输出结果。
- 你需要写在报告里：
    - 你训练后在 dev set 上的最好 UAS 分数
    - 以及在 test set 上的 UAS 分数
- 参考提示：
    - 你可以先 `python run.py -d` (debug 模式) 进行快速训练，看看是否能到 ~65+ 的 UAS；
    - 如果要跑完整训练，大约需要 **1 小时**，最好在 debug 确认没大问题后再跑全量。

#### 2(f) 错误解析分析 (12 分)

作业给出了四个句子的**错误**依存树，每个句子只有一个错误，对应四种常见错误类型：

1. **Prepositional Phrase Attachment Error (介词短语附着错误)**
2. **Verb Phrase Attachment Error (动词短语附着错误)**
3. **Modifier Attachment Error (修饰语附着错误)**
4. **Coordination Attachment Error (并列结构附着错误)**

对每个句子，你需要：

- 指出**错误类型**是哪一种
- 给出**错误的依存关系**（即解析器产出的错误）
- 给出**正确的依存关系**（应该正确附着到哪个词）

例子里也演示了如果句子里有个介词短语，attach 到了错误的 head，需要改成 attach 到正确的 head。

这部分也写在**书面报告**中提交。

## 实操

### 1. 准备代码目录

1. **确认文件结构**  
    你应该有一个类似 `assignment3/` 的文件夹，其中包含：
    
    - `parser_transitions.py`
    - `parser_model.py`
    - `run.py`
    - `collect_submission.sh`
    - 其他辅助文件（比如 `utils/` 目录内的 `parser_utils.py`，以及训练所需的数据等）
2. **主要脚本作用**
    
    - `parser_transitions.py`：里边实现了 **PartialParse** 类及 `minibatch_parse` 函数，需要先完成/测试它们。
    - `parser_model.py`：核心模型定义（Embedding + Forward + Loss 等），需要用 PyTorch 实现；**注意**不允许直接使用 `torch.nn.Linear` 和 `torch.nn.Embedding`，要自己定义。
    - `run.py`：训练（`train()`）、评估并输出最终结果的主要入口。
    - `collect_submission.sh`：打包提交文件用的脚本。

确保自己的作业代码目录与官方需求一致。

---

### 2. 创建并激活 Python 环境

#### 2.1 安装 Miniconda/Anaconda

如果你的机器上还没有 Python 或 Conda，先去 [Miniconda](https://docs.conda.io/en/latest/miniconda.html) 或 [Anaconda](https://www.anaconda.com/products/individual) 官网下载、安装。安装完成后，在命令行（Terminal / Anaconda Prompt）即可使用 `conda` 命令。

#### 2.2 创建虚拟环境 & 安装 PyTorch

本作业要求安装 **PyTorch 1.4.0**（且 CUDA 选项设为 None，即 CPU 版）和 `tqdm` 等依赖。操作示例：

```bash
# 进入作业目录
cd path/to/assignment3

# 创建并激活一个新的 Conda 环境，名字自定义，比如 dp-env
conda create -n dp-env python=3.7
conda activate dp-env

# 安装 PyTorch 1.4.0（CPU 版）
# 注意如果你想在带 GPU 的机器上训练，可安装相应 CUDA 版本，如 pytorch==1.4.0 cudatoolkit=10.1
pip install torch==1.4.0

# 安装 tqdm
pip install tqdm
```

如果官方给了其他依赖（比如 `numpy`, `matplotlib`, `pytest` 等），也一并 `pip install` 或 `conda install`。

**注意**：在作业中，有些版本可能写成"请从 pytorch.org 手动下载并安装特定版本"。如果你想确保跟作业环境一致，就按照作业说明操作，或者用下面的命令（假设只用 CPU 版）：

```bash
pip install https://download.pytorch.org/whl/cpu/torch-1.4.0%2Bcpu-cp37-cp37m-linux_x86_64.whl
```

(以上只是一个示例 CPU 版 wheel，具体要看官方链接。)

---

### 3. 测试 & 开发：`parser_transitions.py`

1. **实现并测试**  
    在 `parser_transitions.py` 文件中，你需要完成：
    
    - `PartialParse` 类的 `__init__` 和 `parse_step()`
    - `minibatch_parse()` 函数
2. **本地测试**  
    作业给了一个命令来做"基本的（非全面）单元测试"：

    ```bash
    python parser_transitions.py part_c
    ```

    用来测试第 (c) 小问实现的部分（`init` & `parse_step`）。

    

    再执行：

    ```bash
    python parser_transitions.py part_d
    ```

    用来测试第 (d) 小问实现的部分（`minibatch_parse`）。

    

如果所有测试通过，你的输出里会出现类似 "All tests passed for part (c)/(d)" 的提示。

---

### 4. 训练模型：`parser_model.py` & `run.py`

#### 4.1 模型实现

- 在 `parser_model.py` 中，你需要按照作业提示实现**三大部分**：
    
    1. **`__init__`**：初始化参数（比如 `self.embedding_weights`, `self.W`, `self.b1`, `self.U`, `self.b2` 等）。
    2. **`embedding_lookup()`**：自己写 embedding lookup 逻辑，不能直接用 `nn.Embedding`。
    3. **`forward()`**：把输入特征通过 embedding、ReLU 等网络层，得到分类预测 logits，再用 softmax 算出概率。
- **注意** 不要用 `torch.nn.Linear` 或 `torch.nn.Embedding`，要自己写。可以用 `torch.mm`、`torch.matmul`、`torch.add` 之类的基础操作。
    
- 作业里一般会有简单的单元测试命令，比如：

    ```bash
    python parser_model.py -e  # 测试 embedding_lookup()
    python parser_model.py -f  # 测试 forward()
    python parser_model.py -ef # 一起测试
    ```

    如果通过了，会显示类似 "Embedding Sanity Check Passed" 或 "Forward Sanity Check Passed"。

    

#### 4.2 开始训练

完成 `parser_model.py` 后，切换到 `run.py` 文件：

- 里边通常有 `train_for_epoch()`、`train()` 等函数需要完成。请按照注释把它们实现好。
- 然后在命令行执行：

    ```bash
    python run.py
    ```

    这会开始**完整训练**，用 Penn Treebank (UD) 训练集进行训练，用 dev set 做验证，再输出 dev/test 的 UAS 指标。

**常见可选参数**：

- `python run.py -d`：**Debug 模式**，只用很少的数据快速跑几轮，便于调试，不用等太久。
    - 在 Debug 模式下，如果一切正常，训练 Loss 通常能低于 0.2，UAS 能到 65+。
- **正式跑**：去掉 `-d`，用完整数据训练，可能需要 ~1 小时（依赖硬件性能）。最终能得到大约 87+ 的 UAS。

训练完成后，会打印出最终在 dev set 或 test set 上的 **Unlabeled Attachment Score (UAS)**。

---

### 5. 查看 & 提交结果

1. **查看最佳 UAS**  
    训练结束后，终端会显示"Best Dev UAS = XX.XX" 以及 "Test UAS = YY.YY"。这就是你要在作业报告里提交的指标。
    
2. **生成提交文件**  
    如果作业要求在 Gradescope 等平台提交一个 zip，则通常运行：

    ```bash
    sh collect_submission.sh
    ```

    它会在当前目录下生成 `assignment3.zip`。

    
3. **提交到服务器**
    
    - 登录 Gradescope，找到 "Assignment 3[coding]"，上传 `assignment3.zip`。
    - 将你解答好的 PDF 或书面解答部分，上传到 "Assignment 3[written]" 并在 Gradescope 上做题号标记即可。

---

### 6. 可能出现的问题与排查

1. **找不到 `tqdm`**：
    - 确认已经在当前激活的环境 (`dp-env`) 内执行 `pip install tqdm`。
2. **PyTorch 版本不匹配**：
    - 如果你安装的是更高版本的 PyTorch，也许能跑，但若有兼容问题可按作业指定的 1.4.0 版本。
3. **训练速度过慢**：
    - 如果在 CPU 上训练完整数据，可能需要一两个小时以上。可以多开 debug 模式先检查逻辑。
4. **无 GPU 却意外安装了 CUDA**：
    - 可能导致报错 "no cuda device available"。可直接安装 CPU 版或者在代码里把 `device` 设置为 CPU。

---

### 7. 总结

- **最简流程**：
    1. `conda create -n dp-env python=3.7 && conda activate dp-env`
    2. `pip install torch==1.4.0 tqdm`
    3. `python parser_transitions.py part_c` / `part_d` 检查实现的过渡函数
    4. `python parser_model.py -e/-f` 检查 model 的 embedding/forward
    5. `python run.py -d`（快速 debug），再 `python run.py`（完整训练）看 Dev/Test UAS
    6. `sh collect_submission.sh` 生成 zip，上传到 Gradescope

祝你一切顺利，成功跑通作业！如果遇到报错，可以把出错信息（Traceback）贴出来，方便进一步排查。

### 训练过程

下面我逐步解释这个训练过程以及输出信息，帮助你更好地理解整个流程：

---

#### 1. 初始化与数据加载

- **INITIALIZING**  
    开始阶段程序先做一些初始化工作，包括读取数据、构建解析器、加载预训练词向量等。
    - **Loading data... took 0.66 seconds**  
        程序读取数据花费了 0.66 秒。
    - **Building parser... took 0.34 seconds**  
        构建依赖解析器（比如初始化数据结构和模型配置）花费了 0.34 秒。
    - **Loading pretrained embeddings... took 0.92 seconds**  
        加载预训练词向量（这些词向量是提前训练好的用于表示单词的数字向量）。
    - **Vectorizing data... took 0.50 seconds**  
        将数据转换为数值（向量）的形式，以便输入模型。
    - **Preprocessing training data... took 12.98 seconds**  
        对训练数据进行预处理，花费了 12.98 秒。这一步通常涉及生成训练样本、构建部分解析的状态等。

---

#### 2. 训练过程（TRAINING）

整个训练过程分为多个 **epoch**（训练周期），这里共训练 10 个 epoch。

##### 每个 Epoch 的主要步骤

1. **训练阶段**
    
    - 输出例如 `Epoch 1 out of 10` 表示当前是第 1 个周期。
    - 进度条（例如：`1848/1848[00:14<00:00, 127.90it/s]`）显示这一轮训练共处理了 1848 个小批次（batch），整个 epoch 用时大约 14 秒，每秒大约处理 127 个 batch。
    - **Average Train Loss:**  
        例如在 Epoch 1 显示 `Average Train Loss: 0.1827`，表示这一轮中模型的平均损失（误差）大约为 0.183。损失越低，说明模型的预测与真实情况的差距越小。
2. **验证阶段（Evaluating on dev set）**
    
    - 每个 epoch 结束后，模型会在开发集（dev set）上评估性能。
    - 输出例如 `- dev UAS: 83.82` 表示开发集上的 **Unlabeled Attachment Score (UAS)** 为 83.82。UAS 是评估依赖解析器性能的指标，数值越高说明解析效果越好。
    - 当本次 epoch 得到的 UAS 比之前所有 epoch 的 UAS 都要高时，输出 `New best dev UAS! Saving model.` 意味着当前模型被认为是最好的，因此保存下来以备后续测试使用。
3. **随着 Epoch 增加**
    
    - 你可以看到损失逐渐下降（从 0.1827 到 0.0662），而 UAS 分数逐步上升，从 83.82 到 88.90（最高）。
    - 这说明随着训练的进行，模型不断学习和改进，预测的依赖关系更加准确。

---

#### 3. 测试阶段（TESTING）

- 在所有 epoch 结束后，程序会加载在验证集上表现最好的模型。
- **FutureWarning**  
    输出中的警告（关于 `torch.load` 的 FutureWarning）只是提醒在未来的 PyTorch 版本中可能需要更改加载方式，不影响当前运行结果。
- 程序在测试集上进行最终评估，并显示例如 `- test UAS: 89.05`。这表示在测试集上，模型的 UAS 分数为 89.05，说明模型在未知数据上的表现也相当不错。

---

#### 总结

- **初始化阶段**：加载数据、构建模型、预处理数据等。
- **训练阶段**：模型经过 10 个 epoch 的训练，每个 epoch 都会遍历所有训练数据，并在每个周期后在开发集上评估模型性能。随着训练，损失逐步下降，UAS 分数逐步提高。
- **测试阶段**：最后加载在开发集上表现最好的模型，并在测试集上进行评估，得到了 89.05 的 UAS 分数。

这个输出整体说明你的模型训练过程运行正常，并且模型的性能逐步提高，最终在测试集上获得了很好的解析效果。希望这些解释对你有所帮助！

下面给你一个简要的**任务概览**，帮助你理清整个作业需要完成的内容及交付物。在实际完成作业时，你需要同时写一些"**理论部分解答**"（相当于 written 作业）和"**代码实现**"（coding 作业），并按照助教要求提交。

---
