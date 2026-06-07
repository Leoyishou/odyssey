---
uid: 80538ebf-54bd-4417-8cd5-bfce973e767a
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡 Summary  

#### Webshare购买与配置

- 推荐Webshare的住宅IP服务，价格低廉，一个IP每月仅需1美元
- 购买时可选择静态住宅IP（适合养号）或动态IP（适合大量注册少量浏览），并可根据需求选择流量和国家
- Webshare提供IP替换功能，一个IP可免费更换50次，提高了性价比

#### AdsPower集成Webshare IP

- 在AdsPower中，通过“代理管理”添加Webshare的SOCKS5代理信息（IP、端口、用户名、密码）
- 新建浏览器环境时，选择已添加的Webshare IP，并可模拟不同的操作系统和浏览器内核
- 配置完成后，通过IP检测工具验证IP的纯净度，Webshare的IP通常风险指数为零，纯净度高

# 🧩 Cues



# 🪞Notes



Claude（特别是 Claude Desktop 版本）可以通过 AdsPower 提供的 Local API MCP Server 来操作 AdsPower。AdsPower 在 2025 年底推出了这个功能，专门让 AI 工具（如 Claude）用自然语言指令自动化管理浏览器环境（创建/打开/关闭 profile、更新指纹、自动化网页任务等）。这非常适合你作为开发者做账号矩阵时的批量操作。核心能力

- 用自然语言命令控制：比如直接对 Claude 说 “在 AdsPower 中创建一个 Chrome 134 的 Android UA profile 并打开它”，或 “关闭所有运行中的 AdsPower 浏览器”。
- 支持创建新 profile、启动/停止浏览器、修改指纹配置、自动化登录/刮取等。
- 基于 MCP（Model Context Protocol）协议，Claude 可以像“代理”一样控制 AdsPower。

前提要求

- 安装 AdsPower 本地客户端（已运行）。
- 安装 Node.js（v18 或更高）。
- 使用 Claude Desktop（Claude 的桌面应用，不是网页版 claude.ai）。如果你用网页版或 Claude Code 终端工具，目前不支持直接 MCP 集成，但可以用 Claude 生成脚本调用 AdsPower Local API。
- AdsPower 账号需启用 Local API（默认开启）。

设置步骤（5-10 分钟搞定）

1. 打开 Claude Desktop，进入 File > Settings > Developer > Edit Config。
2. 编辑配置文件（路径：macOS/Linux 是 ~/Library/Application Support/Claude/claude_desktop_config.json；Windows 是 %APPDATA%/Claude/claude_desktop_config.json）。
3. 在 json 中添加以下内容（根据系统选择）：
    - macOS/Linux：
        
        json
        
        ```json
        {
          "mcpServers": {
            "adspower-local-api": {
              "command": "npx",
              "args": ["-y", "local-api-mcp-typescript"]
            }
          }
        }
        ```
        
    - Windows：
        
        json
        
        ```json
        {
          "mcpServers": {
            "adspower-local-api": {
              "command": "cmd",
              "args": ["/c", "npx", "-y", "local-api-mcp-typescript"]
            }
          }
        }
        ```
        
4. 保存并重启 Claude Desktop。
5. 去 File > Settings > Connectors 检查是否连接成功（看到 AdsPower 的 API）。
6. 确保 AdsPower 客户端正在运行。首次连接后，可在 AdsPower 设置中关闭 “API verification”（仅在信任环境操作，谨慎）。

设置好后，直接在 Claude Desktop 聊天框输入自然语言指令，就能操作 AdsPower 了。官方有 demo 视频（YouTube 搜 “Automate AdsPower with Claude”）。注意事项

- 这属于官方支持的自动化方式，风险比纯脚本低，但仍需遵守平台（小红书/抖音）规则，避免批量违规操作。
- 如果你不用 Claude Desktop，可以用 Claude 生成 Python 脚本，直接调用 AdsPower 的普通 Local API（文档：[https://localapi-doc-en.adspower.com/），本地运行脚本控制。](https://localapi-doc-en.adspower.com/），本地运行脚本控制。)
- MCP Server 是开源的（基于 TypeScript + Playwright），你作为开发者可以二次修改。

这个方案对你做矩阵超级实用，能省大量手动时间。如果你试了遇到问题，贴错误我帮你 debug！