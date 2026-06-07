---
uid: 5f44b943-de3d-4ee0-ac77-6c1aea0cd44a
date created: 2025-07-19
date modified: 2025-10-31
---
# 💡 Summary  



# 🧩 Cues



# 🪞Notes
## nvm 是什么？

**Node Version Manager** - Node.js 版本管理工具

类比理解：

- **npm** 是管理项目依赖包的（装第三方库）
- **nvm** 是管理 Node.js 本身版本的（装 Node.js）

---

## 为什么需要 nvm？

|场景|问题|nvm 解决方案|
|---|---|---|
|**多项目开发**|项目 A 要 Node 14，项目 B 要 Node 18|一键切换版本|
|**版本升级测试**|想试试新版本但不想删旧版|多版本共存|
|**团队协作**|本地版本和服务器不一致导致 bug|统一版本管理|
|**学习新特性**|想体验最新 Node 特性|快速安装最新版|

---

## nvm 常用命令表

|命令|作用|示例|
|---|---|---|
|`nvm install <version>`|安装指定版本 Node|`nvm install 18.16.0`|
|`nvm install --lts`|安装最新长期支持版|-|
|`nvm use <version>`|切换到指定版本|`nvm use 16`|
|`nvm current`|查看当前使用版本|-|
|`nvm ls`|列出已安装的所有版本|-|
|`nvm ls-remote`|列出可安装的所有版本|-|
|`nvm alias default <version>`|设置默认版本|`nvm alias default 18`|
|`nvm uninstall <version>`|卸载指定版本|`nvm uninstall 14`|

---

## 实际使用场景示例

### 场景 1：切换项目版本

```bash
# 进入旧项目（需要 Node 14）
cd old-project
nvm use 14
node -v  # 输出：v14.21.3

# 进入新项目（需要 Node 18）
cd new-project
nvm use 18
node -v  # 输出：v18.16.0
```

### 场景 2：安装并设置默认版本

```bash
# 安装 Node 18 LTS 版本
nvm install 18

# 设为默认版本（每次打开新终端自动使用）
nvm alias default 18

# 验证
nvm current  # 输出：v18.16.0
```

### 场景 3：查看和管理版本

```bash
# 查看已安装的版本
nvm ls
# 输出示例：
#   v14.21.3
#   v16.20.0
# -> v18.16.0  (当前使用的版本)
#   default -> 18 (-> v18.16.0)

# 卸载不需要的旧版本
nvm uninstall 14
```

---

## nvm vs npm vs npx 对比

|工具|管理对象|作用范围|类比|
|---|---|---|---|
|**nvm**|Node.js 版本|系统级|管理 Python 2/3 的 pyenv|
|**npm**|项目依赖包|项目级|pip（Python 包管理）|
|**npx**|临时执行工具|命令级|直接运行命令而不安装|

---

## 项目中的最佳实践

### 1. 在项目根目录创建 `.nvmrc` 文件

```bash
# .nvmrc
18.16.0
```

团队成员只需执行：

```bash
nvm use  # 自动读取 .nvmrc 文件中的版本
```

### 2. 在 package.json 中指定 Node 版本

```json
{
  "engines": {
    "node": ">=18.0.0"
  }
}
```

---

## 常见问题

### Q: 装了 nvm 后，npm 命令找不到？

**A**: 因为每个 Node 版本都有自己的 npm，需要先 `nvm use <version>`

### Q: 每次打开终端都要重新 `nvm use`？

**A**: 设置默认版本：`nvm alias default 18`

### Q: nvm 和直接安装 Node 冲突吗？

**A**: 会冲突，建议先卸载直接安装的 Node，统一用 nvm 管理

---

## 安装 nvm（参考）

```bash
# macOS/Linux
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Windows 使用 nvm-windows
# 下载：https://github.com/coreybutler/nvm-windows/releases
```

---

## 总结

```Java
你的电脑
├── nvm (版本管理器)
│   ├── Node.js 14 
│   │   └── npm 6.x
│   ├── Node.js 16
│   │   └── npm 8.x
│   └── Node.js 18 ✓ (当前使用)
│       └── npm 9.x
│           └── 项目的 node_modules/
│               ├── react
│               ├── vue
│               └── ...
```

**一句话总结**：nvm 让你像切换衣服一样轻松切换 Node.js 版本！
