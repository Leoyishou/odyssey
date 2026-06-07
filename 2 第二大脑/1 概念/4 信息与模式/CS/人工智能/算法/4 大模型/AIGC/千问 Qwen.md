---
uid: a2a81491-7400-4984-bfa7-3dcfc60b6f18
date created: 2025-07-19
date modified: 2025-10-20
---
# Summary

| 模型名称                            | 量化         | 优点               | 潜在缺点                    |
| ------------------------------- | ---------- | ---------------- | ----------------------- |
| Qwen3-30B-A3B-Thinking-2507     | 全精度        | 推理最精确、复杂任务表现最佳   | 显存占用高、推理速度慢             |
| Qwen3-30B-A3B-Thinking-2507-FP8 | FP8（8 位浮点）| 占用显存低、速度快、推理链强   | 少量极端推理任务精度略降            |
| Qwen3-30B-A3B-Instruct-2507-FP8 | FP8（8 位浮点）| 输出直接简洁、速度快、显存占用低 | 缺少显式思考链，复杂推理弱于 Thinking |

## 1. Instruct 与 Thinking 模式的差异

- **Qwen3‑4B‑Instruct‑2507**（非‑Thinking 模式）：专注于提高响应速度和效率，适合聊天机器人、客服等场景，倾向于生成直接的、有指导性的回答，不输出 `<think></think>` 思考块
- **Qwen3‑4B‑Thinking‑2507**：则偏向深度推理，自动触发链式思考（chain-of-thought），适合逻辑或学术推理等复杂任务
    
## 2. 是否为 FP8（量化）版本

- **FP8 版本**：表示该模型已被[[模型量化]]为 $8$ 位浮点格式（FP8），大幅减少了显存和计算资源需求，适合资源有限或注重部署效率的场景，**Qwen3‑4B‑Instruct‑2507‑FP8**：即是 Instruct 模式的 FP8 量化版本（不触发 Thinking 模式）

# Cues

# Notes

## Qwen系列模型完整列表

### 基础语言模型

|模型名称|参数量|类型|上下文长度|适用场景|
|---|---|---|---|---|
|Qwen3-0.5B|5亿|Base/Chat|8K-32K|移动端、边缘设备|
|Qwen3-1.5B|15亿|Base/Chat|8K-32K|轻量级应用、个人设备|
|Qwen3-3B|30亿|Base/Chat|32K|小型服务器、个人电脑|
|Qwen3-7B|70亿|Base/Chat|32K-128K|通用对话、文本生成|
|Qwen3-14B|140亿|Base/Chat|32K-128K|高质量对话、复杂任务|
|Qwen3-32B|320亿|Base/Chat|32K-128K|企业级应用|
|Qwen3-72B|720亿|Base/Chat|32K-128K|旗舰性能、最强推理|
|Qwen1.5-MoE-A2.7B|2.7B激活|MoE|32K|高效推理、参数利用|

---

### 多模态模型

|模型名称|模态|参数量|功能|
|---|---|---|---|
|Qwen-VL|视觉+语言|7B+|图像理解、OCR、视觉问答|
|Qwen-VL-Chat|视觉+语言|7B+|图像对话、图表分析|
|Qwen-VL-Plus|视觉+语言|-|增强版视觉理解（API）|
|Qwen-VL-Max|视觉+语言|-|旗舰级多模态（API）|
|Qwen-Audio|音频+语言|7B+|语音识别、音频分析|
|Qwen-Audio-Chat|音频+语言|7B+|语音对话、声音理解|

---

### 垂直领域模型

|模型名称|专业领域|参数量|核心能力|
|---|---|---|---|
|CodeQwen1.5|代码生成|7B|多语言编程、代码补全、Bug修复|
|Qwen-Coder|代码生成|7B/14B|代码理解、算法实现|
|Qwen-Math|数学推理|7B/14B/72B|复杂数学问题、步骤推理|
|Qwen-SQL|SQL生成|-|数据库查询、SQL优化|
|Qwen-Medical|医疗健康|-|医学知识问答、病历分析|

---

### 检索增强模型

