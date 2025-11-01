---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-31
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  


[[Vue3@]] vs React (TSX + Less) 对比表

| 特性       | Vue                           | React (TSX + Less)             |
| -------- | ----------------------------- | ------------------------------ |
| **文件后缀** | `.vue`                        | `.tsx` + `.less`               |
| **文件结构** | 单文件组件 (SFC)                   | 分离文件                           |
| **模板语法** | `<template>` 标签               | JSX (直接在 JS 中写)                |
| **脚本部分** | `<script setup>` 或 `<script>` | 直接在 `.tsx` 中                   |
| **样式部分** | `<style>` 或 `<style scoped>`  | 独立 `.less` 文件                  |
| **类型支持** | `.vue` + TypeScript           | `.tsx` (原生支持 TS)               |
| **数据绑定** | `{{ data }}`                  | `{data}`                       |
| **条件渲染** | `v-if` / `v-show`             | `{condition && <div>}` / 三元表达式 |
| **循环渲染** | `v-for`                       | `.map()`                       |
| **事件绑定** | `@click` / `v-on:click`       | `onClick`                      |
| **样式绑定** | `:class` / `:style`           | `className` / `style` 对象       |

## 文件示例对比


  对比一览

  - React：JSX → React Element Tree → Fiber 协调 → 真实 DOM
  - Vue：模板 → render 函数 → VNode Tree（patch）→ 真实 DOM

  小例子
```Java
  - React：
      - JSX: <div><p>Hello</p></div>
      - 等价：createElement('div', null, createElement('p', null, 'Hello'))
      - 本质是一个嵌套的元素对象树。
  - Vue：
      - 模板: <div><p>Hello</p></div>
      - 等价（渲染函数）: h('div', null, [h('p', null, 'Hello')])
      - 本质是一个嵌套的 VNode 对象树。
```


  易混点澄清

# 🧩 Cues



# 🪞Notes
