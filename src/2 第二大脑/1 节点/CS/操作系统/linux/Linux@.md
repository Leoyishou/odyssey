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
| `cp source.txt destination.txt` | Copy file |
| `cp -r /source/directory /dest/directory` | Copy directory recursively |
| `mv old_name.txt new_name.txt` | Rename file |
| `mv file.txt /path/to/destination` | Move file to another location |
| `rm file.txt` | Removes a file |
| `rm -r directory/` | Removes a directory and its contents |
| `rm -rf directory/` | Force removes a directory, even if non-empty |
| `cat file.txt` | Outputs the entire file |
| `less file.txt` | Scroll through file contents (with up/down keys) |
| `head file.txt` | View the first 10 lines of a file |
| `tail file.txt` | View the last 10 lines of a file |
| `tail -f logfile.log` | Continuously output the end of a file (useful for logs) |
| `grep "search_term" file.txt` | Search for a term in a file |
| `grep -r "search_term" /path` | Recursively search within files in a directory |
| `grep -i "search_term" file.txt` | Case-insensitive search |
| `uname -a` | Kernel version and system information |
| `df -h` | Disk usage of file systems |
| `free -h` | Displays memory usage |
| `top` | Real-time system resource monitoring |
| `ps aux` | Lists all running processes |
| `kill[pid]` | Terminate a process by its process ID (PID) |
| `kill -9[pid]` | Force kill a process |
| `killall process_name` | Kill all processes with a specific name |
| `chmod 755 file.txt` | Assigns read, write, and execute permissions (Owner: rwx, Group: r-x, Others: r-x) |
| `chmod u+x script.sh` | Gives the owner execute permissions |
| `chmod -R 777 /directory` | Grants all users full permissions on a directory recursively |
| `chown user:group file.txt` | Change owner and group |
| `chown -R user:group /dir` | Recursively change ownership for all files in a directory |
| `find /path -name "file.txt"` | Search by file name |
| `find /path -type f -size +100M` | Find files larger than 100MB |
| `find /path -mtime -1` | Find files modified in the last 24 hours |
| `tar -cvf archive.tar /path` | Create a tar archive of a directory |
| `tar -xvf archive.tar` | Extract a tar archive |
| `tar -czvf archive.tar.gz /path` | Create a compressed gzip tar archive |
| `tar -xzvf archive.tar.gz` | Extract a compressed gzip tar archive |
| `sudo command` | Run a command as superuser/root |
| `sudo su` | Switch to the root user |

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

`ag` 是一个命令行工具，全名为 "The Silver Searcher"。它是一个代码搜索工具，类似于 `grep`，但速度更快、功能更强大。以下是 `ag` 的主要特点和用途：

1. 快速搜索：`ag` 比传统的 `grep` 快很多，尤其是在搜索大型代码库时。
2. 默认递归搜索：无需额外参数，ag 会自动递归搜索子目录。
3. 智能忽略：自动忽略版本控制系统的隐藏文件（如.git 目录）和二进制文件。
4. 彩色输出：搜索结果默认带有颜色高亮，使其更易读。
5. 文件类型搜索：可以限制只搜索特定类型的文件。
6. 正则表达式支持：支持强大的正则表达式搜索。
7. 多核利用：可以利用多核处理器加速搜索。
