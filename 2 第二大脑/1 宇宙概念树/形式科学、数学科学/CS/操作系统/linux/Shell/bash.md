---
comment_id: 4cfca4f9
date created: 2024-07-11
date modified: 2025-02-06
draw: null
title: bash
---
## Bash 命令速查手册

### 简介

Bash是一种Shell，作为Linux系统的命令行接口，用于运行Linux命令。

### 常用文件操作命令

#### ls 命令 (list)

- `ls -l`: 显示详细列表（通常别名为`ll`）
- `ls -ltr`: 按时间逆序显示文件列表
  - `-l`: 显示详细信息
  - `-t`: 按修改时间排序
  - `-r`: 逆序显示

#### 日志查看命令

1. **head** - 查看文件开头

   ```bash
   head -100 file.log  # 查看前100行
   ```

   参数：

   - `-q`: 隐藏文件名
   - `-v`: 显示文件名
   - `-c<数目>`: 显示字节数
   - `-n<行数>`: 显示行数

2. **tail** - 查看文件结尾

   ```bash
   tail -f file.log    # 实时追踪日志
   ```

3. **less** - 分页查看

   ```bash
   less +G file.log    # 从末尾开始查看
   ```

   快捷键：

   - `b`: 向上翻页
   - `空格`: 向下翻页

4. **cat** - 查看完整文件

   ```bash
   cat -n file.log     # 显示行号
   ```

   主要参数：

   - `-n`: 显示行号
   - `-b`: 非空行显示行号
   - `-s`: 压缩空行
   - `-v`: 显示特殊字符

### grep 命令

用于文本搜索，支持正则表达式。

#### 基本语法

```bash
grep [options] pattern [file...]
```

#### 常用选项

- `-i`: 忽略大小写
- `-n`: 显示行号
- `-r`: 递归搜索
- `-v`: 显示不匹配的行
- `-A n`: 显示匹配行及后n行
- `-B n`: 显示匹配行及前n行
- `-C n`: 显示匹配行及前后n行

### Linux日志文件位置

| 日志文件 | 用途 |
|---------|------|
| /var/log/message | 系统信息和错误日志 |
| /var/log/secure | 安全相关日志 |
| /var/log/maillog | 邮件相关日志 |
| /var/log/cron | 定时任务日志 |
| /var/log/boot.log | 系统启动日志 |
| /var/log/wtmp | 用户登录记录 |

#最佳实践

1. 使用`ls -ltr`查看最近修改的文件
2. 使用`tail -f`实时监控日志
3. 使用`less +G`从文件末尾开始查看大文件
4. 结合`grep`进行日志分析

### 常见问题解决

1. 如果`ll`命令不可用，可以设置别名：

   ```bash
   alias ll='ls -l'
   ```

2. 查看大文件时优先使用`less`而不是`cat`
3. 监控日志时，`tail -f`比反复使用`cat`更高效
