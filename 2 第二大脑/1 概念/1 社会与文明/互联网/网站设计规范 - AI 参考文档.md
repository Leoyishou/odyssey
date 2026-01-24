---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  

本文档总结了专业网站设计的核心原则和最佳实践，供 AI 在生成前端代码时参考。遵循这些规范可以避免"AI味"过重的通用设计，打造真正专业、有质感的网站。

一定要用 light 模式，不要用 dark 模式
# 🧩 Cues



# 🪞Notes




---

## 一、核心问题：为什么 AI 生成的网站"差点意思"

AI 生成的网站常见问题：

| 问题 | 表现 | 原因 |
|------|------|------|
| **字体单调** | 全站使用系统默认字体（如 Arial、sans-serif） | 没有专业的字体配对策略 |
| **按钮颜色随意** | 使用 Tailwind 默认蓝色或随机颜色 | 没有根据产品类型选择合适的颜色 |
| **布局缺乏层次** | 所有元素大小相近，缺乏视觉焦点 | 没有建立清晰的视觉层级 |
| **留白不足** | 元素过于紧凑，显得拥挤 | 没有遵循"少即是多"的原则 |
| **装饰过度** | 过多渐变、阴影、动画 | 没有克制，堆砌效果 |

**关键洞察**：字体排印（Typography）是区分专业与业余设计的最重要因素。

---

## 二、字体选择策略

### 2.1 双字体系统

专业网站通常使用两种字体的组合：

| 用途 | 字体类型 | 特点 | 推荐字体 |
|------|----------|------|----------|
| **标题** | Serif（衬线体） | 优雅、权威、有个性 | Merriweather, Playfair Display, Lora |
| **正文** | Sans-serif（无衬线体） | 清晰、现代、易读 | Inter, Source Sans Pro, Open Sans |

### 2.2 按产品类型选择字体配对

| 产品类型 | 标题字体 | 正文字体 | 风格 |
|----------|----------|----------|------|
| **新闻/内容平台** | Merriweather | Inter | 权威、可信 |
| **SaaS/科技产品** | Inter (Bold) | Inter | 现代、简洁 |
| **创意/设计** | Playfair Display | Source Sans Pro | 优雅、艺术 |
| **金融/专业服务** | Lora | Open Sans | 稳重、专业 |
| **个人品牌/咨询** | Merriweather | Inter | 专业、亲和 |

### 2.3 字体引入代码

```html
<!-- 在 index.html 的 <head> 中添加 -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Merriweather:wght@400;700;900&display=swap" rel="stylesheet">
```

### 2.4 CSS 字体配置

```css
/* 在 index.css 中定义 */
@theme inline {
  --font-sans: 'Inter', system-ui, sans-serif;
  --font-serif: 'Merriweather', Georgia, serif;
}

/* 应用字体 */
body {
  font-family: var(--font-sans);
}

h1, h2, h3 {
  font-family: var(--font-serif);
}
```

---

## 三、按钮颜色选择

### 3.1 按产品类型选择主按钮颜色

| 产品类型 | 推荐颜色 | 色值 | 原因 |
|----------|----------|------|------|
| **内容产品/咨询服务** | 深黑色 | `#18181b` | 专业、权威、不抢内容风头 |
| **SaaS/工具类** | 蓝色 | `#2563eb` | 信任、科技感 |
| **电商/消费品** | 品牌色 | 根据品牌 | 强调品牌识别 |
| **社交/娱乐** | 活力色 | 紫色/橙色 | 年轻、活跃 |

### 3.2 深黑色按钮的优势

对于内容型产品（博客、咨询、新闻等），深黑色按钮是最佳选择：

- **不与内容竞争**：让用户专注于内容本身
- **永不过时**：经典配色，不受流行趋势影响
- **高级感**：简洁克制，体现专业品质

### 3.3 按钮样式代码

