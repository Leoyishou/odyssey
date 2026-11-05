# vendor 补丁

`vendor/gpui` 是 crates.io 的 gpui 0.2.2，`vendor/taffy` 是 taffy 0.9.0，都打了本地补丁。项目通过 `Cargo.toml` 的 `[patch.crates-io]` 使用这两个目录。补丁处都带 `quiet.md:` 注释；完整列表以下表和 diff 为准。

和上游对比：

```sh
R=~/.cargo/registry/src/index.crates.io-*/
diff -ru $R/gpui-0.2.2/src vendor/gpui/src
diff -ru $R/taffy-0.9.0/src vendor/taffy/src
```

升级上游时，按下表逐条重新打补丁，并在补丁处保留注释。

## GPUI 0.2.2

| # | 文件 | 改动 | 用途 |
|---|---|---|---|
| 1 | `platform/mac/text_system.rs` | 光栅化时显式打开字体平滑（`set_allows_font_smoothing(true)`、`set_should_smooth_fonts(true)`） | 与原生 AppKit / Catalyst 文字一致（笔画墨量 5.73 vs 5.74 / 像素宽）；关闭时少约 15% |
| 2 | `platform/mac/text_system.rs` | 字形边界改用和光栅化相同字号的 CTFont 来量 | 系统字体会随字号切换光学尺寸（SF Text → SF Display），原来大标题字形左侧会被切掉一条 |
| 3 | `platform/mac/text_system.rs` | 名字以 `KaTeX_` 开头的字体，即使没有 `m` 字形也允许加载 | KaTeX 的符号字体（Size1–4、AMS、Caligraphic、Script）没有小写字母，原来会被拒绝加载 |
| 4 | `text_system/line_wrapper.rs`、`line_layout.rs` | 中文避头尾：`is_no_break_before`（闭合标点不在行首）、`is_no_break_after`（开标点不在行尾），两处换行算法都使用 | 正文、搜索摘要的中文换行符合中文避头尾规则 |
| 5 | `text_system/line.rs` | `paint_rows` / `paint_background_rows`：按显式给出的每行顶部位置绘制 | 行内含公式或按钮时各行高度不同 |
| 6 | `style.rs`、`styled.rs`、`taffy.rs` | `grid_col_widths(Vec<f32>)`：以像素给出网格列宽，覆盖 `grid_cols`；`repeat` 的字面量补上类型 | 表格列宽 |
| 7 | `elements/list.rs` | `ListState::estimated_scroll_metrics` / `scroll_to_estimated_offset`：未量过的行按平均行高估算 | 长文档的滚动条和拖动滚动条 |
| 8 | `platform.rs`、`platform/mac/platform.rs` | `PathPromptOptions.extensions`：限制 NSOpenPanel 可选的文件类型 | 导入对话框只列出 Markdown / 文本和 .xml 文件 |
| 9 | `platform/mac/window.rs` | `mouseDownCanMoveWindow` 返回 NO，新增 `start_window_move`（延后执行 `performWindowDragWithEvent`） | 透明标题栏不再自己拖动窗口，也不再盖住应用按钮；应用自己标出可拖动区域 |
| 10 | `platform/mac/metal_renderer.rs`、`window.rs`、`platform.rs` | `render_to_bgra` / `Window::render_offscreen`：把场景渲染到离屏纹理再读回；`Window::needs_draw` | 脚本截图和录屏，窗口被遮挡或锁屏时也能用（devtools） |
| 11 | `window.rs` | `DispatchEventResult` 改为 `pub`（`Window::dispatch_event` 的返回类型） | devtools 回放事件时调用 `dispatch_event` |
| 12 | `platform/mac/text_system.rs` | 合成斜体：要斜体而字体没有斜体字面时（含 CoreText 回退到的字体），换成一个「倾斜孪生」FontId，光栅化时错切 14°（WebKit 的合成倾斜角），字形边界相应加宽 | 中文斜体像浏览器一样倾斜；原来苹方没有斜体，中文斜体显示为正体 |
| 13 | `text_system/line.rs` | 文字与背景装饰的场景排序边界包含居中、右对齐后的字形位置及超宽文字 | 图片描述不再穿透浮动工具栏，避免工具栏看起来透明 |
| 14 | `platform/mac/text_system.rs`、`platform/mac.rs`、`platform.rs` | `font_ids_by_postscript_name` 一个名字对应多个 FontId，CoreText 回退来的字体按字形数挑出同一份字体；新增 `check_fallback_glyphs` 供测试调用（`editor::text::tests`） | 系统回退用的苹方（PingFangUI.ttc）和下载的 PingFang.ttc 都叫 `PingFangSC-Regular`，字形表不同；原来按名字查到另一份，编辑器里的中文画成错字 |
| 15 | `platform/windows/directx_renderer.rs` | 调试构建编译着色器时，路径按宽字符串传给 `D3DCompileFromFile`（原来 `to_str().unwrap()`） | 仓库放在含中文等非 ASCII 字符的目录下也能跑调试构建 |
| 16 | `platform/windows/clipboard.rs` | 读剪贴板时文字、图片、复制的文件一起读（原来只取第一个认得的格式）；新增位图（`CF_DIB`，转成 PNG）；复制的图片文件读成图片，其他文件的路径按行分开（原来连成一串） | 截图工具放进剪贴板的位图能粘贴，不再贴出截图文件的路径；从资源管理器复制的图片能粘贴 |

## taffy 0.9.0

| 文件 | 改动 | 用途 |
|---|---|---|
| `compute/grid/track_sizing.rs` | 「超出限制」阶段只对受影响的轨道分配空间，增长上限只看 `fit-content()` 参数。这是从 taffy 0.13 移植的修复 | 跨多个 auto 行的合并单元格在 min-content 约束下原来不会撑高这些行 |

## 删掉的上游文件

`gpui/examples/image/black-cat-typing.gif`（4.3 MB）只在 `examples/gif_viewer.rs` 和 `examples/opacity.rs` 里用到，这两个示例不参与本项目的构建；为了减小仓库体积删掉了。
