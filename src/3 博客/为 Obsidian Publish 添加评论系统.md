---
comment_id: a7b3d4f9
date created: 2025-07-10
tags: [Obsidian, 教程, 博客]
---

# 为 Obsidian Publish 添加评论系统

Obsidian Publish 是官方提供的发布服务，但缺少评论功能。本文介绍如何通过 Giscus 为你的 Obsidian Publish 网站添加评论系统。

## 什么是 Giscus

[Giscus](https://giscus.app) 是一个基于 GitHub Discussions 的评论系统。它将评论存储在你的 GitHub 仓库的 Discussions 中，完全免费且数据归你所有。

**优点：**
- 免费开源
- 数据存储在自己的 GitHub 仓库
- 支持 Markdown
- 有反应表情（emoji reactions）
- 支持多种主题

## 准备工作

### 1. 创建 GitHub 仓库

如果还没有，需要创建一个公开的 GitHub 仓库用于存储评论。

### 2. 启用 Discussions

在仓库的 Settings → General → Features 中，勾选 "Discussions"。

### 3. 安装 Giscus App

访问 [Giscus App](https://github.com/apps/giscus) 并安装到你的仓库。

## 配置 Giscus

### 1. 获取配置信息

访问 [giscus.app](https://giscus.app) 并填写：
- 输入你的仓库名（格式：`用户名/仓库名`）
- 选择 Discussion 分类（建议使用 "Announcements" 或创建 "Comments"）
- 选择映射方式（推荐 "pathname" 或 "og:title"）

页面会自动生成配置代码。记下以下信息：
- `data-repo-id`
- `data-category-id`

### 2. 创建 publish.js

在 Obsidian 仓库根目录创建 `publish.js`：

```javascript
// publish.js
function injectGiscus() {
    console.log('Giscus: 尝试注入评论系统...');
    if (document.getElementById('giscus-container')) {
        console.log('Giscus: 评论容器已存在，跳过注入');
        return;
    }
    
    const container = document.createElement('div');
    container.id = 'giscus-container';
    container.style.marginTop = '3rem';
    container.style.maxWidth = '100%';
    container.style.margin = '3rem auto 0';
    container.style.padding = '0';
  
    const s = document.createElement('script');
    s.src = 'https://giscus.app/client.js';
    s.dataset.repo        = '你的用户名/你的仓库名';  // 修改这里
    s.dataset.repoId      = "你的repo-id";           // 修改这里
    s.dataset.category    = 'Comments';
    s.dataset.categoryId  = "你的category-id";       // 修改这里
    s.dataset.mapping     = 'og:title';               // 使用标题映射
    s.dataset.reactionsEnabled = '1';
    s.dataset.theme       = 'preferred_color_scheme';
    s.crossOrigin = 'anonymous'; 
    s.async = true;
  
    container.appendChild(s);
    
    // 尝试多个可能的容器
    const targetElement = 
        document.querySelector('.markdown-preview-view') || 
        document.querySelector('.publish-article-content') ||
        document.querySelector('.content') ||
        document.querySelector('article') ||
        document.querySelector('main');
    
    if (targetElement) {
        targetElement.appendChild(container);
        console.log('Giscus: 评论容器已添加');
        
        // 添加样式以确保宽度一致
        const style = document.createElement('style');
        style.textContent = `
            #giscus-container {
                width: 100% !important;
                box-sizing: border-box !important;
            }
            #giscus-container .giscus {
                max-width: 100% !important;
            }
            #giscus-container iframe {
                max-width: 100% !important;
            }
        `;
        document.head.appendChild(style);
    } else {
        document.body.appendChild(container);
        console.log('Giscus: 未找到合适容器，评论添加到 body 末尾');
    }
}
  
// 确保脚本在合适的时机执行
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', injectGiscus);
} else {
    injectGiscus();
}
  
// 备用：页面完全加载后再次尝试
window.addEventListener('load', () => {
    setTimeout(injectGiscus, 1000);
});
```

### 3. 发布到 Obsidian Publish

1. 在 Obsidian 中打开发布面板
2. 选中 `publish.js` 文件
3. 点击发布

## 高级配置：永久绑定评论

默认情况下，如果文件重命名或移动，评论会丢失。以下是解决方案：

