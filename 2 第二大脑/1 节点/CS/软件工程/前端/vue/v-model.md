---
draw:
title: v-model
tags: []
date created: 2024-06-23
date modified: 2024-11-12
---

## 与 v-bind 的区别

v-model 是双向绑定

v-bind 是单向绑定，单向数据绑定是指数据只能从 [模型 model](模型%20model) 流向 [视图 view](视图%20view)，当数据发生改变时，视图会自动更新。
但无法从 [视图 view](视图%20view) 流向 [模型 model](模型%20model)，比如，当用户手动更改 input 的值，数据不会自动更新。
