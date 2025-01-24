---
draw:
tags: []
title: Assignment 3.1 Neural Machine Translation with RNNs
date created: 2025-01-24
date modified: 2025-01-24
---

## 大白话解释：每个任务是干啥的？

### Part 1: 机器学习和神经网络 (8 分)

- **(a) Adam Optimizer (Adam 优化器)**
    - **干啥:** 这是一种**让模型学得更快、更好**的"秘籍"。就像给学习过程加了"加速器"和"稳定器"，让模型训练起来更有效率，最终效果也更好。
    - **目的:** 让模型**更快收敛**（更快学到东西），并且**更稳定**（不容易跑偏）。
        
- **(b) Dropout (Dropout)**
    
    - **干啥:** 这是一种**防止模型"死记硬背"**的技巧。就像学习的时候，偶尔"偷懒"一下，随机忽略一些知识点，反而能学得更扎实、更灵活。
    - **目的:** **防止模型过拟合**（在训练集上表现很好，但在新数据上表现很差），让模型**泛化能力更强**（在新数据上也能用）。
        

### Part 2: 神经网络的 Transition-Based [[语法分析、依存句法分析]] (44 分)

- **(a) 过一遍例句解析过程**
    
    - **干啥:** 手动模拟一遍**句子是怎么一步步被分析出来**的。就像玩一个解谜游戏，每一步都按规则走，最终把句子结构搞清楚。
    - **目的:** **理解依存句法分析的基本步骤**，明白"Transition-Based"方法是怎么运作的。
        
- **(b) 句子长度为 n 时，需要多少步？**
    
    - **干啥:** 思考一下，**理论上**分析一个句子**最少需要多少步**。就像算一下解谜游戏最快通关需要几步。
    - **目的:** **从理论层面理解解析过程的效率**，为后续实现打基础。
        
- **(c) 实现 PartialParse 类中的 init 和 parse_step**
    
    - **干啥:** **写代码**，实现一个"解析器"，让电脑**按照规则一步一步分析句子**。就像写一个程序，让电脑模拟人玩解谜游戏的过程。
    - **目的:** **动手实现解析器的核心逻辑**，把理论变成实际操作。
        
- **(d) Minibatch Parsing (小批量解析)**
    
    - **干啥:** **升级解析器**，让电脑**一次性分析"一批"句子**，而不是一个一个分析。就像让电脑同时玩多个解谜游戏，提高效率。
    - **目的:** **提高解析效率**，为用神经网络训练模型做准备（因为神经网络训练通常用 minibatch）。
        
- **(e) 训练神经网络来预测解析动作**
    
    - **干啥:** **用神经网络来"教"电脑如何做解析**。就像训练一个 AI 玩家，让它自己学会玩解谜游戏，而不是人一步步教它。
    - **目的:** **用机器学习的方法，让电脑自动学会依存句法分析**。核心是训练一个神经网络分类器，让它预测每一步应该做什么动作（SHIFT, LEFT-ARC, RIGHT-ARC）。
        
- **(f) 分析依存解析的典型错误**
    
    - **干啥:** **当"裁判"**，看看电脑分析错的句子，**找出错误类型**，并**分析错误原因**。就像玩解谜游戏后，看看自己哪里走错了，下次怎么避免。
    - **目的:** **理解当前模型的不足之处**，为未来改进模型提供方向。
        

---

**总而言之，这份作业就是：**

1. **先学两个"武功秘籍"**（Adam 和 Dropout），让模型更厉害。
    
2. **然后学习一种"解句子结构"的方法**（Transition-Based 依存句法分析）。
    
3. **再训练一个"AI 大师"**（神经网络模型），让它自动学会解句子结构。
    
4. **最后"考试"一下**，看看"AI 大师"学得怎么样，哪里还需要提高。
    

希望这个更简洁、更口语化的解释能帮助你更好地理解每个任务的目的！

## 作业整体概览

这份作业分为两个主要部分：

1. **Part 1：Machine Learning & Neural Networks（共 8 分）**
    
    - (a) Adam Optimizer
    - (b) Dropout
2. **Part 2：Neural Transition-Based Dependency Parsing（共 44 分）**
    
    - (a) 到 (d)：理解并实现依存分析（dependency parsing）中的一些核心操作（单个句子的解析、minibatch 的解析等）。
    - (e) 使用 PyTorch 搭建并训练一个简单的神经网络来预测下一步转移操作，从而完成依存解析。
    - (f) 分析模型在真实句子上可能出现的错误类型。

