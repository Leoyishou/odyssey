# 脚本驱动（harness）

用 `FDL_SCRIPT` 回放一份逐行脚本来操作窗口，并截图。整个过程不碰用户的文档库：每次运行都先把 `lib_dir` 复制一份到 `work_dir/lib`，再用 `QUIET_MD_HOME` 指向这份副本。

```sh
cargo build --release
python3 tools/harness/run.py tools/harness/examples/scroll.txt            # 默认用 lib_clean，结果在 /tmp/fdl-run
python3 tools/harness/run.py my.txt /path/to/lib /tmp/my-run              # 自定义文档库和输出目录
FDL_BIN=/other/quiet_md python3 tools/harness/run.py my.txt
```

- 截图：`<work_dir>/<name>.png`。
- 日志：`<work_dir>/log`。性能数字用 `grep -a bench log` 查看。
- `lib_clean/` 是一个审查用的文档库，含一篇「全部 Block 样式审查」和几张示例图片。
- `winid.swift` 列出屏幕上 quiet_md 窗口的 window id（`swift winid.swift`），用来配合系统截图。

## 截图回归

```sh
python3 tools/harness/compare.py tools/harness/examples/menus.txt <旧版二进制> target/release/quiet_md
```

- 用同一份脚本分别跑两个版本，逐张逐像素比较截图。有差异时输出差异像素数和范围，并把差异图写到 `/tmp/fdl-compare/diff-<名字>.png`。
- 只移动代码、不改外观的重构（拆模块、换 token、共用组件），应当每张都是 `identical`。
- `examples/menus.txt` 依次打开「…」菜单和页宽子菜单、编辑模式菜单、斜杠菜单、块菜单、侧栏右键菜单、帮助菜单。
- 比较时光标不闪烁（`FDL_NOBLINK`）。脚本开头要先把指针移到空白处，否则真实鼠标的位置会以 hover 的形式出现在截图里。
- 旧版二进制可以这样编：`git worktree add /tmp/old <提交>`，在那里用 `CARGO_TARGET_DIR=<本仓库>/target cargo build --release`，再把二进制复制出来。

## 脚本命令

每行一条命令；`#` 开头的是注释。完整说明见 `src/devtools.rs` 顶部。

| 命令 | 作用 |
|---|---|
| `wait <ms>` | 等待，期间自己绘制帧（锁屏时也能出图） |
| `key <按键>` | 发送按键，如 `key cmd-b`、`key escape` |
| `type <文字>` | 输入文字（行尾空格有效） |
| `move x y` / `click x y` / `dclick x y` / `rclick x y` / `drag x1 y1 x2 y2` | 鼠标操作（窗口坐标） |
| `wheel x y dy` | 一次滚轮事件 |
| `shot <名字>` / `burst <名字> n` | 截图；连拍 n 张，间隔约 8ms |
| `mark <文字>` / `commit <文字>` | 输入法组合文字 / 提交 |
| `clip html|text <文件>` | 设置模拟剪贴板的内容，之后的 `key cmd-v` 粘贴它 |
| `pick <路径>[|<路径>…]` | 预先回答下一次打开 / 保存对话框，不弹出系统面板 |
| `resize w h` | 调整窗口大小 |
| `place x y` | 把窗口左上角放到主屏幕的 (x, y)，浮在其它窗口上（配合 `FDL_NOPARK` 拍桌面截图） |
| `deskshot <名字> [边距]` | 连同窗口下面的内容截图，四周多留「边距」pt（默认 48）。窗口下面先用 `shared-mock/guides/backdrop.swift` 铺一张壁纸，见 `shared-mock/guides/shots.sh` |
| `bench <n> <命令>` | 把一条命令跑 n 次，每次同步画一帧，输出耗时 |
| `record <文件.mp4>` / `record stop` / `glide` / `scroll` / `caption` | 录制演示视频 |
| `activate` | 让窗口成为 key window。**会抢焦点，一般不要用** |
| `quit` | 退出（`run.py` 会自动在末尾追加） |

环境变量：

- `FDL_SHOTS`：截图目录（`run.py` 已设好）；
- `FDL_TRACE`：输出事件跟踪；
- `FDL_NOPARK`：不把窗口挪到屏幕外；
- `FDL_WINDOW_CAPTURE`：改用窗口服务器截图，截图里带系统画的红绿灯；
- `FDL_NOBLINK`：光标不闪烁（`compare.py` 已设好）。
- `FDL_RECORD_FPS`：录屏帧率，默认 30。

## 使用规则

脚本跑在用户正在使用的电脑上，必须遵守以下规则：

- 不用真实的 ⌘C / ⌘V 系统剪贴板；`key cmd-v` 只在 `clip` 之后使用。
- 不抢焦点：不用 `activate`。窗口默认被挪到屏幕外渲染。
- 不点：
  - 设置 › 文档库的「在访达中显示」按钮；
  - 链接、「网页搜索」、打开链接类按钮（会启动浏览器）；
  - 打印（会弹出系统打印面板）；
  - 快速查看 / `qlmanage -p`。
- 不用 `open` 命令。
- 需要选文件时，用 `pick a|b` 代替系统对话框。
- 不要拿用户的真实文档库当 `lib_dir`。确实需要时只读复制一份，产物里也不要提交用户的内容。
