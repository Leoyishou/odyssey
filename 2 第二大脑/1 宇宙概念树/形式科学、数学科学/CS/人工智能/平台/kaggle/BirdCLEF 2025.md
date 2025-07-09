---
draw:
title: BirdCLEF 2025
date created: 2025-03-24
date modified: 2025-05-12
---

技巧总结：

1. 把声音数据转化成 mel 图做模式提取和类别识别
2. train 数据清晰，而 test 数据嘈杂，所以自己手动给 train 数据加一些噪声，让训练更符合真实情况

## 进度

| 项目   |                                                                                      |
| ---- | ------------------------------------------------------------------------------------ |
| 0512 | 跑这个项目，多尝试几个 backbone  https://github.com/LIHANG-HONG/birdclef2023-2nd-place-solution |
| 0519 | 参数方案                                                                                 |
| 0526 | 多融合几个模型                                                                              |

## run

### 1. 改配置
1. wandb 的 key
2. input 数据的路径
3. cfg.bird_cols_train 的内容改为本次的 206 个种类
### 2.跑方案
1. 只尝试 sed 的，不用试 cnn 的；sed_b3ns.py sed_resnet34.py \ sed_seresnext26t.py sed_v2s.py a
2. sed_v2s.py中下面的 120 改成 12
![CleanShot 2025-05-12 at 00.57.41@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_Cm9OVMGaiR%2F2025%2F05%2F12%2F00-57-48-2880693568a8c8aeb8131cc9c188a721-CleanShot%202025-05-12%20at%2000.57.41-2x-2ccdef.png)
3. 只用 train.ce这个 stage![CleanShot 2025-05-12 at 00.58.45@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_XzkUZ1Ird2%2F2025%2F05%2F12%2F00-59-07-3d070918f46359f4a7209090d36319eb-CleanShot%202025-05-12%20at%2000.58.45-2x-1c9013.png)
[[OpenVINO IR]]







## 1️⃣ 题目场景（Scenario）

- **生态保护背景**  
    赛题来自哥伦比亚马格达莱纳河谷雨林保护与生态修复项目。传统人工观测成本高、覆盖有限，因此希望借助**被动声学监测 (Passive Acoustic Monitoring, PAM)** + 机器学习，在大面积、长时段声景中**自动识别鸟类、两栖类、哺乳类和昆虫**等指示物种，以便持续评估森林修复成效和生物多样性变化。
    
## 2️⃣ 评估指标（Metric）

- **改进的宏平均 ROC-AUC**
    - 对每个物种分别计算 ROC-AUC；
    - **若某物种在该批评估样本里没有正标签，则跳过该类**；
    - 对剩余类别做宏平均（所有被计入类别权重相等）。
        这样的设计既关注整体分类能力，又避免罕见物种在当前批评估中“无正样本”导致分数失真。
        

## 3️⃣ 本质问题（Task Essence）

|维度|说明|
|---|---|
|**任务类型**|**多标签音频分类**：对连续 5 秒声景，输出 _N_ 个物种的出现概率（0–1），每行可同时为多个物种打分。|
|**难点**|① **极度类别不平衡**（稀有/濒危物种样本极少）② 提供的**有标签数据有限**，需利用无标签数据、自监督或半监督技巧③ **提交环境只允许 CPU**，GPU 仅供调试 → 必须注重模型轻量化与推理效率|
|**期望能力**|训练能在小样本场景下泛化的声学特征提取 + 分类模型，并在无需 GPU 的 90 分钟 CPU 运行限制内完成推理与生成 `submission.csv`。|

> 一句话总结：**在资源受限（CPU、有限标注）的条件下，用机器学习从 5 秒声景片段中多标签识别雨林物种，并以宏 ROC-AUC（跳过无正样本类）评估性能。**

下面给你画一张 **“7 阶梯” 框架**（由易到难、由短期到长期），帮助快速建立 BirdCLEF + 2025 赛题的整体认知。你可以把它想成一座逐级攀登的阶梯，每一级都有**目标-方法-产出**三件事。完成上一级产出，就自然进入下一层。

