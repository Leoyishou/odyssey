---
comment_id: 0d9da175
date created: 2023-12-03
date modified: 2025-01-22
draw: null
tags: []
title: Anki算法
---
- [信息源](#%E4%BF%A1%E6%81%AF%E6%BA%90)
- [名词解释](#%E5%90%8D%E8%AF%8D%E8%A7%A3%E9%87%8A)
- [时间维度上：学习的「两阶段」](#%E6%97%B6%E9%97%B4%E7%BB%B4%E5%BA%A6%E4%B8%8A%EF%BC%9A%E5%AD%A6%E4%B9%A0%E7%9A%84%E3%80%8C%E4%B8%A4%E9%98%B6%E6%AE%B5%E3%80%8D)
	- [神经元建立阶段](#%E7%A5%9E%E7%BB%8F%E5%85%83%E5%BB%BA%E7%AB%8B%E9%98%B6%E6%AE%B5)
	- [神经元保活阶段](#%E7%A5%9E%E7%BB%8F%E5%85%83%E4%BF%9D%E6%B4%BB%E9%98%B6%E6%AE%B5)
- [存在的问题](#%E5%AD%98%E5%9C%A8%E7%9A%84%E9%97%AE%E9%A2%98)
	- [忽视了神经元的图结构](#%E5%BF%BD%E8%A7%86%E4%BA%86%E7%A5%9E%E7%BB%8F%E5%85%83%E7%9A%84%E5%9B%BE%E7%BB%93%E6%9E%84)
	- [第一个阶段不适合在anki内完成](#%E7%AC%AC%E4%B8%80%E4%B8%AA%E9%98%B6%E6%AE%B5%E4%B8%8D%E9%80%82%E5%90%88%E5%9C%A8anki%E5%86%85%E5%AE%8C%E6%88%90)
- [空间切面上](#%E7%A9%BA%E9%97%B4%E5%88%87%E9%9D%A2%E4%B8%8A)
	- [优先待毕业的，其次保活，其次最新的](#%E4%BC%98%E5%85%88%E5%BE%85%E6%AF%95%E4%B8%9A%E7%9A%84%EF%BC%8C%E5%85%B6%E6%AC%A1%E4%BF%9D%E6%B4%BB%EF%BC%8C%E5%85%B6%E6%AC%A1%E6%9C%80%E6%96%B0%E7%9A%84)
	- [参数反馈的评价指标,或者说观测性](#%E5%8F%82%E6%95%B0%E5%8F%8D%E9%A6%88%E7%9A%84%E8%AF%84%E4%BB%B7%E6%8C%87%E6%A0%87,%E6%88%96%E8%80%85%E8%AF%B4%E8%A7%82%E6%B5%8B%E6%80%A7)

## Flag

- flag1 打给那些自己知道一意多单词的，等待之后的语义分析词频或者其他技术处理
- flag2 打给那些带了逻辑关系的，属于逻辑工具的
- flag3 是自己读音有问题的
- flag4 是打好标记待删除的

## 信息源

https://docs.ankiweb.net/deck-options.html?highlight=bury#burying

## 名词解释

graduate：建立进化到保活阶段  
lapses：保活回退到建立阶段  
leech：多次 lapses 后会被 leech，类似墨墨中的顽固单词打标  
suspend：逃离整个成长机制，类似墨墨中的标记熟知  
bury：今天先不回答，明天再回答

## Card state

[Anki State 示意图](Anki%20State%20示意图)  
![image.png#pic_center|650](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F13%2F21-08-25-36241769e0343da9534dbbdd19fa106c-20240613210825-c51868.png)

## 时间维度上：学习的「两阶段」

### 神经元建立阶段

对应 new card queue，
目标是毕业，
核心是通过几个 step 实现神经元的初步建立，比如 1m 10m 1d 这三个点形成了 1m-10m 10m-1d 1d-xx 等三个 step  
所有的动作影响的是目前位于第几个 step，good 可以移动到下一个

### 神经元保活阶段

对应 review queue，
核心是失效时间，其增长倍数 ease 是一个大于 1.3 的数字, 默认为 2.5,  
积极反馈的时候倍数增大，惩罚的时候倍数减小，极端情况下回退到习得阶段。
所有的动作最终都是影响距离下一次激活 refresh 的时间  
![image.png|600](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/20240206023828.png)

## 状态机转变

从 review 到 new 是通过 again

| dfda | new1  | new2  | new3 | a   | 2.5x | 2.5 * 1.5 | 1.2  |
| ---- | ----- | ----- | ---- | --- | ---- | --------- | ---- |
| new1 |       |       |      |     |      |           |      |
| new2 |       |       |      |     |      |           |      |
| new3 |       |       |      |     |      |           |      |
| a    |       | again |      |     | good | esay      | hard |
| 2.5x | again |       |      |     |      |           |      |
|      |       |       |      |     |      |           |      |

## 存在的问题

### 忽视了神经元的图结构

BFS 的时间复杂度大于 DFS

### 第一个阶段不适合在 anki 内完成

因为习得的本质在于图结构中目标节点能被快速触达，有赖于不断思考，调整图结构，同时加强连接边的牢固性

## 空间切面上

### 优先待毕业的，其次保活，其次最新的

https://faqs.ankiweb.net/what-spaced-repetition-algorithm.html

### 参数反馈的评价指标,或者说观测性

学会看 anki 内置的数据统计

## 实践

如果英语成功迁移到 anki 的话，可以把 CS 卡片也迁过来，最好是自制一个前端预览页面类似 markji

- Again  
    Moves the card back to the first step setted in [Learning/Relearning Steps.](https://docs.ankiweb.net/deck-options.html?#learning-steps)
    
- Hard  
    Repeats the current step after the first step, and is the average of Again and Good.
    
- Good  
    Moves the card to the [next step](https://docs.ankiweb.net/deck-options.html?#learning-steps). If the card was on the final step, the card is converted into a review card (it 'graduates').
    
- Easy Immediately converts the card into a review card.

学习 Anki 的算法涉及对其背后的记忆原理、算法逻辑以及如何在软件实现中应用这些原理的理解。以下是一些步骤和资源，可以帮助你学习 Anki 的算法：

1. **了解间隔重复原理**：
    
    - 阅读关于间隔重复（Spaced Repetition）的基础理论，如 Ebbinghaus 的遗忘曲线。
    - 了解记忆和遗忘的心理学基础，特别是长期记忆和短期记忆的差异。
2. **研究 Anki 的算法**：
    
    - Anki 使用的是一种修改版的 SuperMemo 算法（特别是 SM2）。研究 SuperMemo 及其不同版本（如 SM2, SM5, SM15 等）的原理。
    - 通过 Anki 的官方文档和论坛了解其算法的特定实现。
3. **深入了解 Anki 的应用**：
    
    - 使用 Anki，熟悉其功能和设置。实际使用可以帮助你更好地理解算法是如何应用的。
    - 阅读有关 Anki 插件开发的文档，了解如何通过编程方式与 Anki 的系统交互。
4. **参考开源代码和社区讨论**：
    
    - Anki 是一个开源项目，其代码在 GitHub 上可用。深入研究代码可以帮助你理解算法的具体实现。
    - 参与 Anki 社区，如 Reddit 的 Anki 板块或其他相关论坛，与其他用户交流经验。
5. **阅读相关文献**：
    
    - 阅读关于记忆、学习和间隔重复的学术文献，包括最新的研究成果。
    - 查找关于 SuperMemo 算法和其他类似系统的研究文章。
6. **实践和实验**：
    
    - 在 Anki 中创建自己的卡片，观察复习间隔是如何变化的。
    - 尝试调整 Anki 的设置，如调整复习间隔、易度等级等，观察这些变化如何影响学习效果。

通过上述方法，你可以逐步深入理解 Anki 算法的工作原理和应用方式。这不仅有助于更有效地使用 Anki，还可以启发你对间隔重复学习方法的深入思考。

  
