---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-04
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

K 折的意思就是不同的数据集划分方式得到K个模型，训练K个模型后取他们的平均分数作为最终的结果


```python
N_SPLITS = 5 # 使用3-5折可以在效果和时间上取得良好平衡
skf = StratifiedKFold(n_splits=N_SPLITS, shuffle=True, random_state=42)

```
# Cues



# Notes
