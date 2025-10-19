---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-05
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

> **“把常出现的字符组合在一起，直到能高效表示所有词”**。


场景：训练第一步需要[[Tokenization 分词]]，把原始文本转换成一个"词汇表"和"合并规则"，让模型知道怎么把文本切分成tokens
问题：词汇表从哪儿来？
解决：
1. 字符分词
2. BPE (Byte Pair Encoding)，BPE 就是自动建立一个这样的词汇表的过程

## 字符分词 vs BPE分词

```Java
字符分词：
  "我爱吃苹果" → [我, 爱, 吃, 苹, 果] # 5个token

BPE分词：
  "我爱吃苹果" → [我爱, 吃, 苹果] # 3个token ✅ 更高效
```

## 有点像[[MapReduce]]的思想，Pair是 Reduce 的依据

|对比点|BPE|MapReduce|
|---|---|---|
|核心任务|找出**最频繁出现的字符对**并合并|对大量数据做**分布式统计或计算**|
|思想|拆成局部统计 → 汇总全局频率 → 更新|拆分任务（Map）→ 汇总结果（Reduce）|
|操作模式|迭代统计 + 合并|并行分发 + 汇总合并|
|最终目的|压缩词表、提取高频“组合单元”|快速完成大规模数据聚合计算|

# Cues



# Notes

## 一、是什么？

**BPE训练是预处理步骤**，为Transformer提供合理的token划分。好的tokenizer让模型训练更高效，因为常见的词语组合被合并成了单个token。

这就是为什么现代大模型都需要先训练tokenizer，再训练语言模型的原因！

BPE (Byte Pair Encoding)，BPE 就是自动建立一个这样的词汇表的过程

```Java
# BPE根据数据自动学出来的vocab
vocab = {
	# 256个字节 (0-255)
	0: b'\x00', 1: b'\x01', ..., 230: b'我', 231: b'爱', ...
	
	# 算法自动发现的高频组合
	256: b'我爱', # 发现"我爱"经常一起出现
	257: b'苹果', # 发现"苹果"经常一起出现
	258: b'我爱吃', # 发现"我爱吃"经常出现
	259: b'。\n', # 发现"句号+换行"经常出现
	
	# 特殊tokens
	260: b'<eos>',
	261: b'<pad>',
}
```

自动的意思是：BPE不需要你告诉它哪些词重要，它自动从数据中发现：
```Java
# 假设训练文本：

text = """
	我爱吃苹果。我爱吃香蕉。
	你爱吃苹果吗？我爱吃苹果。
	我爱吃苹果和香蕉。
"""  

# BPE自动发现：
# "我爱" 出现5次 → 合并成一个token
# "吃苹果" 出现4次 → 合并成一个token
# "苹果" 出现4次 → 合并成一个token
```


```Java
原始文本: "The cat"
    ↓ BPE tokenizer (用这里训练的vocab和merges)
Token IDs: [1523, 8901]  # 对应词汇表中的ID
    ↓ 输入给 Transformer
模型输出: 下一个token的概率分布
```



## 二、详细流程分析

### 1. **初始化阶段** (行105-113)
```python
# 读取训练文本
text = "The cat sat on the mat"  # 示例
text_bytes = [84, 104, 101, 32, 99, 97, 116, ...]  # UTF-8字节

# 初始词汇表：256个字节值
vocab = {0: b'\x00', 1: b'\x01', ..., 84: b'T', 104: b'h', 101: b'e', 32: b' ', ...}

# 初始tokens：每个字节是一个token  
tokens = [b'T', b'h', b'e', b' ', b'c', b'a', b't', b' ', ...]
```

### 2. **迭代合并阶段** (行116-145)
每一轮做4件事：

#### A. 统计相邻pairs的频率
```python
pair_counts = {
    (b'T', b'h'): 5,      # "Th"组合在文本中出现5次
    (b'h', b'e'): 8,      # "he"组合出现8次  
    (b'e', b' '): 12,     # "e "组合出现12次
    # ...
}
```

#### B. 找到最频繁的pair
```python
best_pair = (b'e', b' ')  # 假设"e "最常见
```

#### C. 创建新的合并token
```python
new_token = b'e' + b' ' = b'e '  # 合并成新token
vocab[256] = b'e '              # 添加到词汇表，ID=256
merges.append((b'e', b' '))     # 记录合并规则
```

#### D. 替换文本中所有该pair
```python
# 之前: [b'T', b'h', b'e', b' ', b'c', b'a', b't', b'e', b' ']
# 之后: [b'T', b'h', b'e ', b'c', b'a', b't', b'e ']  # 所有"e "被合并
```

### 3. **添加特殊tokens** (行147-149)
```python
# 最后添加特殊tokens
vocab[512] = b'<eos>'     # 句子结束符
vocab[513] = b'<pad>'     # 填充符
```

## 三、具体例子演示

假设训练文本是 `"aaa bbb aaa"`：

```Java
轮次0: tokens = [b'a', b'a', b'a', b' ', b'b', b'b', b'b', b' ', b'a', b'a', b'a']
       最频繁pair: (b'a', b'a') 出现4次
       合并后: [b'aa', b'a', b' ', b'b', b'b', b'b', b' ', b'aa', b'a']

轮次1: tokens = [b'aa', b'a', b' ', b'b', b'b', b'b', b' ', b'aa', b'a'] 
       最频繁pair: (b'b', b'b') 出现2次
       合并后: [b'aa', b'a', b' ', b'bb', b'b', b' ', b'aa', b'a']

轮次2: tokens = [b'aa', b'a', b' ', b'bb', b'b', b' ', b'aa', b'a']
       最频繁pair: (b'aa', b'a') 出现2次  
       合并后: [b'aaa', b' ', b'bb', b'b', b' ', b'aaa']
```

最终输出

```python
vocab = {
    # 原始字节
    97: b'a', 98: b'b', 32: b' ', 
    # 学到的合并
    256: b'aa',    # 第1轮学到
    257: b'bb',    # 第2轮学到  
    258: b'aaa',   # 第3轮学到
    # 特殊tokens
    259: b'<eos>',
}

merges = [
    (b'a', b'a'),      # 规则1：'a'+'a' → 'aa'
    (b'b', b'b'),      # 规则2：'b'+'b' → 'bb'  
    (b'aa', b'a'),     # 规则3：'aa'+'a' → 'aaa'
]
```
