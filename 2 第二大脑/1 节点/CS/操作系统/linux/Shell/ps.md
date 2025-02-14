---
draw:
title: ps
date created: 2024-12-18
date modified: 2025-02-06
---

**在某种程度上可以类比于 Windows 的任务管理器**。


- **ps (process status)：** 这是 "process status" 的缩写，是一个用于报告当前进程状态的命令。它提供有关系统中正在运行的进程的信息。
- **aux：** 这是 ps 命令的选项组合，每个字母都有不同的含义：
    - **a (all users)：** 显示所有用户的进程，而不仅仅是当前用户的进程。
    - **u (user-oriented)：** 以用户为中心的格式显示进程信息。会显示每个进程的用户名、CPU 和内存使用情况等。
    - **x (processes without controlling tty)：** 显示没有控制终端的进程。这通常包括后台进程、守护进程等。
        

**所以， ps aux 命令的整体含义是：显示所有用户的所有进程（包括没有控制终端的进程），并以用户为中心的格式输出详细信息，例如：

- **USER:** 运行进程的用户名。
    
- **PID:** 进程 ID (Process ID)，每个进程的唯一标识符。
    
- **%CPU:** 进程使用的 CPU 时间百分比。
    
- **%MEM:** 进程使用的物理内存百分比。
    
- **VSZ:** 进程使用的虚拟内存大小。
    
- **RSS:** 进程使用的常驻内存大小 (Resident Set Size)，即实际使用的物理内存大小。
    
- **TTY:** 进程的控制终端。如果进程没有控制终端，则显示 ? 或者 -。
    
- **STAT:** 进程的状态，例如 S (Sleeping), R (Running), Z (Zombie) 等。
    
- **START:** 进程的启动时间。
    
- **TIME:** 进程使用的 CPU 总时间。
    
- **COMMAND:** 启动进程的命令。
