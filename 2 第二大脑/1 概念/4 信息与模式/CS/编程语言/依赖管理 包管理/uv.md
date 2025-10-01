---
date created: 2025-03-24
date modified: 2025-09-20
uid: 0fc91684-868b-49f2-9d2e-c4008eef39b5
---
```Java
# 使用 UV 指定 Python 3.9

uv init --python 3.9
uv add --requirements requirements.txt
```

下面是一套**在 PyCharm 里用 `uv` 管理 Python 环境的最佳实践（2025 版）**，尽量做到“IDE 原生支持 + 项目可复现 + 团队可落地”。

---

# TL;DR（结论先行）

- **用 PyCharm 自带的 `uv` 解释器类型**建/选环境，不要混用 pip/conda 的新建方式。自 2024.3.2 起 PyCharm 原生支持 `uv`，2025.1/2025.2 又补上了 Jupyter 与锁文件的 UI 按钮。([The JetBrains Blog](https://blog.jetbrains.com/pycharm/2025/01/pycharm-2024-3-2/?utm_source=chatgpt.com "PyCharm 2024.3.2: uv Package Management Support and More!"), [JetBrains](https://www.jetbrains.com/pycharm/whatsnew/2025-1/?utm_source=chatgpt.com "What's New in PyCharm 2025.1 - JetBrains"))

- **项目用 `pyproject.toml` + `uv.lock` 做“单一事实来源”**，把这两个文件提交到仓库，`.venv/` 忽略。([Astral Docs](https://docs.astral.sh/uv/guides/projects/?utm_source=chatgpt.com "Working on projects | uv - Astral Docs"))

- **新增依赖用 `uv add`，安装/同步用 `uv sync`**（开发依赖用 `--dev` 或分组 `--group`），不要在环境里临时 `pip install`。([Astral Docs](https://docs.astral.sh/uv/concepts/projects/dependencies/?utm_source=chatgpt.com "Managing dependencies | uv - Astral Docs"))

- **运行和调试用 “Run/Debug Configuration: uv”**；脚本也可用 `uv run` 与 PEP 723 **内联依赖**（适合一次性脚本/工具）。([JetBrains](https://www.jetbrains.com/help/pycharm/run-debug-configuration-uv.html "Run/Debug Configuration: uv | PyCharm Documentation"), [Astral Docs](https://docs.astral.sh/uv/guides/scripts/?utm_source=chatgpt.com "Running scripts | uv - Astral Docs"))

- **CI/生产**：克隆后 `uv sync --frozen`；若环境不得不用 pip，可从锁文件导出 `requirements.txt`。([Astral Docs](https://docs.astral.sh/uv/guides/projects/?utm_source=chatgpt.com "Working on projects | uv - Astral Docs"))

---

# 一、准备工作

1. **升级 PyCharm**：建议使用 2025.1 及以上（含 `uv` 解释器、Jupyter with uv），2025.2 还在 `.toml` 编辑器里提供了“Sync/Lock/Update”持久按钮来操作依赖与锁文件。([JetBrains](https://www.jetbrains.com/pycharm/whatsnew/2025-1/?utm_source=chatgpt.com "What's New in PyCharm 2025.1 - JetBrains"))

2. **安装 `uv`**（一次性）：按官方安装脚本或包管理器安装；如需同时管理 Python 版本，可用 `uv python install`（可加 `--default` 暴露 `python/python3` 可执行文件）。([Astral Docs](https://docs.astral.sh/uv/getting-started/installation/?utm_source=chatgpt.com "Installation | uv - Astral Docs"))

---

# 二、在 PyCharm 正确配置 `uv` 解释器

> 目标：让 PyCharm 的“项目解释器”直接指向 `uv` 环境，这样 IDE 的包管理与运行都走 `uv`。

- **新建/现有项目绑定 `uv`**
    打开 _Settings → Python | Interpreter_ → **Add Interpreter → Add Local Interpreter → 选择 `uv`**。

    - **New uv environment**：选择基准 Python（可以是 `uv` 安装的版本），由 PyCharm 直接创建 `.venv`。

    - **Existing uv environment**：先在项目根目录 `uv venv`（或 `uv sync` 自动创建 `.venv`），再在 PyCharm 里选这个现成环境。
        完成后，你可以在解释器页用 IDE 的“安装/升级包”入口，后台会通过 `uv` 执行。([JetBrains](https://www.jetbrains.com/help/pycharm/uv.html "Configure a uv environment | PyCharm Documentation"))

> 备注：**新建项目时直接选 `uv`**，PyCharm 会自动生成 `pyproject.toml` 作为项目配置的起点。([JetBrains](https://www.jetbrains.com/help/pycharm/uv.html "Configure a uv environment | PyCharm Documentation"))

---

# 三、依赖管理（项目级，推荐工作流）

- **新增依赖**

    ```bash
    uv add fastapi
    uv add --dev pytest ruff      # 开发依赖
    uv add --group lint ruff      # 自定义分组
    ```

    以上操作会更新 `pyproject.toml` 并刷新 `uv.lock`。推荐把分组用于拆分“运行时 vs. 开发/工具”。([Astral Docs](https://docs.astral.sh/uv/concepts/projects/dependencies/?utm_source=chatgpt.com "Managing dependencies | uv - Astral Docs"))

- **安装/同步（全仓一致）**

    ```bash
    uv sync           # 按锁文件安装
    uv sync --frozen  # CI/生产：严格按锁文件，不解新依赖
    ```

    `uv sync`/`uv run` 会确保锁与环境一致，避免“装了但没记”的漂移。([Astral Docs](https://docs.astral.sh/uv/guides/projects/?utm_source=chatgpt.com "Working on projects | uv - Astral Docs"))

- **IDE 内操作**
    2025.2 开始，编辑 `pyproject.toml` 时右上角会出现 **Sync / Lock / Update** 固定按钮；点击即可让 IDE 通过 `uv` 同步或更新。([JetBrains](https://www.jetbrains.com/pycharm/whatsnew/ "What’s New in PyCharm 2025.2"))

- **为什么要提交 `uv.lock`**
    `uv.lock` 记录**精确**解析结果（跨平台可复现），应随代码提交；`.venv/` 忽略。([Astral Docs](https://docs.astral.sh/uv/guides/projects/?utm_source=chatgpt.com "Working on projects | uv - Astral Docs"))

---

# 四、运行与调试

- **使用 `uv` 专用的 Run/Debug**
    打开 _Run → Edit Configurations_，新建 **uv** 模板，填写 _Script path / Module name_、参数、环境变量等；选择你的 **uv 解释器** 即可。常见用法：

    - 以模块运行：`Module name = uvicorn`，参数 `app:app --reload`。

    - 普通脚本：填 `Script path = path/to/main.py`。
        还可以在 “Before launch” 里加一个外部工具：`uv sync --frozen`，确保起跑前环境一致。([JetBrains](https://www.jetbrains.com/help/pycharm/run-debug-configuration-uv.html "Run/Debug Configuration: uv | PyCharm Documentation"))

- **脚本内联依赖（PEP 723）**
    对一次性脚本很香：

    ```python
    #!/usr/bin/env -S uv run --script
    # /// script
    # requires-python = ">=3.12"
    # dependencies = ["httpx>=0.27"]
    # ///
    import httpx
    ```

    直接运行脚本或在 Run 配置里用 `uv` 运行，`uv` 会为该脚本临时准备隔离环境。([Astral Docs](https://docs.astral.sh/uv/guides/scripts/?utm_source=chatgpt.com "Running scripts | uv - Astral Docs"))

- **Jupyter（可选）**
    2025.1 起，可用 `uv` 解释器启动 Jupyter 服务器/内核，Notebook 与项目依赖一致。([JetBrains](https://www.jetbrains.com/pycharm/whatsnew/2025-1/?utm_source=chatgpt.com "What's New in PyCharm 2025.1 - JetBrains"))

---

# 五、团队协作与 CI

- **开发者上手**：`git clone` → `uv sync` → 在 PyCharm 选择项目的 `uv` 解释器即可开始。`pyproject.toml` 与 `uv.lock` 必须在仓库里。([Astral Docs](https://docs.astral.sh/uv/guides/projects/?utm_source=chatgpt.com "Working on projects | uv - Astral Docs"))

- **CI**：用官方 `setup-uv` action 安装 `uv`，然后 `uv sync --frozen && pytest`。([GitHub](https://github.com/astral-sh/setup-uv?utm_source=chatgpt.com "astral-sh/setup-uv: Set up your GitHub Actions workflow ... - GitHub"))

- **与仅支持 pip 的平台集成**：从锁文件**导出** `requirements.txt`（例如 Cloud Functions/老镜像）：

    ```bash
    uv export --no-hashes --format requirements-txt > requirements.txt
    ```

    或者使用 `uv pip compile pyproject.toml -o requirements.txt` 生成需求锁定文件。([Astral Docs](https://docs.astral.sh/uv/reference/cli/?utm_source=chatgpt.com "Commands | uv"))

---

# 六、常见问题排查

- **IDE 识别不到 `uv`**：在“添加解释器”对话框里手动指定 `uv` 可执行文件路径（Windows/Unix 路径不同）；或确认 `uv` 已按官方方法正确安装。([JetBrains](https://www.jetbrains.com/help/pycharm/uv.html "Configure a uv environment | PyCharm Documentation"), [Astral Docs](https://docs.astral.sh/uv/getting-started/installation/?utm_source=chatgpt.com "Installation | uv - Astral Docs"))

- **解释器/索引异常**：偶发错误时，移除该解释器→关闭 IDE→删除项目 `.venv/`→重新 `uv venv`/`uv sync`→再添加解释器。([YouTrack](https://youtrack.jetbrains.com/issue/PY-80706/Errors-using-uv-environment?utm_source=chatgpt.com "Errors using uv environment : PY-80706 - JetBrains YouTrack"))

- **Auto‑import/索引不及时**：多数情况下是解释器没选到项目 `.venv` 或环境未同步，按上一步重新选解释器并 `uv sync` 即可（历史讨论指向同因）。([Stack Overflow](https://stackoverflow.com/questions/79452710/uv-in-pycharm-handling-local-imports?utm_source=chatgpt.com "uv in PyCharm / handling local imports - python - Stack Overflow"))

---

# 七、日常命令速查（最少记忆版）

```bash
# 初始化/引入
uv init                             # 生成 pyproject.toml 等
uv add <pkg>                        # 新增依赖
uv add --dev <pkg>                  # 新增开发依赖
uv sync                             # 按锁文件安装
uv lock                             # 只更新锁（不装）
uv run -m <module> ...              # 在项目环境中运行模块/命令
uv export -o requirements.txt       # 导出给仅支持 pip 的平台
```

（以上命令均与 PyCharm 的 `uv` 解释器/运行配置良好协作。）([Astral Docs](https://docs.astral.sh/uv/guides/projects/?utm_source=chatgpt.com "Working on projects | uv - Astral Docs"))

---

## 建议的项目结构（提交到 Git）

```Java
.
├── pyproject.toml   # 依赖与元数据（手工/uv add 修改）
├── uv.lock          # 锁文件（自动生成，务必提交）
├── .venv/           # 本地虚拟环境（忽略）
└── src/ tests/ ...
```

> 这样配好后，在 PyCharm 里就可以**用 GUI 装包/升级**、用 **uv Run/Debug** 运行、在 **Jupyter** 里复用同一环境，同时确保团队/CI 一致。([JetBrains](https://www.jetbrains.com/help/pycharm/uv.html "Configure a uv environment | PyCharm Documentation"))

---

如果你告诉我 **操作系统** 和 **PyCharm 版本**，我可以按你的环境给出一份带截图的**最少点击步骤**（比如如何把现有项目一键切换到 `uv` 解释器、如何在 Run 配置里加“启动前执行 `uv sync --frozen`”的小钩子）。

# Summary

是不是只要有 pyproject.toml，一跑 uv run 的命令，环境就会自动按照配置好的去构建了

UV 的优势

1. **速度**：UV 用 Rust 写的，比 Conda 快 10-100 倍
2. **自动化**：`uv run` 自动处理环境，不需要手动激活
3. **标准化**：`pyproject.toml` 是 Python 官方标准（PEP 518）
4. **锁文件**：`uv.lock` 精确锁定所有依赖版本

所以你的理解完全正确 - `pyproject.toml` 就是 UV 版本的环境配置文件，功能上等同于 Conda 的 `environment.yml`！

| 操作 | Conda | UV |
|---|---|---|
| 创建环境 | `conda create -n myenv` | `uv venv` 或自动创建 |
| 激活环境 | `conda activate myenv` | `source.venv/bin/activate` 或不需要 |
| 运行命令 | `python script.py` | `uv run python script.py` |
| 安装包 | `conda install numpy` | `uv pip install numpy` |
| 退出环境 | `conda deactivate` | `deactivate` |

为什么 UV 不强调激活？

1. **项目隔离**：UV 为每个项目创建独立的`.venv`
2. **自动检测**：`uv run` 自动找到正确的环境
3. **避免错误**：不会因忘记激活环境而使用错误的 Python

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

下面这张 **uv 常用操作 Cheat Sheet** 选取了最常用、最容易和 conda / pip 搞混的命令，按“装-建-管-跑-辅助”五大类归纳。所有示例均可直接在终端复制粘贴。

---

## 1 安装 / 升级

| 场景 | 命令 | 说明 |
|---|---|---|
| 一键安装（macOS/Linux）| `bash\ncurl -LsSf https://astral.sh/uv/install.sh \| sh\n` | 官方独立安装脚本，会把 `uv` 二进制放到 `~/.local/bin`（或 Windows 的 `%LOCALAPPDATA%\\uv`）[^turn7search1]|
| pipx 安装 | `pipx install uv` | 如果你已经在用 pipx 管理 CLI |
| 升级到最新 | `uv self update` | 自更新，可加 `--version 0.6.14` 锁定版本[^turn3view0]|
| 查看版本 | `uv version` | |

---

## 2 虚拟环境 & Python 版本

| 场景 | 命令 | 备注 |
|---|---|---|
| 在当前目录建 `.venv` | `uv venv` | 不用再手动 source 激活；uv 会自动发现并使用 `.venv`[^turn3view0]|
| 指定路径 / 名称 | `uv venv venv38` | |
| 选择 Python 版本 | `uv venv --python 3.11` | 若本机无该版本，uv 会自动下载 python-build-standalone 发行版[^turn7search0]|
| 列出已装 Python | `uv python list` | |
| 安装新版本 Python | `uv python install 3.12` | 多版本并存，互不干扰[^turn7search0]|

---

## 3 项目级工作流（“像 Cargo 一样”）

| 步骤 | 命令 | 相当于 |
|---|---|---|
| 初始化项目 | `uv init` | `poetry init` / `pipenv --three` |
| 添加依赖 | `uv add numpy pandas` | `pip install` + 写入 `pyproject.toml` + 更新 `uv.lock`[^turn5view0]|
| 移除依赖 | `uv remove pandas` | |
| 同步环境 | `uv sync` | 根据 `uv.lock` 安装 / 升级 / 移除 |
| 只刷新锁文件 | `uv lock` | 不触碰环境 |
| 导出 requirements.txt | `uv export --format requirements` | |
| 查看依赖树 | `uv tree` | `pipdeptree` 替代 |

---

## 4 一次性运行 / 工具模式

| 场景 | 命令 | 说明 |
|---|---|---|
| 运行脚本 | `uv run script.py` | 自动在当前 or 最近的虚拟环境里执行，没有就先建再跑[^turn5view0]|
| 运行并即时安装工具 | `uvx ruff check.` | `uvx` = `uv tool run`，临时环境用完即删[^turn7search4]|
| 安装持久化工具 | `uv tool install[email protected]` | pipx 的超快替代 |
| 执行已装工具 | `uv tool run mytool …` | |

---

## 5 pip-兼容接口（迁移老项目）

| 常见需求 | 命令 |
|---|---|
| 安装 | `uv pip install -r requirements.txt` |
| 卸载 | `uv pip uninstall package` |
| 列包 | `uv pip list` / `uv pip freeze` |
| 依赖树 | `uv pip tree` |
| 依赖一致性检查 | `uv pip check` |
| 用 requirements.txt 同步环境 | `uv pip sync requirements.txt` |

（全部子命令与 pip-tools / virtualenv 接口保持一致，但速度大幅提升）[^turn6view0]

---

## 6 辅助命令

| 命令 | 用途 |
|---|---|
| `uv cache dir` / `prune` | 查看或清理二进制与轮子缓存[^turn3view0]|
| `uv build` / `publish` | 构建 / 上传轮子到 PyPI |
| `--no-managed-python` | 强制使用系统 Python，禁止自动下载 |

---

### 与 conda / pipenv 的关键差异

1. **始终推荐虚拟环境**：uv 默认在项目根自动找到/创建 `.venv`，不激活也能用。
2. **锁文件 `uv.lock`**：精准固定依赖-平台-Python 版，跨平台可复现。
3. **内置 Python 版本管理**：省掉 `pyenv` / `conda install python=…`。
4. **极速解析与安装**：解析依赖用 Rust 写的 `uv-resolver`，官方基准比 pip 高阶工具快 $8-20\times$。

用这份速查表，基本可以把日常 conda + pip 的工作流整体迁移到 uv。祝你体验飞一般的依赖安装速度！
