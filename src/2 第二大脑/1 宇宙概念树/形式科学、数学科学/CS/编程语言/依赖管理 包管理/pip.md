---
comment_id: edb1f503
date created: 2025-03-23
date modified: 2025-04-16
draw: null
title: pip
---
以下是pip常用命令的markdown表格版：

| 功能                    | 命令示例                                                                       | 备注                                 |
| --------------------- | -------------------------------------------------------------------------- | ---------------------------------- |
| 安装包                   | `pip install package-name`                                                 | 默认安装最新版                            |
| 安装特定版本的包              | `pip install package-name==version`                                        | 例: `requests==2.28.1`              |
| 升级包                   | `pip install --upgrade package-name`                                       |                                    |
| 卸载包                   | `pip uninstall package-name`                                               |                                    |
| 显示已安装的所有包             | `pip list`                                                                 |                                    |
| 搜索包（pip 20.3 后已移除）| ~~`pip search package-name`~~                                              | 推荐访问 [pypi.org](https://pypi.org/) |
| 查看已安装包的信息             | `pip show package-name`                                                    |                                    |
| 导出环境中的所有依赖包           | `pip freeze > requirements.txt`                                            | 常用于环境复现                            |
| 根据 requirements 安装依赖包 | `pip install -r requirements.txt`                                          |                                    |
| 升级pip自身               | `pip install --upgrade pip`                                                | 推荐经常升级                             |
| 临时使用其他镜像源安装           | `pip install package -i https://pypi.tuna.tsinghua.edu.cn/simple`          | 国内镜像源示例                            |
| 全局修改pip默认镜像源          | `pip config set global.index-url https://pypi.tuna.tsinghua.edu.cn/simple` | 推荐国内用户使用                           |
| 查看pip当前配置             | `pip config list`                                                          |                                    |
| 检查已安装包是否有新版本          | `pip list --outdated`                                                      |                                    |
| 下载包但不安装               | `pip download package-name`                                                | 下载到当前目录                            |
| 安装本地whl文件             | `pip install./package-name.whl`                                           | 本地离线安装                             |
| 从Git仓库直接安装            | `pip install git+https://github.com/user/repo.git`                         |                                    |

、

很好问题！🎯

---

## ✅ `llama-index[embed]` 是什么？

这是 Python 的一种 **可选依赖（extras）安装语法**，意思是：

> 安装 `llama-index` 包的同时，**额外安装它的 `embed` 功能相关的依赖项**。

这是一种标准的 Python 包机制，允许用户按需选择是否安装某些额外功能所需要的库。

---

## 💡 举个例子

如果你运行下面这个命令：

```bash
pip install llama-index[embed]
```

它的效果相当于你做了两件事：

1. 安装 `llama-index` 主包；
    
2. 同时安装 `embed` 这个子功能所需要的额外依赖，例如：
    
    - `transformers`
        
    - `sentence-transformers`
        
    - `torch`（有时候）
        
    - 还有其他用于 embedding 的模块
        

---

## 🧠 为什么这样设计？

很多现代 Python 包都这样设计，是为了让你按需安装，不浪费资源。例如：

- `llama-index[weaviate]`：安装与 Weaviate 向量数据库的集成支持；
    
- `llama-index[chromadb]`：安装 Chroma 支持；
    
- `llama-index[llms]`：安装 LLM 调用支持；
    
- `llama-index[embed]`：安装嵌入模型支持（你现在就用这个）；
    

你也可以一次性安装全部功能（不推荐除非确实要）：

```bash
pip install llama-index[all]
```

---

## 🔧 补充建议

如果你只是想用 HuggingFace embedding（比如 `bge-large-zh-v1.5`），推荐就用这个：

```bash
pip install llama-index[embed]
```

如果你还要用 Milvus、Weaviate、Chroma 等，可以按需添加，比如：

```bash
pip install llama-index[chromadb,weaviate,embed]
```

---

要我给你列一张表格，总结 `llama-index` 常见的 `[extra]` 模块及用途吗？
