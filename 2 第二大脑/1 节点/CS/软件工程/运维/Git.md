---
aliases: [设置当前仓库]
draw:
tags: []
title: 设置当前仓库
date created: 2024-06-24
date modified: 2024-12-27
linter-yaml-title-alias: 设置当前仓库
---

## Git Commit Message

|关键词|描述|
|---|---|
|**feat**|新功能（feature）|
|**fix**|修复bug|
|**docs**|文档修改|
|**style**|格式调整（不影响代码运行的变动）|
|**refactor**|代码重构|
|**perf**|性能优化|
|**test**|增加或修改测试|
|**chore**|构建过程或辅助工具的变动|
|**revert**|回滚到上一个版本|
|**merge**|代码合并|
|**sync**|同步主线或分支的bug|

|关键词|描述|
|---|---|
|**build**|项目构建相关的修改（如webpack配置）|
|**ci**|持续集成相关的修改|
|**to**|产生diff但不自动修复问题（通常与fix配合使用）|
|**pub**|适用于静态博客仓库，增加或修改内容|

## 常用操作

| 类别   | 子类别  | 命令                | 含义                        |     |
| ---- | ---- | ----------------- | ------------------------- | --- |
| 本地操作 | 仓库维度 | `git init`        | 初始化`.git`，文件夹开始被 git 控制   |     |
|      | 分支维度 | ```bash

## 设置当前仓库

git config pull.rebase true

## 或全局设置

git config --global pull.rebase true

## 然后再拉取

git pull

```0      | 列出、创建或删除分支                |     |
|      |      | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```1    | 切换分支或恢复工作树文件              |     |
|      |      | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```2       | 合并一个或多个分支到当前分支            |     |
|      |      | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```3      | 在另一个分支基础之上重新应用提交          |     |
|      | 文件相关 | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```4         | 从 change 状态到 staged 状态    |     |
|      |      | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```5      | 查看工作目录和暂存区的状态             |     |
|      |      | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```6   | 提交暂存区的更改                  |     |
|      |      | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```7        | 显示文件的变更                   |     |
|      |      | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```8       | 显示文件每一行的最后修改者             |     |
|      | 历史相关 | ```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```9         | 查看提交历史                    |     |
|      |      | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```0       | 重置当前 HEAD 到指定状态           |     |
|      |      | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```1 | 应用某个特定的提交                 |     |
|      |      | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```2         | 创建、列出、删除或验证一个 GPG 签名的标签对象 |     |
|      | 临时存储 | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```3       | 暂存当前工作目录的更改               |     |
|      |      |                   |                           |     |
| 远程交互 | 仓库维度 | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```4       | 克隆远程仓库到本地                 |     |
|      |      | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```5      | 查看远程仓库                    |     |
|      |      | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```6 | 从远程仓库下载对象和引用              |     |
|      | 分支维度 | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```7        | 从远程仓库拉取最新更改并合并到当前分支       |     |
|      |      | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```8        | 将本地的提交推送到远程仓库             |     |
| 配置   |      | ```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```9      | 获取和设置仓库或全局选项              |     |

## Submodule

本质就是父文件夹有一个```bash
A---B---C (master)
     \
      D---E (origin/master)
```0，然后子文件夹也有```bash
A---B---C (master)
     \
      D---E (origin/master)
```1

## diff

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F10%2F15-25-04-5fc2a01876036d2a56ca3f18b4cfde17-202410101525293-73780d.png)

## update


这个错误是因为本地分支和远程分支出现了分歧，Git 需要你明确指定如何处理这种分歧。有三种解决方案：

1. 使用 merge 方式（最安全）：
```bash
# 设置当前仓库
git config pull.rebase false

# 或全局设置
git config --global pull.rebase false

# 然后再拉取
git pull
```

1. 使用 rebase 方式（保持历史整洁）：

```bash
# 设置当前仓库
git config pull.rebase true

# 或全局设置
git config --global pull.rebase true

# 然后再拉取
git pull
```

2. 只允许快进合并：

```bash
# 设置当前仓库
git config pull.ff only

# 或全局设置
git config --global pull.ff only

# 然后再拉取
git pull
```

这三种方式的区别：

1. Merge (rebase false):
- 创建一个新的合并提交
- 保留完整的历史记录
- 可能会使提交历史看起来比较混乱

```bash
A---B---C (master)
     \
      D---E (origin/master)
```

1. Rebase (rebase true):
- 将本地提交移到远程提交之后
- 历史记录看起来更整洁
- 可能需要解决更多冲突

```bash
A---B---D---E---C' (master)
```

1. Fast-forward only (ff only):
- 只有在不需要合并的情况下才允许拉取
- 最安全的选择
- 如果有分歧会直接失败

快速解决方案：

```bash
# 如果你想保留所有历史记录，用 merge
git pull --no-rebase

