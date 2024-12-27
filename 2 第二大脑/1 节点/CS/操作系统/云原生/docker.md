---
draw:
tags: []
title: docker
date created: 2024-07-20
date modified: 2024-12-27
---

`docker run -it --rm viva_backend /bin/bash`  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F20%2F22-46-47-16d0d8394aee51737c6924bf849aa23c-20240720224647-c2b05e.png)

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
