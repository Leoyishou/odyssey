// publish.js —— 放到仓库根目录
let currentGiscusContainer = null;

function injectGiscus() {
    console.log('Giscus: 尝试注入评论系统...');
    
    // 等待页面完全加载
    if (document.readyState !== 'complete') {
        console.log('Giscus: 页面未完全加载，稍后重试...');
        setTimeout(injectGiscus, 500);
        return;
    }
    
    // 移除旧的评论容器
    if (currentGiscusContainer) {
        currentGiscusContainer.remove();
        currentGiscusContainer = null;
        console.log('Giscus: 已移除旧的评论容器');
    }
    
    const container = document.createElement('div');
    container.id = 'giscus-container';
    container.style.marginTop = '3rem';
    container.style.maxWidth = '100%';
    container.style.margin = '3rem auto 0';
    container.style.padding = '0';
    
    currentGiscusContainer = container;
  
    const s = document.createElement('script');
    s.src = 'https://giscus.app/client.js';
    s.dataset.repo        = 'Leoyishou/odyssey';
    s.dataset.repoId      = "R_kgDOPJw6Yg";
    s.dataset.category    = 'Comments';
    s.dataset.categoryId  = "DIC_kwDOPJw6Ys4CstfW";
    
    // 使用页面标题作为唯一标识
    // 这是最可靠的方式，因为每篇文章的标题应该是唯一的
    s.dataset.mapping     = 'og:title';
    s.dataset.strict      = '0';
    s.dataset.reactionsEnabled = '1';
    s.dataset.theme       = 'preferred_color_scheme';
    s.dataset.lang        = 'zh-CN';
    s.crossOrigin = 'anonymous'; 
    s.async = true;
    
    // 调试信息
    console.log('Giscus: 调试信息 --------');
    console.log('- 当前URL:', window.location.href);
    console.log('- 页面标题:', document.title);
    console.log('- OG标题:', document.querySelector('meta[property="og:title"]')?.content);
    console.log('------------------------');
  
    container.appendChild(s);
    
    // 尝试多个可能的容器
    const targetElement = 
        document.querySelector('.markdown-preview-view') || 
        document.querySelector('.publish-article-content') ||
        document.querySelector('.content') ||
        document.querySelector('article') ||
        document.querySelector('main') ||
        document.querySelector('[class*="content"]') ||
        document.querySelector('.cm-content-container');
    
    if (targetElement) {
        targetElement.appendChild(container);
        console.log('Giscus: 评论容器已添加到:', targetElement.className || targetElement.tagName);
    } else {
        // 如果找不到合适的容器，添加到 body 末尾
        document.body.appendChild(container);
        console.log('Giscus: 未找到合适容器，评论添加到 body 末尾');
    }
    
    // 添加样式
    if (!document.getElementById('giscus-style')) {
        const style = document.createElement('style');
        style.id = 'giscus-style';
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
    }
    
    console.log('Giscus: 注入完成！');
}

// 监听页面变化（Obsidian Publish 是单页应用）
let lastTitle = document.title;
let checkInterval = null;

function startMonitoring() {
    // 清除旧的监听器
    if (checkInterval) {
        clearInterval(checkInterval);
    }
    
    checkInterval = setInterval(() => {
        if (document.title !== lastTitle) {
            console.log('Giscus: 检测到页面标题变化:', lastTitle, '->', document.title);
            lastTitle = document.title;
            
            // 等待页面内容加载完成
            setTimeout(() => {
                injectGiscus();
            }, 1000);
        }
    }, 500);
}

// 初始注入
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        injectGiscus();
        startMonitoring();
    });
} else {
    injectGiscus();
    startMonitoring();
}

// 监听浏览器的前进/后退
window.addEventListener('popstate', () => {
    console.log('Giscus: 检测到浏览器导航');
    setTimeout(injectGiscus, 500);
});

// 备用：页面完全加载后再次尝试
window.addEventListener('load', () => {
    setTimeout(injectGiscus, 1000);
});