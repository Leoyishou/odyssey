---
uid: 1b9b29d9-ca93-4041-af69-6651bfc22946
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

通常是这样一条线触发的：

```
用 Docker 部署服务
  → 好奇 Docker 怎么实现隔离的
    → 看到 namespace / cgroup
      → 研究容器安全
        → 接触 seccomp
```

或者：

```
需要做代码沙箱 / 在线判题 / AI 代码执行
  → 必须搞清楚怎么安全隔离用户代码
    → 直接要用这些
```

---

## 想深入的话，推荐路径

**入门**：先把 Docker 原理搞透，推荐直接看 `man 7 namespaces` 和 `man 7 cgroups`，或者搜"Docker 底层原理 namespace cgroup"。

**进阶**：《Linux Kernel Development》或者《Container Security》（Liz Rice 著，O'Reilly），后者专门讲容器安全，非常实用。

**动手**：用 `unshare` 命令自己手动创建一个 namespace，比看十篇文章都清楚：

bash

```bash
# 创建一个独立 PID + 网络 namespace 的 shell
sudo unshare --pid --net --fork --mount-proc bash
# 进去以后 ps aux 只能看到自己
```

---

对于后端开发来说，这些不是日常必须掌握的，但**一旦涉及多租户、代码执行、安全隔离类的业务**，就会变成核心知识点。了解原理之后再看 Kubernetes 的安全策略、云服务的沙箱方案，也会一下子豁然开朗。


# 🧩 Cues



# 🪞Notes
