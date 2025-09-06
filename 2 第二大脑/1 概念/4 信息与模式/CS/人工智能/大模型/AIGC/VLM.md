---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-08-31
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary



# Cues



# Notes
## VLM 是啥？

**VLM** 的全称是 **Vision-Language Model**，中文常译作「视觉-语言模型」。它是大模型家族中的一个分支，和 LLM（Large Language Model, 大语言模型）相呼应。

### 基本含义

- **LLM**：输入是文本 → 输出是文本。
    
- **VLM**：输入可以包含 **图像/视频（视觉模态）+ 文本**，输出通常是文本（有时也能生成图像、做多模态对齐）。
    

### 典型能力

- **图文理解**：比如给它一张图片，问「这张图里的人在做什么？」
    
- **多模态对齐**：把文字和图像信息对应起来，例如「找到图片里标注的红色框区域是什么物体」。
    
- **应用场景**：图片描述生成（Image Captioning）、视觉问答（Visual QA）、图文检索、辅助医学影像诊断等。
    

---

## 和 MLLM 的关系

有时你会看到 **MLLM (Multimodal LLM，多模态大语言模型)**，它和 VLM 的区别比较模糊：

- VLM 一般特指「视觉 + 文本」两模态；
    
- MLLM 范围更广，可能包含语音、动作、传感器数据等多种模态。
