---
uid: a9aa199b-4279-4f29-98dd-ae37d67620d2
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 在 Unix 系统中，在后台默默运行、没有图形界面、长时间驻留的进程被称为 **Daemon (守护进程)**。为了方便区分，人们习惯给这类程序加上 `d` 作为后缀。
> - **launchd (2005):** 名字很谦虚，聚焦于它的核心机制——**“怎么把进程启动起来”** (The Launcher)。
> - **systemd (2010):** 名字很霸气，聚焦于它的管辖范围——**“整个操作系统的状态”** (The System Manager)。
> 



# 🧩 Cues

[[守护线程]]


# 🪞Notes



### 1. 共同的后缀：`d` = Daemon (守护进程)

这是 Unix/Linux 世界最古老的命名传统。
- `ssh` (客户端) $\rightarrow$ `sshd` (服务端的守护进程)
- `http` (协议) $\rightarrow$ `httpd` (Apache Web 服务器守护进程)
- `mysql` (客户端) $\rightarrow$ `mysqld` (数据库守护进程)
    
所以，无论是 `system` + `d` 还是 `launch` + `d`，都在告诉用户：**“我是那个在后台一直运行的大管家。”**

---

### 2. `launchd`：强调“动作” (Launch / 启动)

**名字来源：** Apple 在 2005 年（Mac OS X Tiger）推出它时，核心卖点是 **"Launch-on-Demand" (按需启动)**。
- **设计初衷：** 在 `launchd` 之前，Unix 系统启动时会一口气把所有服务（打印机、蓝牙、Web服务）全开了，无论你用不用。这导致开机慢、费内存。
- **"Launch" 的含义：** Apple 的理念是——**不要提前运行服务，而是监听端口**。只有当你插上打印机或者访问 Web 端口的瞬间，`launchd` 才会**发射 (Launch)** 对应的进程。
- **潜台词：** “我是一个**发射器**。你需要谁，我就帮你把谁弹射出来。”
    
### 3. `systemd`：强调“范围” (System / 系统)

**名字来源：** Lennart Poettering 在 2010 年发布它时，野心要大得多。他不仅仅是想做一个“启动器”，他是想做一个 **"System Manager" (系统管理器)**。
- **设计初衷：** 它的作者认为 Linux 之前的初始化系统太松散了，他想要一个能够接管**整个系统状态**的超级软件。
- **"System" 的含义：** 它不仅仅管进程启动，它还管：
    - 挂载文件系统 (`systemd-mount`)
    - 用户登录 (`systemd-logind`)
    - 系统日志 (`journald`)
    - 网络解析 (`systemd-resolved`)
    - 时间同步 (`systemd-timesyncd`)
- **潜台词：** “我就是**系统本身**。除了内核 (Kernel) 是董事长，剩下的整个公司（System）都归我管。”

**有趣的历史：** `systemd` 的作者其实公开承认过，`systemd` 的很多核心理念（特别是 Socket Activation 按需启动）正是**抄作业**抄的 `launchd`。只不过后来 `systemd` 的野心膨胀得比 `launchd` 大多了。