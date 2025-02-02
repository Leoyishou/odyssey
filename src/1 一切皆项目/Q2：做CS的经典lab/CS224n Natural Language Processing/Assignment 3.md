---
draw:
tags: []
title: Assignment 3
date created: 2025-01-24
date modified: 2025-01-26
---

下面给你一个简要的**任务概览**，帮助你理清整个作业需要完成的内容及交付物。在实际完成作业时，你需要同时写一些"**理论部分解答**"（相当于 written 作业）和"**代码实现**"（coding 作业），并按照助教要求提交。

---

## Part 1: Machine Learning & Neural Networks（8 分）

这是一些**基础理论题**，主要围绕 Adam 优化器和 Dropout 这两种神经网络技术展开。

1. **Adam Optimizer (4 分)**
    
    - (a)(i) 解答关于动量 (momentum) 的直觉：
        - 为什么引入 `m`（动量）可以减少更新的波动？
        - 为什么这种"低方差"更新可以帮助训练更稳定地收敛？
    - (a)(ii) 解答关于自适应学习率 (adaptive learning rate) 的直觉：
        - 为什么用 v\sqrt{v} 来缩放更新？
        - 哪些参数会得到更大或更小的更新？
        - 这样做对学习有什么好处？
2. **Dropout (4 分)**
    
    - (b)(i) 求出 dropout 中的系数 γ\gamma 跟 pdropp_{drop}（丢弃概率）的关系，并简要说明原因。
    - (b)(ii) 为什么在**训练**时要使用 dropout，而在**推断/预测**（evaluation）时则不使用？

这部分你需要**写在书面报告（Assignment 3 [written]）里**，回答上述问题（只需简要解释直觉即可）。

---

## Part 2: Neural Transition-Based Dependency Parsing（44 分）

这一部分是整个作业的核心：**实现一个基于神经网络的依存句法解析器**，并分析一些错误解析案例。你需要完成以下工作：

### 2(a) 理解并列出完整的转移序列 (4 分)

给定示例句 "I parsed this sentence correctly"，以及一张目标依存树，你需要：

- **一步步** 写出 parser 的状态转移（stack、buffer、所用的 transition 以及新增的依存关系）。
- 作业里已经示例了前面三步，你需要把后续步骤完整列出来，直到解析结束。

### 2(b) 解析一个 n 词句子，需要多少步？(2 分)

- 用简洁的理由说明，对长度为 n 的句子，为什么需要特定数量的 SHIFT / LEFT-ARC / RIGHT-ARC 步骤才能完成依存解析。

### 2(c) 实现 `PartialParse` 的两个函数 (6 分)

- 你需要在 `parser_transitions.py` 里完成类 `PartialParse` 的 `init` 与 `parse_step` 方法，来实现"单步"转移的逻辑。
- 可以用 `python parser_transitions.py part_c` 进行基本测试（但并非完整测试）。

### 2(d) 实现 `minibatch_parse` 函数 (8 分)

- 同样在 `parser_transitions.py` 里，按照给定算法"**小批量解析**"多句话。
- 你要用 `model`（提供的一个示例模型）去预测下一步应该做哪种转移，然后对这一小批的 partial parses 同步执行转移。
- 测试命令：`python parser_transitions.py part_d`。

### 2(e) 训练并实现神经网络依存解析器 (12 分)

- 核心是 `parser_model.py` 和 `run.py` 文件中的若干 TODO：
    1. **`parser_model.py`**
        - `__init__`: 初始化网络，包括手写一个 **Embedding 层**、一个 **线性映射**层（hidden 层），再加一个 **输出层**（logits），以及相应的激活函数 ReLU 等。

            > 注意作业要求**不允许**直接用 `torch.nn.Embedding`、`torch.nn.Linear`，而是需要手动写矩阵乘法和参数初始化。

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

### 2(f) 错误解析分析 (12 分)

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

---

## 最终提交

### 1. 代码提交（Assignment 3 [coding]）

