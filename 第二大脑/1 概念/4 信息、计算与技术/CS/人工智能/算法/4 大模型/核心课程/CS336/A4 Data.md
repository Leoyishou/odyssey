---
uid: b98004dd-74e5-41ea-8ce4-d49022783d73
date created: 2025-07-19
date modified: 2025-11-01
---
# Summary

> **一句话**：Lab 4 让你体会「垃圾进 → 垃圾出」的现实——同样的网络结构，喂优质文本就能显著压低验证损失。

## 做了什么

- 整体目标：[[数据清洗]]、构建一个数据预处理管道，将原始网络数据转换为高质量的训练数据
- 为什么重要：大语言模型训练需要海量高质量数据，但原始数据往往包含垃圾、重复、有害内容，需要清洗

##  Pipeline

1. 收集原始数据：从网络爬取HTML页面
2. 提取文本：去掉HTML标签，保留文字内容
3. 语言筛选：只保留目标语言（如英语）
4. 隐私保护：遮盖个人信息
5. 内容审核：过滤不当内容
6. 质量检查：应用质量规则
7. 去重处理：删除重复内容
8. 输出训练数据：得到干净、高质量的训练文本

# Cues

[[FastText]]
[[N-gram]]


# Notes

## 数据

下面是已修正、对齐的 Markdown 表格，内容保持不变，只是格式化为标准的 5 列布局，便于阅读和复制。

| 数据/文件名                 | 用途              | 使用位置（代码）| 来源/备注              |
| ---------------------- | --------------- | ------------------------------------------------ | ------------------ |
| **NLTK 分词数据**<br>punkt | Gopher 规则过滤中的分词 | cs336_data/quality.py:13                         | 运行时按需自动下载          |
| **训练语料（GPT‑2 已分词二进制）** | 训练语言模型          | cs336-basics/configs/experiment/your_data.yaml:5 | 外部路径（集群/自备），不随仓库分发 |
| **验证集（GPT‑2 已分词二进制）**  | 训练时验证           | cs336-basics/configs/experiment/your_data.yaml:8 | 外部路径（集群提供），不随仓库分发  |

## 测试用例总结

### Level 1: 原子级组件测试 (最小粒度)

| 测试文件            | 测试函数                                      | 测试目的                        | 状态  |
| --------------- | ----------------------------------------- | --------------------------- | --- |
| test_extract.py | test_extract_text_from_html_bytes         | 验证HTML文本提取功能，从HTML字节流中提取纯文本 |     |
| test_langid.py  | test_identify_language_english            | 验证英语语言识别功能                  | ✅   |
| test_langid.py  | test_identify_language_chinese_simplified | 验证中文语言识别功能                  | ✅   |
| test_pii.py     | test_mask_emails_single                   | 验证单个邮箱地址的PII脱敏功能            | ✅   |
| test_pii.py     | test_mask_emails_multiple                 | 验证多个邮箱地址的PII脱敏功能            | ✅   |
| test_pii.py     | test_mask_phones_single                   | 验证单个电话号码的PII脱敏功能            | ✅   |
| test_pii.py     | test_mask_ips                             | 验证IP地址的PII脱敏功能              | ✅   |

### Level 2: 组件级测试 (小型组合单元)

| 测试文件             | 测试函数                             | 测试目的                          | 状态  |
| ---------------- | -------------------------------- | ----------------------------- | --- |
| test_pii.py      | test_mask_emails_existing_string | 验证已存在脱敏标记的文本处理                | ✅   |
| test_quality.py  | test_classify_quality            | 验证文本质量分类功能（区分高质量wiki和低质量cc文本）| ✅   |
| test_toxicity.py | test_classify_nsfw               | 验证NSFW内容分类功能                  | ✅   |
| test_toxicity.py | test_classify_toxic_speech       | 验证有害言论分类功能                    | ✅   |

### Level 3: 模块级测试 (复杂组合组件)

| 测试文件 | 测试函数 | 测试目的 | 状态 |
|---------|---------|---------|------|
| test_quality.py | test_gopher_valid_input | 验证Gopher质量过滤器的有效输入处理 | ⚠️ |
| test_quality.py | test_gopher_less_than_50_non_symbol_words | 验证少于50个非符号词的文本过滤 | ⚠️ |
| test_quality.py | test_gopher_more_than_100000_non_symbol_words | 验证超过10万个非符号词的文本过滤 | ⚠️ |
| test_quality.py | test_gopher_average_word_length_less_than_3 | 验证平均词长小于3的文本过滤 | ⚠️ |
| test_quality.py | test_gopher_average_word_length_greater_than_10 | 验证平均词长大于10的文本过滤 | ⚠️ |
| test_quality.py | test_gopher_more_than_30_percent_lines_ending_with_ellipsis | 验证超过30%行以省略号结尾的文本过滤 | ⚠️ |
| test_quality.py | test_gopher_less_than_80_percent_words_with_alphabetic_character | 验证少于80%词包含字母字符的文本过滤 | ⚠️ |

