---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-09-23
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

> [!info]              
>   结论：95% 准确度在 2016-2017 年首次实现（实验室/特定数据集）                                                               
>   - 但普惠化要等到 2022 年 Whisper 开源后                                                                                                
>  - 这不是近期突破，而是 8-9 年前的成就  
> 

  1990s-2000s: GMM-HMM 时代 (WER 20-30%)                                                                                                 
       ↓                                                                                                                                 
  2010: 深度学习革命开始 (DNN 替代 GMM)                                                                                                  
       ↓                                                                                                                                 
  2015: Google LSTM 语音搜索，错误率降低 49%                                                                                             
       ↓                                                                                                                                 
  2016.10: Microsoft 达到人类水平 (5.9% WER) ← 里程碑                                                                                    
       ↓                                                                                                                                 
  2017.05: Google 达到 95% 准确度                                                                                                        
       ↓                                                                                                                                 
  2022.09: OpenAI Whisper 发布 (多语言通用)                                                                                              
       ↓                                                                                                                                 
  2024-2025: SOTA 降至 4-5% WER

___
Claude Code is unoptimized. How can I tell? I needed to do video transcription and out of box it said to use OpenAI Whisper-1 API, which is practicaly deprecated and 200x slower than Groq's Whisper models out of box.
___




语音识别（Automatic Speech Recognition，ASR）采用业内领先的端到端算法模型，准确地将语音内容转写成文字。

实时语音识别用 WebSocket，不是 HTTP。原因：

HTTP 的问题：
用户说话 → 录完 → 一次性发送整段音频 → 等待 → 返回结果
[------------ 延迟很大 ------------]

WebSocket 的优势：
用户说话的同时:
音频片段1 ──→ 服务器 ──→ "你"
音频片段2 ──→ 服务器 ──→ "你好"
音频片段3 ──→ 服务器 ──→ "你好吗"
...持续双向流...

对比：

| 特性     | HTTP                | WebSocket            |
| :------- | :------------------ | :------------------- |
| 连接方式 | 请求-响应，用完断开 | 长连接，保持打开     |
| 数据方向 | 单向（客户端发起）  | 双向（随时互发）     |
| 实时性   | 差（要等完整数据）  | 好（边说边传）       |
| 适合场景 | 文件上传、API 调用  | 实时语音、聊天、游戏 |

火山引擎 ASR 流程：
1. WebSocket 握手（wss://openspeech.bytedance.com/...）
2. 发送配置（采样率、语言等）
3. 边录音边发送 PCM 音频片段（每 100-200ms 一个）
4. 服务器边识别边返回中间结果
5. 发送 final 标志 → 返回最终结果
6. 关闭连接

这就是为什么你能看到文字"实时跳出来"，而不是说完一大段才显示。
# Cues

# Notes