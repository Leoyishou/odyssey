---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary

> [!info]
> Linear 是 Issue-centricIssue-centric（以事项为中心）
> 瑞士军刀软件 vs 有观点的软件

只有 Issue 才有明确的“状态”
Project 是模糊的： 一个项目完成了 45% 还是 55%？这往往是主观的，或者是根据 Issue 数量计算出来的虚数。

Issue 是二元的： 一个 Issue 要么完成了，要么没完成。

![CleanShot 2026-02-03 at 01.11.31@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_WzphbSSqNS%2F2026%2F02%2F03%2F01-11-46-51ac2c814020a3bf145b32ef9ccab15f-CleanShot%202026-02-03%20at%2001.11.31-2x-a21518.png)

- **Triage (分诊台)**：这是 Linear 与 Jira 等传统工具最大的区别之一。新问题不会直接进入待办列表，而是先进入 Triage，由负责人决定是“接受”、“拒绝”还是“放入 Backlog”。
- **Cycles (周期)**：自动化的 Sprint 管理。Linear 会自动处理未完成任务的滚动（Rollover），不需要手动迁移。
- **Command Menu (Cmd+K)**：这是 Linear 的灵魂。几乎所有的操作（改变状态、指派、切换视图）都在这个全局指令菜单中完成，极度依赖键盘操作。
- **Issue Relations (关联)**：非常强调 GitHub PR 的自动关联（当 PR 合并时自动关闭 Issue）。

Linear 不让你随便改。它强行规定了一套它认为最优秀的研发流程（The Linear Method）。

比如它强制使用 Cycle（周期）而不是 Sprint，它对 Issue 的状态流转有严格定义。这反而让团队不需要思考“怎么用工具”，直接照着做就是最佳实践。

**一句话评价：** Linear 是目前硅谷 AI 创业公司和高增长团队的**默认标配**。如果你是做互联网产品的（尤其是 20 人 - 200 人的敏捷团队），用 Linear 会让整个团队感觉自己在做一件“很酷”的事情。

# 🧩 Cues

### PM 视角的工作流建议

既然你之前问到了 **Linear**，现在的最佳工作流应该是这样的：

1. **发现问题：** 你自己试用网站，发现 Bug 或缺功能。
2. **记录（Linear）：** 在 Linear 里建一个 Issue，比如“增加手机号登录功能”。
3. **执行（Replit Agent）：** 把 Linear 的这个 Issue 内容复制给 Replit Agent，让它写代码。
4. **验证：** 部署后，测试通过，去 Linear 关掉 Issue。

# 🪞Notes

**Linear 不是卖功能的，它是卖“工作方式”的。**
- [[Jira]] 的逻辑是：**管理者**需要掌控一切，需要统计报表，需要复杂的权限控制。（服务于老板）
- **Linear** 的逻辑是：**创造者**需要专注，需要快，需要不被打扰。（服务于干活的人）

### . 对待“未完成任务”的态度：是一种“罪过” vs 自然现象

- **传统 Scrum Sprint (Sprint Fail):**
  - **逻辑：** Sprint 开始前，团队要进行详细的估算（Planning Poker），承诺“这两周我们要交付 30 个点”。
  - **痛点：** 到了周五下午，如果还剩 3 个任务没做完，这就是 **Sprint Failed（冲刺失败）**。在回顾会（Retro）上，大家要分析“为什么没做完？”“是不是估算不准？”。
  - **心理后果：** 这种机制会产生一种**负罪感**。为了避免“失败”，大家下次会故意少估算一点（摸鱼），或者加班赶工导致代码质量下降。这就是“永远有做不完的需求”带来的焦虑。
- **Linear Cycle (Auto-Rollover):**
  - **逻辑：** Cycle 是一个时间容器（比如两周）。时间一到，Cycle **自动关闭**。
  - **解法：** Linear 有一个非常人性化的默认设置——**自动顺延（Auto-rollover）**。
  - **体验：** 周期结束时，没做完的任务会自动漂流到下一个周期，或者回到 Backlog，系统不会弹窗警告你“目标未达成”。Linear 认为：**软件开发充满了不确定性，预估不准是常态。** 只要大家在干活，没做完就顺延，不需要这就此事进行“自我检讨”。

## Document

Linear 确实支持 Document，而且你已经有一个了：「人物木板画 Prompt 模板」（关联到「人物语录系列」项目）。

| 对象     | 用途                                                       |
| :------- | :--------------------------------------------------------- |
| Document | 存放 Prompt 模板（如「手绘白板风格」「人物语录风格」）     |
| Issue    | 每期内容，description 里加一行 Prompt 模板: [文档名](链接) |

我可以帮你：
1. 把「手绘白板风」的 prompt 创建成一个 Document
2. 在 Issue 创建时自动关联 Document
3.