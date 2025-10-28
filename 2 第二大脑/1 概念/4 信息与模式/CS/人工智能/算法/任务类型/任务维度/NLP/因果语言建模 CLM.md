---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-25
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  

本质是续写

# 🧩 Cues



# 🪞Notes

## CLM (Causal Language Modeling)

**因果语言建模** - 就是传统的"预测下一个词"任务

**工作方式：**

````
输入: "今天天气"
模型预测: "很好"

输入: "今天天气很"
模型预测: "好"
```

- 从左到右逐个预测
- 只能看到前面的词，不能看到后面的（单向注意力）
- GPT系列模型用的就是这个
- **学习所有token**，样本效率高
