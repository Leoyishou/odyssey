---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-26
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary



# Cues



# Notes

⏺ Update Todos
  ⎿  ☐ 配置本地 SSH 密钥
     ☐ 获取 AutoDL 实例信息
     ☐ 配置 SSH config 文件
     ☐ 安装 VS Code Remote-SSH 插件
     ☐ 连接到 AutoDL 实例

⏺ VS Code Remote SSH 连接 AutoDL 详细步骤

  1. 生成 SSH 密钥（如果还没有）

⏺ Update Todos
  ⎿  ☐ 配置本地 SSH 密钥
     ☐ 获取 AutoDL 实例信息
     ☐ 配置 SSH config 文件
     ☐ 安装 VS Code Remote-SSH 插件
     ☐ 连接到 AutoDL 实例

⏺ # 检查是否已有 SSH 密钥
  ls ~/.ssh/id_rsa.pub

  # 如果没有，生成新的密钥对
  ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
  # 一路回车使用默认设置

  # 查看公钥内容（稍后需要复制到 AutoDL）
  cat ~/.ssh/id_rsa.pub

  2. 在 AutoDL 配置 SSH

⏺ Update Todos
  ⎿  ☒ 配置本地 SSH 密钥
     ☐ 获取 AutoDL 实例信息
     ☐ 配置 SSH config 文件
     ☐ 安装 VS Code Remote-SSH 插件
     ☐ 连接到 AutoDL 实例

⏺ 登录 AutoDL 控制台：
  1. 进入你的实例详情页
  2. 找到 SSH 连接信息：
    - 公网 IP 地址
    - SSH 端口号（通常是随机分配的，如 30877）
    - 用户名（通常是 root）
  3. 在 AutoDL 的 JupyterLab 终端中添加公钥：
  # 在 AutoDL JupyterLab 终端执行
  mkdir -p ~/.ssh
  echo "你的公钥内容" >> ~/.ssh/authorized_keys
  chmod 700 ~/.ssh
  chmod 600 ~/.ssh/authorized_keys

  4. 配置本地 SSH config

⏺ Update Todos
  ⎿  ☒ 配置本地 SSH 密钥
     ☒ 获取 AutoDL 实例信息
     ☐ 配置 SSH config 文件
     ☐ 安装 VS Code Remote-SSH 插件
     ☐ 连接到 AutoDL 实例

⏺ # 编辑 SSH 配置文件
  nano ~/.ssh/config
  # 或使用 vim
  vim ~/.ssh/config

  添加以下内容（替换为你的实际信息）：
  Host autodl
      HostName 123.45.67.89  # 替换为 AutoDL 实例的公网 IP
      User root
      Port 30877             # 替换为 AutoDL 实例的 SSH 端口
      IdentityFile ~/.ssh/id_rsa
      ServerAliveInterval 60
      ServerAliveCountMax 10

  保存后测试连接：
  # 测试 SSH 连接
  ssh autodl

  # 如果成功，你应该能看到 AutoDL 实例的命令行

  4. 安装 VS Code Remote-SSH 插件

⏺ Update Todos
  ⎿  ☒ 配置本地 SSH 密钥
     ☒ 获取 AutoDL 实例信息
     ☒ 配置 SSH config 文件
     ☐ 安装 VS Code Remote-SSH 插件
     ☐ 连接到 AutoDL 实例

⏺ 1. 打开 VS Code
  2. 点击左侧扩展图标（或按 Cmd+Shift+X）
  3. 搜索 "Remote - SSH"
  4. 安装 Microsoft 官方的 "Remote - SSH" 插件

  5. 连接到 AutoDL

⏺ Update Todos
  ⎿  ☒ 配置本地 SSH 密钥
     ☒ 获取 AutoDL 实例信息
     ☒ 配置 SSH config 文件
     ☒ 安装 VS Code Remote-SSH 插件
     ☐ 连接到 AutoDL 实例

⏺ 1. 按 Cmd+Shift+P 打开命令面板
  2. 输入 "Remote-SSH: Connect to Host..."
  3. 选择 "autodl"（你在 config 中配置的名称）
  4. VS Code 会打开新窗口并连接到 AutoDL

  首次连接时：
  - 选择 Linux 作为远程系统类型
  - 等待 VS Code Server 自动安装（需要几分钟）

  6. 使用技巧

  连接成功后：
  - 使用 Cmd+O 打开远程文件夹（如 /root/CS336/）
  - 终端会自动连接到远程服务器
  - 可以安装 Python、Jupyter 等插件，在远程运行

  断线重连：
  # 如果连接断开，在 VS Code 中按 F1
  # 选择 "Remote-SSH: Kill VS Code Server on Host..."
  # 然后重新连接

  同步本地代码到 AutoDL：
  # 在本地终端执行
  rsync -avz --exclude='venv/' --exclude='__pycache__/' \
    /Users/liuyishou/usr/projects/learn/CS336/ \
    autodl:/root/CS336/

⏺ Update Todos
  ⎿  ☒ 配置本地 SSH 密钥
     ☒ 获取 AutoDL 实例信息
     ☒ 配置 SSH config 文件
     ☒ 安装 VS Code Remote-SSH 插件
     ☒ 连接到 AutoDL 实例

> 替换为 AutoDL 实例的 SSH 端口
  这个怎么知道

⏺ 在 AutoDL 获取 SSH 端口的方法：

  1. AutoDL 控制台查看

  登录 https://www.autodl.com/console/instance/list：

  2. 进入"我的实例"页面
  3. 找到你的实例，点击"详情"或"管理"
  4. 在实例详情页会显示：
    - 登录地址：类似 region-x.autodl.com:30877
    - SSH 端口：冒号后面的数字（如 30877）
    - 公网 IP：用于 SSH 连接

  5. JupyterLab 内查看

  如果已经能访问 JupyterLab：
  # 在 JupyterLab 终端执行
  echo $SSH_CONNECTION
  # 或者
  ss -tlnp | grep sshd

  6. AutoDL 实例信息示例

  通常显示格式如下：
  登录地址：region-1.autodl.com:30877
  SSH连接：ssh -p 30877 root@123.45.67.89
  密码：your_password

  其中 30877 就是 SSH 端口号。
