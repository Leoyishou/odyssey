---
draw:
title: pandas$
date created: 2025-05-02
date modified: 2025-05-02
---

下面把 **“理论介绍 / 学习目标”** 全部补回，同时保留 **`None` 占位的练习代码**，方便您边读概念边动手。每一步依旧遵循：

> 理论介绍 → 编程任务（`None` 占位）→ 验证代码

---

## Pandas 基础入门教程 (Exercism 风格 · 占位版)

|步骤|主题|目标|
|---|---|---|
|1|DataFrame / Series 创建与属性|能构造表格并查看 shape、columns、dtypes 等元信息|
|2|CSV 读写|把 DataFrame 保存到硬盘并读回|
|3|数据选择与筛选|熟练使用 `loc` / `iloc` / 布尔索引|
|4|常见列操作|增删列、重命名、排序、唯一值、`value_counts`|
|5|缺失值处理|发现、填补、删除 NaN|
|6|分组聚合 `groupby`|按类别 Split-Apply-Combine|
|7|拼接与合并|`concat` 行扩展 & `merge` 键合并|
|8|apply / map|对列或行执行自定义函数 / 映射|
|9|类型转换|字符串→数值 / 字符串→日期|
|10|时间序列 & 重采样|DatetimeIndex 和按月汇总|

---

### 1 · DataFrame / Series 创建与基本属性

**理论介绍**

- **Series**：一维带索引的数据结构，可看作“带标签的数组”。
    
- **DataFrame**：二维表格，行索引 + 列标签。
    
- 核心属性：`.shape` 行列尺寸、`.index` 行索引、`.columns` 列名、`.dtypes` 每列类型。
    
- 快速预览：`head()` / `tail()`。
    

```python
import pandas as pd

# TODO: 创建整数 Series
s = None

# TODO: 创建 3×2 DataFrame（列 A,B）
df = None

print("Series:\n", s)
print("DataFrame:\n", df)
print("shape:", df.shape,
      "| columns:", df.columns,
      "| index:", df.index,
      "| dtypes:", df.dtypes)
```

```python
assert isinstance(s, pd.Series)
assert isinstance(df, pd.DataFrame)
assert df.shape == (3, 2)
assert list(df.columns) == ['A', 'B']
assert s.iloc[0] == 10 and s.iloc[2] == 30
print("第1步验证通过！")
```

---

### 2 · CSV 数据读写

**理论介绍**

- `pd.read_csv`：把 CSV 文件加载为 DataFrame。
    
- `DataFrame.to_csv`：将数据写入 CSV。参数 `index=False` 可避免保存行索引。
    
- 常用参数还有 `sep`（分隔符）、`encoding`、`usecols` 等。
    

```python
import pandas as pd

df = pd.DataFrame({
    'Name': ['Alice', 'Bob', 'Charlie'],
    'Age':  [25, 30, 35]
})

# TODO: 保存 df 至 CSV（省略行索引）
None

# TODO: 再读取回 df2
df2 = None

print(df2)
```

```python
assert df2.equals(df)
print("第2步验证通过！")
```

---

### 3 · 数据选择与筛选

**理论介绍**

- `loc[行标签, 列标签]`：按名字取值；切片“包含右端”。
    
- `iloc[行位置, 列位置]`：按整数位置；切片“不含右端”。
    
- 布尔索引 `df[条件]`：用 True/False 数组过滤行。
    

```python
import pandas as pd
df = pd.DataFrame({
    'A':[10,15,20,25,30],
    'B':[ 5, 6, 7, 8, 9],
    'C':['X','Y','X','Y','X']
}, index=['a','b','c','d','e'])

# TODO: 取索引 'c' 行
row_c  = None

# TODO: 取位置 2 行
row_c2 = None

# TODO: 行 b~d & 列 A,C
sub_df = None

# TODO: A 列 >20 的行
filtered_df = None
```

```python
assert row_c.equals(row_c2)
assert list(sub_df.index)==['b','c','d'] and list(sub_df.columns)==['A','C']
assert list(filtered_df.index)==['d','e']
print("第3步验证通过！")
```

---

### 4 · 常见列操作

**理论介绍**

- 新增列：`df['new']=...`
    
- 删除列：`df.drop(columns=[...])`
    
- 重命名：`df.rename(columns={'旧':'新'})`
    
- 排序：`df.sort_values(by='列', ascending=False)`
    
- 唯一值：`series.unique()`；计数：`series.value_counts()`。
    

```python
import pandas as pd
df = pd.DataFrame({
    'Name':['Alice','Bob','Charlie','Bob'],
    'Score':[85,90,85,75]
})

# TODO: 新增 Score2 = Score+5
df['Score2'] = None

# TODO: Name → Student
df = None

# TODO: 删除 Score2
df = None

# TODO: 按 Score 降序
sorted_df = None

# TODO: 唯一值 + 计数
unique_students = None
student_counts  = None
```

```python
assert (sorted_df['Score'].diff().dropna()<=0).all()
assert student_counts['Bob']==2
print("第4步验证通过！")
```

