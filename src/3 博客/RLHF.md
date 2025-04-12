---
draw:
title: RLHF
date created: 2025-04-06
date modified: 2025-04-06
---

## 步骤21：强化学习对齐（RLHF）

理论：基于人类反馈的强化学习 (RLHF) 是近年用来对齐 (Align) 大型语言模型行为的关键技术。它通过引入人类偏好来优化模型，使模型输出更符合人意图、安全可靠。RLHF流程通常：先有一个预训练语言模型，通过人类标注一些模型输出的质量，训练一个奖励模型打分，然后使用强化学习（如PPO算法）让语言模型尽量生成高分输出，同时不偏离原先语言能力太远。OpenAI 的 InstructGPT、ChatGPT 都应用了RLHF，使得模型愿意遵循指令、减少不良行为。

由于完整RLHF实现复杂，这里用一个极简模拟说明其原理。我们假设一个策略要输出动作，我们通过奖励函数指引其学习目标行为。

编程任务：模拟一个简单环境：策略每回合从 {A, B} 两个动作中选择一个，动作 'A' 的奖励为 +1，'B' 的奖励为 -1。我们的目标是通过强化学习使策略最终总是选 'A'。用概率表示策略，对其进行多轮更新（类似policy gradient），观察概率趋向。

```python
import random
import math

# 初始化策略: 概率p_A（选A的概率）初始为0.5
logit_A = 0.0
logit_B = 0.0

def get_prob_A():
    # 由logit计算softmax概率
    pA = math.exp(logit_A) / (math.exp(logit_A) + math.exp(logit_B))
    return pA

print("初始选择A的概率:", get_prob_A())

# 强化学习迭代
for episode in range(1000):
    # 根据当前策略概率选择动作
    pA = get_prob_A()
    action = 'A' if random.random() < pA else 'B'
    # 获得奖励
    reward = 1 if action == 'A' else -1
    # Policy Gradient 简单更新: 增大选择的动作对应logit按 reward 大小
    if action == 'A':
        logit_A += 0.01  reward  # reward=+1, 增加logit_A
    else:
        logit_B += 0.01  reward  # reward=-1, 增加(因为是负负得正) logit_B？实际这里reward=-1,加上会减少logit_B
    # 简化: 未考虑baseline和严格梯度公式，只做比例更新
final_pA = get_prob_A()
print("训练后选择A的概率:", final_pA)
```

结果验证：初始时 `p(A)=0.5`。经过强化学习迭代，我们期望策略越来越偏向动作A。输出可能：

```Java
初始选择A的概率: 0.5  
训练后选择A的概率: 0.99
```

最终概率应非常接近1，表示策略几乎总是选 'A'。这就模拟了RLHF中模型学会选择人类偏好的行为。在实际RLHF中：

- 动作 = 模型生成的完整回答；
    
- 奖励 = 人类反馈训练的奖励模型打分（高分表示人类喜欢的回答）；
    
- 更新策略 = 用PPO等算法微调模型参数，使其输出分布朝着高奖励方向移动。
    

通过RLHF，大模型可以纠正一些不符合人意的倾向。例如原模型可能喜欢用复杂术语回答，现在人类偏好简单解释，那奖励模型就会对简单答案给高分，于是强化学习让模型学会给出简单回答。

提示：上述策略更新我们使用一个非常简单的“增加所选动作logit”的规则，并未严格遵循RL理论（只是直观演示）。但结果仍有效，因为动作A正向奖励，算法会不断提高A概率。现实用PPO需要维护旧策略、裁剪更新、防止梯度爆炸等，复杂许多。不过核心思想就是引导模型朝人类偏好的方向自我调整。
