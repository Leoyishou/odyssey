---
comment_id: 297629cb
date created: 2025-04-07
date modified: 2025-04-07
draw: null
title: pnpm
---
pnpm 是一个 JavaScript 包管理器，类似于 npm 和 Yarn，但有一些独特的特点和优势：

1. **磁盘空间效率**：pnpm 使用硬链接和符号链接来共享依赖包，而不是像 npm 那样为每个项目复制一份依赖。这大大节省了磁盘空间。
2. **非扁平的 node_modules 结构**：pnpm 创建一个非扁平的 node_modules 结构，这意味着项目只能访问 package.json 中声明的依赖，避免了"幽灵依赖"问题。
3. **更快的安装速度**：由于其独特的依赖管理方式，pnpm 通常比 npm 和 Yarn 更快。
4. **严格的依赖管理**：pnpm 确保你的项目只使用显式声明的依赖，这有助于避免一些常见的依赖问题。
5. **向后兼容**：pnpm 支持 npm 的所有功能，使用方式也非常相似，命令基本一致。

pnpm 的名称来源于"performant npm"（高性能的 npm），反映了它的主要设计目标之一。
