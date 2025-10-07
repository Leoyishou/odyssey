---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

就是[[Transformer架构]]某个函数的参数 ABC，起名叫 QKV，其中 K 和 V 相关的计算可以被缓存，Q 的缓存不了，所以只有 K cache 和 V cache。

# Cues



# Notes

## KV-cache介绍

* 先来看看Attention的公式：

    $$
    Attention(Q, K, V) = softmax(\frac{QK^T}{\sqrt{d_k}})V
    $$

* 现在主流的 [Decoder-only](Decoder-only.md) LLM 的自回归生成模式，每一个iteration都会将上一次输出的token作为输入，用以预测next token。
* 但是在这个过程中，对于每一个Attention Block的计算，其中Q只涉及当前最新位置上的一列vector (假设batchsize=1)。而K，V的虽然涉及所有位置上的vector，但是其中也只有一列是来自于最新位置的（fresh），其余位置都是以往已经计算过的（stale）。因此(3) 提出将这每一步stale的计算缓存在GPU HBM上，用以在下一个iteration进行计算。
* 详细公式推导可以参考(1)
* Q是对于当前token的编码，但是K、V是对于之前+当前token的编码矩阵
* 在Transformer架构中，只有self attention会进行token间交互，这里就会使用KV-cache：即记录下之前的kv矩阵
* 一般情况下，kv-cache常驻显存

## KV-cache显存计算

$$
\text{Size} = (\text{batch\_size} \times \text{seq\_len} \times \text{num\_layers} \times \text{num\_heads} \times \text{head\_dim}) \times 2 \times \text{precision}
$$

* **例子：OPT-30B**
    * `num_layers` = 48
    * `num_heads` = 56
    * `head_dim` = 128
    * `precision` = 2 (fp16)
    * `batch_size` = 1
    * `seq_len` = 2048
    * **Size** = (1 * 2048 * 48 * 56 * 128) * 2 * 2 = 2.8 GB



## 具体例子：生成 "I love AI" 序列

假设我们有一个简化的模型，其中 `d_model = 4`（隐藏维度）。

### Step 1: 输入 "I"，生成 "love"

**输入处理：**

- 输入 token: "I"
- 经过 embedding 和位置编码后得到向量：`h_1 =[1, 2, 3, 4]`

**计算 Q, K, V：**

```Java
Q_1 = W_Q × h_1 = [0.5, 1.0, 1.5, 2.0]  (1×4 矩阵)
K_1 = W_K × h_1 = [0.6, 1.2, 1.8, 2.4]  (1×4 矩阵)
V_1 = W_V × h_1 = [0.7, 1.4, 2.1, 2.8]  (1×4 矩阵)
```

**Attention 计算：**

```Java
Attention = softmax(Q_1 × K_1^T / √4) × V_1
          = softmax([7.2]) × V_1  
          = 1.0 × V_1
          = [0.7, 1.4, 2.1, 2.8]
```

**缓存 K_1 和 V_1** ✓

### Step 2: 输入 "I love"，生成 "AI"

**新输入处理：**

- 新 token: "love"
- 编码后：`h_2 =[2, 3, 4, 5]`

**只计算新位置的 Q, K, V：**

```Java
Q_2 = W_Q × h_2 = [1.0, 1.5, 2.0, 2.5]  (1×4 矩阵，只有当前位置)
K_2 = W_K × h_2 = [1.2, 1.8, 2.4, 3.0]  (1×4 矩阵，新位置)
V_2 = W_V × h_2 = [1.4, 2.1, 2.8, 3.5]  (1×4 矩阵，新位置)
```

**关键：使用 KV-cache！**

```Java
K_full = [K_1]  = [0.6, 1.2, 1.8, 2.4]  (2×4 矩阵)
         [K_2]    [1.2, 1.8, 2.4, 3.0]

V_full = [V_1]  = [0.7, 1.4, 2.1, 2.8]  (2×4 矩阵)
         [V_2]    [1.4, 2.1, 2.8, 3.5]
```

**Attention 计算：**

```Java
Attention = softmax(Q_2 × K_full^T / √4) × V_full
          = softmax([1.0×0.6+1.5×1.2+2.0×1.8+2.5×2.4,    # Q_2与K_1的点积
                     1.0×1.2+1.5×1.8+2.0×2.4+2.5×3.0])   # Q_2与K_2的点积
          = softmax([12.0, 16.2]) × V_full
          = [0.01, 0.99] × V_full
          ≈ 0.01×[0.7,1.4,2.1,2.8] + 0.99×[1.4,2.1,2.8,3.5]
          ≈ [1.39, 2.09, 2.79, 3.49]
```

**更新缓存：** K_cache = K_full, V_cache = V_full

### Step 3: 输入 "I love AI"，生成下一个 token

**新计算：**

```Java
Q_3 = W_Q × h_3  (1×4，只有最新位置)
K_3 = W_K × h_3  (1×4，只有最新位置)
V_3 = W_V × h_3  (1×4，只有最新位置)

K_full = [K_1]    (3×4 矩阵，前两行来自缓存)
         [K_2]
         [K_3]

V_full = [V_1]    (3×4 矩阵，前两行来自缓存)
         [V_2]
         [V_3]
```

## 内存节省示例

**不使用 KV-cache：**

- 每次都重新计算所有位置的 K, V
- Step 3 需要计算 3×3 = 9 次矩阵乘法

**使用 KV-cache：**

- 只计算新位置的 K, V
- Step 3 只需要计算 1×3 = 3 次矩阵乘法
- 节省了 66% 的计算量！

## 关键要点

1. **Q 始终只有当前位置**：因为我们只需要为当前 token 计算注意力分数
2. **K, V 包含所有历史**：需要让当前 token 能够 "看到" 所有之前的 token
3. **缓存的价值**：避免重复计算已经处理过的 token 的 K, V 值
4. **内存常驻**：KV-cache 保存在 GPU 显存中，用空间换时间

这就是为什么说 "Q 是对当前 token 的编码，而 K、V 是对之前+当前 token 的编码矩阵"。