|模型名称|类型|参数量|输出维度|使用场景|
|---|---|---|---|---|
|Qwen-Embedding-7B|向量化|7B|768+|语义检索、RAG召回|
|Qwen-Embedding-14B|向量化|14B|768+|高精度检索|
|Qwen-Reranker-7B|重排序|7B|相关性分数|RAG精排、结果优化|
|Qwen-Reranker-14B|重排序|14B|相关性分数|高质量重排序|

---

### 量化与部署版本

|格式类型|量化精度|显存占用（7B为例）|推理速度|适用场景|
|---|---|---|---|---|
|FP16/BF16|16-bit|~14GB|基准|标准部署|
|INT8|8-bit|~7GB|快|平衡性能|
|INT4|4-bit|~4GB|很快|资源受限|
|GPTQ|4-bit|~4GB|很快|GPU推理优化|
|AWQ|4-bit|~4GB|很快|高精度量化|
|GGUF|多精度|可变|中等|CPU推理、llama.cpp|

---

### 云端API服务（DashScope）

|API名称|类型|特点|价格定位|
|---|---|---|---|
|qwen-turbo|文本生成|快速响应、高并发|经济型|
|qwen-plus|文本生成|平衡性能和成本|标准型|
|qwen-max|文本生成|最强性能|旗舰型|
|qwen-long|文本生成|超长上下文（10M+ tokens）|长文档|
|qwen-vl-plus|视觉理解|图像分析|标准型|
|qwen-vl-max|视觉理解|高级视觉任务|旗舰型|
|text-embedding-v3|向量化|高质量Embedding|按量计费|

---

### Agent与工具能力

|能力类型|支持模型|功能描述|
|---|---|---|
|Function Calling|Qwen-Chat系列|调用外部函数和API|
|Tool Use|Qwen-Agent|使用计算器、搜索引擎等工具|
|ReAct|Qwen-Chat系列|推理-行动循环|
|Code Interpreter|Qwen-Chat系列|代码执行和结果解释|
|Multi-Agent|Qwen-Agent|多智能体协作|

---

### 特殊版本与实验模型

|模型类型|说明|状态|
|---|---|---|
|Qwen-Long|128K-1M tokens上下文|已发布|
|Qwen-RLHF|强化学习优化版本|研究中|
|Qwen-DPO|直接偏好优化版本|研究中|
|Qwen-Distill|知识蒸馏小模型|部分开源|

---

### 模型选型快速参考

|你的需求|推荐模型|部署方式|
|---|---|---|
|移动APP|Qwen3-0.5B/1.5B-INT4|本地GGUF|
|个人电脑聊天|Qwen3-7B-Chat-INT4|Ollama/LM Studio|
|写代码|CodeQwen1.5-7B|VSCode插件|
|看图说话|Qwen-VL-Chat|GPU服务器|
|RAG检索|Qwen-Embedding-7B + Reranker|向量数据库|
|企业级部署|Qwen3-14B/32B|vLLM集群|
|快速原型|qwen-turbo API|云端调用|
|最强性能|Qwen3-72B / qwen-max|GPU集群/API|
|长文档分析|Qwen-Long / qwen-long|GPU服务器/API|
|数学题求解|Qwen-Math|GPU服务器|

---

## 补充说明

1. **参数量**：部分模型有多个规格，表格列出代表性版本
2. **上下文长度**：具体长度因版本而异，持续更新中
3. **API价格**：请查阅阿里云DashScope官网最新定价
4. **开源情况**：Base/Chat系列大多开源，部分垂直模型和API专属


## 模型变体和用途

### 1. **基础版 vs 指令版 vs 思考版**

- **Base基础模型**：预训练的基础模型，可进一步微调
- **Instruct指令模型**：经过微调，能够遵循指令并进行对话交互
- **Thinking思考模型**：专门针对推理任务优化（如新的"Thinking-2507"变体）

### 2. **量化格式**

不同的压缩技术在保持性能的同时减小模型大小：

- **FP8**：$8$位浮点量化
- **GPTQ-Int4/Int8**：$4$位或$8$位整数量化
- **AWQ**：激活感知权重量化
- **GGUF**：针对CPU推理优化的格式
- **MLX**：苹果芯片优化，支持多种精度（$4$位、$6$位、$8$位、bf16）

### 3. **架构变体**

- **A22B系列**：似乎是架构改进版（如$235B-A22B$模型）
- **A3B系列**：另一种架构变体（如$30B-A3B$模型）
