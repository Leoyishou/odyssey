---
date created: 2024-08-08
date modified: 2025-07-10
uid: bea04784-723e-4913-be36-31f1d06c39dc
---
## Grafana Cheatsheet

### 常用术语

- **Dashboard**：仪表盘，包含多个Panel
- **Panel**：单个图表或指标显示单元
- **Datasource**：数据源，如Prometheus, MySQL, InfluxDB等
- **Alert**：告警，监控指标触发时发送通知
- **Query**：查询语句，获取数据源数据

### 基本操作

| 操作 | 方法 |
| --- | --- |
| 新建Dashboard | `Dashboards -> New Dashboard` |
| 添加Panel | `Add Panel` |
| 配置Panel | 选择Panel -> `Edit` |
| 保存Dashboard | 点击顶部 `Save` |
| 导出Dashboard | `Share` -> `Export` |

### 📌 Functions 分类及含义

#### ① Sorting（排序类函数）

用于对数据系列进行排序，例如：

- **sortByMaxima**：按数据点的最大值排序。
- **sortByMinima**：按数据点的最小值排序。
- **sortByName**：按指标名称排序。

---

#### ② Filter Data（数据过滤函数）

用于过滤数据，例如：

- **removeAboveValue**：移除大于特定值的数据。
- **removeBelowValue**：移除小于特定值的数据。
- **limit**：限制返回的数据系列数量。

---

#### ③ Alias（别名函数，截图所示）

用于对指标设置易读的别名：

- **alias**：直接给指标起一个固定的别名。
- **aliasByMetric**：以指标（Metric）的名称作为别名。
- **aliasByNode**：根据指标名称节点位置作为别名（节点用点号 `.` 分割）。
- **aliasByTags**：根据指标附带的标签作为别名（适用于使用标签的指标）。
- **aliasSub**：根据正则表达式替换指标名称，灵活设置别名。

### 常用Panel类型

- **Time Series**: 时间序列图
- **Bar chart**: 柱状图
- **Pie chart**: 饼图
- **Stat**: 单值指标
- **Gauge**: 仪表盘
- **Table**: 表格
- **Heatmap**: 热力图

### 查询语言

- **PromQL**（Prometheus）示例:

  ```Java
  rate(http_requests_total[5m])
  ```

- **InfluxQL**（InfluxDB）示例:

  ```sql
  SELECT mean("value") FROM "cpu_usage" WHERE time > now() - 1h GROUP BY time(10m)
  ```

### 单位设置

| 分类            | 含义   | 常见单位           |
| ------------- | ---- | -------------- |
| Misc          | 杂项   | count, percent |
| Acceleration  | 加速度  | m/s²           |
| Angle         | 角度   | degree, radian |
| Area          | 面积   | m², km²        |
| Computation   | 计算性能 | FLOPS          |
| Concentration | 浓度   | mg/m³          |

### Overrides使用

- **用途**: 对特定字段应用特殊格式或设置。
- **示例**:
  - 指定特定字段颜色
  - 调整特定字段单位

### 常用快捷键

| 快捷键 | 功能 |
|---|---|
| `E` | 编辑选中的Panel |
| `Esc` | 退出编辑模式 |
| `Ctrl+S` | 保存Dashboard |
| `F` | 面板全屏显示 |

### 告警（Alert）配置

- **设置步骤**：选择Panel -> `Alert`标签页 -> 配置触发条件
- **通知渠道**：Email, Slack, Webhook等

### 最佳实践

- Panel布局尽量简单清晰
- 使用合理的单位与标题
- 关键指标设置告警及时反馈
