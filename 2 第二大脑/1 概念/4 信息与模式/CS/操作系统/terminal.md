---
aliases: [终端]
date created: 2024-07-14
date modified: 2025-10-21
uid: 8a43ba85-33f4-4c6b-a8dc-b0bf3125fa65
---

git clone 一个 github 上的仓库，太慢，经常连接失败，但是 github 官网流畅访问，为什么？- Khej-Trhyk 的回答 - 知乎  
https://www.zhihu.com/question/27159393/answer/3139968357

[warp](warp)

# Notes
## Teminal 的翻墙配置

您当前打开的 config 文件是 Shadowrocket 配置文件（iOS 代理工具），它不会影响 macOS 上 Cursor IDE 中的 terminal。

您的系统是 macOS（darwin 24.3.0），Cursor 中的 terminal（bash/zsh）需要通过其他方式配置代理。

---

## 💡 macOS Terminal 配置代理的方法

### 1️⃣ 通过环境变量配置（推荐）

在您的 shell 配置文件中设置代理环境变量：

编辑 ~/.zshrc（因为您用的是 zsh）：

# 代理配置

export http_proxy="http://127.0.0.1:7890"

export https_proxy="http://127.0.0.1:7890"

export all_proxy="socks5://127.0.0.1:7890"

# 不走代理的地址

export no_proxy="localhost,127.0.0.1,*.local,qunar.com,qunarzz.com"

# 代理开关函数（可选）

proxy_on() {

    export http_proxy="http://127.0.0.1:7890"

    export https_proxy="http://127.0.0.1:7890"

    export all_proxy="socks5://127.0.0.1:7890"

    echo "✅ 代理已开启"

}

proxy_off() {

    unset http_proxy

    unset https_proxy

    unset all_proxy

    echo "❌ 代理已关闭"

}

说明：

- 端口 7890 是常见的代理端口，根据您的实际代理工具调整

- no_proxy 可以设置不走代理的域名（比如 qunar.com）

- 可以通过 proxy_on 和 proxy_off 命令快速切换

### 2️⃣ 在 Cursor 中单独配置 Terminal 环境变量

在 Cursor 的设置文件中配置：

Settings JSON (Cmd + Shift + P → "Preferences: Open User Settings (JSON)")：

{

    "terminal.integrated.env.osx": {

        "http_proxy": "http://127.0.0.1:7890",

        "https_proxy": "http://127.0.0.1:7890",

        "all_proxy": "socks5://127.0.0.1:7890",

        "no_proxy": "localhost,127.0.0.1,*.local,qunar.com"

    }

}

这样只会影响 Cursor 中打开的 terminal，不影响系统其他 terminal。
