---
comment_id: d8e3e556
date created: 2024-09-16
date modified: 2025-02-06
draw: null
title: Viva La Vida
---
## 待办区

- [ ] 逐步梳理清楚文件结构，是的 git 的东西变得清晰
- [ ] **需要在数据库中为** **`active_use_scenario`** **列创建一个** **GIN (Generalized Inverted Index)** **索引。具体来说：**
- [ ] 首页加一篇文章，传递自己的认知

### 载入词典

[](https://github.com/fxsjy/jieba#%E8%BD%BD%E5%85%A5%E8%AF%8D%E5%85%B8)

- 开发者可以指定自己自定义的词典，以便包含 jieba 词库里没有的词。虽然 jieba 有新词识别能力，但是自行添加新词可以保证更高的正确率
- 用法：jieba.load_userdict(file_name) # file_name 为文件类对象或自定义词典的路径
- 词典格式和 `dict.txt` 一样，一个词占一行；每一行分三部分：词语、词频（可省略）、词性（可省略），用空格隔开，顺序不可颠倒。`file_name` 若为路径或二进制方式打开的文件，则文件必须为 UTF-8 编码。
- 词频省略时使用自动计算的能保证分出该词的词频。

**例如：**

```Java
创新办 3 i
云计算 5
凱特琳 nz
台中
```

- 更改分词器（默认为 `jieba.dt`）的 `tmp_dir` 和 `cache_file` 属性，可分别指定缓存文件所在的文件夹及其文件名，用于受限的文件系统。
    
- 范例：
    
    - 自定义词典：[https://github.com/fxsjy/jieba/blob/master/test/userdict.txt](https://github.com/fxsjy/jieba/blob/master/test/userdict.txt)
        
    - 用法示例：[https://github.com/fxsjy/jieba/blob/master/test/test_userdict.py](https://github.com/fxsjy/jieba/blob/master/test/test_userdict.py)
        
        - 之前：李小福 / 是 / 创新 / 办 / 主任 / 也 / 是 / 云 / 计算 / 方面 / 的 / 专家 /
            
        - 加载自定义词库后：　李小福 / 是 / 创新办 / 主任 / 也 / 是 / 云计算 / 方面 / 的 / 专家 /
            

### 调整词典

[](https://github.com/fxsjy/jieba#%E8%B0%83%E6%95%B4%E8%AF%8D%E5%85%B8)

- 使用 `add_word(word, freq=None, tag=None)` 和 `del_word(word)` 可在程序中动态修改词典。
    
- 使用 ```Java  
[
    {"role": "system", "content": "You are an assistant who can compose sentences with given words or phrases or sentences, you are expert in Chinese and English."},  
    {"role": "user", "content": f"如果"{word}" 是一个单词或者短语，那么用其生成一个30词以上的托福听力风格的英文句子，并把"{word}" 对应的英文用（）围起来，；如果"{word}" 是一句话的话，把它原封不动返回给我就好，返回结果中不应当有括号"}  
]

word 分别是 in a row， resolution， revolution， Like， that， toilet

```0 可调节单个词语的词频，使其能（或不能）被分出来。
    
- 注意：自动计算的词频在使用 HMM 新词发现功能时可能无效。

## 初心

我记得在 22 年的时候，自己几乎没有什么编程基础，简单学了一些 python，那个时代也没有 ChatGPT 这样的 AI 工具。但是自己就是靠着一股子学习英语的热诚，和一些本能的数据思维。借助了简单的爬虫、IO 流制作了能实现托福听力词频分析、日记词汇累积统计这样一些功能的小程序。说来也有趣，当时的前端用的就是 Pycharm 的 Teminal 界面，数据库用的就是 excel。粗糙且原始，但是其实已实现了后端开发，所谓 data in data out 的本质。更重要的是，那次的编程实践，真正激发了我对计算机技术的热情。没有那一次的实践，可能也不会有后来的转码，自己也不会从事上真正热爱的事业了。

所以，随着自己技术的进步和对英语学习认知的加深，我更怀有无比的热诚，去将自己积累的方法论固化到一个真正的软件之中。完成自己数据驱动的梦想，亦是将高效科学的学习方式分享给大众。

Viva 是我加入的第一家公司去哪儿网的办公楼的名字。Viva La Vida 是一句西班牙语，『生命万岁』或『酷玩人生』之意，恰如落笔之此刻，恰如这一路的计算机和英语学习之旅！

## 第一性原理

英语学习的本质就是大脑中的信息储备足以填满下面这样表结构的一张表

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F16%2F02-40-49-ae6c8c39ebb7dcff5f840967b6dd4834-202409160240119-c85568.png)

每个映射代表一块砖，四面墙代表四种映射  

左上：阅读词汇  
右上：听力词汇  
左下：口语词汇  
右下：写作词汇  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F07%2F31%2F14-39-20-28ac40c13e87651a3c4ef0a5ac3e7909-kisspng-brick-wall-clip-art-brick-5ab58e97d68fa9.2673084215218479598789-f32cc2.png)

四种映射共同组成了英语能力  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F07%2F31%2F14-38-12-512934aac593bfcb3810ce88f4f1350e-kisspng-stone-wall-brick-clip-art-brick-wall-5abf2a18ab3631.5554477315224775927013-4aa636.png)

```Java
[  
    {"role": "system", "content": "You are an assistant who can compose sentences with given words or phrases or sentences, you are expert in Chinese and English."},  
    {"role": "user", "content": f"如果“{word}” 是一个单词或者短语，那么用其生成一个30词以上的托福听力风格的英文句子，并把“{word}” 对应的英文用（）围起来，；如果“{word}” 是一句话的话，把它原封不动返回给我就好，返回结果中不应当有括号"}  
]

word 分别是 in a row， resolution， revolution， Like， that， toilet


```

## 两小两大

[音标](音标.md)和[英语语法@](英语语法@.md)是两个非常小的模块  
主要的工作量在于[被动词汇](被动词汇)和[主动词汇](主动词汇.md)的积累
