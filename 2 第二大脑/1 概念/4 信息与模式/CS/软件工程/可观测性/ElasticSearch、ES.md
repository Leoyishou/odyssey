---
date created: 2024-06-29
date modified: 2025-09-16
uid: bc090318-53b4-4f46-98e2-e0f9f41d5e3b
---
- [基础](#%E5%9F%BA%E7%A1%80)
	- [行维度](#%E8%A1%8C%E7%BB%B4%E5%BA%A6)
	- [select](#select)
- [存储与搜索](#%E5%AD%98%E5%82%A8%E4%B8%8E%E6%90%9C%E7%B4%A2)
	- [双存储模式](#%E5%8F%8C%E5%AD%98%E5%82%A8%E6%A8%A1%E5%BC%8F)
- [数据类型](#%E6%95%B0%E6%8D%AE%E7%B1%BB%E5%9E%8B)
	- [1. 搜索需求](#1.%20%E6%90%9C%E7%B4%A2%E9%9C%80%E6%B1%82)
	- [2. 存储需求](#2.%C2%A0%E5%AD%98%E5%82%A8%E9%9C%80%E6%B1%82)
	- [3. 查询需求](#3.%20%E6%9F%A5%E8%AF%A2%E9%9C%80%E6%B1%82)


# Summary

1. 与 mysql 的 sql 语言不同，es 操作都是http 接口
2. 一张表叫 index，一行数据叫一个 document，本质就是一个 Json，key 就是字段名，叫 field
3. Mapping 类似 [MySQL](MySQL.md) 的 Desc看字段类型 + CREATE INDEX建索引


# Cues

[倒排索引](倒排索引.md)

# Notes

- [行维度](#%E8%A1%8C%E7%BB%B4%E5%BA%A6)
- [存储与搜索](#%E5%AD%98%E5%82%A8%E4%B8%8E%E6%90%9C%E7%B4%A2)
- [双存储模式](#%E5%8F%8C%E5%AD%98%E5%82%A8%E6%A8%A1%E5%BC%8F)
- [select](#select)
- [布尔组合查询](#%E5%B8%83%E5%B0%94%E7%BB%84%E5%90%88%E6%9F%A5%E8%AF%A2)
- [数据类型与索引](#%E6%95%B0%E6%8D%AE%E7%B1%BB%E5%9E%8B%E4%B8%8E%E7%B4%A2%E5%BC%95)
- [1. 搜索需求](#1.%20%E6%90%9C%E7%B4%A2%E9%9C%80%E6%B1%82)
- [2. 存储需求](#2.%C2%A0%E5%AD%98%E5%82%A8%E9%9C%80%E6%B1%82)
- [3. 查询需求](#3.%20%E6%9F%A5%E8%AF%A2%E9%9C%80%E6%B1%82)


## 基础
### 行维度

一行数据就是一个 Json

insert 语句

```shell
curl -X POST "http://localhost:9200/tours/_doc/1" -H 'Content-Type: application/json' -d '{
  "destination": "北京",
  "days": 3,
  "price": 1500,
  "tags": ["文化", "历史"],
  "description": "三天两晚深度游览故宫、长城和颐和园"
}'
```

Json 的 key 就是 field



## 存储与搜索


Query 关键词 vs 倒排索引存储模式

| 层面       | 内容               | 说明        |
| -------- | ---------------- | --------- |
| Query 层面 | term、match 等查询类型 | 决定如何搜索    |
| 存储层面     | 倒排索引的存储方式        | 决定数据如何被索引 |

### 双存储模式
原始文档模式 + [倒排索引](倒排索引.md)模式
```Java
// 原始文档（_source）
{
  "destination": "北京市"  // 完整值，用于显示
}

// 倒排索引（实际搜索用的）
"北京" → [文档1, 文档2, ...]  // 分词后的索引
"市" → [文档1, 文档3, ...]    // 分词后的索引```
## 表维度

show tables
```shell
curl -X GET "http://localhost:9200/_cat/indices?v"
```

建表语句 create table
```shell
curl -X PUT "http://localhost:9200/tours"
```

### select
[[match查询]]
match 是支持分词的，用「北京市」可以搜出「北京」的结果
```http
curl -X GET "http://localhost:9200/tours/_search" -H 'Content-Type: application/json' -d '{
  "query": {
    "match": { "destination": "北京市" }
  }
}'
{"took":3,"timed_out":false,"_shards":{"total":1,"successful":1,"skipped":0,"failed":0},"hits":{"total":{"value":2,"relation":"eq"},"max_score":0.9777894,"hits":[{"_index":"tours","_id":"1","_score":0.9777894,"_source":{
  "destination": "北京市",
  "days": 3,
  "price": 1500,
  "tags": ["文化", "历史"],
  "description": "三天两晚深度游览故宫、长城和颐和园"
}},{"_index":"tours","_id":"2","_score":0.39713606,"_source":{
  "destination": "北京",
  "days": 3,
  "price": 1500,
  "tags": ["文化", "历史"],
  "description": "三天两晚深度游览故宫、长城和颐和园"
}}]}}% 
```


### 布尔组合查询

布尔查询允许组合多个条件：
- `must`（AND）
- `should`（OR）
- `must_not`（NOT）

```shell
curl -X GET "http://localhost:9200/tours/_search" -H 'Content-Type: application/json' -d '{
  "query": {
    "bool": {
      "must": [
        { "match": { "destination": "北京" } },
        { "range": { "price": { "lt": 2000 } } }
      ]
    }
  }
}'

```
## 数据类型与索引
Mapping 类似 [MySQL](MySQL.md) 的 Desc看字段类型 + CREATE INDEX建索引

```json
{
  "mappings": {
    "properties": {
      "id": {"type": "long"},
      
      "title": {
        "type": "text",
        "analyzer": "ik_max_word",
        "fields": {
          "keyword": {"type": "keyword"}
        }
      },
      
      "city": {"type": "keyword"},
      "province": {"type": "keyword"},
      "country": {"type": "keyword"},
      "days": {"type": "integer"},
      "price": {"type": "float"},
      "tags": {"type": "keyword"},
      
      "description": {"type": "text"},
      "location": {"type": "geo_point"},
      
      "created_at": {
        "type": "date",
        "format": "yyyy-MM-dd HH:mm:ss"
      },
      
      "route_details": {
        "type": "nested",
        "properties": {
          "day": {"type": "integer"},
          "poi_list": {"type": "keyword"}
        }
      }
    }
  }
}
```

核心类型：text、keyword、numeric、date
复杂类型：object、nested、array
特殊类型：geo_point、ip、boolean

| 类别         | 类型          | 描述与特点                                                                         | 示例                                                                                                                                                                                       |
|:--------- |:---------- |:---------------------------------------------------------------------------- |:--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **核心数据类型** | `Text`      | **用途**：用于长文本，如文章内容、产品描述。<br>**特点**：会被分词器分词，用于全文搜索和模糊匹配。| ```json<br>{<br>  "description": {<br>    "type": "text",<br>    "analyzer": "standard",<br>    "search_analyzer": "ik_smart",<br>    "index": true<br>  }<br>}<br>```                   |
|            | `Keyword`   | **用途**：用于短文本，如城市名、状态、标签。<br>**特点**：不分词，需要进行精确匹配、排序或聚合。| ```json<br>{<br>  "city": {<br>    "type": "keyword",<br>    "ignore_above": 256<br>  }<br>}<br>```                                                                                      |
|            | `Numeric`   | **用途**：用于存储数字，如年龄、价格、数量。<br>**类型**：`integer`, `long`, `float`, `double` 等。| ```json<br>{<br>  "age": {"type": "integer"},<br>  "price": {"type": "float"},<br>  "score": {"type": "double"},<br>  "count": {"type": "long"}<br>}<br>```                              |
|            | `Date`      | **用途**：用于存储日期和时间。<br>**特点**：可以指定多种格式，支持日期范围查询。| ```json<br>{<br>  "created_at": {<br>    "type": "date",<br>    "format": "yyyy-MM-dd HH:mm:ss"<br>  }<br>}<br>```                                                                       |
| **复杂数据类型** | `Object`    | **用途**：用于存储层级结构的 JSON 对象。<br>**特点**：默认类型，用于内部对象的映射。| ```json<br>{<br>  "user": {<br>    "type": "object",<br>    "properties": {<br>      "name": {"type": "text"},<br>      "age": {"type": "integer"}<br>    }<br>  }<br>}<br>```           |
|            | `Nested`    | **用途**：用于存储对象数组，并保持数组内对象字段之间的关联性。<br>**特点**：将对象数组中的每个对象作为独立的文档进行索引。| ```json<br>{<br>  "comments": {<br>    "type": "nested",<br>    "properties": {<br>      "author": {"type": "keyword"},<br>      "content": {"type": "text"}<br>    }<br>  }<br>}<br>``` |
|            | `Array`     | **用途**：用于存储值的列表。<br>**特点**：Elasticsearch 没有专门的 array 类型，默认情况下任何字段都可以包含零个或多个值。| ```json<br>{<br>  "tags": {"type": "keyword"},<br>  "prices": {"type": "float"}<br>}<br>```                                                                                              |
| **地理位置类型** | `Geo Point` | **用途**：用于存储经纬度坐标点。<br>**特点**：支持地理距离计算、范围查询等。| ```json<br>{<br>  "location": {<br>    "type": "geo_point"<br>  }<br>}<br>```                                                                                                            |
|            | `Geo Shape` | **用途**：用于存储复杂的地理形状，如多边形、圆形等。<br>**特点**：支持地理形状之间的相交、包含等查询。| ```json<br>{<br>  "area": {<br>    "type": "geo_shape"<br>  }<br>}<br>```                                                                                                                |
| **特殊类型**   | `IP`        | **用途**：用于存储 IPv4 或 IPv6 地址。<br>**特点**：支持 IP 地址的范围查询。| ```json<br>{<br>  "ip_address": {"type": "ip"}<br>}<br>```                                                                                                                               |
|            | `Boolean`   | **用途**：用于存储 `true` 或 `false` 值。<br>**特点**：用于精确匹配的过滤查询。| ```json<br>{<br>  "is_active": {"type": "boolean"}<br>}<br>```                                                                                                                           |

### 1. 搜索需求
- 需要全文搜索 → text
- 需要精确匹配 → keyword
- 需要范围查询 → numeric 或 date

### 2. 存储需求
- 节省空间 → 选择合适的数值类型
- 性能优化 → 合理使用 keyword vs text

### 3. 查询需求
- 聚合统计 → keyword 或 numeric
- 地理位置 → geo_point 或 geo_shape


# Elasticsearch 知识结构图

| 类别            | 核心概念           | 详细内容                                                  |     |
| ------------- | -------------- | ----------------------------------------------------- | --- |
| **基础概念**      | 文档（Document）| JSON格式的数据单元，类似关系型数据库的行                                | ✅   |
|               | 索引（Index）| 文档的集合，类似关系型数据库的表                                      | ✅   |
|               | 类型（Type）| 7.x版本后已废弃，之前用于索引内的逻辑分类                                |     |
|               | 字段（Field）| 文档中的属性，类似数据库的列                                        | ✅   |
|               | 映射（Mapping）| 定义字段类型和索引方式的元数据                                       | ✅   |
| **集群架构**      | 节点（Node）| 集群中的单个服务器实例                                           |     |
|               | 集群（Cluster）| 一组协同工作的节点集合                                           |     |
|               | 分片（Shard）| 索引的物理存储单元，分为主分片和副本分片                                  |     |
|               | 副本（Replica）| 主分片的复制，提供高可用和负载均衡                                     |     |
|               | 主节点（Master）| 管理集群状态和元数据的节点                                         |     |
| **数据操作**      | 索引操作           | PUT/POST 创建或更新文档                                      |     |
|               | 查询操作           | GET 检索文档                                              | ✅   |
|               | 更新操作           | POST _update 部分更新文档                                   |     |
|               | 删除操作           | DELETE 删除文档或索引                                        |     |
|               | 批量操作           | _bulk API 批量执行多个操作                                    |     |
| **[[查询DSL]]** | 全文查询           | [match查询](match查询.md)：match、match_phrase、multi_match等 |     |
|               | 精确查询           | [[fuzzy]]、term、terms、range、exists等                    |     |
|               | [[复合查询]]       | bool（must、[[should]]、must_not、filter）|     |
|               | 聚合查询           | metrics、bucket、pipeline聚合                             |     |
|               | 高亮显示           | highlight 突出显示匹配内容                                    |     |
| **分析器**       | 字符过滤器          | 预处理原始文本                                               |     |
|               | 分词器（Tokenizer）| 将文本分解为词项                                              |     |
|               | 词项过滤器          | 处理分词后的词项（如转小写、同义词）|     |
|               | 内置分析器          | standard、simple、whitespace、keyword等                   |     |
| **数据类型**      | 文本类型           | text（全文搜索）、keyword（精确匹配）|     |
|               | 数值类型           | long、integer、short、byte、double、float                  |     |
|               | 日期类型           | date、date_nanos                                       |     |
|               | 布尔类型           | boolean                                               |     |
|               | 复杂类型           | object、nested、geo_point、geo_shape                     |     |
| **性能优化**      | 索引优化           | 合理设置分片数、使用合适的映射                                       |     |
|               | 查询优化           | 使用filter代替query、避免深度分页                                |     |
|               | 缓存机制           | 节点查询缓存、分片请求缓存、字段数据缓存                                  |     |
|               | 监控调优           | _cat API、_stats API、慢查询日志                             |     |
| **高级特性**      | 索引模板           | 预定义索引设置和映射                                            |     |
|               | 索引生命周期         | ILM自动管理索引的生命周期                                        |     |
|               | 快照备份           | snapshot/restore API备份恢复数据                            |     |
|               | 安全认证           | X-Pack Security提供认证授权功能                               |     |
|               | 跨集群搜索          | CCS跨多个集群执行搜索                                          |     |
| **常用插件**      | 分词插件           | IK分词器（中文）、jieba（中文）|     |
|               | 监控插件           | Kibana、Grafana集成                                      |     |
|               | 同步插件           | Logstash、Beats数据采集                                    |     |
|               | 管理插件           | Cerebro、ElasticHQ集群管理                                 |     |

这个表格涵盖了 Elasticsearch 的主要知识点，从基础概念到高级特性都有涉及。您可以根据学习进度逐步深入每个类别的具体内容。
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F29%2F17-20-19-83d3934bbb7c374bda5b24e45987e38b-20240629172018-9a370a.png)



基础与概念（1–12）

1. Elasticsearch 的核心概念有哪些（Index、Document、Shard、Replica、Mapping、Segment）？分别起什么作用？
2. 倒排索引的基本结构与工作原理是什么？
3. Lucene 与 Elasticsearch 的关系是什么？
4. ES 中的近实时（NRT）是什么意思？refresh 与 flush 有何区别？
5. 索引（index）与数据流（data stream）的区别与适用场景？
6. 文档的 _id 是如何确定的？自行提供与自动生成的优缺点？
7. ES 的一致性模型（primary-terminology、sequence number、global checkpoint）如何保障？
8. 什么是 segment？为什么需要 segment merge？
9. routing 是什么？默认 routing 的规则？自定义 routing 的利弊？
10. 什么是 fielddata？为什么 text 字段默认不启用 fielddata？
11. ES 的硬删与软删（soft deletes）有什么不同？
12. explain API 的用途是什么？能从中看出哪些打分细节？


映射与分析器（13–24）

1. 动态映射（dynamic mapping）与显式映射（explicit mapping）如何选择？
2. keyword 与 text 的差异与使用场景？
3. analyzer、tokenizer、filter 各自的作用与常见配置？
4. 如何为同一个字段设计 multi-fields（如 text + keyword）？
5. numeric、date、scaled_float、dense_vector 等类型的选型建议？
6. nested 类型与 object 类型的区别，nested 查询如何写？
7. geo_point 与 geo_shape 的差异与典型使用？
8. runtime fields 的原理、使用场景与性能注意事项？
9. 字段映射变更的限制有哪些？如何演进 schema？
10. 自定义同义词（synonym）如何配置？热更新有哪些办法与坑？
11. normalizer 与 analyzer 的差异是什么？
12. 禁用 _source 的影响是什么？哪些场景下可以考虑禁用或裁剪？


查询 DSL（25–38）

1. term、terms、match、match_phrase 的差异是什么？
2. bool 查询中 must、should、must_not、filter 的语义和性能差异？
3. minimum_should_match 的用途与常见写法？
4. multi_match 的几种 type（best_fields、most_fields、cross_fields）如何选？
5. range 查询在时间序列数据上的最佳实践？
6. prefix、wildcard、regexp 的成本差异与替代方案？
7. script 查询的应用场景与风险？
8. function_score 的常见用法与打分可控性？
9. highlight 的工作原理与常见优化？
10. search_after 与 scroll 的区别及各自适用场景？
11. PIT（Point In Time）是什么？与 search_after 搭配的优势？
12. collapsing/field collapsing（近似去重）如何实现与限制？
13. percolate（反向查询）适用哪些业务？
14. KNN/向量检索（dense_vector、kNN search）的基本原理与限制？


聚合（39–48）

1. bucket 与 metric 聚合的区别是什么？
2. terms 聚合的 size、shard_size、order 如何影响结果与性能？
3. cardinality 估算原理（HLL++）与误差控制？
4. date_histogram 与 auto_date_histogram 有何不同？
5. pipeline 聚合（moving_avg、derivative、bucket_script）的使用场景？
6. composite 聚合的分页方案与适用场景？
7. percentiles 与 percentile_ranks 的区别与落地案例？
8. top_hits 聚合作为“去重取样”的局限与替代？
9. nested/children 聚合如何写？性能注意事项？
10. 聚合与查询并行执行、circuit breaker 与内存控制机制？


索引管理与生命周期（49–58）

1. index template（v2 组件化模板）如何组织大型集群的映射与设置？
2. shard 个数如何估算？过多或过少分别会带来什么问题？
3. replica 设置对读/写的影响？
4. force merge 的作用与风险？何时可以做？
5. rollover + ILM（Index Lifecycle Management）如何设计热温冷存储？
6. 冷/冷冻/只读存储层的权衡与查询策略？
7. refresh_interval、translog、flush 策略在写入延迟与可靠性中的权衡？
8. index sorting（预排序）有什么用？适合哪些查询模式？
9. frozen/shadow replica（历史概念）与当前架构的差异？
10. 数据再索引（reindex）策略：在线迁移、最小化停机与回滚？


集群架构与运维（59–70）

1. master-eligible 节点职责是什么？为什么要奇数个投票节点？
2. 集群发现与选主（Zen2、cluster formation）如何工作？
3. shard allocation awareness / filtering 的用途？
4. 断路器（circuit breaker）与熔断策略有哪些？
5. 查询与写入在节点角色（coordinating、data、ingest、ml、transform）上的分工？
6. snapshot/restore 的后端（S3、HDFS、NFS）怎么选？一致性与带宽规划？
7. 集群升级路径（滚动升级 vs 停机升级）与版本兼容性注意点？
8. node 与 shard 的资源隔离：JVM 堆、直接内存、FS cache 的平衡？
9. thread_pool（search/write/management）与 queue 配置如何调优？
10. translog、异步复制与写入确认（durability）策略？
11. 集群红黄绿三色分别意味着什么？常见自愈手段？
12. CCR（跨集群复制）与 CCS（跨集群搜索）的区别与场景？


性能优化与容量规划（71–82）

1. 如何诊断“查询慢”？使用哪些内建工具（profile API、hot threads、_nodes/stats）？
2. filter cache、query cache 的命中机制与失效策略？
3. 段合并策略（tiered/soft-deletes）对查询/写入的影响？
4. 大字段、多字段（high cardinality）的代价与优化手段？
5. Top K 优化（early termination、terminate_after）怎么用？
6. doc_values 的原理与在排序/聚合中的作用？
7. 稀疏向量/稠密向量检索的索引结构（HNSW）与内存考量？
8. 批量写入（_bulk）的最佳实践：batch size、并发度、重试策略？
9. refresh 策略如何在写多读少与读多写少场景下分别调优？
10. 大规模 terms 查询/聚合的替代方案（预计算、Lakehouse、外部 KV）？
11. segment store 与文件系统 cache 的关系及 I/O 调优思路？
12. 典型容量规划：每分区 QPS、写入速率、字段数量、存储与堆的配比？


安全、监控与可观测性（83–90）

1. X-Pack 安全（TLS、用户/角色/索引级权限）如何最小权限化？
2. Field- & Document-level security 的适用边界与性能影响？
3. 审计日志（audit logging）记录了什么？如何排障取证？
4. 节点间加密与 HTTP 层加密分别如何配置？
5. Watcher / Alerting 的实现原理与替代（外部告警系统对接）？
6. _cluster/health、_cat 接口、Stack Monitoring 的常用指标有哪些？
7. 搜索慢日志、索引慢日志如何配置与解读？
8. ILM/SLM 失败时的常见告警与自动化修复思路？


实战与排错（91–96）

1. “磁盘水位”导致分片迁移/分配失败如何排查与处置？
2. 写入 429/503、队列打满时的缓解方案？
3. 查询 OOM 或 circuit breaker 触发时如何快速止血与长期治理？
4. 映射爆炸（mapping explosion）是如何产生的？如何防范？
5. 同义词更新后搜索结果异常的常见原因与回滚策略？
6. 集群从 yellow 长期恢复不到 green 的诊断步骤？


版本与生态（97–100）

1. 近几个大版本（7→8→当前）在安全默认、查询、聚合或存储层的关键变化有哪些？
2. data stream + ILM 在日志/指标等时间序列里的标准化用法？
3. transform、ingest pipeline（含 enrichment、geoip）常见编排模式？
4. 与 OpenSearch、Solr 的关键差异、迁移注意事项与取舍？
