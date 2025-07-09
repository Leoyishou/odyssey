---
draw:
title: display
date created: 2025-02-16
date modified: 2025-02-16
---

我来创建一个图示来展示 `display: flex` 的主要概念和特性。

<chat-artifact id="flex-diagram" version="0" type="image/svg+xml" title="Flexbox Layout Explanation">
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 500">
    <!-- 标题 -->
    <text x="300" y="30" font-family="Arial" font-size="20" text-anchor="middle" fill="#333">Flexbox 布局示意图</text>
    
    <!-- 主轴说明 -->
    <line x1="100" y1="80" x2="500" y2="80" stroke="#FF6B6B" stroke-width="2"/>
    <polygon points="500,80 490,75 490,85" fill="#FF6B6B"/>
    <text x="300" y="70" font-family="Arial" font-size="14" text-anchor="middle" fill="#FF6B6B">主轴 (Main Axis)</text>
    
    <!-- 交叉轴说明 -->
    <line x1="50" y1="100" x2="50" y2="400" stroke="#4ECDC4" stroke-width="2"/>
    <polygon points="50,400 45,390 55,390" fill="#4ECDC4"/>
    <text x="40" y="250" font-family="Arial" font-size="14" text-anchor="middle" transform="rotate(-90,40,250)" fill="#4ECDC4">交叉轴 (Cross Axis)</text>
    
    <!-- Flex 容器 -->
    <rect x="100" y="100" width="400" height="300" fill="#f8f9fa" stroke="#dee2e6" stroke-width="2"/>
    <text x="110" y="120" font-family="Arial" font-size="12" fill="#666">Flex Container</text>
    
    <!-- Flex 项目 -->
    <rect x="120" y="150" width="100" height="200" fill="#FFE0B2" stroke="#FFB74D" stroke-width="2"/>
    <text x="170" y="250" font-family="Arial" font-size="12" text-anchor="middle" fill="#666">Item 1</text>
    
    <rect x="240" y="150" width="100" height="200" fill="#C8E6C9" stroke="#81C784" stroke-width="2"/>
    <text x="290" y="250" font-family="Arial" font-size="12" text-anchor="middle" fill="#666">Item 2</text>
    
    <rect x="360" y="150" width="100" height="200" fill="#BBDEFB" stroke="#64B5F6" stroke-width="2"/>
    <text x="410" y="250" font-family="Arial" font-size="12" text-anchor="middle" fill="#666">Item 3</text>
    
    <!-- 属性说明 -->
    <text x="100" y="440" font-family="Arial" font-size="14" fill="#333">常用属性：</text>
    <text x="120" y="460" font-family="Arial" font-size="12" fill="#666">• justify-content: 主轴对齐方式</text>
    <text x="120" y="480" font-family="Arial" font-size="12" fill="#666">• align-items: 交叉轴对齐方式</text>
    <text x="350" y="460" font-family="Arial" font-size="12" fill="#666">• flex-direction: 主轴方向</text>
    <text x="350" y="480" font-family="Arial" font-size="12" fill="#666">• gap: 项目间距</text>
</svg>
</chat-artifact>

`display: flex` 的主要特点：

1. **容器和项目**
   - 设置 `display: flex` 的元素成为 flex 容器
   - 其直接子元素自动成为 flex 项目

2. **主要属性**：
   - `flex-direction`: 决定主轴方向
     - `row`（默认）：从左到右
     - `column`：从上到下
     - `row-reverse`：从右到左
     - `column-reverse`：从下到上
   - `justify-content`: 主轴对齐方式
     - `flex-start`（默认）：起点对齐
     - `center`：居中对齐
     - `flex-end`：终点对齐
     - `space-between`：两端对齐
     - `space-around`：环绕对齐
   - `align-items`: 交叉轴对齐方式
     - `stretch`（默认）：拉伸对齐
     - `center`：居中对齐
     - `flex-start`：起点对齐
     - `flex-end`：终点对齐
   - `gap`: 项目之间的间距

3. **常见用途**：
   - 导航栏布局
   - 居中对齐
   - 等分布局
   - 响应式设计

使用示例：

```css
.container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
}
```

Flexbox 是现代网页布局的重要工具，它提供了一种更加灵活和直观的方式来创建各种布局结构。

## grid
