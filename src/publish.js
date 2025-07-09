// publish.js —— 放到仓库根目录
function injectGiscus() {
    if (document.getElementById('giscus-container')) return;
    const container = document.createElement('div');
    container.id = 'giscus-container';
    container.style.marginTop = '3rem';
  
    const s = document.createElement('script');
    s.src = 'https://giscus.app/client.js';
    s.dataset.repo        = 'Leoyishou/odyssey';
    s.dataset.repoId      = "R_kgDOPJw6Yg";
    s.dataset.category    = 'Comments';
    s.dataset.categoryId  = "DIC_kwDOPJw6Ys4CstfW";
    s.dataset.mapping     = 'pathname';    // 以页面路径匹配
    s.dataset.reactionsEnabled = '1';
    s.dataset.theme       = 'preferred_color_scheme';
    s.crossOrigin = 'anonymous'; s.async = true;
  
    container.appendChild(s);
    
    // 尝试多个可能的选择器
    const targetElement = 
        document.querySelector('.markdown-preview-view') || 
        document.querySelector('.publish-article-content') ||
        document.querySelector('.content') ||
        document.querySelector('article') ||
        document.querySelector('main');
    
    if (targetElement) {
        targetElement.appendChild(container);
    } else {
        // 如果找不到合适的容器，添加到 body 末尾
        document.body.appendChild(container);
    }
  }
  
  // 使用多个事件确保脚本执行
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', injectGiscus);
  } else {
    injectGiscus();
  }
  
  // 备用：页面完全加载后再次尝试
  window.addEventListener('load', () => {
    setTimeout(injectGiscus, 1000);
  });
  