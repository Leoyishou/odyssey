---
date created: 2025-04-07
date modified: 2025-08-06
uid: 230f28b4-80db-4c5f-8ec5-b104e8863a27
---
Transformer学习笔记一：Positional Encoding（位置编码）- 猛猿的文章 - 知乎
https://zhuanlan.zhihu.com/p/454482273

# Summary
分为绝对位置编码和相对位置编码

#第一性原理 解决的是[自注意力机制 Self-attention](自注意力机制%20Self-attention.md)是顺序无关，但是一个句子的真实含义也取决于token 出现的顺序的，所以需要加入位置编码

位置编码的作用是：
1. *为句子中的**每一个位置**（第0个位置，第1个位置，…）生成一个**独特的向量**。这个向量只跟**位置**有关，跟这个位置上**是什么词**无关。
2. 把这个**位置向量** “加” 到对应位置的**词嵌入向量**上。
3. 这样，输入到 Transformer 注意力层的新向量就**同时包含了“这个词是什么意思”和“这个词在句子哪个位置”** 这两种信息。
4. 注意力机制在计算词与词之间的关系时，就能把位置信息也考虑进去了，从而理解句子的顺序。

位置编码需要满足的三个性质：
1. 能定位：它能用来表示一个token在序列中的绝对位置
2. 均匀性：在序列长度不同的情况下，不同序列中token的相对位置/距离也要保持一致
3. 可扩展：可以用来表示模型在训练过程中从来没有看到过的句子长度。


# Cues
[[RoPE]]

作者可能是从[[傅里叶级数]]中得到的灵感

![CleanShot 2025-07-13 at 19.28.46@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_wH6UCAmTNl%2F2025%2F07%2F13%2F19-28-55-d11e0948643ddf9199fbda923939143d-CleanShot%202025-07-13%20at%2019.28.46-2x-d0f99b.png)


# Notes


