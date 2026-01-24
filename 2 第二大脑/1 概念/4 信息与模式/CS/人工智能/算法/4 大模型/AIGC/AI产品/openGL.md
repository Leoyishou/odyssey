---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 



# 🧩 Cues



# 🪞Notes


3. OpenGL / Cairo (渲染引擎)
角色：画笔

它是啥： 真正负责“把像素点画在屏幕上”的底层图形库。

为什么重要： 这就是为什么 Manim 分裂成了两个版本（你之前图片里的 CE 和 GL）：

Cairo (凯罗)： 是 ManimCE (社区版) 的默认引擎。它用 CPU 画图，画出来的线条非常圆润、完美，适合做高质量视频，但速度慢。

OpenGL： 是 ManimGL (3B1B版) 的引擎。它调用显卡 (GPU) 加速。速度极快，能实时由代码生成 3D 画面（就像玩游戏一样），但配置很难。