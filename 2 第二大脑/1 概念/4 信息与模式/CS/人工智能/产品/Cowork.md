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



# 🧩 Cues



# 🪞Notes

Claude Cowork 里说的“沙箱隔离”（sandbox isolation），简单讲就是 Anthropic 故意把 Claude 的执行环境和你的 Mac 主系统隔离开来，防止它乱动不该动的东西，或者万一出 bug/被 prompt injection 攻击也不会搞坏你的整个电脑。具体机制（基于官方文档、help center 和社区逆向分析，2026 年 1 月最新情况）：

- 核心是虚拟机（VM） + 文件系统挂载：  
    当你启动 Cowork 任务时，Claude Desktop app 会用 Apple 的 Virtualization Framework（VZVirtualMachine）快速启动一个轻量级 Linux-based 微型虚拟机（micro-VM）。这个 VM 是完全隔离的，和你 macOS 主系统跑在不同的“空间”里。  
    你选的那个文件夹（比如 ~/Documents/MyProject）会被“挂载”（mount）到这个 VM 里面，作为 VM 的工作目录。Claude 只能在这个挂载的文件夹里读/写/创建/删除文件——它根本看不到你 Mac 上其他地方的文件、系统文件夹、Downloads、Desktop、Photos 库啥的。
- 隔离的层面：
    - 文件系统隔离：VM 看不到 / 改不了 VM 外部的任何东西（类似 Docker 容器或 Linux bubblewrap 的效果）。官方用 OS 级原语强制执行这个限制。
    - 网络隔离：默认有限制（除非你明确允许，比如通过 Chrome 扩展连浏览器），防止它偷偷上传你的文件或下载恶意东西。
    - 进程隔离：Claude 跑的命令、脚本、子进程都在这个 VM 里执行，不会直接触碰你 Mac 的内核或敏感进程。
    - 权限控制：你每次启动任务都要手动选文件夹（或复用之前的），不是全局授权。重大操作（如永久删除文件）现在还会弹窗问你确认（最近更新加的 delete protection）。
- 和 computer-use-demo 的区别：
    - demo 是完整 Docker 沙箱 Ubuntu 桌面（Claude 通过 API 控制鼠标键盘看截图），更“重”，更隔离，但慢且贵。
    - Cowork 的沙箱更“轻”+ 直接：文件操作是 native 的（VM 里直接读写挂载文件夹），速度快很多，tokens 省，但自主性集中在文件/命令上，不是全 GUI 控制。
- 实际安全含义（社区和官方都反复强调的）：
    - 好的一面：沙箱防住了“越界”风险——Claude 再怎么幻觉或被坏 prompt 诱导，也改不了你 Mac 的系统文件、隐私数据（除非你自己授权了敏感文件夹）。
    - 风险还在：在你授权的文件夹内，它有完整读写权限。如果 prompt 写得不好，它可能删错文件、覆盖重要东西、搞乱结构。所以官方建议：
        - 先用测试文件夹玩
        - 备份重要数据
        - 别授权含财务/密码/个人隐私的文件夹
        - 监控进度，随时暂停/取消

总的来说，这个“沙箱隔离”就是 Anthropic 为了让 Cowork 实用又不至于太危险，搞的一个折中方案：给你真实文件访问能力，但把破坏范围严格限制在你指定的文件夹 + VM 内部。比完全无沙箱的本地 agent 安全多了，但还是需要你自己负责任地用（很多人第一次试都只给个空文件夹测试）。你试 Cowork 的时候授权了啥文件夹？遇到过啥奇怪行为没？可以分享下，我帮分析。

![](https://pbs.twimg.com/profile_images/1550495588071923712/bCXl0Wbt_normal.jpg)

![](https://pbs.twimg.com/profile_images/2005801140626284544/m3w4wR23_normal.jpg)

![](https://pbs.twimg.com/profile_images/2010367809654755328/tI2tx6fQ_normal.jpg)

9 posts

![](https://t3.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=https%3A%2F%2Fsimonw.substack.com%2Fp%2Ffirst-impressions-of-claude-cowork&size=256)

![](https://t3.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=https%3A%2F%2Fventurebeat.com%2Ftechnology%2Fanthropic-launches-cowork-a-claude-desktop-agent-that-works-in-your-files-no&size=256)

![](https://t3.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=https%3A%2F%2Fwww.datacamp.com%2Ftutorial%2Fclaude-cowork-tutorial&size=256)

19 web pages