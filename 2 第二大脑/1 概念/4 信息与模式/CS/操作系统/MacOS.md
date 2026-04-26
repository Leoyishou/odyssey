---
uid: cc30ddf9-9c90-489b-aae8-07878c388406
---
[[公证]]

1. User Agents: 你登录后运行的任务。[[守护进程 PID 1]]
2. Global Daemons: 系统启动时运行的 root 权限任务。
3. System Agents: macOS 自带的系统服务。

**为什么推荐把主结构放在 Documents（文档）文件夹？** 这是 macOS 最推荐、最合理的位置，主要有以下 7 个核心原因：

### 1. **官方默认的“家”**
- Apple 就是把 Documents 设计成你存放**长期、有价值文件**的地方。
- 系统很多默认保存路径（如 Pages、Keynote、Numbers、Xcode 项目等）都会优先建议保存到 Documents。

### 2. **备份最安全、最方便**
- Time Machine 默认完整备份 Documents。
- iCloud Drive 可以选择只同步 Documents 里的特定文件夹，不会把整个电脑都同步（节省空间）。
- 如果电脑坏了、重装系统，这个文件夹是最容易找回的。

### 3. **和临时文件夹明确分开**

| 文件夹 | 用途 | 清理频率 |
| :--- | :--- | :--- |
| **Desktop** | 正在处理的最紧急文件 | 每天/每周 |
| **Downloads** | 新下载的东西 | 每周清理 |
| **Documents** | 长期存放、归档 | 很少清理 |

把重要结构放在 Documents，能避免桌面和下载文件夹越来越乱。

### 4. **Finder 侧边栏一键访问**
Documents 默认就在 Finder 左侧边栏最显眼的位置，你点一下就能直达整个文件夹体系，非常顺手。

### 5. **权限和搜索最友好**
- Spotlight、Alfred、Raycast 对 Documents 里的文件索引最完整，搜索最快。
- 权限问题最少，不会像 Library 或其他系统文件夹那样容易被 macOS 限制。

### 6. **跨设备、跨 Mac 同步最稳**
- 换新 Mac 后，用 Migration Assistant 或直接登录 iCloud，最先同步的就是 Documents。
- 如果用 iCloud Drive + “优化 Mac 存储”，大文件也能智能管理。

### 7. **符合程序员 + PM 的工作习惯**
- 代码、PRD、设计稿、会议纪要、Roadmap 这些都不是“临时文件”，属于长期资产，应该放在 Documents。
- 很多开发工具（VS Code、JetBrains、GitHub Desktop 等）默认也会建议把项目存在 Documents/Code 路径下。

---

**小建议**：
- 如果你文件非常多（> 100GB），可以把 Documents 整个移到外接 SSD 或 NAS，但**文件夹结构保持不变**。
- 不要把主结构放在 Desktop 或 Downloads，那两个文件夹系统默认是“临时区”。

想更极致一点的，还可以把 Documents 里的「00_当前工作」文件夹拖到 Finder 侧边栏最顶部，做成「今日战场」快捷入口。 你现在 Documents 里文件多吗？要不要我给你一个更精简的「极简版」结构，只保留程序员+PM 最常用的 5-6 个顶级文件夹？

| 概念 | 说明 |
| :--- | :--- |
| launchd | macOS 的服务管理框架，相当于 Linux 的 systemd |
| LaunchAgent | 用户级守护进程，登录后自动运行 |
| LaunchDaemon | 系统级守护进程，开机就运行（需要 root） |
| plist | Property List，macOS 的配置文件格式（XML） |

对比其他系统：
- Linux: systemd / cron
- Windows: 任务计划程序 / 服务
- macOS: launchd（推荐）/ cron（也能用但不推荐）

为什么 launchd 比 cron 好：
1. 系统原生，更稳定
2. 支持失败重试、依赖管理
3. 开机/登录自动恢复
4. 日志管理更方便

这块知识在做 后台任务、定时脚本、服务持久化 时会用到。

# Cues

[[LaunchControll]]