---
date created: 2025-03-23
date modified: 2025-07-10
uid: 7a1d0e7d-2df7-438c-ae84-af0ed966b1d9
---

你遇到的报错问题为：

```Java
torch.OutOfMemoryError: CUDA out of memory.
```

简单来说就是：

你当前训练的配置**显存占用超过了GPU可用显存**。

## 具体问题原因

- 模型（Qwen2.5-1.5B）本身虽然不算特别大，但你当前的训练配置中设置了：
  - `--per_device_train_batch_size 8`（每个设备的批次大小）
  - `--max_seq_length 16384`（序列长度达到16384 token，非常长）

这两个因素同时作用，导致显存使用远远超过你的GPU（RTX 4090，24GB）的容量，从而训练启动后直接显存溢出。

## 解决方法（任选一种或多种组合）

| 方法                  | 修改示例                                                          | 效果说明                             |
|-----------------------|-------------------------------------------------------------------|--------------------------------------|
| 降低每次训练批次大小      | 如：`--per_device_train_batch_size 2`或`1`                            | 显著降低显存占用                     |
| 缩短最大序列长度         | 如：`--max_seq_length 4096`或`2048`                                    | 显著降低显存占用                     |
| 使用DeepSpeed的ZeRO-3优化 | 检查确认`recipes/accelerate_configs/zero3.yaml`正确配置了offload      | 进一步节约显存，允许训练更大的模型   |
| 启用更高效的优化策略      | 如gradient accumulation（梯度累计）| 在显存受限条件下增大等效批次大小     |

## 推荐的快速解决方案

建议你先尝试把批次大小降低到1，同时序列长度先降低至8192甚至4096，确保训练能正常跑起来：

```bash
accelerate launch --config_file=recipes/accelerate_configs/zero3.yaml src/open_r1/sft.py \
    --model_name_or_path /root/autodl-tmp/Qwen2.5-1.5B-Instruct \
    --dataset_name /root/autodl-tmp/projects/open-r1/data/OpenR1-Math-220k \
    --learning_rate 1.0e-5 \
    --num_train_epochs 1 \
    --packing \
    --max_seq_length 4096 \
    --per_device_train_batch_size 1 \
    --gradient_checkpointing \
    --bf16 \
    --output_dir data/Qwen2.5-1.5B-Open-R1-Distill
```

确保训练稳定后，再逐步尝试增加批次大小或序列长度，以最大化你的GPU使用效率。

---

## 补充知识点

- 显存占用主要由：
  **模型参数量** + **训练批次大小** + **序列长度** 共同决定。

- 显存溢出时，通常的处理方式依次是：
  1. 降低批次大小（batch size）
  2. 缩短序列长度（max_seq_length）
  3. 启用显存优化技术（gradient checkpointing, gradient accumulation, ZeRO优化等）

以上调整方案即可有效解决你的显存溢出问题。
