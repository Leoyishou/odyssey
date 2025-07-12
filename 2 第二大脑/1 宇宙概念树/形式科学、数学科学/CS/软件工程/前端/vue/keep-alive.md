---
date created: 2024-06-24
date modified: 2025-07-10
uid: 65d24bfb-7f0c-4b81-8bd5-94d8d9d46a7c
---

使用 keep-alive 可以在组件切换时保持组件的状态，而不需要重新加载或重新渲染

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F24%2F22-29-34-ca5f616bed31a9f48c106d7b8bdb68cf-20240624222934-1b92e6.png)  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F24%2F22-30-00-fc28a54677421af1fc39b0ca6a5cc042-20240624222958-27fc55.png)

```js
router.afterEach((to, _from) => {  
  const keepAliveComponentsName = store.getters['keepAlive/keepAliveComponentsName'] || []  
  const name = to.matched[to.matched.length - 1].components.default.name  
  if (to.meta && to.meta.cache && name && !keepAliveComponentsName.includes(name)) {  
    store.commit('keepAlive/addKeepAliveComponentsName', name)  
  }  
  NProgress.done();  
});
```
