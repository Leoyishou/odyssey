---
draw:
tags: []
title: PostgreSQL
date created: 2024-09-15
date modified: 2024-11-12
---

## 数据结构

### JSSONB

使用 `JSONB` 类型的优势包括：

1. 灵活性：可以存储任意数量的例句，而不需要预先定义固定的列数。
2. 查询效率：PostgreSQL 提供了多种 JSON 操作符和函数，可以高效地查询和操作 JSON 数据。
3. 索引支持：可以在 JSONB 列上创建索引，提高查询性能。
	- [ ] 需要在数据库中为 `active_use_scenario` 列创建一个 GIN (Generalized Inverted Index) 索引。
4. 数据完整性：PostgreSQL 会验证插入的 JSON 数据的有效性。
