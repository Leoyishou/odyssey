---
draw:
tags: []
title: docker compose
date created: 2024-07-20
date modified: 2024-11-12
---
Docker Compose 是一个用于定义和运行多容器 [[docker]] 应用程序的工具。
- 是 Docker 的补充工具
- 用于管理多个容器
- 使用 `docker-compose.yml` 定义服务

这是一个Docker构建过程的日志，主要完成了以下几个关键步骤：

1. 基础环境设置：
- 使用 `continuumio/miniconda3:latest` 作为基础镜像
- 设置工作目录为 `/app`

2. 系统依赖安装：
- 更新apt包列表
- 安装了一些必要的系统依赖：
  - pkg-config
  - default-libmysqlclient-dev
  - build-essential
- 这些是编译和运行某些Python包所必需的

3. Conda环境配置：
- 复制 `environment.yml` 文件到容器
- 创建了一个名为 "viva" 的新conda环境
- 根据yml文件安装了大量Python包，主要包括：
  - FastAPI 和 Uvicorn (Web框架和服务器)
  - SQLAlchemy 和 psycopg2 (数据库相关)
  - Anthropic、OpenAI (AI相关SDK)
  - LangChain (大语言模型框架)
  - Pandas, NumPy (数据处理)
  - Matplotlib (数据可视化)
  - Spacy (自然语言处理)
  - 其他工具包如python-dotenv、python-multipart等

4. Shell配置：
- 设置默认shell为conda环境中的bash，确保后续命令在conda环境中执行

整个过程是在构建一个包含完整数据科学和AI开发环境的Docker容器。这个环境似乎是为了运行一个结合了数据库操作、AI模型调用和Web服务的应用程序。构建过程中遇到了一些废弃警告，但整体构建是成功的。