完成后，你需要提交两部分：一份是"coding"部分（你的代码），一份是"written"部分（你的书面答案）。

接下来，我们会**按照作业顺序**，对每个小问题逐步进行解释，让你明白背后在做什么、为什么要这样做、以及要实现哪些内容。

---

## Part 1. Machine Learning & Neural Networks (8 分)

### 1(a) Adam Optimizer

Adam 优化器（Adam Optimizer）是一种在实践中非常常用且有效的梯度下降优化算法。它是在**随机梯度下降（SGD）**的基础上，引入了两个重要技巧：

1. **Momentum（动量）**：在 Adam 中对应的变量是 mm；
2. **Adaptive Learning Rates**（自适应学习率）：在 Adam 中对应的变量是 vv。

> **回顾**：
>
> - **普通的随机梯度下降（SGD）**更新公式是：
>
> θ←θ−α∇θJminibatch(θ)\theta \leftarrow \theta - \alpha \nabla_\theta J_{\text{minibatch}}(\theta)
>
> 其中 θ\theta 表示模型参数；α\alpha 表示学习率（通常是一个固定值或慢慢衰减）；∇θJ\nabla_\theta J 表示损失函数相对于参数的梯度。
>
> - **Adam** 则在此基础上增加了两组更新：一个是对梯度的滚动平均 mm，另一个是对梯度平方的滚动平均 vv。

#### (i) 动量（Momentum）的直观理解

Adam 中**第一步**引入的动量更新为：

m←β1m+(1−β1)∇θJminibatch(θ)m \leftarrow \beta_1 m + (1 - \beta_1)\nabla_\theta J_{\text{minibatch}}(\theta)

然后用

θ←θ−αm\theta \leftarrow \theta - \alpha m

进行参数更新。

- 这里的 β1\beta_1 通常取一个 0 到 1 之间的数，一般为 0.9。
- 你可以把 mm 理解为**对最近多次梯度的加权平均**，这就像"动量"一样，让更新方向更平滑、更稳定，减少了梯度的抖动（抖动就是每次因为随机小批量数据而带来的小幅度不一致）。

**为什么减少更新的抖动会有帮助？**

- 如果仅仅使用原始梯度做更新，每次训练可能受到单个小批量噪声的影响，导致更新方向大幅变动。
- 引入动量后，可以让更新方向更加"稳定"，朝着平均方向前进，不会过度受到一次噪声梯度的影响。
- 这样的低方差更新通常会使得模型更快、更稳定地收敛。

#### (ii) 自适应学习率（Adaptive Learning Rates）

Adam 中**第二步**的关键是维护一个 **vv**（梯度平方的滚动平均）：

v←β2v+(1−β2)(∇θJminibatch(θ)⊙∇θJminibatch(θ))v \leftarrow \beta_2 v + (1 - \beta_2) \left(\nabla_\theta J_{\text{minibatch}}(\theta) \odot \nabla_\theta J_{\text{minibatch}}(\theta)\right)

然后最终更新时，还要**对动量除以 v\sqrt{v}**：

θ←θ−α⋅mv\theta \leftarrow \theta - \alpha \cdot \frac{m}{\sqrt{v}}

- ⊙\odot 表示元素逐个相乘（Hadamard product），也就是对每个参数位置单独平方。
- β2\beta_2 也是一个类似 0.99 这样的超参数。

**会得到什么效果？**

- 那些**梯度长期比较小**的参数位置，vv 值就相对比较小，因此1v\frac{1}{\sqrt{v}} 会比较大，这意味着**更新幅度相对更大**。
- 那些**梯度长期比较大**的参数位置，vv 值就相对比较大，因此1v\frac{1}{\sqrt{v}} 会比较小，这意味着**更新幅度相对变小**。

这就是"自适应学习率"的含义：对不同参数位置，**学习率会自动调节**。这样可以在某些维度上快速前进，但在另一些梯度特别大的维度上则减小步伐，以免震荡太厉害或训练不稳定。总体上，这种做法对训练非常有帮助。

---

### 1(b) Dropout

Dropout 是深度学习里常见的一种**正则化手段**（减少过拟合）。它的做法是：在训练过程中，随机将网络隐藏层中部分神经元"抛弃"（置为 0），用概率 pdropp_{\text{drop}} 来控制抛弃多少。然后为了保持输出的期望值不变，会乘上一个常数 γ\gamma 进行缩放。

数学形式是：

$$hdrop=γ d⊙hh_{\text{drop}} = \gamma \, d \odot h$$

