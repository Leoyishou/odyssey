---
draw:
title: Vue3@
date created: 2024-06-23
date modified: 2025-04-14
---

#第一性原理

Vue 是为[[SPA 单页应用]]的开发服务的，[[dist]]

在 Vue 代码这一层，我们可以采取一套模块化、声明式且以状态管理和组件复用为核心的方法论，以确保整体代码结构既清晰又易于维护，同时能够灵活响应 UI 的不断演进与需求变更。下面给出一些实用的实践步骤和建议：

---

## 1. 组件化设计与分层架构

- **单一职责原则（SRP）：**  
    每个组件只关注一个单一功能或视图模块。例如，对于一个聊天 UI，可以将整体分为多个子组件：
    
    - **ChatHeader**：顶部的时间、标题及图标显示区域
        
    - **ChatMessages**：主要的消息列表
        
    - **ChatInput**：输入区、工具按钮
        
    - **NavBar**：底部导航条
        
    - **Feature Cards**：例如行程规划、酒店比价等独立模块
        
- **目录结构规划：**  
    建议建立专门的文件夹结构，如 `components` 用于通用组件，`views` 用于页面级组件，`composables` 用于抽离的业务逻辑等，这样有助于按照功能、模块或者领域进行分层管理和维护。
    

---

## 2. 采用 Composition API 管理状态与逻辑

- **解耦数据与视图：**  
    使用 Composition API 可以将状态（reactive state）、计算属性（computed）、观察者（watch）以及方法通过函数封装（composables）的方式抽离出来，让组件变得更加聚焦于展示和交互。
    
- **抽离复用逻辑：**  
    比如，你可以将“滚动到底部”这一交互逻辑封装成一个 composable 函数 `useScrollToBottom()`，然后在多个消息列表组件中复用。
    

    js

    

    Copy

    

    `// composables/useScrollToBottom.js import { onMounted, nextTick } from 'vue'; export function useScrollToBottom(containerRef) {   const scrollToBottom = () => {     nextTick(() => {       if (containerRef.value) {         containerRef.value.scrollTop = containerRef.value.scrollHeight;       }     });   };   onMounted(scrollToBottom);   return { scrollToBottom }; }`

    
- **使用 Provide/Inject：**  
    对于全局或跨层级共享的状态，比如用户信息或全局配置，可以使用 `provide` 和 `inject` 机制来减少 props 传递的层级深度。
    

---

## 3. 数据驱动与响应式编程实践

- **Props & Events：**  
    遵循“一次数据流下行、事件上行”的设计理念。父组件通过 props 向子组件传递数据，而子组件通过事件来传递用户交互行为。
    例如，在 ChatInput 中，当用户点击发送按钮时，组件通过 `emit('sendMessage')` 将事件上报给父组件进行处理。
    
- **使用 Computed 与 Watcher：**  
    利用 computed 属性来封装数据转换、格式化的逻辑；watcher 用于监控响应数据的变化，并触发后续的操作。例如，当用户输入变化时，可以即时计算是否显示错误提示或使按钮可用。
    

---

## 4. 样式管理与模块化

- **Scoped CSS 与 CSS Modules：**  
    每个 Vue 单文件组件都可以使用 `<style scoped>`，确保样式局部生效，避免全局污染。如果样式较为复杂，也可以使用 CSS Modules 或者 SASS/LESS 等预处理器进一步组织和复用样式。
    
- **设计 Tokens 与全局样式变量：**  
    定义统一的颜色、间距、字体等设计 tokens 放在全局样式文件中，确保整个应用的风格保持一致。例如，通过 CSS 变量或 SASS 变量来实现全局统一管理。
    

---

## 5. 状态管理与跨组件通信

- **简单场景下：**  
    如果组件层级不是特别复杂，可以仅依赖 props 与事件进行数据流管理。
    
- **复杂状态管理时：**  
    考虑使用 Pinia（或 Vuex）来管理全局状态，特别是用户会话、路由状态、全局通知等信息。利用 Pinia 可以将业务逻辑与视图更加清晰地分离。
    

---

## 6. 开发工具与代码风格

- **工具链：**  
    利用 Vite 或 Vue CLI 进行项目搭建，配置 ESLint、Prettier 保持统一代码风格，确保代码格式化、可维护性和可读性。
    
- **单元测试与集成测试：**  
    结合 Vue Test Utils 和 Vitest/Jest，为核心组件编写单元测试，保证关键交互逻辑如滚动、事件触发、计算属性更新等行为正确。
    

---

## 7. 性能优化与懒加载

- **懒加载组件：**  
    对较大的或者非首屏组件使用 Vue 的异步组件或 Vue Router 的懒加载功能，减少初始加载时的包体积。
    
- **代码拆分与动态 import：**  
    根据路由或者功能按需加载 JavaScript 模块，利用现代打包工具的 Tree Shaking 特性，优化整体性能。
    

---

## 8. 实践示例与流程

1. **组件搭建：**
    
    - 确定设计稿后，先定义好各个页面组件（比如 ChatHeader.vue、ChatMessages.vue、ChatInput.vue 等）。
        
    - 编写基础模板（HTML 结构）及挂钩简单的 props 和事件。
        