### Level 4: 系统级测试 (完整系统/管道)

| 测试文件                  | 测试函数                                        | 测试目的                  | 状态  |
| --------------------- | ------------------------------------------- | --------------------- | --- |
| test_deduplication.py | test_exact_line_deduplication               | 验证精确行去重功能，处理多个文档文件    | ⚠️  |
| test_deduplication.py | test_minhash_deduplication_exact_duplicates | 验证MinHash算法对精确重复文档的去重 | ⚠️  |
| test_deduplication.py | test_minhash_deduplication_fuzzy_duplicates | 验证MinHash算法对模糊重复文档的去重 | ✅   |

## 项目结构

```Java
assignment4-data/
├── cs336_data/                    # 你的实现目录（基本为空）
│   └── __init__.py
├── cs336-basics/                  # 来自assignment1的预训练模型代码
│   ├── cs336_basics/
│   ├── pyproject.toml
│   └── scripts/
├── tests/                         # 测试文件
│   ├── fixtures/                  # 测试数据
│   │   ├── moby.html             # HTML测试文件
│   │   ├── moby_extracted.txt    # 期望的提取结果
│   │   ├── high_quality_wiki_reference.txt
│   │   ├── low_quality_cc.txt
│   │   ├── documents_with_line_duplicates/
│   │   ├── documents_line_deduplicated/
│   │   └── documents_with_fuzzy_duplicates/
│   ├── adapters.py               # 测试适配器（需要实现）
│   ├── test_extract.py           # HTML文本提取测试
│   ├── test_langid.py            # 语言识别测试
│   ├── test_pii.py               # PII脱敏测试
│   ├── test_quality.py           # 文本质量测试
│   ├── test_toxicity.py          # 有害内容测试
│   └── test_deduplication.py     # 去重测试
├── pyproject.toml                # 项目配置
└── README.md
```


```Java
assignment4-data/
├── fasttext (核心分类器)
├── nltk (文本处理)
├── transformers (预训练模型)
├── tiktoken (分词)
├── resiliparse (网页解析)
├── fastwarc (数据格式)
└── 其他工具包
```


## 项目代码执行流程

### 1. UV 环境管理器启动

```bash
uv run pytest
```

当执行这个命令时：
1. UV 读取 `pyproject.toml`，解析项目依赖
2. 创建或激活虚拟环境
3. 在虚拟环境中执行 pytest

### 2. Pytest 初始化流程

2.1 配置加载
```toml
# pyproject.toml 中的 pytest 配置
[tool.pytest.ini_options]
testpaths = ["tests"]
pythonpath = ["."]
```

2.2 测试发现
```Java
Pytest 扫描 tests/ 目录，找到所有 test_*.py 文件：
tests/
├── test_extract.py
├── test_langid.py
├── test_pii.py
├── test_quality.py
├── test_toxicity.py
└── test_deduplication.py
```

### 3. 测试执行流程（以 test_extract.py::test_extract_text_from_html_bytes 为例）

3.1 Fixture 加载
```python
# tests/common.py
FIXTURES_PATH = Path(__file__).parent / "fixtures"
```

3.2 测试函数执行
```python
# tests/test_extract.py:8-17
def test_extract_text_from_html_bytes():
    # 1. 读取HTML测试文件
    moby_path = FIXTURES_PATH / "moby.html"
    with open(moby_path, "rb") as f:
        moby_bytes = f.read()
    
    # 2. 读取期望结果
    moby_expected_path = FIXTURES_PATH / "moby_extracted.txt"
    with open(moby_expected_path) as f:
        moby_expected_text = f.read()
    
    # 3. 调用适配器函数
    actual_output = run_extract_text_from_html_bytes(moby_bytes)
    
    # 4. 验证结果
    assert moby_expected_text == actual_output
```

3.3 适配器调用
```python
# tests/adapters.py:6-8
def run_extract_text_from_html_bytes(html_bytes: bytes) -> str | None:
    # 原本是: raise NotImplementedError
    # 现在是: 调用你的实现
    return extract_text_from_html_bytes(html_bytes)
```

3.4 实际实现执行
```python
# cs336_data/extract.py (你需要创建)
def extract_text_from_html_bytes(html_bytes: bytes) -> str | None:
    """从HTML字节流中提取纯文本"""
    # 你的实现代码
    pass
```

