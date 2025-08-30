---
date created: 2024-07-17
date modified: 2025-08-09
uid: 3c07133f-c8f3-40be-aeed-befd62f7040b
---
这就是为什么现在做NLP任务，HuggingFace已经成为事实标准的脚手架了！比从头写PyTorch训练循环要方便太多。



```bash
huggingface-cli login
```


```Java
from transformers import AutoModelForCausalLM, AutoTokenizer

# 替换为你的模型本地路径
model_path = './'

# Hugging Face 模型仓库名称：你的用户名/新模型名
repo_name = 'LuisLeonard/Qwen2.5-1.5B-Open-R1-Distill'

# 加载模型和 tokenizer
model = AutoModelForCausalLM.from_pretrained(model_path)
tokenizer = AutoTokenizer.from_pretrained(model_path)

# 上传到 Hugging Face
model.push_to_hub(repo_name)
tokenizer.push_to_hub(repo_name)

```

```Java
git lfs install

git clone https://LuisLeonard:${HF_TOKEN}@huggingface.co/LuisLeonard/Qwen2.5-1.5B-Open-R1-Distill

git clone https://huggingface.co/LuisLeonard/Qwen2.5-1.5B-Open-R1-Distill
cd Qwen2.5-1.5B-Open-R1-Distill
cp -r /path/to/your/model/files/* .
git add .
git commit -m "Add fine-tuned model"
git push

```

机器学习界的 github

Huggingface 超详细介绍 - 基本粒子的文章 - 知乎  
https://zhuanlan.zhihu.com/p/535100411

下面用 Markdown 表格通俗直观地对比一下 **Hugging Face、GitHub、Kaggle**：

|维度/平台|Hugging Face 🤗|GitHub 🐙|Kaggle 🥇|
|---|---|---|---|
|**主要用途**|AI模型、数据集分享与部署|源码托管、代码协作|数据科学比赛、数据分析|
|**社区特点**|AI研究人员、模型开发者|程序员、开源贡献者|数据科学家、数据分析师|
|**主打内容**|模型、数据、模型微调、推理|代码仓库、开源项目|公开数据集、竞赛、笔记本|
|**算力支持**|提供免费算力用于模型推理（Spaces）|不提供免费算力|提供免费 GPU、TPU 用于数据分析|
|**最常见的用户目的**|使用或微调已有的预训练模型|协作开发代码、分享项目|参与数据竞赛、练习机器学习|
|**交互模式**|模型与数据下载、在线部署、在线推理|代码克隆、PR 协作、Issue 沟通|上传数据集、编写 Notebook|
|**擅长的领域**|NLP、CV、语音等AI领域|所有领域的软件开发|数据分析、机器学习、AI竞赛|
|**典型场景举例**|微调一个BERT模型、发布AI应用|开发开源库、团队合作开发|预测房价、图像分类比赛|

简而言之：

- **Hugging Face 🤗** 聚焦于AI模型共享、交流和应用。
- **GitHub 🐙** 是开源代码和项目协作的中心。
- **Kaggle 🥇** 则更侧重于数据科学的实践和竞赛。

尽管关注的侧重点不同，但**Hugging Face** 和 **Kaggle** 经常被用户共同使用：

- 很多Kaggle用户会使用Hugging Face上的预训练模型进行比赛。
- Kaggle上公开的数据集也会被用户放到Hugging Face数据仓库进行更方便地调用和传播。
- 一些模型训练、调优经验常常在两个平台之间互相分享。

___

```java
model_name = "sentence-transformers/all-MiniLM-L6-v2" 
tokenizer = AutoTokenizer.from_pretrained(model_name) 
model = AutoModel.from_pretrained(model_name)
```

- [分词器](分词器.md) 可以生成 [token](token.md)，作为模型的 input
- model 可以用输入的 embedding 计算得到输出结果
- [数据集](数据集.md)：您可以使用 `datasets` 库来加载和处理数据集。
- 评估：使用 `evaluate` 库来评估模型性能。
- [Pipeline](Pipeline.md)：使用 `pipeline` 快速实现常见 NLP 任务。

以下用最精炼、通俗的方式整理一份：

# 🤗 Hugging Face 快速入门指南（基于 AutoDL 环境）

**目标**：快速熟悉 Hugging Face，从零开始完成一次模型下载、简单推理到部署的完整过程。

---

# 🚩 步骤1：注册与登录 Hugging Face

1. 打开官网：[https://huggingface.co/](https://huggingface.co/)
2. 点击右上角的 **Sign Up** 或 **Log In** 完成注册或登录。
    - 也可以使用 GitHub 或 Google 账号快速登录。

---

# 🚩 步骤2：认识 Hugging Face 平台

Hugging Face 核心功能包括：

- **Models**：海量 AI 模型，可直接下载使用。
- **Datasets**：多样数据集，便于模型训练。
- **Spaces**：在线快速部署模型。

---

# 🚩 步骤3：在 AutoDL 环境中使用 Hugging Face 模型

**1. 创建并登录 AutoDL**

- 前往 [AutoDL官网](https://www.autodl.com/)，注册并登录账号。
- 创建一个 Notebook 或打开已有的项目空间。

**2. 安装 Transformers 库**

```python
pip install transformers
```

**3. 使用 Hugging Face 模型进行快速推理**

```python
from transformers import pipeline

classifier = pipeline("sentiment-analysis")
result = classifier("I love Hugging Face!")
print(result)
```

运行上述代码后，你将获得类似以下输出：

```python
[{'label': 'POSITIVE', 'score': 0.9998}]
```

至此，你成功使用 AutoDL 完成了模型加载和预测。

---

# 🚩 步骤4：部署 Hugging Face Spaces（无需代码！）

Spaces 允许你快速部署在线模型应用：

1. 打开 [Spaces主页](https://huggingface.co/spaces)，点击右上角 **"New Space"** 创建。
2. 输入 Space 名称，选择 Gradio 框架，点击创建。
3. 在 Space 编辑器内，复制以下代码：

```python
import gradio as gr
from transformers import pipeline

classifier = pipeline("sentiment-analysis")

def analyze_sentiment(text):
    return classifier(text)

demo = gr.Interface(fn=analyze_sentiment, inputs="text", outputs="json")
demo.launch()
```

点击 **Commit** 按钮即可运行并部署在线应用。

---

# 🚩 步骤5：进阶体验——模型微调（可选）

你可以使用 Hugging Face 的 [AutoTrain](https://huggingface.co/autotrain) 功能，实现零代码或少代码快速微调模型，也可以通过 AutoDL Notebook 按照[官方教程](https://huggingface.co/docs/transformers/training)进一步深入。

---

🎉 **欢迎进入 Hugging Face 与 AutoDL 的 AI 世界，祝你探索愉快！**
