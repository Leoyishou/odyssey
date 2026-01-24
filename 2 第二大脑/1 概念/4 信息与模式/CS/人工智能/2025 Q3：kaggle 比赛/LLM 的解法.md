---
uid: bd150f67-6c99-43ba-a28b-5a55f155b301
date created: 2025-08-10
date modified: 2025-08-16
---

# Summary

1. mac 上验证代码
2. 传送数据 `sync_simple.sh`
3. autodl 验证代码`autodl_quick.sh`
4. autodl跑训练`train_autodl.sh`
5. 得到训练结果
```Java
/root/qwen-training/outputs/
	├── checkpoint-500/          # 第一个检查点（500步后）
	│   ├── adapter_config.json
	│   ├── adapter_model.safetensors  # LoRA权重
	│   ├── optimizer.pt
	│   ├── rng_state.pth
	│   ├── scheduler.pt
	│   ├── trainer_state.json
	│   └── training_args.bin
	├── checkpoint-1000/         # 第二个检查点（如果训练超过1000步）
	│   └── ...
	├── adapter_config.json      # 最终LoRA配置
	├── adapter_model.safetensors # 最终LoRA权重
	├── tokenizer.json          # 分词器文件
	├── tokenizer_config.json
	├── special_tokens_map.json
	├── vocab.json
	└── merges.txt
```


## 🔄 使用流程

bash

```bash
# 1. Mac本地测试 - 验证代码逻辑
python scripts/run_train.py --config configs/small_model.yaml

# 2. 代码没问题后，直接上传到AutoDL
scp -r reddit-rule-classification/ root@autodl:/root/

# 3. AutoDL上运行 - 只改配置文件路径
python scripts/run_train.py --config configs/large_model.yaml
```

## 💡 额外建议

可以再加一个中间配置，用于在AutoDL上快速测试：

```yaml
# configs/medium_model.yaml (AutoDL快速测试)
model:
  name: "Qwen/Qwen3-4B-Instruct"  # 4B模型
  use_lora: true
  lora_r: 8
  load_in_8bit: false
  
data:
  max_length: 384
  batch_size: 8
  
training:
  num_epochs: 1  # 只跑1轮
  learning_rate: 3e-5
```

这样可以：

1. **Mac测试**：0.6B模型，确保代码无bug
2. **AutoDL快速验证**：4B模型，1个epoch，确保GPU环境OK
3. **AutoDL正式训练**：14B模型，完整训练




___



# 📝 给Claude Code的完整Prompt


我正在参加Kaggle的"Jigsaw - Agile Community Rules Classification"比赛。这是一个文本二分类任务，需要判断Reddit评论是否违反特定的社区规则。比赛使用AUC作为评分标准。

# 比赛背景
- 数据来源：Reddit评论，包含各种子版块的内容
- 任务：二分类，预测评论是否违反给定规则
- 评估指标：column-averaged AUC
- 特点：标注数据很少（small dev set），需要处理Reddit特有的文本格式

# 我的技术方案
1. **模型选择**：使用Qwen3-14B-Instruct-2507进行微调
2. **训练策略**：LoRA微调（$r=16$），配合8bit量化节省显存
3. **开发流程**：
   - 先在Mac本地用Qwen3-0.6B验证代码
   - 然后在AutoDL的A800 GPU上训练14B模型
4. **数据处理**：需要处理Reddit特殊格式（/s表示讽刺，Edit:标记，TL;DR等）
5. **训练技巧**：K-fold交叉验证，ensemble多个模型

# 项目需求

请帮我创建一个完整的项目，包含以下功能：

## 1. 项目结构
```Java
reddit-rule-classification/
├── data/
│   ├── raw/ # 存放原始train.csv, test.csv
│   └── processed/ # 处理后的数据
├── src/
│   ├── config.py # 配置管理
│   ├── data.py # 数据处理
│   ├── model.py # 模型定义
│   ├── train.py # 训练逻辑
│   └── predict.py # 推理逻辑
├── notebooks/
│   └── eda.ipynb # 数据探索分析
├── configs/
│   ├── small_model.yaml # Mac本地测试用(0.6B模型)
│   └── large_model.yaml # A800训练用(14B模型)
├── scripts/
│   ├── run_train.py # 主训练脚本
│   └── run_predict.py # 推理脚本
└── requirements.txt
```

## 2. 核心功能模块