|阶梯|目标|关键方法/动作|典型产出|
|---|---|---|---|
|**0. 赛题规则 & 环境就绪**|明确约束：只能 CPU 90 min、提交 `submission.csv`、宏 ROC-AUC评价|- 阅读官方 Overview / Evaluation - Notebook 设 `accelerator=CPU` 演练一次 “空提交”|“0.5 分” 空提交成功 (合法格式)|
|**1. 数据理解 & EDA**|搞清数据目录、标签分布、样本不平衡程度|- `os.listdir` + `pandas` 浏览 `train_metadata.csv` - Plot 类别频次 Zipf 长尾图|EDA 报告 (markdown + 图)|
|**2. _极简 Baseline_**|能跑通训练 → 生成可提交预测|- **切 5 秒片段**→Mel-Spectrogram - ResNet18 / EfficientNet-B0 微调 - `batch_size=16` CPU 推理脚本|第一次得分 (_≈0.65 AUC_)|
|**3. 数据扩充 & 类别均衡**|提升长尾类别召回|- Mixup、SpecAugment - `WeightedRandomSampler` / focal-loss - 分层 K-Fold 交叉验证|分数 +0.02 ~ 0.05|
|**4. 迁移学习 + 半监督**|用「有限标注 + 大量无标签」|- 预训练 PANNs/HTS-AT 特征 - Pseudo-labeling / Mean-Teacher - BYOL-A / Wav2Vec2.0 冻结特征|分数逼近 0.80 AUC|
|**5. 推理优化 & 集成**|在 CPU 90 min 内压榨极限|- `onnxruntime` / `torch.compile` - Clip-level → Window-level 后处理 (阈值调优) - Rank + Avg 模型集成|0.82-0.85 AUC，推理 < 60 min|
|**6. 误差分析 & 工作笔记**|准备冲击前 5 % + Working Note 奖|- 监听 false-positive/negative 音频 - Species-wise ROC / PR 曲线 - 写 CLEF Working Notes (贡献、原创性、清晰度)|>0.85 AUC & 一篇结构完整的技术论文|

## 使用建议

1. **每日节奏**：0-2 阶一天内能搞定；3-4 阶用一周；5-6 阶根据排期迭代。
    
2. **评估 checkpoint**：每升一阶，跑一次验证集 AUC；同时检查推理时间是否仍 < 90 min。
    
3. **提交策略**：Kaggle 每天 5 次额度——上午调参、下午只保留 _最稳_ 2 次提交。
    

> **记忆口诀：** _规则-理解-跑通-均衡-无监督-压缩-复盘_——依序对齐七层梯子，就能把赛题从“能提交”一路推到“有创新、有论文、能冲奖”。祝你攀梯顺利!

## 提交格式 submission.csv
**一行（一个 `row_id`）= 一段 5 秒声景**，之后的每一列就是“这个物种在这 5 秒里出现的概率”。具体要点：

| 概念       | 说明                                                              |
| -------- | --------------------------------------------------------------- |
| `row_id` | 测试集中唯一编号，格式像 `soundscape_8358733_5`（文件名 + 第几个 5 秒窗口）。|
| 206 个物种列 | 列名是数值 ID（例如 `1139490`），官方在 `train_metadata.csv` 里给了对应学名/俗名的映射表。|
| 单元格数值    | 取值 0–1，代表模型确信程度。**不要求所有列加起来等于 1**，因为一段声景里可能同时出现多种动物。|

[[EfficientNet]] 是一类轻量、高效的卷积神经网络（[[CNN]]）架构，由谷歌提出。

这个代码是用于参加 **BirdCLEF 2025** 比赛的一个机器学习训练流程，核心思想如下：

## 训练数据

每一行是一段 recording，primary_label是这段录音所属的物种

|     | primary_label | secondary_labels | type | filename             | collection | rating | url                                               | latitude | longitude | scientific_name       | common_name           | author            | license         |
| --- | ------------- | ---------------- | ---- | -------------------- | ---------- | ------ | ------------------------------------------------- | -------- | --------- | --------------------- | --------------------- | ----------------- | --------------- |
| 0   | 1139490       |['']|['']| 1139490/CSA36385.ogg | CSA        | 0.0    | http://colecciones.humboldt.org.co/rec/sonidos... | 7.3206   | -73.7128  | Ragoniella pulchella  | Ragoniella pulchella  | Fabio A. Sarria-S | cc-by-nc-sa 4.0 |
| 1   | 1139490       |['']|['']| 1139490/CSA36389.ogg | CSA        | 0.0    | http://colecciones.humboldt.org.co/rec/sonidos... | 7.3206   | -73.7128  | Ragoniella pulchella  | Ragoniella pulchella  | Fabio A. Sarria-S | cc-by-nc-sa 4.0 |
| 2   | 1192948       |['']|['']| 1192948/CSA36358.ogg | CSA        | 0.0    | http://colecciones.humboldt.org.co/rec/sonidos... | 7.3791   | -73.7313  | Oxyprora surinamensis | Oxyprora surinamensis | Fabio A. Sarria-S | cc-by-nc-sa 4.0 |
| 3   | 1192948       |['']|['']| 1192948/CSA36366.ogg | CSA        | 0.0    | http://colecciones.humboldt.org.co/rec/sonidos... | 7.2800   | -73.8582  | Oxyprora surinamensis | Oxyprora surinamensis | Fabio A. Sarria-S | cc-by-nc-sa 4.0 |
| 4   | 1192948       |['']|['']| 1192948/CSA36373.ogg | CSA        | 0.0    | http://colecciones.humboldt.org.co/rec/sonidos... | 7.3791   | -73.7313  | Oxyprora surinamensis | Oxyprora surinamensis | Fabio A. Sarria-S | cc-by-nc-sa 4.0 |
|     |               |                  |      |                      |            |        |                                                   |          |           |                       |                       |                   |                 |
|     |               |                  |      |                      |            |        |                                                   |          |           |                       |                       |                   |                 |

