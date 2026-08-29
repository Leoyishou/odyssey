---
uid: 8e295604-4d16-4fd7-b9ec-ab4f5dd00325
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡Summary

> [!Info]
> 传统大数据：数据太大，一台机器放不下 → 分布式存储 → 分布式计算 → 分布式查询
> DuckDB 思路：现代单机性能够强（M1/M2 芯片 + NVMe SSD），GB~TB 级数据单机就能处理，不需要那套复杂架构。

![CleanShot 2026-02-02 at 20.39.07@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_cnOV8ZZg6T%2F2026%2F02%2F02%2F20-39-13-f1a07aa0a836e287e414d55eff999b90-CleanShot%202026-02-02%20at%2020.39.07-2x-ed84b2.png)

![CleanShot 2026-02-02 at 20.38.23@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_GlXvEgHDGI%2F2026%2F02%2F02%2F20-38-43-5b7f3ddc24621d100cc186a2fb2aa8cd-CleanShot%202026-02-02%20at%2020.38.23-2x-e88cf2.png)

# 🧩 Cues


# 🪞Notes

⏺ 对，单机所以那些分布式组件都省了。DuckDB 快的原因：

## 一、列式存储 vs 行式存储

行式存储（MySQL/PostgreSQL）：

`存储：[1,张三,25,10000] [2,李四,30,15000] [3,王五,28,12000]`

| id | name | age | salary |
|---|---|---|---|
| 1 | 张三 | 25 | 10000 |
| 2 | 李四 | 30 | 15000 |
| 3 | 王五 | 28 | 12000 |

[[列式存储]]（DuckDB）：

```
存储：
id: [1, 2, 3]
name: [张三, 李四, 王五]
age: [25, 30, 28]
salary: [10000, 15000, 12000]
```


效果：`SELECT AVG(salary) FROM employees` 只读 salary 那一列，不碰其他列。

类比 Java：
```java
// 行存：遍历整个对象
employees.stream().map(e -> e.getSalary()).average()

// 列存：直接操作数组
salaries.stream().average()  // 内存连续，CPU 缓存命中率高
```

## 二、向量化执行

传统数据库：一行一行处理（Volcano 模型）
```java
for (Row row : table) {
    process(row);  // 每行一次函数调用
}
```

DuckDB：一批一批处理（向量化）
```java
int[] batch = getBatch(1024);  // 一次取 1024 行
processAll(batch);             // 批量处理，利用 SIMD 指令
```

SIMD：CPU 单条指令同时处理多个数据（类似 GPU 并行思想）

## 三、为什么单机能快过集群

| 因素 | 分布式集群 | DuckDB 单机 |
|---|---|---|
| 网络 IO | 节点间数据 shuffle | 无 |
| 序列化 | 跨节点需要序列化 | 内存直接操作 |
| 协调开销 | 分布式事务、一致性 | 无 |
| 调度延迟 | YARN 调度 | 直接执行 |

真实场景：10GB 数据，DuckDB 本地跑 2 秒，[[Spark]] 集群可能要 30 秒（启动 + 调度 + shuffle）。

## 四、适用边界

| 数据量 | 推荐方案 |
|---|---|
| < 100GB | DuckDB 单机 |
| 100GB ~ 1TB | DuckDB 仍可用（需要 SSD） |
| > 1TB | 考虑分布式（Spark/Presto） |

你的场景：Supabase 数据大概率在 GB 级别，DuckDB 绰绰有余。