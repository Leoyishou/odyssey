---
date created: 2024-09-15
date modified: 2025-10-07
uid: 22d93b7f-936f-4ae6-955b-3ee4da9d74f5
---

### 1. 极速启动与执行（Agent 不会“卡住”）

Claude Code 执行任务时，经常会运行 `docker run` 来启动数据库、测试环境或验证代码。

- **Docker Desktop:** 启动引擎通常需要几十秒甚至更久。如果你的 Docker 后台没开，Agent 执行命令会报错，或者让你等很久。
    
- **OrbStack:** 几乎是秒开（Instant startup）。即使后台没挂着，Claude Code 敲下命令的瞬间，OrbStack 就能唤醒并执行。这保证了 Agent 的“心流”不被打断。
    

### 2. 资源占用极低（留给 LLM 和 IDE）

运行 Claude Code（尤其是本地模型或高频 API 调用）本身就消耗系统资源，你肯定不希望 Docker 再吃掉 4GB+ 内存。

- **Docker Desktop:** 基于传统虚拟机，吃内存大户，空载也容易发热。
    
- **OrbStack:** 采用轻量级虚拟化技术，内存占用极低（通常只有 DD 的 1/5），CPU 占用更是几乎可以忽略不计。这对于保持电脑流畅运行 Agent 至关重要。
    

### 3. Agent 友好的网络与域名特性

这是 OrbStack 最“杀手”的功能，对 Agent 开发 web 应用特别方便。

- **场景:** Claude Code 写了一个 web 服务，运行在容器的 8000 端口。
    
- **Docker Desktop:** 你得去检查端口映射 (`-p 8000:8000`)，然后用 `localhost:8000` 访问。如果端口冲突，Agent 可能会报错。
    
- **OrbStack:** 它自动分配域名。容器名是 `my-app`，你（和 Agent）直接访问 `http://my-app.orb.local` 即可，甚至不需要手动做端口映射。这减少了 Agent 写 `docker-compose.yml` 时的复杂度和出错率。
    

### 4. 完美的 Socket 兼容性

Claude Code 本质上是调用 shell 命令。OrbStack 会自动在这个路径建立软连接：`/var/run/docker.sock`。 这意味着：**Claude Code 根本不知道自己在用 OrbStack，它以为自己在用 Docker。** 你不需要对 Agent 做任何特殊配置，直接用即可。




docker 桌面版本在 macOS上实在不好玩，还是 orbstack 好用，占用资源更少。



![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F15%2F15-13-32-2a249c0f1962200ac555310c5b417ae8-202409151513791-1c49ce.png)  
这两个地方其实把容器化最关心的一些问题都抽象出来了，太棒了！

从截图和提示信息来看，你当前使用的终端窗口是 **连接到 OrbStack 虚拟机 (MITS081) 内部的 shell**，而不是本地（macOS）的 shell。以下是判断的依据和解释：

1. **命令行提示 (Prompt)**：
    命令行前缀中出现了 `liuyishou@MITS081`，这通常意味着你当前的 shell 会话是在名为 "MITS081" 的机器（虚拟机）中运行。
    

    在本地 macOS 的终端中，提示符一般是 `username@hostname ~ %` 或类似格式，不会出现虚拟机名（如 MITS081）作为主机名。

    
2. **OrbStack 提示信息**：
    提示框中显示了"OrbStack update available!"等信息，这表明当前的会话是通过 OrbStack 启动和管理的环境，应该是虚拟机内部的 shell。
    
3. **目录结构挂载**：
    OrbStack 会将你的 macOS 主机的用户目录（如 `/Users/liuyishou`）挂载到虚拟机中，从而在虚拟机中也能访问你的本地文件。这就解释了你在虚拟机的 shell 中仍然能看到类似 macOS 路径的目录。
    

如果你还不确定，可以在当前终端下执行以下命令进行验证：

- `uname -a`：如果显示的是 Linux 内核信息和 arm64 架构信息，那说明是在虚拟机 (Linux) 环境中。
- `cat /etc/os-release`：查看系统版本信息，如果显示 Ubuntu 等 Linux 发行版信息，则说明你在虚拟机内。

总的来说，这个终端是 OrbStack 为你启动的虚拟机内部 shell。
