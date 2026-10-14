---
uid: 7a176873-57df-4ade-b67e-b6aa79fb89ee
date created: 2025-07-19
date modified: 2025-10-13
---
# Summary


1. **比解析文本更稳定**：不用担心模型输出 `"YES!!!"` / `"yes it does"` 之类的花样字符串。
2. **能得到概率分布**：不仅知道模型“选了 Yes”，还能知道它给 Yes 0.9、No 0.1 的分布。
3. **比赛需求**：像 Kaggle Jigsaw 这类比赛要交概率而不是 hard label，这种方法直接契合。



为什么要这么做
- **普通做法**：大模型生成一个字符串（比如 `"Yes"` 或 `"No"`），然后我们去解析这个文本。
- **这里的做法**：在 `llm.generate(..., logprobs=2)` 里，已经让 vLLM 把每个位置可能 token 的 **logits → logprob** 全部返回。
    - `MultipleChoiceLogitsProcessor` 把搜索空间限制成了 **["Yes", "No"]**。
    - 所以我们直接拿到这两个候选的 logprob。
- 这样就能直接得到“模型认为 Yes 的概率是多少 / No 的概率是多少”，而不是靠解码的最终字符串。
    

---

## 为什么不直接看生成结果

- **生成结果只有一个 token**，丢失了概率信息。
    
    - 模型可能生成 `"Yes"`，但其实 `"No"` 的概率也很接近。
        
- 在比赛（比如 Jigsaw）里，提交需要的是 **一个概率值[0,1]**，而不是单个分类标签。
    
    - 所以直接用 logprob，可以更好地反映模型的不确定性。
        
    - 再经过 softmax → 得到 Yes 的概率分布 → 就能作为提交分数。




对于[生成式模型](2%20第二大脑/1%20概念/4%20信息与模式/CS/人工智能/算法/任务类型/表征学习/生成式模型.md)做分类任务，直接取 logits 比生成文本后再提取结果更高效

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
