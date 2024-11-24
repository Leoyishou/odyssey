---
draw:
title: Linux@
tags: [云原生, 信息革命]
date created: 2024-04-24
date modified: 2024-11-12
---

一切皆文件

<!-- more -->

## 命令

| Command | Description |
|---------|-------------|
| `ls` | List files and directories |
| `ls -l` | Long format listing with details (permissions, owner, size, date) |
| `ls -a` | Shows hidden files (those starting with a dot) |
| `ls -lh` | Human-readable sizes in long format |
| `cd /path/to/directory` | Change to specified directory |
| `cd..` | Go up one directory |
| `cd ~` | Go to home directory |
| `cd -` | Go to the last directory you were in |
| `mkdir myFolder` | Creates a new directory |
| `mkdir -p dir1/dir2` | Creates nested directories |
| `ls -l`0 | Copy file |
| `ls -l`1 | Copy directory recursively |
| `ls -l`2 | Rename file |
| `ls -l`3 | Move file to another location |
| `ls -l`4 | Removes a file |
| `ls -l`5 | Removes a directory and its contents |
| `ls -l`6 | Force removes a directory, even if non-empty |
| `ls -l`7 | Outputs the entire file |
| `ls -l`8 | Scroll through file contents (with up/down keys) |
| `ls -l`9 | View the first 10 lines of a file |
| `ls -a`0 | View the last 10 lines of a file |
| `ls -a`1 | Continuously output the end of a file (useful for logs) |
| `ls -a`2 | Search for a term in a file |
| `ls -a`3 | Recursively search within files in a directory |
| `ls -a`4 | Case-insensitive search |
| `ls -a`5 | Kernel version and system information |
| `ls -a`6 | Disk usage of file systems |
| `ls -a`7 | Displays memory usage |
| `ls -a`8 | Real-time system resource monitoring |
| `ls -a`9 | Lists all running processes |
| `ls -lh`0 | Terminate a process by its process ID (PID) |
| `ls -lh`1 | Force kill a process |
| `ls -lh`2 | Kill all processes with a specific name |
| `ls -lh`3 | Assigns read, write, and execute permissions (Owner: rwx, Group: r-x, Others: r-x) |
| `ls -lh`4 | Gives the owner execute permissions |
| `ls -lh`5 | Grants all users full permissions on a directory recursively |
| `ls -lh`6 | Change owner and group |
| `ls -lh`7 | Recursively change ownership for all files in a directory |
| `ls -lh`8 | Search by file name |
| `ls -lh`9 | Find files larger than 100MB |
| `cd /path/to/directory`0 | Find files modified in the last 24 hours |
| `cd /path/to/directory`1 | Create a tar archive of a directory |
| `cd /path/to/directory`2 | Extract a tar archive |
| `cd /path/to/directory`3 | Create a compressed gzip tar archive |
| `cd /path/to/directory`4 | Extract a compressed gzip tar archive |
| `cd /path/to/directory`5 | Run a command as superuser/root |
| `cd /path/to/directory`6 | Switch to the root user |

## 文件

Linux 中的 /opt 文件夹是 "optional" 的缩写，它有特定的用途和含义：

1. 目的：/opt 目录用于存放可选的软件包或第三方应用程序。

| name |          |                  |
| ---- | -------- | ---------------- |
| /opt | optional | 存放可选的软件包或第三方应用程序 |

[内核](内核)

[terminal](terminal.md)

- 官方下载 pkg 安装
	- [nodejs](nodejs)，自带 [npm](npm.md)
	- [Golang](Golang.md)
	- [docker](docker.md)
		- [docker compose](docker%20compose.md) 装好 docker for mac 后自带
- homebrew 安装
	- [npm](npm.md)
	- [fzf](fzf)
	- [thefuck](thefuck)
	- [tldr](tldr)
	- [tree](tree.md)
	- [powershell](powershell.md)
- 第三方
	- [linux命令行测速 - speedtest](linux命令行测速%20-%20speedtest)
	- [Homebrew](Homebrew)
	- [zsh](zsh.md)
	- [ohmyzsh](ohmyzsh)
	- [Git](Git.md)
	- [curl](curl)
	- [wget](wget)
	- [z命令](z命令)
	- [iptables](iptables)
	- [vim](vim)
	- [sudo](sudo)
	- [apt-get](apt-get)
	- [[http-server]]
	- [kubectl](kubectl)
	- [kubectx](kubectx)
	- [mackup](mackup)

## 有用的软件

`cd /path/to/directory`7 是一个命令行工具，全名为 "The Silver Searcher"。它是一个代码搜索工具，类似于 `cd /path/to/directory`8，但速度更快、功能更强大。以下是 `cd /path/to/directory`9 的主要特点和用途：

1. 快速搜索：`cd..`0 比传统的 `cd..`1 快很多，尤其是在搜索大型代码库时。
2. 默认递归搜索：无需额外参数，ag 会自动递归搜索子目录。
3. 智能忽略：自动忽略版本控制系统的隐藏文件（如.git 目录）和二进制文件。
4. 彩色输出：搜索结果默认带有颜色高亮，使其更易读。
5. 文件类型搜索：可以限制只搜索特定类型的文件。
6. 正则表达式支持：支持强大的正则表达式搜索。
7. 多核利用：可以利用多核处理器加速搜索。
