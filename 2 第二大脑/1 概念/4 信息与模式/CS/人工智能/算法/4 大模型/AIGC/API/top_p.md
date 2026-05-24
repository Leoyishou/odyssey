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

如果调低 temperature，A 会变得更占优势，输出更稳定。  
如果设置 top_p = 0.9，模型只会在 A、B、C 里选，因为它们累计概率已经到 90%，D/E 被排除。

| 对比点               | temperature        | top_p                      |
| -------------------- | ------------------ | -------------------------- |
| 控制方式             | 改概率形状         | 截断候选范围               |
| 是否保留低概率 token | 还在，只是概率变了 | 可能直接被排除             |
| 低值效果             | 更确定、更像贪心   | 候选更少、更保守           |
| 高值效果             | 更随机、更跳跃     | 候选池更大，但不一定更发散 |
|                      |                    |                            |

实用建议：

- 做分类、抽取、JSON 输出、代码修复：temperature: 0 或 0.1~0.3，top_p: 1
- 做普通问答：temperature: 0.3~0.7，top_p: 0.9~1
- 做创意写作、头脑风暴：temperature: 0.7~1.0，top_p: 0.9~1
# 🧩 Cues



# 🪞Notes
