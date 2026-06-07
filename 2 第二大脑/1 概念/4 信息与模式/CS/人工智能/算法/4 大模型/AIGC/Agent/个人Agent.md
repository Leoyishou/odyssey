---
uid: e9fe0bec-38ec-4699-93cf-b7d036fb84a8
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 


```
  飞书 ──WebSocket──> Claude (调度 session，固定不变)
                           │
                           ├──> claude -p "任务1" (独立进程)
                           ├──> claude -p "任务2" (独立进程)
                           └──> claude -p "任务3" (独立进程)
                           
                           
                           
## 本地OS启动Agent，这个会监听一个websocket
source ~/.bash_profile && claude --dangerously-skip-permissions --dangerously-load-development-channels plugin:feishu-channel@g-claude-code-plugins

## 飞书机器bot早已连接好上述的端口


## 通信

飞书发消息给机器人，机器人拉起新session，完成任务后返回

```




# 🧩 Cues



# 🪞Notes
