---
date created: 2024-10-25
date modified: 2025-10-25
uid: 8edeae73-0ee8-42ed-80aa-4d4af780abbc
---

左边的栈里面可以 set frame 跳到之前的栈，实现方法里面的进退自如。


# 一、cursor 的 python debug

**结论：** 对于 FastAPI + Uvicorn，必须选择 **"Debug using launch.json"** 并配置好 launch.json 文件！

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F10%2F25%2F01-05-00-84c9c54bf7fa28bcdda85d7f94157af0-202510250105800-db7c7e.png)


**❌ 不要用 "Debug Python File"**  
因为直接运行 `main.py` 不会启动 Uvicorn 服务器

**✅ 应该用 "Debug using launch.json"**  
因为需要通过 uvicorn 模块启动，而不是直接运行文件

|特性|Debug Python File|Debug using launch.json|
|---|---|---|
|配置文件|不需要|需要 launch.json|
|适用场景|简单脚本|复杂应用/服务器|
|参数传递|不支持|支持|
|环境变量|不支持|支持|
|模块启动|不支持|支持 (如 uvicorn)|
|**你的情况**|❌ 不适用|✅ 使用这个|

# `launch.json` 在哪儿？
```Java
你的项目根目录/
├── .vscode/
│   └── launch.json  👈 在这里
├── app/
│   └── main.py
├── requirements.txt
└── ...
```

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "FastAPI Debug",
            "type": "debugpy",
            "request": "launch",
            "module": "uvicorn",
            "args": [
                "app.main:app",
                "--host", "0.0.0.0",
                "--port", "8080",
                "--reload"
            ],
            "jinja": true,
            "justMyCode": false
        }
    ]
}
```



# mark object

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F25%2F13-33-01-2ccc8744b9fb80ec602b39fd8948da82-202410251333169-062813.png)

# step out

出栈到调用这个函数的地方  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F25%2F13-59-29-1cbef40445cbc35d4a4e56826be01eee-202410251359218-4dfacf.png)
