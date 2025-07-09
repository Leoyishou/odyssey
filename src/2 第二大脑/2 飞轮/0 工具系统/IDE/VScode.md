---
comment_id: fff8600e
date created: 2024-07-11
date modified: 2024-11-12
draw: null
tags: []
title: VScode
---
setting.json

作用：obsidian 和 logseq 的批量编辑能力较差，可以用 vscode 打开进行批量编辑，比如正则替换，yaml 区的批量编辑等。安装 foam 插件以后，核心的双链功能已经完美支持了。

同时还有可能使用上 copilot 插件的威力，辅助创作。

- [Github Copilot](Github%20Copilot)
	- 智能 AI 写代码，这是未来
- [tabnine](tabnine)
	- 和 pilot 互补，但感觉已经被 copilot 秒杀了。
- [gitlens](gitlens)
	- 图形化的 git 操作有时候就是方便
- [project manager](project%20manager)
	- 并行多项目开发必备
- [rest client](rest%20client)
	- 轻量地快速发起一个 http 请求
- [remote - containers](remote%20-%20containers)
	- 连接远程容器开发必备
- [remote - ssh](remote%20-%20ssh)
	- 连接远程服务器开发必备
- [markdown lint](markdown%20lint)
	- 自动格式化 md 文件

插件学习

## 不同级别的配置设置

您说得对,Visual Studio Code 中确实存在多个位置的 settings.json 文件,它们的作用领域是不同的。让我为您简要解释一下:

1. 用户设置 (User Settings):
    - 位置: 用户文件夹下
    - 作用: 适用于所有的 VS Code 实例和项目
2. 工作区设置 (Workspace Settings):
    - 位置: 项目根目录下的.vscode 文件夹中
    - 作用: 仅适用于当前工作区(项目)
3. 文件夹设置 (Folder Settings):
    - 位置: 多根工作区中每个文件夹的.vscode 目录下
    - 作用: 仅适用于该特定文件夹及其子文件夹

这些设置的优先级从高到低为:文件夹设置 > 工作区设置 > 用户设置。这意味着更具体的设置会覆盖更一般的设置。

这种分层的设置方式允许您在不同级别上自定义 VS Code 的行为,从而实现更灵活的配置管理。

## 跑[Vue](Vue)项目

在 Cursor 的终端中，导航到你的项目根目录，然后运行以下命令：
