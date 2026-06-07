---
uid: df31456d-8c70-4792-83b1-eceb3edecf7c
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡Summary

> [!Info]
> 一句话说清楚

流程对比

| 阶段     | Obsidian                                           | App Store                                          |
| :------- | :------------------------------------------------- | :------------------------------------------------- |
| 提交方式 | 向 obsidian-releases 仓库提 PR，只改一个 JSON 文件 | Xcode/Transporter 上传二进制包到 App Store Connect |
| 自动审核 | ObsidianReviewBot 扫描源码（ESLint 规则）          | 自动检测崩溃、私有 API、元数据合规                 |
| 人工审核 | Obsidian 团队成员 review（1-2 周）                 | Apple 审核团队（通常 24-48 小时）                  |
| 审核标准 | 代码质量（类型安全、Promise 处理、UI 文本规范）    | App Review Guidelines（隐私、内容、功能完整性）    |
| 修改反馈 | 直接推代码到你的仓库，bot 自动重新扫描             | 被拒后在 App Store Connect 回复或重新提交          |
| 发布     | PR 合并后自动上架                                  | 审核通过后手动/自动发布                            |
| 后续更新 | 只需创建新 GitHub Release，无需再提 PR             | 每次都要重新提交审核                               |

Obsidian 的具体流程

# 🧩 Cues

# 🪞Notes