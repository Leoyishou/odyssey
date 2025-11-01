---
date created: 2024-07-13
date modified: 2025-10-28
uid: 92689d98-303f-4b22-a8a0-c67872330a2e
---
# Summary

[[python 虚拟环境venv]] 和 conda 都**不会**在项目间复用包
```Java
# venv
project1/.venv/  ← 有自己的完整包
project2/.venv/  ← 有自己的完整包
# 两个环境互相独立，不共享

# conda  
~/anaconda3/envs/project1/  ← 有自己的完整包
~/anaconda3/envs/project2/  ← 有自己的完整包
# 同样互相独立，不共享
```

真正的区别

| 维度             | venv       | conda                   |
| -------------- | ---------- | ----------------------- |
| **环境位置**       | 分散在各项目目录   | 集中在 `~/anaconda3/envs/` |
| **包来源**        | PyPI (pip) | Anaconda 仓库 + PyPI      |
| **依赖解析**       | 简单（可能冲突）| 强大（SAT solver）|
| **非 Python 包** | ❌ 不支持      | ✅ 支持（如 CUDA、系统库）|
|                |            |                         |
|                |            |                         |

用 conda 管"环境"，用 [[uv]] 装"Python 包"（快！）
```Java
conda create -n ml_project python=3.11
conda activate ml_project
conda install cudatoolkit  # conda 装系统级依赖

uv pip install torch torchvision pandas scikit-learn
```



# Cues

[[Anaconda]]

# Notes

## Channel

Channels就像不同的图书供应商，比如清华源、华为源、阿里源

## install 用 pip 还是 conda

大部分情况下，二者没有区别，怎么安装都可以work，不过，二者混着装各种package你真的会因为版本冲突而发疯！

[conda](https://zhida.zhihu.com/search?content_id=493927916&content_type=Answer&match_order=1&q=conda&zhida_source=entity)有严格的检查机制，它会保证你当前装的package安装好之后能work，但是，它只检查用conda安装过的package。例如，你新安装的package会依赖[numpy](https://zhida.zhihu.com/search?content_id=493927916&content_type=Answer&match_order=1&q=numpy&zhida_source=entity)，不过你已经安装numpy(e.g., 1.19.2)，但是用pip安装的，不好意思，它会认为你没安装，然后用conda再安装一个依赖版本的numpy(e.g., 1.18.5)。这个时候，两个numpy可能就打架了。并且你pip list与conda list显示的numpy版本可能不一致，结果不知道最后运行程序的时候调用了哪个版本的numpy。

这个时候，uninstall可能也卸载不干净，卸了一个还有一个，最后还可能一直list有却是一个空壳。这个时候，快刀斩乱麻的方法，找到[anaconda](https://zhida.zhihu.com/search?content_id=493927916&content_type=Answer&match_order=1&q=anaconda&zhida_source=entity)的lib/site-packages/numpy, 手动删掉它！

pip的一个好处是可以安装时既检查conda安装过package的也检查pip安装过的package。不过，它只负责要什么装什么，不负责能不能把装的一堆packages打通，可能装好不work:(。

一般原则，在新环境中，如果装多个packages，既用到conda，又用到pip，那就先conda 的都装好，再pip，如果能用一种装到底，就不要来回换着用。

血泪教训，end.