- hh 是原始的隐藏层输出（一个向量大小为 DhD_h）。
- d∈{0,1}Dhd \in \{0,1\}^{D_h} 是一个掩码向量，每个位置独立以 pdropp_{\text{drop}} 的概率为 0、(1−pdrop)(1 - p_{\text{drop}}) 的概率为 1。
- γ\gamma 是一个缩放系数，使得经过 dropout 后的**期望输出**仍能和原来未做 dropout 时保持一致。

#### (i) γ\gamma 应该是多少？

因为每个神经元都有 pdropp_{\text{drop}} 的概率被置为 0，1−pdrop1 - p_{\text{drop}} 的概率被保留为 1，所以**保留下来的期望比例**是 (1−pdrop)(1 - p_{\text{drop}})。如果不乘以任何系数，则隐藏层的期望值会变小。为了让期望不变，通常：

γ=11−pdrop.\gamma = \frac{1}{1 - p_{\text{drop}}}.

这样一来，虽然一部分神经元被丢掉了，但那些保留下来的神经元输出值被放大，相当于补偿了丢掉的部分，使得"期望输出"仍和原来一样。

#### (ii) 为什么训练时用 Dropout，推断/测试时不用？

- 训练时用 dropout：可以**有效减少过拟合**，因为随机丢弃神经元的方式相当于让网络的结构"随机稀疏化"，迫使网络不能过度依赖某些特定节点或路径，从而提高了模型的泛化能力。
- 测试时就**不需要**了：推断时我们希望网络得到最稳定、最强的表达能力，所以不再随机丢弃神经元，同时会将**所有神经元**都用上。一般做法是把整层的输出保留，并且不会再乘以缩放系数（或者说把 dropout 部分关掉），这样预测会更稳定。

---

## Part 2. Neural Transition-Based Dependency Parsing (44 分)

这一部分的目标：**实现并训练一个基于神经网络的 transition-based 依存句法解析器**，并最终在给定的测试集上算出 UAS（Unlabeled Attachment Score）。

### 背景概念：Transition-Based Parsing

"依存解析（dependency parsing）"的目的是给定一句话，找出所有单词之间的"从属（依存）关系"，比如动词与主语之间、动词与宾语之间、形容词与名词之间等。

**Transition-Based** 的方法是：我们会维护一个**部分解析状态（partial parse）**，包括：

- **栈（stack）**：已经在处理或处理完一部分单词
- **缓冲区（buffer）**：还未处理的剩余单词
- **依存关系列表（dependencies）**：当前已经预测出的依存关系

初始时，`stack =[ROOT]`、`buffer =`（整句所有单词顺序）、`dependencies =[]`。我们可以通过三个操作来一点点地构建整棵依存树：

1. `SHIFT`: 将 buffer 中第一个单词弹出并推入 stack。
2. `LEFT-ARC`: 将 stack 中顶上第二个元素视为"从属（dependent）"，把它的"依存"指向 top（最上面）的元素，并将那个第二个元素从栈弹出。
3. `RIGHT-ARC`: 与 LEFT-ARC 类似，但反过来，把栈顶元素标为"从属"，依存于第二个元素，然后把栈顶元素弹出。

不断重复以上步骤，直到 buffer 为空且栈只剩下[ROOT]，就完成了依存树的构建。

---

### 2(a) 过一遍示例句子的解析过程

例句："I parsed this sentence correctly" 的依存树给定。作业要求你一步一步列出：

- 当前 `stack`
- 当前 `buffer`
- 采用的 transition 以及添加了什么新的依存关系

给出的示例前三步是：

1. 初始状态：
    
    - `stack =[ROOT]`
    - `buffer =[I, parsed, this, sentence, correctly]`
    - `dependencies =[]`
    - **无 transition** (初始)
2. 第一步 transition: `SHIFT`
    
    - `stack =[ROOT, I]`
    - `buffer =[parsed, this, sentence, correctly]`
    - **无新的依存关系**，因为 SHIFT 只是把第一个 buffer 元素推入 stack
3. 第二步 transition: `SHIFT`
    
    - `stack =[ROOT, I, parsed]`
    - `buffer =[this, sentence, correctly]`
    - **无新的依存关系**
4. 第三步 transition: `LEFT-ARC`
    
    - `stack =[ROOT, parsed]`（因为把第二个栈顶元素 I 依存于 parsed 并 pop 掉 I）
    - `buffer =[this, sentence, correctly]`
    - **新的依存关系**：`parsed -> I`（即 parsed 是 I 的 head）

一直这样直到句子解析完，你需要把所有步骤列完整。

