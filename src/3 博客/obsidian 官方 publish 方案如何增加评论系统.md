---
draw:
tags: []
title: obsidian 官方 publish 方案如何增加评论系统
date created: 2024-07-15
date modified: 2024-12-27
---

[Integrating Comments in Obsidian Publish - Sparrow's Digital Garden](https://garden.sparrow.zone/Integrating+Comments+in+Obsidian+Publish)

obsidian 的官方发布方案无疑是最省心最方便的一种笔记发布方式，遗憾的是，其并没有提供原生的评论系统。我们可以利用其 publish.js 文件的机制手动添加。

评论系统的方案可以参考 [博客评论](博客评论.md) 中列举的，这里选择了比较轻量化的 utterances

下面是 publish.js 的代码

```javascript
console.log("publish.js script started");

function addUtterancesIframe() {
  console.log("addUtterancesIframe function called");

  const scriptContainer = document.createElement('div');
  scriptContainer.id = 'utterances-container';
  scriptContainer.style.width = '60%';  // Set the width to 60%
  scriptContainer.style.marginTop = '20px';
  scriptContainer.style.float = 'left';  // Align to the left

  const script = document.createElement('script');
  script.src = 'https://utteranc.es/client.js';
  script.setAttribute('repo', 'Leoyishou/brain.liugongzi.org');
  script.setAttribute('issue-term', 'pathname');
  script.setAttribute('theme', 'github-light');
  script.crossOrigin = 'anonymous';
  script.async = true;

  scriptContainer.appendChild(script);

  console.log("Utterances script created");

  function checkAndInsert() {
    const markdownView = document.querySelector('.markdown-preview-view');
    if (markdownView) {
      markdownView.appendChild(scriptContainer);
      console.log("Utterances script appended to the markdown view");
    } else {
      console.log("Error: Could not find .markdown-preview-view element, retrying in 500ms");
      setTimeout(checkAndInsert, 500);
    }
  }

  checkAndInsert();
}

function initTest() {
  console.log("initTest function called");
  if (document.readyState === 'complete') {
    console.log("Document already complete, adding iframes immediately");
    addUtterancesIframe();
  } else {
    console.log("Document not yet complete, adding load event listener");
    window.addEventListener('load', addUtterancesIframe);
  }
}

initTest();
```
