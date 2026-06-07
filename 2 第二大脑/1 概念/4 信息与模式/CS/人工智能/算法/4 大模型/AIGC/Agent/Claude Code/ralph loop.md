---
uid: 014276ef-04b2-4fc3-bf3f-f557e717e80d
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡Summary

> [!Info]
> Standard software practices is to build it vertically brick by brick - like Jenga but these days I approach everything as a loop.

| 传统软件开发                            | 他所谓的 Ralph loop                                  |
| --------------------------------------- | ---------------------------------------------------- |
| 人手动拆需求、写代码、调 bug            | 人设计目标、提示词、约束和验证机制                   |
| AI 只是 Cursor/Claude Code 里的提效工具 | AI 是“可编程的新电脑”，工程师是在编程这个 loop       |
| 重点是一次性写出正确代码                | 重点是让失败暴露出来，然后把失败变成下一轮改进       |
| 多 agent / agent 通信看起来很高级       | 他更偏向单进程、单仓库、单任务循环，避免非确定性混乱 |
|                                         |                                                      |
|                                         |                                                      |

| 维度    | 方案一：Infi-loop                      | 方案二：SubAgent                                  | 方案三：外置脚本                                                          |
| :---- | :--------------------------------- | :-------------------------------------------- | :---------------------------------------------------------------- |
| 原理    | 官方提供的 stop-hook 阻止退出，同一 session 循环 | 每个 task 派一个 subagent                          | 每个 task 起一个全新 claude 进程                                           |
| 上下文隔离 | 无（同一 session）                      | 部分（subagent 独立，但对 main agent 有开销）             | 完全隔离（独立进程）                                                        |
| 实现    | 官方的 ralph-loop 插件                  | openspec apply + openspec-task-executor agent | openspec-runner.sh 外置脚本<br><br>https://github.com/snarktank/ralph |
| 适用规模  | 任务少、快速完成                           | 中等任务量（<100 tasks）                             | 大规模任务（理论上无上限）                                                     |
| 主要风险  | 长时运行触发 auto-compaction             | main agent 上下文随 task 数增长                      | 无跨 task 上下文，需靠文件传递状态                                              |



# 🧩 Cues

# 🪞Notes

## 方案一的原理
一句话：利用 Claude Code 的 Stop Hook，在 Claude 想退出时拦截它、把原始任务重新喂给它，强迫它继续干活。

---
流程图
```

用户执行 /ralph-loop "任务描述" --max-iterations 10
       ↓
创建状态文件 .claude/ralph-loop.local.md
  记录：iteration=1, max=10, 原始prompt, session_id
       ↓
Claude 开始干活...完成一轮后想退出
       ↓
【Stop Hook 拦截】stop-hook.sh 被触发
       ↓
  判断逻辑：
  ├── 没有状态文件？→ 放行退出
  ├── iteration >= max？→ 放行退出
  ├── 输出含 <promise>COMPLETE</promise>？→ 放行退出
  └── 都不满足？→ 拦截！
        ├── iteration + 1
        └── 返回 {"decision": "block", "reason": "原始任务prompt"}
               ↓
        Claude Code 把 reason 作为新消息注入对话
               ↓
        Claude 继续干活（同一个 session）

---
关键细节

Stop Hook 返回值协议
{"decision": "block", "reason": "这里的文字会变成新的用户消息"}
这是 Claude Code 的内置协议，reason 内容会直接注入为用户输入，Claude 就继续响应了。

状态文件
---
iteration: 3
max_iterations: 10
session_id: abc123
completion_promise: ALL TESTS PASS
---
完成一个 REST API 的 CRUD 功能，跑测试，直到全部通过

跨 session 保护：状态文件记录了启动时的 session_id，如果另一个 Claude 窗口触发了 hook，检测到 session 不匹配就直接放行，不会互相干扰。

---

```


和方案三（外置脚本）的本质区别

| | ralph-loop | 外置脚本 |
| :--- | :--- | :--- |
| 上下文 | 同一个 session，一直累积 | 每个 task 全新进程，完全隔离 |
| 实现位置 | Claude Code 内部 hook | 外部 bash 循环 |
| 适合场景 | 任务少、快速完成 | 任务多、需要长时间运行 |




## 方案三的核心代码

```bash
OUTPUT=$(claude --dangerously-skip-permissions --print < "$SCRIPT_DIR/CLAUDE.md" 2>&1) || true    
```

## 为啥叫 Ralph
来自《辛普森一家》里的角色 Ralph Wiggum。
Ralph 是个经典的"好心但脑子不太灵光"的角色——总是好意，但会不断犯错、经常答非所问，需要反复引导 and 纠正。
Geoffrey Huntley 用这个角色比喻这种 agentic loop 模式：

| Ralph 角色特征 | Loop 模式对应 |
| :--- | :--- |
| 好心但常犯错 | AI 每轮都在尝试，但不一定一次成功 |
| 需要反复纠正 | 靠 progress.txt 累积教训，下轮改进 |
| 最终还是完成了任务 | 靠"最终一致性"而非首次完美 |

核心思路是：不追求 AI 一次完美输出，而是接受它像 Ralph 一样"歪歪扭扭但最终到达终点"——让 bash loop 提供结构性保障，AI 提供实现能力。

## 用户故事

Story 是敏捷开发里的术语，全称 User Story（用户故事）。                                                     
                                                                                                              
  在 Ralph 的语境里，就是 一个具体的、可独立完成的开发任务，比如：                                            
  { "story": "实现用户登录功能，支持邮箱+密码，失败时显示错误提示" }                                          
                                                                                                              
  为什么叫 story？                                                                                            
  传统敏捷里会写成「作为用户，我想要...，以便...」的格式，但实际用的时候通常就是一句话描述任务。              
                                                                                                              
  Ralph 里 story 的要求：                                                                                     
  - 粒度要合适 — 一个 story 要能在一个 context 窗口内完成实现       
  - 太大就拆小，比如「做一个完整的电商系统」要拆成十几个 story                                                
  - 每个 story 对应 prd.json 里的一条记录，Ralph 每轮循环取一条来做