2. **逻辑抽离：**
    
    - 将复杂的内部状态管理、滚动监听、动画控制等逻辑拆分成 composable 函数，放入 `composables/` 目录中，便于复用与单元测试。
        
3. **数据对接与交互：**
    
    - 利用 computed 和 watch 实现响应式数据更新，比如实时更新用户输入、消息列表变化后自动滚动到底部。
        
4. **样式与动画：**
    
    - 制作初版设计稿后，根据设计指导文档（如 Material Design、Apple HIG）调整样式，利用 CSS transition 或 Vue 内置过渡组件 `<transition>` 及 `<transition-group>` 处理列表动画效果。
        
5. **测试与迭代：**
    
    - 每添加一个关键组件，进行单元测试和交互测试，确保无论是视图更新还是状态切换都符合预期。通过用户反馈进一步迭代优化代码与 UI 效果。
        

---

## 总结

具体到 Vue 代码层的方法论在于建立一套以组件化、模块化、响应式编程和复用性为核心的开发模式。通过拆分单一职责的组件、抽离逻辑到 composables、采用统一的设计 tokens、以及严格依循一流的状态管理和测试体系，可以确保你在将整体 UI 美化的同时，使代码保持清晰、易维护和高扩展性。这样的方法论既符合现代前端开发实践，也能够很好地支持从设计到实现、再到持续优化的全生命周期开发流程。

Vue 项目在执行 `npm run build` 后生成的 `dist` 目录就是打包后的静态文件，包含了所有前端资源，比如：

- index.html（入口文件）
- 编译后的 JavaScript 文件
- 编译后的 [[CSS]] 文件
- 静态资源（图片等）

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

```Java
在 Vue.js 中，<div class="chat-container" v-cloak> 是一个常见的写法，其中 v-cloak 是一个 Vue 指令。让我来解释一下它的作用：

v-cloak 的作用

v-cloak 指令用于在 Vue 实例尚未完成编译和渲染之前，隐藏未编译的 DOM 元素。换句话说，它可以防止页面加载时用户看到未解析的 Vue 模板代码（比如 {{ message }} 这样的占位符）。

工作原理

1. 当页面加载时，Vue 需要时间来解析模板并将数据绑定到 DOM。
    
2. 在这个短暂的解析过程中，如果不做处理，用户可能会看到原始的模板代码（例如 {{ message }}），这会影响用户体验。
    
3. v-cloak 指令会让带有该指令的元素保持隐藏（通常通过 CSS），直到 Vue 完成编译为止。
    
4. 一旦 Vue 编译完成，v-cloak 指令会被自动移除，元素就会显示出来。
    

使用方式

通常，v-cloak 会配合 CSS 使用。例如：

html


<div class="chat-container" v-cloak>
  {{ message }}
</div>

[v-cloak] {
  display: none;
}


- 在这个例子中，<div class="chat-container"> 在 Vue 编译完成前会被隐藏（display: none）。
    
- 当 Vue 加载完成并将 {{ message }} 替换为实际数据后，v-cloak 属性会被移除，CSS 规则不再生效，<div> 就会显示。
    

具体到你的例子

`<div class="chat-container" v-cloak> `表示这是一个带有 chat-container 类名的容器，可能是一个聊天界面的根元素。v-cloak 的作用是确保在 Vue 完成初始化之前，这个容器不会显示未编译的内容，从而提升加载时的用户体验。

为什么用 v-cloak？

- 避免闪烁问题：防止用户看到未渲染的模板。
    
- 简单易用：只需添加一个指令和简单的 CSS，无需复杂逻辑。
    
- 常见场景：在单页应用（SPA）或组件加载时特别有用。
    

如果你还有其他关于 Vue 或这段代码的问题，随时问我！
```
```Java







```





```Java
好的，我们来探讨一下样式（SCSS）和项目结构优化的问题。

关于样式抽取与 SCSS

你目前使用的是 Vue 的 <style scoped>，这是一个非常好的实践，它能确保组件的样式只作用于当前组件，避免全局污染。

经典问题变体： 这涉及到代码组织和关注点分离 (Separation of Concerns) 的权衡。我们希望样式易于维护、复用，并且与组件逻辑保持适当的关联。

解法/策略：

1. 是否必须抽取？

- 不一定。 对于中小型项目，或者组件样式相对独立、不复杂的情况，将样式保留在 .vue 文件内的 <style scoped> 中是完全可以接受的，甚至更方便，因为它将模板、脚本、样式聚合在一起，便于查找和修改。

- 何时考虑抽取？

- 共享样式/变量： 当你有很多跨组件共享的颜色、字体大小、间距单位、或者通用的样式模式（比如卡片、按钮的基础样式）时，抽取到 SCSS 文件中可以提高复用性和一致性。

- 样式逻辑复杂： 如果组件样式非常复杂，包含大量嵌套、计算或逻辑（虽然 CSS 变量也能处理部分情况），使用 SCSS 的高级特性（Mixins, Functions, Loops）可能会更清晰。

- 团队协作/设计系统： 在大型项目或需要对接设计系统时，将核心样式（如主题变量、基础 UI 元素样式）抽取出来，有助于统一规范和维护。
```
