/*****************************************************************
 * publish.js —— 放在仓库根目录
 * 功能：在 Obsidian Publish 页面为每篇笔记加载 giscus 评论，
 *       用 front-matter 中的 comment_id 作为唯一键。
 ******************************************************************/

/* --------- 工具函数：从原始 Markdown 抽取 comment_id --------- */
function getUid() {
    /* Publish 会把 Markdown 原文放进 <script type="text/plain">…</script> */
    const raw =
      document.querySelector('script[type="text/plain"]')?.textContent ||
      document.body.innerText;                          // 兜底：直接扫可见文本
  
    const m = raw.match(/uid:\s*([0-9a-fA-F-]+)/);
    return m ? m[1].trim() : null;                      // 例如 “8f4c3d0a”
  }
  
  /* ------------------------ 主注入函数 ------------------------ */
  function injectGiscus() {
    console.log('[Giscus] 尝试注入…');
  
    /* 防重复 */
    if (document.getElementById('giscus-container')) {
      console.log('[Giscus] 已存在，跳过');
      return;
    }
  
    /* 等页面完全加载再执行 */
    if (document.readyState !== 'complete') {
      setTimeout(injectGiscus, 300);
      return;
    }
  
    /* 读取 comment_id，没有就放弃加载评论 */
    const uid = getUid();
    if (!uid) {
      console.warn('[Giscus] 未找到 uid，页面不加载评论');
      return;
    }
  
    /* 创建容器 */
    const container = document.createElement('div');
    container.id = 'giscus-container';
    container.style.cssText = 'margin:3rem auto 0;padding:0;';
  
    /* 创建 giscus 脚本 */
    const s = document.createElement('script');
    s.src = 'https://giscus.app/client.js';
    s.dataset.repo        = 'Leoyishou/odyssey';
    s.dataset.repoId      = 'R_kgDOPJw6Yg';
    s.dataset.category    = 'Comments';
    s.dataset.categoryId  = 'DIC_kwDOPJw6Ys4CstfW';
  
    /* 关键：用 comment_id 作为主键 */
    s.dataset.mapping     = 'id';
    s.dataset.term        = uid;
  
    s.dataset.reactionsEnabled = '1';
    s.dataset.theme       = 'preferred_color_scheme';
    s.dataset.lang        = 'zh-CN';
    s.crossOrigin         = 'anonymous';
    s.async               = true;
  
    container.appendChild(s);
  
    /* 选个合适节点挂载 - 优先选择 .mod-footer */
    const modFooters = document.querySelectorAll('.mod-footer');
    const lastModFooter = modFooters[modFooters.length - 1];
    
    if (lastModFooter) {
      // 如果找到 .mod-footer，插入到它的后面（作为兄弟元素）
      lastModFooter.parentNode.insertBefore(container, lastModFooter.nextSibling);
      console.log('[Giscus] 已挂载到 .mod-footer 后面');
      return;
    } else {
      // 如果没有 .mod-footer，使用原来的逻辑
      const target =
        document.querySelector('.markdown-preview-sizer') ||
        document.querySelector('.markdown-preview-view') ||
        document.querySelector('.publish-article-content') ||
        document.querySelector('.content') ||
        document.querySelector('article') ||
        document.querySelector('main') ||
        document.querySelector('[class*="content"]') ||
        document.body;
      
      target.appendChild(container);
      console.log('[Giscus] 已挂载到:', target.className || target.tagName);
    }
  
    /* 附加自适应样式 */
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
  
  /* --------------------- 单页应用路径监控 --------------------- */
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
  
  /* ------------------------- 初始注入 ------------------------- */
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', injectGiscus);
  } else {
    injectGiscus();
  }
  window.addEventListener('load', () => setTimeout(injectGiscus, 800));
  window.addEventListener('popstate', () => setTimeout(injectGiscus, 500));
  