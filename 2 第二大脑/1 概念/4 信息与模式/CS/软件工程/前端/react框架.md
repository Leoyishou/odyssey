---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-23
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  

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
