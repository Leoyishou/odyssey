---
draw:
tags: []
title: viva 数据库
date created: 2024-10-02
date modified: 2024-12-27
---

## 更新时间和创建时间字段

1. 为这俩字段设置默认值`CURRENT_TIMESTAMP`
2. 设置函数
3. 为表增加触发器，自动调用更新函数

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F02%2F19-01-18-de2c47b30144bfa9d697a4669946b81e-202410021901492-72bd13.png)

## 链路

当然可以。作为一个 Java 和 MyBatis 的使用者，我理解你可能对 Python 中的数据库交互流程感到陌生。让我为你详细解释一下 Python 中的这个链路，特别是在这个项目中使用的模式。

1. 数据库连接和会话管理

在 Python 中，我们通常使用 SQLAlchemy 这个 ORM（对象关系映射）库来处理数据库操作。它类似于 Java 中的 Hibernate。

```python
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

# 创建数据库引擎
engine = create_engine('postgresql://user:password@localhost/dbname')

# 创建会话工厂
SessionLocal = sessionmaker(bind=engine)

# 创建会话
session = SessionLocal()
```

1. 模型定义

模型定义类似于 Java 中的实体类。在 SQLAlchemy 中，我们这样定义：

```python
from sqlalchemy import Column, Integer, String, DateTime, Boolean
from sqlalchemy.ext.declarative import declarative_base

Base = declarative_base()

class Essay(Base):
    __tablename__ = 'essays'

    essay_id = Column(Integer, primary_key=True)
    user_id = Column(String)
    title = Column(String)
    content = Column(String)
    create_time = Column(DateTime)
    update_time = Column(DateTime)
    is_deleted = Column(Boolean)
```

1. 仓库层 (Repository)

仓库层负责与数据库交互，类似于 MyBatis 中的 Mapper。

```python
class EssayRepository:
    def get_essays_by_user_id(self, user_id: str) -> List[Dict]:
        with db_manager.session_scope() as session:
            essays = session.query(Essay).filter(
                Essay.user_id == user_id,
                Essay.is_deleted == False
            ).all()
            
            return [essay.__dict__ for essay in essays]
```

这里的 `db_manager.session_scope()` 是一个上下文管理器，用于管理数据库会话的生命周期。

1. 用例层 (Use Case)

用例层包含业务逻辑，调用仓库层来获取数据。

```python
class GetUserEssaysUseCase:
    def __init__(self, essay_repository: EssayRepository):
        self.essay_repository = essay_repository

    def execute(self, user_id: str) -> List[Dict]:
        return self.essay_repository.get_essays_by_user_id(user_id)
```

1. 控制器层 (Controller)

控制器层处理 HTTP 请求，调用用例层，并返回响应。

```python
@router.get("/essays", response_model=List[EssaySchema])
async def get_user_essays(
    current_user: dict = Depends(get_current_user),
    essay_repository: EssayRepository = Depends(EssayRepository)
):
    use_case = GetUserEssaysUseCase(essay_repository)
    essays_data = use_case.execute(current_user['id'])
    return [EssaySchema(**essay_data) for essay_data in essays_data]
```

1. 模式验证 (Schema Validation)

Pydantic 用于数据验证和序列化，类似于 Java 中的 Jackson 或 Gson。

```python
class EssaySchema(BaseModel):
    essay_id: int
    user_id: str
    title: str
    content: str
    create_time: datetime
    update_time: datetime
    is_deleted: bool
```

整体流程：

1. HTTP 请求到达控制器
2. 控制器创建用例实例
3. 用例调用仓库方法
4. 仓库方法打开数据库会话，执行查询
5. 查询结果返回给用例
6. 用例将结果返回给控制器
7. 控制器使用 Pydantic 模型验证和序列化数据
8. 序列化后的数据作为 HTTP 响应返回

与 MyBatis 的主要区别：

1. Python 使用动态类型，不需要像 Java 那样显式定义返回类型
2. SQLAlchemy 的查询构建更加 Pythonic，不需要 XML 映射文件
3. 依赖注入通过 FastAPI 的 Depends 实现，而不是像 Spring 那样使用注解

希望这个解释能帮助你理解 Python 中的数据库交互流程！