## 1. 比赛背景

BirdCLEF 是一个鸟类声音识别的竞赛，参赛者需要根据提供的鸟类音频数据训练模型，使其能够精确识别音频中的鸟类物种。

## 2. 技术栈

- 使用了 **PyTorch** 和 **Timm库**（提供大量预训练的图像模型）来搭建深度学习模型。
- 音频处理方面使用了 **librosa** 来提取音频特征（Mel频谱图）。

## 3. 核心流程

整个代码流程主要分为几个关键步骤：

###（1）环境和配置设置

- 设定随机数种子（保证实验可复现）。
- 指定数据路径、设备选择（GPU或CPU）、模型类型（EfficientNet）、训练超参数（学习率、批大小、折交叉验证策略）。

###（2）数据预处理（Pre-processing）

- 加载音频文件，使用 librosa 转换为 Mel频谱图（Melspectrogram）。
- 将原始音频数据处理成模型能够直接输入的图像形式（频谱图）。

###（3）数据增强（Augmentation）

- 在数据加载阶段，通过随机裁剪、随机起始位置截取音频片段，增强数据多样性，防止过拟合。
- 利用音频片段动态生成不同的样本，增加训练数据量。

###（4）模型定义与训练

- 使用 EfficientNet 作为基础模型，模型接受频谱图作为输入。
- 配置了学习率调整策略（Cosine Annealing LR Scheduler）。
- 使用交叉熵（CrossEntropyLoss）作为损失函数。

###（5）验证与交叉验证策略

- 采用K折交叉验证（代码中为5折），确保模型在不同的数据子集上泛化性能良好。

## 4. 输出

- 模型训练后会保存权重文件，可用于后续的推理和提交到比赛平台。

---

**第 1 步：提取与审查**  
你当前的疑问是：“我对多层感知机（MLP）有一定了解，但对这个项目（即以上 BirdCLEF 2023 相关的代码）实际上在做什么并不清楚。它到底实现了什么功能，解决了什么问题？”

- 你显式的认知是：MLP 是一种基础的神经网络结构，但对于这些看起来相当复杂、分布在多个文件且应用了许多 PyTorch Lightning、timm、torchaudio 和自定义数据处理的代码，并不确定它具体的功能和流程。
    
- 潜在的隐性假设：
    
    1. 认为代码可能只是简单的神经网络实现（如 MLP），对音频分类的复杂度、数据预处理流程、模型结构（CNN、SED 等）没有足够认识。
        
    2. 可能忽视了项目对音频特征提取、数据增强以及多种神经网络架构（EfficientNet、ResNet 等）的深度运用。
        

---

**第 2 步：显性揭示盲点**

1. **音频识别与多通道输入**
    
    - 这些代码实现了鸟类鸣叫检测或分类的全流程，包括：读取音频文件、做数据增强（如随机噪声、时间掩码、频率掩码等）、提取梅尔频谱或其他声学特征，然后再输入到深度神经网络中进行训练。
        
    - 与普通的 MLP 不同，这里多采用 CNN、SED（Sound Event Detection）模型或混合结构，并需要大量的音频预处理知识。
        
2. **多模型与多配置**
    
    - 代码中可以看到不同的配置文件（例如 `cnn_resnet34d.py`, `cnn_v2s.py`, `sed_v2s.py` 等），每个配置里都有针对模型结构、超参数（如学习率、batch size）、数据预处理细节（如混合器、二次 mixup 等）的定义。
        
    - 这意味着并非只有一个“网络模型”，而是针对不同架构或不同阶段（pretrain_bce、train_ce、finetune 等）执行不同训练策略。
        
