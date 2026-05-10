---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
#  💡Summary  

[[codex]]中多 subAgent 的本质是[[有向无环图 DAG]]

> [!Info]
> Spawn [multiple Claude Code agents](https://code.claude.com/docs/en/sub-agents) that work on different parts of a task simultaneously. A lead agent coordinates the work, assigns subtasks, and merges results.
> 

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/20260416121240196.png)

                                                  
  Ralph 最有意思的地方是：每一轮 Agent 都是无状态的新实例，靠文件系统间接协作。这其实是 Orchestrator + Shared State 的结合：                 
```
  ralph.sh (编排者，bash 循环)                                   
    ↓ spawn                                     
  Claude 实例 #1 → 读 prd.json → 做 US-001 → commit → 写 progress.txt                                  
    ↓ 退出，spawn 下一个                                                                                                                     
  Claude 实例 #2 → 读 prd.json → 做 US-002 → commit → 写 progress.txt                                                                        
    ↓ ...                                                                                                                    
  直到所有 story passes: true → <promise>COMPLETE</promise>    
```

  所以准确说，Ralph 是 模式 2（Orchestrator）为主框架，内嵌模式 1（Generator-Verifier），用模式 5（Shared                                    
  State）做跨轮记忆。三种模式叠加，但每一层都很简单——符合那篇文章的核心建议：从简单开始，哪里卡住再升级。  
# 🧩 Cues

[[SubAgent]]
[[【cc】Agent Team]]


# 🪞Notes