1. 你在 `parser_transitions.py`、`parser_model.py` 和 `run.py` 里完成的所有实现。
2. 作业官方提供了 `collect_submission.sh` 脚本来打包你的代码到 `assignment3.zip`。
3. 把 `assignment3.zip` 上传到 Gradescope 的 "**Assignment 3[coding]**"。

### 2. 书面报告提交（Assignment 3 [written]）

你对 Part 1 (Adam + Dropout)、Part 2 (a)(b)(f) 等**问答题**、**错误解析分析**、**模型性能汇报**等写在一个 PDF（或学校要求的格式）里，上传到 "**Assignment 3 [written]**"。

---

## 总结

- **Part 1 (written)**：回答关于 Adam 和 Dropout 的理论题。
- **Part 2 (written + coding)**：
    1. (a)～(b)：手动写依存解析转移序列 & 解析步数；
    2. (c)～(d)：在 `parser_transitions.py` 完成 "单步 + 小批量" 解析逻辑（coding）；
    3. (e)：实现并训练神经网络解析器（在 `parser_model.py` + `run.py` 里 coding），并汇报 UAS 分数（written）；
    4. (f)：分析并给出 4 个句子的错误附着 (attachment) 类型与纠正（written）。

只要你**按作业要求实现并回答上述所有部分**，就完成了 Assignment 3 的全部要求。祝顺利完成！

以下是CS 224n Assignment #3 的主要任务分解和实现要点，帮助你高效完成作业：

---

### **1. 理论部分（8分）**

#### **(a) Adam优化器**

- **问题 i**：解释动量的作用  
  **答案要点**：
  - **动量（Momentum）** 通过维护梯度的移动平均（`m`），使更新方向更稳定，减少随机梯度下降中的震荡。
  - **低方差的好处**：参数更新方向更一致，加速收敛，尤其在损失函数曲率复杂（如峡谷地形）时避免震荡。
- **问题 ii**：自适应学习率的影响  
  **答案要点**：
  - **参数更新幅度**：梯度较小的参数会获得更大的更新（因为 `v` 较小，分母 `sqrt(v)` 较小）。
  - **帮助学习的原因**：自适应调整不同参数的学习率，避免梯度较小的参数被忽略，适合稀疏数据。

#### **(b) Dropout**

- **问题 i**：计算 `γ`  
  **答案要点**：
  - `γ = 1 / (1 - p_drop)`  
  - **推导**：`E[h_drop]= γ * (1 - p_drop) * h = h` ⇒ `γ = 1 / (1 - p_drop)`
- **问题 ii**：训练与评估的区别  
  **答案要点**：
  - **训练时应用**：增强模型鲁棒性，防止过拟合（通过随机失活模拟集成学习）。
  - **评估时不应用**：保持确定性输出，避免随机性干扰预测结果。

---

### **2. 依赖解析实现（44分）**

#### **(a) 解析步骤分析（4分）**

- **句子**："I parsed this sentence correctly"  
  **解析步骤示例**（需补充完整）：

  | 堆栈 (Stack) | 缓冲区 (Buffer) | 新增依赖 (New Dependency) | 操作 (Transition) |
  |--------------|------------------|---------------------------|-------------------|
  |[ROOT]|[I, parsed,...]| -                         | 初始状态          |
  |[ROOT, I]|[parsed,...]| -                         | SHIFT             |
  |[ROOT, parsed]|[this,...]| parsed → I                | LEFT-ARC          |

#### **(b) 解析步骤数（2分）**

- **答案**：`2n - 1` 步（n为句子词数）
  **解释**：每个词需一次SHIFT进入堆栈，每个依赖需一次ARC操作（共n-1次）。

#### **(c) 实现 `PartialParse` 类（6分）**

