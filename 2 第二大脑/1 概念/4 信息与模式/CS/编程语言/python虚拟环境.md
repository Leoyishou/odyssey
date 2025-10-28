---
date created: 2025-03-23
date modified: 2025-10-20
uid: c3a087b7-bf8a-4fb1-97a7-510d1b3c1e0e
---
# Summary

1. 当你执行 `python3 -m venv.venv` 命令时，会发生以下事情：
	1. Python 在你当前所在的目录下，创建一个名为 `.venv` 的新文件夹。
	2. 在这个 `.venv` 文件夹内部，它会创建一个**独立的、隔离的**Python 运行环境。这包括：
	    - 一份 Python 解释器的副本（或链接）。
	    - 一个独立的 `site-packages` 目录，用来存放**只属于这个项目**的第三方库。
	    - 一些管理脚本（比如 `activate`），用来激活和退出这个环境。
2. `source.venv/bin/activate` 
		1. `.venv/bin/activate` 这个文件本质上就是一个 shell 脚本
		2. `source` 命令的作用是：**在当前的 shell 会话中读取并执行指定文件中的命令。**
3. `rm -rf.venv`

```Java

新建：    python3 -m venv .venv
进入：    source .venv/bin/activate
退出：    deactivate
删除：    rm -rf.venv
```


```shell
# 基础依赖
torch>=2.1.0
torchvision>=0.16.0
torchaudio>=2.1.0

# Hugging Face
transformers>=4.40.0
datasets>=2.19.0
evaluate>=0.4.0

# 常用工具
scikit-learn>=1.3.0
numpy>=1.26.0
pandas>=2.1.0

# 训练相关
accelerate>=0.29.0
tqdm>=4.66.0
wandb>=0.15.0  # 添加 wandb 用于实验跟踪

# 可选（日志、调试）
tensorboard>=2.15.0

```



# Notes
这属于 **Python虚拟环境（Virtual Environment）的创建和激活操作**。

---

具体解释如下：

| 命令                         | 含义              | 说明                    |
| -------------------------- | --------------- | --------------------- |
| `python -m venv venv`      | 创建名为`venv`的虚拟环境 | 通过venv模块创建独立的Python环境 |
| `source venv/bin/activate` | 激活虚拟环境          | 激活后，安装包会安装到这个环境中      |
| `deactivate`               | 退出虚拟环境          | 完成工作后退出当前虚拟环境         |

---

**用途：**

- **隔离不同项目的Python环境**
- 避免不同项目依赖版本冲突
- 更好地进行项目管理和部署
