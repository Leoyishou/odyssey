---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-28
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary



# Cues



# Notes

  | 特征              | Encoder | Decoder | Decoder-only |
  |-----------------|---------|---------|--------------|
  | Token Embedding | ✓       | ✓       | ✓            |
  | Self-Attention  | ✓       | ✓       | ✓            |
  | 因果掩码            | ✗       | ✓       | ✓            |
  | 双向注意力           | ✓       | ✗       | ✗            |
  | Cross-Attention | ✗       | ✓       | ✗            |

  Decoder-only = 有 Decoder 的因果注意力，但没有 Encoder 和 Cross-Attention

  这种架构适合自回归语言建模任务，如 GPT 系列模型。
