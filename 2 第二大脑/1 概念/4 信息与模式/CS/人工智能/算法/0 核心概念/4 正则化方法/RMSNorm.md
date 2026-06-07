---
uid: bf3c39fc-06c1-4444-a387-833c3277f925
date created: 2025-07-19
date modified: 2025-09-22
---
# Summary

	RMSNorm 是 Root Mean Square Normalization (均方根归一化) 的缩写
	
最大的优势：整个计算过程简洁高效，只需要一次平方、一次均值、一次倒数平方根和一次乘法运算

# Cues



# Notes

## 一、RMSNorm 在[Transformer](Transformer.md)中的哪一步？

为什么使用Pre-Norm架构？
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F09%2F22%2F12-00-37-f26f111d1c80e3a270d7c50ac70163d6-202509221200736-c3cbdb.png)

这种设计有几个优势：
1. 训练稳定性：RMSNorm在残差连接之前，有助于梯度流动
2. 计算效率：避免了Post-Norm需要额外的归一化步骤
3. 现代标准：大多数现代Transformer都采用Pre-Norm架构

### 1. 每个Transformer Block内部（Pre-Norm架构）
在每个TransformerBlock中，RMSNorm被使用了两次：
```python
# 第一次：在注意力机制之前
attn_input = rmsnorm(weights["ln1.weight"], x)  # 第37行
attn_out = multihead_self_attention(...)
x = x + attn_out  # 残差连接

# 第二次：在FFN之前  
ffn_input = rmsnorm(weights["ln2.weight"], x)   # 第55行
ffn_out = swiglu(...)
x = x + ffn_out  # 残差连接
```

### 2. 最终层归一化（Final Layer Norm）
在所有Transformer Block之后，输出层之前：
```python
# Final layer norm
x = rmsnorm(weights["ln_final.weight"], x, eps=1e-5)  # 第120行
```


## 二、RMSNorm 计算过程

假设我们有一个输入向量 **$x =[2, 4, 6, 8]$**，维度为4。
### **第1步：计算均方根（RMS）**

$$
\begin{aligned}
\text{RMS} &= \sqrt{\frac{1}{n} \sum_{i=1}^{n} x_i^2} = \sqrt{\frac{1}{4} \times (2^2 + 4^2 + 6^2 + 8^2)} \\
&= \sqrt{\frac{1}{4} \times (4 + 16 + 36 + 64)} \\
&= \sqrt{\frac{1}{4} \times 120} \\
&= \sqrt{30} \\
&\approx 5.477
\end{aligned}
$$

### **第2步：归一化**

将每个元素除以RMS：
	- $x_1' = 2 / 5.477 \approx 0.365$
	- $x_2' = 4 / 5.477 \approx 0.730$
	- $x_3' = 6 / 5.477 \approx 1.095$
	- $x_4' = 8 / 5.477 \approx 1.461$

归一化后：**$x' =[0.365, 0.730, 1.095, 1.461]$**

### **第3步：缩放（可选）**

假设缩放参数 $\gamma =[1.2, 0.8, 1.0, 1.5]$（通常是可学习的参数）

最终输出：
	- $y_1 = 0.365 \times 1.2 = 0.438$
	- $y_2 = 0.730 \times 0.8 = 0.584$
	- $y_3 = 1.095 \times 1.0 = 1.095$
	- $y_4 = 1.461 \times 1.5 = 2.192$

最终结果：**$y =[0.438, 0.584, 1.095, 2.192]$**


## 三、与LayerNorm的区别

如果用[[LayerNorm]]处理同样的数据：
1. 先计算均值：$\mu = (2+4+6+8)/4 = 5$
2. 计算方差：$\sigma^2 =[(2-5)^2 + (4-5)^2 + (6-5)^2 + (8-5)^2]/4 = 5$
3. 标准化：$x' = (x - \mu) / \sqrt{\sigma^2 + \epsilon}$

可以看到，RMSNorm跳过了计算均值和减去均值的步骤，直接使用均方根进行归一化，计算更简单高效。

## 四、代码逐行解析

用同样的数据 **$x =[2, 4, 6, 8]$** 来解释每行代码：

**1. `x_sq = x.pow(2)`** 计算每个元素的平方
$x_{sq} =[2^2, 4^2, 6^2, 8^2]=[4, 16, 36, 64]$

**2. `mean_sq = x_sq.mean(-1, keepdim=True)`** 计算平方的均值（沿最后一个维度）
$mean_{sq} = (4 + 16 + 36 + 64) / 4 = 120 / 4 = 30$

**3. `r_rms = torch.rsqrt(mean_sq + eps)`** 计算倒数平方根 (reciprocal square root)
$r_{rms} = 1 / \sqrt{30 + 1e-5} \approx 1 / \sqrt{30} \approx 1 / 5.477 \approx 0.1826$

**4. `y = x * r_rms * g`** 归一化并缩放
假设 $g =[1.2, 0.8, 1.0, 1.5]$

$y_1 = 2 \times 0.1826 \times 1.2 = 0.438$
$y_2 = 4 \times 0.1826 \times 0.8 = 0.584$
$y_3 = 6 \times 0.1826 \times 1.0 = 1.095$
$y_4 = 8 \times 0.1826 \times 1.5 = 2.192$

## 三、其他细节

1. **RMS计算优化**：
   - 手动计算：$RMS = \sqrt{\text{mean}(x^2)}$，然后 $x' = x / RMS$
   - 代码优化：直接计算 $r_{rms} = 1/\sqrt{\text{mean}(x^2)}$，然后 $y = x \times r_{rms}$

2. **[[eps]]的作用**：
   - 防止除零错误
   - 提高数值稳定性

3. **保存中间变量**：
   ```python
   ctx.save_for_backward(x, g, r_rms)
   ```
   这是为反向传播保存必要的中间结果。


总结一下代码的巧妙之处：
- 使用 `torch.rsqrt()` 直接计算倒数平方根，避免了先开方再求倒数
- 使用 `keepdim=True` 保持维度，便于后续的广播运算
- 整个计算过程简洁高效，只需要一次平方、一次均值、一次倒数平方根和一次乘法运算
