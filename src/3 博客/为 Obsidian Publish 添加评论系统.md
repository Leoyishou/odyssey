---
date created: 2025-07-10
date modified: 2025-07-10
uid: e6196a61-a77d-4291-9108-2b85fc5ff0e8
---

明白！下面是**结合你现在用 uid 作为评论主键**，并且需要修改 publish.css 以确保 front-matter 可见的完整教程文章（适配 Obsidian Publish + Giscus + note_uid_generator 插件 + CSS 方案）。

---

## 为 Obsidian Publish 添加评论系统（基于 uid 唯一标识）

Obsidian Publish 是官方提供的发布服务，但缺少评论功能。本文介绍如何通过 Giscus 为你的 Obsidian Publish 网站添加评论系统，并结合 [note_uid_generator](obsidian://show-plugin?id=note_uid_generator) 插件生成的 `uid` 字段，实现评论与页面的永久绑定。

---

### 什么是 Giscus

Giscus 是一个基于 GitHub Discussions 的评论系统。它将评论存储在你的 GitHub 仓库的 Discussions 中，完全免费且数据归你所有。

**优点：**
- 免费开源
- 数据存储在自己的 GitHub 仓库
- 支持 Markdown
- 有反应表情（emoji reactions）
- 支持多种主题

---

### 准备工作

#### 1. 创建 GitHub 仓库

如果还没有，需要创建一个公开的 GitHub 仓库用于存储评论。

#### 2. 启用 Discussions

在仓库的 Settings → General → Features 中，勾选 "Discussions"。

#### 3. 安装 Giscus App

访问 [Giscus App](https://github.com/apps/giscus) 并安装到你的仓库。

---

### 配置 Giscus

#### 1. 获取配置信息

访问 [giscus.app](https://giscus.app/) 并填写：

- 输入你的仓库名（格式：用户名/仓库名）
- 选择 Discussion 分类（建议使用 "Announcements" 或创建 "Comments"）
- 选择映射方式（**选择 "Specific"**，后续我们会用 uid 作为 term）
- 页面会自动生成配置代码。记下以下信息：
  - data-repo
  - data-repo-id
  - data-category
  - data-category-id

---

### 2. 为每篇笔记生成唯一 uid

推荐使用 [note_uid_generator](obsidian://show-plugin?id=note_uid_generator) 插件自动为每篇笔记的 front-matter 添加 `uid` 字段。例如：

```yaml
---
uid: 3694d835-33ec-462a-9fd4-e4118c24e664
title: 一切皆项目$
date created: 2024-08-23
date modified: 2025-06-22
---
```

确保每篇需要评论的笔记都包含唯一的 `uid` 字段。

---

### 3. 修改 publish.css 让 front-matter 可见

Obsidian Publish 默认会隐藏 front-matter，但我们的评论系统需要 JS 能读取到页面上的 uid。

请在你的仓库根目录下创建或修改 `publish.css`，加入以下内容：

```css
/* 强制让 front-matter 显示出来 */
.frontmatter,
.frontmatter-container,
.markdown-preview-view:not(.show-frontmatter) .frontmatter {
  display: block !important;   /* 取消 display:none */
  white-space: pre;            /* 保留缩进 */
}

/* 可选：更通用的兜底，防止主题覆盖 */
[class*="frontmatter"] {
  display: block !important;
  white-space: pre !important;
}

/* 可选：调试时加背景色 */
.frontmatter, .frontmatter-container {
  background: #f8f8f8;
  border-left: 3px solid #e0e0e0;
  padding: 0.5em 1em;
}
```

这样可以确保 front-matter 在页面上可见，JS 才能正确提取 uid。

---

### 4. 创建 publish.js

在 Obsidian 仓库根目录创建 `publish.js`，内容如下（已适配 uid）：

```js
/*****************************************************************
 * publish.js —— 放在仓库根目录
 * 功能：在 Obsidian Publish 页面为每篇笔记加载 giscus 评论，
 *       用 front-matter 中的 uid 作为唯一键。
 ******************************************************************/

// 工具函数：从原始 Markdown 抽取 uid
function getUid() {
    // Publish 会把 Markdown 原文放进 <script type="text/plain">…</script>
    const raw =
        document.querySelector('script[type="text/plain"]')?.textContent ||
        document.body.innerText; // 兜底：直接扫可见文本

    // 适配 note_uid_generator 插件生成的 uid
    const m = raw.match(/uid:\s*([0-9a-fA-F-]+)/);
    return m ? m[1].trim() : null;
}

// 主注入函数
function injectGiscus() {
    console.log('[Giscus] 尝试注入…');

    // 防重复
    if (document.getElementById('giscus-container')) {
        console.log('[Giscus] 已存在，跳过');
        return;
    }

    // 等页面完全加载再执行
    if (document.readyState !== 'complete') {
        setTimeout(injectGiscus, 300);
        return;
    }

    // 读取 uid，没有就放弃加载评论
    const uid = getUid();
    if (!uid) {
        console.warn('[Giscus] 未找到 uid，页面不加载评论');
        return;
    }

    // 创建容器
    const container = document.createElement('div');
    container.id = 'giscus-container';
    container.style.cssText = 'margin:3rem auto 0;max-width:100%;padding:0;';

    // 创建 giscus 脚本
    const s = document.createElement('script');
    s.src = 'https://giscus.app/client.js';
    s.dataset.repo        = '你的用户名/你的仓库名';  // 修改这里
    s.dataset.repoId      = '你的repo-id';           // 修改这里
    s.dataset.category    = 'Comments';
    s.dataset.categoryId  = '你的category-id';       // 修改这里

    // 关键：用 uid 作为主键
    s.dataset.mapping     = 'specific';
    s.dataset.term        = uid;

    s.dataset.reactionsEnabled = '1';
    s.dataset.theme       = 'preferred_color_scheme';
    s.dataset.lang        = 'zh-CN';
    s.crossOrigin         = 'anonymous';
    s.async               = true;

    container.appendChild(s);

    // 选个合适节点挂载
    const target =
        document.querySelector('.markdown-preview-view') ||
        document.querySelector('.publish-article-content') ||
        document.querySelector('.content') ||
        document.querySelector('article') ||
        document.querySelector('main') ||
        document.querySelector('[class*="content"]') ||
        document.body;

    target.appendChild(container);
    console.log('[Giscus] 已挂载到:', target.className || target.tagName);

    // 附加自适应样式
    if (!document.getElementById('giscus-style')) {
        const style = document.createElement('style');
        style.id = 'giscus-style';
        style.textContent = `
            #giscus-container{width:100%!important;box-sizing:border-box!important;}
            #giscus-container .giscus{max-width:100%!important;}
            #giscus-container iframe{max-width:100%!important;}
        `;
        document.head.appendChild(style);
    }
}

// 单页应用路径监控（可选，适配 SPA 路由变化）
let lastPath = window.location.pathname;
function watchPathChange() {
    if (window.location.pathname !== lastPath) {
        lastPath = window.location.pathname;
        console.log('[Giscus] 路径变更，重新注入');
        document.getElementById('giscus-container')?.remove();
        setTimeout(injectGiscus, 500);
    }
}
setInterval(watchPathChange, 1000);

// 初始注入
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', injectGiscus);
} else {
    injectGiscus();
}
window.addEventListener('load', () => setTimeout(injectGiscus, 800));
window.addEventListener('popstate', () => setTimeout(injectGiscus, 500));
```

---

### 5. 发布到 Obsidian Publish

1. 在 Obsidian 中打开发布面板
2. 选中 `publish.js` 和 `publish.css` 文件
3. 点击发布

---

### 高级配置：评论永久绑定

**为什么用 uid？**  
如果你用文件名、路径或标题作为评论主键，重命名/移动文件会导致评论丢失。用 `uid` 作为唯一标识，即使文件移动、重命名，评论也不会丢失。

---

### 调试技巧

- 检查是否加载成功：打开浏览器开发者工具（F12），查看控制台是否有 "[Giscus]" 开头的日志
- 检查评论容器：在 Elements 面板搜索 `giscus-container`，确认容器位置和样式是否正确
- 评论区宽度不对：检查 CSS 样式
- 评论不显示：确认 GitHub 仓库是公开的，Giscus 配置项填写正确
- 评论丢失：检查每篇笔记的 front-matter 是否有唯一的 `uid` 字段

---

### 总结

通过以上步骤，你的 Obsidian Publish 网站就拥有了完整的评论功能。使用 `uid` 方案后，即使文件重命名或移动，评论也会永久保留。

**优势：**
- ✅ 完全免费
- ✅ 数据自主可控
- ✅ 支持 Markdown 和表情
- ✅ 评论永不丢失
- ✅ 与 Obsidian 工作流无缝集成

---

如需自动批量补全 uid，可用 note_uid_generator 插件，或用脚本批量处理 front-matter。

如需进一步定制，欢迎随时提问！
