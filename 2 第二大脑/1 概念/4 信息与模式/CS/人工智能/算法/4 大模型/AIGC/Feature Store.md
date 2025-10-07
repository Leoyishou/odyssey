---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-05
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 🧩 Summary  



# 💡 Cues

[[Feast]]

# Notes


### 场景

预测用户是否会购买商品，需要特征：
- `user_7d_purchase_count`（用户过去7天购买次数）
- `user_avg_price`（用户历史平均消费金额）
- `user_last_login_hours`（距离上次登录的小时数）

离线训练（数据科学家用 Spark）
线上推理（工程师用 Python 重写）

## 问题

1. **逻辑不一致**：训练/推理代码分离，工程师翻译出错
2. **维护困难**：特征改了，两处都要改
3. **调试困难**：线上效果差，不知道是模型问题还是特征问题

## 解决

Feature Store 的解决方案

### 1. 统一定义特征（只写一次）


```python
# features.py（唯一真相源）
from feast import FeatureView, Entity, Feature, FileSource
from feast.value_type import ValueType
from datetime import timedelta

# 定义实体
user = Entity(name="user_id", value_type=ValueType.INT64)

# 定义特征视图
user_stats = FeatureView(
    name="user_purchase_stats",
    entities=["user_id"],
    ttl=timedelta(days=30),  # 特征新鲜度
    features=[
        Feature("user_7d_purchase_count", ValueType.INT64),
        Feature("user_avg_price", ValueType.FLOAT),
        Feature("user_last_login_hours", ValueType.FLOAT),
    ],
    batch_source=FileSource(
        path="s3://features/user_stats.parquet",
        event_timestamp_column="event_time",  # 关键！时间列
    ),
    online=True  # 同步到在线 Redis
)
```

### 2. 离线特征计算（Spark/Airflow 定时跑）

```python
# compute_features.py（每小时运行）
from pyspark.sql import functions as F

def compute_user_features(spark):
    orders = spark.read.parquet("s3://raw/orders")
    
    # 计算特征（与定义一致）
    features = (
        orders
        .withColumn("event_time", F.current_timestamp())  # 时间戳
        .groupBy("user_id")
        .agg(
            # 7天购买次数
            F.count(F.when(
                F.col("order_time") > F.current_date() - 7, 1
            )).alias("user_7d_purchase_count"),
            
            # 平均消费（不 ROUND，保持精度）
            F.avg("price").alias("user_avg_price"),
            
            # 距上次登录小时数
            F.round(
                (F.unix_timestamp(F.current_timestamp()) - 
                 F.unix_timestamp(F.max("login_time"))) / 3600,
                2
            ).alias("user_last_login_hours")
        )
    )
    
    # 写入特征存储
    features.write.parquet("s3://features/user_stats.parquet")
    
compute_user_features(spark)

# 同步到在线存储
!feast materialize-incremental $(date +%Y-%m-%d)
```

### 3. 离线训练（自动 Point-in-Time Join）

```python
# train.py
from feast import FeatureStore
import pandas as pd

store = FeatureStore(repo_path=".")

# 训练标签（带时间戳！）
labels = pd.DataFrame({
    "user_id": [123, 456, 789],
    "event_time": [
        "2024-01-01 10:00:00",
        "2024-01-01 11:00:00", 
        "2024-01-01 12:00:00"
    ],
    "label": [1, 0, 1]
})

# ✅ Feast 自动做 Point-in-Time Join
training_df = store.get_historical_features(
    entity_df=labels,
    features=[
        "user_purchase_stats:user_7d_purchase_count",
        "user_purchase_stats:user_avg_price",
        "user_purchase_stats:user_last_login_hours",
    ]
).to_df()

# 结果：每行取的是 <= event_time 的最新特征值
print(training_df)
"""
   user_id  event_time           label  user_7d_purchase_count  user_avg_price
0  123      2024-01-01 10:00:00  1      3                      45.6
1  456      2024-01-01 11:00:00  0      0                      0.0
2  789      2024-01-01 12:00:00  1      7                      123.4
                                         ↑                       ↑
                        取的是 2024-01-01 10:00 之前的值，避免标签泄露！
"""

model.fit(training_df)
```

