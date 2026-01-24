---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  



# 🧩 Cues



# 🪞Notes
Firecrawl 是一个专门为 AI 和大语言模型（LLM）设计的网页爬取与数据提取工具。简单来说，它的核心能力是将任意网页或整个网站转换成 LLM 易于理解的干净数据（如 Markdown）。

https://www.firecrawl.dev/app 是它的管理后台（Dashboard），核心服务能力如下：

1. 核心功能模块
Scrape（单页爬取）：

输入一个 URL，将其内容提取为干净的 Markdown 格式（这对 LLM 训练和 RAG 非常友好）。

支持截取网页截图。

支持提取 HTML 源码。

PDF/DOCX 解析： 可以直接读取并解析网页上的 PDF 和 Word 文档内容。

Crawl（全站爬取）：

输入一个主页链接，它会自动遍历整个网站的所有子页面（无需 Sitemap）。

适合获取整个文档站、博客或知识库的数据。

Search（网页搜索）：

这是其较新的功能，支持直接搜索互联网内容并返回完整的页面数据（不仅仅是搜索引擎的摘要）。

Map（链接映射）：

快速获取某个网站下的所有 URL 链接列表，用于了解站点结构。

Extract（结构化提取）：

配合 LLM 能力，根据你提供的 Schema（结构定义），从网页中提取指定的结构化数据（如提取所有的价格、产品名称、日期等 JSON 格式数据）。