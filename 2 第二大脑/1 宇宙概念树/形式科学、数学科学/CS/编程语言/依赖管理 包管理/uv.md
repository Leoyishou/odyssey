---
date created: 2025-03-24
date modified: 2025-07-10
uid: 0fc91684-868b-49f2-9d2e-c4008eef39b5
---
```Java
uv venv taurus --python 3.12
source taurus/bin/activate

uv pip install -r requirements.txt

```

`uv` 是由 Astral 公司开发的一款高性能 Python 包管理工具，旨在替代传统的 `pip`、`pip-tools` 和 `virtualenv` 等工具。它使用 Rust 编写，具有以下主要优势：

1. **极快的速度**：得益于 Rust 的高性能，`uv` 在依赖解析和包安装方面表现出色。在无缓存的情况下，`uv` 比 `pip` 和 `pip-tools` 快 8-10 倍；在有缓存的情况下，速度提升可达 80-115 倍。[^1]
2. **功能全面**：`uv` 提供了一站式的包管理体验，包括安装 Python 包、生成和管理锁文件、创建虚拟环境等功能。它支持可编辑安装、Git 依赖项、URL 依赖项、本地依赖项、约束文件、源码分发、自定义索引等，设计上与现有工具无缝兼容。[^2]
3. **轻量且独立**：`uv` 作为单一的静态二进制文件发布，无需预装 Python 环境即可运行，避免了在多个 Python 版本之间选择安装程序的困扰。[^3]
4. **全局缓存**：`uv` 使用全局模块缓存，避免重复下载和构建依赖项，并在支持的文件系统上利用 Copy-on-Write 和硬链接，最小化磁盘空间使用。[^4]
5. **无缝兼容**：`uv` 支持传统的 `requirements.txt` 和 `pip` 命令，可配合 `poetry`、`pdm` 等现代工具工作，生成与 `pip-compile` 兼容的锁文件。[^5]

通过这些特点，`uv` 为 Python 开发者提供了高效、可靠且易用的包管理解决方案，提升了开发体验和工作效率。

---

下面这张 **uv 常用操作 Cheat Sheet** 选取了最常用、最容易和 conda / pip 搞混的命令，按“装‑建‑管‑跑‑辅助”五大类归纳。所有示例均可直接在终端复制粘贴。

---

## 1 安装 / 升级

| 场景                | 命令                                                         | 说明                                                                                         |
| ----------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| 一键安装（macOS/Linux）| `bash\ncurl -LsSf https://astral.sh/uv/install.sh \| sh\n` | 官方独立安装脚本，会把 `uv` 二进制放到 `~/.local/bin`（或 Windows 的 `%LOCALAPPDATA%\\uv`）citeturn7search1 |
| pipx 安装           | `pipx install uv`                                          | 如果你已经在用 pipx 管理 CLI                                                                        |
| 升级到最新             | `uv self update`                                           | 自更新，可加 `--version 0.6.14` 锁定版本citeturn3view0                                            |
| 查看版本              | `uv version`                                               |                                                                                            |

---

## 2 虚拟环境 & Python 版本

|场景|命令|备注|
|---|---|---|
|在当前目录建 `.venv`|`uv venv`|不用再手动 source 激活；uv 会自动发现并使用 `.venv`citeturn3view0|
|指定路径 / 名称|`uv venv venv38`||
|选择 Python 版本|`uv venv --python 3.11`|若本机无该版本，uv 会自动下载 python‑build‑standalone 发行版citeturn7search0|
|列出已装 Python|`uv python list`||
|安装新版本 Python|`uv python install 3.12`|多版本并存，互不干扰citeturn7search0|

---

## 3 项目级工作流（“像 Cargo 一样”）

| 步骤                  | 命令                                | 相当于                                                                 |
| ------------------- | --------------------------------- | ------------------------------------------------------------------- |
| 初始化项目               | `uv init`                         | `poetry init` / `pipenv --three`                                    |
| 添加依赖                | `uv add numpy pandas`             | `pip install` + 写入 `pyproject.toml` + 更新 `uv.lock`citeturn5view0 |
| 移除依赖                | `uv remove pandas`                |                                                                     |
| 同步环境                | `uv sync`                         | 根据 `uv.lock` 安装 / 升级 / 移除                                           |
| 只刷新锁文件              | `uv lock`                         | 不触碰环境                                                               |
| 导出 requirements.txt | `uv export --format requirements` |                                                                     |
| 查看依赖树               | `uv tree`                         | `pipdeptree` 替代                                                     |

---

## 4 一次性运行 / 工具模式

|场景|命令|说明|
|---|---|---|
|运行脚本|`uv run script.py`|自动在当前 or 最近的虚拟环境里执行，没有就先建再跑citeturn5view0|
|运行并即时安装工具|`uvx ruff check.`|`uvx` = `uv tool run`，临时环境用完即删citeturn7search4|
|安装持久化工具|`uv tool install[email protected]`|pipx 的超快替代|
|执行已装工具|`uv tool run mytool …`||

---

## 5 pip‑兼容接口（迁移老项目）

| 常见需求                    | 命令                                   |
| ----------------------- | ------------------------------------ |
| 安装                      | `uv pip install -r requirements.txt` |
| 卸载                      | `uv pip uninstall package`           |
| 列包                      | `uv pip list` / `uv pip freeze`      |
| 依赖树                     | `uv pip tree`                        |
| 依赖一致性检查                 | `uv pip check`                       |
| 用 requirements.txt 同步环境 | `uv pip sync requirements.txt`       |

（全部子命令与 pip-tools / virtualenv 接口保持一致，但速度大幅提升）citeturn6view0

---

## 6 辅助命令

|命令|用途|
|---|---|
|`uv cache dir` / `prune`|查看或清理二进制与轮子缓存citeturn3view0|
|`uv build` / `publish`|构建 / 上传轮子到 PyPI|
|`--no-managed-python`|强制使用系统 Python，禁止自动下载|

---

### 与 conda / pipenv 的关键差异

1. **始终推荐虚拟环境**：uv 默认在项目根自动找到/创建 `.venv`，不激活也能用。
    
2. **锁文件 `uv.lock`**：精准固定依赖‑平台‑Python 版，跨平台可复现。
    
3. **内置 Python 版本管理**：省掉 `pyenv` / `conda install python=…`。
    
4. **极速解析与安装**：解析依赖用 Rust 写的 `uv-resolver`，官方基准比 pip 高阶工具快 8‑20×。
    

用这份速查表，基本可以把日常 conda + pip 的工作流整体迁移到 uv。祝你体验飞一般的依赖安装速度！
