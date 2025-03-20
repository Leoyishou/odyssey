---
draw:
title: 微调实战 -- 用Open-R1微调Qwen模型，强化AI数学推理能力
date created: 2025-03-11
date modified: 2025-03-11
---

## 参考比赛

https://www.kaggle.com/competitions/ai-mathematical-olympiad-progress-prize-2

## 目的和效果

对于这些类型的问题：

- 数学考试题（例如四则运算、代数方程、几何题）；
    
- 数学逻辑推理题；
    
- 数学竞赛题目的推导过程等。
    

微调的意义：

- **微调之前**的模型：可以处理一般对话和指令，但在复杂数学问题面前可能表现一般，回答容易出错或不够严谨。
    
- **微调之后**的模型：在面对**数学推理问题**时，将更擅长：
    
    - **理解数学题意**
        
    - **推导详细解题步骤**
        
    - **最终给出正确的数学答案**
        

---

### 举个例子

**微调前**：

> 问："一个长方形长4米，宽是长的两倍，面积是多少？"
>
> 模型回答可能偏模糊或仅给结果。

- 回答：`面积是长乘宽，所以可能是……`
    

**微调后的模型：**

清晰明确：

`1. 宽是4米（假设），长是宽的两倍，即8米。` `2. 面积 = 长 × 宽 = 8 × 4 = 32平方米。` `答案：32平方米。`

---

## 内容简介

