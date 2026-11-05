# 存储格式

## 文档库目录

默认位置：`~/Documents/Quiet.md`（「文稿」里的独立文件夹，用户能直接看到和备份）。可以用环境变量 `QUIET_MD_HOME` 指定别处。

```
Quiet.md/
├── format.json          {"format": 1}：目录布局版本（store::FORMAT）
├── index.json           文档 id → Meta
├── index.json.bak       上一版 index.json
├── settings.json        窗口级 UI 设置（侧栏宽度等）
├── .lock                写锁（flock）：改写索引时短暂独占，不是单实例标记
├── docs/<id>.xml        每篇文档一份 DocxXML
├── assets/              粘贴或插入的图片、附件；assets/thumbs 是文件预览缩略图
└── trash/
    ├── index.json       id → Trashed { meta, deleted, root }
    └── <id>.xml
```

### Meta 字段

| 字段 | 含义 | 省略时 |
|---|---|---|
| `created` / `modified` / `opened` | unix 秒 | 必填；`opened` 默认 0 |
| `pinned` | 置顶 | false |
| `starred` | 收藏时间，0 表示未收藏 | 0 |
| `parent` | 父文档 id（文档树） | 顶层 |
| `order` | 同级排序键（最近一次移动的毫秒数） | 按 `created` 排 |
| `width` | 页宽：`standard`（820）/ `wide`（1020）/ `full` | standard |
| `catalogue_closed` | 目录已收起 | false |
| `reading` | 阅读模式 | false |
| `info` | 文档信息行：`{hidden, created, modified}` | 只显示修改时间 |
| `refs_hidden` | 隐藏引用列表 | false |

新字段一律带 `#[serde(default)]`，值为默认时不写出。这样旧版程序读新库会忽略它不认识的字段，新版程序读旧库会用默认值。

### 可靠性约定

- **原子写入。** 所有文件先写到 `<name>.tmp`，再 `rename` 覆盖原文件。崩溃时，磁盘上要么是旧内容，要么是新内容。
- **索引损坏后恢复。** 每次保存 `index.json` 前，先把旧文件复制成 `.bak`。读取时如果解析失败：
  - 坏文件改名为 `<name>.corrupt-<秒>` 保留，不会被覆盖；
  - 改用 `.bak`，并提示用户一次；
  - 没有可用的 `.bak` 时，用空索引。文档本身还在 `docs/` 里，只是置顶、层级这些信息会丢。
- **版本标记。** 打开库时，如果 `format.json` 里的版本比本程序新，会提示「请升级后再编辑」。改目录布局、会让旧版本误读的时候，要把 `FORMAT` 加一。
- **手放的文件也会出现。** 直接放进 `docs/` 的 `.xml` 文件（比如从飞书导出的 DocxXML）会出现在列表里。
- **保存失败不静默。**

## DocxXML

DocxXML 是飞书 / Lark 的文档 XML 格式。`crates/core/tests/fixtures/all-blocks.xml` 收录了每种 block 的规范写法，读入再写出必须逐字节不变（测试 `fixture_round_trips`）。

### 写出规则

- 顶层 block 每个占一行。子级换行，每层缩进两个空格。
- **表格单元格里的 block 写在一行内，不缩进。** 内容里的换行写成 `&#10;`（比如代码块的行），以免和排版用的换行混淆。
- 空的列表项如果有子段落，写成 `<li><p></p>…`。不加这个空 `<p>`，读回时子段落会被并进列表项。
- 折叠写成 `folded="true"`。图标和封面写在 `<title icon=… cover=… cover-x=… cover-y=…>` 上。

### 读入规则

- 只丢掉**源码排版产生的空白**：一个换行加缩进，以及嵌套 block 前面的空格和 Tab。其余空白都是内容，例如 `<p> </p>` 保留那个空格。
- 不认识的元素原样保留，写出时原样输出：block 级的是 `BlockKind::Raw`。
- 行内对象（公式、文件、按钮、日期提醒、@文档，以及不认识的行内元素）在文本里各占一个 U+FFFC，样式里存一个 `Inline { kind, xml }`：
  - `xml` 是读入时的原文，写出时原样输出，所以不认识的属性也不会丢；
  - `kind` 是读入时解析出来的类型，视图和导出只看它。
  - 编辑器新建或修改对象时，经 `docxml::to_inline` 先拼出规范 XML 再读回一遍，两者始终一致，存盘再读回也不变。

### 本地扩展

DocxXML 里没有、本应用自己加的写法。别的工具不认识这些写法，按上一节的规则它们只会被当作未知元素或属性：

- `<countdown duration start-time notify color timing/>`：倒计时。云端的倒计时是小组件，不在 DocxXML 里。
- `<time … is-whole-day="true">`：全天提醒。模型里有 `isWholeDay`，但 XML 里没有这个属性。没有此属性、且时间是本地 23:59 的提醒，也按全天处理。
- 图片和附件的本地文件写成 `path="…"`（云端用 `token` / `href`）。`<figure><source path=… name=…/></figure>` 表示视频或文件。

### 已知限制

- **单元格内的尾随空格。** 如果单元格里某个 block 有嵌套子级，它文本末尾的空格或 Tab 读回时会丢失。原因：旧版本写单元格时带缩进，读入要能兼容，所以必须丢掉嵌套 block 前的空白。fuzzer 的 `round_trip` 对这一点做了相同的预处理（`trim_before_nested`）。
- 代码块、分割线、图片等不能有子级的 block 如果在内存里带了子级，写出时这些子级会被丢掉。编辑器保证不会产生这种结构（见 ARCHITECTURE.md 的不变量）。

### Odyssey 扩展

- 文档引用 `<cite type="doc" doc-id="稳定文档 UUID" file-type="docx" alias="显示文字">标题快照</cite>` 可选 `alias` 属性独立于目标身份。旧引用不写此属性；未知属性仍遵从原有原文保留规则。
- Odyssey 默认库目录为 `~/Documents/Odyssey`，优先读取 `ODYSSEY_HOME`，测试 harness 兼容 `QUIET_MD_HOME`。目录布局版本未变。