### 数据处理 (src/data.py)
- 加载train.csv和test.csv
- 清洗Reddit评论文本：
  - 移除/处理 "/s"（讽刺标记）
  - 处理 "Edit:"、"UPDATE:"等编辑标记
  - 处理 "TL;DR"摘要
  - 清理URL、用户标记(u/username)、子版块标记(r/subreddit)
  - 处理emoji和特殊字符
- 创建训练prompt，格式如下：
```Java
任务：判断以下Reddit评论是否违反了指定规则。

规则：{rule_text}

评论：{comment_text}

请回答这条评论是否违反了上述规则（是/否）。
```
- 实现K-fold数据分割（5折交叉验证）
- 数据增强（可选）：paraphrase、同义词替换

### 模型模块 (src/model.py)
- 加载Qwen3模型（支持0.6B测试和14B训练）
- 配置LoRA微调：
- target_modules:["q_proj", "v_proj", "k_proj", "o_proj"]
- $r=16$, $alpha=32$, $dropout=0.1$
- 支持8bit[[模型量化]]加载（for 14B model）
- 自定义分类头

### 训练模块 (src/train.py)
- 实现训练循环，使用Hugging Face Trainer
- 训练参数：
- $learning\_rate: 2e-5$
- $batch\_size: 4$ (14B) 或 $8$ (0.6B)
- $gradient\_accumulation\_steps: 4$
- $num\_epochs: 3$
- $warmup\_ratio: 0.1$
- $fp16: True$
- 评估指标计算（AUC为主，accuracy和F1为辅）
- 模型checkpoint保存
- 早停策略
- 训练日志记录

### 推理模块 (src/predict.py)
- 加载训练好的模型
- 批量推理优化
- Test Time Augmentation (TTA)：
- 对同一文本使用不同prompt模板
- 取平均概率
- K-fold模型ensemble
- 生成submission.csv

## 3. 配置文件

### configs/small_model.yaml (Mac测试用)
```yaml
model:
  name: "Qwen/Qwen3-0.6B"
  use_lora: false
  load_in_8bit: false

data:
  max_length: 256
  batch_size: 16

training:
  num_epochs: 1
  learning_rate: 5e-5
```

### configs/large_model.yaml (A800训练用)

```yaml
model:
  name: "Qwen/Qwen3-14B-Instruct-2507"
  use_lora: true
  lora_r: 16
  lora_alpha: 32
  load_in_8bit: true
  
data:
  max_length: 512
  batch_size: 4
  
training:
  num_epochs: 3
  learning_rate: 2e-5
  gradient_accumulation_steps: 4
  fp16: true
```

## 4. 主执行脚本

### scripts/run_train.py

- 命令行参数：--config, --fold, --debug
- 完整的训练流程：数据加载→模型初始化→训练→保存
- 支持单折训练或全部5折训练
- debug模式：使用10%数据快速验证

### scripts/run_predict.py

- 加载多个fold的模型
- ensemble预测
- 生成符合Kaggle格式的submission.csv

## 5. EDA Notebook

创建一个Jupyter notebook进行数据探索：

- 数据分布分析（标签平衡性）
- 评论长度分布
- 常见违规规则类型
- Reddit特殊格式出现频率
- 样本展示和分析

## 6. 特殊要求

1. **错误处理**：所有模块都要有try-except，特别是GPU OOM错误
2. **日志系统**：使用logging模块，记录训练过程和指标
3. **进度条**：使用[[tqdm]]显示训练和推理进度
4. **内存优化**：
    - 使用gradient checkpointing
    - 及时清理不用的变量
    - 混合精度训练
5. **可复现性**：设置随机种子，保存所有配置

## 7. 使用流程

```bash
# 1. Mac本地测试
python scripts/run_train.py --config configs/small_model.yaml --debug

# 2. AutoDL A800训练单折
python scripts/run_train.py --config configs/large_model.yaml --fold 0

# 3. 训练所有5折
python scripts/run_train.py --config configs/large_model.yaml --all-folds

# 4. 生成预测
python scripts/run_predict.py --config configs/large_model.yaml
```

# 输出要求

1. 生成完整的、可直接运行的Python代码
2. 每个模块都要有详细的docstring和注释
3. 代码要模块化，便于调试和修改
4. 提供requirements.txt文件
5. 在关键步骤打印信息，方便监控进度

请基于以上需求，帮我实现这个完整的Kaggle比赛项目。重点确保：

- 代码的健壮性（错误处理）
- 可以在不同环境运行（Mac小模型测试、A800大模型训练）
- 训练过程可监控、可恢复
- 最终能生成正确格式的submission.csv文件
