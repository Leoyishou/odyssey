---
draw:
title: conda
date created: 2024-07-13
date modified: 2025-03-13
---

请问大神们，pip install 和 conda install 有什么区别吗？- ZERO-XJ 的回答 - 知乎  
https://www.zhihu.com/question/395145313/answer/2551141843

## 常用操作

```Java
conda create -n 环境名 python=3.10


列出所有环境
conda info --envs

激活某一环境
conda activate viva-English
```

## 导出 pom

`conda env export > environment.yml`

## pipreqs

pipreqs 是一个非常有用的 Python 包，能分析项目里所有 Python 文件中的 import，生成一个只包含必要依赖的 requirements.txt 文件

## Anaconda 是啥？

conda 类似于 java 的 maven，Anaconda 相当于 maven里的仓库里预装了一些包，可以进行可视化UI,比较用户友好  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F19%2F22-10-47-3d47aab1111bbce386c7b6e00f87aebb-202409192210463-918d48.png)

## Channel

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
