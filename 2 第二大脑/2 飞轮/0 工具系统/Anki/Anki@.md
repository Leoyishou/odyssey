---
date created: 2024-05-12
date modified: 2025-07-10
uid: e85fb2fc-c7e0-4bf9-8ed7-0893bf6f31d4
---
1. **不要把 Anki 当学习本体，把它当“懒惰点探测器 + 复现调度器”。** 每次刷卡时你不是在证明自己会了，而是在寻找哪里还只是熟悉、哪里还没有真正建模。
2. 要让 **懒惰点变成一等公民**：刷卡的目标不是全部答对，而是每天抓到几个“我其实没 dive in”的点。
3. 大脑会偷懒地选择最省力路径：
	- 看到卡面有熟悉感，就按 Good
	- 看到答案后觉得“哦我知道”，但其实没主动想出来
	- 遇到难卡不 deep dive，只想赶紧过
	- 只记住这张卡的问法，换个问法就不会了
4. Karpicke & Roediger 2008 的实验强调 repeated testing 比 repeated studying 更能巩固长期记忆

# Cues

[[Goodhart's law]]



# Notes

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/20260521144041224.png)





卡片的 [anki卡片状态机](anki卡片状态机.md)

[bury](bury.md) 临时冻结

[suspend](suspend.md) 永久冻结

## 临时学习集

这张图片展示的是 Anki（一款流行的间隔重复学习软件）中创建或编辑一个过滤牌组（Filtered Deck）的界面。让我为您解释一下这个界面的主要功能：

1. Deck（牌组）：
   - Name（名称）: "Filtered Deck 01:30" 是这个过滤牌组的名称。

2. Filter（过滤器）：
   - Search（搜索）: "deck:viva is:due" 是过滤条件，表示从名为 "viva" 的牌组中选择当前到期（due）的卡片。
   - Limit to（限制数量）: 100 张卡片
   - Selected by（选择方式）: Random（随机选择）

3. Options（选项）：
   - "Reschedule cards based on my answers in this deck"（根据在此牌组中的回答重新安排卡片）已被选中。
   - "Enable second filter"（启用第二过滤器）未被选中。

4. "Show any excluded cards"（显示任何被排除的卡片）是一个可点击的链接。
5. 底部有 Help（帮助）、Cancel（取消）和 Build（构建）按钮。

过滤牌组的主要用途是：

1. 临时学习：从现有牌组中快速创建一个临时学习集，专注于特定类型的卡片（如到期的、特定标签的等）。
2. 复习策略：可以根据特定条件（如困难度、最近表现）来创建复习集。
3. 考试准备：在考试前快速复习特定主题或最近学习的内容。
4. 自定义学习顺序：通过过滤和随机选择，可以改变常规学习顺序。
5. 集中复习：将分散在多个牌组中的相关内容集中起来复习。

这个功能允许用户灵活地组织和复习他们的学习材料，以适应不同的学习需求和场景。

                

## Anki Connect

"allowDuplicate": True,`

  ## 这将告诉 Anki 不要进行重复检查，允许添加重复的卡片

```python
payload = {
	"action": "addNote",
	"version": 6,
	"params": {
		"note": {
			"deckName": deck_name,
			"modelName": "active_brick",  # 使用active_brick卡片类型
			"fields": {
				"context": context,
				"incorrect_translation": mapping_wrong_english,
				"correct_translation": mapping_correct_english,
				"pronunciation": audio_url
			},
			"options": {

				"duplicateScope": "none" 
			},
			"tags": ["added_by_api"]
		}
	}
}
```
