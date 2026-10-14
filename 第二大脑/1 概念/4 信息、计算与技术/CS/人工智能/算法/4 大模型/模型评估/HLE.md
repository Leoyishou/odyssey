---
uid: 8031133d-678e-402d-9fd9-42c6ea9523fb
date created: 2026-06-24
date modified: 2026-06-24
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

32.1% 是某个模型/agent 在 HLE 上的答对率，不是题目分类。HLE 本身是 2,500 道专家出的闭卷型题，覆盖数学、自然科学、医学、计算机、人文社科等 100+ 方向；约 14% 要看图/图表，约 24% 是选择题，其余多是短答案。官方定位是“Google-proof”：不能靠搜一个事实就答，要有专家知识或多步推理。来源：[HLE 官方页](https://agi.safe.ai/)、[Scale leaderboard](https://labs.scale.com/leaderboard/humanitys_last_exam)、[Artificial Analysis 示例](https://artificialanalysis.ai/evaluations/humanitys-last-exam)。

# 🧩 Cues



# 🪞Notes


|方向|题目长相|考什么|
|---|---|---|
|古典学 / 铭文学|给一张罗马墓碑铭文图片，再给 Palmyrene script 的转写，让你翻译这段铭文。|不是普通翻译，得懂古文字、专名、墓志语境。|
|生态 / 鸟类解剖|问蜂鸟尾部某个 sesamoid bone 支撑多少对肌腱，要求只回答一个数字。|很窄的生物解剖事实，普通百科很难直接命中。|
|语言学 / 古俄语|给一组罗马化古俄语词组及“第几音节重读”的标注，让你归纳规律，再对 6 个新词组输出 6 位数字答案。|更像语言学奥赛题，重点是从样例中归纳重音规则。|

另外还有密码学题：给一段双层替换密码，说明其中一步是把 `a` 替成 `bd`，再给同密钥的长密文，让模型解密目标句；以及生物物理题：根据 FTIR 峰位变化判断缓步动物蛋白水凝胶形成时的二级结构变化。总体不是“常识问答”，更像各学科专家出的高难度闭卷题。