---

### 5 · 缺失值处理

**理论介绍**

- `isnull()` 标记 NaN。
    
- `fillna()` 填补空值，可配合常数 / 前向填充。
    
- `dropna()` 删除含缺失的行/列；`how='any'|'all'` 控制条件。
    

```python
import pandas as pd
df = pd.DataFrame({
    'A':[1,None,3,None,5],
    'B':[10,20,None,None,50]
})

# TODO: 统计列缺失数
missing_counts = None

# TODO: NaN → 0
df_filled = None

# TODO: 删掉含任意 NaN 的行
dropped_any = None

# TODO: 删掉整行全 NaN
dropped_all = None
```

```python
assert missing_counts['A']==2 and missing_counts['B']==2
assert df_filled.isnull().sum().sum()==0
assert list(dropped_any.index)==[0,4]
assert 3 not in dropped_all.index
print("第5步验证通过！")
```

---

### 6 · 分组聚合 (`groupby`)

**理论介绍**

- `groupby('键')['数值列'].agg()` 可一次做求和、均值、计数。
    
- 聚合后索引为分组键，返回 Series 或 DataFrame。
    

```python
import pandas as pd
df = pd.DataFrame({
    'Category':['A','B','A','B','B'],
    'Value':[10,20,30,40,60]
})

# TODO: 组内求和
sum_by_cat = None

# TODO: 组内均值
mean_by_cat = None

# TODO: 组内计数
count_by_cat = None
```

```python
assert sum_by_cat['A']==40 and sum_by_cat['B']==120
assert mean_by_cat['B']==40
assert count_by_cat['A']==2
print("第6步验证通过！")
```

---

### 7 · 拼接与合并

**理论介绍**

- `pd.concat([...], axis=0/1)`：简单堆叠。
    
- `pd.merge(left, right, on='键', how='inner|outer|left|right')`：SQL-like join。
    

```python
import pandas as pd
df1 = pd.DataFrame({'City':['NY','LA'],'Pop':[8.4,4.0]})
df2 = pd.DataFrame({'City':['CHI','HOU'],'Pop':[2.7,2.3]})

# TODO: 行拼接
df_concat = None

df3 = pd.DataFrame({'City':['NY','LA','CHI'],'State':['NY','CA','IL']})
df4 = pd.DataFrame({'City':['CHI','NY','SEA'],'Mayor':['LL','EA','BH']})

# TODO: City 内连接
df_merged = None

# TODO: City 外连接
df_merged_outer = None
```

```python
assert df_concat.shape==(4,2)
assert set(df_merged['City'])=={'NY','CHI'}
assert df_merged_outer.shape[0]==4
print("第7步验证通过！")
```

---

### 8 · apply / map

**理论介绍**

- `Series.apply(func)`：对每个元素执行函数。
    
- `Series.map(dict_or_func)`：映射替换。
    
- `DataFrame.apply(func, axis=1)`：逐行/列自定义计算。
    

```python
import pandas as pd
df = pd.DataFrame({
    'Student':['Alice','Bob','Charlie','David'],
    'Score':[85,92,77,60]
})

# TODO: 生成 Passed 列
df['Passed'] = None

# TODO: 姓名映射为 ID
id_map = {'Alice':1001,'Bob':1002,'Charlie':1003,'David':1004}
df['ID'] = None
```

```python
assert list(df['Passed'])==['Yes' if x>=80 else 'No' for x in df['Score']]
assert list(df['ID'])==[id_map[n] for n in df['Student']]
print("第8步验证通过！")
```

---

### 9 · 类型转换

**理论介绍**

- `astype()` 改数值/类别等 dtype。
    
- `pd.to_datetime()` 把字符串解析为日期。
    

```python
import pandas as pd
df = pd.DataFrame({
    'date_str':['2020-01-01','2021-06-15','2020-12-31'],
    'temp_str':['30.5','35.0','28.0']
})

# TODO: 字符串 → datetime
df['date'] = None

# TODO: 字符串 → float
df['temp'] = None
```

```python
assert df['date'].dtype=='datetime64[ns]'
assert df['temp'].dtype=='float64'
assert df.loc[1,'date'].year==2021
print("第9步验证通过！")
```

---

### 10 · 时间序列与重采样

**理论介绍**

- `pd.date_range` 生成日期索引。
    
- `resample('M').sum()`：把日数据聚合到月度。
    

```python
import pandas as pd
dates = pd.date_range('2021-01-01','2021-02-28',freq='D')
ts_df = pd.DataFrame({'Value':range(1,len(dates)+1)}, index=dates)

# TODO: 月度汇总
monthly_sum = None
```

```python
assert monthly_sum.shape[0]==2
assert monthly_sum.loc['2021-01-31','Value']==496
assert monthly_sum.loc['2021-02-28','Value']==1274
print("第10步验证通过！")
```

---

阅读理论 → 填写 `None` → 运行验证，

全部 “验证通过” 即完成 10 小时 Pandas 基础练习。祝学习愉快!
