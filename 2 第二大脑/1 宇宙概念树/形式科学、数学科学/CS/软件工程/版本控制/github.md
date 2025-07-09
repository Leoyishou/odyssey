---
comment_id: 2292e89b
date created: 2024-07-22
date modified: 2025-03-08
draw: null
title: github
---
谢邀，我来分享一下我从大一萌新一路走来用Github踩坑的一些经验。

先说个冷知识：GitHub个人主页其实可以DIY成动态简历。在仓库里新建一个和自己账号同名的repo，写个README.md文件，用[Markdown](https://zhida.zhihu.com/search?content_id=712626173&content_type=Answer&match_order=1&q=Markdown&zhida_source=entity)语法加上动态图标和项目展示，这样你就拥有了一个赛博名片。如果未来找工作，面试的时候能够当着面试官介绍你在Github上的项目，会很加分。

Github上有很多优秀的[开源项目](https://zhida.zhihu.com/search?content_id=712626173&content_type=Answer&match_order=1&q=%E5%BC%80%E6%BA%90%E9%A1%B9%E7%9B%AE&zhida_source=entity)，要复刻或者学习开源项目，首先需要定一个目标，自己想要学会哪些东西，比如说想学习操作系统，可以复刻一个开源小型操作系统的实现，如果想自己搭建一个个人博客，可以照着开源博客实现前后端程序，或者只是想了解一点一些有意思的项目，可以确定自己想要使用的技术栈，看看有没有人做过。

Github上的资源很丰富，所以你先得学会如何过滤信息，检索自己需要的内容，下面是一些搜索技巧：

**1.关键词组合搜索**

- **按技术栈**：`language:python stars:>1000`（搜索Python项目，星标超1000）
- **按主题**：`topic:machine-learning` 或 `topic:web-development`
- **按文件名**：`filename:README.md`（找有详细文档的项目）
- **排除词**：`NOT tutorial`（过滤教程类仓库）

**2\. [Awesome系列](https://zhida.zhihu.com/search?content_id=712626173&content_type=Answer&match_order=1&q=Awesome%E7%B3%BB%E5%88%97&zhida_source=entity)**

- 搜索 `awesome[技术名]`，如 `awesome-react`，获取高质量资源合集。
- 直接访问 [Awesome Lists](https://link.zhihu.com/?target=https%3A//github.com/topics/awesome) 主题页。

**3\. 趋势项目**

- 访问 [GitHub Trending](https://link.zhihu.com/?target=https%3A//github.com/trending)，按日/周/月查看热门项目。
- 筛选语言（如Python、JavaScript）关注新兴技术。

学会如何检索之后，就可以开始自己的学习之路了，有个小建议是刚开始复刻开源项目的时候最好选择文档丰富，社区氛围好的来尝试，不然很容易一开始就卡死，丧失了继续学习的动力。

千里之行始于足下，如果你从大一就可以开始自己写项目做项目，相信你在毕业的时候一定会感谢大一的自己的。

注册账号，然后用啊～

给几个具体的建议：

1）经常浏览高星项目，了解最新动态。

2）主动参与各类开源项目和志愿者。技术好的发 PR (Pull Request)，技术还在修炼的，可以参与翻译、写介绍软文等等。要主动积极参与啊～做个热爱生活的人也是大学学习生活的一个目标。

3）可以用 Jekyll 写 Github Pages，拥有一个属于自己的Blog，写作能力永远是个人能力中的第一生产力，没有之一。

4）熟练掌握 git、github action、github project 等功能。但不要局限于这些功能，体会这些 SaaS 背后的工程管理精髓，包括但不限于：如何思考流程优化、成本与技术、自动化等等等等～

5）认识志同道合的伙伴～同龄人一起成长，碰到大佬就适当结交、合理求带。

读书千遍，其义自现。积极参与进去，自然会有自己的答案。

开源的世界很精彩，加油！

现在马上立即就去找几个自己喜欢的项目做起。先给这些项目提交文档、再给这些项目提交代码补丁。最后弄几个大家都喜欢的项目，比如 b 站音乐播放器、b 站视频下载器，热门游戏编辑器之类的。目标就定在 1000 个 stars.

比你开局捡到个老爷爷还有效。

等你入行以后，你一定会感谢现在的你在知乎提出这个问题。

如何开始在 github 上学习东西？- 吴师兄学算法的回答 - 知乎  
https://www.zhihu.com/question/30119197/answer/1772131025

## fork 和 clone

fork 和 clone 最大的区别是 fork 之后我 github 账号里的内容会变，而 clone 的话，和我的 github 账号其实没啥关系

- **fork**：
    当你在 GitHub 上 **fork** 某个公共仓库时，会在你的 GitHub 账号下生成这个仓库的一个副本。这意味着你的 GitHub 主页上会出现一个独立的仓库，其他人也可以从你的这个 fork 仓库中查看代码或进行 clone。这是一个 GitHub 平台级的操作，直接影响你的账号内容。
- **clone**：
    当你在本地 **clone** 某个 GitHub 仓库时，只是将其代码复制到你的本地电脑文件系统中。GitHub 账号本身并没有发生任何变化，你的主页也不会增加新的仓库。Clone 只是一个本地操作，不会对你的 GitHub 账号产生直接影响。

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F22%2F20-24-42-c1c0c47b25e8e27011a734442041517c-20240722202441-46bf1e.png)
