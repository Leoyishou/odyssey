---
draw:
tags: []
title: "@git"
date created: 2024-06-25
date modified: 2024-11-12
---

git 一定要用 [GUI](GUI.md)  
git 一定要用 [GUI](GUI.md)  
git 一定要用 [GUI](GUI.md)

## @如何规范 git commit  

如何规范你的 Git commit？- 阿里云开发者的文章 - 知乎  
https://zhuanlan.zhihu.com/p/182553920

## @新手如何用 [1 最近项目/++Vue学习/github](1%20最近项目/++Vue学习/github)

新手该如何使用 GitHub？- 程序员吴师兄的回答 - 知乎  
https://www.zhihu.com/question/21669554/answer/1230957628

## 本地仓库的本质是入度为 1，出度不限的 [有向图](有向图)

- [branch](branch) 本质是用终点 node 表示的 LinkedList
- [merge](merge) 的本质是给两个 [branch](branch) 生成一个共同的新的 [revision](revision)，[rebase](rebase) 是移植一个 branch 到另一个上
	- 一次 [revision](revision) 就是 [branch](branch) 上的一个 node 节点，[commit](commit.md) 操作可以生成一个新的 [revision](revision)，[tag](tag) 与某个 [revision](revision) 绑定
	- [head](head) 是当下的游标指向的位置
- [check out](check%20out) 可以切换 [branch](branch) 或者 [revision](revision)
	- 场景：要从 A [branch](branch) 切换到 B，这时候还不想把新写的 uncommited 的东西 [commit](commit.md) 到 A，那么就可以先 [stash](stash)
	- [cherry-pick](cherry-pick.md) 可以把某个 [revision](revision) 移动到某个 [branch](branch) 上

## 本地与远程库交互

在本地初始化一个远程库：[clone](clone.md)  
本地同步远程库的状态：[pull](pull)=[fetch](fetch)+[merge](merge)、 [pull -rebase](pull%20-rebase) = [fetch](fetch)+[rebase](rebase)  
把本地的状态推送到远程库：[push](push)
