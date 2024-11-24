---
draw:
tags: []
title: Pydantic
date created: 2024-10-24
date modified: 2024-11-12
---

Pydantic 和 Zod 都是数据验证库，但它们分别针对不同的编程语言生态系统。让我为你详细介绍：

## Pydantic

Pydantic 是一个 Python 的数据验证库：

1. 主要特点：
- 基于 Python 类型注解的数据验证
- 自动类型转换
- JSON Schema 生成和验证
- 高性能（核心验证使用 Rust 实现）
- 与 FastAPI 等框架深度集成

1. 基本使用示例：

```python
from pydantic import BaseModel

class User(BaseModel):
    id: int
    name: str
    email: str
    age: int = None  # 可选字段

# 有效数据
user = User(id=1, name="John", email="john@example.com")

# 自动类型转换
user = User(id="1", name="John", email="john@example.com")  # id 会自动转换为 int

# 无效数据会抛出异常
user = User(id="abc", name="John", email="invalid-email")  # 会抛出验证错误
```

2. 常用功能：
- 复杂的嵌套数据结构验证
- 自定义验证器
- 数据序列化和反序列化
- 环境变量处理
- 密码字段处理

## Zod

Zod 是一个 TypeScript 的数据验证库：

1. 主要特点：
- 完全类型安全
- 零依赖
- 可以直接从 schema 推导 TypeScript 类型
- 链式 API
- 高度可扩展性

1. 基本使用示例：

```typescript
import { z } from "zod";

// 定义 schema
const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
  email: z.string().email(),
  age: z.number().optional()
});

// 类型推导
type User = z.infer<typeof UserSchema>;

// 验证数据
const userData = {
  id: 1,
  name: "John",
  email: "john@example.com"
};

// 解析和验证
const user = UserSchema.parse(userData);

// 会抛出错误的情况
const invalidData = {
  id: "abc",
  name: 123,
  email: "invalid-email"
};
```

2. 常用功能：
- 复杂对象和数组验证
- 条件验证
- 自定义错误信息
- 数据转换（transform）
- 默认值设置

## 两者比较

1. 语言生态：
- Pydantic: Python 生态系统
- Zod: TypeScript/JavaScript 生态系统

1. 使用场景：
- Pydantic: 后端 API 开发，数据模型定义，配置管理
- Zod: 前端表单验证，API 类型定义，运行时类型检查

1. 主要优势：
- Pydantic:
  - 与 Python 类型系统完美集成
  - 性能优越
  - 广泛的框架支持
  
- Zod:
  - 完整的 TypeScript 类型推导
  - 链式 API 设计
  - 更灵活的验证规则定义

1. 使用建议：
- 如果你在开发 Python 后端，特别是使用 FastAPI，选择 Pydantic
- 如果你在开发 TypeScript 前端或 Node.js 应用，选择 Zod

两个库都非常优秀，选择主要取决于你的技术栈和具体需求。它们都能提供强大的数据验证能力，帮助提高代码的可靠性和类型安全性。

这个问题问得很好。确实LLM输出的JSON经常会有各种"不规范"的情况。Pydantic处理这类"不完美"输出主要通过以下机制：

1. 容错解析机制

```python
# LLM可能输出这样的不规范JSON
"""
{
    name: "John",  # 少了引号
    'age': 25,     # 混用单引号
    "score": 89.5  # 正常格式
}
"""
```

Pydantic会尝试：

- 使用Python的ast模块解析而不是json.loads
- 自动修复常见的格式问题(比如缺少引号)
- 处理注释和尾随逗号
- 支持单引号和双引号混用

1. 智能类型转换

```python
class Person(BaseModel):
    age: int
    is_student: bool
    score: float

# LLM输出的各种"奇怪"格式都能正确解析
data = {
    "age": "25",          # 字符串数字
    "is_student": "yes",  # 字符串表示布尔
    "score": "89.5"       # 字符串浮点数
}
person = Person.parse_obj(data)  # 自动转换为正确类型
```

2. 部分数据提取

```python
from pydantic import BaseModel, Extra

class Response(BaseModel):
    name: str
    age: int
    
    class Config:
        extra = Extra.ignore  # 忽略多余字段
        
# LLM返回了额外信息也能正常解析
data = {
    "name": "John",
    "age": 25,
    "extra_field": "some value",  # 会被自动忽略
    "another_field": 123
}
```

3. 容错值处理

```python
from typing import Optional
from pydantic import Field

class Response(BaseModel):
    name: str = Field(default="Unknown")  # 默认值
    age: Optional[int] = None  # 可空字段
    tags: list[str] = []  # 空列表默认值
```

4. 自定义解析逻辑

```python
from pydantic import validator

class Response(BaseModel):
    status: str

    @validator('status', pre=True)
    def parse_status(cls, v):
        status_map = {
            'ok': 'success',
            'yes': 'success',
            'no': 'failed',
            'error': 'failed'
        }
        return status_map.get(v.lower(), v)  # 统一各种异常表达
```

实践建议：

1. 在prompt中尽量给出明确的输出格式要求
2. 使用更宽松的类型定义（比如Optional）
3. 为关键字段提供默认值和解析函数
4. 添加详细的错误处理逻辑

您觉得这些机制能覆盖您在实际项目中遇到的异常情况吗？或者还有其他特殊场景需要讨论？​​​​​​​​​​​​​​​​
