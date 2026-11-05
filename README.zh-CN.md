# Odyssey

本地优先 + 类飞书原生富文本/表格编辑 + Obsidian 式双链体验。

这是 **build 4 阶段源码**，基于 [quiet.md](https://github.com/s1dashu/quiet-md) 的 `52afb284d7132dba1a1efb2944a456aaed9d44f3`，不是全部功能完成版。

已实现正文/表格双链补全、别名、预览跳转、未创建目标、纯文字链接和 Esc 取消候选修复。继承 quiet 的富文本及表格编辑。默认库独立为 `~/Documents/Odyssey`，已禁用上游自动更新。

Vault 管理、四栏工作台、标签与分屏、持久块锚点、嵌入和图谱仍在开发规划中。本阶段使用 DocxXML 存储，不支持直接打开 Obsidian Markdown 库或加载其插件。

构建与测试命令见 [README](README.md)。使用临时测试库，先备份重要笔记。本仓库暂不提供公证安装包，Windows 尚未针对本快照验证。

产品边界见 [PRODUCT.md](PRODUCT.md)：四栏重新组织功能，quiet 原有交互保持逻辑；非 Publish 功能免费，Publish 本阶段不做。

MIT 开源，保留上游版权；依赖及素材分别遵循 [第三方许可](THIRD_PARTY.md)。
