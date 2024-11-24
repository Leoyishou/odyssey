---
draw:
tags: []
title: FastAPi部署
date created: 2023-11-22
date modified: 2024-11-12
---

FastAPI 是一个可快速构建 API 服务的 Web 框架，可与 NodeJS 和 Go 比肩的极高性能（归功于 Starlette 和 Pydantic），是最快的 Python Web 框架之一。更多详情见官网 [FastAPI官网地址](https://link.zhihu.com/?target=https%3A//fastapi.tiangolo.com/zh/)

  

本文对 FastAPI 的开发部署以及生产环境部署做一个记录。

## 开发部署

安装 `uvicorn` 作为 asgi 应用服务器

```text
pip install uvicorn
```

例：main.py

```python3
from fastapi import FastApi

app = FastApi()


@app.get('/hello')
async def hello():
    return {'message': 'hello World'}
```

nvicorn main:app --reload 开发模式下运行 热加载

如果是想在 Pycharm 等 IDE 中直接运行，可以在代码中加入

```python3
if __name__ == '__main__':
    uvicorn.run("main:app", host="127.0.0.1", port=8000, log_level="info")
```

即可直接 run 运行，不需要使用命令行，但只适合用于开发环境。

## 生产环境部署

生产环境：CentOS7.x +Nginx

  
1.安装 Gunicorn

Gunicorn 是一个 unix 上被广泛使用的高性能的 Python WSGI UNIX HTTP Server，和大多数的 Web 框架兼容，并具有实现简单，轻量级，高性能等特点。

使用 gunicorn 启动应用程序的好处是，它可以处理大量的并发连接,，并且其使用的是预派生子进程的方式，这意味着它能够更好地利用多核 CPU。

安装命令

```text
pip install uvicorn
pip install gunicorn
```

Shell 中执行 `gunicorn -v` 有版本输出表示安装成功

  
2.以配置文件方式启动应用

创建 gunicorn.py 文件，里面包含下列内容

```python3
import os

# 设置守护进程
daemon=True
# 监听内网端口8000
bind='0.0.0.0:8000'
# 设置进程文件目录
pidfile='./gunicorn.pid'
chdir='./' # 工作目录
# 工作模式
worker_class='uvicorn.workers.UvicornWorker'
# 并行工作进程数 核心数*2+1个
workers=3  #multiprocessing.cpu_count()+1
# 指定每个工作者的线程数
threads=2
# 设置最大并发量
worker_connections = 2000
loglevel='debug' # 错误日志的日志级别
access_log_format = '%(t)s %(p)s %(h)s "%(r)s" %(s)s %(L)s %(b)s %(f)s" "%(a)s"'
# 设置访问日志和错误信息日志路径
log_dir = "./log"
if not os.path.exists(log_dir):
    os.makedirs(log_dir)
accesslog = "./log/gunicorn_access.log"
errorlog = "./log/gunicorn_error.log"
```

## 运行项目

1.上面的 gunicorn.py 文件配置好后，使用如下命令：

```text
gunicorn main:app -c gunicorn.py
```

在 gunicorn_error.log 文件中看到日志输出表示启动成功。

2.当然也可以不用配置文件，直接执行下面命令启动应用

```text
gunicorn main:app -b 0.0.0.0:8000 -w 4 -k uvicorn.workers.UvicornWorker --daemon
```

这是一个启动命令，主要作用是使用 gunicorn 作为应用部署服务器，并指定服务器的启动参数：

- `main:app` 指定了 Gunicorn 要运行的应用程序入口点；
- `-b 0.0.0.0:8000` 指定了服务器的 IP 地址和端口；
- `-w 4` 指定了 Gunicorn 使用 4 个工作进程同时处理请求；
- `-k uvicorn.workers.UvicornWorker` 指定使用 UvicornWorker 作为工作进程的类型；
- `--daemon` 参数表示将服务器以守护进程 (daemon) 模式启动（后台运行）。

## 停止项目

如果想要停止 Gunicorn 项目，可以先通过查看进程树找到进程 pid，然后使用 kill 命令结束进程

1.获取 gunicorn 进程树

```text
pstree -ap | grep gunicorn
```

2.终止 gunicorn 任务

```text
kill -HUP 进程pid
```

3.如果使用了多进程，那么执行了上述命令后还会有子进程在运行，可以使用如下命令杀死

```text
kill -9 进程pid
```

当然，这样一个一个关闭太繁琐了，我们可以编写一个 Shell 脚本，来实现一键杀死所有主进程和子进程的功能。

脚本示例：

```bash
#!/bin/bash
​
# 查找 gunicorn 主进程 PID
gunicorn_pid=$(ps aux | grep 'gunicorn' | grep -v 'grep' | awk '{print $2}')
​
# 如果找到了主进程 PID
if [ -n "$gunicorn_pid" ]; then
  echo "Found gunicorn process: $gunicorn_pid"
  
  # 给主进程发 SIGINT 信号，请求正常停止进程
  kill -INT $gunicorn_pid
  
  # 睡眠 5 秒等待主进程结束 
  sleep 5
  
  # 查找所有 gunicorn 子进程 PID
  gunicorn_child_pids=$(pstree -p $gunicorn_pid | grep -oP '([0-9]+)(?=\))')
  
  # 如果找到了子进程 PID
  if [ -n "$gunicorn_child_pids" ]; then
    echo "Found gunicorn child processes: $gunicorn_child_pids"
    
    # 杀死所有子进程
    for pid in $gunicorn_child_pids; do
      kill -9 $pid
    done
  fi
  
  echo "Stopped gunicorn process and child processes"
  
else
  echo "No running gunicorn process found"
fi
```

这个脚本可以完成以下操作：

1. 查找 gunicorn 主进程 PID
2. 给主进程发送 SIGINT 信号请求正常关闭
3. 睡眠 5 秒等待主进程结束
4. 使用 `pstree` 命令查找所有 gunicorn 子进程 PID
5. 使用 `kill` 命令杀死所有子进程

您只需要将以上代码保存到一个文件中 (例如 stop_gunicorn.sh)，并确保该文件有执行权限，然后在命令行界面运行该脚本，即可一次性停止 gunicorn 进程和所有子进程：

```text
bash stop_gunicorn.sh
```

## 配置开机自启 (可选）

配置 gunicorn.service 服务开机自启动

```text
cat >/usr/lib/systemd/system/gunicorn.service << EOF
[Unit]
Description=Gunicorn fast
After=syslog.target network.target remote-fs.target nss-lookup.target
[Service]
Type=forking
PIDFile=/var/run/gunicorn.pid
ExecStart=/root/.local/share/virtualenvs/fastapi-Xq8atoqR/bin/gunicorn  -c 
/opt/web/fastapi/gunicorn.py main:app
ExecReload=/bin/kill -s HUP $MAINPID
ExecStop=/bin/kill -s QUIT $MAINPID
PrivateTmp=true
 
[Install]
WantedBy=multi-user.target
 
EOF
```

依次执行下面命令

```text
systemctl daemon-reload
systemctl enable gunicorn
systemctl start gunicorn
```

查看服务状态

```text
systemctl status gunicorn
```

## 配置 Nginx 代理访问 (可选）

```nginx
server {
            listen 80;
           # listen 443 ssl;
            server_name api.hmily.vip;
            access_log  /var/log/nginx/access.log;
            location / {
                proxy_pass http://127.0.0.1:8000;
                proxy_set_header Host $host;
                proxy_set_header X-Forwarded-For
                $proxy_add_x_forwarded_for;
        }
    }
```

运行 nginx -s reload，至此就完成了 FastAPI 的生产部署！

## **docs/redoc**文档显示空白问题

有时候我们用不同设备或者浏览器会出现 docs/redoc 文档空白不能显示的问题，这是因为模板文件用的是外国的 cdn，所以国内访问会比较慢，出现访问失败的情况，导致显示空白。

解决办法：

修改 FastAPI 源文件，把模板文件保存到本地，然后修改文件路径让其从本地访问。

1.首先修改源文件，模板文件路径如下：

```text
fastapi/openapi/docs.py
```

把 docs.py 文件里的下面这三行

```python3
swagger_js_url: str = "https://cdn.jsdelivr.net/npm/swagger-ui-dist@4/swagger-ui-bundle.js",
swagger_css_url: str = "https://cdn.jsdelivr.net/npm/swagger-ui-dist@4/swagger-ui.css",
swagger_favicon_url: str = "https://fastapi.tiangolo.com/img/favicon.png",
```

修改为

```python3
swagger_js_url: str="/static/swagger-ui/swagger-ui-bundle.js",
swagger_css_url: str="/static/swagger-ui/swagger-ui.css",
swagger_favicon_url: str="/static/swagger-ui/favicon.png",
```

2.修改我们的项目代码，添加下面这段代码

```python3
from fastapi.staticfiles import StaticFiles
app.mount('/static', StaticFiles(directory='static'),
          name='static')
```

注意：directory 路径需要根据自己的实际情况设置。

  

3.把我们的 static 模板文件放置到项目根目录下，静态文件我放网盘了 [百度云文件](https://link.zhihu.com/?target=https%3A//pan.baidu.com/s/1nWFSSKtCHA0QEkhEiJTh1w%3Fpwd%3D0516)，有需要可自行下载。

  

最后重新运行我们的项目，便大功告成！

  

  
2022.11.06  
如需转载请详细注明来源
