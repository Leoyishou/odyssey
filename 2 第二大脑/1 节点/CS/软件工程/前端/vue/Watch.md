---
draw:
tags: []
title: Watch
date created: 2024-07-31
date modified: 2024-11-12
---

```js
watch(() => props.data, () => {
  totalItems.value = props.data.length;
  currentPage.value = 1;
}, { immediate: true })
```

观察者模式，响应数据变化，用于前端分页

当 props.data 发生变化时，第二个参数的函数会被调用。在这个函数中，你更新了 totalItems.value 和 currentPage.value。具体来说，totalItems.value 被设置为 props.data 数组的长度，currentPage.value 被重置为 1。这通常用于更新页面显示的条目总数和重置当前页面。
