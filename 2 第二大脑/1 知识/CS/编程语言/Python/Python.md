---
draw:
title: Python
date created: 2023-12-18
date modified: 2025-03-26
---






2. 布尔逻辑判断，坚持用`and`**, `or`**, **`not`**

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F01%2F16-16-11-6bce63c81909280b883a0e96da394ff0-202410011616703-32886c.png)

## 高频代码

```python
import re
from collections import Counter

# 假设的代码内容字符串
code_content = """
[这里是完整的代码内容]
"""

# 正则表达式匹配方法调用
method_pattern = r'(\w+)\s*\('

# 找出所有的方法调用
method_calls = re.findall(method_pattern, code_content)

# 统计方法调用频率
method_counter = Counter(method_calls)

# 获取前20个最常用的方法
top_20_methods = method_counter.most_common(20)

print("Top 20 most frequently used methods:")
for method, count in top_20_methods:
    print(f"{method}: {count}")

```

根据对您提供的代码的分析,以下是项目中最常用的20个Python方法及其使用频次:

| Rank | Method        | Usage Count | 刻意练习         |     |
| ---- | ------------- | ----------- | ---------------- | --- |
| 1    | Column        | 38          |                  |     |
| 2    | mapped_column | 15          |                  |     |
| 3    | relationship  | 14          |                  |     |
| 4    | print         | 13          |                  |     |
| 5    | get           | 10          |                  |     |
| 6    | add           | 9           |                  |     |
| 7    | post          | 8           |                  |     |
| 8    | open          | 7           |                  |     |
| 9    | strip         | 7           |                  |     |
| 10   | lower         | 6           |                  |     |
| 11   | append        | 6           |                  |     |
| 12   | join          | 5           |                  |     |
| 13   | load_dotenv   | 5           |                  |     |
| 14   | getenv        | 5           |                  |     |
| 15   | write         | 5           |                  |     |
| 16   | create_engine | 4           |                  |     |
| 17   | decode        | 4           |                  |     |
| 18   | json          | 4           |                  |     |
| 19   | dumps         | 4           | Object -> String |     |
| 20   | loads         | 4           | String -> Object |     |

这些方法可以大致分为几类:

1. 数据库相关: Column, mapped_column, relationship, create_engine
2. HTTP请求: get, post
3. 文件操作: open, write
4. 字符串处理: strip, lower, join
5. 环境变量: load_dotenv, getenv
6. 数据结构操作: append
7. JSON处理: json, dumps, loads
8. 编码/解码: decode
9. 输出: print

建议:

1. 重点学习SQLAlchemy ORM的使用,特别是Column和mapped_column的用法。
2. 熟悉HTTP请求库(如requests)的get和post方法。
3. 练习文件操作,包括打开、读取、写入文件。
4. 掌握常用的字符串处理方法,如strip()、lower()和join()。
5. 学习如何处理环境变量,使用load_dotenv()和os.getenv()。
6. 熟悉JSON数据的处理,包括dumps()和loads()方法。
7. 了解编码/解码的基本概念和方法。

通过重点学习这些高频使用的方法,您将能更好地理解和编写类似的Python项目。同时,这也反映了在实际开发中常见的任务和操作,如数据库交互、HTTP请求、文件处理等。

```python
for page in reader.pages:  
    text = page.extract_text()

for word, count in pdf_words.most_common():
	new_words.append((word, count))

with open(txt_path, 'r', encoding='utf-8') as file:
```

110 道 Python 面试题（真题）- 不加班程序员的文章 - 知乎  
https://zhuanlan.zhihu.com/p/54430650

## python 学习

类的成员方法为什么要传 self

从 Python 3.3 开始，引入了"隐式命名空间包"的概念，这意味着在某些情况下，即使没有 `__init__.py` 文件，目录也可以被视为包。但是，为了兼容性和明确性，仍然建议使用 `__init__.py`。

## 代码风格

[PEP8](PEP8.md)

## python有些小工具库使用起来特别好用

  

如

Celery，用于[分布式计算](https://zhida.zhihu.com/search?content_id=601910841&content_type=Answer&match_order=1&q=%E5%88%86%E5%B8%83%E5%BC%8F%E8%AE%A1%E7%AE%97&zhida_source=entity)，定时任务

ZeroMQ，方便实现c/c++通信交互，有很多网络模式。

ping3，可以ping网络

psutil，可以用于[进程管理](https://zhida.zhihu.com/search?content_id=601910841&content_type=Answer&match_order=1&q=%E8%BF%9B%E7%A8%8B%E7%AE%A1%E7%90%86&zhida_source=entity)

gtts，Google的文本到语音

pydub 音频处理库

moviepy [视频编辑](https://zhida.zhihu.com/search?content_id=601910841&content_type=Answer&match_order=1&q=%E8%A7%86%E9%A2%91%E7%BC%96%E8%BE%91&zhida_source=entity)

PIL 图片处理

selenium 基于浏览器的测试助手，爬虫

PySide6 官方版的基于python的qt库

Ray [并行计算](https://zhida.zhihu.com/search?content_id=601910841&content_type=Answer&match_order=1&q=%E5%B9%B6%E8%A1%8C%E8%AE%A1%E7%AE%97&zhida_source=entity)，机器学习框架

dask 分布式数据分析框架

cupy 基于gpu的[numpy](https://zhida.zhihu.com/search?content_id=601910841&content_type=Answer&match_order=1&q=numpy&zhida_source=entity)
