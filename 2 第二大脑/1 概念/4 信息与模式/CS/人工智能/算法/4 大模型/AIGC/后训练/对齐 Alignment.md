---
uid: 67ad50b8-ca18-43a5-a26e-6d4d5857ef71
date created: 2025-07-19
date modified: 2025-07-27
---
# Summary


## 选择思路
1. **选 [[RLHF]] 如果**：
    - 你是大公司，有充足资源
    - 需要最好的效果
    - 可以承受复杂性
2. **选 DPO 如果**：
    - 你想快速实验
    - 计算资源有限
    - 偏好数据质量很好
3. **选 GRPO 如果**：
    - 你有多样化的用户反馈
    - 想要平衡效果和效率
    - 正在做前沿研究

目前趋势是从 RLHF 向 [[DPO]] 这样更简单的方法发展，因为大家发现简单方法往往也能达到不错的效果！

# Cues

[模型评估](模型评估.md)

# Notes

| 方法       | 全称                                 | 特点          |
| -------- | ---------------------------------- | ----------- |
| [[PPO]]  | Proximal Policy Optimization       | 使用截断来限制策略更新 |
| [[DPO]]  | Direct Preference Optimization     | 直接优化偏好      |
| [[GRPO]] | Group Relative Policy Optimization | 组内相对优化      |

|特征|RLHF<br>(Reinforcement Learning from Human Feedback)|DPO<br>(Direct Preference Optimization)|GRPO<br>(Group Relative Policy Optimization)|
|---|---|---|---|
|**发布时间**|2017年首次提出<br>2022年被OpenAI推广|2023年5月<br>Stanford提出|2024年<br>DeepMind提出|
|**核心思想**|训练奖励模型，然后用RL优化|直接从偏好数据优化，跳过奖励模型|在一组响应中学习相对偏好|
|**技术流程**|1. SFT (监督微调)<br>2. 训练奖励模型<br>3. PPO强化学习优化|1. SFT (监督微调)<br>2. 直接偏好优化|1. SFT (监督微调)<br>2. 组内相对优化|
|**需要的模型**|3个模型：<br>- Policy模型<br>- Reward模型<br>- Reference模型|2个模型：<br>- Policy模型<br>- Reference模型|2个模型：<br>- Policy模型<br>- Reference模型|
|**数据需求**|需要大量人类偏好标注<br>(通常是成对比较)|同样需要偏好数据<br>但利用效率更高|需要成组的响应排序<br>（不只是成对）|
|**训练复杂度**|**高**<br>- PPO不稳定<br>- 超参数敏感<br>- 需要多阶段训练|**中**<br>- 单阶段训练<br>- 更稳定<br>- 类似监督学习|**中**<br>- 比DPO稍复杂<br>- 但比RLHF简单|
|**计算成本**|**高**<br>需要反复采样和评估|**低**<br>不需要在线采样|**中**<br>需要生成多个响应|
|**优势**|✅ 理论成熟<br>✅ 被广泛验证（GPT-4等）<br>✅ 可以持续改进|✅ 简单高效<br>✅ 训练稳定<br>✅ 不需要训练奖励模型<br>✅ 内存占用少|✅ 更好的泛化能力<br>✅ 利用组内信息<br>✅ 减少噪声影响|
|**劣势**|❌ 训练不稳定<br>❌ 计算成本高<br>❌ 实现复杂|❌ 可能过拟合<br>❌ 理论理解不完全<br>❌ 对数据质量敏感|❌ 相对较新<br>❌ 需要更多推理成本<br>❌ 研究较少|
|**数学目标**|最大化：<br>`E[R(y) - β·KL(π‖π_ref)]`|最大化：<br>`E[log σ(β·(r_θ(preferred) - r_θ(rejected)))]`|最大化：<br>组内响应的相对得分|
|**代码示例**|`python<br># RLHF 伪代码<br>reward_model = train_reward_model(comparisons)<br>for step in range(n_steps):<br> responses = policy.generate(prompts)<br> rewards = reward_model(responses)<br> policy.update_with_ppo(rewards)<br>`|`python<br># DPO 伪代码<br>for batch in preference_data:<br> loss = -log(sigmoid(<br> β * (log_p_preferred - log_p_rejected)<br>))<br> optimizer.step(loss)<br>`|`python<br># GRPO 伪代码<br>responses =[policy.generate(prompt) <br> for _ in range(k)]<br>rankings = get_rankings(responses)<br>loss = group_relative_loss(rankings)<br>`|
|**适用场景**|- 需要最高质量输出<br>- 有充足计算资源<br>- 需要持续改进|- 资源有限<br>- 需要快速迭代<br>- 偏好数据质量高|- 有多样化偏好<br>- 需要鲁棒性<br>- 中等资源|
|**实际应用**|- ChatGPT<br>- Claude<br>- GPT-4|- Zephyr<br>- Neural Chat<br>- 许多开源模型|- 一些最新研究模型<br>- 还在推广中|

## 简单解释

### 用餐厅类比理解

**RLHF**：
- 先训练一个"美食评论家"（奖励模型）
- 厨师做菜 → 评论家打分 → 厨师根据分数改进
- 过程复杂但效果好

**DPO**：
- 直接告诉厨师："顾客更喜欢A菜而不是B菜"
- 厨师直接学习做更像A的菜
- 简单直接但可能过于死板

**GRPO**：
- 厨师同时做5道菜，顾客排序
- 学习"这道菜比那几道都好"的相对关系
- 更全面但需要更多尝试
