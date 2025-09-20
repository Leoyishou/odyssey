---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-14
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

对于[生成式模型](2%20第二大脑/1%20概念/4%20信息与模式/CS/人工智能/深度学习/表征学习/生成式模型.md)做分类任务，直接取 logits 比生成文本后再提取结果更高效

```Java
输入 → Encoder → Hidden States → Logits → Softmax → 采样 → Token ID → 解码成文本
                                    ↑                      ↓
                              (所有词汇的概率)         "0.75"
```


```Java
输入 → Encoder → Hidden States → Logits → 我们直接在这里截取！
                                    ↑
                            (只看 Yes/No 的概率)
```


## 🚀 **为什么这样更高效**
1. **计算量减少**
    - 不需要多次前向传播（生成多个token）
    - 只需要一次前向传播到 logits 层
2. **避免了采样的随机性**
    - 即使 temperature=0，采样仍可能有数值误差
    - 直接获取概率是确定性的
3. **信息更完整**
    - 获得了完整的概率分布
    - 不是模型"选择告诉你"的数字

## 💡 **类比理解**

**标准方法**：
- 问学生："这道题选A还是B？"
- 学生思考后写下："我选A，把握度75%"
- 你读取学生写的答案

**LogitsProcessor方法**：
- 直接用脑电波扫描学生大脑
- 看到大脑中：A区域活跃度75%，B区域25%
- 不需要学生写出来

# Cues



# Notes

## **标准 Decoder 过程 vs LogitsProcessor 方法**

### 方法一：从结果文本中正则提取


```Java
输入 → Encoder → Hidden States → Logits → Softmax → 采样 → Token ID → 解码成文本
                                    ↑                      ↓
                              (所有词汇的概率)         "0.75"
```


```python
# 生成
outputs = llm.generate(
	batch_prompts,
	sampling_params,
	lora_request=lora_request,
	use_tqdm=True
)

# 解析输出
for output in outputs:
	text = output.outputs[0].text.strip()
	prob = parse_probability(text)
	all_probabilities.append(prob)

# ==================== 解析概率 ====================
def parse_probability(text):
    """从文本中解析概率值"""
    try:
        # 尝试提取数字
        import re
        match = re.search(r'(\d*\.?\d+)', text[:10])
        if match:
            prob = float(match.group(1))
            return min(max(prob, 0.0), 1.0)
        
        # 文本判断
        text_lower = text.lower()[:20]
        if 'yes' in text_lower or 'violate' in text_lower:
            return 0.8
        elif 'no' in text_lower or 'not' in text_lower:
            return 0.2
        
    except:
        pass
    
    return 0.5  # 默认值
```



```python
# 步骤更多，计算量更大
1. 计算所有 50000+ 个token的logits
2. Softmax 归一化
3. 采样选择 token（比如 "0"）
4. 继续生成下一个 token（"."）
5. 继续生成（"7"）
6. 继续生成（"5"）
7. 解码成文本 "0.75"
8. 正则提取
```





### 方法二：logit 分类


```Java
输入 → Encoder → Hidden States → Logits → 我们直接在这里截取！
                                    ↑
                            (只看 Yes/No 的概率)
```


```python
df["prompt"] = prompts

mclp = MultipleChoiceLogitsProcessor(tokenizer, choices=['Yes','No'])
outputs = llm.generate(
    prompts,
    vllm.SamplingParams(
        skip_special_tokens=True,
        max_tokens=1,
        logits_processors=[mclp],
        logprobs=2,
    ),
    use_tqdm=True,
    lora_request=LoRARequest("default", 1, LORA_PATH)
)
logprobs = [
    {lp.decoded_token: lp.logprob for lp in out.outputs[0].logprobs[0].values()}
    for out in outputs
]
logit_matrix = pd.DataFrame(logprobs)[['Yes','No']]
```



```python
# 步骤少，更高效
1. 计算所有 token 的 logits
2. 只看 "Yes" 和 "No" 的 logits
3. 计算这两个的概率
4. 直接返回概率分布
# 结束！不需要实际生成token
```
