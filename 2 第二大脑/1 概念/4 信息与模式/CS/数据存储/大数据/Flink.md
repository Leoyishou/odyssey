---
date created: 2025-02-24
date modified: 2025-10-07
uid: 7c85238c-94d9-43a3-9f59-437dffe75dcd
---

Flink：实时流的专家

Flink 跟 Spark 有点像，也是个计算引擎，但它更擅长处理"实时数据"。比如，你网站上的用户点击数据源源不断地进来，Flink 能一边接收一边处理，延迟很低。Spark 也能做流处理，但它更偏向"微批次"（攒一小批数据再处理），而 Flink 是真正的"流水线式"处理。

Flink 也可以跟 Hadoop 合作，用 HDFS 存数据，但它更专注于实时场景，比如监控系统、实时推荐这种。

# Notes


## 核心概念

| 编号  | 练习名称             | 功能描述                                      | Flink 核心概念                                   |
| --- | ---------------- | ----------------------------------------- | -------------------------------------------- |
| 01  | 环境 & Hello Table | 创建 TableEnvironment，从内存数据创建表，简单 select 查询 | `TableEnvironment`、`from_elements`、基本列选择和表达式 |
| 02  | Select & Where   | 使用条件过滤数据（`cat == 'A' AND val > 10`）| 投影（Projection）、过滤（Filter）、布尔表达式              |
| 03  | 计算列              | 添加派生列（`amount = price * qty`）| 表达式计算、列别名、数学运算符                              |
| 04  | GroupBy & 聚合     | 按类别分组，计算 sum 和 count                      | 分组聚合、聚合函数（`sum`, `count`）、流批统一基础             |
| 05  | 排序与 Limit        | 按分数降序排序，取 Top-3                           | 排序（Order By）、`fetch(n)` 限制行数、全局 Top-N        |
| 06  | Join             | 用户表 join 订单表，计算每用户订单总额                    | 内连接（Inner Join）、多表关联、Join + 聚合组合             |
| 07  | 标量 UDF           | 定义 Python 文本清洗函数，注册为 UDF 并调用              | 用户自定义函数（UDF）、标量函数、扩展处理能力                     |
| 08  | 窗口（桶近似）| 按时间戳分桶（60秒窗口），统计每桶事件数                     | 时间窗口、时间维度聚合、流处理窗口基础                          |
| 09  | 分组 Top-N         | 每个类别内取分数最高的 2 条记录                         | 窗口函数（`ROW_NUMBER()`）、`PARTITION BY`、实时排行榜    |
| 10  | WordCount 项目     | 读取文本文件，分词，统计词频，输出 Top-10                  | 综合应用：数据读取 + UDF + 聚合 + 排序，大数据 Hello World    |

### 学习路径

- **阶段1（01-03）**：基础查询 - SELECT、WHERE、表达式
- **阶段2（04-05）**：聚合操作 - 分组、聚合函数、排序
- **阶段3（06-07）**：复杂操作 - 多表 Join、自定义函数
- **阶段4（08-09）**：时间/窗口 - 窗口概念、分组排名
- **阶段5（10）**：综合实战 - 完整数据处理流程




## 目录结构

```Java
learnFlink/
  README.md                # 本文件
  Makefile                 # 一键命令
  requirements.txt         # Python 依赖
  docker-compose.yml       # 可选：Flink 集群
  scripts/
    setup.sh               # venv & 依赖安装
    run_exercise.sh        # 运行指定练习
  exercises/
    common.py              # 公用工具：创建 t_env、collect
    01_setup_hello.py      # 练习01：环境 & Hello Table
    02_select_where.py     # 练习02：选择与过滤
    03_computed_columns.py # 练习03：派生列与表达式
    04_groupby_agg.py      # 练习04：聚合
    05_sort_limit.py       # 练习05：排序与限制
    06_join.py             # 练习06：表连接
    07_scalar_udf.py       # 练习07：标量 UDF
    08_window_bucket.py    # 练习08：桶代替窗口（离线近似）
    09_topn_per_group.py   # 练习09：分组 Top-N（SQL）
    10_wordcount.py        # 练习10：迷你项目 WordCount
  solutions/               # 参考答案（同名文件）
  data/
    sample_text.txt        # 示例文本数据
```

---

## 练习指南（节选示意）

每个练习均可直接运行：
- 查看任务与 TODO
- 完成后运行文件，断言通过即说明完成

示例：
```Java
make run ex=01
```
输出包含表结果与“测试通过”。

---

## 常见问题

- Q: 没网络/装不了 pyflink？
  - A: 可在有网环境下打包依赖带到当前机器，或使用 Docker 方式。
- Q: collect() 一直不返回？
  - A: 本教程均使用有限数据（批模式），不会卡住；若自改为流式 source，请避免无界流使用 collect。
- Q: Windows 功能在哪里？
  - A: 为确保可离线验证，提供“桶”分组近似窗口的方法；若使用 Docker + SQL Client，可尝试真正的 `TUMBLE`/`HOP` 窗口。

祝学习顺利！
