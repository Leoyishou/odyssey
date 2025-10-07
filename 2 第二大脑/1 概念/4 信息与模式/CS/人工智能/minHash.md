---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-06
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 🧩 Summary  

## 场景

：训练 LLM 时,数据集可能有几十亿条文本:
- 直接逐对比较相似度 → 计算量爆炸 O(n²)
- 需要快速找出"几乎重复"的文本并删除

```Java
小红书数据特点：
- 爆款帖子被大量抄袭/改写
- "杭州三日游攻略" 有100篇相似内容

不去重的后果：
检索结果全是重复内容 → LLM生成千篇一律
```

## 解决

解决：把文本转换成一个**小的指纹(signature)**,相似的文本有相似的指纹。**MinHash** 是一种**快速找出相似文本**的算法,在大规模数据清洗中用来**去重**。



### **MinHash工作原理（通俗版）**



```python
# 类比：比较两篇作文相似度

传统方法（慢）：
逐字比对，计算相同词占比

MinHash方法（快）：
1. 把文章分成词的集合
   文章A = {杭州, 西湖, 灵隐寺, 三日游, 攻略}
   文章B = {杭州, 西湖, 雷峰塔, 三日游, 推荐}

2. 用N个哈希函数"采样"
   Hash1(A) = 灵隐寺,  Hash1(B) = 雷峰塔  ✗
   Hash2(A) = 西湖,    Hash2(B) = 西湖    ✓
   Hash3(A) = 杭州,    Hash3(B) = 杭州    ✓
   
3. 相同采样占比 ≈ 真实相似度
   2/3 ≈ 0.67 → 判定为重复

# 实际代码
from datasketch import MinHash, MinHashLSH

lsh = MinHashLSH(threshold=0.7, num_perm=128)

for doc_id, text in documents:
    m = MinHash(num_perm=128)
    for word in text.split():
        m.update(word.encode('utf8'))
    lsh.insert(doc_id, m)
    
# 查询相似文档
result = lsh.query(m)  # 秒级返回
```

**参数建议：**

- `threshold=0.7`：相似度>70%视为重复
- `num_perm=128`：准确度与速度平衡点




## 通俗例子

假设有两篇文章:

```Java
文章A: "今天天气很好,适合出去玩"
文章B: "今天天气不错,适合出去玩"
       (只有"很好"换成了"不错")
```

**传统方法:**

```python
# 需要比较完整的文本
similarity = compare_full_text(A, B)  # 慢!
```

**MinHash 方法:**

```python
# 1. 先把文本转成"指纹"
hash_A = minhash(A)  # 例如: [3, 7, 12, 45, 89]
hash_B = minhash(B)  # 例如: [3, 7, 12, 48, 91]

# 2. 比较指纹(超快!)
similarity = compare_hash(hash_A, hash_B)  # 3/5 = 60% 相似
```

## MinHash 工作原理

### 1. 文本 → N-gram 集合

```python
文本: "今天天气很好"

# 提取 3-gram (shingles)
shingles = {
    "今天天",
    "天天气", 
    "天气很",
    "气很好"
}
```

### 2. 多次哈希,取最小值

```python
# 用很多个哈希函数(比如 128 个)
def minhash_signature(shingles):
    signature = []
    for i in range(128):  # 128 个哈希函数
        min_hash = min(hash_i(s) for s in shingles)
        signature.append(min_hash)
    return signature

# 得到一个 128 维的向量
sig_A = [3, 7, 12, 45, 89, ...]  # 128 个数
sig_B = [3, 7, 12, 48, 91, ...]
```

### 3. 比较签名

```python
# Jaccard 相似度估计
similarity = sum(a == b for a, b in zip(sig_A, sig_B)) / 128
# 如果 100/128 位相同 → 约 78% 相似
```

## 在 LLM 数据清洗中的应用

### 场景: CommonCrawl 网页去重

```python
# 有 10 亿个网页
# 目标: 删除几乎重复的页面

# 步骤:
for document in corpus:
    # 1. 计算 MinHash 签名
    signature = compute_minhash(document)
    
    # 2. 用 LSH (Locality Sensitive Hashing) 快速找相似文档
    similar_docs = lsh_index.query(signature)
    
    # 3. 如果相似度 > 80%,标记为重复
    if similar_docs:
        mark_as_duplicate(document)
```

### 实际案例

**GPT-3 的数据清洗:**

- 使用 MinHash 去除重复文档
- 过滤掉低质量、过度重复的内容

**LLaMA 的数据清洗:**

- 用 MinHash 在 CommonCrawl 中去重
- 相似度阈值设为 0.8 (80% 相似就删除)

## 为什么比直接比较快?

```Java
直接比较:
10亿个文档 × 10亿个文档 = 10^18 次比较 ❌

MinHash + LSH:
1. 每个文档算一次签名: 10亿次
2. LSH 分桶,只比较同一桶里的
3. 总比较次数 ≈ 10^8 次 ✅ (快 10^10 倍!)
```

## 代码示例(简化版)

```python
from datasketch import MinHash

# 文档 A
mh1 = MinHash(num_perm=128)
for word in "今天天气很好".split():
    mh1.update(word.encode('utf8'))

# 文档 B
mh2 = MinHash(num_perm=128)
for word in "今天天气不错".split():
    mh2.update(word.encode('utf8'))

# 估计 Jaccard 相似度
print(mh1.jaccard(mh2))  # 0.75 (75% 相似)
```

## 你说的"模型结构比较收敛"

是的,现在的趋势确实是:

✅ **模型架构:** 基本就是 Transformer + 一些小改进

- Pre-norm vs Post-norm
- RoPE, ALiBi 等位置编码
- 激活函数(SwiGLU, GeGLU)

✅ **数据质量更重要:**

- 去重(MinHash)
- 去毒(过滤有害内容)
- 多样性(不同领域平衡)
- 质量过滤(去除低质量文本)

**"Data-centric AI"** - 与其设计新架构,不如清洗更好的数据!




# 💡 Cues



# Notes
