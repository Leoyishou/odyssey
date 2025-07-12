---
date created: 2025-03-03
date modified: 2025-07-10
uid: 2f4f9ec3-f120-4a08-8842-d59bdf3eb521
---
> 思路一：纯生成路线  
> 这老师用 DeepSeek r1 14b 当 base，open r1 grpo弄半天。结果正确率还下降了，哈哈哈  
> 2025-03-03 00:19:34

> 思路二：2base  
> 先用模型生成 python 代码，然后用 py 跑这个问题  
> https://www.kaggle.com/competitions/ai-mathematical-olympiad-prize/discussion/519303

比赛  
https://www.kaggle.com/competitions/ai-mathematical-olympiad-progress-prize-2

做 110 道数学题，评价的唯一标准是准确度

关注这个大佬的 code  
https://www.kaggle.com/code/danielphalen/grpotrainer-deepseekr1

[[GRPOTrainer]]

{'id': 8,

'problem': 'Given a set of data x1,x2,x3,x4,x5x1​,x2​,x3​,x4​,x5​ with a mean of 8 and variance of 2, find the mean and variance of a new set of data: 4x1+1,4x2+1,4x3+1,4x4+1,4x5+14x1​+1,4x2​+1,4x3​+1,4x4​+1,4x5​+1.',

'solution': "Firstly, let's focus on the mean of the new set. Since the original set has a mean of 8, when we apply the transformation yi=4xi+1yi​=4xi​+1 to each element, the mean will be scaled by a factor of 4 and then increased by 1. Therefore, the mean of the new set is given by:\n\ntextNewmean=4⋅Mean of original set+1=4⋅8+1=32+1=33.\n\ntextNewmean=4⋅Mean of original set+1=4⋅8+1=32+1=33​.\n\n\nNext, we consider the change in variance due to this transformation. Adding 1, a constant, does not affect the variance. However, multiplying by 4 scales the variance by the square of that factor. Therefore, the variance of the new set is given by:\n\ntextNewvariance=42⋅Variance of original set=16⋅2=32.\n\ntextNewvariance=42⋅Variance of original set=16⋅2=32​.\n\n\nHence, the correct answer for the mean and variance of the new set is 33 and 32, respectively.",

'answer': '32',

'**index_level_0**': 8}
