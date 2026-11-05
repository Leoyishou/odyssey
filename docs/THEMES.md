# 文档主题（CSS）

Quiet.md 的每个文档主题就是一份 CSS 文件。选中一个主题，就是选中了这份 CSS；在 设置 › 外观 点「编辑 CSS」，会用你的编辑器打开它，改完保存，所有窗口里的文档立即按新样式重画。

首次启动默认使用飞书（Feishu）主题，已有的主题选择会保留。下拉菜单按飞书、Craft、Notion、Obsidian 排列，只显示选项；选择后更新下方的页面预览。

- 内置主题（飞书、Craft、Notion、Obsidian）在 `assets/themes/`，每份都**写满了全部属性的当前值**，可以直接当参考。
- 编辑内置主题时，应用先把它完整复制到文档库的 `themes/<id>.css`，这份副本从此代替内置版本（菜单中显示「已修改」，「恢复默认」删除副本）。
- 「复制为新主题」生成 `themes/<id>-copy.css`，改名后就是你自己的主题。也可以直接把别人写的 `.css` 放进 `themes/` 文件夹，应用会自动读到。
- 写错的值、不认识的选择器或属性不会让主题失效：它们被跳过，并在设置里列为「未生效」。

## 文件结构

```css
:root {
  --theme-name: "我的主题";          /* 显示名，必填 */
  --theme-name-en: "My theme";       /* 英文界面下的名字 */
  --theme-author: "你的名字";
  --theme-description: "主题简介";    /* 保留为元数据，不在菜单中显示 */
  --theme-base: craft;               /* 块的画法：craft 或 feishu */

  --accent: hsl(254, 80%, 68%);      /* 自定义变量，下面用 var() 引用 */
  --color-red: #d44c47;              /* 命名颜色，见下文 */
}

.page { color: #37352f; font-size: 16px; line-height: 1.5; max-width: 708px; }
h1 { font-size: 1.875em; font-weight: 600; margin-top: 32px; }
blockquote { border-left: 3px solid var(--accent); }
```

- 支持 `/* 注释 */`、`var(--x, 备用值)`、逗号分隔的多个选择器；`@media` 等 at 规则整段忽略。
- 长度：`px`（= pt）、`em`（字号相对正文，其它相对当前元素的字号）、`rem`（16）、无单位的 `line-height`（倍数）。
- 颜色：`#rgb`、`#rgba`、`#rrggbb`、`#rrggbbaa`、`rgb()`、`rgba()`、`hsl()`、`hsla()`、`transparent`、`white`、`black`；颜色属性写 `inherit` 表示沿用文字颜色。
- `font-family` 取列表里第一个已安装的字体；`system-ui`、`-apple-system`、`sans-serif` 是系统字体，`ui-monospace`、`monospace` 是系统等宽字体。

## 选择器与属性

`--theme-base: feishu` 的主题按飞书的方式画块，只使用标「飞书」的属性；其它属性写了会列为「未生效」。

| 选择器 | 属性 | 说明 | 飞书 |
|---|---|---|---|
| `.page`（`body`） | `background-color` `color` `font-family` `font-size` `line-height` `caret-color` | 纸面、正文 | ✓ |
| `.page` | `max-width` | 「标准」页宽时正文栏的宽度；「较宽」「全宽」不受影响 | ✓ |
| `::selection` | `background-color` | 选中文字的底色 | ✓ |
| `p` | `margin-bottom` | 块与块之间的距离 | ✓ |
| `.title` | `font-family` `font-size` `line-height` `font-weight` `color` | 文档标题 | ✓ |
| `h1`–`h9` | `font-family` `font-size` `line-height` `font-weight` `color` `margin-top` `margin-bottom` | 各级标题 | 字号、行高、字重、上下间距 |
| `strong`（`b`） | `font-weight` `color` | 粗体 | |
| `a` | `color` `text-decoration`（`underline` / `none`）`text-decoration-color` | 链接；`underline` 时始终画下划线 | 仅 `color` |
| `a:hover` | `color` | 悬停的链接 | ✓ |
| `code` | `color` `background-color` `border-color` `border-radius` `padding-left` | 行内代码 | |
| `pre` | `font-family` `font-size` `line-height` `color` `background-color` `border-color` `border-radius` | 代码块 | 颜色与字体 |
| `.line-numbers` | `display`（`block` / `none`）`color` `background-color` | 代码块行号 | 颜色 |
| `.token.comment` `.token.keyword` `.token.tag` `.token.string` `.token.number` `.token.boolean` `.token.class-name` `.token.attr-name` `.token.function` | `color` | 代码高亮（沿用 Prism 的类名） | ✓ |
| `blockquote` | `border-left`（或 `border-left-width` / `-color`）`padding-left` `color` `font-style` | 引用：竖线、缩进、文字 | 仅竖线颜色 |
| `ul` | `padding-left` | 子级缩进 | |
| `ul li::marker` / `ol li::marker` / `li::marker` | `color` | 圆点 / 序号的颜色 | |
| `.task`（`input[type=checkbox]`） | `width`（= 高）`border-radius` `border-color` | 待办方框 | 仅 `border-color` |
| `.task:checked` | `background-color` `color` | 勾选后的底色、对勾颜色 | |
| `.task-done` | `color` `text-decoration`（`line-through` / `none`） | 已完成的待办文字 | 仅 `color` |
| `hr` | `border-top`（或 `border-top-style` `-width` `-color`） | 分隔线：`dotted` / `solid` / `dashed` | 仅颜色 |
| `table`（`td`） | `font-size` `line-height` `border-color` `border-radius` | 表格文字与线 | 仅 `border-color` |
| `th` | `background-color` | 表头底色 | ✓ |
| `.callout` | `border-radius` `border-width` `padding`（上 / 左） | 高亮块 | |

### `:root` 里的命名颜色

文档里的文字色、背景色、高亮块颜色都按名字存（红、橙、黄…），主题决定它们实际显示成什么颜色：

| 变量 | 用途 |
|---|---|
| `--color-gray` … `--color-purple` | 文字颜色（gray red orange yellow green blue purple） |
| `--highlight-light-red`、`--highlight-red` … | 文字高亮（浅一档 `light-*`，深一档不带前缀；灰色另有 `medium-gray`） |
| `--background-light-red`、`--background-medium-red` … | 块背景 |
| `--callout-light-red`、`--callout-medium-red` … | 高亮块底色 |
| `--border-red` … | 高亮块边框 |

### 其它细节：`--token-*`

文档里还有一些界面细节的颜色（块悬停、块选中、表格工具条、尺寸选择格等），在 `:root` 里用 `--token-<名字>` 覆盖，完整列表和当前值见内置主题文件末尾的「其它界面细节」一节。

## 范围

- 主题只改**文档本身**：侧栏、菜单、设置等界面不受主题影响。
- 暂无深色模式：纸面可以设成深色，但界面外壳仍是浅色。
- 块的整体结构（例如代码块的工具条、表格的行列工具）不由 CSS 决定。
