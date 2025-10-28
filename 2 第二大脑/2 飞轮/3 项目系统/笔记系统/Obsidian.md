---
date created: 2024-07-15
date modified: 2025-10-20
uid: 29dcaf0f-a1d1-4568-a64d-820adaa4556e
---

1. 笔记有用吗？浪费注意力？复写率
	1. 人脑的特点：缓存的逻辑（人脑与世界信息之间的缓存）
	2. 手写还是电子：
		1. 容易编辑
		2. 搜索
			1. 命名，标准化，与世界知识尽量对齐，[[实体链接, 挂靠]]，摒弃了过去自己发明语言系统做比喻
			2. 和 IDE 一样的搜索体验
2. [DIKW](DIKW.md)    [[第二大脑]]：从数据到智慧
	2. 所有的看到的听到的，和模型聊天收到的信息都结构化归入
	3. inbox解决两个痛点 一是很多[闪念笔记](闪念笔记.md)不及时记录会丢，二是各个平台有很多收藏从来不看 [管理学@](管理学@.md)
3. [PARA](PARA.md)  工作流
4. 本质：五种[学习@](学习@.md)方法的结合、[信息块理论](信息块理论.md)
5. 其他
	1. 标签系统 #第一性原理 #最佳实践   #comment
	2. 关于语义搜索，覆盖率×召回率，召回率够，但是覆盖率不足，无法获取新知识，老知识不如用对齐的文件名搜索
	3. public 和 sync


# Notes
## 数据量
现代人可能对文本数据大小以及计算机处理文本速度的**数量级**概念不深了。

维基百科中文全文总共才 6 亿字，每个中文UTF-8编码3个字节计算，2GB不到。全文下载的XML文件解压后约10GB（有一大堆占地方的元数据XML标记）

用最基本的没有任何索引优化的 grep 命令做全文正则表达式搜索，在一台Macbook Pro M1上大概就1分钟。

任何正常人类能够积累的个人知识库，用[markdown](https://zhida.zhihu.com/search?content_id=554082947&content_type=Answer&match_order=1&q=markdown&zhida_source=entity)存储的大小都比维基百科小几个数量级。不做任何优化的 grep 全文检索耗时如果超过两秒那电脑得扔了（主要是IO有问题）。

我的从大学起积累到现在快20年的笔记库，从[emacs-wiki](https://zhida.zhihu.com/search?content_id=554082947&content_type=Answer&match_order=1&q=emacs-wiki&zhida_source=entity)到[muse](https://zhida.zhihu.com/search?content_id=554082947&content_type=Answer&match_order=1&q=muse&zhida_source=entity)到markdown经历了多次全库格式转换，数千篇笔记，至今最有效的全文检索依然是一个grep秒出。



## Obsidian FastStart脚本使用指南

## 背景介绍

Obsidian是一款以速度闻名的笔记软件，即使在较旧的设备上运行也很快。但是，当安装大量插件（25个或更多）时，应用程序的启动时间会变得较慢，特别是在移动设备上。

### 为什么需要提高启动速度？

虽然PC上几秒钟的启动时间可以接受，但在移动设备上（如iPhone和iPad）5-8秒的启动时间会影响使用体验：

1. 移动设备内存有限，后台程序容易被系统卸载
2. 每次重新打开都需要完整加载
3. 启动延迟会造成使用摩擦，影响使用意愿

## FastStart脚本原理

### Obsidian的启动过程

Obsidian启动时需要完成以下步骤：

1. 初始化内部内存缓存（保管库文件信息数据库）
2. 加载UI元素（按钮、窗格、工作区等）
3. 逐个加载已启用的插件

### FastStart的工作方式

FastStart脚本通过以下方式优化启动速度：

1. 仅启用必要的核心插件进行启动
2. 快速完成初始化，使软件可用
3. 在后台分批延迟加载其他插件：
   - 第一批：启动2秒后加载
   - 第二批：启动30秒后加载

## 安装配置

### 前期准备

1. 安装Templater插件
2. 设置Templater模板文件夹

### 必要文件

需要创建以下文件：

1. `FastStart-StartupScript`（放在Templater文件夹中）
   - 包含主要JavaScript代码
   - 控制插件加载逻辑

2. `FastStart-GenerateListOfInstalledPlugins`（放在Templater文件夹中）
   - 用于生成已安装插件列表及ID

3. `FastStart-Plugins-ShortDelay`（可放在任意位置）
   - 存放需要短延迟加载的插件ID列表

4. `FastStart-Plugins-LongDelay`（可放在任意位置）
   - 存放需要长延迟加载的插件ID列表

### 配置步骤

1. **配置Templater**
   - 在Templater设置中指定startup template
   - 选择FastStart-StartupScript作为启动脚本

2. **基准测试**
   - 记录当前启动时间作为基准
   - 测试安全模式下的最小启动时间

3. **插件分类**
   - 确定必须立即启动的插件（如Templater）
   - 规划短延迟加载的插件
   - 规划长延迟加载的插件

## 优化调试

### 调试方法

1. 启用"Debug startup time"选项
2. 记录并对比不同配置下的启动时间
3. 测试不同插件组合的效果

### 性能调优

1. 调整延迟时间（默认2秒和30秒）
2. 优化插件加载顺序
3. 处理特殊插件（某些插件可能不支持延迟加载）

## 最佳实践

1. 仔细评估每个插件的必要性
2. 将最常用的插件放入短延迟列表
3. 定期检查和调整插件配置
4. 为不同设备定制最佳延迟时间

## 注意事项

1. Templater插件必须保持启用状态
2. 某些插件可能需要立即启动
3. 调试过程需要反复测试和优化
4. 可能需要针对不同设备调整配置
