---
date created: 2025-04-06
date modified: 2025-10-05
uid: 7fd966cd-e94a-41e9-a1b9-9064bc55188d
---
Temperature 通过缩放 logits 来控制概率分布的"尖锐度"：
`logits = logits[:, -1,:]/ temperature`

1. temperature = 1.0（默认值）
	- 不改变 logits，保持原始概率分布
	- 平衡的随机性
2. temperature < 1.0（例如 0.5）
	- logits 被放大（除以小于1的数）
	- 经过 softmax 后，概率分布更加尖锐/集中
	- 模型更倾向选择最可能的 token
	- 输出更确定、保守、重复性高
3. temperature > 1.0（例如 1.5）
	- logits 被缩小（除以大于1的数）
	- 经过 softmax 后，概率分布更加平滑/均匀
	- 模型会考虑更多可能性较低的 token
	- 输出更多样、创造性、但可能不连贯

是一个用于控制模型回答的"随机性"或"创造性"的参数。
    - 当温度接近 0 时，模型更倾向于选择概率最高的词语，回答会相对保守、确定性较高且重复率可能较高。
    - 当温度较高时，模型回答会更具多样性和创意，可能更有"发散思维"，但同时不确定性也增加，回答可能显得更不稳定。


结果验证：`model_gpt.generate` 将基于 GPT-2 继续生成文本。`max_length=50` 指定总长度上限（包含初始prompt的tokens）。对贪婪搜索，模型每一步都选概率最高的词，输出确定；对随机采样，我们设置了 `do_sample=True`，并使用 top-k 和 top-p (nucleus) 采样方法来引入随机选择（top_k=50表示每步仅从概率最高的50个词中抽样，top_p=0.9表示从累计概率0.9的候选中抽样），这些技术通常使文本更有趣且避免太离谱的词。

您会看到两段不同的续写，例如：

```Java
贪婪生成:
 In a distant future, humans and robots are not the only ones who have been living in the past. The world is a place where people are not allowed to be...

随机采样生成:
 In a distant future, humans and robots wage war side by side. The last remnants of humanity dwell in bunkers beneath a scarlet sky, while autonomous machines patrol the wastelands...
```
