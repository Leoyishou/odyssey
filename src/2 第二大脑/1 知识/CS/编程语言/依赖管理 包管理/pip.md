---
draw:
title: pip
date created: 2025-03-23
date modified: 2025-03-23
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