- **关键函数**：

  ```python
  class PartialParse:
      def __init__(self, sentence):
          self.stack = ["ROOT"]
          self.buffer = list(sentence)
          self.dependencies = []
      
      def parse_step(self, transition):
          if transition == "SHIFT":
              self.stack.append(self.buffer.pop(0))
          elif transition == "LEFT-ARC":
              dependent = self.stack.pop(-2)
              head = self.stack[-1]
              self.dependencies.append((head, dependent))
          elif transition == "RIGHT-ARC":
              dependent = self.stack.pop()
              head = self.stack[-1]
              self.dependencies.append((head, dependent))
  ```

#### **(d) 实现 `minibatch_parse`（8分）**

- **算法核心**：分批处理未完成的解析状态，动态移除已完成解析。
  **代码框架**：

  ```python
  def minibatch_parse(sentences, model, batch_size):
      partial_parses = [PartialParse(s) for s in sentences]
      unfinished_parses = partial_parses.copy()
      while unfinished_parses:
          batch = unfinished_parses[:batch_size]
          transitions = model.predict(batch)
          for parse, trans in zip(batch, transitions):
              parse.parse_step(trans)
          unfinished_parses = [p for p in unfinished_parses if not p.is_complete()]
      return [p.dependencies for p in partial_parses]
  ```

#### **(e) 实现神经网络模型（12分）**

- **关键组件**：
  - **嵌入层（Embedding）**：手动实现（禁止使用 `torch.nn.Embedding`）。
  - **线性层（Linear）**：手动实现矩阵乘法（禁止使用 `torch.nn.Linear`）。

  **前向传播代码示例**：

  ```python
  class ParserModel(nn.Module):
      def __init__(self, embeddings, n_features, hidden_size, n_classes):
          super().__init__()
          self.embed_size = embeddings.shape[1]
          self.embeddings = nn.Parameter(torch.tensor(embeddings))
          self.W = nn.Parameter(torch.randn(n_features * self.embed_size, hidden_size))
          self.b1 = nn.Parameter(torch.zeros(hidden_size))
          self.U = nn.Parameter(torch.randn(hidden_size, n_classes))
          self.b2 = nn.Parameter(torch.zeros(n_classes))
      
      def embedding_lookup(self, w):
          return self.embeddings[w].view(-1, self.embed_size * w.shape[1])
      
      def forward(self, w):
          x = self.embedding_lookup(w)
          h = torch.relu(torch.matmul(x, self.W) + self.b1)
          logits = torch.matmul(h, self.U) + self.b2
          return logits
  ```

- **训练与评估**：
  - **目标**：在开发集（dev set）上达到UAS > 87，测试集（test set）接近92.5。
  - **调参技巧**：调整隐藏层维度（`hidden_size`）、Adam超参数（`β1`, `β2`）、学习率（`α`）。

#### **(f) 错误分析（12分）**

- **错误类型与修正示例**：
  **i. 句子**：

  ```text
  I disembarked and was heading to a wedding fearing my death .
  ```

  - **错误类型**：Verb Phrase Attachment Error  
  - **错误依赖**：`fearing → wedding`  
  - **正确依赖**：`fearing → heading`

  **ii. 句子**：

  ```text
  It makes me want to rush out and rescue people from dilemmas of their own making .
  ```

  - **错误类型**：Coordination Attachment Error  
  - **错误依赖**：`rescue → people`  
  - **正确依赖**：`rescue → rush`

---

### **3. 提交要求**

- **代码**：实现 `parser_transitions.py` 和 `parser_model.py`，确保通过测试。
- **报告**：记录开发集和测试集的UAS分数（目标：开发集 > 87，测试集接近论文的92.5）。
- **错误分析**：对每个句子明确错误类型、错误依赖和正确依赖。

---

### **关键调试技巧**

1. **梯度检查**：使用 `torch.autograd.gradcheck` 验证手动实现的梯度。
2. **维度对齐**：确保所有张量操作维度匹配（如嵌入向量拼接后的输入维度）。
3. **超参数调整**：若训练不稳定，尝试降低学习率（如从 `0.001` 开始）。

通过以上步骤，你将系统完成依赖解析器的实现与理论分析。遇到具体问题时，可进一步讨论代码实现细节或理论推导！
