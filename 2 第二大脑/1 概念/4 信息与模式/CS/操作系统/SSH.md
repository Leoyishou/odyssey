---
date created: 2024-09-23
date modified: 2025-07-26
uid: c41441d2-b9ac-42bd-9cd5-d6e0cadbcdb9
---
SSH（Secure Shell）是一种加密的网络协议，用于在不安全的网络中安全地访问和管理远程计算机。

# 主要用途

**1. 远程登录**

```bash
# 基本连接
ssh username@hostname

# 指定端口
ssh -p 2222 username@hostname

# 使用密钥
ssh -i ~/.ssh/id_rsa username@hostname
```

**2. 文件传输**

```bash
# 使用 scp 复制文件
scp local_file.txt user@remote:/path/to/destination
scp user@remote:/path/to/file.txt ./local_directory

# 使用 sftp 交互式传输
sftp user@remote
```

**3. 端口转发（隧道）**

```bash
# 本地端口转发
ssh -L 8080:localhost:80 user@remote

# 远程端口转发
ssh -R 9090:localhost:3000 user@remote
```

# 工作原理

1. **建立连接**：客户端连接到服务器的 SSH 端口（默认22）
2. **协商加密**：双方协商使用的加密算法
3. **服务器认证**：客户端验证服务器身份（通过主机密钥）
4. **用户认证**：用户通过密码或密钥认证
5. **安全通信**：所有数据都经过加密传输

# 认证方式

**密码认证**：

- 简单但相对不够安全
- 容易受到暴力破解攻击

**密钥认证**：

```bash
# 生成密钥对
ssh-keygen -t rsa -b 4096

# 复制公钥到服务器
ssh-copy-id user@remote
```

# 配置文件

**~/.ssh/config** 示例：

```Java
Host myserver
    HostName 192.168.1.100
    User admin
    Port 2222
    IdentityFile ~/.ssh/myserver_key
    
Host github
    HostName github.com
    User git
    IdentityFile ~/.ssh/github_key
```

使用配置后可以简化连接：

```bash
ssh myserver  # 相当于 ssh -p 2222 -i ~/.ssh/myserver_key admin@192.168.1.100
```

# 常用选项

- `-v`：显示详细调试信息
- `-X`：启用 X11 转发（图形界面）
- `-N`：不执行远程命令（仅用于端口转发）
- `-f`：后台运行
- `-C`：压缩数据传输

# 安全建议

1. **禁用密码登录**：只使用密钥认证
2. **更改默认端口**：避免自动化攻击
3. **限制用户访问**：使用 AllowUsers 或 AllowGroups
4. **使用强密钥**：至少 2048 位 RSA 或 Ed25519
5. **定期更新**：保持 SSH 软件最新
6. **使用 fail2ban**：防止暴力破解

# 实际应用场景

- **服务器管理**：远程管理 Linux 服务器
- **Git 操作**：通过 SSH 推送代码到 GitHub/GitLab
- **数据库隧道**：安全访问远程数据库
- **自动化部署**：CI/CD 流程中的自动部署
- **跳板机**：通过堡垒机访问内网服务器

SSH 是系统管理员和开发人员的必备工具，它提供了安全、灵活的远程访问方案。