---

### 2(b) 句子长度为 n 时，需要多少步？

这里的答案跟解析过程的"理论性质"有关：对于一个包含 n 个单词的句子，需要进行 **2n - 1** 步 或者 **3n - 2** 步（不同文献定义的左右 ARC 次数不同，但在本作业中答案是**2n - 1**）。

- 直观理解：
    - 你需要把 n 个单词都 "SHIFT" 进 stack（这需要 n 步）；
    - 最后每个单词还要通过一次 ARC 操作（LEFT 或 RIGHT）跟别的单词建立依存并弹出（除了 ROOT 不弹），这需要 n-1 步；
    - 总共是 `n + (n-1) = 2n-1` 步。

---

### 2(c) 实现 `PartialParse` 类中的 `init` 和 `parse_step`

你会在 `parser_transitions.py` 中看到一个 `PartialParse` 类，它管理 `stack`, `buffer`, 和 `dependencies`。作业要你完成以下方法：

- `__init__(self, sentence)`: 初始化 `stack`, `buffer`, `dependencies`
- `parse_step(self, transition)`: 根据给定的 `transition` 类型（"SHIFT"、"LEFT-ARC"、"RIGHT-ARC"）更新内部状态。

**要点**：

- `SHIFT` 就是从 buffer 取出第一个词，放到 stack 顶端。
- `LEFT-ARC` 是"第二个顶"被"最顶"依存。要**往 `dependencies` 里加一条** `head -> dependent`，并**弹出**那个"第二个顶"。
- `RIGHT-ARC` 是"最顶"被"第二个顶"依存。依赖关系要记得记录，然后弹出"最顶"。

写完后可以用 `python parser_transitions.py part_c` 来做一些基本测试。

---

### 2(d) Minibatch Parsing

神经网络通常可以**批量预测（batch prediction）**，效率更高。此时我们就可以针对一个"minibatch"里的多个句子，同时预测下一个 transition，然后同时更新他们的局部解析状态。

这部分需要你实现 `minibatch_parse` 函数。伪代码如下：

```pseudo
Initialize partial_parses = [PartialParse(sentence) for sentence in sentences]
Initialize unfinished_parses = shallow copy of partial_parses

while unfinished_parses not empty:
    Take the first batch_size parses in unfinished_parses as a minibatch
    Use model to predict the next transition for each partial parse
    Perform a parse step on each partial parse
    Remove the completed parses from unfinished_parses

Return the dependencies for each parse in partial_parses
```

关键就是：

- 一次取一批（batch_size）还没完成的解析
- 调用"模型"给出每个句子的下一个转移操作，再批量执行
- 直到所有句子都完成解析

写完后用 `python parser_transitions.py part_d` 测试。

---

### 2(e) 训练神经网络来预测解析动作

现在我们要做一个**分类器**：给定当前 `PartialParse` 的状态（stack, buffer, dependencies），预测下一步应该执行 "SHIFT", "LEFT-ARC" 还是 "RIGHT-ARC"。

#### 特征提取

我们使用了"Chen and Manning (2014)"论文中用到的特征：
在 `utils/parser_utils.py` 有一个函数，给定一个 `PartialParse`，返回一个**固定大小**的 token 列表，这些 token 其实就是：

- 栈顶几个元素
- buffer 前几个元素
- 以及它们各自的依赖子节点（或父节点）
    ……等等。

这些 token 都被映射成词典里的某个 integer id（如果不在词典里就会变成一个特殊 id）。这样我们就得到一个大小为 `m` 的整型列表 `[w1, w2,..., wm]`。

#### 嵌入层 + 前向网络

然后我们自己实现的模型是一个很简单的前向网络：

1. **Embedding lookup**：对每个 token wiw_i 查一个**词向量**（embedding），然后把这些 embedding 拼起来（concat）成一个向量 `x`。
    - 如果 embedding 维度是 `d`，而特征数是 `m`，那么最终 `x` 的维度就是 `m * d`。
    - 注意作业要求**自己实现** embedding lookup（不要用 PyTorch 自带的 `nn.Embedding`）。其实就是一个词典大小为 `|V|`、embedding 维度为 `d` 的矩阵 `E`，每一行是对应词的向量。
2. **全连接 + ReLU**：h=ReLU(xW+b1)h = \text{ReLU}(x W + b_1)
    - 如果隐藏层大小是 `hidden_dim`，那么 `W` 的大小就是 `(m * d, hidden_dim)`，`b_1` 大小是 `hidden_dim`。
    - ReLU 是激活函数，`ReLU(z) = max(z, 0)`（逐元素）。
