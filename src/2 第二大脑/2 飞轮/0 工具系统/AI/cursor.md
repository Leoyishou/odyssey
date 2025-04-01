---
draw:
tags: []
title: cursor
date created: 2024-09-09
date modified: 2024-11-12
---
### 打印
2. 找bug的时候， 我更喜欢用AI自动化的生成很多 print, 然后再根据结构让AI找出问题的地方， 缩小bug范围， 比直接让AI找出bug， 可能更快， 因为有时候我们找不到bug， 提示词本身就有误导

1. 从[Dribbble](Dribbble.md)找 UI 灵感，截图给 cursor
2. 修改 ai 给的第一版错误代码的时候，为了排查原因可以让 ai 加一些日志，然后运行，然后把日志结果给ai，帮助 ai 排查


https://www.v2ex.com/t/1070302#reply2

 Chat 是有记忆功能的长聊天，Composer 是一次性的对话

## 右侧 review

根据 git 内容的变更，来对代码做增两 review

## RAG

两种方式，`@`或者在setting-feature 里手动加文件  
所以，如果做什么屎山项目，可以多试试在提问之前，先把RAG给做了。

## Real World Examples

一些函数在 github 里的实际用法案例

## apply 前后版本的 checkout

1/ 先头脑风暴，再写代码 Claude/o1在这里是你最好的助手。你应该创建一个包含项目每一个细节的完整文档。

- 核心功能
- 目标和宗旨
- 技术栈和使用的包
- 项目文件夹结构
- 数据库设计
- 登陆页面组件
- 配色方案
- 文案 将所有这些内容放入一个名为 `[http://instruction.md](https://t.co/GzdTkD8jW4)` 的文件（名字随便起），这样Cursor可以随时进行索引。

2/ 获取一个 `.cursorrules` 文件 很多人忽略了这一步。我理解，编写 `.cursorrules` 文件可能让人望而生畏，但它确实能带来极大帮助。这是一个我总是推荐的优秀仓库，可以帮助你入门。选择你的技术栈，并根据你的偏好进行编辑：
https://github.com/PatrickJS/awesome-cursorrules?tab=readme-ov-file 这个网站提供了很多最佳实践

3/ 使用 v0 构建登陆页面 从你的 `[http://instructions.md](https://t.co/dpTHYqzjIN)` 文件中获取核心功能、配色方案和组件。额外提示：可以参考其他登陆页面的截图，以便让 v0 更好地理解你的想法。使用组件库，我推荐 shadcn，因为 v0 与它配合良好。我也常用 MagicUI。记住，你不需要让 v0 完美无缺。你只需要一个足够好的基础，便于你后续在 Cursor 中进一步编辑和优化。

4/ 聊天 vs 编辑器 使用聊天功能完成较小的任务，解释代码/命令。可以用它来提问和导航。用编辑器（Composer）来写代码，始终在编辑器中标记你的 `[http://instructions.md](https://t.co/dpTHYqzjIN)` 文件，并告诉它随着项目进展进行更新。每次只让编辑器执行一个任务，逐步进行更改。如果你让它编辑多个文件，它可能会产生幻觉，导致你失去控制。在批准更改之前，始终检查代码是否干净整洁。将你的Claude积分留给编辑器，使用GPT-4o-mini来进行聊天。


1. Docs 标记你的文档 复制你所使用框架的文档。进入Cursor设置 > 功能 > 文档 粘贴这些链接，并在聊天/编辑器中通过 `@ Docs` 使用它们。
