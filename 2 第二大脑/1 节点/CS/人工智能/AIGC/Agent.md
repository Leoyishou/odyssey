---
draw:
tags: []
title: Agent
date created: 2023-11-23
date modified: 2024-11-12
---

## tool

- [ ] 多参数输入
	- [ ] 至少得是 agen-type
- [ ] 参数检查，抛出异常
- [ ] catch 异常，作为 result 返回给用户
---

拿到结果，保存记忆，带记忆调用？还是迭代调用

- [ ] 异常处理，补足参数
- [ ] 重新调用
- [ ] 参数检验  
		@model_validator(mode='before')  
		Pydantic 是一个 Python 库，它用于数据解析和验证。它广泛应用于快速创建数据模型，
