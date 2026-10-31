---
uid: 17ff9743-e1bc-4e43-b28f-68f5c31c4e06
date created: 2026-10-30
date modified: 2026-10-30
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

## 一、记忆单元

**每一路召回的都是同一种东西：记忆单元（fact）。** 每个 fact 带一个 `type`，取值是 `world`、`experience` 或 `observation`。四路检索只是“用什么方式找”不同，找出来的东西类型一样。⁠[![](https://img.notionusercontent.com/ext/https%3A%2F%2Fwww.google.com%2Fs2%2Ffavicons%3Fdomain%3Dhindsight.vectorize.io%26sz%3D64/raw?imgBuildSrc=createUnsignedImageVariantURL)](https://hindsight.vectorize.io/developer/api/recall)⁠

**三种记忆类型**

|type|是什么|例子|
|---|---|---|
|`world`|客观事实|“Alice 在 Google 工作”|
|`experience`|事件和对话，也就是经历|“Alice 升职为高级工程师”（带 `occurred_start` 时间）|
|`observation`|由多条记忆整合出来、去重过、有证据支撑的观点或结论|从多次对话里总结出的用户偏好|
⁠

## 二、**四路检索**

| 路             | 找什么                      | 对应前面说的类型       |
| ------------- | ------------------------ | -------------- |
| Semantic（语义）  | 意思相近、说法不同的记忆             | **稠密检索**       |
| Keyword（BM25） | 精确的人名、术语                 | **稀疏检索**       |
| Graph（图遍历）    | 通过共同实体关联起来的记忆，哪怕字面上完全不重合 | 图检索            |
| Temporal（时间）  | 某个时间段里的记忆，比如“上周”“三月”     | 偏结构化（按时间过滤和排序） |

## 三、存

1. 确定存入哪个 bank
2. 输入文档：1 号、2 号
3. 生成分块 1号文档 - 3 个分块、2 号文档 - 5 个分块
4. 抽取事实/经历（抽取用`gpt-6.1-terra`），
    1. 8 个分块一起抽取出 3 个事实/ 2个经历
    2. 实体识别：3 个事实/ 2个经历提取出 6 个实体
5. 归纳阶段（归纳用 `gpt-6.1-sol`）
    1. 事实部分
        1. 观察：多条相关事实summary 一下
    2. 经历部分
        1. 观察：多条相关经历summary 一下

## 事实和经历的区别
- **事实（world）：** 关于外部世界的客观信息，和 agent 本身无关。例如「Leonard 的主力号码是 177 开头」「RDS 基础款每年约 228 元」。
- **经历（experience）：** agent 自己做过、发生在自己身上的事，是第一人称的。例如「我 22 时 52 分把 18 条写进了 grok-bot::research」「我第一次冷启动测试因为证书缺失失败了」。
# 🧩 Cues



# 🪞Notes

