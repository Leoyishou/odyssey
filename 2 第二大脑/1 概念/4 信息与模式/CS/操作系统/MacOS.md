---
uid: cc30ddf9-9c90-489b-aae8-07878c388406
---
[[公证]]

1. User Agents: 你登录后运行的任务。[[守护进程 PID 1]]
2. Global Daemons: 系统启动时运行的 root 权限任务。
3. System Agents: macOS 自带的系统服务。



| 概念           | 说明                                |
| :----------- | :-------------------------------- |
| launchd      | macOS 的服务管理框架，相当于 Linux 的 systemd |
| LaunchAgent  | 用户级守护进程，登录后自动运行                   |
| LaunchDaemon | 系统级守护进程，开机就运行（需要 root）            |
| plist        | Property List，macOS 的配置文件格式（XML）  |

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

