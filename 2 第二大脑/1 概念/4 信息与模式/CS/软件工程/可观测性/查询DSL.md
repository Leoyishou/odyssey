---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-28
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

## 7. 实际应用建议

| 使用场景      | 推荐方案              | 原因        |
| --------- | ----------------- | --------- |
| 用户名/邮箱查询  | fuzzy             | 单个词的拼写纠错  |
| 商品搜索      | match + fuzziness | 支持多词查询和容错 |
| SKU/ID 查询 | term              | 需要完全精确匹配  |
| 搜索建议      | prefix + fuzzy 组合 | 既支持前缀又容错  |

# Cues



# Notes
