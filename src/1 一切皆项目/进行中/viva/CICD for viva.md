---
date created: 2024-12-27
date modified: 2025-07-10
uid: ee9db5ce-f455-4661-adbf-f6edf48d5216
---

| 步骤       | 当前问题      | 优化后效果             |
| -------- | --------- | ----------------- |
| Docker构建 | 缓存未利用     | 利用分层缓存，速度提升 > 70% |
| 镜像大小     | 大量依赖，镜像偏大 | 镜像减小 >50%，拉取速度更快  |
| SCP文件上传  | 全量上传慢     | 仅镜像仓库拉取或增量上传，快数倍  |
| 服务器镜像构建  | 重复构建镜像    | 镜像不再构建，仅拉取，提速明显   |

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

1. **分支保护策略**：

```plaintext
保护主分支设置流程：
1. 进入 Settings → Branches
2. 添加分支保护规则：
   - 要求 Pull Request 审查
   - 禁止直接推送到主分支
   - 要求状态检查通过才能合并
```

1. **工作流程建议**：

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

1. **环境变量管理**：
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

---

你这个 CI/CD 流程整体思路清晰，结构明确，Docker 和 SSH 部署都配置得比较完整。但每次触发 GitHub Action 都比较慢，通常和**构建流程**、**镜像大小**、**网络传输**、**服务器 IO 性能**等环节有关。接下来详细分析你的配置并提供优化建议：

---

## 🚩 一、为什么你的 GitHub Action 执行慢？

根据你的配置，慢的环节主要有：

### **1. Docker 构建（Build）**

- 每次都在执行完全的 Docker 构建，未有效使用缓存，导致每次构建过程缓慢。
- Docker 的依赖安装步骤重复，耗时较长。

### **2. 文件上传（SCP）**

- 使用 `appleboy/scp-action` 每次上传整个项目到服务器，包含了大量可能并未修改的文件，导致上传缓慢。

### **3. SSH 部署过程中的 Docker 镜像重建**

- SSH 到服务器后，又强制执行了后端镜像的重建：

    ```bash
    docker-compose build --no-cache backend
    ```

    这一步会完全忽略缓存，导致再次耗费大量时间。

---

## 🛠️ 二、优化建议

### 🚀 **1. 充分利用 Docker 缓存**

Docker 的分层缓存机制能极大提高速度：

- **Dockerfile 优化**  
    **原理**：Dockerfile 中依赖安装（如 Python 依赖）和代码拷贝应分离，以避免每次小改动都重新安装依赖。
    
    **示例 Dockerfile 优化**：

    ```dockerfile
    FROM python:3.12-slim
    
    WORKDIR /app
    
    # 先复制依赖清单并安装依赖
    COPY requirements.txt .
    RUN pip install -r requirements.txt
    
    # 最后才复制源代码，充分利用缓存
    COPY . .
    
    CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
    ```

    每次只有当 `requirements.txt` 有变化时才重新安装依赖，代码的变动则只更新最上层，速度更快。

    

---

### 🌊 **2. Docker 镜像优化：减小镜像体积**

- 使用轻量级基础镜像，如 `python:3.12-alpine` 或 `python:3.12-slim`。
- 清理构建时产生的临时文件：

    ```dockerfile
    RUN pip install -r requirements.txt && rm -rf /root/.cache/pip
    ```

---

### 📦 **3. 改善文件上传过程**

你目前使用 SCP 一次性上传所有文件，即使很多文件未发生变动：

**改进方案**：

- **只上传 Docker Compose 文件和构建好的镜像（推荐）** 实践中，前后端镜像一般推送到 Docker Hub 或阿里云镜像仓库，服务器端仅需拉取镜像即可，无需上传大量源码文件：

    ```yaml
    - name: Build and push Docker images
      run: |
        docker-compose build
        docker-compose push
    ```

    然后 SSH 到服务器仅拉取镜像：

    ```bash
    docker-compose pull
    docker-compose up -d
    ```

- **采用 rsync 代替 SCP** 如果确实需要上传文件，使用`appleboy/rsync-action`，只上传差异文件，速度明显提升。
    

---

### 🚨 **4. 服务器端减少镜像重建**

当前你 SSH 后执行了：

```bash
docker-compose build --no-cache backend
```

建议改为直接使用从 Docker Hub 拉取的镜像，而非在服务器上重新构建：

```bash
docker-compose pull
docker-compose up -d
```

服务器端无需再重复构建镜像，速度显著提高。

---

### ⏱️ **5. GitHub Action 优化：使用缓存 Action**

在 GitHub Actions 中加入缓存：

```yaml
- name: Cache Docker layers
  uses: actions/cache@v3
  with:
    path: /tmp/.buildx-cache
    key: ${{ runner.os }}-buildx-${{ github.sha }}
    restore-keys: |
      ${{ runner.os }}-buildx-
```

在构建时指定缓存路径：

```yaml
- name: Set up Docker Buildx
  uses: docker/setup-buildx-action@v2
  with:
    driver-opts: image=moby/buildkit:latest
```

---

### ⚙️ **6. Workflow 结构优化**

- 可考虑分离测试与部署 job，单独做快速的单元测试检查，后端容器镜像的构建只在成功测试后才执行，这样可以更快速定位问题。
- 同时合理设置 Action 并发运行策略，防止过多任务并发，挤占 Action 的资源。

---

## 🌟 **推荐的 CI/CD 流程改进**

整体优化的推荐流程：

**GitHub Actions:**

1. 拉取代码
2. 单元测试（快速）
3. 构建 Docker 镜像（使用缓存，优化Dockerfile）
4. 推送到 Docker Hub 或其他镜像仓库

**服务器端:**

1. 拉取镜像 (`docker-compose pull`)
2. 直接启动服务 (`docker-compose up -d`)

这样速度最快，效果最明显。

---

## 🚦 **示例优化后流程**

**GitHub Action 改进版配置片段示例**：

```yaml
- name: Build and push Docker images
  uses: docker/build-push-action@v5
  with:
    context: .
    push: true
    tags: louisleonard/viva-backend:latest
    cache-from: type=gha
    cache-to: type=gha,mode=max

- name: SSH deploy
  uses: appleboy/ssh-action@master
  with:
    host: ${{ secrets.SERVER_B_HOST }}
    username: ${{ secrets.SERVER_B_USERNAME }}
    password: ${{ secrets.SERVER_B_PASSWORD }}
    script: |
      cd /path/to/deploy
      docker-compose pull
      docker-compose up -d
```

---

## 🎯 **总结优化后效果**

|步骤|当前问题|优化后效果|
|---|---|---|
|Docker构建|缓存未利用|利用分层缓存，速度提升 > 70%|
|镜像大小|大量依赖，镜像偏大|镜像减小 >50%，拉取速度更快|
|SCP文件上传|全量上传慢|仅镜像仓库拉取或增量上传，快数倍|
|服务器镜像构建|重复构建镜像|镜像不再构建，仅拉取，提速明显|

通过这些优化，你的 GitHub Action 流程执行速度将有显著提升，开发部署效率会大幅提高。
