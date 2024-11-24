---
draw:
tags: []
title: npm
date created: 2024-11-08
date modified: 2024-11-12
---

## 目录

- node_modules/ - 依赖包目录
- package-lock.json - 依赖版本锁定文件

## 命令

 `npm create vue@latest`
- 创建一个新的 Vue.js 项目脚手架，类似 Spring Boot 项目脚手架
- 通过交互式命令行让你选择项目配置（TypeScript、路由、Pinia、测试工具等）
- 自动生成项目结构和基础代码
配置好构建工具（通常是 Vite）
相当于
`curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`


1. `rm -rf node_modules package-lock.json`
   - 这就像在 Maven 中删除 `target` 文件夹和 `.m2` 缓存一样
   - `rm` 是删除命令
   - `-rf` 是参数，表示递归(`r`)强制(`f`)删除
   - `node_modules` 相当于 Maven 的 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`0 文件夹，存放所有依赖包
   - `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`1 相当于 Maven 的 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`2，记录了确切的依赖版本信息


2. `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`3
   - 这就像 Maven 的 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`4
   - 它会根据 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`5（相当于 Maven 的 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`6）重新下载所有依赖
   - 会重新生成 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`7
   - 会重新创建 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`8 文件夹并下载所有依赖

简单来说，这两个命令组合起来就是：

1. 清理掉所有已下载的依赖和版本锁定文件
2. 重新下载所有依赖

这在依赖出问题时很常用，就像 Maven 项目出问题时我们经常用 `curl https://start.spring.io/starter.zip -d dependencies=web,data-jpa -d type=maven-project -o demo.zip`9 一样。
