---
draw:
tags: []
title: CICD for viva
date created: 2024-12-27
date modified: 2024-12-28
---
1. 本地开发 Viva 项目代码
2. 推送到 GitHub，触发 GitHub Actions, GitHub Actions 执行以下步骤：
	- 检出代码
	- 设置 Docker Buildx
	- 登录 Docker Hub
	- 构建 Docker 镜像并推送到 Docker Hub
	- 前端镜像：your-dockerhub-username/viva-frontend:latest
	- 后端镜像：your-dockerhub-username/viva-backend:latest
	- 将配置文件传输到服务器 B：
3. 在服务器 B 上执行部署：
	- 拉取最新的 Docker 镜像
	- 停止旧容器
	- 启动新容器

## 团队协作开发

1. **仓库访问权限设置**：

```plaintext
GitHub 仓库设置流程：
1. 进入仓库 Settings → Collaborators and teams
2. 添加团队成员，可以设置不同权限级别：
   - Read: 只能查看和克隆
   - Write: 可以推送代码
   - Admin: 可以管理仓库设置
```

2. **分支保护策略**：

```plaintext
保护主分支设置流程：
1. 进入 Settings → Branches
2. 添加分支保护规则：
   - 要求 Pull Request 审查
   - 禁止直接推送到主分支
   - 要求状态检查通过才能合并
```

3. **工作流程建议**：

```yaml:.github/workflows/deploy.yml
name: Deploy Viva Project

on:
  push:
    branches:
      - main          # 生产环境部署
  pull_request:       # 添加 PR 触发器
    branches:
      - main

jobs:
  test:               # 添加测试任务
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run tests
        run: |
          # 添加测试命令
          echo "Running tests..."
          
  deploy:
    needs: test       # 需要测试通过才能部署
    if: github.ref == 'refs/heads/main'  # 只在主分支部署
    runs-on: ubuntu-latest
    
    steps:
      # ... 之前的部署步骤保持不变 ...
```

1. **环境分离**：

```yaml:docker-compose.yml
services:
  frontend:
    image: your-dockerhub-username/viva-frontend:${TAG:-latest}  # 使用环境变量
    # ...

  backend:
    image: your-dockerhub-username/viva-backend:${TAG:-latest}   # 使用环境变量
    environment:
      - NODE_ENV=${NODE_ENV:-production}
      - DATABASE_URL=${DATABASE_URL}
    # ...
```

1. **团队开发流程**：

```plaintext
标准开发流程：

1. 克隆仓库
   git clone https://github.com/your-org/viva.git

2. 创建功能分支
   git checkout -b feature/new-feature

3. 开发并提交
   git add .
   git commit -m "feat: add new feature"
   git push origin feature/new-feature

4. 创建 Pull Request
   - 在 GitHub 上创建 PR
   - 等待代码审查
   - CI 自动运行测试

5. 合并到主分支
   - 审查通过后合并
   - 自动触发部署
```

1. **建议的项目结构**：

```Java
.
├── .github
│   └── workflows
│       ├── deploy.yml          # 生产环境部署
│       └── pr-check.yml        # PR 检查
├── docker-compose.yml          # 生产环境配置
├── docker-compose.dev.yml      # 开发环境配置
├── viva-frontend
│   ├── Dockerfile
│   └── nginx.conf
└── viva-backend
    └── Dockerfile
```

2. **环境变量管理**：
- 在 GitHub 仓库设置中配置 Secrets：
  - `DOCKERHUB_USERNAME` louisleonard
  - `DOCKERHUB_TOKEN` dckr_pat_U2MIMviLW8x3YR5kAQS5k8RBrkA
  - `SERVER_B_HOST` 43.134.93.160
  - `SERVER_B_USERNAME` root
  - `SERVER_B_PASSWORD` Lys876538875..
  - `DATABASE_URL`
  - 其他环境特定的变量

1. **代码审查流程**：

```plaintext
PR 审查清单：
1. 代码质量
2. 测试覆盖
3. 文档更新
4. 性能影响
5. 安全考虑
```

这样的设置可以让团队成员：

- 使用自己的 GitHub 账号进行开发
- 通过 PR 提交代码变更
- 自动运行测试和检查
- 只有通过审查的代码才能部署到生产环境
- 保持代码质量和部署的安全性

建议：

1. 为团队制定明确的开发规范
2. 使用语义化版本控制
3. 保持良好的文档习惯
4. 定期进行代码审查会议
5. 建立开发环境和测试环境

这样可以确保团队协作顺畅，同时保持部署的安全性和可控性。

## 为什么用 docker

### 通过 Docker 镜像部署（当前方式）

优点：

1. **环境一致性**
   - 开发、测试和生产环境完全一致
   - 避免了"在我电脑上能运行"的问题

2. **版本控制**
   - 每个镜像都有具体的标签
   - 方便回滚到之前的版本
   - 可以保留历史版本

3. **隔离性**
   - 应用和依赖都封装在容器内
   - 不同应用间互不影响
   - 更安全，降低安全风险

4. **可移植性**
   - 可以轻松迁移到其他服务器
   - 支持横向扩展
   - 便于实现负载均衡

缺点：

1. **构建时间**
   - 需要额外的时间构建镜像
   - 需要推送到镜像仓库

2. **存储空间**
   - 需要额外的镜像存储空间
   - Docker Hub 带宽消耗

3. **学习成本**
   - 需要了解 Docker 相关知识
   - 配置相对复杂

### 直接部署源代码

优点：

1. **部署速度快**
   - 无需构建镜像
   - 直接传输源代码

2. **简单直接**
   - 配置较少
   - 学习成本低

3. **资源占用少**
   - 不需要额外的镜像存储
   - 服务器负担小

缺点：

1. **环境依赖**
   - 需要在服务器上安装所有依赖
   - 可能出现环境不一致问题

2. **版本管理难**
   - 回滚较麻烦
   - 难以维护多个版本

3. **扩展性差**
   - 迁移服务器需要重新配置环境
   - 横向扩展复杂

### 建议

对于现代应用开发，特别是微服务架构，推荐使用 Docker 方式，因为：

1. 长期维护成本更低
2. 更适合团队协作
3. 更容易实现 DevOps
4. 更好的可扩展性

但如果是小型项目或个人项目，直接部署源码也是可以的，主要看：

1. 项目规模和复杂度
2. 团队大小
3. 维护周期
4. 扩展需求
