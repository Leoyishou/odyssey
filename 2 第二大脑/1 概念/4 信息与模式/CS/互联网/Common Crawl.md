---
uid: 2895dde8-4f90-489f-b4b3-272053bb0fbb
date created: 2025-11-01
date modified: 2025-11-01
---
# Summary

**一句话总结**：Common Crawl 是互联网的"公共图书馆"，为 AI 研究提供了训练大型语言模型所需的海量真实网页数据，几乎所有现代 LLM（GPT、LLaMA、T5 等）都使用它作为训练语料的一部分。

**Common Crawl** 是一个非营利 501(c)(3) 组织，负责爬取网页并免费向公众提供其档案和数据集
Common Crawl 构建并维护一个开放的网页爬取数据仓库，任何人都可以访问和分析

## 核心特点

### 1. **规模惊人**

Common Crawl 的网页档案包含自 2008 年以来收集的 PB 级数据，大约每月完成一次爬取

这是一个由超过 500 亿个网页组成的网页爬取数据语料库

```Java
数据量：PB 级别（1 PB = 1000 TB）
网页数：500+ 亿
时间跨度：2008 年至今
更新频率：每月一次
```

### 2. **完全免费、开放**

爬取数据对来自任何地方的任何人都是免费的。数据由亚马逊网络服务的开放数据集赞助计划托管

**访问方式**：

- AWS S3: `s3://commoncrawl/`
- HTTPS: `https://data.commoncrawl.org/`
- 可以在 AWS 云中处理或通过 HTTP(S) 免费下载

---

## 数据格式

Common Crawl 目前使用 Web ARChive (WARC) 格式存储爬取数据

### 三种文件类型

|格式|全称|内容|大小|
|---|---|---|---|
|**WARC**|Web ARChive|原始 HTML + HTTP 请求/响应|最大|
|**WAT**|Web Archive Transformation|元数据（链接、HTTP 头等）JSON 格式|中等|
|**WET**|Web Extracted Text|提取的纯文本|最小|

```python
# WARC 文件示例结构
WARC/1.0
WARC-Type: response
WARC-Target-URI: http://example.com/page.html
Content-Length: 1234

HTTP/1.1 200 OK
Content-Type: text/html

<html>
  <body>网页内容...</body>
</html>
```

---

## 语言分布

在 2023 年 3 月版本的 Common Crawl 数据集中，英语是 46% 文档的主要语言。接下来最常见的主要语言是德语、俄语、日语、法语、西班牙语和中文，每种语言的文档比例不到 6%

```Java
英语：46%
德语、俄语、日语、法语、西语、中文：各 <6%
其他语言：剩余部分
```

---

## 典型应用场景

### 1. **训练大型语言模型（LLM）**

几乎所有主流 LLM 都使用 Common Crawl 数据：

|模型|使用的数据集|基于 Common Crawl|
|---|---|---|
|**GPT-3**|WebText|✅|
|**T5**|C4 (Colossal Clean Crawled Corpus)|✅|
|**BERT**|BooksCorpus + Wikipedia|部分 ✅|
|**LLaMA**|Common Crawl + 其他|✅|
|**GPT-4**|未公开，但很可能包含|✅|

Google 的 Common Crawl 版本称为 Colossal Clean Crawled Corpus（简称 C4），它是为 2019 年训练 T5 语言模型系列而构建的

---

### 2. **NLP 研究**

- 词向量训练（GloVe、Word2Vec）
- 多语言模型训练
- 文本分类、情感分析数据集构建

---

### 3. **网络安全研究**

Common Crawl 数据集可以帮助快速发现额外的攻击面，并识别敏感信息的暴露

```sql
-- 示例：查找暴露的数据库备份文件
SELECT url FROM "ccindex"."ccindex" 
WHERE crawl = 'CC-MAIN-2022-27' 
  AND fetch_status=200 
  AND content_mime_detected = 'application/x-sql'
```

---

### 4. **搜索引擎研究**

ChatNoir 是由 Webis 研究组开发的网络搜索引擎，基于 ClueWeb09、ClueWeb12 和 Common Crawl 数据集

---


## Common Crawl vs 其他数据源

|数据源|规模|质量|成本|适用场景|
|---|---|---|---|---|
|**Common Crawl**|极大|中等（需清洗）|免费|LLM 预训练|
|**Wikipedia**|中等|高|免费|知识密集型任务|
|**Reddit/Twitter**|大|较低|部分免费|对话/社交文本|
|**Books Corpus**|中等|高|有版权限制|长文本理解|
|**自己爬取**|可控|可控|高（技术+法律成本）|特定领域|

## 历史与发展

Common Crawl 由 Gil Elbaz 创立，亚马逊网络服务于 2012 年通过其公共数据集项目开始托管 Common Crawl 的档案

**时间线**：

- **2008**：开始收集数据
- **2012**：AWS 开始托管，免费开放
- **2013**：改用 Apache Nutch 爬虫
- **2019**：Google 基于 CC 创建 C4 数据集（用于 T5）
- **至今**：每月持续更新

---

## 总结

|特性|说明|
|---|---|
|**是什么**|全球最大的开放网页爬取数据集|
|**规模**|500+ 亿网页，PB 级数据|
|**访问**|完全免费，AWS S3/HTTPS|
|**格式**|WARC/WAT/WET|
|**更新**|每月一次|
|**主要用途**|训练 LLM、NLP 研究、搜索引擎|
|**在 CS336 中**|Assignment 4 的核心数据源|
