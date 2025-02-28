---
draw:
title: vue学习$
date created: 2024-07-17
date modified: 2025-02-20
---

```ts
.
├── dist                # 构建后的生产环境文件目录
│   ├── assets
│   ├── favicon.ico
│   └── index.html

├── package.json        # 项目配置和依赖声明文件
├── package-lock.json   # npm 依赖版本锁定文件

├── src                 # 源代码目录
│   ├── main.js            # 入口函数，要用 router、element-plus 这种都需要在这里写
│   ├── App.vue            # 根 Vue 组件
│   ├── router             # 各页面的跳转逻辑
│   ├── api                # 需要调的后端接口
│   ├── components         # 一些可复用的 vue 组件
│   ├── utils              # js 片段，一些可复用的工具函数
│   ├── assets
│   ├── config
│   ├── directive
│   ├── layout
│   ├── store
│   ├── theme
│   └── views

├── mock                # 放 mock 数据的地方
│   ├── adminTools.js
│   ├── baseTools.js
│   ├── priceTools.js
│   ├── temp.js
│   └── user.js

├── mockProdServer.js
├── index.html
├── node_modules       # npm 安装的依赖包目录，有点像 java 项目的 library
│   ├── @amap
│   ├── @babel
│   ├── vue-router
│   ├── vue-tsc
│   ├── vuex
│   ├── wrappy
│   └── zrender
├── pom.xml
├── public
│   └── favicon.ico
├── scripts
│   └── fix.sh
├── LICENSE
├── README.md
└── vite.config.js
```
