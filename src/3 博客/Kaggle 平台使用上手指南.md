---
comment_id: '25209407'
date created: 2025-03-23
date modified: 2025-03-23
draw: null
title: Kaggle 平台使用上手指南
---
## Kaggle房价预测比赛入门指南

Kaggle 上的 "House Prices: Advanced Regression Techniques"（房价预测）比赛是初学者练习完整机器学习流程的一个很好的起点[1]。通过这个比赛，您可以学习从数据探索、模型训练到生成提交结果的整个过程。本指南将一步步带您从零开始参与该比赛，最终成功提交预测结果并查看分数（排名高低并不重要，重在参与和学习）。

### 步骤1：找到并加入比赛页面

1. **登录 Kaggle**：使用您的 Kaggle 帐号登录官网。登录后，在页面顶部导航栏找到 **"Competitions"**（竞赛）栏目。
2. **查找比赛**：在 Competitions 页面搜索 **"House Prices: Advanced Regression Techniques"**。您也可以直接访问比赛链接[2]。找到该比赛后点击进入其主页。
3. **加入比赛**：在比赛主页点击 **"Join Competition"** 按钮（加入比赛）。首次加入时会弹出规则对话框，点击接受（Accept）比赛规则[3]。完成加入后，您才能下载数据并提交结果。加入比赛后，比赛主页的提交按钮才会激活，允许您上传预测结果[4]。

**提示**：比赛主页包含多个标签页：**Overview**（概览）介绍比赛目标和评价指标，**Data** 列出提供的数据集，**Code** 展示公开的笔记本代码，**Discussion** 是讨论区等。您可以浏览 Overview 了解比赛要求，例如需要为每个测试集房屋预测最终售价[5]。比赛的评价指标是预测值与实际值对数转换后的均方根误差（RMSE）[6]（对数处理使高价和低价房屋的预测误差影响相等）。

### 步骤2：Fork（复制）并运行入门Notebook

加入比赛后，下一步是在Kaggle上创建或复制一个Notebook（笔记本）来探索数据和训练模型。对于新手，最简单的方法是**fork一个已有的入门Notebook**，然后运行其中的代码：

1. **打开公开笔记本列表**：在比赛主页选择 **"Code"** 标签页。这里汇集了社区用户分享的该比赛代码（以前称为Kernel）。您可以按"Most Votes"（最高投票）或"Recently Updated"等排序，查找一个简单的入门笔记本。例如，很多初学者会选择标题带有 "Starter" 或 "Beginner" 的笔记本。
2. **Fork 笔记本**：点击选中的笔记本进入其页面，然后点击右上角的 **"Copy and Edit"** 按钮。这会创建一份该笔记本的副本到您的账号下（这就是 fork 策略），您可以在其中自由运行和修改代码，而不会影响原作者的版本[7]。
3. **检查数据集挂载**：复制的Notebook通常已经关联了比赛数据。如果需要，您可以在右侧的 **"Data"** 面板确认 `train.csv`, `test.csv` 等数据集是否已经添加。如果没有，点击 **"Add Data"**，选择 Competitions -> House Prices... 数据集进行添加。通常从比赛页面创建或fork的Notebook会自动包含比赛数据。
4. **运行 Notebook**：现在，您可以点击顶部菜单的 **"Run"** 按钮逐个执行代码单元，或者选择 **"Run All"** 一键运行全部代码。如果一切顺利，Notebook 将加载数据、进行一些分析并训练模型。对于初学者的入门Notebook，可能已经包含了基本的数据读取和模型代码，您只需运行即可得到结果。

**提示**：在 Kaggle Notebook 中运行代码和在本地 Jupyter 环境类似。您可以插入新的代码单元、修改现有代码然后重新运行。若要保存代码运行的结果，需要**保存版本（Commit）**。点击界面右上角的 **"Save Version"** 按钮，在弹出的对话框中选择 "Save & Run All (Commit)" 并确认保存[8]。这样会从头执行整个Notebook并保存输出结果（包括生成的文件）。稍后我们将利用这一功能来生成提交文件。

### 步骤3：理解并探索数据（EDA）

在运行入门Notebook或自行编写代码时，首先需要熟悉数据。比赛提供了四个主要文件[9]：

