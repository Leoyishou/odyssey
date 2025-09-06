---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-08-31
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

ReAct 是 2022 年 Google Research 提出的一个 **提示框架**，核心思想是让大语言模型在推理时

**交替进行「思考 (Reasoning)」和「行动 (Acting)」**。

# Cues



# Notes

## 📌 什么是 ReAct 框架？



- **Reasoning**：模型在自然语言里写出它的推理过程（思考链）。
    
- **Acting**：模型调用外部工具或环境（例如搜索引擎、计算器、数据库查询等）。
    
- 两者交替执行，直到得到最终答案。
    

---

## ⚙️ 举个例子

任务：问「法国总统的出生年份的平方是多少？」

- **Reasoning**：法国总统是马克龙，我需要查他的出生年份。
    
- **Action**：调用搜索引擎 → 得到「1977」。
    
- **Reasoning**：现在我需要计算 1977²。
    
- **Action**：调用计算器 → 得到「3,908,529」。
    
- **Answer**：法国总统的出生年份平方是 3,908,529。
    

👉 这就是 ReAct：**推理 + 工具调用链条化执行**。

---

## 🌟 为什么重要？

1. **增强可解释性**：模型输出自己的思路，便于检查和调试。
    
2. **增强能力**：不再局限于模型参数里的知识，而是能调用外部工具。
    
3. **Agent 基础**：大多数 AI Agent 框架（LangChain、LlamaIndex 等）都基于 ReAct 思路。
    

---

✅ 总结一句：
**ReAct 是一种提示框架，把 LLM 变成会“边思考边调用工具”的 Agent，提升了模型的可解释性和实用性。**

---
