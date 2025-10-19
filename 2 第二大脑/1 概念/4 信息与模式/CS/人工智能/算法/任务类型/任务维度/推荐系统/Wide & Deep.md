---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-10
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  

**Wide & Deep 就是:**
- 左手拿着"记忆手册"(Wide - 记住具体规则)
- 右手拿着"智慧大脑"(Deep - 举一反三)
- 两手配合,既不死板,也不乱推荐

```Java
输入特征
   ├──→ Wide 部分 (线性模型) ──┐
   │                           ├──→ 联合训练 ──→ 输出
   └──→ Deep 部分 (神经网络) ──┘
```

# 🧩 Cues

[[DeepFM]]

# 🪞Notes

问题 1: 只用 Deep 模型会"过度泛化"
场景: 用户点了"冰美式咖啡"
- ❌ 纯 Deep 模型: "咖啡和咖啡很接近!" → 推荐"热拿铁"
- ✅ Wide & Deep: "用户明确要'冰'的!" → 推荐"冰拿铁"
有时记住特定组合作为规则和例外非常重要,比如用户要冰拿铁时,不能因为嵌入空间相近就泛化推荐热拿铁

问题 2: 只用 Wide 模型泛化能力差
- ❌ 纯 Wide 模型: 只能推荐"见过的"组合
- ✅ Wide & Deep: 可以推荐"没见过但合理的"组合
