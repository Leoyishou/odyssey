---
uid: c5b29969-ef7c-4a68-9b49-a100d614d747
---
[[superpower]]
[[phone Use]]

**判断多个****skill****要不要在一个****agrnt****下就看公司里这些技能的人会不会合作** **如果从不碰头那说明在具体目标上一般不会共同合作**

核心原则：Skill 是场景入口，脚本`script`是原子能力。不需要为每个原子能力都建一个 skill。

> [!info]
> **“无头应用（Headless App）”** 这个比喻几乎触及了本质。
> 作为产品经理和有 CS 背景的人，你可以把 **Claude Skills** 理解为：**运行在 LLM 这个“操作系统”之上的、剥离了 GUI（图形界面）的微服务或功能模块。**
> 本质上是将提示词、工具代码、专业知识、素材等封装成一个持久化的资产，让 Claude 能重复执行特定任务，
> "技能越细分，Claude 表现越好 - 单一巨型技能会产生泛化输出，独立技能永远不会混淆上下文。"

总结：最值得安装的 Top 5

| 排名 | Skill | 来源 | 理由 |
| :--- | :--- | :--- | :--- |
| 1 | superpowers 全家桶 | obra | 完整开发方法论，TDD + 子代理驱动 |
| 2 | frontend-design | anthropics | 解决 AI 生成 UI 千篇一律的问题 |
| 3 | docx/pdf/xlsx/pptx | anthropics | 文档处理刚需 |
| 4 | Trail of Bits Security | 社区 | 安全审计必备 |
| 5 | mcp-builder | anthropics | 扩展 Claude 能力 |

## 我的 skill

| 1 |
| :--- |
| [[ima_downloader]] |

## 二、本地环境 or 远程 sandbox

| 特性 | Claude Code | claude.ai |
| :--- | :--- | :--- |
| **运行环境** | 你本地电脑 | 远程沙盒 |
| **依赖保留** | ✅ 永久保留 | ❌ 每次重装 |
| **文件访问** | ✅ 本地所有文件 | ❌ 需要上传 |
| **网络限制** | ✅ 无限制 | ⚠️ 有限制 |
| **Skills 存储** | `~/.claude/skills/` | 上传 zip |
| **执行速度** | ⚡ 本地快 | 🐢 远程慢 |
| **安装包** | ✅ 装一次永久 | ❌ 每次重装 |

## 一、四种能力级别的 skill

| 四种级别 | | | | |
| :--- | :--- | :--- | :--- | :--- |
| 简单 Skill： | 预制的prompt | 如 commit-helper，生成 git 提交信息 ： | `allowed-tools: 为空` | |
| 带工具权限的 Skill | 带 bash tool 的 | 如 code-reviewer，用于代码审查 | `allowed-tools: Read, Grep, Glob` | |
| 多文件 Skill： | 带自己 py 脚本的 | 如 pdf-processing，支持提取文本、填表、合并 PDF | `allowed-tools: Read, Bash(python:*)` | |
| 有子 agent 的 Skill | 调用其他 skill | | `skills: pr-review, security-check` | |

## 二、os 在远程

是 **Anthropic 服务端的沙盒环境**，不是你本地的 macOS！

Claude 操作的是：
- **远程 Linux 沙盒**（Ubuntu 24）
- 每次对话都是独立的临时环境
- 文件系统会在对话结束后重置

不是：
- ❌ 你本地的 macOS
- ❌ 能访问你电脑的文件
- ❌ 能直接操作你的应用程序

---

### 📁 文件交互方式

上传流程：
1. 你上传文件到 claude.ai
2. 文件被传到服务端的 `/mnt/user-data/uploads/`
3. Claude 可以在沙盒中读取和处理

下载流程：
1. Claude 在沙盒中创建文件（如 docx、pdf、代码等）
2. 文件放到 `/mnt/user-data/outputs/`
3. 你可以下载到本地

### Skills 设计原则：尽量用预装的包

环境重置规则：
- **同一对话内**：环境持续存在，依赖只需装一次
- **新对话**：环境重置，需要重新安装

Anthropic 的优化：
1. **常用包预装**：pandas, numpy, requests 等常见库已经预装
2. **快速镜像**：使用 CDN 加速的包管理器
3. **Skills 设计原则**：尽量用预装的包

```text
# 这些已经可用，无需 pip install 
pandas, numpy, matplotlib, requests, PIL (Pillow), openpyxl, python-docx, pypdf, beautifulsoup4, opencv-python, scikit-learn, torch (基础版)
```

## 三、四个作用范围

- **Enterprise**
  - 由组织统一管理（“See managed settings”）
  - 应用于组织内的所有用户
  - 适合企业级统一规范或安全策略
- **Personal**
  - 路径：`~/.claude/skills/`
  - 只对你个人生效，跨所有项目都能用
  - 适合个人工作流、实验性功能
- **Project**
  - 路径：`.claude/skills/`（在具体项目仓库里）
  - 对该仓库的所有协作者生效
  - 适合团队共享的项目特定技能
- **Plugin**
  - 随 Claude Code 插件一起打包
  - 安装插件的用户自动获得
  - 适合通用功能 or 第三方扩展

这四个维度其实就是 **作用范围的层级**：
- 从最大范围的 **Enterprise**（组织级），
- 到个人级 **Personal**，
- 再到项目级 **Project**，
- 最后是插件级 **Plugin**。