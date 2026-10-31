---
uid: 168b1de2-9d56-4b97-abfd-2cb27d022d54
date created: 2026-10-31
date modified: 2026-10-31
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 



# 🧩 Cues



# 🪞Notes

**1. 一次搜多个写法（用正则把同义词并在一起）**

```
grep -rniE "核心看板|主看板|仪表盘|驾驶舱|dashboard|cockpit|kpi" .
```

`-i` 忽略大小写，所以 `CoreDashboard`、`core_dashboard`、`core-dashboard` 都能被 `dashboard` 命中。命名风格的问题基本靠这个解决。

**2. 先搜“入口”，再顺藤摸瓜**

中文名常出现在多语言文案、菜单或路由配置里：

```
第 1 步：grep "核心看板"
  → 在 locales/zh.json 里命中："menu.coreBoard": "核心看板"
第 2 步：grep "coreBoard"
  → 在 router.ts 里命中：{ path: '/core', component: CoreBoardPage }
第 3 步：grep "CoreBoardPage"
  → 找到组件文件
```

这一步很关键。**每次搜到的结果，都会给下一次搜索提供新的关键词。** 这和前面说的图检索思路一样：从一个实体出发，沿着引用关系一跳一跳走下去。区别在于这里由 LLM 手动来走。

**3. 从结构入手（如果有** **`find`** **或** **`ls`****）**

先看目录结构，比如 `pages/`、`views/`、`dashboard/`，经常比搜内容更快。