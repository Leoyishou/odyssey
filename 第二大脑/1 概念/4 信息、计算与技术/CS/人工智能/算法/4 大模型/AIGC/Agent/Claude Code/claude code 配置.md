---
uid: d80cb825-078f-4441-a80c-76087821b057
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 



# 🧩 Cues



# 🪞Notes
## 你未使用的配置项

### 核心设置

|配置项|说明|
|---|---|
|`apiKeyHelper`|自定义脚本生成 API 认证|
|`cleanupPeriodDays`|非活跃会话清理周期（默认 30 天）|
|`companyAnnouncements`|启动时展示的公告信息|
|`env`|为每个 session 设置环境变量|
|`includeCoAuthoredBy`|是否在 git commit/PR 中加 co-authored-by 标注|
|`statusLine`|自定义状态栏显示|
|`outputStyle`|输出风格（如 `"Explanatory"`）|
|`disableAllHooks`|全局禁用所有 hooks|
|`forceLoginMethod`|限制登录方式（`claudeai` 或 `console`）|
|`forceLoginOrgUUID`|自动选择组织，跳过选择步骤|

### 权限设置（`permissions` 对象）

|配置项|说明|
|---|---|
|`permissions.allow`|允许的工具规则列表|
|`permissions.ask`|需确认的工具规则列表|
|`permissions.deny`|禁止的工具/文件规则列表|
|`permissions.additionalDirectories`|额外允许访问的目录|
|`permissions.defaultMode`|默认权限模式（`default`/`acceptEdits`/`plan`/`bypassPermissions`）|
|`permissions.disableBypassPermissionsMode`|设为 `"disable"` 可禁用跳过权限模式|

### 沙盒设置（`sandbox` 对象）

|配置项|说明|
|---|---|
|`sandbox.enabled`|启用 bash 沙盒隔离|
|`sandbox.autoAllowBashIfSandboxed`|沙盒中自动批准 bash 命令|
|`sandbox.excludedCommands`|排除在沙盒外运行的命令|
|`sandbox.allowUnsandboxedCommands`|是否允许命令逃逸沙盒|
|`sandbox.network.allowUnixSockets`|沙盒中允许的 Unix socket 路径|
|`sandbox.network.allowLocalBinding`|允许绑定 localhost 端口|
|`sandbox.network.httpProxyPort`|自定义 HTTP 代理端口|
|`sandbox.network.socksProxyPort`|自定义 SOCKS5 代理端口|
|`sandbox.enableWeakerNestedSandbox`|Docker 内启用弱沙盒（降低安全性）|

### MCP 服务器相关

|配置项|说明|
|---|---|
|`enableAllProjectMcpServers`|自动批准所有项目 `.mcp.json` 的 MCP 服务器|
|`enabledMcpjsonServers`|指定批准的 MCP 服务器列表|
|`disabledMcpjsonServers`|指定拒绝的 MCP 服务器列表|
|`allowedMcpServers`|MCP 白名单（managed-settings 用）|
|`deniedMcpServers`|MCP 黑名单（managed-settings 用）|

### 插件设置

|配置项|说明|
|---|---|
|`enabledPlugins`|✅ 你已使用。控制启用哪些插件|
|`extraKnownMarketplaces`|定义额外的插件市场来源|

### AWS/云相关

|配置项|说明|
|---|---|
|`awsAuthRefresh`|自定义 AWS 认证刷新脚本|
|`awsCredentialExport`|自定义 AWS 凭据导出脚本|

