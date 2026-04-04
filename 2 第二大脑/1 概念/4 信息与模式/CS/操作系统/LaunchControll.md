---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 


LaunchControl 的收费逻辑在于它是一个专业级的 IDE（集成开发环境），只不过对象不是代码，而是 macOS 的服务配置。
如果你只是偶尔写一个简单的定时脚本，也许不需要它。
如果你需要管理复杂的自动化任务、守护进程，或者不想在 plist XML 的语法错误上浪费半小时，它的价格通常被认为是物有所值的。


| 状态       | 含义                                           | 常见原因 / 说明                                    | 颜色 / 背景提示  |
| -------- | -------------------------------------------- | -------------------------------------------- | ---------- |
| OK       | 一切正常，已加载且无问题                                 | launchd 加载成功，无错误                             | 绿色背景常见     |
| Running  | 当前正在运行中（有 PID，launchd 正在执行程序）                | 任务正在执行中，可能周期性或手动触发                           | 通常绿色或突出显示  |
| Unloaded | 已定义但未加载（launchctl unload 状态，或从未加载）           | 不会运行，直到手动 load 或满足启动条件                       | 灰色或中性      |
| Unknown  | LaunchControl 无法确定状态                         | 常见于系统任务、权限不足或 LaunchControl 读不到信息            | 通常黄色/橙色或灰色 |
| Error XX | 上次运行/加载失败，退出码（exit code）是 XX                 | 任务尝试运行但出错，具体看 XX 值（launchd 的 LastExitStatus） | 红色/橙色高亮    |
| Error 2  | 经典 POSIX 错误：文件未找到（No such file or directory） | Program 路径写错、脚本/可执行文件不存在、权限问题（chmod +x）      | 红色         |
| Error 78 | 配置错误（configuration error）                    | plist 语法/键值错误、参数错、环境变量问题、依赖缺失                | 红色         |
# 🧩 Cues

[[launchd]]

# 🪞Notes
