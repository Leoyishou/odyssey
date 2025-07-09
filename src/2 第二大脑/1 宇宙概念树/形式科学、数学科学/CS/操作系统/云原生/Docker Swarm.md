---
comment_id: 2dec0a3e
date created: 2024-12-28
date modified: 2025-03-25
draw: null
title: Docker Swarm
---
- - **Docker Swarm**：与 Docker Compose 类似的声明式编排语法（同一个 `docker-compose.yml` 文件稍加改动就能直接用于 Swarm），并且 Swarm 直接内置在 Docker Engine 中，不需要额外安装其他组件，易于上手。
- **功能完整，能够满足常见生产场景**
    
    - **容器编排**：Swarm 自带服务发现与编排能力，可以根据需要将容器分配到 Swarm 集群中的不同节点。
    - **负载均衡**：Swarm 默认提供内部负载均衡，Service 之间会自动分发流量，不用额外配置。
    - **服务扩展**：只需简单修改副本数，就可轻松地在集群中水平扩容/缩容容器。
    - **滚动更新**：Swarm 可以以滚动的方式更新容器镜像和配置，保证在更新期间服务的可用性。
- **运维简单，符合多数小中型项目需求**
    
    - **集群搭建简单**：Swarm 模式只需要 `docker swarm init` / `docker swarm join` 等几条命令即可快速搭建集群；不需要像 Kubernetes 那样有繁琐的部署和依赖。
    - **与 Docker 生态无缝衔接**：直接复用 Docker CLI 与 Docker Compose 语法，一切保持一致，减少了学习和运维成本。
    - **足够应对大部分生产场景**：Swarm 在稳定性、扩展性和可维护性方面，对普通的企业或个人项目来说已经十分够用。
