---
date created: 2024-07-13
date modified: 2025-07-10
uid: 92689d98-303f-4b22-a8a0-c67872330a2e
---
## Conda 常用命令 Cheatsheet

### 环境管理

#### 创建和激活环境

```bash
# 创建新环境
conda create --name myenv python=3.9

# 创建环境并安装包
conda create --name myenv python=3.9 numpy pandas

# 激活环境
conda activate myenv

# 退出当前环境
conda deactivate
```

#### 环境查看和删除

```bash
# 列出所有环境
conda env list
# 或
conda info --envs

# 删除环境
conda env remove --name myenv

# 复制环境
conda create --name newenv --clone oldenv
```

### 包管理

#### 安装包

```bash
# 安装单个包
conda install numpy

# 安装特定版本
conda install numpy=1.20.0

# 安装多个包
conda install numpy pandas matplotlib

# 从特定通道安装
conda install -c conda-forge package_name
```

#### 更新和删除包

```bash
# 更新单个包
conda update numpy

# 更新所有包
conda update --all

# 删除包
conda remove numpy

# 清理未使用的包和缓存
conda clean --all
```

#### 查看包信息

```bash
# 列出已安装的包
conda list

# 查找可用的包
conda search package_name

# 查看特定包的信息
conda info package_name
```

### 通道管理

```bash
# 添加通道
conda config --add channels conda-forge

# 列出通道
conda config --get channels

# 设置通道优先级
conda config --set channel_priority strict
```

### 其他实用命令

```bash
# 更新 conda 自身
conda update conda

# 获取 conda 信息
conda info

# 导出环境
conda env export > environment.yml

# 从文件创建环境
conda env create -f environment.yml

# 查看 conda 版本
conda --version
```

请问大神们，pip install 和 conda install 有什么区别吗？- ZERO-XJ 的回答 - 知乎  
https://www.zhihu.com/question/395145313/answer/2551141843

### 常用操作

```Java
conda create -n 环境名 python=3.10


列出所有环境
conda info --envs

激活某一环境
conda activate viva-English
```

### 导出 pom

`conda env export > environment.yml`

### pipreqs

pipreqs 是一个非常有用的 Python 包，能分析项目里所有 Python 文件中的 import，生成一个只包含必要依赖的 requirements.txt 文件

### Anaconda 是啥？

conda 类似于 java 的 maven，Anaconda 相当于 maven里的仓库里预装了一些包，可以进行可视化UI,比较用户友好  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F19%2F22-10-47-3d47aab1111bbce386c7b6e00f87aebb-202409192210463-918d48.png)

### Channel

如果把Conda比作一个图书馆系统:

- Channels就像不同的图书供应商
- 包就是书籍
- 安装包就像从特定供应商订购图书

在Conda中,channel（通道）实际上就是一个包含一系列conda包的存储库或源。

1. **包的来源**: Channels是Conda查找和下载包的地方。
2. **版本控制**: 不同的channel可能包含同一个包的不同版本。
3. **特定领域**: 某些channel可能专注于特定领域的包(如生物信息学)。

常用Channels

- **defaults**: Conda的默认channel,由Anaconda公司维护。
- **conda-forge**: 社区驱动的channel,包含大量的包。
- **bioconda**: 专注于生物信息学的channel。

使用方式

1. **安装包时指定channel**:  
    `conda install -c conda-forge package_name`
2. **添加channel到配置**:  
    `conda config --add channels conda-forge`
3. Conda会按照channel的优先级顺序搜索包。可以通过`conda config --show channels`查看当前的channel优先级。
