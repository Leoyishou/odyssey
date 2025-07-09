---
draw:
title: lora
date created: 2025-06-17
date modified: 2025-06-19
---

**LoRA = Low‑Rank Adaptation**  
它是一种**参数高效微调（PEFT）**方法，由 Microsoft 在 2021 年提出，专门用来把大模型的“迁移成本”降到传统全参微调的千分之一甚至万分之一。核心思想是：

W′  =  W  +  BA,A∈Rr×d,  B∈Rd×r,  r≪d\mathbf W' \;=\; \mathbf W\;+\;\mathbf B\mathbf A, \qquad \mathbf A\in\mathbb R^{r\times d},\; \mathbf B\in\mathbb R^{d\times r},\; r\ll d

> • 冻结预训练权重 W\mathbf W；
> • 只训练两个低秩矩阵 A,B\mathbf A,\mathbf B（秩 _r_ 很小，通常 4–16）；
> • 推理时把 BA\mathbf B\mathbf A 与原权重相加即可，不引入额外延迟。([arxiv.org](https://arxiv.org/abs/2106.09685?utm_source=chatgpt.com "LoRA: Low-Rank Adaptation of Large Language Models"))

相较于全参微调，LoRA **可将可训练参数减少 10 000 ×、显存占用减少 3 ×**，而在 GPT‑3、RoBERTa 等模型上的效果与全参持平甚至更好。([arxiv.org](https://arxiv.org/abs/2106.09685?utm_source=chatgpt.com "LoRA: Low-Rank Adaptation of Large Language Models"))

---

## 为什么 `llama3_lora_sft.yaml` 里有 “lora”？

在 **LLaMA‑Factory** 中，示例 YAML 的命名规则一般是

```Java
<模型简称>_<微调算法>_<任务>.yaml
```

|片段|含义|
|---|---|
|`llama3`|以 Meta‑Llama‑3 系列为基础模型|
|`lora`|使用 **LoRA** 作为微调算法|
|`sft`|所做任务是 _Supervised Fine‑Tuning_|

打开官方文档给出的范例文件可以看到关键信息：

```yaml
stage: sft
finetuning_type: lora     # 指定使用 LoRA
lora_target: all          # 给哪些线性层插 LoRA，默认 all
```

([aidoczh.com](https://www.aidoczh.com/llamafactory/en/getting_started/sft.html "SFT 训练 - LLaMA Factory"))

---

## YAML 中与 LoRA 相关的常见字段

|字段|作用|常见取值|
|---|---|---|
|`finetuning_type`|选择算法|`lora` / `qlora` / `full` / `freeze`|
|`lora_target`|在哪些层注入 LoRA|`all` / `q_proj,v_proj,...`|
|`lora_rank`|秩 _r_|4 – 32，显存越低可适当取小|
|`lora_alpha`|缩放系数 α\alpha|16 或 32（影响学习率尺度）|
|`lora_dropout`|随机失活率|0 – 0.1，防过拟合|

> **调参提示**：显存紧张时优先降低 `lora_rank` 或只给注意力/FFN 层上 LoRA，而不是一味减 batch size。

---

## LoRA 与 QLoRA、全参微调的区别

|方案|训练参数量|显存占用|推理速度|典型场景|
|---|---|---|---|---|
|Full FT|100 %|100 %|不变|研究机构、有数百 GB 显存|
|**LoRA**|0.1 % – 1 %|↓60 % 左右|无额外开销|单卡 24 GB GPU 就能调 8 B 模型|
|**QLoRA**|同 LoRA，但基座 4‑bit 量化|再省 40 % 显存|略有量化开销|显存 ≤ 12 GB 的消费级 GPU|

---

## LoRA 在 LLaMA‑Factory 的完整闭环

1. **训练**
    
    ```bash
    llamafactory-cli train examples/train_lora/llama3_lora_sft.yaml
    ```
    
2. **验证/推理**
    
    ```bash
    llamafactory-cli chat examples/inference/llama3_lora_sft.yaml
    ```
    
3. **合并权重（可选，部署更方便）**
    
    ```bash
    llamafactory-cli export examples/merge_lora/llama3_lora_sft.yaml \
      adapter_name_or_path=saves/llama3-8b/lora/sft \
      output_dir=saves/llama3-8b/merged/sft
    ```
    

如此，你只需保存几百 MB 的 LoRA 适配器文件，就能把 80 GB 的大模型“定制”为你的专用版本；需要“打包”时再一键合并成完整权重即可。