- [一、什么是位置编码](#%E4%B8%80%E3%80%81%E4%BB%80%E4%B9%88%E6%98%AF%E4%BD%8D%E7%BD%AE%E7%BC%96%E7%A0%81)
- [二、构造位置编码的方法 /演变历程](#%E4%BA%8C%E3%80%81%E6%9E%84%E9%80%A0%E4%BD%8D%E7%BD%AE%E7%BC%96%E7%A0%81%E7%9A%84%E6%96%B9%E6%B3%95%20/%E6%BC%94%E5%8F%98%E5%8E%86%E7%A8%8B)
	- [2.1 用整型值标记位置](#2.1%20%E7%94%A8%E6%95%B4%E5%9E%8B%E5%80%BC%E6%A0%87%E8%AE%B0%E4%BD%8D%E7%BD%AE)
	- [2.2 用[0,1]范围标记位置](#2.2%20%E7%94%A8%5B0,1%5D%E8%8C%83%E5%9B%B4%E6%A0%87%E8%AE%B0%E4%BD%8D%E7%BD%AE)
	- [2.3 用二进制向量标记位置](#2.3%20%E7%94%A8%E4%BA%8C%E8%BF%9B%E5%88%B6%E5%90%91%E9%87%8F%E6%A0%87%E8%AE%B0%E4%BD%8D%E7%BD%AE)
	- [2.4 用周期函数（sin）来表示位置](#2.4%20%E7%94%A8%E5%91%A8%E6%9C%9F%E5%87%BD%E6%95%B0%EF%BC%88sin%EF%BC%89%E6%9D%A5%E8%A1%A8%E7%A4%BA%E4%BD%8D%E7%BD%AE)
	- [2.5 用sin和cos交替来表示位置](#2.5%20%E7%94%A8sin%E5%92%8Ccos%E4%BA%A4%E6%9B%BF%E6%9D%A5%E8%A1%A8%E7%A4%BA%E4%BD%8D%E7%BD%AE)
- [三、Transformer中位置编码方法：Sinusoidal functions](#%E4%B8%89%E3%80%81Transformer%E4%B8%AD%E4%BD%8D%E7%BD%AE%E7%BC%96%E7%A0%81%E6%96%B9%E6%B3%95%EF%BC%9ASinusoidal%20functions)
	- [3.1 Transformer 位置编码定义](#3.1%20Transformer%20%E4%BD%8D%E7%BD%AE%E7%BC%96%E7%A0%81%E5%AE%9A%E4%B9%89)
	- [3.2 Transformer位置编码可视化](#3.2%20Transformer%E4%BD%8D%E7%BD%AE%E7%BC%96%E7%A0%81%E5%8F%AF%E8%A7%86%E5%8C%96)
	- [3.3 Transformer位置编码的重要性质](#3.3%20Transformer%E4%BD%8D%E7%BD%AE%E7%BC%96%E7%A0%81%E7%9A%84%E9%87%8D%E8%A6%81%E6%80%A7%E8%B4%A8)


## 一、什么是位置编码

在transformer的encoder和decoder的输入层中，使用了Positional Encoding，使得最终的输入满足：

$input = input\_embedding + positional\_encoding$

这里，input_embedding是通过常规embedding层，将每一个token的向量维度从vocab_size映射到`d_model`，由于是相加关系，自然而然地，这里的positional_encoding也是一个d_model维度的向量。（在原论文里，$d_{model}$ = 512）

在self-attention模型中，输入是一整排的tokens，对于人来说，我们很容易知道tokens的位置信息，比如：
（1）绝对位置信息。a1是第一个token，a2是第二个token......
（2）相对位置信息。a2在a1的后面一位，a4在a2的后面两位......
（3）不同位置间的距离。a1和a3差两个位置，a1和a4差三个位置....
但是这些对于self-attention来说，是无法分辩的信息，因为self-attention的运算是无向的。因为，我们要想办法，把tokens的位置信息，喂给模型。

## 二、为啥朴素的解法行不通

### 2.1 用整型值标记位置

一种自然而然的想法是，给第一个token标记1，给第二个token标记2...，以此类推。
这种方法产生了以下几个主要问题：
（1）模型可能遇见比训练时所用的序列更长的序列。不利于模型的泛化。
（2）模型的位置表示是无界的。随着序列长度的增加，位置值会越来越大。

### 2.2 用`[0,1]`范围标记位置

为了解决整型值带来的问题，可以考虑将位置值的范围限制在`[0, 1]`之内，其中，0表示第一个token，1表示最后一个token。比如有3个token，那么位置信息就表示成`[0, 0.5, 1]`；若有四个token，位置信息就表示成`[0, 0.33, 0.69, 1]`。

但这样产生的问题是，当序列长度不同时，token间的相对距离是不一样的。例如在序列长度为3时，token间的相对距离为0.5；在序列长度为4时，token间的相对距离就变为0.33。

>三个性质
因此，我们需要这样一种位置表示方式，满足于：
（1）它能用来表示一个token在序列中的绝对位置
（2）在序列长度不同的情况下，不同序列中token的相对位置/距离也要保持一致
（3）可以用来表示模型在训练过程中从来没有看到过的句子长度。

### 2.3 用二进制向量标记位置

考虑到位置信息作用在input embedding上，因此比起用单一的值，更好的方案是用一个和input embedding维度一样的向量来表示位置。这时我们就很容易想到二进制编码。如下图，假设$d_{model}$ = 3，那么我们的位置向量可以表示成：
![CleanShot 2025-07-19 at 21.18.44@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_8M2APprtEz%2F2025%2F07%2F19%2F21-18-51-0151cedf6272aa66456f584512b2c1a9-CleanShot%202025-07-19%20at%2021.18.44-2x-3ad352.png)

这下所有的值都是有界的（位于0，1之间），且transformer中的$d_{model}$本来就足够大，基本可以把我们要的每一个位置都编码出来了。

**这种编码方式也存在**“**问题**”，*违背了2.均匀性*，这样编码出来的位置向量，处在一个离散的空间中，不同位置间的变化是不连续的。
    - 这是最关键的问题。你想想，位置 0 是 `000`，位置 1 是 `001`。它们只有一个位不同。
    - 但位置 1 是 `001`，位置 2 是 `010`。它们有两个位不同。
    - 位置 3 是 `011`。
    - 如果只看二进制位数的改变，位置 `001` 到 `010` 的“距离”可能比 `000` 到 `001` 的“距离”更大（因为更多位变了）。
    - 这就导致了**相邻位置之间的“相似度”或“距离”并不是均匀变化的**。在真实世界里，位置1和位置2的距离应该和位置0和位置1的距离一样，都是“1步”。但二进制编码无法体现这种**连续的相对位置关系**。

举例：`d_model=2` 时，四个位置 `[0,0],[0,1],[1,0],[1,1]`
    - 位置0 (`00`) 和位置1 (`01`) 差1位。
    - 位置1 (`01`) 和位置2 (`10`) 差2位。
    - 这种跳跃式的变化，会让模型难以理解“相邻”和“远近”的连续概念。Transformer 需要理解的不仅仅是某个词在哪个位置，更重要的是**一个词相对于另一个词的“远近”关系**。这种离散的二进制编码很难捕捉这种连续的相对位置信息。
![CleanShot 2025-07-19 at 21.19.27@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_cO2DrwnXRP%2F2025%2F07%2F19%2F21-19-34-9c7eed31b5beefaee5c671daebd5ed0b-CleanShot%202025-07-19%20at%2021.19.27-2x-a36eea.png)
如果我们能把离散空间（黑色的线）转换到连续空间（蓝色的线），那么我们就能解决位置距离不连续的问题。同时，我们不仅能用位置向量表示整型，我们还可以用位置向量来表示浮点型。

### 2.4 用周期函数（sin）来表示位置

回想一下，现在我们需要一个有界又连续的函数，最简单的，正弦函数sin就可以满足这一点。我们可以考虑把位置向量当中的每一个元素都用一个sin函数来表示，则第t个token的位置向量可以表示为：

$$PE_t =[sin(\frac{1}{2^0}t),sin(\frac{1}{2^1}t)...,sin(\frac{1}{2^{i-1}}t),...,sin(\frac{1}{2^{d_{model}-1}}t)]$$

结合下图，来理解一下这样设计的含义。图中每一行表示一个 $PE_t$，每一列表示 $PE_t$ 中的第i个元素。旋钮用于调整精度，越往右边的旋钮，需要调整的精度越大，因此指针移动的步伐越小。每一排的旋钮都在上一排的基础上进行调整（函数中t的作用）。通过频率 $\frac{1}{2^{i-1}}$ 来控制sin函数的波长，频率不断减小，则波长不断变大，此时sin函数对t的变动越不敏感，以此来达到越向右的旋钮，指针移动步伐越小的目的。这也类似于二进制编码，每一位上都是0和1的交互，越往低位走（越往左边走），交互的频率越慢。
![CleanShot 2025-07-19 at 21.20.31@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_hSBDFy5hRG%2F2025%2F07%2F19%2F21-20-39-470ec0d3cbd6b63d2b2138212b7d7af9-CleanShot%202025-07-19%20at%2021.20.31-2x-8d68f5.png)

由于sin是周期函数，因此从纵向来看，如果函数的频率偏大，引起波长偏短，则不同t下的位置向量可能出现重合的情况。比如在下图中($d_{model}$ = 3），图中的点表示每个token的位置向量，颜色越深，token的位置越往后，在频率偏大的情况下，位置响亮点连成了一个闭环，靠前位置（黄色）和靠后位置（棕黑色）竟然靠得非常近：
![CleanShot 2025-07-19 at 21.25.39@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_Vu2SVlHRGu%2F2025%2F07%2F19%2F21-25-45-d4baad8f6acf672cc08ab55b0700b58a-CleanShot%202025-07-19%20at%2021.25.39-2x-1b0e19.png)

为了避免这种情况，我们尽量将函数的[[波长]]拉长。一种简单的解决办法是同一把所有的频率都设成一个非常小的值。因此在transformer的论文中，采用了 $\frac{1}{10000^{i/(d_{model}-1)}}$ 这个频率（这里i其实不是表示第i个位置，但是大致意思差不多，下面会细说）

总结一下，到这里我们把位置向量表示为：

$$PE_t =[sin(w_0t),sin(w_1t)...,sin(w_{i-1}t),...,sin(w_{d_{model}-1}t)]$$

其中，$w_{i} = \frac{1}{10000^{i/(d_{model}-1)}}$

### 2.5 用sin和cos交替来表示位置

目前为止，我们的位置向量实现了如下功能：
1. 能定位，每个token的向量唯一（每个sin函数的频率足够小）
2. 位置向量的值是有界的，且位于连续空间中。模型在处理位置向量时更容易泛化，即更好处理长度和训练数据分布不一致的序列（sin函数本身的性质）

那现在我们对位置向量再提出一个要求，**不同的位置向量是可以通过线性转换得到的**。这样，我们不仅能表示一个token的绝对位置，还可以表示一个token的相对位置，即我们想要：

$PE_{t+\Delta t} = T_{\Delta t} * PE_{t}$

这里，T表示一个线性变换矩阵。观察这个目标式子，联想到在向量空间中一种常用的线形变换——旋转。在这里，我们将t想象为一个角度，那么 $\Delta t$ 就是其旋转的角度，则上面的式子可以进一步写成：

$$\begin{pmatrix} \sin(t + \Delta t)\\ \cos((t + \Delta t) \end{pmatrix}=\begin{pmatrix} \cos\Delta t&\sin\Delta t \\ -\sin\Delta t&\cos\Delta t \end{pmatrix}\begin{pmatrix} \sin t\\ \cos t \end{pmatrix}$$

有了这个构想，我们就可以把原来元素全都是sin函数的 $PE_t$ 做一个替换，我们让位置两两一组，分别用sin和cos的函数对来表示它们，则现在我们有：

$$PE_t =[sin(w_0t),cos(w_0t), sin(w_1t),cos(w_1t),...,sin(w_{\frac{d_{model}}{2}-1}t), cos(w_{\frac{d_{model}}{2}-1}t)]$$

在这样的表示下，我们可以很容易用一个线性变换，把 $PE_t$ 转变为 $PE_{t + \Delta t}$:

$$PE_{t+\Delta t} = T_{\Delta t} * PE_{t} =\begin{pmatrix} \begin{bmatrix} cos(w_0\Delta t)& sin(w_0\Delta t)\\ -sin(w_0\Delta t)& cos(w_0\Delta t) \end{bmatrix}&...&0 \\...&...&...\\ 0&...& \begin{bmatrix} cos(w_{\frac{d_{model}}{2}-1 }\Delta t)& sin(w_{\frac{d_{model}}{2}-1}\Delta t)\\ -sin(w_{\frac{d_{model}}{2}-1}\Delta t)& cos(w_{\frac{d_{model}}{2}-1}\Delta t) \end{bmatrix} \end{pmatrix}\begin{pmatrix} sin(w_0t)\\ cos(w_0t)\\...\\ sin(w_{\frac{d_{model}}{2}-1}t)\\ cos(w_{\frac{d_{model}}{2}-1}t) \end{pmatrix} = \begin{pmatrix} sin(w_0(t+\Delta t))\\ cos(w_0(t+\Delta t))\\...\\ sin(w_{\frac{d_{model}}{2}-1}(t+\Delta t))\\ cos(w_{\frac{d_{model}}{2}-1}(t+\Delta t)) \end{pmatrix}$$

## 三、Transformer中位置编码方法：Sinusoidal functions

### 3.1 Transformer 位置编码定义

有了上面的演变过程后，现在我们就可以正式来看transformer中的位置编码方法了。

定义：
- t是这个token在序列中的实际位置（例如第一个token为1，第二个token为2...）
- $PE_t\in\mathbb{R}^d$ 是这个token的位置向量，$PE_{t}^{(i)}$ 表示这个位置向量里的第i个元素
- $d_{model}$ 是这个token的维度（在论文中，是512)

则 $PE_{t}^{(i)}$ 可以表示为：

$$PE_{t}^{(i)} = \left\{\begin{matrix} \sin(w_kt),&if\ i=2k \\ \cos(w_kt),&if\ i = 2k+1 \end{matrix}\right.$$

这里：
$w_k = \frac{1}{10000^{2k/d_{model}}}$
$k = 0,1,2,3,...,\frac{d_{model}}{2} -1$

看得有点懵不要紧，这个意思和2.5中的意思是一模一样的，把512维的向量两两一组，每组都是一个sin和一个cos，这两个函数共享同一个频率 $w_k$，一共有256组，由于我们从0开始编号，所以最后一组编号是255。sin/cos函数的波长（由 $w_k$ 决定）则从 $2\pi$ 增长到 $2\pi * 10000$

### 3.2 Transformer位置编码可视化

下图是一串序列长度为50，位置编码维度为128的位置编码可视化结果：
![CleanShot 2025-07-19 at 21.30.21@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_lN7GzmN99f%2F2025%2F07%2F19%2F21-30-30-a3dbb4cf105eafb3ca8a79c8997a989a-CleanShot%202025-07-19%20at%2021.30.21-2x-d731d7.png)

可以发现，由于sin/cos函数的性质，位置向量的每一个值都位于[-1, 1]之间。同时，纵向来看，图的右半边几乎都是蓝色的，这是因为越往后的位置，频率越小，[[波长]]越长，所以不同的t对最终的结果影响不大。而越往左边走，颜色交替的频率越频繁。

### 3.3 Transformer位置编码的重要性质

让我们再深入探究一下位置编码的性质。

**(1) 性质一：两个位置编码的点积(dot product)仅取决于偏移量 $\Delta t$，也即两个位置编码的点积可以反应出两个位置编码间的距离。**

证明：

$$\begin{aligned} PE_{t}^{T} \cdot PE_{t+\Delta t} &= \sum_{k = 0}^{\frac{d_{model}}{2}-1}[sin(w_kt)sin(w_k(t+\Delta t)) + cos(w_kt)cos(w_k(t+\Delta t))]\\ &= \sum_{k = 0}^{\frac{d_{model}}{2}-1}cos(w_k(t-(t+\Delta t)))\\ & = \sum_{k = 0}^{\frac{d_{model}}{2}-1}cos(w_k\Delta t) \end{aligned}$$

**(2) 性质二：位置编码的[[点积]]是无向的，即 $PE_{t}^{T} \cdot PE_{t+\Delta t} = PE_{t}^{T} \cdot PE_{t-\Delta t}$**

证明：
由于cos函数的对称性，基于性质1，这一点即可证明。
我们可以分别训练不同维度的位置向量，然后以某个位置向量 $PE_t$ 为基准，去计算其左右和它相距 $\Delta t$ 的位置向量的点积，可以得到如下结果：

这里横轴的k指的就是 $\Delta t$，可以发现，距离是对成分布的，且总体来说，$\Delta t$ 越大或者越小的时候，内积也越小，可以反馈距离的远近。也就是说，虽然位置向量的点积可以用于表示**距离(distance-aware)**，但是它却不能用来表示位置的**方向性(lack-of-directionality)**。

当位置编码随着input被喂进attention层时，采用的映射方其实是：

$PE_t^TW_Q^TW_KPE_{t+k}$
![CleanShot 2025-07-19 at 21.30.48@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_DcejqFDrPf%2F2025%2F07%2F19%2F21-30-57-c63706a1350415213a07a26b5d37dc56-CleanShot%202025-07-19%20at%2021.30.48-2x-24366d.png)

这里 $W_Q^T$ 和 $W_K$ 表示self-attention中的query和key参数矩阵，他们可以被简写成 $W$ 表示attention score的矩阵，到这里看不懂也没事，在self-attention的笔记里会说明的）。我们可以随机初始化两组 $W_1, W_2$，然后将 $PE_t^TW_1PE_{t+k}$，$PE_t^TW_2PE_{t+k}$ 和 $PE_t^TPE_{t+k}$ 这三个内积进行比较

![CleanShot 2025-07-19 at 21.32.11@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_bK3XRb7ZxL%2F2025%2F07%2F19%2F21-32-39-2e1f03868ebf93ff2eccaa1891ccd005-CleanShot%202025-07-19%20at%2021.32.11-2x-1bf7b5.png)



绿色和黄色即是 W1W_1 和 W2W_2 的结果。可以发现，进入attention层之后，内积的**距离意识(distance-aware)**的模式也遭到了破坏。更详细的细节，可以参见复旦大学这一篇用transformer做NER的[论文](https://link.zhihu.com/?target=https%3A//arxiv.org/pdf/1911.04474.pdf)中。
  
在Transformer的论文中，比较了用positional encoding和learnable position embedding(让模型自己学位置参数）两种方法，得到的结论是两种方法对模型最终的衡量指标差别不大。不过在后面的BERT中，已经改成用learnable position embedding的方法了，也许是因为positional encoding在进attention层后一些优异性质消失的原因（猜想）。Positional encoding有一些想象+实验+论证的意味，而编码的方式也不只这一种，比如把sin和cos换个位置，依然可以用来编码。关于positional encoding，我也还在持续探索中。

  
___



**为什么用那个复杂的 sin/cos 公式？**

简单说，这个公式设计得很巧妙，它生成的**位置向量**有几个好特性：

- 每个位置的向量都是**独一无**二的。
- 不同维度的 sin/cos 使用了不同的**频率**（分母上的 `10000^(...)`），就像用不同速度的波来编码位置。低频波（i 比较小）可以区分较远的位置，高频波（i 比较大）可以区分很近的位置。
- 模型可能更容易学习到**相对位置关系**。比如，位置 `pos+k` 的编码可以通过位置 `pos` 的编码进行线性变换得到，这有助于模型理解“前面第 k 个词”这样的概念。
