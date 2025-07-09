---
comment_id: dfbb1a16
date created: 2024-12-03
date modified: 2025-04-14
draw: null
title: CSS 框架
---
CSS 框架主要有以下几类：

1. 传统全功能框架
- Bootstrap：最流行的 CSS 框架，提供完整的组件和样式
- Foundation：同样是全功能框架，强调响应式设计
- Bulma：基于 Flexbox 的现代 CSS 框架

1. 原子化 CSS 框架
- Tailwind CSS：使用原子类的方式构建界面
- Windi CSS：Tailwind 的替代品，按需编译
- UnoCSS：即时原子化 CSS 引擎，性能更好

1. UI 组件库
- Material UI：谷歌 [[Material Design]] 的实现
- Ant Design：蚂蚁金服开发的企业级组件库
- Element UI/Plus：Vue 生态中流行的组件库
- Chakra UI：现代化的 React 组件库

1. CSS-in-JS 方案
- styled-components：React 生态最流行的 CSS-in-JS 库
- Emotion：灵活的 CSS-in-JS 方案
- Stitches：高性能、类型安全的 CSS-in-JS

1. 低级别工具类框架
- PostCSS：CSS 处理工具
- SCSS/SASS：CSS 预处理器
- Less：另一个流行的 CSS 预处理器

Tailwind CSS 和 Tailwind UI 的关系是：

1. Tailwind CSS
- 是一个底层的 CSS 框架
- 提供了大量原子化的 CSS 类（utility classes）
- 是免费开源的
- 是基础工具，你可以用它来构建任何样式

1. Tailwind UI
- 是建立在 Tailwind CSS 之上的[[UI组件库]]
- 提供了一系列预构建的界面组件和模板
- 是付费的商业产品
- 由 Tailwind CSS 的创建团队开发维护

从图片中可以看到，多个 UI 组件库都基于 Tailwind CSS 构建，比如：

- Headless UI：专注于无样式组件
- Daisy UI：提供了 35 个组件
- Flowbite Vue：27 个组件  
等等

简单来说，Tailwind CSS 是基础框架，而 Tailwind UI 是在这个框架上构建的一个高级组件库产品。这就像是 Vue.js（基础框架）和 Element Plus（组件库）的关系。