```css
/* 主按钮 - 深黑色 */
.btn-primary {
  background-color: #18181b;
  color: white;
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 500;
  transition: background-color 0.2s;
}

.btn-primary:hover {
  background-color: #27272a;
}

/* 次要按钮 - 边框样式 */
.btn-secondary {
  background-color: transparent;
  color: #18181b;
  border: 1px solid #e4e4e7;
  padding: 12px 24px;
  border-radius: 8px;
}
```

---

## 四、Linear 风格设计语言

Linear 是现代 SaaS 产品设计的标杆，其设计语言的核心特点：

### 4.1 核心原则

| 原则 | 说明 | 实践 |
|------|------|------|
| **大量留白** | 元素之间保持充足间距 | section 间距 120px+，卡片内边距 32px+ |
| **克制配色** | 主色调为黑白灰 | 彩色仅用于强调，面积不超过 5% |
| **简洁边框** | 使用细边框而非阴影 | `border: 1px solid #e4e4e7` |
| **大标题** | 标题字号要足够大 | Hero 标题 48-72px，section 标题 36-48px |
| **居中布局** | Hero 区域居中对齐 | 文字和按钮都居中 |

### 4.2 间距系统

```css
/* 推荐的间距变量 */
:root {
  --spacing-xs: 8px;
  --spacing-sm: 16px;
  --spacing-md: 24px;
  --spacing-lg: 48px;
  --spacing-xl: 80px;
  --spacing-2xl: 120px;
}

/* Section 间距 */
section {
  padding: var(--spacing-2xl) 0;
}

/* 卡片内边距 */
.card {
  padding: var(--spacing-lg);
}
```

### 4.3 颜色系统

```css
/* Linear 风格配色 */
:root {
  /* 背景色 */
  --bg-primary: #ffffff;
  --bg-secondary: #fafafa;
  --bg-tertiary: #f4f4f5;
  
  /* 文字色 */
  --text-primary: #18181b;
  --text-secondary: #52525b;
  --text-tertiary: #a1a1aa;
  
  /* 边框色 */
  --border-light: #e4e4e7;
  --border-medium: #d4d4d8;
  
  /* 强调色（克制使用） */
  --accent: #18181b;
  --accent-light: #f4f4f5;
}
```

---

## 五、布局最佳实践

### 5.1 Hero 区域

```jsx
{/* Hero 区域 - 居中布局 */}
<section className="py-32 text-center">
  <div className="max-w-3xl mx-auto px-4">
    {/* 小标签 */}
    <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full border border-border mb-8">
      <span className="w-2 h-2 bg-green-500 rounded-full" />
      <span className="text-sm text-muted-foreground">标签文字</span>
    </div>
    
    {/* 大标题 - 使用衬线字体 */}
    <h1 className="font-serif text-5xl md:text-6xl font-bold tracking-tight mb-6">
      主标题文字
    </h1>
    
    {/* 副标题 */}
    <p className="text-xl text-muted-foreground mb-10 max-w-2xl mx-auto">
      副标题描述文字，简洁明了地说明价值主张
    </p>
    
    {/* CTA 按钮组 */}
    <div className="flex items-center justify-center gap-4">
      <Button size="lg" className="bg-primary text-white">
        主要行动
      </Button>
      <span className="text-sm text-muted-foreground">辅助说明</span>
    </div>
  </div>
</section>
```

### 5.2 卡片设计

```jsx
{/* 简洁卡片 - 细边框，无阴影 */}
<div className="border border-border rounded-xl p-8 hover:border-foreground/20 transition-colors">
  {/* 图标 */}
  <div className="w-12 h-12 rounded-lg border border-border flex items-center justify-center mb-6">
    <Icon className="w-6 h-6" />
  </div>
  
  {/* 标题 */}
  <h3 className="font-serif text-xl font-semibold mb-3">
    卡片标题
  </h3>
  
  {/* 描述 */}
  <p className="text-muted-foreground leading-relaxed">
    卡片描述文字
  </p>
</div>
```