3. **输出层**：l=hU+b2l = h U + b_2
    - 这里 `U` 的大小是 `(hidden_dim, 3)`，因为有 3 个类别（"SHIFT", "LEFT-ARC", "RIGHT-ARC"）。
    - `b_2` 的大小是 3。
    - 最终得到一个大小为 3 的 logits，之后用 softmax 得到 3 个类别的预测分布。

#### 损失函数 (Cross Entropy)

对每个训练样本，根据真实标签 (y)，计算交叉熵损失：

J(θ)=−∑i=13yilog⁡(y^i)J(\theta) = - \sum_{i=1}^3 y_i \log(\hat{y}_i)

其中 y^i\hat{y}_i 是预测概率，yiy_i 是 one-hot 的真实标签。

训练时，我们会:

1. 前向计算预测结果
2. 计算 loss
3. 反向传播 (backprop)
4. 用 Adam 或其他方式更新参数

训练完后，就能用这个模型来做解析（minibatch_parse）。

**你需要做的**：在 `parser_model.py` 的指定位置补全以下函数：

- `init`：初始化网络的参数矩阵（embedding 矩阵 `E`, 两层的权重 `W`, `U` 等）
- `embedding_lookup`：把一批整数 token 的索引变成 embedding 并拼好
- `forward`：把拼好的 embedding 送入网络，得到 logits 或者最终概率
- 然后在 `run.py` 里完成 `train_for_epoch` 与 `train` 函数（大部分框架可能已经给你搭好了，你只需要把训练循环写好等）。

最后执行 `python run.py` 进行训练，看看在开发集（dev set）和测试集（test set）上的 UAS。

**一些提示**：

- 作业可能会让你先用 `python parser_model.py -e` / `-f` 来做简单检查（embedding 或 forward）。
- 可以用 `python run.py -d` 打开 debug 模式，只在很小一部分训练集上跑，以便调试快一点。
- debug 模式下，你能期待的 dev UAS 大概 65+。
- 不开 debug 的话，大约要 1 小时左右训练，全量数据可以达到 87+ 的 dev UAS。

#### 最终你需要提交

1. 你最后在 dev set 得到的最佳 UAS
2. 在 test set 得到的 UAS

---

### 2(f) 分析依存解析的典型错误

这里给了几个句子以及它们的错误解析树，需要你识别错误是哪一种类型、错误依存关系是啥、正确的依存关系又该是什么。作业给出了常见的四种错误类型：

1. **Prepositional Phrase Attachment Error**（介词短语附着错误）
    
    - 例如 "sent troops **into Afghanistan**" 中，短语 "into Afghanistan" 应该附着在动词 "sent" 上，却被错误地附着在 "troops" 上。
2. **Verb Phrase Attachment Error**（动词短语附着错误）
    
    - 例如 "Leaving the store unattended, I went outside..." 里 "Leaving the store unattended" 应该依存于 "went"，但可能被错误地依存到别的词。
3. **Modifier Attachment Error**（修饰语附着错误）
    
    - 例如 "I am **extremely** short."，单词 "extremely" 修饰 "short" 而不是别的词。
4. **Coordination Attachment Error**（并列结构附着错误）
    
    - 例如 "brown rice or garlic naan"，正确依存是 "garlic naan" 并列依存到 "brown rice"，而不是依赖别的地方。

题目给了四个句子，每个句子有一个错误，需要你：

- 指出错误类型（以上四种之一）
- 指出**错误的依存关系**（"谁 -> 谁"）
- 指出**正确的依存关系**

---

## 总结

以上就是对整个作业的**核心逻辑与每一步所做事情**的**中文初学者向**讲解。做完本作业，你会对**Adam 优化器**、**Dropout**、以及**基于转移的依存解析**流程和**简单神经网络实现**有更扎实的理解：

1. **Adam**：你会知道为什么要有动量（Momentum）和自适应学习率（Adaptive LR）。
2. **Dropout**：理解它的数学期望和为什么只在训练时用。
3. **Transition-Based Parsing**：感受解析器如何一步步构造依存树，以及如何在小批量（minibatch）中做转移动作预测。
4. **简单神经网络（Embedding + 前馈层 + ReLU + softmax）**：实现并训练一个依存分析模型。
5. **错误类型分析**：看实际句子中错误的常见分类。

希望这个分步骤的讲解，能帮助你更好地完成作业！祝学习顺利，也祝你在训练模型时能拿到好的 UAS 分数。加油！
