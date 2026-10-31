---
uid: 10f6e163-1ff9-4a24-98e3-0bc9b157dc94
date created: 2025-10-19
date modified: 2025-10-19
aliases:
  - "RN (React Native)"
  - "RN"
---
   - **是什么？**  
     React Native 是 Facebook（现 Meta）开发的一个开源框架，用于使用 JavaScript 和 React 构建**跨平台原生移动应用**。它不是简单地渲染 Web 页面，而是通过桥接（Bridge）机制将 JS 代码转换为平台的原生组件（如 Android 的 View 或 iOS 的 UIView），从而实现“学一次，到处写”（Learn once, write anywhere）。简单说，就是用 Web 开发者的技能（JS/React）来做接近原生的 App。
   
   - **优缺点：**
     - **优点**：开发效率高，支持热更新（不用重新发布 App），跨平台（iOS 和 Android 共用大部分代码），社区活跃。
     - **缺点**：性能不如纯原生（尤其复杂动画或高负载场景），需要处理桥接开销，可能有兼容性问题。
   
   - **应用场景**：适合中型项目，如二级页面或功能稳定的模块（例如电商 App 的详情页）。知名案例：Facebook、Instagram、Airbnb 等 App 部分功能用 RN 实现。
   
   - **为什么前端同学提到？** 前端开发者熟悉 React/JS，所以 RN 是他们的舒适区，能快速迭代。
