---
draw:
title: FastAPI
date created: 2024-11-09
date modified: 2025-02-06
---

```Java
my_fastapi_project/
├── app/
│   ├── __init__.py
│   ├── main.py          # FastAPI 应用程序实例和主路由
│   ├── api/             # API 路由模块
│   │   ├── __init__.py
│   │   ├── v1/         # API 版本控制
│   │   │   ├── __init__.py
│   │   │   ├── endpoints/
│   │   │   │   ├── users.py
│   │   │   │   └── items.py
│   │   │   └── router.py
│   ├── core/           # 核心配置
│   │   ├── __init__.py
│   │   ├── config.py   # 配置设置
│   │   └── security.py # 安全相关
│   ├── db/             # 数据库相关
│   │   ├── __init__.py
│   │   ├── base.py
│   │   └── session.py
│   ├── models/         # SQLAlchemy 模型
│   │   ├── __init__.py
│   │   ├── user.py
│   │   └── item.py
│   ├── schemas/        # Pydantic 模型
│   │   ├── __init__.py
│   │   ├── user.py
│   │   └── item.py
│   ├── crud/          # CRUD 操作
│   │   ├── __init__.py
│   │   ├── base.py
│   │   └── user.py
│   └── services/      # 业务逻辑
│       ├── __init__.py
│       └── user_service.py
├── tests/            # 测试文件
│   ├── __init__.py
│   ├── conftest.py
│   └── test_api/
├── alembic/          # 数据库迁移
│   ├── versions/
│   └── env.py
├── logs/            # 日志文件
├── .env            # 环境变量
├── .gitignore
├── requirements.txt
├── alembic.ini     # Alembic 配置
└── README.md
```

`async` 和普通的 `def` 在处理 I/O 操作（如 API 调用、数据库操作等）时有重要区别：

1. **异步（async）方式**：

优点：

- 非阻塞：其他请求可以同时处理
- 更高的并发能力
- 适合 I/O 密集型操作
- 内存效率更高

缺点：

- 阻塞：一次只能处理一个请求
- 每个请求都创建新实例，资源消耗大
- 并发能力受限

## 写法

```python:app/main.py
from fastapi import FastAPI
from app.services.ai_service import AIService

app = FastAPI()

# 方式1：全局实例 + async（推荐）
ai_service = AIService()

@app.post("/api/summarize")
async def summarize_text(text: str):
    """
    - 复用全局实例
    - 异步处理请求
    - 高并发性能好
    """
    return await ai_service.summarize_text(text)

# 方式2：每次新建实例 + async（不推荐）
@app.post("/api/analyze")
async def analyze_text(text: str):
    """
    - 每次创建新实例，资源消耗大
    - 虽然是异步，但实例创建开销大
    """
    service = AIService()
    return await service.summarize_text(text)

# 方式3：同步方式（不推荐）
@app.post("/api/process")
def process_text(text: str):
    """
    - 阻塞操作
    - 低并发性能
    - 不适合 I/O 密集型操作
    """
    service = AIService()
    return service.summarize_text(text)
```

## 最佳实践

- 使用**全局实例 + async 方式**
- 在 FastAPI 中尽量使用异步路由
- 避免在每个请求中创建新实例
- 对于需要共享的资源（如数据库连接、API 客户端），使用全局实例

```python:app/main.py
# 推荐的写法
from fastapi import FastAPI
from app.services.ai_service import AIService

app = FastAPI()
ai_service = AIService()  # 全局实例

@app.post("/api/summarize")
async def summarize_text(request: SummarizeRequest):
    return await ai_service.summarize_text(request.text)
```

这样可以获得最好的性能和资源利用率。
