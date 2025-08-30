---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-26
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

    rmsnorm 是 Root Mean Square Normalization (均方根归一化) 的缩写

# Cues



# Notes

我来用一个具体的小例子演示RMSNorm的计算过程。

假设我们有一个输入向量 **$x =[2, 4, 6, 8]$**，维度为4。

## 计算步骤

**第1步：计算均方根（RMS）**

$$
\begin{aligned}
\text{RMS} &= \sqrt{\frac{1}{n} \sum_{i=1}^{n} x_i^2} = \sqrt{\frac{1}{4} \times (2^2 + 4^2 + 6^2 + 8^2)} \\
&= \sqrt{\frac{1}{4} \times (4 + 16 + 36 + 64)} \\
&= \sqrt{\frac{1}{4} \times 120} \\
&= \sqrt{30} \\
&\approx 5.477
\end{aligned}
$$

**第2步：归一化**

将每个元素除以RMS：
- $x_1' = 2 / 5.477 \approx 0.365$
- $x_2' = 4 / 5.477 \approx 0.730$
- $x_3' = 6 / 5.477 \approx 1.095$
- $x_4' = 8 / 5.477 \approx 1.461$

归一化后：**$x' =[0.365, 0.730, 1.095, 1.461]$**

**第3步：缩放（可选）**

假设缩放参数 $\gamma =[1.2, 0.8, 1.0, 1.5]$（通常是可学习的参数）

最终输出：
- $y_1 = 0.365 \times 1.2 = 0.438$
- $y_2 = 0.730 \times 0.8 = 0.584$
- $y_3 = 1.095 \times 1.0 = 1.095$
- $y_4 = 1.461 \times 1.5 = 2.192$

最终结果：**$y =[0.438, 0.584, 1.095, 2.192]$**

## 与LayerNorm的区别

如果用LayerNorm处理同样的数据：
1. 先计算均值：$\mu = (2+4+6+8)/4 = 5$
2. 计算方差：$\sigma^2 =[(2-5)^2 + (4-5)^2 + (6-5)^2 + (8-5)^2]/4 = 5$
3. 标准化：$x' = (x - \mu) / \sqrt{\sigma^2 + \epsilon}$

可以看到，RMSNorm跳过了计算均值和减去均值的步骤，直接使用均方根进行归一化，计算更简单高效。

---

我来将代码与之前的计算过程对应起来。让我用同样的数据 **$x =[2, 4, 6, 8]$** 来解释每行代码：

## 代码逐行解析

**1. `x_sq = x.pow(2)`**

计算每个元素的平方
$x_{sq} =[2^2, 4^2, 6^2, 8^2]=[4, 16, 36, 64]$


**2. `mean_sq = x_sq.mean(-1, keepdim=True)`**

计算平方的均值（沿最后一个维度）
$mean_{sq} = (4 + 16 + 36 + 64) / 4 = 120 / 4 = 30$


**3. `r_rms = torch.rsqrt(mean_sq + eps)`**
计算倒数平方根 (reciprocal square root)
$r_{rms} = 1 / \sqrt{30 + 1e-5} \approx 1 / \sqrt{30} \approx 1 / 5.477 \approx 0.1826$

**4. `y = x * r_rms * g`**
归一化并缩放
假设 $g =[1.2, 0.8, 1.0, 1.5]$

$y_1 = 2 \times 0.1826 \times 1.2 = 0.438$
$y_2 = 4 \times 0.1826 \times 0.8 = 0.584$
$y_3 = 6 \times 0.1826 \times 1.0 = 1.095$
$y_4 = 8 \times 0.1826 \times 1.5 = 2.192$

## 关键点对应

1. **RMS计算优化**：
   - 手动计算：$RMS = \sqrt{\text{mean}(x^2)}$，然后 $x' = x / RMS$
   - 代码优化：直接计算 $r_{rms} = 1/\sqrt{\text{mean}(x^2)}$，然后 $y = x \times r_{rms}$

2. **$eps$的作用**：
   - 防止除零错误
   - 提高数值稳定性

3. **保存中间变量**：
   ```python
   ctx.save_for_backward(x, g, r_rms)
   ```
   这是为反向传播保存必要的中间结果。

## 完整示例验证：完美！代码的计算结果与我们手动计算的结果一致

总结一下代码的巧妙之处：
- 使用 `torch.rsqrt()` 直接计算倒数平方根，避免了先开方再求倒数
- 使用 `keepdim=True` 保持维度，便于后续的广播运算
- 整个计算过程简洁高效，只需要一次平方、一次均值、一次倒数平方根和一次乘法运算