|   |   |   |   |   |
|---|---|---|---|---|
||名称|通俗解释|作用举例|官方地址|
|模型基座|Qwen2.5-1.5B-Instruct|一个通用的人工智能模型（像个聪明但不太专业的学生）|能够回答基本问题，但数学解题能力一般，需要额外训练||
|微调数据集|OpenR1-Math-220k|一个包含约22万道数学题目的数据集，每道题附带清晰的解题过程和答案（就像一本附答案详解的数学练习册）|提供大量题目供模型练习，教会模型解决数学题的思路|[https://huggingface.co/datasets/open-r1/OpenR1-Math-220k](https://huggingface.co/datasets/open-r1/OpenR1-Math-220k)|
|训练框架|Open-R1框架|一个训练工具或平台（相当于专门的数学教练），用来教导模型如何清晰、有逻辑地推理解答数学问题|指导模型如何进行有效的推理训练，使数学能力变强|[https://github.com/huggingface/open-r1?tab=readme-ov-file](https://github.com/huggingface/open-r1?tab=readme-ov-file)|

文档内容：详细介绍了使用AutoDL云平台搭建GPU环境，并以Open-R1项目为实例进行部署与微调训练的全过程。内容涵盖了GPU机器选型、环境配置、依赖安装、项目文件获取、基础模型（Qwen2.5-1.5B-Instruct）及数据集（OpenR1-Math-220k）的下载步骤，以及如何通过修改YAML配置文件参数，启动基于Accelerate框架的监督微调（SFT）。附有配置文件中各参数的详细说明，帮助大家快速上手Open-R1项目的训练实践。

1. ## 买机器、选框架
    

GPU 和框架

4090 + PyTorch / 2.5.1 / 3.12(ubuntu22.04) / 12.4

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=ZTMxYmZiNDE2ODU3ZmFjN2Q2MmE1YTU3OGRhNjFmNzRfY0JVejZGRWNkVU9ZS3VoNVFDM05nTWxKUldGZG5sNE9fVG9rZW46R2phSGJ5YlZjb2M0dGp4eW11emNhaVVrbjdmXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

1. ## 上机器
    

  

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=YjJhNGMzMWI5NmZhYjk3NGZkNDZhNGVhMDc5MTg3MWRfOU93YThCVnEzdGhmRGNLeUU5d0NmQmNFUmVWM2MxdVZfVG9rZW46VHFOUGJnMHFEb0RzeFJ4TGNvWGNxdU91bmtiXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=YzBmNjQ0N2VlMGRjZGQ4MzcwMDI3MTM2YTFlZDczMTVfbjFkR0FxUmc4Qjk2Z1F2QjUyUnVuY0xkQnh2S2RCUVRfVG9rZW46RE1Rc2I3VU5Sb1RHakl4TGhwU2NvQnU3bkllXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

## 开梯子

```sql
source /etc/network_turbo
```

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=MmZhMWEyYmMzMzViMTgxNjg5ZDM0ZjgwMDgwMThlNDRfc0VtbE5VVVpwQ3J1UXZNT2pxRDNveE9YdVpvUDA5MkVfVG9rZW46Q0o0eGJQcVZnb296N1d4OUFUWmNOQzZnblFiXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=Mjg4MjZiNTBiOWE3ZTk1NTI3MjJiZDlmZmFlYjkwYjlfWThDbkF3RjF1N1JVeExJVjRlZWZDdEZCY3pxckxOZnVfVG9rZW46RzBHTmJGSEpwb2Z2OHJ4R3hyeWNhZXpObnNiXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

## 切到数据盘的路径

```Shell
cd root/autodl-tmp/

export HF_HOME=/root/autodl-tmp/huggingface_cache
export HF_DATASETS_CACHE=/root/autodl-tmp/huggingface_cache/datasets
export TRANSFORMERS_CACHE=/root/autodl-tmp/huggingface_cache/transformers

# 原因如下
```

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=OWExZGI3NzYzMjJjNTM2ZWNjZTE2NDcyMmM5NmRiYzJfdUl1WGhMbzZzelR5d20zbFRHTHhGYks4bmdOeERwV3RfVG9rZW46Um5RQWJzNjVrb0ZYZWN4TVRhR2NJMG9Fbk9jXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

## 装依赖

```Shell
# 弄个虚拟环境
uv venv openr1 --python 3.11 && source openr1/bin/activate && uv pip install --upgrade pip

# 安装依赖
pip install vllm==0.7.2
pip install setuptools
pip install flash-attn --no-build-isolation         参数的意思是不用装它的依赖

# 下载项目文件
git clone https://github.com/huggingface/open-r1.git

apt-get update
sudo apt-get install git-lfs
```

```Shell
# 对应的是下面截图中的这些依赖
GIT_LFS_SKIP_SMUDGE=1 pip install -e ".[dev]"
```

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=ZDM4YWFlYzY0NTdkNGEyOTMwMzMzMzk0OGYyMTdlZTBfSDJkN2NOUzBNTU45aGRmSDdud093Rkc3Y05PbDVMZjhfVG9rZW46TXZobWIyM3JBb0dOMzN4WVIyWWNNNW1IbkNoXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

1. ## 下载基座模型和训练数据

```yaml
# 先登录huggingface，根据提示去生成 token
huggingface-cli login

# 下载模型和数据集
huggingface-cli download Qwen/Qwen2.5-1.5B-Instruct
huggingface-cli download open-r1/OpenR1-Math-220k \   --repo-type dataset
```

## 改训练的配置参数

    

1. `/root/autodl-tmp/open-r1/recipes/accelerate_configs/zero3.yaml` 用于配置分布式训练框架（DeepSpeed Accelerate）的运行环境，比如使用多少个GPU、是否启用混合精度（bf16）、显存优化策略（Zero3）等，以提高训练效率和节省资源。

|   |   |   |   |
|---|---|---|---|
|Key|Value|通俗解释|改为|
|compute_environment|LOCAL_MACHINE|计算环境使用本地机器进行训练||
|debug|FALSE|是否启用调试模式（不开启）||
|deepspeed_multinode_launcher|standard|使用标准方式启动多节点训练任务||
|offload_optimizer_device|none|不将优化器计算任务分流到其他设备||
|offload_param_device|none|不将模型参数分流到其他设备||
|zero3_init_flag|TRUE|启用DeepSpeed Zero-3的初始化功能||
|zero3_save_16bit_model|TRUE|保存Zero-3训练后的16位精度模型||
|zero_stage|3|使用DeepSpeed Zero的第三阶段优化||
|distributed_type|DEEPSPEED|使用DeepSpeed作为分布式训练框架||
|downcast_bf16|'no'|不降低bf16精度数据的位宽||
|machine_rank|0|当前机器的编号（单机时为0）||
|main_training_function|main|主训练函数入口名称||
|mixed_precision|bf16|使用bf16混合精度训练，节省显存||
|num_machines|1|训练使用机器总数（单机为1）||
|num_processes|8|同时启动8个并行进程参与训练|1（因为我们不是 8 卡的，只是 1 卡的）|
|rdzv_backend|static|使用静态方式协调进程启动||
|same_network|TRUE|所有机器在同一网络环境中||
|tpu_env|[]|不使用TPU环境||
|tpu_use_cluster|FALSE|不启用TPU集群||
|tpu_use_sudo|FALSE|不使用sudo权限操作TPU||
|use_cpu|FALSE|不使用CPU训练，仅使用GPU||

1. `root/autodl-tmp/open-r1/recipes/Qwen2.5-1.5B-Instruct/sft/config_demo.yaml` 用于配置具体训练任务，包括选择模型、训练数据、训练参数（比如学习率、batch size、训练轮数），以及训练过程中是否将结果推送到远程平台（如Hugging Face或WandB）。

|                             |                                   |                          |                |
| --------------------------- | --------------------------------- | ------------------------ | -------------- |
| 名字                          | 值                                 | 通俗解释                     | 改为             |
| model_name_or_path          | Qwen/Qwen2.5-1.5B-Instruct        | 要微调的基础模型名字或路径            |                |
| model_revision              | main                              | 模型仓库分支（默认主分支）|                |
| torch_dtype                 | bfloat16                          | 模型精度类型，训练更快更省显存          |                |
| attn_implementation         | flash_attention_2                 | 注意力机制实现方式，更快的GPU运算       |                |
| dataset_name                | open-r1/OpenR1-Math-220k          | 用于微调的数据集名称               |                |
| dataset_num_proc            | 8                                 | 并发加载数据集时使用的线程数量          | 8 下载东西时候的并发线程数 |
| bf16                        | TRUE                              | 开启bf16格式（更高效的训练格式）|                |
| do_eval                     | FALSE                             | 是否进行模型评估（这里不评估）|                |
| eval_strategy               | no                                | 评估策略（无评估）|                |
| gradient_accumulation_steps | 1                                 | 梯度累积次数（多少个batch更新一次模型）|                |
| gradient_checkpointing      | TRUE                              | 节省显存，用计算换显存              |                |
| use_reentrant               | FALSE                             | 使用更高效的梯度checkpoint方式     |                |
| hub_model_id                | Qwen2.5-1.5B-Open-R1-Distill      | 模型在Hub上发布的名字（如果推送）|                |
| hub_strategy                | every_save                        | 上传模型到Hub的频率（每次保存都上传）|                |
| learning_rate               | 5.00E-05                          | 学习率，决定模型参数更新幅度           |                |
| log_level                   | info                              | 日志记录级别（一般记录）|                |
| logging_steps               | 5                                 | 每隔多少步记录一次训练日志            |                |
| logging_strategy            | steps                             | 按步数记录日志                  |                |
| lr_scheduler_type           | cosine_with_min_lr                | 学习率调度方式（余弦退火，有最低限度）|                |
| min_lr_rate                 | 0.1                               | 最终学习率会降到初始学习率的10%        |                |
| packing                     | TRUE                              | 是否将多个短样本合并成长序列，加速训练      |                |
| max_length                  | 16384                             | 模型支持的最大token序列长度         |                |
| max_steps                   | -1                                | 最大训练步数（-1代表不限制，按epoch训练）|                |
| num_train_epochs            | 1                                 | 数据集训练的遍数                 |                |
| output_dir                  | data/Qwen2.5-1.5B-Open-R1-Distill | 训练模型保存的文件夹               |                |
| overwrite_output_dir        | TRUE                              | 如果输出目录存在则覆盖              |                |
| per_device_eval_batch_size  | 16                                | 每个设备（GPU）评估时的批次大小        |                |
| per_device_train_batch_size | 16                                | 每个设备（GPU）训练时的批次大小        |                |
| push_to_hub                 | TRUE                              | 是否将模型推送到HuggingFace Hub  | FALSE          |
| report_to                   | wandb                             | 训练指标不上报到任何平台             | "none"         |
| save_strategy               | "steps"                           | 模型保存策略，按步数保存             |                |
| save_steps                  | 100                               | 每100步保存一次检查点             |                |
| save_total_limit            | 1                                 | 最多只保留1个最近的模型检查点          |                |
| seed                        | 42                                | 随机种子，确保结果可重复             |                |
| use_liger                   | TRUE                              | 使用Liger优化库，加快训练          |                |
| warmup_ratio                | 0.05                              | 前5%的训练阶段逐渐升高学习率，避免不稳定    |                |

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=ZDM2NTJmNmZjNDA5ZjZhN2MzYTVjMDFkMmU3ZTc3M2RfU203OVBic3FmMXg0ZDIzQnN2QXFPUFdyOVNnVExtcTNfVG9rZW46VTlndWI1RklSb2IwcW54VWtsVGNNbkhOblBnXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)

## 开始 SFT 微调训练

```sql
accelerate launch --config_file recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py \
    --config recipes/Qwen2.5-1.5B-Instruct/sft/config_demo.yaml
```

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=ZGE2MzEwNjk4YTU4YTFkMWE0MzlkZjVkYTBhMDY2YzJfS1BkMEpCMnNTRFRxNFRWMEVTUWZEanZiV3U3cXNiakdfVG9rZW46QVRudmJnQUMzb1lJSlZ4eW9HeWN5ZERFbndjXzE3NDE2NjI4MDU6MTc0MTY2NjQwNV9WNA)
