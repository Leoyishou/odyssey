---
comment_id: 996af7b8
date created: 2024-11-25
date modified: 2025-02-06
draw: null
title: gitigonre
---
- `/node_modules` - 依赖包体积大，而且可以通过 package.json 重新安装
- `dist` 和 `build` - 这是构建产物，不需要提交到代码库
- 环境文件 (.env.local) - 通常包含敏感信息如 API 密钥
- 日志文件 - 这些是本地调试信息，对其他开发者没有用
- 编辑器配置 (.idea,.vscode) - 这些是个人开发环境的配置
- 系统文件 (.DS_Store) - 这些是操作系统自动生成的文件
