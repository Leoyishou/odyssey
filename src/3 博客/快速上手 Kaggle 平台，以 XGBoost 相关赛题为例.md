---
draw:
title: 快速上手 Kaggle 平台，以 XGBoost 相关赛题为例
date created: 2025-03-02
date modified: 2025-03-02
---
**🎯 Kaggle 是一个数据科学的在线平台，每个赛题就是一个数据分析的场景，而且上面的代码都是公开的，数据也都存储在云上，只要点几下，就能直接在云端环境运行别人的代码，快速实现项目复现和模型训练。是一个很好的算法学习和实战的平台，可以用来学习和练手：**

  **传统机器模型**：包括决策树、随机森林、XGBoost、LightGBM、CatBoost、逻辑回归、支持向量机等。

  **深度学习模型**：CNN、RNN、Transformer、LSTM、BERT、图神经网络（GNN）等。

  **数据分析与特征工程**：特征工程、数据可视化、特征选择、PCA降维、缺失值插补、特征编码与构造。

  **统计分析和可视化工具**：Pandas、Numpy、Matplotlib、Seaborn、Plotly 等，提供直观的数据探索和可视化分析能力。

---

下面以一个简单的题目为例，介绍一下这个平台的使用方式

1. ## 找题目
    

在 kaggle 的比赛模块中找到这个比赛 https://www.kaggle.com/competitions/amex-default-prediction

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=MjNkODVkZmU5ZmNkODQyZjIzY2JiNmM5YjkxZmZlYzJfdjAwY2VhWE9iZ0tFdTZ5cTc3QWRZRk9vSnhlSVYwY1lfVG9rZW46TzdMZ2I2VG1yb2xpU2Z4Tk8zWmNJTjZTbkVoXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)

1. ## 找答案
    

点击 code 后进入一个『作业区』，里面是所有选手针对这个题目给出的作业，排名靠前的就是优秀作业

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=N2MyZDRkZGFiYzg3YTEzM2YxMTg5NzJiNzY1MGJlNjVfUXFiUW9lemhYNW1kNmlSZ3pIUmluaERtVnBVQnByd0RfVG9rZW46UThiNGJyUUJsbzRrY3F4NFJ4YWNwcG5PbjFnXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)

1. ## 抄作业
    

选择一份优秀作业后，点进来，点击右上角的 Copy&Edit，就相当于 fork 这份作业到自己的空间了

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=ZmJiYjY3MWRlZWQ4NzU4ZWE4NDUwNTQ1NDFhNjlkMThfUjRBMGt0UkdoZWxEa2FZZFVJUFlQQkI1a3N4bXhMNDdfVG9rZW46UTBMQ2J3SlJZb3dzSFZ4UXdrRWMxaDVpbkFjXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)

## 4. 运行作业

跑作业就是类似 Jupyter Notebook 的逻辑了，点击 run all 就可以运行代码（在 kaggle 的云环境中）

input 区域里放这次比赛的训练数据和测试数据，有很多云文件，可以直接搜索添加

output 区域就是放最后模型跑完之后的结果文件，然后系统会根据结果自动打分（准确率）

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=MTRhZDA4N2Y1NDQxODhhNTljYzBjMDM5ZDg4NmE0YzJfNUdFTXJXVkhhelVPak5ISDhxRWNUTHVTcDQ3N3duNERfVG9rZW46UU9WWGJiZkhZb0t4dWh4OTRLaGNJZGhGbmJnXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)

  

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=MTZlYjYyMGZmMjY4NzhjZjc3N2Q4YmI2NmQzZTQxODNfcExxYjJycXlPZUxwRGI2TVZlY2o3eFQzMXd1Q0hMYmtfVG9rZW46TVk2NmJ6cWpLb25OSHV4bjFremNrQm1nbkdmXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)

Setting 中可以选择是否联网，是否使用 GPU 加速

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=OTQyNGJlMzVhMDZlZTY3MmQ5OTdmZDg0ZjAzMTM3YmRfMXEyQllmbFlCMDZWY0ZPMUhZUEtpZTVkTTE3S0t2YkdfVG9rZW46Tm50TWJBZWlCb2czeGF4ZnFyUWNCRnFQblhlXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)

1. ## 交作业
    

点击Notebook右上角的按钮：

- 如果Notebook已经运行结束，右上角的按钮会变成 **Save Version**。
    
- 点击后，选择 "**Save & Run All (Commit)**" 或 "**Quick Save**"（推荐选前者确保所有代码已完全正确运行）。
    

等待几分钟后，Kaggle服务器会自动保存并记录你生成的结果。

- 一般代码最后都会在 Output 区域生成一个名为 `submission.csv` 的文件
    
- 在 Output 区域点击 **Submit to Competition** 按钮，把你的预测结果文件提交到比赛排行榜。
    

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=MTM4MDQ2OTQ3NWZiMWU0ZGU0ZTZiMjg4ZDBlOTA0NzNfOWF3Q3BZVklrclBSZVd6eTQzWVhWUDJlTUx4Y2Rlc0ZfVG9rZW46QnZKMWJXdzFob0NLN2t4ZmFta2NiWmVubkdiXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)

## 6. 看排名

可以进入比赛主页的 **Leaderboard（排行榜）** 页面查看你提交的模型分数。

  

  

  

  

  

  

## 📌 Tips

通过下图中这个浏览器插件 `Immersive Translate`，可以实现页面内 HTML 原生的英文翻译中文。

![](https://hf7l9aiqzx.feishu.cn/space/api/box/stream/download/asynccode/?code=OGIwYmQyNzM5ZDFlMTFmMTE2MTZmNWIwYzgyYmI0YTJfZDlmWXg4eTdncDF6OThmaUx2YnFVU1VwMUt4cEFqNkJfVG9rZW46REJZZWI0azlJbzhFZUh4a0VVU2NPMDZTbjRiXzE3NDA4NTE1Mjg6MTc0MDg1NTEyOF9WNA)
