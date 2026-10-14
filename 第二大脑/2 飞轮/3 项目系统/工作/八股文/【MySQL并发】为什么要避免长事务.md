---
date created: 2024-08-12
date modified: 2025-07-10
uid: 33cf8be2-56c8-4a5d-b713-be4a7ddcaa36
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F08%2F12%2F11-29-54-afaad237c32da0a46c9a36ac04a76511-202408121129413-3d6a12.png)

回滚日志undolog中保留着过去版本的回滚方法，

只有当没有事务再需要用到这些回滚日志时，回滚日志会被删除。

什么时候才不需要了呢？就是当系统里没有比这个回滚日志更早的 read-view 的时候。

长事务就和老玩家一样，他玩这么长，看到太多老版本了，所以回滚日志就得为他保存这些老版本的回滚，这就会导致大量占用存储空间。

都是短事务的话，慢慢地经历过老版本的人都死光了，undolog也就没必要保存这些老版本了
