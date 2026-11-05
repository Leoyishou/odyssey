# Odyssey

**本地优先 + Obsidian 双链体验 + 类飞书编辑体验。**

Odyssey 是一个本地优先的知识编辑器，将 Obsidian 式双链与工作区导航，和原生富文本、表格编辑结合。基于 [quiet.md](https://github.com/s1dashu/quiet-md) 二次开发，采用 Rust + GPUI。

[下载预览版](https://github.com/Leoyishou/odyssey/releases) · [English](README.en.md) · [产品定位](PRODUCT.md)

## 愿景与设计原则

**现阶段优先面向国内用户**，以中文产品介绍、使用说明与版本公告为默认，优先打磨中文写作和本地知识管理体验。

让个人知识留在自己手里，写起来顺手、连接起来自然，长期积累成可检索、可复用的知识网络。表格是核心，单元格与正文拥有同等的知识连接能力。

- **本地优先**：本地笔记不默认上传，优先确保保存可靠、编辑流畅与知识连接顺畅。
- **UI 延续 quiet.md**：保留字体、配色、控件和视觉风格；原有富文本与表格编辑动作逻辑保持。
- **UX 按四栏重组**：工具栏 → 文件导航 → 主编辑器 → 大纲与引用辅助栏；外围导航参考 Obsidian。
- **基础功能免费**：未来仅 Publish 发布服务收费，其余功能全部免费。当前专注基础体验，暂不开发 Publish、支付或托管服务。

## 当前进展

当前源码为 **build 6 修复预览版**，完整功能仍在开发与验收中。Cargo 包版本暂沿用上游的 `0.1.5`，不代表 Odyssey 以该版本号正式发布。

已实现并在 macOS Apple Silicon 上验证过的主要场景：

- 继承 quiet.md 的原生富文本与表格编辑。
- 输入 `[[`、`【【`、`［［` 弹出文档候选，保留原有 `@` 入口。
- 链接显示别名、通过 `⌘⇧K` 将选中文字转为链接、未创建链接的持久保存及点击创建。
- 行内链接仅改变文字颜色，无前置文档图标或胶囊背景；支持悬浮预览、反向引用与出站引用列表。
- 表格内建链、反链回到来源单元格、重命名保持引用身份，以及撤销重做、保存重开场景。
- 四栏工作区复用 quiet.md 的视觉组件与编辑动作，右侧栏承载大纲与引用列表。
- 新建库、打开库、最近库；开库前保存当前文档，目标库在独立窗口运行，原窗口保留；独立库进程及最近使用库恢复。
- 独立的 Odyssey 应用身份和默认数据目录，上游自动更新已禁用。
- build6：混合 XML/Markdown 库拒绝写入；新默认库使用应用数据目录，避免与 `Documents/odyssey` 大小写碰撞。
- build6：先预览再将 Markdown 复制导入新库，保留原文归档、目录层级页、可解析链接和被引用附件；转换限制写入报告。
- build6：右上角图标独立开合辅助栏；搜索和新建位于左侧，文档菜单位于正文标题栏右侧。

**尚未完成的能力**：完整工作区恢复、标签页与分屏、持久块/标题/单元格锚点、文档级多别名、未解析标题消歧、未链接提及、嵌入与循环保护、图谱，以及完整导入导出和性能回归。

## Markdown / Obsidian 复制导入（build6）

从“文档库 → 复制导入”选择源目录，先查看文件数与转换提示，再选择一个**尚不存在、位于源库之外**的新目录。源文件不会被修改。基础 Markdown 转为原生文档；原文逐字节归档到 `.odyssey-import/source`，完整报告在 `.odyssey-import/report.json`。文件夹映射为目录页，已解析链接使用稳定文档 ID，被引用的本地附件复制到新库。

这不是直接编辑 Obsidian 格式库。YAML 属性只保留代码文本；插件、文档嵌入、标题/块锚点、同名歧义和缺失目标会保留原文并提示，不保证无损。隐藏目录与符号链接跳过；混合库中的旧 XML 不会随 Markdown 导入，仍留在源目录。预览上限为每篇 16 MiB、Markdown 合计 256 MiB、每附件 64 MiB、附件合计 512 MiB。大型真实库的大规模导入尚未验证。

主编辑区使用富文本，当前尚未实现 Markdown-only 的 Obsidian Vault，也不宣称兼容 Obsidian 插件生态。长行内链接仍是独立编辑对象：宽度过大时整体移到下一行，再在对象内部换行，周围文字尚不能连续流入对象内部。

## 构建与测试

使用 `rust-toolchain.toml` 固定的 Rust 版本。macOS 需要 Xcode 与 Metal 工具链；如提示缺少 Metal 工具链，可执行 `xcodebuild -downloadComponent MetalToolchain` 安装。

```sh
cargo build --release --locked
ODYSSEY_HOME=/tmp/odyssey-scratch ./target/release/quiet_md
scripts/check.sh
python3 scripts/package-odyssey.py
```

打包脚本生成 `dist/Odyssey.app`，使用本地 ad-hoc 签名；脚本不负责安装、发布、Apple 公证或退出正在运行的应用。当前预览包尚未完成 Apple 开发者签名与公证。上游的发布和上传脚本已在此分支禁用。

build6 新默认库为 `~/Library/Application Support/Odyssey/Vaults/Default`，已有纯 Odyssey 旧库保持原位；混合 Markdown 库会拒绝写入并提示复制导入。可用 `ODYSSEY_HOME` 指定独立目录；保留 `QUIET_MD_HOME` 以支持上游测试工具。**开发测试请使用独立测试库，不要指向真实笔记。** 数据采用 DocxXML 与稳定文档 UUID；纯文本导出不能无损保留所有富文本和表格结构。

- [功能验收清单](docs/ACCEPTANCE.md)
- [版本发布检查流程](docs/RELEASE-CHECKLIST.md)
- [GUI 测试工具说明](tools/harness/README.md)

自动检查涵盖工作区测试、内置指南往返校验与 clippy。大版本需运行自动化与关键界面回归，旧版本通过记录不能代替新版验证。公开源码快照不包含个人文档库、截图与本地会话日志。

## 开源许可与致谢

采用 [MIT 许可证](LICENSE)，保留原 quiet.md 版权声明。第三方依赖与素材遵循各自许可证，详见 [THIRD_PARTY.md](THIRD_PARTY.md)。

Odyssey 应用图标为原创罗盘设计。上游商业界面图标已替换为 ISC/MIT 编辑器图标与原创几何图形；quiet.md 名称及应用品牌归原项目所有。

上游源码基点：`52afb284d7132dba1a1efb2944a456aaed9d44f3`。分支说明见 [ODYSSEY.md](ODYSSEY.md)，存储扩展见 [docs/FORMAT.md](docs/FORMAT.md)。
