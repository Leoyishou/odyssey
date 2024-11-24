---
draw:
title: CSS
tags: []
date created: 2024-06-21
date modified: 2024-11-12
---

[设计语言](设计语言.md) - [UIUE](UIUE.md) - [组件库](组件库.md) 本质是给了一堆已经设计好的 css

<!-- more -->
  

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
