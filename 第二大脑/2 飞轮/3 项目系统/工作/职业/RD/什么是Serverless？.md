---
uid: d624b72a-df59-47ce-ab30-277f86d2943d
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

⏺ 你有 3 个 Edge Functions，每个都是典型的 Serverless 事件驱动场景：                                                                                                                                        
                                                                                                                                                                                                            
  ---                                                                                                                                                                                                       
  1. daily-agent-summary - AI Agent 日报                                                                                                                                                                    
                                                                                                                                                                                                            
  触发方式： HTTP 请求（定时任务 Cron 调用）                                                                                                                                                                
                                                                                                                                                                                                            
  逻辑流程：                                                                                                                                                                                                
  请求进来 → 查 Supabase 今日 agent_sessions                                                                                                                                                                
           → 统计工具使用、项目、消息数                                                                                                                                                                     
           → 调 OpenRouter AI 生成日报摘要                                                                                                                                                                  
           → 存入 agent_daily_reports 表                                                                                                                                                                    
           → 同步写入 Notion 数据库                                                                                                                                                                         
                                                                                                                                                                                                            
  对应 Lambda 场景： 定时任务 + 数据聚合 + 多服务串联         

# 🧩 Cues



# 🪞Notes
