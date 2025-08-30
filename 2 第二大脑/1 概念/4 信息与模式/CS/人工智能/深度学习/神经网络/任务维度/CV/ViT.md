---
date created: 2025-03-23
date modified: 2025-07-10
uid: 3d956e54-4d0d-4879-8bd7-8707cf8e89d8
---

CV（计算机视觉）中的 **ViT** 全称为 **Vision Transformer**，是一种基于Transformer架构的深度学习模型，主要用于图像分类、检测、分割等视觉任务。

---

## 🚩 ViT（Vision Transformer）的背景

- ViT 由Google Research团队于2020年提出，论文题为：[**"An Image is Worth 16x16 Words: Transformers for Image Recognition at Scale"**](https://arxiv.org/abs/2010.11929)。
    
- 之前Transformer在NLP领域广泛使用（如BERT、GPT系列），但在CV领域主流模型通常是CNN（卷积神经网络），如ResNet、DenseNet等。
    
- ViT的提出标志着Transformer架构正式跨入CV领域，带来了视觉模型设计的新思路。
    

---

## 🖼️ ViT 的核心思想

传统的CNN模型使用卷积提取图像特征，而ViT直接使用**Transformer结构处理图像**：

- 将输入图像拆分成多个固定大小的小块（Patch）。
- 每个Patch通过线性映射（Linear Projection）得到一个特征向量。
- 将这些特征向量视为序列输入到Transformer中。
- 利用Transformer的Attention机制学习图像Patch之间的关联，最终进行图像分类或其他视觉任务。

简言之，ViT把图像"分割成类似文本序列的结构"，并用Transformer捕捉这些序列的内在关系。

---

## 📌 ViT 模型结构示意图

```Java
输入图像 → 拆分成Patch → 线性映射 → 加入位置编码 → Transformer Encoder（Attention层）→ 分类头 → 输出类别
```

具体步骤：

1. **图像分割**：将图片 (如224×224像素) 分割为多个Patch（例如每个16×16像素）。
2. **Embedding**：每个Patch通过一个Linear层映射成固定长度的向量。
3. **加入位置编码**（Positional Embedding）：提供Patch的位置信息（类似Transformer在文本中的应用）。
4. **Transformer Encoder**：多个Transformer层（包含Multi-head Self-Attention和MLP层）对Patch序列特征进行提取和融合。
5. **分类头**：使用一个特殊的分类token（class token）完成图像分类任务。

---

## 🌟 ViT的优势与特点

- **捕获全局信息**：Transformer的Attention机制能够从全局尺度捕捉图像中远距离的依赖关系，而CNN则更多地关注局部信息。
- **可扩展性强**：在大规模数据集和大尺寸模型训练中表现尤为突出。
- **泛化能力强**：在大规模预训练后，ViT在迁移学习任务中表现优异。

---

## 📊 ViT的应用场景

- 图像分类（Image Classification）
- 目标检测（Object Detection）
- 图像分割（Image Segmentation）
- 视频理解（Video Understanding）
- 多模态学习（Multimodal Learning）等。

---

## 🚨 ViT的挑战

- ViT相比CNN需要更多的训练数据才能发挥出色性能，通常需要大规模预训练（如ImageNet-21k、JFT-300M）。
- ViT对于图像分辨率敏感，高分辨率输入可能带来巨大的计算负担。
- 后续发展出许多ViT变种，如Swin Transformer等，以提高计算效率和适配更小的数据规模。

---

## 🔖 常见的ViT变种与发展

- **DeiT (Data-efficient Image Transformers)**：提高了Transformer在小数据集上的性能。
- **Swin Transformer**：提出了层级式结构，引入了局部窗口注意力机制。
- **CvT、CrossViT** 等：结合CNN思想或跨尺度结构进一步提升性能。

---

综上所述，**ViT** 是视觉领域中Transformer架构的重要突破，开创了CV领域新的模型范式。
