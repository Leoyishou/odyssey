---
draw:
title: CSS
date created: 2024-06-21
date modified: 2025-02-16
---

[设计语言](设计语言.md) - [UIUX](UIUX.md) - [UI组件库](UI组件库.md) 本质是给了一堆已经设计好的 css

<!-- more -->

[Styles](Styles.md)

| 分类       | 子类     | 属性                                                               | 常用值                                                                 |            |
| -------- | ------ | ---------------------------------------------------------------- | ------------------------------------------------------------------- | ---------- |
| **布局类**  | 显示方式   | [display](display.md)                                            | `block`, `inline`, `flex`, `grid`, `none`                           | ✅          |
|          |        |                                                                  |                                                                     |            |
|          |        |                                                                  |                                                                     |            |
|          | Flex布局 | `flex-direction`<br>`justify-content`<br>`align-items`           | `row`, `column`<br>`center`, `space-between`<br>`center`, `stretch` |            |
|          | Grid布局 | `grid-template-columns`<br>`grid-gap`                            | `1fr 1fr`, `auto`<br>`10px`, `1rem`                                 |            |
|          | [[定位]] | `position`                                                       | `static`, `relative`, `absolute`, `fixed`                           |            |
|          | 浮动     | `float`                                                          | `left`, `right`, `none`                                             |            |
| **盒模型类** | 尺寸     | `width`, `height`<br>`max-width`, `min-height`                   | `100px`, `50%`, `auto`                                              | ✅          |
|          | 边距     | `margin`<br>`padding`                                            | `10px`, `1rem`, `auto`                                              | ✅          |
|          | 边框     | `border`<br>`border-radius`                                      | `1px solid black`<br>`5px`, `50%`                                   | ✅  圆角      |
| **视觉类**  | 颜色     | `color`<br>`background-color`                                    | `#fff`, `rgb(0,0,0)`<br>`rgba(0,0,0,0.5)`                           | ✅          |
|          | 背景     | `background-image`<br>`background-size`<br>`background-position` | `url()`<br>`cover`, `contain`<br>`center`, `top left`               |            |
|          | 阴影     | `box-shadow`<br>`text-shadow`                                    | `0 2px 4px rgba(0,0,0,0.1)`<br>`1px 1px 2px black`                  |            |
|          | 滤镜     | `filter`<br>`backdrop-filter`                                    | `blur(4px)`<br>`brightness(150%)`                                   | ✅          |
| **文字类**  | 字体     | `font-family`<br>`font-size`<br>`font-weight`                    | `Arial`, `sans-serif`<br>`16px`, `1.2rem`<br>`400`, `bold`          | ✅          |
|          | 文本     | `text-align`<br>`line-height`<br>`text-decoration`               | `left`, `center`<br>`1.5`<br>`underline`, `none`                    |            |
|          | 段落     | `text-indent`<br>`word-spacing`<br>`white-space`                 | `2em`<br>`2px`<br>`nowrap`, `normal`                                |            |
| **动画类**  | 过渡     | `transition`<br>`transition-duration`                            | `all 0.3s ease`<br>`0.5s`                                           |            |
|          | 动画     | `animation`<br>`@keyframes`                                      | `bounce 1s infinite`<br>`from`, `to`                                |            |
|          | 变换     | `transform`                                                      | `rotate(45deg)`<br>`scale(1.2)`                                     |            |
| **响应式**  | 媒体查询   | `@media`                                                         | `screen and (max-width: 768px)`                                     |            |
|          | 弹性单位   | `rem`, `em`, `vw`, `vh`                                          | `1.5rem`, `100vh`                                                   |            |
| **其他**   | 鼠标     | `cursor`                                                         | `pointer`, `not-allowed`                                            |            |
|          | 层级     | `z-index`                                                        | `1`, `999`, `-1`                                                    | ✅          |
|          | 不透明度   | `opacity`                                                        | `0.5`, `1`                                                          |            |
|          | 溢出处理   | `overflow`                                                       | `hidden`, `scroll`, `auto`                                          | ✅决定出不出现滚动条 |

`z-index: 1000; /* 确保模态框在最上层 */` 可以理解为图层的上下优先级

card 里面有 table，<el-card 外面必须套个 container 才能实现 table 居中的这个效果吗，这是什么原理？

- 外层 container 方法利用了 Flexbox 或 Grid 布局的能力来控制子元素的位置。
- 直接在 card 上应用样式方法使用了块级元素的 `margin: auto` 特性来实现水平居中。
- CSS Grid 方法利用了网格布局的强大特性来精确控制元素位置。

```css
justify: 自己怎么对齐

align：内部的 item 怎么对齐

display：控制显示隐藏

border-radius: 10px 20px 30px 40px;  控制圆角，从左上顺时针转
```

```Java
row.find("#t_replay").html("<a target='_blank' href='http://l-siriusvm16.h.cn2.qunar.com:8080/open/mockQueryOrderPrice.jsp?orderQuery="    + n.orderQuery+ + "&sectionQuery="+ n.sectionQuery +"'>回放</a>");
```

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F26%2F15-42-51-6c0da2fbab6f762ef4c6fd2236757efd-20240626154250-d057e5.png)  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F26%2F15-43-07-24344db62c5347678186c1e065b69acf-20240626154306-d974ea.png)  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F26%2F15-45-38-5c0c7a6c8de90ec16d10ad6a036a509e-20240626154537-237cea.png)

##


```css
.modal-content {
  background-color: #ffffff;
  width: 90%;
  max-width: 800px;
  height: 90vh;
  border-radius: 20px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 8px 100px rgba(42, 161, 240, 0.263);
  position: relative;


- 0 - x轴偏移量

- 阴影在水平方向的偏移

- 正值向右偏移，负值向左偏移

- 这里是0，表示不左右偏移

2. 8px - y轴偏移量

- 阴影在垂直方向的偏移

- 正值向下偏移，负值向上偏移

- 这里是8px，表示阴影向下偏移8像素

- 30px - 模糊半径(blur radius)

- 定义阴影的模糊程度

- 值越大，阴影越模糊柔和

- 值为0则阴影边缘清晰
}
```

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F11%2F03%2F23-37-53-a4f4e78e4f5cda38831a6c32807a9fb1-202411032337457-881f65.png)

## 边距

![CleanShot 2025-02-16 at 01.18.15@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_bl9yDkE0aJ%2F2025%2F02%2F16%2F01-18-23-004f87cd7acb877ddc824d7f2e82b012-CleanShot%202025-02-16%20at%2001.18.15-2x-650621.png)

## 记忆技巧

1. **从外到内**
   - 先掌握布局类（决定整体结构）
   - 再学习盒模型（处理具体区块）
   - 最后是内容样式（文字、颜色等）

2. **按使用频率**
   - 高频：`display`, `margin`, `padding`, `color`, `font-size`
   - 中频：`position`, `background`, `border`
   - 低频：动画、阴影等特效属性

3. **按功能场景**
   - 页面布局时关注布局类属性
   - 组件开发时注重盒模型和视觉类
   - 文字排版时使用文字类属性

4. **按相关性**
   - Flex相关属性放在一起学习
   - 字体相关属性一起掌握
   - 动画相关属性组合记忆

5. **常用组合**
   - 居中布局：`display: flex; justify-content: center; align-items: center;`
   - 文字样式：`font-size`, `font-weight`, `line-height`
   - 边框圆角：`border`, `border-radius`

这个分类方式可以帮助你：

- 更系统地学习和记忆 CSS 属性
- 在开发时快速定位需要的属性
- 建立属性之间的关联记忆
- 按照实际开发场景组织知识
