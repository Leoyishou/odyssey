---
draw:
title: Uvicorn
date created: 2025-03-24
date modified: 2025-03-25
---

通俗来说，**Uvicorn** 就是一个用来运行 Python 写的 Web 应用程序的服务器工具。

想象一下：

你用 Python 写好了一个网站或接口（比如用 FastAPI 或者 Starlette），代码本身只是静静地躺在文件里。你得有个东西帮你“启动”它，让外面的人能真正访问它。这时候，就需要一个“服务器”来帮忙。

**Uvicorn** 就是这种服务器之一。它的特点是：

- **快**：基于 Python 的异步特性（asyncio），运行效率特别高。
- **简单**：启动命令很直观，一行命令就能把程序跑起来。
- **兼容性强**：专门为支持“ASGI”接口标准设计的（类似于Flask/Werkzeug支持的是WSGI），比如 FastAPI、Starlette、Quart 等等异步框架，都能顺畅运行。

比如你写了一个叫做 `app.py` 的程序，里面定义了一个 FastAPI 对象叫做 `app`：

```python
from fastapi import FastAPI

app = FastAPI()

@app.get("/")
async def root():
    return {"message": "Hello World"}
```

你只需要执行：

```bash
uvicorn app:app --reload
```

它就会帮你启动服务器，在浏览器里输入网址后，就能看到你的网站或者接口结果了。

---

总结：

**Uvicorn** 就像一个高效的“快递员”，帮你把 Python 写的接口迅速地传递给访问你网站的人。
