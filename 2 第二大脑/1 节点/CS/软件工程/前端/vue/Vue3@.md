---
draw:
title: Vue3@
tags: []
date created: 2024-06-23
date modified: 2024-11-12
---

| Category            | Topics                                                                                                                                                                                                                                                                               |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Essentials          | - Creating an `Application<br>`- Template `Syntax<br>`- Reactivity `Fundamentals<br>`- Computed `Properties<br>`- Class and Style `Bindings<br>`- Conditional `Rendering<br>`- List `Rendering<br>`- Event `Handling<br>`- Form Input `Bindings<br>`- Lifecycle `Hooks<br>`- `Watchers<br>`- Template Refs |
| Reusability         | - `Composables<br>`- Custom `Directives<br>`- Plugins                                                                                                                                                                                                                                    |
| Built-in Components | - `Transition<br>`- `TransitionGroup<br>`- [KeepAlive](KeepAlive.md)<br>- `Teleport<br>`- Suspense                                                                                                                                                                                                     |
| Scaling Up          | - Single-File `Components<br>`- `Tooling<br>`- `Routing<br>`- State `Management<br>`- `Testing<br>`- Server-Side Rendering (SSR)                                                                                                                                                               |
| Best Practices      | - Production `Deployment<br>`- `Performance<br>`- `Accessibility<br>`- Security                                                                                                                                                                                                            |
| TypeScript          | - `Overview<br>`- TS with Composition `API<br>`- TS with Options API                                                                                                                                                                                                                     |
| Extra Topics        | - Ways of Using `Vue<br>`- Composition API `FAQ<br>`- Reactivity in `Depth<br>`- Rendering `Mechanism<br>`- Render Functions & `JSX<br>`- Vue and Web `Components<br>`- Animation Techniques                                                                                                     |

## Vue 的特点

[组件](组件.md)  
[Vue3 和 Vue2](Vue3%20和%20Vue2.md)  
[渐进式 JS 框架](渐进式%20JS%20框架.md)  
JS 版本：[ES6](ES6.md)  
[模块化的开发方式](模块化的开发方式.md)  
[ref](ref.md)  
[mount](mount.md)

## 前端的本质？

[模型 model](模型%20model) 和 [视图 view](视图%20view)，模型就是数据，视图的话是一个 [DOM](DOM.md)，会有一个 [生命周期](生命周期.md)。

## 浏览器上会发生哪些事？

- 用户输入 url 跳转到对应页面——[router](router.md)
	- 用户在页面上点击后跳转到其他页面
	- 未登录不得访问——[全局前置守卫](全局前置守卫.md)
	- 来回切换页面的时候保留输入框信息——[状态管理](状态管理.md) [keep-alive](keep-alive.md)
- 利用 [生命周期](生命周期.md) 实现网站整个背景密布 [水印码](水印码.md)
- 请求后端接口拿数据：[axios](axios.md)
- 页面要动态变化：[v-bind](v-bind.md)、[v-model](v-model.md)  
- [v-for](v-for.md)
	- ![image.png|300](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F18%2F21-11-26-58a150f03afcf8c747139315ef229e10-20240718211126-a63a24.png)
- 上图卡片三行的 [遍历展示](遍历展示.md)
	- ![image.png#pic_center|650](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F23%2F17-03-48-5520d96d8c67befe28fe43086cf24602-20240623170348-90a96f.png)
	- 每行 tag 支持 [显隐](显隐.md)，只有第二行展示 tag：
	- 第二行 tag 的『4 晚』是 [动态计算](动态计算.md) 的，[computed](computed.md)
- 从 A 页面切到 B 页面，再切回 A 后，之前填的数据仍然保留——[组件缓存](组件缓存.md)、[状态管理](状态管理.md)

## 拆组件

[组件](组件.md)

## 现成组件

[UI组件库](UI组件库.md)

## 实战

[报价检查工具的设计语言](报价检查工具的设计语言)

## 抽象到学习方式

比较过去的表格式学习，和现在的图式学习 [第二大脑](第二大脑.md)