- **train.csv**：训练集，包含房屋的特征和已知售价（目标值）。共有1460条样本，80个特征列和1个目标列。
- **test.csv**：测试集，包含房屋特征，但不包含售价。需要您为每条测试数据预测售价，共有1459条样本。
- **data_description.txt**：数据说明，解释每个特征的含义（例如字段含义、单位等）。建议阅读以理解领域背景。
- **sample_submission.csv**：提交示例文件，格式为两列：Id和SalePrice。您最终提交的结果文件需要与其格式一致。

在Notebook中，您可以使用 Pandas 来读取数据并进行初步探索：

```python
import pandas as pd
train_df = pd.read_csv('/kaggle/input/house-prices-advanced-regression-techniques/train.csv')
test_df = pd.read_csv('/kaggle/input/house-prices-advanced-regression-techniques/test.csv')
print(train_df.shape)  # 查看维度
train_df.head()        # 查看前几行数据
```

初步观察训练集，您会看到每行是一套房屋的数据，各列是不同特征，例如 OverallQual（整体材料和做工质量评分）、GrLivArea（地上居住面积平方英尺）等，以及目标列 SalePrice（售价）。您可以计算一些基本统计:

- **总体规模**：训练集有多少行、多少列；测试集行数是否一致（训练1460行，测试1459行）。
- **目标分布**：查看 `train_df['SalePrice'].describe()` 获得售价的均值、中位数、最大值等。绘制售价的直方图可以发现其分布**偏右偏态**，即高价房屋数量相对较少[10]（这在房价数据中很常见）。如果需要，可以对SalePrice取对数以使分布更接近正态，但这是进阶技巧，可暂时不做。
- **缺失值**：检查各列的缺失值数量，例如 `train_df.isnull().sum()`。您会发现有些特征缺失值较多，例如 **Alley**（小巷通道）在多数房屋中为空值，表示这些房子没有巷道[11][12]。对于缺失值，简单起见我们可以先用简单策略处理（如用平均值或中位数填充，或在模型中忽略这些列）。
- **特征关系**：通过计算相关系数或绘图，了解哪些特征与房价相关度高。例如，总体施工质量评分（OverallQual）和居住面积（GrLivArea）与房价的相关性很强[13]。您可以试着绘制 `GrLivArea` 与 `SalePrice` 的散点图，通常会看到面积越大的房子售价总体越高，尽管有一些异常值（如极大的居住面积但价格不成比例，可能是异常点）。简单EDA能够帮助您直观理解数据，对后续建模有所指导。

**小结**：在这一阶段，不需要对数据做非常复杂的处理。目标是**熟悉数据**：知道有哪一些特征、目标是什么分布，大致了解可能的重要特征和潜在的数据质量问题（如缺失值、异常值）。这些认知将帮助我们在下一步选择建模策略。

### 步骤4：训练基础模型（线性回归 / XGBoost）

有了对数据的初步了解，我们就可以训练一个基础模型来预测房价。初学者可以从简单的模型开始，例如线性回归模型，或者直接使用效果较好的梯度提升树模型（如 XGBoost）。这里我们介绍两种方法：

**方法A：线性回归模型**–这种模型简单易懂。我们需要先准备好特征矩阵 X 和目标向量 y：

1. **特征选择与预处理**：为了简化流程，我们可以选取几个与房价关系较强的特征作为示范。例如使用 **GrLivArea**（居住面积）和 **OverallQual**（整体质量）这两个数值型特征来预测。由于这两个特征在训练和测试中可能都没有缺失，可以免去缺失值处理和类别编码的复杂步骤。我们构造训练集和测试集的特征 DataFrame，并确保对齐：

    ```python
    features = ['GrLivArea', 'OverallQual']
    X_train = train_df[features]
    y_train = train_df['SalePrice']
    X_test = test_df[features]
    ```

    如果您选用更多特征，要注意处理数据中的缺失值。例如可以用 `fillna()` 用中位数填充数值空值；对于类别型特征，需要先转换为数值（如One-Hot编码）才能用于大多数模型[14]。入门阶段可以暂时舍弃复杂预处理，只用几项数值特征来快速建立模型。

    
