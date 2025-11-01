---
uid: a4e76b7b-0235-4fbe-8de2-e7a22b4ff6f3
date created: 2025-07-16
date modified: 2025-11-01
---
# Kaggle *中类似券相关的比赛

基于您的查询，我通过网络搜索工具查询了“Kaggle competitions related to coupon prediction or recommendation”，并获取了相关结果。Kaggle 上确实有一些与优惠券（coupon）相关的比赛或数据集，主要涉及券的购买预测、兑换预测、推荐系统等。这些比赛通常与电商、营销或个性化推荐场景相关，数据往往包括用户行为、上下文特征（如时间、地点、天气）等。以下是筛选出的主要相关比赛和数据集（按相关性排序），我附上了链接和简要描述。如果需要更详细的数据或代码示例，我可以进一步使用工具浏览具体页面或执行模拟代码。

## 1. **Coupon Purchase Prediction**
   - **链接**：https://www.kaggle.com/c/coupon-purchase-prediction
   - **描述**：这是一个经典的Kaggle比赛，由Recruit Communications主办。目标是基于用户的历史浏览和购买数据，为每个用户推荐一个排序的券列表（ranked list of coupons）。数据包括用户列表、券详情等，适合推荐系统算法（如CatBoost、XGBoost）。比赛已结束，但数据集公开可用。
   - **相关性**：直接涉及券购买预测，与旅行券推荐类似（可扩展到旅行场景，如酒店/航班券）。
   - **额外**：有讨论论坛（如https://www.kaggle.com/competitions/coupon-purchase-prediction/discussion/15768）和规则页面。

## 2. **In-Vehicle Coupon Recommendation**
   - **链接**：https://www.kaggle.com/datasets/mathurinache/invehicle-coupon-recommendation (数据集) 和 https://www.kaggle.com/competitions/mbd2022-in-vehicle-coupon-recommendation (比赛版本)
   - **描述**：预测司机在特定驾驶场景（如目的地、时间、天气、乘客等）下是否接受推荐的券。数据来自UCI Machine Learning Repository，目标是二分类（接受/拒绝）或概率预测。有一个2022年的比赛版本（MBD2022），要求预测接受概率。
   - **相关性**：强调上下文推荐（如天气影响），类似于旅行券系统中的实时推荐（e.g., 雨天推送室内景点券）。有代码示例可用（如https://www.kaggle.com/code/maherabdelllatif/invehicle-coupon-recommendation）。
   - **额外**：适合CatBoost等模型处理分类特征（如天气类别）。

## 3. **Predicting Coupon Redemption**
   - **链接**：https://www.kaggle.com/datasets/vasudeva009/predicting-coupon-redemption
   - **描述**：预测客户是否会兑换通过不同渠道收到的券。数据包括客户特征、营销渠道等，目标是帮助零售商优化营销策略，提高兑换率。
   - **相关性**：焦点在券兑换预测，与您提到的券系统中的兑换管理和欺诈检测高度相关。可用于旅行场景，如预测用户是否兑换酒店券。

## 4. **Coupon Purchase Prediction for Ponpare**
   - **链接**：https://rstudio-pubs-static.s3.amazonaws.com/136006_19010ce2f5144f4c89556a3eee159c57.html (外部分析，但基于Kaggle数据)
   - **描述**：使用Ponpare（日本券平台）的一年用户数据（浏览、购买历史）预测用户未来购买哪些券。类似于第一个比赛，但更注重时间序列预测。
   - **相关性**：适用于动态券系统，如基于用户旅行历史的预测。

## 其他相关资源
- **Acquire Valued Shoppers Challenge**（虽未在搜索中直接列出，但类似）：https://www.kaggle.com/c/acquire-valued-shoppers-challenge–预测用户是否使用特定券，数据规模大，涉及电商场景。
- 搜索结果显示，这些比赛多在2014-2022年间举办，许多数据集仍活跃，用于练习推荐算法（如协作过滤、梯度提升模型）。如果您想在CatBoost上下文中使用，我建议从“Coupon Purchase Prediction”入手，因为它有丰富的分类特征。

如果这些不是您想要的，或者需要更具体的代码模拟（e.g., 使用CatBoost在这些数据集上训练模型），请提供更多细节！我可以调用工具浏览比赛页面或执行Python代码（注意工具环境有torch等ML库，但无CatBoost；我可以用类似如XGBoost模拟）。