3. **混合损失与评估方法**
    
    - 项目中使用了诸如 BCEWithLogitsLoss、CrossEntropyLoss 等不同形式的损失来兼顾多标签分类或多分类问题。
        
    - 还使用了像平均精度（Average Precision，尤其是 label ranking average precision score）以及 C-MAP（Competition Mean Average Precision）等自定义或竞赛指定的指标来评价模型。
        
    - 这些部分对于初学者而言可能不熟悉，但在竞赛或科研中很常见。
        
4. **代码组织与复现**
    
    - 该项目使用了 PyTorch Lightning 简化训练流程、日志记录和保存模型等操作；还可以将训练好的模型导出为 ONNX / OpenVINO 等格式，以便在推理时加速或在不同环境下部署。
        
    - 这些都超越了一个简单 MLP 的范畴，涉及完整的生产或竞赛级别管线（pipelines），对数据加载、采样策略、混合精度训练、分布式等都有考虑。
        

---

**第 3 步：提供权威参照与对比**

1. **学术/业界共识与经典参考**
    
    - **音频分类与 SED**：若想深入理解此类代码，可以参考 IEEE ICASSP 上的相关论文，如在 Sound Event Detection、Bird Audio Detection、Bioacoustics 等领域的研究；以及 DCASE（Detection and Classification of Acoustic Scenes and Events）挑战赛的相关文献。
        
    - **数据增强与自监督学习**：可阅读 Stanford、MIT 等高校课程或 K. P. Murphy 的《Probabilistic Machine Learning》关于数据增强、混合方法（mixup/SpecAugment）以及自监督表示学习的章节。
        
    - **Kaggle/BirdCLEF 竞赛**：BirdCLEF 是由 LifeCLEF 主办的系列挑战赛，官方有许多报告、论文记录和往届解决方案，可以对比你当前的代码加深理解。
        
2. **与“简单 MLP”之对比**
    
    - MLP 通常难以高效地处理原始音频或时频图像（spectrograms），并且对大规模数据的表征能力也有限。因此本项目使用了 CNN 或 SED 架构以捕捉更丰富的时频特征。
        
    - 在音频分类中，CNN、EfficientNet、Transformer 等已成为研究或比赛的主流方法，简单 MLP 较少直接应用于长序列音频数据。
        

---

**第 4 步：提供认知更新与校准建议**

1. **深入理解音频预处理与数据增强**
    
    - 建议先掌握如何将原始波形转换为梅尔频谱（Mel-spectrogram），以及如加性噪声、时频掩码、混合增广（mixup）等在音频分类中常用的技巧。
        
    - 若对 librosa 或 torchaudio 不熟悉，可以查阅官方文档或教程，以了解其在短时傅里叶变换、滤波器处理等方面的实现细节。
        
2. **关注 SED（Sound Event Detection）结构**
    
    - SED 模型通常包含时序注意力（attention blocks）或帧级别预测（framewise output）。这种思路与图像分类的全局特征不同，要尤其注意音频的时序层面。
        
    - 可以查阅如 DCASE 的竞赛论文，或谷歌研究院（Google Research）在音频事件检测方面的开源项目（如 YAMNet、AudioSet），对比它们与这里的实现思路。
        
3. **学习多配置、训练流程与日志管理**
    
    - 这些代码示例在 `train.py` 等文件里调用配置、加载模块、注册回调函数、启用混合精度训练（precision=16/32）或分布式等，能让你了解更完整的工程化训练流程。
        
    - 如果想复现实验，建议使用相同的 conda 或 pip 环境（Python 版本、PyTorch Lightning、timm、torchaudio、librosa 版本等），以免在包依赖上出现问题。
        
4. **实践建议**
    
    - 你可以先在一个小规模数据集上运行这些脚本，并在调试模式（fast_dev_run）或指定少量 epoch 的情况下进行快速测试，观察各个阶段（数据加载、train_step、val_step）如何工作。
        
    - 逐步阅读 `train.py`、`modules/model.py`、`modules/dataset.py` 等文件，掌握项目的核心逻辑和数据流动，再结合你对多层感知机的理论知识加以类比，体会这些 CNN/SED 模型对音频数据的针对性改进和增强。
        

通过以上步骤，你可以逐渐理解这套 BirdCLEF 2023 代码不只是一个“多层感知机”，而是一个包含完整音频处理、特征提取、深度模型（尤其 CNN 或 SED）以及工程化训练管线的综合性项目，能够帮助你从更广阔和系统的角度审视音频分类任务的挑战与解决思路。
