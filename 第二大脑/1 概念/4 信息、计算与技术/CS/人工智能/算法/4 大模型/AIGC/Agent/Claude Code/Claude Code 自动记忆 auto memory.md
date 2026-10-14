---
uid: f7cfbd41-1a1d-4f9e-83e2-8b625d494f28
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

![CleanShot 2026-04-04 at 16.37.14@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2016.37.14%402x.png)

![CleanShot 2026-04-04 at 21.45.29@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2021.45.29%402x.png)




## 自动记忆

Enable or disable auto memory  
启用或禁用自动记忆

Auto memory is on by default. To toggle it, open `/memory` in a session and use the auto memory toggle, or set `autoMemoryEnabled` in your project settings:  
自动记忆功能默认开启。如需切换，请在会话中打开 `/memory` 并使用自动记忆开关，或在项目设置中设置 `autoMemoryEnabled` ：

```
{
  "autoMemoryEnabled": false
}
```

To disable auto memory via environment variable, set `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.  
若要通过环境变量禁用自动记忆，请设置 ` `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` `。

## 不同记忆维度
![CleanShot 2026-04-04 at 22.32.44@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/CleanShot%202026-04-04%20at%2022.32.44%402x.png)


## 记忆腐化


但时间久了，这些记忆文件会越来越乱（叫 memory rot）：

- 旧的笔记堆积
- 矛盾的信息（比如你后来改了技术栈，但旧笔记还在）
- 相对日期（“下周五”）没更新
- 重复内容
- 已经不存在的文件引用

结果就是 Claude 越用越“糊涂”，启动时要读一大堆垃圾上下文，效果变差。

## 做梦清理

Auto-dream 就是专门解决这个问题的「夜间清理工」：

- 它像人类 REM 睡眠一样，在你不使用 Claude 的时候（后台）自动运行。
- 每隔 24 小时 + 至少 5 个 session 后触发一次（也可以手动触发）。
- 具体会做：
    - 扫描所有 Auto-memory 记录
    - 合并重复项
    - 删除矛盾/过时的内容
    - 把相对日期改成绝对日期
    - 精简 MEMORY.md 索引（控制在 200 行以内）
    - 整体重构，让下次启动更快、更准

# 🧩 Cues



# 🪞Notes
