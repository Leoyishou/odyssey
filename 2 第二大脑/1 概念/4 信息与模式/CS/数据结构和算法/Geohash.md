---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-08
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

空间换时间的分块思想。
将二维的经纬度换成一维的二进制的行程区域的门牌号，能瞬间找到相邻的位置
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F09%2F08%2F15-11-10-8532238d3b215382ba514748b1834cb2-202509081511760-18fe81.png)
# Cues

[文件系统](文件系统.md)中的多级索引数据块

# Notes

假设，现在有一个数据库存储了全国的店铺，如果我们要查询距离五道口地铁站（坐标 39.9929°N，116.3379°E）最近的几个剧本杀店。
在查询的时候，即使采用全部遍历一一比较的策略，我们也没有必要真的遍历世界上所有的店铺，只需要选择北京市海淀区的店铺就可以了。这样我们就过滤了大量不可能是查询目标的数据，提高了查询效率，所需要的代价仅仅是为每个数据增加一个“省-市—区”的标签而已。这样的标签，实际上可以认为是对地理位置做了一个以“区”为粒度的分块，在检索距离最近的标签时，我们直接查找同一个区内的就行。
