---
draw:
title: 快速上手 Kaggle 平台，以 XGBoost 相关赛题为例
date created: 2025-03-02
date modified: 2025-04-03
---
**🎯 Kaggle 是一个数据科学的在线平台，每个赛题就是一个数据分析的场景，而且上面的代码都是公开的，数据也都存储在云上，只要点几下，就能直接在云端环境运行别人的代码，快速实现项目复现和模型训练。是一个很好的算法学习和实战的平台，可以用来学习和练手：**

  **传统机器模型**：包括决策树、随机森林、XGBoost、LightGBM、CatBoost、逻辑回归、支持向量机等。

  **深度学习模型**：CNN、RNN、Transformer、LSTM、BERT、图神经网络（GNN）等。

  **数据分析与特征工程**：特征工程、数据可视化、特征选择、PCA降维、缺失值插补、特征编码与构造。

  **统计分析和可视化工具**：Pandas、Numpy、Matplotlib、Seaborn、Plotly 等，提供直观的数据探索和可视化分析能力。

---

下面以一个简单的题目为例，介绍一下这个平台的使用方式

- [1. 找题目](#1.%20%E6%89%BE%E9%A2%98%E7%9B%AE)
- [2.找答案](#2.%E6%89%BE%E7%AD%94%E6%A1%88)
- [3. 抄作业](#3.%20%E6%8A%84%E4%BD%9C%E4%B8%9A)
- [4. 运行作业](#4.%20%E8%BF%90%E8%A1%8C%E4%BD%9C%E4%B8%9A)
- [5. 交作业](#5.%20%E4%BA%A4%E4%BD%9C%E4%B8%9A)
- [6. 看排名](#6.%20%E7%9C%8B%E6%8E%92%E5%90%8D)

## 1. 找题目

    

在 kaggle 的比赛模块中找到这个比赛 https://www.kaggle.com/competitions/amex-default-prediction

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-52-55-fad89ea92e0b44a2089ad3fe539ca3b7-202504031452206-1cc6cf.png)

## 2.找答案

    

点击 code 后进入一个『作业区』，里面是所有选手针对这个题目给出的作业，排名靠前的就是优秀作业

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-53-33-11fd7a1a08f2f35aaac25492772dfd0a-202504031453686-04afa7.png)

## 3. 抄作业

    

选择一份优秀作业后，点进来，点击右上角的 Copy&Edit，就相当于 fork 这份作业到自己的空间了

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-54-20-208f9b3a66bbb0c8c324e75a5612ddd2-202504031454427-fe2632.png)

## 4. 运行作业

跑作业就是类似 Jupyter Notebook 的逻辑了，点击 run all 就可以运行代码（在 kaggle 的云环境中）

input 区域里放这次比赛的训练数据和测试数据，有很多云文件，可以直接搜索添加

output 区域就是放最后模型跑完之后的结果文件，然后系统会根据结果自动打分（准确率）

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-54-42-b6aa3a24c9c9e677d1e3d5877d31798d-202504031454307-0e87ab.png)

  

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-55-05-6ed08815023cc90d4d1e8a5d3b748f81-202504031455034-bba151.png)

Setting 中可以选择是否联网，是否使用 GPU 加速

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-56-40-f996b9762fcc9cce5abacd518b3a611f-202504031456715-1f8904.png)

## 5. 交作业

    

点击Notebook右上角的按钮：

- 如果Notebook已经运行结束，右上角的按钮会变成 **Save Version**。
    
- 点击后，选择 "**Save & Run All (Commit)**" 或 "**Quick Save**"（推荐选前者确保所有代码已完全正确运行）。
    

等待几分钟后，Kaggle服务器会自动保存并记录你生成的结果。

- 一般代码最后都会在 Output 区域生成一个名为 `submission.csv` 的文件
    
- 回到这个比赛的主页，点击 **Submit t** 按钮，把你的预测结果文件提交到比赛排行榜。
    

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-56-58-2349ebfc4eb2ca0c17218638596ba513-202504031456349-32109d.png)

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F04%2F03%2F14-57-07-c40a726ba9c13c38c6132c505cf5d97a-202504031457874-c153e9.png)

## 6. 看排名

可以进入比赛主页的 **Leaderboard（排行榜）** 页面查看你提交的模型分数。

![CleanShot 2025-04-03 at 14.59.56@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_AT8ySUh8F7%2F2025%2F04%2F03%2F15-00-15-c470a7705c3a48e099f22375083c0568-CleanShot%202025-04-03%20at%2014.59.56-2x-37abdf.png)
