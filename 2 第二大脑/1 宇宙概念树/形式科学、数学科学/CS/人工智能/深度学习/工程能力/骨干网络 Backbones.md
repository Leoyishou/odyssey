---
draw:
title: 骨干网络 Backbones
date created: 2025-05-12
date modified: 2025-05-12
---
```Java

计算机科学
└── 人工智能（AI）
    └── 机器学习（ML）
        └── 表征学习 / 表示学习（Representation Learning）
            └── 深度学习（Deep Learning）
                ├── 训练范式
                │   ├── 监督学习
                │   ├── 自监督 / 预训练 ✧
                │   └── 迁移学习 / 微调 ✧
                └── 神经网络架构（Neural Network Architectures）
                    ├── 感知机 & 多层全连接
                    ├── 卷积网络（CNN）
                    ├── 循环 / 时序网络（RNN, LSTM, GRU）
                    ├── Transformer 系列
                    ├── 图神经网络（GNN）
                    └── ……       
                    └─┬─ **骨干网络 Backbones ✧**   ←※ 目标概念
                      │   （负责特征抽取的主干）
                      │
                      ├── 图像领域
                      │   ├── VGG / ResNet / DenseNet
                      │   ├── EfficientNet / ConvNeXt
                      │   └── Swin / ViT 等视觉 Transformer
                      │
                      ├── 语音与音频
                      │   ├── CNN14 / HTS-AT
                      │   └── CRNN / SED-CRNN
                      │
                      ├── 自然语言处理
                      │   ├── BERT / RoBERTa
                      │   └── GPT / LLaMA / T5
                      │
                      ├── 多模态
                      │   ├── CLIP-ViT
                      │   └── SAM-ViT / BEiT-3
                      │
                      └── 自定义派生
                          └── 任务-specific Head（分类、检测、生成…）

```

## 如何阅读这棵树

1. **定位大类**：Backbone 属于 _深度学习_ → _神经网络架构_ 这一支。
    
2. **功能角色**：它是“抽特征的主干”，通常与上层的 _迁移学习 / 预训练-微调_ 范式配合。
    
3. **跨领域扩展**：图像、音频、文本、多模态各自都有常见的 backbone 实现。
    
4. **向下衍生**：在 backbone 之上再挂“任务专属的小脑袋（head）”即可形成完整模型。
    

> ✧ 星号位置表示：
> • **自监督 / 预训练** 是生成高质量 backbone 的主流手段；
> • **迁移学习** 则是把通用 backbone 迁到特定任务/数据集时常用的套路。