### 4. 在线推理（自动从 Redis 读）

python

```python
# predict_api.py
from feast import FeatureStore

store = FeatureStore(repo_path=".")

def predict(user_id: int):
    # ✅ 直接从 Redis 读取，<10ms
    features = store.get_online_features(
        features=[
            "user_purchase_stats:user_7d_purchase_count",
            "user_purchase_stats:user_avg_price",
            "user_purchase_stats:user_last_login_hours",
        ],
        entity_rows=[{"user_id": user_id}]
    ).to_dict()
    
    # 特征自动和训练时一致！
    return model.predict(features)

# 线上 AUC = 0.85，和离线一致！🎉
```


# 💡 Cues



# Notes

## Feature Store（如 Feast）的理念解释

### 核心概念

**Feature Store** 是机器学习特征的"中央仓库"，解决特征工程中的**一致性、复用、管理**问题。

**Feast** 是开源的 Feature Store 实现，体现了以下核心理念：

---

### 1. **离线/在线一致性**（最重要）
```
问题：训练用 Spark 算特征，线上用 Python 重写 → 逻辑不一致 → 模型效果下降

Feast 方案：
  ┌─────────────────┐
  │  特征定义（统一） │  ← 用户年龄、7日消费额等
  └────┬───────┬────┘
       │       │
   离线存储   在线存储
   (Parquet)  (Redis)
       │       │
    训练时    推理时
   读同一份   读同一份
   特征定义   特征定义
```

---

### 2. **特征复用与共享**
```python
# 团队 A 定义了特征
user_age = Feature(name="user_age", dtype=Int64)

# 团队 B 直接复用，不用重复开发
features = store.get_online_features(
    features=["user_age", "user_7d_spend"]
)
```

---

### 3. **时间点正确性**（Point-in-Time Join）
```
避免"标签泄露"：
  训练样本时间 = 2023-01-01
  ❌ 错误：取到 2023-01-05 的特征
  ✅ Feast：自动取 ≤ 2023-01-01 的最新特征值
```

---

### 4. **低延迟在线服务**
```
Redis/DynamoDB 存储热特征
API 调用 < 10ms
支持批量特征获取
```

---

### 5. **特征血缘与治理**
- 版本控制：特征定义变更可追溯
- 监控：特征分布漂移告警
- 文档：自动生成特征目录

---

### Feast 典型工作流

```python
# 1. 定义特征
user_features = FeatureView(
    name="user_stats",
    entities=["user_id"],
    features=[
        Feature("age", Int64),
        Feature("7d_purchase", Float32)
    ],
    batch_source=ParquetSource(path="s3://..."),
    online=True  # 同步到在线存储
)

# 2. 离线训练
training_df = store.get_historical_features(
    entity_df=labels_df,  # 带时间戳
    features=["user_stats:age", "user_stats:7d_purchase"]
).to_df()

# 3. 在线推理
features = store.get_online_features(
    features=["user_stats:age"],
    entity_rows=[{"user_id": 123}]
).to_dict()
```

---

### 总结对比

| 维度 | 传统方式 | Feature Store 理念 |
|------|---------|-------------------|
| **一致性** | 离线/在线代码分离，易不一致 | 统一定义，自动同步 |
| **复用** | 各团队重复开发 | 中央仓库共享 |
| **时间正确性** | 手动处理，易泄露 | Point-in-Time Join |
| **延迟** | 现算现取，慢 | 预计算 + 缓存 |
| **治理** | 靠文档 | 自动化血缘/监控 |

这就是 Feast 等 Feature Store 要解决的核心问题！🎯