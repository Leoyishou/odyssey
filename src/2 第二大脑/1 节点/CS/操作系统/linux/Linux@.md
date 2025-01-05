---
draw:
tags: []
title: Linux@
date created: 2024-11-25
date modified: 2024-12-28
---

## Linux 系统参考指南

> 一切皆文件

## 文件

### 常用目录解释

| 目录   | 含义                    | 描述                                 |
| ---- | --------------------- | ---------------------------------- |
| /etc | et cetera源自拉丁语，表示"等等" | 存放系统和应用程序的配置文件，最初用于存放不属于其他目录的文件    |
| /usr | Unix System Resources | 代表 Unix 系统资源                       |
| /var | variable              | 代表可变数据，用于存放可变数据，如数据库文件、软件包数据、用户数据等 |

`/var/lib/` 存储系统运行时的持久性数据，

### 一、常用命令速查表

#### 1. 文件和目录操作

| 命令 | 说明 |
|------|------|
| `ls` | 列出文件和目录 |
| `ls -l` | 长格式列表（显示权限、所有者、大小、日期）|
| `ls -a` | 显示隐藏文件（以点开头的文件）|
| `ls -lh` | 以人类可读格式显示文件大小 |
| `cd /path/to/directory` | 切换到指定目录 |
| `cd..` | 返回上级目录 |
| `cd ~` | 进入家目录 |
| `cd -` | 返回上次所在目录 |
| `mkdir myFolder` | 创建新目录 |
| `mkdir -p dir1/dir2` | 创建嵌套目录 |

#### 2. 文件操作

| 命令 | 说明 |
|------|------|
| `cp source.txt destination.txt` | 复制文件 |
| `cp -r /source/directory /dest/directory` | 递归复制目录 |
| `mv old_name.txt new_name.txt` | 重命名文件 |
| `mv file.txt /path/to/destination` | 移动文件 |
| `rm file.txt` | 删除文件 |
| `rm -r directory/` | 删除目录及其内容 |
| `rm -rf directory/` | 强制删除目录及其内容 |

#### 3. 文件内容操作

| 命令 | 说明 |
|------|------|
| `cat file.txt` | 输出整个文件内容 |
| `less file.txt` | 分页查看文件内容 |
| `head file.txt` | 查看文件前10行 |
| `tail file.txt` | 查看文件后10行 |
| `tail -f logfile.log` | 实时查看文件末尾（常用于日志）|
| `grep "search_term" file.txt` | 在文件中搜索内容 |
| `grep -r "search_term" /path` | 递归搜索目录中的文件内容 |
| `grep -i "search_term" file.txt` | 不区分大小写搜索 |

#### 4. 系统信息和进程管理

| 命令 | 说明 |
|------|------|
| `uname -a` | 显示内核版本和系统信息 |
| `df -h` | 显示文件系统使用情况 |
| `free -h` | 显示内存使用情况 |
| `top` | 实时系统资源监控 |
| `ps aux` | 列出所有运行进程 |
| `kill[pid]` | 终止指定进程 |
| `kill -9[pid]` | 强制终止进程 |
| `killall process_name` | 终止所有指定名称的进程 |

#### 5. 权限管理

| 命令 | 说明 |
|------|------|
| `chmod 755 file.txt` | 设置权限（所有者：rwx，组：r-x，其他：r-x）|
| `chmod u+x script.sh` | 给所有者添加执行权限 |
| `chmod -R 777 /directory` | 递归设置目录权限 |
| `chown user:group file.txt` | 更改文件所有者和组 |
| `chown -R user:group /dir` | 递归更改目录所有权 |

#### 6. 文件查找和压缩

| 命令 | 说明 |
|------|------|
| `find /path -name "file.txt"` | 按文件名搜索 |
| `find /path -type f -size +100M` | 查找大于100MB的文件 |
| `find /path -mtime -1` | 查找24小时内修改的文件 |
| `tar -cvf archive.tar /path` | 创建tar归档 |
| `tar -xvf archive.tar` | 解压tar归档 |
| `tar -czvf archive.tar.gz /path` | 创建压缩的tar归档 |
| `tar -xzvf archive.tar.gz` | 解压压缩的tar归档 |

### 二、系统目录结构

#### /opt 目录

- 全称：optional（可选的）
- 用途：存放可选的软件包或第三方应用程序

### 三、实用工具推荐

#### 1. ag (The Silver Searcher)

一个比 grep 更快的代码搜索工具，具有以下特点：

- 搜索速度快
- 默认递归搜索
- 自动忽略版本控制系统文件
- 彩色输出
- 支持文件类型过滤
- 支持正则表达式
- 支持多核加速

#### 2. 常用工具清单

1. 官方安装包：
	- nodejs（自带 npm）
	- Golang
	- docker（包含 docker compose）

2. homebrew 安装：
	- npm
	- fzf
	- thefuck
	- tldr
	- tree
	- powershell

3. 第三方工具：
	- speedtest（命令行测速）
	- Homebrew
	- zsh
	- ohmyzsh
	- Git
	- curl
	- wget
	- iptables
	- vim
	- kubectl
	- kubectx
	- mackup