### 5.3 数据展示

```jsx
{/* 数据统计 - 大数字 + 小标签 */}
<div className="grid grid-cols-2 md:grid-cols-4 gap-8">
  <div className="text-center">
    <div className="font-serif text-4xl md:text-5xl font-bold mb-2">
      5000+
    </div>
    <div className="text-sm text-muted-foreground">
      统计标签
    </div>
  </div>
</div>
```

---

## 六、常见误区与避免方法

### 6.1 三个必须避免的设计误区

| 误区 | 问题 | 正确做法 |
|------|------|----------|
| **使用系统默认字体** | 缺乏个性，显得廉价 | 引入 Google Fonts 专业字体 |
| **按钮颜色随意** | 与产品调性不符 | 根据产品类型选择合适颜色 |
| **装饰过度** | 毛玻璃、渐变、阴影堆砌 | 克制使用，简洁为主 |

### 6.2 检查清单

在交付设计前，检查以下项目：

- [ ] 是否使用了专业字体配对？
- [ ] 标题是否使用衬线字体？
- [ ] 按钮颜色是否符合产品类型？
- [ ] 留白是否足够？
- [ ] 是否避免了过度装饰？
- [ ] Hero 区域是否居中对齐？
- [ ] 卡片是否使用简洁边框而非阴影？

---

## 七、AI Prompt 技巧

### 7.1 生成前端代码时的明确指令

在让 AI 生成网站时，可以使用以下 prompt 模板：

```
请按照以下设计规范生成网站：

1. 字体配对：
   - 标题使用 Merriweather（衬线字体）
   - 正文使用 Inter（无衬线字体）
   - 在 index.html 中引入 Google Fonts

2. 按钮颜色：
   - 主按钮使用深黑色 #18181b
   - 次要按钮使用边框样式

3. 设计风格：
   - Linear 风格：大量留白、克制配色
   - 不使用毛玻璃效果
   - 卡片使用细边框，不使用阴影
   - Hero 区域居中对齐

4. 布局：
   - Section 间距 120px
   - 卡片内边距 32px
   - 最大内容宽度 1200px
```

### 7.2 快速参考代码

```css
/* 复制这段 CSS 到 index.css 作为基础 */

@import "tailwindcss";

@theme inline {
  --font-sans: 'Inter', system-ui, sans-serif;
  --font-serif: 'Merriweather', Georgia, serif;
  
  --color-background: oklch(100% 0 0);
  --color-foreground: oklch(14.08% 0.004 285.82);
  --color-muted: oklch(96.83% 0.003 247.86);
  --color-muted-foreground: oklch(55.19% 0.014 257.81);
  --color-border: oklch(91.97% 0.004 286.32);
  --color-primary: oklch(14.08% 0.004 285.82);
  --color-primary-foreground: oklch(100% 0 0);
}

/* 字体应用 */
body {
  font-family: var(--font-sans);
  background-color: var(--color-background);
  color: var(--color-foreground);
}

h1, h2, h3, h4 {
  font-family: var(--font-serif);
}

/* 容器 */
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
}

/* Section 间距 */
section {
  padding: 120px 0;
}

/* 卡片 */
.card {
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 32px;
  transition: border-color 0.2s;
}

.card:hover {
  border-color: oklch(70% 0 0);
}
```

---

## 八、总结

打造专业网站的核心要点：

1. **字体是灵魂** - 使用 Serif + Sans-serif 双字体系统
2. **颜色要克制** - 内容产品用深黑色按钮，彩色仅用于强调
3. **留白要充足** - 宁可多留白，不要挤在一起
4. **装饰要简洁** - 细边框优于阴影，简洁优于花哨
5. **布局要清晰** - Hero 居中，建立明确的视觉层级

> **记住**：好的设计是"看不见"的设计。用户应该专注于内容，而不是被设计元素分散注意力。

---

*文档作者：Manus AI*  
*最后更新：2026年1月*
