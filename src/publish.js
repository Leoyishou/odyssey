// publish.js —— 放到仓库根目录
function injectGiscus() {
    console.log('Giscus: 尝试注入评论系统...');
    if (document.getElementById('giscus-container')) {
        console.log('Giscus: 评论容器已存在，跳过注入');
        return;
    }
    const container = document.createElement('div');
    container.id = 'giscus-container';
    container.style.marginTop = '3rem';
    
    // 添加样式以匹配正文宽度
    container.style.maxWidth = '100%';
    container.style.margin = '3rem auto 0';
    container.style.padding = '0';
  
    const s = document.createElement('script');
    s.src = 'https://giscus.app/client.js';
    s.dataset.repo        = 'Leoyishou/odyssey';
    s.dataset.repoId      = "R_kgDOPJw6Yg";
    s.dataset.category    = 'Comments';
    s.dataset.categoryId  = "DIC_kwDOPJw6Ys4CstfW";
    // 获取文档的唯一标识符（按优先级）
    const getDocumentId = () => {
        // 1. 尝试从页面内容中查找 comment_id
        // Obsidian Publish 可能会将 frontmatter 渲染在页面某处
        const pageContent = document.body.innerText;
        const commentIdMatch = pageContent.match(/comment_id:\s*([a-f0-9]{8})/);
        if (commentIdMatch) {
            console.log('Giscus: 找到 comment_id:', commentIdMatch[1]);
            return commentIdMatch[1];
        }
        
        // 2. 尝试从 meta 标签获取（如果 Obsidian 支持）
        const commentIdMeta = document.querySelector('meta[name="comment-id"]');
        if (commentIdMeta?.content) {
            console.log('Giscus: 从 meta 标签找到 comment_id:', commentIdMeta.content);
            return commentIdMeta.content;
        }
        
        // 3. 使用文件路径作为稳定标识
        // 保留完整路径结构，这样即使文件名相同但在不同目录也能区分
        const pathname = window.location.pathname;
        const cleanPath = pathname
            .replace(/\.html$/, '')
            .replace(/\/$/, '')
            .replace(/^\//, '');
        
        if (cleanPath && cleanPath !== 'index') {
            console.log('Giscus: 使用路径作为标识:', cleanPath);
            return cleanPath;
        }
        
        // 4. 最后使用标题
        const title = document.title || 'untitled';
        console.log('Giscus: 使用标题作为标识:', title);
        return title;
    };
    
    const documentId = getDocumentId();
    console.log('Giscus: 最终使用的文档ID:', documentId);
    console.log('Giscus: 当前页面URL:', window.location.href);
    console.log('Giscus: 页面标题:', document.title);
    
    s.dataset.mapping     = 'specific';
    s.dataset.term        = documentId;
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
        console.log('Giscus: 评论容器已添加到:', targetElement.className || targetElement.tagName);
        
        // 获取父容器的计算样式，以确保评论区对齐
        const parentStyles = window.getComputedStyle(targetElement);
        const parentPadding = parentStyles.getPropertyValue('padding-left');
        const parentWidth = parentStyles.getPropertyValue('max-width');
        
        // 如果父容器有特定的内边距或最大宽度，应用到评论容器
        if (parentPadding && parentPadding !== '0px') {
            container.style.paddingLeft = '0';
            container.style.paddingRight = '0';
        }
        
        // 添加响应式样式
        const style = document.createElement('style');
        style.textContent = `
            #giscus-container {
                width: 100% !important;
                box-sizing: border-box !important;
            }
            #giscus-container .giscus {
                max-width: 100% !important;
            }
            /* 确保 iframe 也是响应式的 */
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
  