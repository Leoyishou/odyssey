---
uid: b66f7876-4196-4694-8371-16a63ddeb450
date created: 2026-10-30
date modified: 2026-10-30
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 


## CDP 是啥

⏺ Chrome DevTools Protocol — Chrome 浏览器内置的远程调试协议。

本质就是 Chrome 暴露了一个 WebSocket 接口（默认 ws://127.0.0.1:9222），外部程序可以通过这个接口控制浏览器：导航页面、点击元素、截图、读取 DOM 等。

和 --debug 模式的关键区别：

|          | --debug（cookie 快照）     | --cdp（直连）             |
| :------- | :------------------------- | :------------------------ |
| 原理     | 复制 cookie → 注入新浏览器 | 直接操控正在运行的 Chrome |
| 认证     | 只有 cookie，部分站点不认  | 完整 session，啥都认      |
| 隔离性   | 高（独立进程）             | 低（操作的是真实浏览器）  |
| 典型失败 | Google 服务要求重新验证    | 无                        |

简单说：--debug 是「复印你的身份证去办事」，--cdp 是「直接拿你本人去办事」。上次发 CWS 就是因为 Google 不认复印件，才加了 --cdp 模式。

# 🧩 Cues



# 🪞Notes
