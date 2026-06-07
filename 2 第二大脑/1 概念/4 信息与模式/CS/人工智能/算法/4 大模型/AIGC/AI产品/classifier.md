---
uid: 6564838a-b3a0-45fc-a70b-d44ac1b36ae4
date created: 2025-07-19
date modified: 2025-10-07
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 
## 更安全的替代方案：Auto 模式

Auto 模式旨在替代 `--dangerously-skip-permissions`，同时不带来中断。当 classifier 拦截某个操作时，Claude 不会暂停等待输入，而是自动尝试更安全的替代方案。如果一个 session 累计 3 次连续拦截或 20 次总拦截，才会升级到人工。 [Anthropic](https://www.anthropic.com/engineering/claude-code-auto-mode)

bash

```bash
# 更安全的方式
claude --permission-mode auto -p "你的任务"
```

---

## 务必在容器内使用




## Classifier 是什么

Classifier 是一个**转录分类器**，它接收用户消息和 agent 的工具调用作为输入（Claude 自己的文字和工具返回结果会被剥离掉），来判断某个操作是否危 [Anthropic](https://www.anthropic.com/engineering/claude-code-auto-mode)险。

本质上是一个**独立的小模型**，专门用来审查 Claude Code 的行为，不是 Claude 本身在自我审查。

## 它怎么工作

Classifier 分两阶段运行：先用一个快速的单 token 过滤器（判断"是/否"拦截），只有第一层标记了问题才会触发第二层的 chain-of-thought 推理。这样大多数操作在第一层就通过了，推理 token 只花在真正有疑问的地方。 [Anthropic](https://www.anthropic.com/engineering/claude-code-auto-mode)

## 为什么故意不让它看 Claude 的输出

Classifier 只看用户消息和工具调用，Claude 自己的文字被剥离，是为了防止 Claude 生成说服性的理由（比如"用户之前已经隐式同意了这个操作"），把 classifier 说服成做出错误判断。 [Anthropic](https://www.anthropic.com/engineering/claude-code-auto-mode)

## 它管子 agent 吗

管，而且有两个检查点：spawn 时（子 agent 启动前评估任务描述）+ return 时（子 agent 跑完后审查完整行为历史）。Return 检查存在的原因是，子 agent 启动时是安全的，但可能在运行中被它读取的某个文件里的 prompt injection 攻击给污染了。 [Claude](https://code.claude.com/docs/en/permission-modes)

## 和 `--dangerously-skip-permissions` 的关系

你截图里是 **bypass 模式**，classifier **不运行**，这就是为什么叫"dangerously"——连这层安全网都没有了。Auto 模式才会启用 classifier。

# 🧩 Cues



# 🪞Notes