### 方案 1：使用标题映射

将 `mapping` 设置为 `'og:title'`，这样只要文章标题不变，评论就会保留。

### 方案 2：使用唯一 ID（推荐）

为每篇文章添加唯一标识符，使评论永久绑定。

#### 1. 创建 ID 生成脚本

创建 `add_comment_ids.py`：

```python
#!/usr/bin/env python3
import os
import re
import uuid
import yaml
from pathlib import Path

def generate_comment_id():
    return str(uuid.uuid4())[:8]

def has_comment_id(content):
    match = re.match(r'^---\s*\n(.*?)\n---\s*\n', content, re.DOTALL)
    if match:
        try:
            frontmatter = yaml.safe_load(match.group(1))
            return frontmatter and 'comment_id' in frontmatter
        except:
            pass
    return False

def add_comment_id_to_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    if has_comment_id(content):
        return False
    
    comment_id = generate_comment_id()
    match = re.match(r'^---\s*\n(.*?)\n---\s*\n(.*)', content, re.DOTALL)
    
    if match:
        try:
            frontmatter = yaml.safe_load(match.group(1)) or {}
            frontmatter['comment_id'] = comment_id
            new_content = f"---\n{yaml.dump(frontmatter, allow_unicode=True)}---\n{match.group(2)}"
        except:
            new_content = f"---\n{match.group(1)}\ncomment_id: {comment_id}\n---\n{match.group(2)}"
    else:
        new_content = f"---\ncomment_id: {comment_id}\n---\n\n{content}"
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(new_content)
    
    return True

# 处理所有 Markdown 文件
for filepath in Path('.').rglob('*.md'):
    if add_comment_id_to_file(filepath):
        print(f"已添加 comment_id: {filepath}")
```

#### 2. 运行脚本

```bash
pip install pyyaml
python add_comment_ids.py
```

#### 3. 更新 publish.js

修改 `getDocumentId` 函数以支持 comment_id：

```javascript
const getDocumentId = () => {
    // 尝试从页面内容中查找 comment_id
    const pageContent = document.body.innerText;
    const commentIdMatch = pageContent.match(/comment_id:\s*([a-f0-9]{8})/);
    if (commentIdMatch) {
        return commentIdMatch[1];
    }
    
    // 使用完整路径作为备选
    const pathname = window.location.pathname;
    const cleanPath = pathname
        .replace(/\.html$/, '')
        .replace(/\/$/, '')
        .replace(/^\//, '');
    
    if (cleanPath && cleanPath !== 'index') {
        return cleanPath;
    }
    
    // 最后使用标题
    return document.title || 'untitled';
};

// 使用 specific 映射
s.dataset.mapping = 'specific';
s.dataset.term = getDocumentId();
```

### 设置 Git Hook（可选）

自动为新文件添加 comment_id：

创建 `.git/hooks/pre-commit`：

```bash
#!/bin/bash
# 获取新增的 .md 文件
STAGED_NEW_MD=$(git diff --cached --name-only --diff-filter=A | grep '\.md$')

if [ -z "$STAGED_NEW_MD" ]; then
    exit 0
fi

echo "为新文件添加 comment_id..."
python add_comment_ids.py

# 重新添加修改的文件
git add -u .
exit 0
```

赋予执行权限：

```bash
chmod +x .git/hooks/pre-commit
```

## 调试技巧

1. **检查是否加载成功**
   - 打开浏览器开发者工具（F12）
   - 查看控制台是否有 "Giscus:" 开头的日志

2. **检查评论容器**
   - 在 Elements 面板搜索 `giscus-container`
   - 确认容器位置和样式是否正确

3. **常见问题**
   - 评论区宽度不对：检查 CSS 样式
   - 评论不显示：确认 GitHub 仓库是公开的
   - 评论丢失：检查映射方式是否正确

## 总结

通过以上步骤，你的 Obsidian Publish 网站就拥有了完整的评论功能。使用 comment_id 方案后，即使文件重命名或移动，评论也会永久保留。

这个方案的优势：
- ✅ 完全免费
- ✅ 数据自主可控
- ✅ 支持 Markdown 和表情
- ✅ 评论永不丢失
- ✅ 与 Obsidian 工作流无缝集成