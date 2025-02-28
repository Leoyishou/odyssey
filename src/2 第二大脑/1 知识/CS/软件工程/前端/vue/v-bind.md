---
draw:
title: v-bind
date created: 2024-06-23
date modified: 2025-02-06
---

 v-bind 可以简写为 colon

下面代码中的 key color 这些属性前面加了 colon 后，都是可以动态改变的

```js
<el-timeline-item center  
                  v-for="(activity, index) in activities"  
                  :key="index"  
                  :icon="activity.icon"  
                  :type="activity.type"  
                  :color="activity.color"  
                  :size="activity.size"  
                  :timestamp="activity.timestamp"  
>  
  <div>  
    {{ activity.content }}  
    <el-tag v-if="activity.showTag" class="day-tag">1晚</el-tag>  
  </div>  
</el-timeline-item>
```

## 和 [ref](ref.md) 有什么区别

v-bind 是单向绑定  
单向数据绑定是指数据只能从 [模型 model](模型%20model) 流向 [视图 view](视图%20view)，当数据发生改变时，视图会自动更新。
但无法从 [视图 view](视图%20view) 流向 [模型 model](模型%20model)，比如，当用户手动更改 input 的值，数据不会自动更新。

ref 是绑定的是 model，bind 是动态绑定属性，实现动态改 view