# 如果你想要整洁的历史记录，用 rebase
git pull --rebase

# 如果你想要更安全的方式，用 fast-forward
git pull --ff-only
```

建议：

1. 对于个人项目，使用 rebase 保持历史整洁
2. 对于团队项目，使用 merge 保持历史完整
3. 如果不确定，使用默认的 merge 策略

如果你想查看当前的配置：

```bash
git config --get pull.rebase
git config --get pull.ff
```

需要我详细解释某个方案吗？

```Java
-----BEGIN CERTIFICATE-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQDaWyT+qv8dduj8
6M4EBcV08mF3pzqQQ5mHhTIhP94u4A4Z/Vb70cmucG64LkDP1tCWhrcHCDpNgmL5
uSqB++wra5KS6faqKjyTTUZU+d9VHbvEzjMzE6525IZW65scQgKNzIRI1tSeDh3a
c7vYdMLxVdDH94ED8erCKYvcnkPMdHRRY7hsHTQ0z0FD2bWspTcOao5jwSfg1MVs
WlELMl0sCV866DR2O9Nc8C+BQ+GIP1hXAvxv8/nLVUSpdtPr8dZgoiXb2EbI9fTn
6NODo6WkTX2jx/klK/rlJV7z6AXOlIg5x6KGGnp4v7iV9RLjoAYTl9m8TYUy2Pl8
B4SEqHvNAgMBAAECggEAAbxsrNKp6kkBY4HFVSTBM/1GVzSlUE024zjLOnzyJ9Mm
5/MXiTNOJDzA5v/evfHiOpDx7RSJaoekgLUSa7I+Gsurs88E464n/G+f/sZ2AL/J
0aMZc89BzJbjo4n5gHrDRiKi+mis2MamqALSDELdyhxQARSZOiSm/jayDDjwGif4
gBtXmeqJUUjlXjXONbw/rbvgWeYCCu8jx1lrnkLoEeFvKuRTSYqTDO9iAAtesAc9
yvLpjHeD9FIT/fAopnvdnKR9WEuBqj3vZh/nGVKCNxKGQ6a3tuLW7fbJLEYRq2tI
EocXu6gU+hJ6k/rp5JVpCJJ6Fs+7VLDX+WGJZeNVaQKBgQD92n09G/opiLFEh2jJ
I9fQM6LaMXB2KOrXz3v3pD/qKvgw6rZlmq0z9jxrxlDr7FZtk63X+bWH+psFqiwp
67JNxN+t16j1XZvYVO59Kq8dzS2rnJZ+2qCRidNiBZypFezc562Eotmh9IuBIK8b
4hRvXaSiBbGMKpjb2JmXB/EVxQKBgQDcM9CW8ggNSlz/Q2skIZjl1/olEGjx6aS+
n7GuFuACczjnmaALgJveHOovXTiDk1jBM9dkMh22aOvQS71w6ej3RfNcYy4Vuarp
t9mthOZQLwBPIQVTR2Xw6LBTt5DWZsbSpV9jELea2rKCWtHt6wRSNJThJjOMJUZy
R6u+2WM2aQKBgD94W4vl7Nxpovjhal+wKIuT/YnUhqvkeyNOa051sM1ZxQhmBwjM
bbXqQR0pTp5OTyXCoNq5QpwnGo+GBtjMZ55uNQcAPuxqZUbnpWGCRJqM7HhWBILf
IvqAnWuKv6q/IqPxAT+CgCuwOxkkFORB2J4nqbwg9E15cRK2y2ApyiuJAoGAcD8d
YgRoJFY2x4wXTmit5uPMSB5wUHhYGS+QQ/y3HaicZjqCQbrCOVul6JTtXOy8Pmft
sM53JOeg5SHmpGsn7PxETGDmOf7V56NnG7N2L4N19eAQNd5cUYxbbaFR/GiRiwjv
cbEw1ogKnLYs8sls/3Z8ObTea98IZPKx0mw/zZkCgYA49/x9SH8CnLT/W6zunbQ7
QpQKB5QmOCh7uxSi6CxGs3KTyCmEqDj2p9/ajtqWnscNYbQu9Dyj+ASoqcLZnPBB
iP8DD7UZEMHFzUEj1TFQ9VynH3EpjML75uEk6v+gLq9qhFp3G+HOo1mGlGguq3wm
XcnteNJZzErXw2I/VbPdDQ==
-----END CERTIFICATE-----



```
