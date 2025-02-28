---
draw:
title: WEB容器
date created: 2023-12-25
date modified: 2025-02-06
---

forward proxy, reverse proxy

从哪儿到哪儿算正向？
---

nginx 和 OR 有啥区别？

"Nginx"（发音为 "Engine-X"）和 "OR"（通常指 OpenResty，一种基于 Nginx 的开源 web 平台）之间的主要区别在于它们的功能和用途。

1. [Nginx](Nginx.md):
    - Nginx 是一种高性能的 HTTP 和反向代理服务器。
    - 它也被用作邮件代理服务器。
    - Nginx 以其高性能、稳定性、丰富的功能集、简单的配置和低资源消耗而闻名。
    - 它通常用于提供静态内容、作为负载均衡器或反向代理服务器。
    - Nginx 是独立的软件，可以在多种操作系统上运行。
2. [OpenResty](OpenResty.md):
    - OpenResty 是一个基于 Nginx 实现的全功能 web 平台。
    - 它集成了 LuaJIT（一个高性能的 Lua 编译器），允许使用 Lua 脚本语言进行开发，使得在 Nginx 服务器上运行的动态 web 应用更加强大和灵活。
    - OpenResty 旨在构建动态的 web 应用、web 服务和动态网关。
    - 它通过 Nginx 模块扩展了 Nginx 的功能，提供了更多的网络协议支持和脚本能力。
    - OpenResty 经常用于需要高度定制和性能要求的 web 应用场景。
简而言之，Nginx 是一种高效的 web 服务器和代理工具，而 OpenResty 是基于 Nginx 的、用于构建动态 web 应用和服务的平台，它通过 Lua 脚本增强了 Nginx 的功能。OpenResty 通常被视为 Nginx 的一个扩展或补充，提供了更多的灵活性和开发功能。
