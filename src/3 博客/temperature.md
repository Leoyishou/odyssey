---
date created: 2025-04-06
date modified: 2025-07-10
uid: 7fd966cd-e94a-41e9-a1b9-9064bc55188d
---

结果验证：`model_gpt.generate` 将基于 GPT-2 继续生成文本。`max_length=50` 指定总长度上限（包含初始prompt的tokens）。对贪婪搜索，模型每一步都选概率最高的词，输出确定；对随机采样，我们设置了 `do_sample=True`，并使用 top-k 和 top-p (nucleus) 采样方法来引入随机选择（top_k=50表示每步仅从概率最高的50个词中抽样，top_p=0.9表示从累计概率0.9的候选中抽样），这些技术通常使文本更有趣且避免太离谱的词。

您会看到两段不同的续写，例如：

```Java
贪婪生成:
 In a distant future, humans and robots are not the only ones who have been living in the past. The world is a place where people are not allowed to be...

随机采样生成:
 In a distant future, humans and robots wage war side by side. The last remnants of humanity dwell in bunkers beneath a scarlet sky, while autonomous machines patrol the wastelands...
```
