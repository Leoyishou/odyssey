---
draw:
tags: []
title: Docker
date created: 2024-07-20
date modified: 2025-01-08
---

所有有状态的服务，都别放在docker里跑。例如mysal、redis、elasticsearch、activemq。因为这些服务都要保证高可用和稳定性，放在docker里很难做到，就算是k8s里，也不建议放。这也是为什么云厂商会提供这些有状态的saas服务，例如在aws里，会提供mysal、redis、mq的服务。

学习 Docker 生态的三步通常是指：

Docker (基础)  
[docker compose](docker%20compose.md) (多容器应用管理)  
[[Docker Swarm]] 或 [[Kubernetes]] (容器编排，用于生产环境)

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F20%2F22-46-47-16d0d8394aee51737c6924bf849aa23c-20240720224647-c2b05e.png)

## run：运行某个 container

``docker run -it --rm viva_backend /bin/bash`  `

## exec：进入某个 container

`docker exec -it e16baf6e001c /bin/bash`

## build

![image.png|1800](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F12%2F28%2F02-09-07-13f6fe76d8c90b92a33f7dbfa237e686-202412280209138-04c38c.png)

![Uploading file...i61jc]()

[Orbstack](Orbstack.md) 是 [MacBook](MacBook.md) 用 docker 的最佳实践

从这个错误看，问题可能出在 SSL 握手阶段。让我们一步步检查：

1. 首先检查 443 端口是否被 Nginx 正确监听：

bash

Copy

`# 在服务器上运行 sudo netstat -tulpn | grep:443 # 或者用 docker 命令检查 docker-compose exec frontend netstat -tulpn | grep:443`

1. 检查证书和密钥的权限：

bash

Copy

`ls -l ~/viva/ssl/ # 确保权限是 644 对于 cert.pem，600 对于 key.pem chmod 644 ~/viva/ssl/cert.pem chmod 600 ~/viva/ssl/key.pem`

1. 检查 Nginx 配置文件在容器中的实际位置：

bash

Copy

`# 进入容器 docker-compose exec frontend bash # 查看 nginx 配置 nginx -T # 检查证书文件是否正确挂载 ls -l /etc/nginx/ssl/`

1. 查看 Nginx 错误日志：

bash

Copy

`docker-compose exec frontend tail -f /var/log/nginx/error.log`

1. 验证前端容器的 Dockerfile 是否正确配置了 SSL：

bash

Copy

`# 检查 viva-frontend/Dockerfile 内容 cat viva-frontend/Dockerfile`

可能需要确保 Dockerfile 中安装了 SSL 相关包：

dockerfile

Copy

`FROM nginx:alpine # 安装 SSL 相关包 RUN apk add --no-cache openssl #... 其他配置...`

1. 确认 nginx.conf 文件的位置和内容是否正确：

bash

Copy

`# 检查配置文件位置 docker-compose exec frontend sh -c 'ls -l /etc/nginx/conf.d/'`

要执行这些检查吗？从哪个开始？我们可以根据结果一步步找出问题所在。

## 文件路径问题

我理解你的问题 - 这是一个典型的 Docker 容器中的文件路径问题。让我解释一下原因和解决方案：

问题的根源在于:

1. 在 Docker 容器中,工作目录被设置为 `/app`
2. 你的日志配置使用的是相对路径 `logs/app.log`
3. 当容器重启时,除非明确配置 volume 映射,否则写入的文件会丢失

这里有两种解决方案:

1. 将日志目录映射到宿主机:

```yaml
services:
  backend:
    volumes:
      - ./viva-backend/.env:/app/.env
      - ./viva-backend/logs:/app/logs  # 添加这行来持久化日志
```

2. 修改日志配置使用绝对路径:

```python
# logging_config.py
log_dir = '/app/logs'  # 使用绝对路径
if not os.path.exists(log_dir):
    os.makedirs(log_dir)

file_handler = RotatingFileHandler(
    os.path.join(log_dir, 'app.log'),
    maxBytes=10485760,
    backupCount=5
)
```

我建议使用第一种方案,因为:

1. 可以在容器外部直接访问日志文件
2. 容器重启时日志不会丢失
3. 便于日志收集和分析

另外,建议在 Dockerfile 中添加创建日志目录的步骤:

```dockerfile
# 在 WORKDIR /app 之后添加
RUN mkdir -p /app/logs && chmod 777 /app/logs
```

这样可以确保容器启动时日志目录已存在且具有适当的权限。

你觉得这些解决方案中哪一个更适合你的需求?
