---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-08-26
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

When I review my [CLAUDE.md/slash](http://claude.md/slash) command and agent files, it feels like reading my own software philosophy—a reflection of what I've learned, what I value, and how I think code should be built. If it doesn't resonate with you personally, it won't guide the AI effectively.


# Cues



# Notes
我来介绍一下 Claude Code 的常用命令，包括一些需要谨慎使用的"危险"命令：

## 基础命令

### 项目管理
```bash
# 创建新项目
claude-code new <project-name>

# 在当前目录初始化项目
claude-code init

# 查看项目状态
claude-code status
```

### 代码操作
```bash
# 让 Claude 编写代码
claude-code write "实现一个排序算法"

# 让 Claude 审查代码
claude-code review <file>

# 让 Claude 重构代码
claude-code refactor <file>

# 让 Claude 修复 bug
claude-code fix <file>
```

## ⚠️ "危险"命令（需谨慎使用）

### 批量操作命令
```bash
# 批量修改多个文件
claude-code batch-edit "*.js" --prompt "添加错误处理"

# 自动应用所有建议（无需确认）
claude-code apply --all --no-confirm

# 递归处理整个目录
claude-code process . --recursive
```

### 系统级操作
```bash
# 执行 Claude 建议的系统命令
claude-code exec --allow-system

# 自动安装依赖
claude-code deps --auto-install

# 清理和重构整个项目
claude-code cleanup --aggressive
```

### 代码生成相关
```bash
# 完全重写文件
claude-code rewrite <file> --force

# 基于描述生成整个模块
claude-code generate-module "完整的用户认证系统"

# 删除"冗余"代码
claude-code optimize --remove-unused
```

## 安全建议

### 使用前备份
```bash
# 创建备份
git commit -am "Before Claude Code changes"

# 或使用内置备份
claude-code backup create
```

### 安全模式
```bash
# 开启安全模式（所有更改需确认）
claude-code config set safety-mode on

# 设置只读模式（只分析不修改）
claude-code --read-only
```

### 查看变更
```bash
# 预览将要进行的更改
claude-code <command> --dry-run

# 查看历史操作
claude-code history

# 撤销上次操作
claude-code undo
```

## 实用技巧

### 配置文件 (.claude-code.yml)
```yaml
# 设置默认行为
safety:
  confirm-changes: true
  backup-before-change: true
  
exclude:
  - node_modules/
  - dist/
  - "*.min.js"
  
limits:
  max-file-size: 100KB
  max-files-per-operation: 10
```

### 常用组合
```bash
# 安全地重构
claude-code refactor src/ --dry-run
# 确认后再执行
claude-code refactor src/

# 增量式改进
claude-code review --suggest-only
claude-code apply --interactive
```

## ⚠️ 特别注意

**绝对要避免的操作：**
1. 在生产代码上直接使用 `--force` 或 `--no-confirm`
2. 对关键系统文件使用 `rewrite` 或 `cleanup --aggressive`
3. 在没有备份的情况下批量修改
4. 给予过于宽泛的权限（如 `--allow-all`）

建议先在测试环境或个人项目上熟悉这些命令，充分了解每个命令的影响后再在重要项目中使用。

需要查看更详细的文档，可以访问：https://docs.anthropic.com/en/docs/claude-code