2. **训练模型**：使用 scikit-learn 的线性回归模型进行训练:

    ```python
    from sklearn.linear_model import LinearRegression
    model = LinearRegression()
    model.fit(X_train, y_train)
    predictions = model.predict(X_test)
    ```

    这几行代码就完成了模型的训练和对测试集的预测。因为我们只用了两个主要特征，模型会根据这两个因素线性地估计房价。线性回归模型训练速度很快，但可能欠拟合复杂关系，不过作为baseline可以获得一个大致的结果。

    

**方法B：XGBoost模型**–XGBoost是一种基于决策树提升的集成算法，在Kaggle比赛中非常流行。它对特征的预处理要求相对宽松（对缺失值鲁棒，对类别需要编码，但可以先以整数标签代替）。您可以尝试用所有主要特征训练一个 XGBoost 回归模型：

```python
!pip install xgboost  # 如果环境中尚未安装XGBoost（Kaggle默认环境通常已预装）
from xgboost import XGBRegressor

model = XGBRegressor(n_estimators=100, learning_rate=0.1)
model.fit(X_train, y_train)  # X_train可以包括更多特征，但需先处理缺失/编码
predictions = model.predict(X_test)
```

XGBoost 通常能比线性回归取得更低的误差，但训练过程会稍慢一些。对于初始尝试，可以先用默认参数运行少量树模型获得结果。

**提示**：无论使用哪种模型，**不要直接用训练集的SalePrice均值或其他未来信息泄露到测试**。确保特征处理对训练和测试是一致的[15]。另外，初学阶段模型无需调参，使用默认设置即可得到一个初步结果。

经过以上步骤，您将获得一个 `predictions` 数组，其中包含对每个测试集房屋的预测售价。下一步就是将这些预测结果保存成符合要求的提交文件。

### 步骤5：生成提交文件 (submission.csv)

按照比赛要求，提交的文件应包含两列：`Id` 和 `SalePrice`，对应测试集每个Id的预测房价。Kaggle提供的 `sample_submission.csv` 就是一个模板。我们可以利用 pandas 将预测结果保存：

```python
output = pd.DataFrame({'Id': test_df['Id'], 'SalePrice': predictions})
output.to_csv('submission.csv', index=False)
```

运行上述代码后，会在当前工作目录生成一个名为 `submission.csv` 的文件。这就是我们准备提交的结果文件，其中每行对应 `test.csv` 中相应Id的房价预测。

**确认文件格式**：您可以用 `output.head()` 查看前几行，确保格式正确。例如：

```Java
     Id  SalePrice  
0  1461  169000.0  
1  1462  187000.0  
...
```

应当有 1459 行（不包括表头），Id从1461到2919（这是测试集中的房屋ID范围），SalePrice是您的预测值。如果行数或列名不符，提交可能会失败。比赛要求提交CSV格式，可以压缩成zip上传。通常直接.csv文件即可。

在 Kaggle Notebook 中生成文件后，需要**保存Notebook版本**以导出文件。在Notebook编辑界面，点击右上角 **"Save Version"** 按钮，并选择 "Save & Run All (Commit)" 来运行所有单元并保存结果[8]。完成后，跳转到该Notebook的版本页面，在 **"Output"** 选项卡下就能看到刚才生成的 `submission.csv` 文件。您可以点击文件旁的下载按钮将其下载到本地，或直接从Kaggle界面提交（见下一步）。

### 步骤6：提交结果到 Kaggle 并查看得分

**提交预测结果并查看分数**[16]：按照以下步骤，将刚才生成的 `submission.csv` 提交到Kaggle评测系统：

1. **进入提交页面**：打开比赛主页，点击右侧的 **"Submit Predictions"** 按钮（如果比赛已结束则显示为 "Late Submission"）。会进入提交结果的页面。
2. **上传文件**：在提交页面的 **Step 1: Upload submission file** 部分，点击中间的虚线框或 **"Upload Submission File"** 按钮，选择您要提交的预测文件（即 `submission.csv`）。文件上传后，页面会提示文件名已选择。
3. **填写描述（可选）**：在 **Step 2: Describe submission** 中，您可以给此次提交添加一个描述备注（例如 "First try with linear regression"），这对您自己管理提交记录有用，但不是必须填。
4. **确认提交**：确认无误后，点击页面下方的 **"Make Submission"** 按钮。之后页面会显示一个提交成功的提示。几秒钟后，Kaggle会计算您的分数，并在页面上显示结果。根据比赛设置，您会看到本次提交在**公共榜单（Public Leaderboard）**上的评分。

