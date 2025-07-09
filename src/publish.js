// publish.js —— 放到仓库根目录
function injectGiscus() {
    console.log('Giscus: 尝试注入评论系统...');
    
    // 防止重复注入
    if (document.getElementById('giscus-container')) {
        console.log('Giscus: 评论容器已存在，跳过注入');
        return;
    }
    
    // 等待页面完全加载
    if (document.readyState !== 'complete') {
        console.log('Giscus: 页面未完全加载，稍后重试...');
        setTimeout(injectGiscus, 500);
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
    s.dataset.repo        = 'Leoyishou/odyssey';
    s.dataset.repoId      = "R_kgDOPJw6Yg";
    s.dataset.category    = 'Comments';
    s.dataset.categoryId  = "DIC_kwDOPJw6Ys4CstfW";
    
    // 使用更简单可靠的路径映射
    // Obsidian Publish 的 URL 格式通常是：https://your-site.obsidian.md/path/to/file
    const pathname = window.location.pathname;
    const cleanPath = pathname
        .replace(/^\//, '')      // 去掉开头的 /
        .replace(/\/$/, '')      // 去掉结尾的 /
        .replace(/\.html$/, '')  // 去掉 .html 后缀
        .replace(/\.md$/, '');   // 去掉 .md 后缀
    
    // 如果路径为空或是首页，使用特殊标识
    const documentId = cleanPath || 'homepage';
    
    console.log('Giscus: 调试信息 --------');
    console.log('- 当前URL:', window.location.href);
    console.log('- 路径名:', pathname);
    console.log('- 清理后路径:', cleanPath);
    console.log('- 最终ID:', documentId);
    console.log('- 页面标题:', document.title);
    console.log('------------------------');
    
    // 使用 pathname 映射，这是最可靠的方式
    s.dataset.mapping     = 'pathname';
    s.dataset.strict      = '0';  // 允许模糊匹配
    s.dataset.reactionsEnabled = '1';
    s.dataset.theme       = 'preferred_color_scheme';
    s.dataset.lang        = 'zh-CN';
    s.crossOrigin = 'anonymous'; 
    s.async = true;
  
    container.appendChild(s);
    
    // 尝试多个可能的容器
    const targetElement = 
        document.querySelector('.markdown-preview-view') || 
        document.querySelector('.publish-article-content') ||
        document.querySelector('.content') ||
        document.querySelector('article') ||
        document.querySelector('main') ||
        document.querySelector('[class*="content"]');
    
    if (targetElement) {
        targetElement.appendChild(container);
        console.log('Giscus: 评论容器已添加到:', targetElement.className || targetElement.tagName);
        
        // 添加样式
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
        // 如果找不到合适的容器，添加到 body 末尾
        document.body.appendChild(container);
        console.log('Giscus: 未找到合适容器，评论添加到 body 末尾');
    }
    
    console.log('Giscus: 注入完成！');
}
  
// 监听页面变化（Obsidian Publish 可能是单页应用）
let lastPath = window.location.pathname;
const checkForPageChange = () => {
    if (window.location.pathname !== lastPath) {
        console.log('Giscus: 检测到页面变化，重新注入评论系统');
        lastPath = window.location.pathname;
        
        // 移除旧的评论容器
        const oldContainer = document.getElementById('giscus-container');
        if (oldContainer) {
            oldContainer.remove();
        }
        
        // 重新注入
        setTimeout(injectGiscus, 500);
    }
};

// 每秒检查一次页面是否变化
setInterval(checkForPageChange, 1000);

// 初始注入
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', injectGiscus);
} else {
    injectGiscus();
}

// 备用：页面完全加载后再次尝试
window.addEventListener('load', () => {
    setTimeout(injectGiscus, 1000);
});