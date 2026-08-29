---
uid: 1aa1ab64-6ec6-4c5b-ab77-a945d7ca2b30
---



## 数据量
现代人可能对文本数据大小以及计算机处理文本速度的**数量级**概念不深了。

维基百科中文全文总共才 6 亿字，每个中文UTF-8编码3个字节计算，2GB不到。全文下载的XML文件解压后约10GB（有一大堆占地方的元数据XML标记）

用最基本的没有任何索引优化的 grep 命令做全文正则表达式搜索，在一台Macbook Pro M1上大概就1分钟。

任何正常人类能够积累的个人知识库，用[markdown](https://zhida.zhihu.com/search?content_id=554082947&content_type=Answer&match_order=1&q=markdown&zhida_source=entity)存储的大小都比维基百科小几个数量级。不做任何优化的 grep 全文检索耗时如果超过两秒那电脑得扔了（主要是IO有问题）。

我的从大学起积累到现在快20年的笔记库，从[emacs-wiki](https://zhida.zhihu.com/search?content_id=554082947&content_type=Answer&match_order=1&q=emacs-wiki&zhida_source=entity)到[muse](https://zhida.zhihu.com/search?content_id=554082947&content_type=Answer&match_order=1&q=muse&zhida_source=entity)到markdown经历了多次全库格式转换，数千篇笔记，至今最有效的全文检索依然是一个grep秒出。