提交完成后，您可以在比赛的 **"My Submissions"** 或 **"Leaderboard"** 标签页查看自己的成绩和排名。对于房价预测比赛，评估采用对数RMSE误差，分数越低表示误差越小。初次提交的模型可能得分相对较高（误差大），比如可能在0.15左右甚至更高，这很正常。我们的目标只是确保成功走通提交流程。**排名不重要**，哪怕分数一般，能出现在Leaderboard上就证明您完成了一次有效提交。

根据Kaggle提供的信息，每个测试集文件应有 1459 行预测结果且有表头，否则会提示错误。在提交时Kaggle也会进行这些基本检查。提交后若分数显示为 **"0"** 或 **错误**, 请检查文件是否正确生成或格式有误。

### 步骤7：改进模型与进一步尝试（可选）

恭喜您完成第一次提交！接下来，您可以尝试改进模型，提升预测准确率。在这个过程中，您会学到更多数据科学技巧：

- **特征工程**：考虑处理我们在EDA中发现的问题，例如对偏态的SalePrice取对数来建模，对类别特征进行合理编码，填补缺失值等。创造新的特征或组合特征也可能提高模型效果（例如总面积 = 地上居住面积 + 地下室面积等）。这些需要一定的创造力和业务理解，是提升模型的关键领域[17]。
- **模型调整**：尝试更加复杂的模型或集成方法。例如，可以在线性模型基础上尝试决策树、随机森林、梯度提升机（XGBoost/LightGBM）等，并对模型超参数进行调优。注意每次只改变一个因素并使用交叉验证验证效果，这样可以逐步找到更优组合。
- **参考优秀笔记本**：在比赛的Code页面，寻找高分选手分享的分析。例如有人分享了使用Stacking集成多模型获得Top 4%成绩的经验[18]。阅读他们的方案可以学到很多实用技巧。您也可以直接 fork 这些优秀笔记本运行，看是否复现相同的分数，并对其中的代码加入自己的注释理解。
- **持续迭代**：机器学习建模是一个反复迭代的过程。每提交一个新结果，都留意分数的变化和是否改进。如果某次改动导致分数变差，也不必气馁，这是分析模型的机会。通过不断尝试特征处理和模型调整，通常都能比最初的baseline有明显提升[19]。

最后，享受学习的过程！通过参与Kaggle比赛，您不仅掌握了房价预测这个案例的解决流程[20]，也为日后更复杂的比赛打下基础。即使最初排名不靠前也没关系——每一次提交和改进都是进步。希望这篇指南能够帮助您顺利完成从入门到提交的全过程，祝您在Kaggle学习愉快！

## 参考资料

[1]动手学深度学习 2.0.0 documentation, "4.10. 实战Kaggle比赛：预测房价"

[2]GitHub - massquantity/Kaggle-HousePrices, "Kaggle竞赛——房价预测(House Prices).md"

[3]Kaggle, "House Prices - Advanced Regression Techniques"

[4]Kaggle, "Predict the Housing Price"

[5]GitHub - PierreExeter/kaggle-house-prices, "House Prices: Advanced Regression Techniques"

[6]GitHub - PierreExeter/kaggle-house-prices, "Metric"

[7]Kaggle, "Forks in Kaggle mean"

[8]Kaggle, "Save Version and Run All"

[9]Medium, "Predicting House Prices—My First Machine Learning Project"

[10]Kaggle, "House prices: Lasso, XGBoost, and a detailed EDA"

[11]GitHub - PierreExeter/kaggle-house-prices, "Alley access"

[12]GitHub - Ewenwan/gluon-tutorials-zh, "chapter_deep-learning-basics/kaggle-house-price.md"

[13]CSDN博客, "Kaggle入门赛 - 房价预测"

[14]Dataquest, "Getting Started with Kaggle: House Prices Competition"

[15]Dataquest, "Step 3: Build a linear model"

[16]腾讯云开发者社区, "〖深度学习基础〗多层感知机 | 实战Kaggle比赛：预测房价"

[17]Dataquest, "Before we prepare the data"

[18]博客园, "Kaggle比赛（二）House Prices: Advanced Regression Techniques"

[19]Medium, "Mastering Machine Learning: A Beginner's Attempt at Predicting House Prices"

[20]CSDN博客, "Kaggle入门赛 - 房价预测"
