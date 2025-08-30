---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-08-10
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary
https://github.com/AllenDowney/ThinkStats

# Cues

# Notes
根据章节标题和内容，这本《Think Stats》主要涵盖以下统计知识点：

* 基础概念与数据探索
    * 探索性数据分析 (EDA)
    * 数据清洗与转换
    * 汇总统计量（均值、中位数、方差等）
    * 数据可视化

* 分布理论
    * 频率表与频率分布
    * 概率质量函数 (PMF)
    * 累积分布函数 (CDF)
    * 概率密度函数 (PDF)
    * 百分位数与百分位排名
    * 分布建模（正态分布、指数分布等）

* 关系分析
    * 散点图与可视化
    * 相关性分析（皮尔逊相关、秩相关）
    * 因果关系与相关性

* 统计推断
    * 参数估计
    * 抽样分布
    * 标准误差
    * 置信区间
    * 假设检验（$p$值、显著性检验）

* 回归分析
    * 最小二乘法
    * 线性回归
    * 多元回归
    * 模型诊断与验证

* 高级主题
    * 时间序列分析
    * 生存分析
    * 分析方法（正态概率图、中心极限定理等）

* 计算统计方法
    * 重抽样技术
    * 模拟方法
    * Bootstrap方法

```Java
⏺ 基于查看的代码，统计推断部分的实现和调用链路如下：

  核心代码实现位置

  1. 参数估计 (nb/chap08.ipynb)

  - 均值估计: np.mean(sample) - 第8章第64行
  - 中位数估计: np.median(sample) - 第8章第64行
  - 方差估计:
    - 有偏估计: biased_var() - 第8章第131行
    - 无偏估计: unbiased_var() - 第8章第144行

  2. 抽样分布 (nb/chap08.ipynb)

  - 参数重抽样: resample() - 第8章第187行
  def resample(sample):
      m, s = np.mean(sample), np.std(sample)
      return np.random.normal(m, s, len(sample))
  - 生成抽样分布: 第8章第194-195行
  sample_means = [np.mean(resample(weights)) for i in range(1001)]

  3. 标准误差 (nb/chap08.ipynb)

  - 计算标准误差: 第8章第209行
  standard_error = np.std(sample_means)
  - 近似标准误差: 第8章第223行
  def approximate_standard_error(sample):
      n = len(sample)
      return np.std(sample) / np.sqrt(n)

  4. 置信区间 (nb/chap08.ipynb)

  - 百分位数法: 第8章第235行
  ci90 = np.percentile(sample_means, [5, 95])
  - 辅助函数: thinkstats.py第1314行
  def confidence_interval(cdf, percent=90):
      alpha = 1 - percent / 100
      return cdf.inverse([alpha/2, 1-alpha/2])

  5. 假设检验 (nb/chap09.ipynb)

  - 计算p值: 第9章第579行
  def compute_p_value(simulated, observed):
      return (np.asarray(simulated) >= observed).mean()
  - 置换检验: 第9章第213行
  def simulate_groups(data):
      pool = np.hstack(data)
      np.random.shuffle(pool)
      return pool[:n], pool[-m:]
  - 相关性检验: 第9章第686行
  def permute(data):
      xs, ys = data
      new_xs = xs.values.copy()
      np.random.shuffle(new_xs)
      return new_xs, ys

  调用链路

  1. 参数估计流程:
  原始数据 → make_sample() → np.mean()/np.median() → 点估计值
  2. 抽样分布和置信区间流程:
  原始样本 → resample()重抽样 → 循环生成多个估计值
  → np.std()计算标准误差 → np.percentile()计算置信区间
  3. 假设检验流程:
  原始数据 → 计算观测统计量 → 定义零假设模型
  → simulate_groups()/permute()生成模拟数据
  → 计算模拟统计量 → compute_p_value()计算p值

  关键依赖库

  - NumPy: 基础数值计算(mean, std, percentile, random)
  - SciPy: gaussian_kde用于密度估计
  - empiricaldist: Pmf, Cdf等分布对象
  - Pandas: 数据框操作
  - Matplotlib: 可视化
```
