---
uid: 6fbeef72-b3be-4ad8-9ba6-14ad8f0b89ce
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡 Summary  



# 🧩 Cues



# 🪞Notes
这是一个非常经典且棘手的 **"AI 编程的中期陷阱"**。

**核心痛点：** 在使用 Manus/Cursor 这类工具时，从 0 到 1 做 MVP 极快（Vibe Build 很爽），但一旦涉及到 **核心数据模型（Schema）的变更**（比如从 `Tweet` 变成 `Tweet + Newsletter`），AI 往往因为上下文窗口（Context Window）限制，或者无法理解全局依赖，导致改了一处坏了三处，或者直接把代码改乱。

针对你现在的产品形态（日报聚合类），要引入 Newsletter 这种异构数据源，**最佳实践不是让 Manus 去“修改”现有代码，而是采用“增量开发 + 适配器模式”的思路。**

以下是具体的操作思路：

### 核心策略：不要动旧的，建立“中间层”

你现在的痛点是：Twitter 数据结构和 Newsletter 数据结构完全不同，改动会破坏现有的 UI 和逻辑。

解决思路： 抽象出一个统一的内容标准（Unified Content Schema）。

#### 第一步：设计“通用数据层” (The Universal Table)

不要试图把 Newsletter 强塞进 `tweets` 表，也不要让 UI 分别去读两张表。你需要让 Manus 帮你设计一张新的聚合表（或者视图）。

你可以这样提示 Manus：

> "我现在需要引入 Newsletter。请不要修改现有的推特抓取逻辑。请帮我设计一个新的 Supabase 表叫 `daily_feed_items`，用来统一存储所有类型的信息流。"

这张表应该长这样（抽象化）：

- `id`: UUID
    
- `source_type`: Enum ('twitter', 'newsletter')
    
- `original_source_id`: (关联到推特博主或 Newsletter 源)
    
- `title`: (推特就是推文摘要，Newsletter 就是标题)
    
- `content_text`: (正文)
    
- `summary_ai`: (DeepSeek/Gemini 总结后的内容)
    
- `published_at`: DateTime↳
    
- `metadata`: JSONB (这里最关键！推特的 Retweet 数、Newsletter 的原始 HTML 链接等**特有字段**全扔这里，互不干扰)
    

#### 第二步：增量开发“适配器” (The Adapter)

**不要让 Manus 去重构 `TwitterScraper.ts`，而是让它新建一个 `NewsletterScraper.ts`。**

你需要让 Manus 写两个**转化函数**，把异构数据变成上面的“通用数据”：

1. `convertTweetToFeedItem(tweet) -> FeedItem`
    
2. `convertNewsletterToFeedItem(email) -> FeedItem`
    

Vibe Build 技巧：

你可以直接把 Newsletter 的一段 JSON 数据样本扔给 Manus，说：

> "这是 Newsletter 的数据结构，请写一个独立的函数，把它清洗并转换成 `daily_feed_items` 表的格式。只给我这个函数的代码，不要动其他文件。"

#### 第三步：UI 层的“热插拔”

现在你的 MVP 网站是直接渲染推特的。要改动 UI 且不破坏原有 Vibe（Matter 风格），你需要做一个**通用组件**。

提示 Manus：

> "我要做一个新的 React 组件叫 FeedCard。它接受 daily_feed_items 的数据。
> 
> 如果 source_type 是 twitter，渲染成之前的推特样式（引用之前的组件）。
> 
> 如果 source_type 是 newsletter，渲染成一个新的文章卡片样式（突出标题和长文摘要）。"

这样，你的改动就变成了：

1. DB 层：新增一张表（不影响旧表）。
    
2. 逻辑层：新增一个转换器（不改旧逻辑）。
    
3. UI 层：新增一个路由/组件（旧组件作为子集被复用）。
    

### 针对你的产品（Twitter Digest）的具体建议

既然你在做 **Twitter + Newsletter 聚合**，且面临上下文过长的问题，我有几个针对性的“逃课”方案：

**1. 利用 RSS 作为中间格式（最推荐）**

- Newsletter 本质上都能转 RSS。
    
- Twitter (通过 Nitter/RSSHub) 也能转 RSS。
    
- **你可以把你的后端逻辑彻底改成“只处理 RSS”**。
    
- 这样 Manus 只需要维护一套解析逻辑：`RSS -> AI Summary -> Daily Report`。
    
- 你只需要让 Manus 写一个模块：**"给特定 Newsletter URL 生成 RSS 的模块"**。
    

**2. 只有“Summary”入库**

- 如果你的卖点是“日报”，用户其实不一定要看原始推文/邮件。
    
- 你可以让 AI Agent 在抓取阶段就直接把 Twitter 和 Newsletter 都“消化”掉，只把 **纯文本的 Summary** 存入数据库。
    
- 这样你的前端永远只需要渲染“Markdown 文本”，完全解耦了数据源的复杂性。
    

3. 解决“上下文太长”的 Prompt 技巧

当你想改功能时，不要把整个项目丢给它。

- **错误做法：** "把我的项目改成支持 Newsletter。"（Manus 会尝试读取所有文件，然后迷失）
    
- **正确做法：** "这是我现有的 `Schema.sql`。我想增加一个 Newsletter 表。请帮我写出 SQL Migration 语句。然后，请帮我写一个独立的 `fetch_newsletter.ts` 脚本，要求能把数据插入这个新表。" -> **分模块生成，人工粘贴组合。**
    

### 总结

你的产品逻辑很棒（Mailbrew 的现代化复活），只要**数据层解耦**做得好，后面加 Podcast、YouTube 摘要都很容易。

现在的动作建议：

不要去改现在的 MVP 代码，把 Newsletter 当作一个全新的微服务去写。写好后，只在“数据库”和“前端渲染入口”这两个点上进行对接。