---
uid: 190770aa-bb81-4365-9b2e-d2e6ed691951
date created: 2025-11-01
date modified: 2025-11-01
---
# Summary

 **Resili**ent（韧性、容错）**Parse**（解析）

**Resiliparse** 是一个**高性能的网页档案分析工具包**（robust web archive analytics toolkit），由 ChatNoir（Webis 研究组）开发 [Chatnoir](https://resiliparse.chatnoir.eu/en/stable/)###

1. Resiliparse 提供了一个轻量级、快速的 HTML 解析和 DOM 处理库，基于 **Lexbor 网页浏览器引擎**
2. **编码检测**提供基于 uchardet 的通用字符编码检测器
3. **FastWARC**（WARC 文件解析）FastWARC 是一个用 C++/Cython 编写的高性能 WARC 解析库，支持压缩和未压缩的 WARC/1.0 和 WARC/1.1 流。**WARC**（Web ARChive）是存储网页爬取数据的标准格式，Common Crawl 就使用这种格式。

## 与其他 HTML 解析库对比

|库|速度|容错性|适用场景|
|---|---|---|---|
|**BeautifulSoup**|慢|好|小规模爬虫、学习|
|**lxml**|快|中|中等规模爬虫|
|**Resiliparse**|**极快**|**极好**|**大规模数据处理（Common Crawl）**|
|html.parser|慢|差|Python 标准库，简单任务|