## 实现各组件的代码

你需要实现以下核心功能模块：

### 1. 文本提取模块 (extract.py)
```python
def extract_text_from_html_bytes(html_bytes: bytes) -> str | None:
    """从HTML字节流中提取纯文本内容"""
    # 使用BeautifulSoup或其他HTML解析库
    pass
```

### 2. 语言识别模块 (langid.py)
```python
def identify_language(text: str) -> tuple[str, float]:
    """识别文本语言，返回语言代码和置信度"""
    # 使用langid或fasttext等库
    pass
```

### 3. PII脱敏模块 (pii.py)
```python
def mask_emails(text: str) -> tuple[str, int]:
    """脱敏邮箱地址，返回处理后的文本和脱敏数量"""
    pass

def mask_phone_numbers(text: str) -> tuple[str, int]:
    """脱敏电话号码"""
    pass

def mask_ips(text: str) -> tuple[str, int]:
    """脱敏IP地址"""
    pass
```

### 4. 内容分类模块 (classifiers.py)
```python
def classify_nsfw(text: str) -> tuple[str, float]:
    """分类NSFW内容"""
    pass

def classify_toxic_speech(text: str) -> tuple[str, float]:
    """分类有害言论"""
    pass

def classify_quality(text: str) -> tuple[str, float]:
    """分类文本质量"""
    pass
```

### 5. 质量过滤模块 (quality.py)
```python
def gopher_quality_filter(text: str) -> bool:
    """Gopher质量过滤器，实现多个质量检查规则"""
    pass
```

### 6. 去重模块 (deduplication.py)
```python
def exact_line_deduplication(input_files: list[Path], output_directory: Path):
    """精确行去重"""
    pass

def minhash_deduplication(
    input_files: list[Path],
    num_hashes: int,
    num_bands: int,
    ngrams: int,
    jaccard_threshold: float,
    output_directory: Path,
):
    """MinHash模糊去重"""
    pass
```

## 训练数据

Assignment 4 的数据处理管道包括：

### 1. 原始数据来源
- HTML网页内容
- 多语言文本
- 包含PII的文本
- 不同质量的文本（wiki vs cc）
- 重复和相似文档

### 2. 数据处理流程
```Java
原始数据 → 文本提取 → 语言识别 → PII脱敏 → 质量过滤 → 去重 → 训练数据
```

### 3. 数据质量检查
- **Gopher质量过滤器**：检查词数、词长、标点符号等
- **内容安全**：NSFW和有害言论检测
- **重复检测**：精确重复和模糊重复

## 核心组件的作用

| 组件 | 作用 | 通俗解释 |
|------|------|----------|
| **HTML文本提取** | 从网页中提取纯文本 | 就像"剥洋葱"，去掉HTML标签，只保留文字内容 |
| **语言识别** | 判断文本语言 | 像"翻译官"，告诉计算机这段文字是什么语言 |
| **PII脱敏** | 隐藏个人信息 | 像"涂黑笔"，把邮箱、电话、IP等敏感信息遮盖 |
| **内容分类** | 判断文本类型和质量 | 像"质检员"，区分好坏内容，过滤垃圾信息 |
| **质量过滤** | 应用质量规则 | 像"筛子"，过滤掉太短、太长、质量差的文本 |
| **去重** | 删除重复内容 | 像"整理员"，去掉重复和相似的内容，节省存储空间 |

## Assignment 4: Data 通俗解释

### 它干嘛的？
- **整体目标**：构建一个数据预处理管道，将原始网络数据转换为高质量的训练数据
- **为什么重要**：大语言模型训练需要海量高质量数据，但原始数据往往包含垃圾、重复、有害内容，需要清洗

### 要实现的东西
你要实现一个完整的数据处理流水线，包括：

1. **文本提取**：从HTML网页中提取纯文本
2. **语言识别**：判断文本语言，过滤非目标语言
3. **隐私保护**：脱敏邮箱、电话、IP等个人信息
4. **内容过滤**：检测和过滤NSFW、有害言论
5. **质量评估**：应用Gopher等质量规则
6. **去重处理**：删除重复和相似内容

### 数据处理流程（通俗版）
1. **收集原始数据**：从网络爬取HTML页面
2. **提取文本**：去掉HTML标签，保留文字内容
3. **语言筛选**：只保留目标语言（如英语）
4. **隐私保护**：遮盖个人信息
5. **内容审核**：过滤不当内容
6. **质量检查**：应用质量规则
7. **去重处理**：删除重复内容
8. **输出训练数据**：得到干净、高质量的训练文本

