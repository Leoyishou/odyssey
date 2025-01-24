---
draw:
tags: []
title: obsidian 官方 publish 方案如何增加评论系统
date created: 2024-07-15
date modified: 2025-01-21
---

关于评论系统的更换。

前几天有位朋友给我留言，让我挺受感动的：

Eason²⁰²⁰：

Hi, 初次造访，你的知识储备让我感到震撼，我会向你学习:smile:

本来想回复他，可是来必力的评论竟然无法在页面下显示，让我颇为郁闷，网上查阅了相关资料后依旧没有解决，所以最后决定更换成valine评论系统，目前感觉确实更好用一些，不过之前的评论是没法迁移过来了（尽管比较少），这里还是稍微有点可惜。

评论系统参考资料（leancloud）记得选国际版：

[https://yuanmomo.net/2019/06/20/hexo-add-valine/](https://yuanmomo.net/2019/06/20/hexo-add-valine/)

[https://tding.top/archives/ed8b904f.html](https://tding.top/archives/ed8b904f.html)

[http://www.zhaojun.im/hexo-valine-admin/](http://www.zhaojun.im/hexo-valine-admin/)

[https://github.com/DesertsP/Valine-Admin](https://github.com/DesertsP/Valine-Admin)

[https://mrhuanhao.cn/2020/03/25/emailreply/](https://mrhuanhao.cn/2020/03/25/emailreply/)

备注：

leancloud数据迁移参考资料：

[https://felixxiong.github.io/2020/09/25/LeanCloud%E8%BF%81%E7%A7%BB%E8%87%B3%E5%9B%BD%E9%99%85%E7%89%88/](https://felixxiong.github.io/2020/09/25/LeanCloud%E8%BF%81%E7%A7%BB%E8%87%B3%E5%9B%BD%E9%99%85%E7%89%88/)

这里补充下邮件提醒碰到的坑

## [](https://doraemonzzz.com/2020/10/17/%E6%9B%B4%E6%8D%A2%E8%AF%84%E8%AE%BA%E7%B3%BB%E7%BB%9F/#%E9%82%AE%E4%BB%B6%E6%8F%90%E9%86%92 "邮件提醒")邮件提醒

- SMTP_PASS为授权码
- 如果用自己的邮箱留言，应该是不会收到邮件提醒的。
- 535报错：[https://moxiaoqin.club/post/valine-pei-zhi-bo-ke-pei-zhi-you-xiang-ti-xing-chu-xian-535jie-jue-fang-an/](https://moxiaoqin.club/post/valine-pei-zhi-bo-ke-pei-zhi-you-xiang-ti-xing-chu-xian-535jie-jue-fang-an/)

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
