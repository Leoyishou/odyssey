---
comment_id: 4643792e
date created: 2025-03-15
date modified: 2025-03-15
draw: null
title: Python 中的相对导入和绝对导入
---
## 绝对导入

绝对导入是从项目的根目录开始引用模块的方式。无论你的当前文件位于项目的哪个位置，绝对导入的路径都是固定的。

### 示例

假设你有以下项目结构：

```Java
myproject/
├── app/
│   ├── __init__.py
│   ├── main.py
│   ├── interfaces/
│   │   ├── __init__.py
│   │   └── api/
│   │       ├── __init__.py
│   │       └── essay_controller.py
│   └── domain/
│       ├── __init__.py
│       └── services/
│           ├── __init__.py
│           └── essay_service.py
```

在 `essay_controller.py` 中使用绝对导入：

```python
# 绝对导入
from app.domain.services.essay_service import EssayService
```

绝对导入的特点：

- 路径从项目根目录开始
- 不受文件位置影响，在任何地方都可以使用相同的导入语句
- 更清晰，容易理解代码之间的依赖关系
- 需要项目根目录在 Python 的模块搜索路径中

## 相对导入

相对导入是基于当前文件的位置来引用模块的方式。使用点号（`.`）来表示相对路径。

### 示例

在同样的项目结构下，在 `essay_controller.py` 中使用相对导入：

```python
# 相对导入
from ....domain.services.essay_service import EssayService  # 向上四级，再向下到domain/services
```

或者在 `main.py` 中：

```python
# 相对导入
from .interfaces.api.essay_controller import router as essay_router  # 当前目录下的interfaces/api
```

相对导入的特点：

- 使用点号（`.`）表示相对路径
  - 一个点（`.`）表示当前包
  - 两个点（`..`）表示上一级包
  - 更多点表示更上级的包
- 依赖于文件在包结构中的位置
- 如果文件移动，相对导入可能需要更改
- 只能在包内使用，不能在直接运行的脚本中使用

## 你项目中的问题

在你的项目中，出现了混合使用两种导入方式的情况：

1. 在 `main.py` 中使用相对导入：

```python
from interfaces.api.essay_controller import router as essay_router
```

2. 而在 `essay_controller.py` 中使用绝对导入：

```python
from app.application.use_cases.get_sentence_use_case import GetSentenceUseCase
```

当你在容器中运行 `python app/main.py` 时：

- Python 将 `/app` 目录添加到模块搜索路径
- `main.py` 中的相对导入可以找到 `interfaces.api.essay_controller`
- 但 `essay_controller.py` 中的绝对导入找不到 `app` 模块，因为 Python 在寻找 `/app/app` 而不是将 `/app` 识别为 `app` 模块

## 最佳实践

1. **保持一致性**：在整个项目中使用同一种导入方式
   
2. **推荐使用绝对导入**：
   - 更清晰，不受文件位置影响
   - 在大型项目中更容易维护
   
3. **正确设置 Python 路径**：
   - 确保项目根目录在 Python 的模块搜索路径中
   - 可以通过设置 `PYTHONPATH` 环境变量或在代码中修改 `sys.path`

4. **使用 `__init__.py` 文件**：
   - 在每个目录中添加 `__init__.py` 文件，使其成为一个 Python 包
   - 这样 Python 可以正确识别和导入包结构

通过我们的修改（更改工作目录和设置 `PYTHONPATH`），我们解决了这个问题，使 Python 能够正确找到所有模块。
