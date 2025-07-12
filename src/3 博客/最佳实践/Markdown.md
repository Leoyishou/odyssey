---
date created: 2024-03-28
date modified: 2025-07-10
uid: 617cb0e5-a9df-4ab7-b065-2d5858097804
---

Markdown 是 bit 赠予世界的笔，就像 code style 一样，Linter 是个很好的风格固化工具！

<!-- more -->

微软的工具：[**https://x.com/karminski3/status/1913760024435863982?s=46**](https://x.com/karminski3/status/1913760024435863982?s=46)

- [ ] 探索一段时间 linter 后把配置信息粘贴到这里就可以了 ⏰ 2024-05-07 📅 2024-05-07

大标题和小标题之间要有内容过渡

## 标题

标题要简短，结尾不带标点符号。

（源码上）标题前后空一行（段前距和段后距）。

### 层级

正文的标题从 ```Java  
段落1（这里是第一行）

段落2（这里其实是第三行）

```2 开始，```Java  
段落1（这里是第一行）

段落2（这里其实是第三行）

```3 要留给整篇文章的标题（也可以不写）。这样的大纲结构也可以很方便地转换为思维导图（Markdown to [Xmind](https://www.xmind.cn/) / [MindNode](https://mindnode.com/) / [幕布](https://mubu.com/) / [百度脑图](http://naotu.baidu.com/)……），也符合 SEO（搜索引擎优化）的规则。

谨慎使用四级标题（除非是长文章）。如果三级标题下有并列性的内容，可以使用加粗、有序列表或无序列表代表末级标题。

标题要避免孤立编号（即同级标题只有一个）

> Markdown 是 bit 赠予世界的笔

1. 手工排序
2. 自动排序
3. 巧妙！

## 序列号

序列号之前空一行，不然有的 markdown 渲染器不能正常缩进

序列号之中如果有代码块，需要将代码块整体向后缩进，不然有的 markdown 渲染器会截断序列号

## 空格

中文和数字或者英文之间应该有一个空格的距离

## 中文博客排版指南

### 标题

正确：（Markdown 语法）

```markdown
# 文章的标题

## 文章内容的一级标题

### 文章内容的二级标题

#### 文章内容的三级标题
```

原则：

（1）文章内容的标题一般用 ```Java  
段落1（这里是第一行）

段落2（这里其实是第三行）

```4 和 ```Java  
段落1（这里是第一行）

段落2（这里其实是第三行）

```5。为保证层级的简单，请尽量避免出现 ```Java  
段落1（这里是第一行）

段落2（这里其实是第三行）

```6 的标题。

（2）为保证标题的连贯性，```Java
段落1（这里是第一行）

段落2（这里其实是第三行）
```7 的标题下不要直接出现 ```Java
段落1（这里是第一行）

段落2（这里其实是第三行）
```8 的标题。

### 图片

#### 1、图片大小

为方便网络传输，且保证清晰度，图片大小尽量控制在**100 KB**至**500 KB**之间。

如果图片过大，可以使用网址 <https://tinypng.com/> 进行压缩；如果图片不够清晰，请重新制图。

#### 2、图片格式

![](https://img.oldwinter.top/20190908_2238.png)

静态图片用 png、jpg 格式。

动态图用 gif 格式。gif 动图可以使用软件 [LICEcap](https://www.cockos.com/licecap/) 录制屏幕生成。

**gif 图的帧数不要超过 300 帧**，否则无法上传到公众号。一般情况下，gif 图的大小控制在 500kb 以内，基本能符合要求。

#### 3、图片文件名

图片的文件名使用**当前时间戳**，格式：```Java
段落1（这里是第一行）

段落2（这里其实是第三行）
```9。格式举例：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```

#### 4、截图工具

- 最常用的截图工具：[Snipaste](https://zh.snipaste.com/)
- 截图时可带阴影的截图工具：[Xnip](https://zh.xnipapp.com/)、[iShot](https://sspai.com/post/57302)
- gif 录屏工具：[LICEcap](https://www.cockos.com/licecap/)

#### 5、流程图制作工具

- 流程图、思维导图（在线）：<https://www.processon.com>
- 思维导图（在线）：<https://naotu.baidu.com>
- 思维导图（本地）：<https://www.xmind.cn>
- 列清单、思维导图（在线）：<https://mubu.com>

#### 6、图片素材网站

- <https://unsplash.com>

Unsplash 是国外知名度最高的免费高清网站（图片素材免费、无版权），是现代年轻人喜欢的 ins 简约风，到处都充满生活剪影。

- <https://pixabay.com>

免费正版高清图片素材库。

### Markdown 规范

#### 1、换行

每两段的中间，需要空出一行。

正确：

```Java
段落1（这里是第一行）

段落2（这里其实是第三行）
```

错误：

```Java
段落1（这里是第一行）
段落2（这里是第二行）
```

#### 2、四级标题

文中的四级标题，请不要用 ```Java  
段落1（这里是第一行）

段落2（这里是第二行）

```0，建议使用加粗文本即可。

正确：

```Java
**我是四级标题**：
```

错误：（四级标题渲染出来的字体大小，其实跟普通文本差不多，所以不要用）

```Java
#### 我是四级标题
```

### 空格

#### 中英文之间需要增加空格

正确：

```Java
任何可以使用 JavaScript 来编写的应用，最终都会由 JavaScript 编写。
```

错误：

```Java
任何可以使用JavaScript来编写的应用，最终都会由JavaScript编写。
```

#### 中文和数字之间需要增加空格

正确：

```Java
一年有 12 个月。
```

错误：

```Java
一年有12个月。
```

#### 数字和单位之间需要加空格

正确：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```0

错误：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```1

#### 完整的英文整句时标点与单词之间需要加空格

正确：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```2

错误：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```3

#### 例外：度的标志、百分号不加空格

正确：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```4

#### 例外：全角标点与其他字符之间不加空格

正确：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```5

### 专有名词使用正确的大小写

正确：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```6

错误：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```7

正确：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```8

### 标点

#### 关于全角和半角

简单来说，全角占两个字节，半角占一个字节。你可以理解成中文汉字和中文标点是全角，英文字母和英文标点是半角。体现在排版上的差异就是，全角字符屏幕打印宽度是两个，而半角字符屏幕打印宽度是一个，如中文逗号和英文逗号他们的显示分别是「，」和「,」。

更详细的介绍，可以查看维基百科的词条「[全角和半角](https://zh.wikipedia.org/wiki/%E5%85%A8%E5%BD%A2%E5%92%8C%E5%8D%8A%E5%BD%A2)」

#### 使用全角中文标点

中文排版中，所有的标点都应该使用全角中文标点。

示例：

```Java
20190902_1010.png

20180228_1505.jpg

20180616_1618.gif
```9

#### 遇到英文整句、特殊名词时使用半角标点

示例：

```Java
段落1（这里是第一行）

段落2（这里其实是第三行）
```0

### 需要特别强调的词，建议用直角引号

示例：

```Java
段落1（这里是第一行）

段落2（这里其实是第三行）
```1

### 图床工具

写文章免不了要做图片传图片，推荐使用 [PicGo](https://molunerfinn.com/PicGo/) 工具，不论屏幕截图、还是复制图片，都可以自动上传、保存 Markdown 格式的链接，直接粘贴插入。

对了，在使用 PicGo 之前，先要保证你要有一个保存图片的图床服务器，配置到 PicGo 上，然后再让 PicGo 帮你生成图片的 url 链接。

### 公众号 Markdown 编辑器推荐

> 市面上常见的、大众的、小众的、在线版的 Mardown 排版工具，都在这儿了。

> 大部分排版工具，可以针对外链，自动生成脚注。

#### 可能吧排版

- 网址<：https://knb.im/mp/>

#### Rabbit Hole 微信排版编辑器

- 网址：https://rabbit.zoepi.online

无广告、沉浸式写作、支持定制化主题、审美在线、一键复制后即可发布 的排版编辑器。免登陆＋免费使用。

详细介绍：[Juicing 工具篇——「Rabbit Hole」微信排版编辑器上线](https://mp.weixin.qq.com/s/JOZsf4CL0yY_yj70gjlacw)、[Juicing Juice 导论 | 一份榨汁机说明书](https://mp.weixin.qq.com/s/aMJHXw57oOYzQwGJGdTVag)

#### Markdown 在线编辑器

- <https://markdown.com.cn/editor/>

#### lab.lyric.im 排版

- markdown 在线转换工具：<https://lab.lyric.im/wxformat>

歌词经理作品。

#### zkqiang.cn 排版

- 网址：<http://prod.zkqiang.cn/wxeditor/index.html>

#### aclickall.com 排版

- 网址：<http://md.aclickall.com/>

#### openwrite 排版

- 网址：<https://md.openwrite.cn/>

#### mdnice 排版

- mdnice 网址：<https://www.mdnice.com/>

这个 markdown 排版网站用的人挺多，在程序员的圈子流传很广。但自 2020 年 10 月底开始，需要登录才能使用；它已经不再单纯是一个排版工具了，逐渐向社区演进，略显臃肿。

### 文本纠错

写作猫（文本纠错网站）：<https://xiezuocat.com/>

### 参考链接

- [《中文技术文档的写作规范》](https://github.com/ruanyf/document-style-guide)
- [《写给大家看的中文排版指南》](https://zhuanlan.zhihu.com/p/20506092)
- [《中文文案排版指北》](https://github.com/sparanoid/chinese-copywriting-guidelines)
- [《中文排版指南》](https://github.com/ctf-wiki/ctf-wiki/wiki/%E4%B8%AD%E6%96%87%E6%8E%92%E7%89%88%E6%8C%87%E5%8D%97)
- [《README 文档的规范写法》](https://github.com/AweiLoveAndroid/CommonDevKnowledge/blob/master/github_README/README%E6%96%87%E6%A1%A3%E7%9A%84%E8%A7%84%E8%8C%83%E5%86%99%E6%B3%95.md)
- [stormzhang | 每个人都需要的中文排版指南](https://mp.weixin.qq.com/s/k5DAmYtMrdSlK1jHsW-hrg)
- [《这是曹将公众号排版的所有秘密！》](https://mp.weixin.qq.com/s/DEUUcO4FhCiIYmh61wGXnw)
- [文章排版规范](https://wiki-power.com/#/post/%E6%95%88%E7%8E%87/%E6%96%87%E7%AB%A0%E6%8E%92%E7%89%88%E8%A7%84%E8%8C%83)

---

### 出处

[GitHub - qianguyihao/document-guide: 中文博客的排版指南。](https://github.com/qianguyihao/document-guide)
