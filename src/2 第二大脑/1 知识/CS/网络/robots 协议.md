---
draw:
tags: []
title: robots 协议
date created: 2024-07-04
date modified: 2024-11-12
---
- 目的: 规定爬虫可以访问网站的哪些部分,以及如何访问。
- 实现: 通常通过在网站根目录下放置一个名为 "robots.txt" 的文本文件来实现。
- 主要指令:
    - Allow: 允许访问的目录或文件
    - Disallow: 禁止访问的目录或文件
    - Sitemap: 指定网站地图的位置
    - User-agent: 指定适用的爬虫
- 非强制性:robots 协议是一种**君子协定**,爬虫可以选择遵守或忽略。
