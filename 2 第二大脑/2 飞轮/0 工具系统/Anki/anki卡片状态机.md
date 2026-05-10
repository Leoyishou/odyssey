---
date created: 2024-06-02
date modified: 2025-07-10
uid: 0f6864ff-3a42-43dd-8289-b7bd66fe5235
---
Anki 核心学习状态可以按这个理解：

| 状态 | 含义 | 会不会复习 |
|---|---|---|
| `New` | 新卡，还没开始学 | 会，等你第一次学 |
| `Learning` | 新卡刚开始学，还在短间隔学习步骤里 | 会，很快出现 |
| `Review` | 已经毕业成长期复习卡 | 会，按间隔复习 |
| `Relearning` | 复习卡答错后，回到短间隔重学 | 会，很快出现 |
| `Suspended` | 暂停卡片 | 不会，直到手动取消暂停 |
| `Buried` | 临时埋藏，通常当天不再出现 | 暂时不会，之后自动/手动恢复 |

你说的“熟知不需要复习”，在 Anki 里严格来说不是 `known` 状态，而通常应该用：

| 目标 | 推荐做法 |
|---|---|
| 永久不想复习 | `Suspend` 暂停 |
| 只是标记自己认识 | 加 `known` 标签，但卡片仍可能复习 |
| 想低频复习 | 让它进入 `Review`，并给很长间隔 |
| 完全不导入/不显示 | 不建卡或删除卡 |

所以现在我们导进去的 `known` 只是 Viva 来源标签，不等于 Anki 里的“熟知免复习”。如果你希望这 1,900 个 `known` 词块不进入复习队列，应该把这些卡批量 `Suspend`；如果只是想保留但低频复习，就应该保留为 `New/Review`，靠 Anki 间隔控制。





When only the question is shown, Anki shows three numbers like 6 + 9 + 59 at the bottom of the screen. These represent the [new](new.md) cards (blue), cards in [learning](learning) (orange), and cards to[review](%20review) (green).

![image.png#pic_center|650](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F02%2F21-38-41-06b6f592698733deafd6be333b525c60-20240602213841-9f36ff.png)
