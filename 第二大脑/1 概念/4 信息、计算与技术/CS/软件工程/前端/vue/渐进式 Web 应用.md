---
date created: 2024-07-29
date modified: 2025-10-27
uid: c7ab2beb-efaa-4758-a8eb-a98e2bc440cd
aliases:
  - "PWA - 渐进式 Web 应用"
  - "PWA"
---
# Summary

1. 效果：快速一个网页应用，通过浏览器可以添加到桌面，类似 app 的使用体验
2. PWA 是一套标准，不是框架。用 Next.js、Vue、纯 HTML 甚至 WordPress 都能做 PWA

## 两个核心

1. `manifest.json`  - PWA 的"身份证"
	告诉浏览器这是个可安装的 App，定义：
	- 名称、图标
	- 主题颜色
	- 打开方式（standalone = 全屏，无浏览器框）

2. **Service Worker** - 离线能力的核心
	一个在后台运行的 JS 脚本，负责：
	- 缓存静态资源（HTML、CSS、JS、图标）
	- 让 App 即使断网也能打开


# Notes
```Java








### 3. **Next.js** - 开发框架

React 的增强版，自动处理路由、优化等
```
