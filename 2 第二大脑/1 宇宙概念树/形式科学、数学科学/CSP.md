---
comment_id: e7053ee3
date created: 2025-04-13
date modified: 2025-04-15
draw: null
title: CSP
---
## 总结

总体来说，Chrome 插件能弹出侧边栏而不触发 CSP，主要是因为它利用了扩展的隔离世界及 iframe 加载扩展内部页面的机制，并且严格遵守了 CSP 的要求，比如避免内联代码和动态生成的代码全部来自扩展包内预定义的资源。这样，侧边栏既能实现丰富的功能，又不会违背目标网站的内容安全策略。

CSP（内容安全策略，Content Security Policy）是一种安全机制，可以通俗地理解为网站或应用的"安全防护墙"。它的主要作用是帮助网站管理员控制哪些资源可以被加载和执行，从而防止各种常见的网络攻击，特别是跨站脚本攻击（XSS）。

用日常生活来打个比方：

想象你有一个家，CSP就像是你家门口的安保系统。这个系统会检查每个想要进入你家的人：

- 你可以设置规则："只允许家人和已登记的朋友进入"
- 或者更具体："只允许从正门进入，不允许从窗户或后门进入"

在网络世界中：

- CSP让你指定哪些服务器的资源可以被加载（比如脚本、样式、图片等）
- 可以指定哪些内联脚本可以执行
- 可以限制哪些插件可以运行
- 甚至可以控制表单提交的目标地址

对于浏览器插件来说，*CSP尤为重要，因为插件具有比普通网页更高的权限，所以更需要严格的安全策略*。如果你的Edge插件要使用Firebase，你需要在manifest.json文件中明确声明允许与Firebase相关的域名通信，否则浏览器会出于安全考虑阻止这些连接。

简单来说，CSP就是告诉浏览器："这些资源是安全的，可以加载；那些资源是不安全的，禁止加载。"

从 Manifest V3 开始（Edge 插件基于 Chromium 内核，跟 Chrome 的扩展机制基本类似），对扩展的 CSP（Content Security Policy）确实变得非常严格：**不允许**在 `script-src` 里面直接使用远程脚本来源（例如 `https://www.gstatic.com/` 这类域名），同时也默认不允许 `eval`、`inline` 脚本等。

在这种机制下，如果你想在扩展的页面（例如 Options Page、Popup Page 或 background service worker）中使用 Firebase SDK 这种需要从 Google 静态资源库（`https://www.gstatic.com`）直接加载远程脚本的做法，**几乎就是行不通的**。因为只要在 `content_security_policy.extension_pages` 中写了 `https://www.gstatic.com`，就会被判定为“insecure CSP value”。这**并不是**你的写法问题，而是 **Manifest V3 明确禁止**这么做。

---

## 可能的解决思路

虽然说“无解”比较夸张，但在 MV3（或 Edge 的新插件规范）环境下，确实**没有办法在扩展页面中引用远程脚本**。通常有这几种思路可以尝试：

1. **把 Firebase 脚本打包到扩展里本地化**
    
    - 你可以把 firebase.js、firebase-app.js、firebase-auth.js 等脚本都直接下载下来，然后放进扩展的 `js/` 文件夹中，作为本地脚本引用。
        
    - 在你的 `manifest.json` 里，`content_security_policy` 可以只写 `script-src 'self'`（默认就会允许扩展包内的脚本）。
        
    - 缺点是：一旦 Firebase 升级，你需要手动更新扩展里的脚本；而且你需要确认下载的 SDK 是否许可你以这种方式二次分发（一般来说 Firebase SDK 应该是可以的）。
        
2. **改用后端 API 或者通过 service worker 做一些代理**
    
    - 有些 Firebase 功能（比如 Realtime Database、Firestore、Auth 等）其实可以通过后端服务调用、或通过 fetch 请求来完成，而不是**必须**在前端加一段远程 script。
        
    - 换言之，你可以在扩展内部写逻辑，用 `fetch` / `XMLHttpRequest` 去访问 Firebase 的 REST API（若有），或结合一些后端函数（Cloud Functions）来做处理，而不是纯前端 SDK。
        
    - 这种做法开发量会比较大，但好处是摆脱了直接引入 Firebase 脚本的硬性需求。
        
3. **如果必须使用远程脚本，改用 Manifest V2**（但已经逐步被淘汰，不推荐且会失去后续支持）
    
    - Manifest V2 在过去对远程脚本的限制相对没那么严格，可以通过 `content_security_policy` 里加上远程域名的方式绕过。但是现在 **Chrome/Edge 已经不鼓励**，并且后面会**强制升级**到 MV3，V2 迟早会被弃用。所以并不是长久之计。
        
4. **使用 Webpack / Rollup 等打包工具**
    
    - 如果你想使用 Firebase 全家桶，但又不想在浏览器里直接 `import 'https://www.gstatic.com/...'`，可以先把 Firebase SDK 装到本地项目（`npm install firebase`），然后用打包工具把你的代码和 Firebase SDK 都打包编译到一个最终的 JS 文件中，再在扩展的 HTML 里以 `<script src="bundle.js"></script>` 的形式导入。这样依旧走的是 `'self'` 脚本来源。
        
    - 这种是比较常见的做法，可以自动把 Firebase SDK 的依赖打进同一个 bundle 里，完全不需要在线引用。
        

---

## 小结

所以你看到的那个报错——

```Java
Error'content_security_policy.extension_pages': Insecure CSP value "https://www.gstatic.com/" in directive 'script-src'.
```

**不是你写法问题**，也不是 Firebase 本身哪里有 Bug，而是 **Manifest V3 / Edge 扩展对安全策略的硬性要求** 所导致，必须去掉这类远程脚本资源引用。

**要解决的话只能把 Firebase SDK 打包到扩展里本地化，或者你只通过扩展后端请求 Firebase API，不能直接在扩展里插入远程脚本。** 这也是官方推荐的解决方案，没有捷径可以绕过 CSP。

因此，可以说它对“在线加载脚本”这件事本质上是**无解**的，但通过**把脚本本地化**或**改走别的后端/打包路径**，依旧可以使用 Firebase，并不是完全不能用。希望对你有所帮助。