完成后，你就有了一个能自动处理原始网络数据、输出高质量训练数据的完整管道！🎉




## 1. Lab 4 在整个课程中的定位

|作业|关键词|你要掌握的核心技能|产出物|
|---|---|---|---|
|Lab 1 _(Basics)_|**Transformer** 最小实现|搭积木：Embedding、Self‑Attention、FFN、优化器|能跑通一个 100 M 参数级别的中文/英文小模型|
|Lab 2 _(Efficiency)_|内核融合、Flash‑Attn、混合精度|把 Lab 1 加速到单卡吃得下、跑得快|秒/step↓、显存↓|
|Lab 3 _(Scaling)_|数据/模型并行、ZeRO|多 GPU 多机训练|Gbit 以上带宽下稳定训练|
|**Lab 4 _(Data)_**|**高质量语料管道**|清洗 + 去重 + 分词 + 分片 + 验证|1. `cs336_data`  Python package2. 一个.tar.gz 提交包3. 在官方 leaderboard 取得越低的 val loss 越好 ([GitHub](https://raw.githubusercontent.com/stanford-cs336/assignment4-data/main/README.md "raw.githubusercontent.com"))|
|Lab 5 _(Alignment)_|RLHF / DPO|安全、守护式微调|基于人偏好对话模型|

---

## 2. 官方仓库结构速览

```Java
assignment4-data/
├── cs336-basics/     # 教学组给的“参考实现”训练脚本
├── cs336_data/       # 现在几乎为空，等你填逻辑
├── README.md         # 作业说明 & 目录约定
└── test_and_make_submission.sh
```

`README.md` 明确写道：`cs336_data` “**基本是空的！这里才是你要写代码的地方**” ([GitHub](https://raw.githubusercontent.com/stanford-cs336/assignment4-data/main/README.md "raw.githubusercontent.com"))。
此外还有一个独立仓库 `assignment4-data-leaderboard` 用来记成绩；表头按 _Validation Loss_ 排序 ([GitHub](https://raw.githubusercontent.com/stanford-cs336/assignment4-data-leaderboard/main/README.md "raw.githubusercontent.com"))。

---

## 3. Lab 4 要解决的 5 件事

> 下文每条都给出「**为什么**要做」→「**怎么**做」→「可选进阶」→「常见坑」。

### 3.1 解析原始 Common Crawl 分片

- **为什么**  
    原始 WARC/CC-MAIN 文件动辄 TB 级，既含 HTML 标签也混着非文本资源。
    
- **怎么做**  
    使用 `warcio` 或 `gzip` 流式读取；遇到 `Content-Type: text/html` 时用 `BeautifulSoup` 抽正文。控制台打印进度 + 错误计数，别一次性读进内存。
    
- **进阶**
    - 只保留你关心的语言（如英文）→ 快速 `langdetect`，后续更准确用 `fasttext`。
        
- **常见坑**
    - `warcio` 在部分坏块上会抛 `ArchiveLoadFailed`; 要 `try/except` 跳过。
        

### 3.2 文本质量过滤

|过滤维度|建议阈值|检测方法|
|---|---|---|
|Unicode 可打印比例|≥ 0.9|`sum(c.isprintable() for c in text)/len(text)`|
|非字母字符比例|≤ 0.3|正则|
|重复行 / boiler‑plate|去掉 `<header>、<nav>、cookie banner` 等|XPath、`justext`|

- **进阶**：
    - 训练一个极小 GPT‑2 (≈ 25 M) 做 _ppl filter_——剔除困惑度>100 的文档。
        
- **常见坑**：
    - 先 `strip()` 后再测长度；别把全空行当高质量文本。
        

### 3.3 去重

- **为什么**  
    重复文本会让模型过拟合，虚高 loss。
    
- **怎么做**
    - 粗去重：`hashlib.md5(text)` 放到 `set()`；
    - 近似去重：`simhash` 或 MinHash；`datasketch` 库能做 128‑bit MinHash + LSH。
        
- **进阶**
    - 在 token 级别用分块 sort‑shuffle，其效果与 _ExactDedup_ 相近但内存亲民。
        
- **常见坑**
    - 切记分片并行化后仍要全局去重，否则同一 url 会分到不同 worker，结果重复。
        

### 3.4 分词与分片

- **为什么**  
    训练脚本固定用 `tiktoken`‑like BPE；一条样本默认 2048 token。
    
- **怎么做**
    
    ```python
    enc = tiktoken.get_encoding("gpt2")
    ids = enc.encode(text)
    for i in range(0, len(ids) - 2048, 2048):
        yield ids[i:i+2048]
    ```
    
- **进阶**
    
    - 按 “自然段落” 拼接再截断，能减少句子被腰斩的概率。
        
- **常见坑**
    
    - token 长度不足 64 时可丢弃——短上下文对语言模型价值不大。
        

### 3.5 生成可提交的数据集

- **要求**
    
    1. 把所有 `token_id` 序列写成 `uint16` / `uint32` NumPy 数组，压成 `npz` 或 `bin`。
        
    2. `./test_and_make_submission.sh` 能一键跑完并输出 `val_loss=...`。
        
- **验证**
    
    - 用 `cs336-basics` 提供的训练脚本跑 1‑2 epoch；只看 _validation loss curve_ 即可，下降越快通常说明你的数据更“干净”。
        

---

## 4. 与 Lab 1 的联系 & 差异

|维度|Lab 1 (模型)|Lab 4 (数据)|
|---|---|---|
|主要代码|`cs336_basics`|`cs336_data`|
|关注点|**算子实现与训练 loop**|**原料质量**|
|成功指标|模型在固定数据集的 _val loss/perplexity_|固定模型在你数据上的 _val loss_|
|典型 debug|梯度爆炸 / 学不到|loss 不降或波动大＝数据脏|

---

## 5. 逐步落地指南（给 Java 后端工程师的最小可行路径）

1. **先把仓库跑通**：
    
    ```bash
    git clone https://github.com/stanford-cs336/assignment4-data.git
    cd assignment4-data
    uv pip install -e .
    ./test_and_make_submission.sh  # 先跑 baseline，确认环境 ok
    ```
    
2. **写数据管道**（推荐放 `cs336_data/pipeline.py`）：
    
    - 用 `IterableDataset`，边读边 token 化，保证 < 3 GB 内存也能跑。
        
    - 日志用 Python `logging`，打 INFO 行数、WARN 错误数。
        
3. **本地 sanity check**：
    
    ```python
    from cs336_data.pipeline import build_dataset
    ds = build_dataset("/path/to/raw")
    print(next(iter(ds)))  # 应该是长度 2048 的 list[int]
    ```
    
4. **小规模训练**：
    
    - 设置 `num_train_tokens=5e6`，单 3090 约 15 min 即能看到趋势。
        
    - 追踪验证集 loss，如果 > baseline 的 4.0 就说明过滤做坏了。
        
5. **优化**：
    
    - 尝试不同语言检测+去重组合，每改一次都留 _commit_，方便 A/B Test。
        
    - 观察 loss 曲线的斜率；前 1000 step 确定改动好坏，剩下靠耐心。
        
6. **打包 & 提交**：
    
    ```bash
    ./test_and_make_submission.sh  # 生成 submission.tar.gz
    # 按 README 指示 PR 至 leaderboard 仓库
    ```
    

---

## 6. 常见报错速查表

|报错信息|可能原因|快速修复|
|---|---|---|
|`UnicodeDecodeError: 'utf-8' codec can't decode`|直接用 `open(..., 'r')` 读 gzip|用 `gzip.open` + `.decode("utf‑8", errors='ignore')`|
|`CUDA out of memory`|token 序列过长 or batch 太大|调小 `batch_size`, 或把 `sequence_length` 固定 1024/1536|
|验证 loss 一直是 `nan`|出现空样本 / 非数字 token|pipeline 最后加 `assert len(seq) == 2048 and all(isinstance(x, int))`|

---

## 7. 延伸阅读 & 工具

|主题|推荐资源|
|---|---|
|Common Crawl 解析|[https://commoncrawl.org](https://commoncrawl.org/) 官方 FAQ|
|质量过滤 heuristics|[Pile 论文 §3.1]、[OpenWebText2 数据集博客]|
|近似去重算法|“SimHash: hashing for similarity” (Google 2002)|
|多进程流水线|Python `multiprocessing` + `queue.SimpleQueue`|
|分词器|`tiktoken`, `sentencepiece`, `huggingface/tokenizers`|

---

### 小结

- **核心心法**：数据质量>模型大小——想象你在调优搜索引擎，先把网页垃圾扫干净。
    
- **最小目标**：实现 ➊ 基本清洗 + ➋ 行级 MD5 去重 + ➌ BPE 分词 + ➍ 固定 2048‑token 切片。做到这四步，验证 loss 就能比 baseline（4.0↑）明显下降，踏进 leaderboard 前列没问题。
    
- **进阶方向**：软指标过滤（ppl filter）、多语言混合策略、MinHash 去重、Dataset‑streaming。
    

祝你玩得开心，也欢迎把你的 leaderboard 名次分享出来！
