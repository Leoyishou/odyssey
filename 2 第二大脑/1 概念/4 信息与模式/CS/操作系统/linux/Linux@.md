---
date created: 2024-11-25
date modified: 2025-10-28
uid: b87d3d45-2f74-4c71-971e-169b6929a9ae
---
# Linux 系统参考指南

> 一切皆文件

```Java


# docker、nginx、ssh 配置放哪里？
etc  包含系统配置文件，这些文件通常用于配置系统服务和应用程序

# conda、anaconda3、cuda 放哪里？
opt 安装额外的软件包，通常不是由系统的包管理器安装，而是由用户手动安装 38

usr 用户相关的应用程序和文件的存放地，如用户安装的软件、库文件等


# ===== /usr 代表 Unix 系统资源 =====
/usr/bin/python3                   # Python解释器
/usr/bin/vim                       # Vim编辑器
/usr/bin/git                       # Git版本控制


# ===== /etc 配置文件 比如 nginx、ssh=====
/etc/                              # 系统配置根目录
/etc/nginx/                        # Nginx配置目录
/etc/nginx/nginx.conf              # Nginx主配置文件


/etc/ssh/                          # SSH配置目录
/etc/ssh/sshd_config               # SSH服务端配置
/etc/ssh/ssh_config                # SSH客户端配置


# ===== /opt、optional  第三方软件，比如 cuda、anaconda =====
/opt/                              # 第三方软件根目录
/opt/cuda/                         # CUDA（如果装在这里）

/opt/anaconda3/                    # Anaconda
/opt/anaconda3/bin/                # Anaconda命令



# ===== /home 用户目录 =====
/home/                             # 用户主目录根
/home/yishou.liu/                  # 你的用户目录
/home/yishou.liu/.bashrc           # 用户bash配置
/home/yishou.liu/.bash_profile     # 登录时执行
/home/yishou.liu/.profile          # 登录配置
/home/yishou.liu/.config/          # 用户应用配置
/home/yishou.liu/.config/pip/      # 用户pip配置
/home/yishou.liu/.config/pip/pip.conf  # pip配置文件
/home/yishou.liu/.local/           # 用户级安装
/home/yishou.liu/.local/bin/       # 用户命令
/home/yishou.liu/.local/lib/       # 用户库

/home/yishou.liu/qwen-fast-test/   # 你的项目目录
/home/yishou.liu/qwen-fast-test/venv/  # 虚拟环境
/home/yishou.liu/qwen-fast-test/venv/bin/python  # 虚拟环境Python
/home/yishou.liu/qwen-fast-test/venv/lib/python3.10/site-packages/  # 包安装位置




```
![image.png|600](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F08%2F06%2F14-33-06-c19f3bb70c53a9042b88efc580da2a34-202508061433716-84e3dd.png)

# 文件

## 常用目录解释

| 目录   | 含义                    | 描述                                 |
| ---- | --------------------- | ---------------------------------- |
| /etc | et cetera源自拉丁语，表示"等等" | 存放系统和应用程序的配置文件，最初用于存放不属于其他目录的文件    |
| /usr | Unix System Resources | 代表 Unix 系统资源                       |
| /var | variable              | 代表可变数据，用于存放可变数据，如数据库文件、软件包数据、用户数据等 |

`/var/lib/` 存储系统运行时的持久性数据，

## 二、系统目录结构

### /opt 目录

- 全称：optional（可选的）
- 用途：存放可选的软件包或第三方应用程序

## 三、实用工具推荐

### 1. ag (The Silver Searcher)

一个比 grep 更快的代码搜索工具，具有以下特点：

- 搜索速度快
- 默认递归搜索
- 自动忽略版本控制系统文件
- 彩色输出
- 支持文件类型过滤
- 支持正则表达式
- 支持多核加速

### 2. 常用工具清单

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



**wget 的“本质”**
- **定位**：它是 GNU 项目中的一款 _命令行网络下载器_，主要功能就是向指定 URL 发起 **HTTP/HTTPS/FTP** 请求并把远端资源原封不动保存到本地文件系统。
- **特性**：支持断点续传、限速、递归抓取整站、后台运行、代理、认证等——所以常被用来批量或自动化下载。
