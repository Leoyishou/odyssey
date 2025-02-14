---
aliases: [查看一下现有python的位置]
draw:
tags: []
title: 查看一下现有python的位置
date created: 2023-11-22
date modified: 2024-12-27
linter-yaml-title-alias: 查看一下现有python的位置
---

系统默认自带了一个 ```bash

## 查看一下现有python的位置

whereis python  
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7

## 在一个不冲突的位置创建安装目录

mkdir -p /usr/local/python3  
```3，我们有的应用需要用到 ```bash

## 查看一下现有python的位置

whereis python  
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7

## 在一个不冲突的位置创建安装目录

mkdir -p /usr/local/python3  
```4 来运行，那我们就来为系统安装一个 ```bash

## 查看一下现有python的位置

whereis python  
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7

## 在一个不冲突的位置创建安装目录

mkdir -p /usr/local/python3

```5 为后面的开发做准备。

## 下载压缩包

```bash
wget https://www.python.org/ftp/python/3.6.1/Python-3.6.1.tgz
```

Bash

### 创建目录

```bash
# 查看一下现有python的位置
whereis python
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7
# 在一个不冲突的位置创建安装目录
mkdir -p /usr/local/python3
```

Bash

### 解压并安装到指定目录

```bash
tar -zxvf Python-3.6.1.tgz
cd Python-3.6.1
./configure --prefix=/usr/local/python3
make && make install
```

错误代码 ModuleNotFoundError: No module named '_ctypes'  
该错误是因为本机缺少 libffi-devel 包，
只需安装此包即可 # 安装命令  
注意在安装完缺少的依赖包后，仍需重新运行对应所在的配置、编译和执行安装命令

```shell
yum install -y libffi-devel 
./configure --prefix=/usr/local/python3
make && make install
```

Bash

### 链接软连接

```bash
ln -s /usr/local/python3/bin/python3 /usr/bin/python3

ln -s /usr/local/python3/bin/pip3 /usr/bin/pip
```

Bash

### 查看版本

```bash
/usr/local/python3/bin/python3 -V
Python 3.6.1
```

### 添加 PATH 环境变量

```bash
vim ~/.bash_profile
# 在现有PATH上添加/usr/local/python3/bin
PATH=PATH:HOME/bin:/usr/local/python3/bin
# 保存退出后，使其生效
source ~/.bash_profile
# 已生效
python3 -V
Python 3.6.1
# 系统自带python版本
python -V
Python 2.7.5
```

Bash

这样系统就并存了 ```bash

## 查看一下现有python的位置

whereis python  
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7

## 在一个不冲突的位置创建安装目录

mkdir -p /usr/local/python3  
```6 和 ```bash

## 查看一下现有python的位置

whereis python  
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7

## 在一个不冲突的位置创建安装目录

mkdir -p /usr/local/python3

```7，也为后面的多应用独立环境的隔离做好了准备工作。

## 六、pip 配置工作

### 1. pip 介绍

> pip 是一个 Python 包管理器，用于安装、升级、卸载 Python 包（即模块或库）。它能够自动处理依赖关系，并从 Python Package Index (PyPI) 下载并安装包。简而言之，pip 是一个用于 Python 包管理的工具。

### 2. 查看当前 pip 版本

> 查看 pip 的版本

```bash
[root@jeven Python-3.11.3]# pip -V
```

### 3. 升级 pip 版本

> 升级 pip 版本

```bash
[root@jeven Python-3.11.3]# python3 -m pip install --upgrade pip
Looking in indexes: http://mirrors.aliyun.com/pypi/simple/
Requirement already satisfied: pip in /usr/local/python311/lib/python3.11/site-packages (22.3.1)
Collecting pip
  Downloading http://mirrors.aliyun.com/pypi/packages/08/e3/57d4c24a050aa0bcca46b2920bff40847db79535dc78141eb83581a52eb8/pip-23.1.2-py3-none-any.whl (2.1 MB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 2.1/2.1 MB 4.4 MB/s eta 0:00:00
Installing collected packages: pip
  Attempting uninstall: pip
    Found existing installation: pip 22.3.1
    Uninstalling pip-22.3.1:
      Successfully uninstalled pip-22.3.1
Successfully installed pip-23.1.2
WARNING: Running pip as the 'root' user can result in broken permissions and conflicting behaviour with the system package manager. It is recommended to use a virtual environment instead: https://pip.pypa.io/warnings/venv
```

复制

### 4. 修改 pip.conf 配置镜像

> 修改 pip.conf 文件

```bash
mkdir -p ~/.pip
vim ~/.pip/pip.conf
```

复制

```bash
# 查看一下现有python的位置
whereis python
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7
# 在一个不冲突的位置创建安装目录
mkdir -p /usr/local/python3
```0

复制

### 5. 查看已安装的第三⽅模块

> 查看已安装的第三⽅模块

```bash
# 查看一下现有python的位置
whereis python
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7
# 在一个不冲突的位置创建安装目录
mkdir -p /usr/local/python3
```1

## 二、部署 python 应用

### 3.1 将本地开发环境的依赖项目生成清单文件

。在本地的开发环境中，env 下执行：

```bash
# 查看一下现有python的位置
whereis python
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7
# 在一个不冲突的位置创建安装目录
mkdir -p /usr/local/python3
```2

### 3.2 将 Python 项目上传到服务器

### 3.3 为项目创建虚拟环境

1. 新建虚拟环境  
	```bash
# 查看一下现有python的位置
whereis python
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7
# 在一个不冲突的位置创建安装目录
mkdir -p /usr/local/python3
```8
2. 切换到目录下，启用虛拟环境  
	```bash
# 查看一下现有python的位置
whereis python
python: /usr/bin/python /usr/bin/python2.7 /usr/bin/python2.7-config /usr/lib/python2.7 /usr/lib64/python2.7 /etc/python /usr/include/python2.7
# 在一个不冲突的位置创建安装目录
mkdir -p /usr/local/python3
```9  
	```bash
tar -zxvf Python-3.6.1.tgz
cd Python-3.6.1
./configure --prefix=/usr/local/python3
make && make install
```0
3. 安装依赖清单里的库  
	```bash
tar -zxvf Python-3.6.1.tgz
cd Python-3.6.1
./configure --prefix=/usr/local/python3
make && make install
```1  
	```bash
tar -zxvf Python-3.6.1.tgz
cd Python-3.6.1
./configure --prefix=/usr/local/python3
make && make install
```2


- 在指定环境下完成任务后关闭虛拟环境

这样一来在运行 python 就是全局的 python 环境，在虛拟环境下执行命令：

cd env-dir/bin

source activate #激活进入虚拟环境

deactivate #退出虚拟环境

## 参考

https://blog.csdn.net/smilehappiness/article/details/117337943
