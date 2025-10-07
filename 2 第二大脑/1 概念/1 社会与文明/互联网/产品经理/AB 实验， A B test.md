---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

场景：库里换了新球鞋，三分命中率提升了吗？
解决：
```Java
场景：库里换了新球鞋，三分命中率提升了吗？

旧球鞋：投100次，进42个（42%）  → A组（对照组）
新球鞋：投100次，进48个（48%）  → B组（实验组）

问题：48% vs 42%，是真提升还是运气波动？

AB测试的本质：
1. H0：新球鞋命中率 = 旧球鞋命中率（新鞋没用）
2. H1：新球鞋命中率 ≠ 旧球鞋命中率（新鞋有效）
3. 用Z检验计算P值
4. P < 0.05 → 拒绝H0 → 新鞋有效！
```


1. 本质是[假设检验](假设检验.md)
2. 应该把 AB 实验和[[开关 feature gate]]当成一个 Object，才能 seamless 地做实验

| 维度      | 说明                     |
| ------- | ---------------------- |
| **本质**  | 双样本[[假设检验]]            |
| **最常用** | [[Z检验]]（比例，如点击率、转化率）|
| **次常用** | [[t检验]]（均值，如停留时长、消费金额）|
| **多版本** | χ²检验 或 ANOVA           |
| **H0**  | 新版本 = 旧版本（改版没用）|
| **H1**  | 新版本 ≠ 旧版本（改版有效）|

# Cues



# Notes


这个视频系统而完整地讲解了AB实验的关键知识点，旨在帮助观众达到AB实验认知的顶尖水平。内容主要围绕**实验的价值**、**实验体系的设计**和**重要的统计知识**展开。

以下是视频的总结：

## 1. 为什么要做实验？(实验的价值)

- **价值源于“意外”（Surprises）：** 实验的目标不应是确认已有的好想法，而是挑战你的想法[[02:11](http://www.youtube.com/watch?v=9kh1YvzZYks&t=131)]。当一个你认为好的想法被实验证明是坏的，或者一个你认为不重要的想法被发现非常好时，实验的价值才得以体现[[02:28](http://www.youtube.com/watch?v=9kh1YvzZYks&t=148)]。
    
- **假设的成功率低：** 大部分公司的初始假设成功率只有 20% 到 30%[[02:49](http://www.youtube.com/watch?v=9kh1YvzZYks&t=169)]，这意味着约 80% 的情况下你会感到意外。
    
- **促进指数级增长：** 通过实验，可以及时停止（kill）那些带来负面影响（negative impact）的功能（feature），从而使指标（metrics）增长更加稳定和持续，实现更高的指数增长[[03:20](http://www.youtube.com/watch?v=9kh1YvzZYks&t=200)]。
    
- **提高覆盖率：** 应该想方设法提高实验的覆盖率（coverage），目标是让所有新做的 feature 都经过实验验证[[03:42](http://www.youtube.com/watch?v=9kh1YvzZYks&t=222)]。
    
- **带来更高的“智力诚实” ([[intellectual honesty 智识诚实]])：** 实验通过事实数据帮助人们了解想法的不足之处，并带来一个更有自主性（agency）和更高开发速度的工程师文化[[09:01](http://www.youtube.com/watch?v=9kh1YvzZYks&t=541)]。
    

## 2. 如何设计实验体系？

- **Feature Gate 与 Experiment 结合：** 在底层系统设计上，应将 **Feature Gate**（控制功能是否对用户可见的条件规则）和 **Experiment** 设计成同一个对象[[04:21](http://www.youtube.com/watch?v=9kh1YvzZYks&t=261)]。
    
    - 这样，工程师在进行逐步发布（stage rollout）时（如先发布给 1% 用户），实验也会自然而然地开启[[05:58](http://www.youtube.com/watch?v=9kh1YvzZYks&t=358)]。
        
    - 这可以让你从一年做几十个实验扩展到几万个实验，因为实验本身不再是开发的瓶颈或限制（constraint）[[10:46](http://www.youtube.com/watch?v=9kh1YvzZYks&t=646)]。
        
- **构建可信赖的数据系统：** 数据的可信赖性至关重要[[11:09](http://www.youtube.com/watch?v=9kh1YvzZYks&t=669)]。
    
    - 建议使用 **Matrice Catalog** (指标目录) 等产品化的工具，集中定义日志表（logging table）和指标，实现从日志到指标的端到端可追溯性（end-to-end traceable），从而确保数据的可信度[[12:04](http://www.youtube.com/watch?v=9kh1YvzZYks&t=724)]。
        
- **坚持简单化：** 简单和标准化是实现实验系统可扩展性（scalable）的关键[[13:35](http://www.youtube.com/watch?v=9kh1YvzZYks&t=815)]。不要为了展示专业价值而将事情复杂化，除非复杂性是为了解决一个真实存在的问题[[14:01](http://www.youtube.com/watch?v=9kh1YvzZYks&t=841)]。
    

## 3. AB实验的重要统计知识

- **Trade-off（权衡）：** 在样本量（sample size）给定的情况下，实验运行时间越长，就能检测到越小的效应（minimum detectable effect），即准确度越高[[15:30](http://www.youtube.com/watch?v=9kh1YvzZYks&t=930)]。
    
- **并行实验（Concurrent Experiments）：** 大多数情况下，同时跑多个实验是可以的，交互效应（interaction effect）不是一个大问题，只要实验是正交的即可[[16:38](http://www.youtube.com/watch?v=9kh1YvzZYks&t=998)]。
    
- **方差减小（Variance Reduction）：**
    
    - 目标是降低最小可检测效应（MDE），从而缩短实验时间[[18:46](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1126)]。
        
    - 最有效的方法是 **CUPED**（Covariate Adjusted Post-Experiment Data，协变量调整的实验后数据）或回归调整（regression adjustment）[[19:03](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1143)]。
        
    - CUPED 可以帮助你减少约 80% 应被减少的方差[[20:03](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1203)]。
        
- **贝叶斯（Bayesian）与频率学派（Frequentist）：**
    
    - 两者是对同一数据的不同哲学解释，但不会改变实验数据本身[[20:50](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1250)]。
        
    - 如果不使用带有主观偏向（bias）的先验（prior），两者在决策规则上最终是一致的[[21:13](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1273)]。
        
    - 不建议使用带有主观倾向的 prior 来进行点估计（point estimation），因为它很容易被滥用[[21:59](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1319)]。
        
- **序贯检验（Sequential Testing）：**
    
    - **解决 Peeking（偷看数据）问题：** 在实验结束前提前查看结果并决定是否上线（Peeking），会导致虚假发现率（False Discovery Rate, FDR）升高[[25:06](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1506)]。
        
    - 序贯检验通过“花费功效（spend power）”来解决这个问题，它在一开始会给予一个更宽的置信区间（confidence interval），从而惩罚（penalize）提前查看的行为[[26:10](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1570)]。
        
    - 建议采用保守（conservative）的序贯检验方法，假设用户会无限次地偷看数据，从而系统性地降低 FDR[[27:22](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1642)]。
        
    - 如果实验结果显示出统计显著的**负面**效应，可以提前终止（abandon/kill）实验（可能存在 Bug 或极差的用户体验），但不鼓励提前上线（shape early）[[27:33](http://www.youtube.com/watch?v=9kh1YvzZYks&t=1653)]。
        

**视频链接：** [AB实验，有哪些重要却不为人知的知识？｜课代表数据大师课5](http://www.youtube.com/watch?v=9kh1YvzZYks)
