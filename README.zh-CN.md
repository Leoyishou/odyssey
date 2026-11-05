# Odyssey

本地优先的知识编辑器：保留原生富文本与表格体验，加入 Obsidian 式链接和工作区导航。基于 quiet.md，采用 MIT，保留上游及第三方版权。

当前为开发快照 build 5，尚未完成全部功能。已提供双括号文档搜索、别名、纯文字行内链接、悬浮预览、未创建链接和表格引用；新增四栏工作区与新建、打开、最近文档库入口。开库前保存当前文档，目标库在独立窗口运行，原窗口保留。字体、配色及控件延续 quiet.md，仅调整工作区 UX。

标签/分屏、历史、持久块锚点、图谱和嵌入仍在开发，不宣称兼容 Obsidian 插件或 Markdown-only Vault。验收标准与版本回归流程见 docs/ACCEPTANCE.md 和 docs/RELEASE-CHECKLIST.md；旧版通过不能代替新版测试。

构建、测试、数据隔离、已知限制和许可说明见 [README.md](README.md)。默认数据位于 `~/Documents/Odyssey`。开发请使用独立的 `ODYSSEY_HOME`，不要使用真实笔记库。
