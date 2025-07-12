---
date created: 2024-05-08
date modified: 2025-07-10
uid: f9e319fc-0c1d-4bdc-8b16-9e12fd21a86f
---
1. 想象你有一个超大的玩具箱，里面装满了各种各样的玩具。
2. Kibana 就像是一个魔法镜子，可以帮你快速找到你想要的玩具，还能告诉你有多少个。
3. 不仅如此，它还能把信息变成漂亮的图画，比如饼图或柱状图，让你一眼就能看懂有多少种玩具，哪种玩具最多。
4. 如果你想知道昨天和今天的玩具有什么不同，Kibana 也能帮你对比。
5. 它还能设置警报，比如当某种玩具快没有了，它会提醒你。

- Elasticsearch 是大玩具箱
- Kibana 是魔法眼镜
- Logstash 就是帮你收集和整理玩具的机器人朋友

<!-- more -->

用 ELK（Elasticsearch+Logstash+Kibana）技术栈，自行搭建一个日志存储和分析系统。

1. [ElasticSearch](ElasticSearch.md):
    - Elasticsearch 是一个分布式、实时的搜索和分析引擎。它用于存储和检索大量的数据，支持全文搜索、结构化搜索和复杂查询。Elasticsearch 的强大之处在于其分布式性能、高可用性和水平扩展能力，使得它成为处理日志和大数据的理想选择。
2. [Kibana](Kibana.md):
    - Kibana 是一个用于数据可视化和分析的工具。它允许用户通过创建仪表盘、图表和可视化图像来探索存储在 Elasticsearch 中的数据。Kibana 提供了一个直观的用户界面，使用户能够通过交互式方式进行数据分析和可视化。
3. **Logstash:**
    - Logstash 是一个数据收集引擎，用于将不同来源的数据收集、处理和转换，然后发送到目标系统（如 Elasticsearch）。它支持多种数据源的输入（如日志文件、消息队列、数据库等）和多种目标的输出。

"ELK" 组合通常用于搭建日志管理和分析平台，可以帮助用户实现实时的数据收集、存储、搜索和可视化，从而更好地理解数据、发现趋势和问题，以及支持决策制定。最近，"ELK" 也逐渐被 "Elastic Stack"（Elastic Stack 包括 Elasticsearch、Logstash、Kibana 以及 Beats 等组件）所取代，以反映 Elastic 公司对其产品和生态系统的整体发展。
