---
draw:
tags: []
title: DDD
date created: 2024-08-19
date modified: 2024-12-27
---

ddd说白了就是以前的service太乱太多了，把他们按照微服务的思想group by在了几个核心领域下  
mapping的出生和essay的出生有关，但是之后的更新就和essay包括sentence都没关系了。ddd的核心就是把domain划分好，每个domain提供自己的能力，最外层的大逻辑放在application层  
![image.png|1800](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F01%2F16-13-49-a18227f44755f78e7dffc31ab0b840fa-202410011613361-7387ac.png)

## 贫血充血？

贫血模型：业务逻辑主要在服务层实现。就比如普通的`VO`和`DTO`，只是作为一个数据 bean，里面不会有太多动作  
充血模型：业务逻辑主要在域对象内部实现，比如`ProductRoom`这种，类里面有很多业务动作

贫血模型：对象主要负责存储数据，类似于数据结构。
充血模型：对象不仅存储数据，还负责实现与之相关的业务逻辑。

## 分层

```sql
com/qunar/hotel/
│
├── interfaces/
│   ├── DetailController.java            # 传统的controller
│   │
│   └── service/
│       └── AbstractPriceService.java    # 传统的service
│
├── engine/
│   └── TransferBizProcess.java          # 传统service中的核心处理逻辑
│
└── infrastructure/  # 基础层
    └── future/                          # 外部调用
        ├── AsyncFuture.java
        ├── FutureCallback.java
        ├── FutureTask.java
        └── ... (其他相关的future类)
```

## 通用语言

怎么理解通用语言这个概念呢？在事件风暴过程中，通过团队交流达成共识的，能够简单、清晰、准确描述业务涵义和规则的语言就是通用语言。也就是说，通用语言是团队统一的语言，不管你在团队中承担什么角色，在同一个领域的软件生命周期里都使用统一的语言进行交流。

比如商促、抽佣、优惠黑名单这些就是通用语言。我们现在==事件风暴==还不够规范，导致一些通用语言并没有对齐。

不过，一个好处是，在数字化工具中，通过工程的方式实现了下面的表格。
![image.png|1500](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F29%2F14-28-41-ab406962b2bc951065bd6535229b5b7b-202409291428559-e48d3c.png)

Here's the Markdown representation of the table in the image:

| 层   | 聚合  | 领域对象名称     | 领域类型 | 依赖的领域对象    | 包名                                     | 类名                                 |
| --- | --- | ---------- | ---- | ---------- | -------------------------------------- | ---------------------------------- |
| 应用层 | /   | 创建请假信息应用服务 | 应用服务 | 创建请假信息领域服务 |.leave.application.service            | CreateLeaveInfoAppService          |
| 应用层 | /   | 请假审批已通过    | 事件发布 | 请假审批（审核）|.leave.application.event.publish      | SendApprovalEventInfo              |
| 领域层 | 请假  | 请假单        | 聚合根  |            |.leave.domain.leave.entity            | Leave                              |
| 领域层 | 请假  | 创建请假信息     | 命令   |            |.leave.domain.leave.entity            | Leave                              |
| 领域层 | 请假  | 审批轨迹       | 值对象  |            |.leave.domain.leave.entity            | ApprovalTrace                      |
| 领域层 | 请假  | 创建审批轨迹信息   | 命令   |            |.leave.domain.leave.entity            | ApprovalTrace                      |
| 领域层 | 请假  | 创建请假信息     | 领域服务 | 创建请假信息     |.leave.domain.leave.service           | CreateLeaveInfoDomService          |
| 领域层 | 请假  | 创建审批轨迹信息   | 领域服务 | 创建审批轨迹信息   |.leave.domain.leave.service           | CreateApprovalTraceDomService      |
| 领域层 | 人员  | 人员         | 聚合根  |            |.leave.domain.person.entity           | Person                             |
| 领域层 | 人员  | 创建人员信息     | 命令   |            |.leave.domain.person.entity           | Person                             |
| 领域层 | 人员  | 组织关系       | 值对象  |            |.leave.domain.person.entity           | PersonRelationship                 |
| 领域层 | 人员  | 创建组织关系     | 命令   |            |.leave.domain.person.entity           | PersonRelationship                 |
| 领域层 | 人员  | 创建人员信息     | 领域服务 | 创建人员信息     |.leave.domain.person.service          | CreatePersonInfoDomService         |
| 领域层 | 人员  | 创建组织关系     | 领域服务 | 创建组织关系     |.leave.domain.person.service          | CreatePersonRelationshipDomService |
| 基础层 | 请假  | 请假仓储接口     | 仓储接口 |            |.domain.leave.repository.facade       | LeaveRepositoryInterface           |
| 基础层 | 请假  | 请假仓储实现     | 仓储实现 |            |.domain.leave.repository.persistence  | LeaveRepositoryImpl                |
| 基础层 | 人员  | 人员仓储接口     | 仓储接口 |            |.domain.person.repository.facade      | PersonRepositoryInterface          |
| 基础层 | 人员  | 人员仓储实现     | 仓储实现 |            |.domain.person.repository.persistence | PersonRepositoryImpl               |

|     |       |                       |
| --- | ----- | --------------------- |
|     | 场景    | sentence + focus_word |
|     | 文章    | essay                 |
|     |       |                       |
|     | 词卡    | word_card             |
|     | 用户的翻译 | user_translation      |
|     | AI的翻译 | ai_translation        |

## DDD课程大纲

### 开篇词

| 序号 | 标题 | 时长 |
|------|------|------|
| 1 | 开篇词 \| 学好了DDD，你能做什么？| 10:18 |

### 基础篇

| 序号  | 标题                       | 我的理解                   |
| --- | ------------------------ | ---------------------- |
| 01  | 领域驱动设计：微服务设计为什么要选择DDD？|                        |
| 02  | 领域、子域、核心域、通用域和支撑域：傻傻分不清？|                        |
| 03  | 限界上下文：定义领域边界的利器          | 一个限界上下文理论上就可以设计为一个微服务。|
| 04  | 实体和值对象：从领域模型的基础单元看系统设计   |                        |
| 05  | 聚合和聚合根：怎样设计聚合？|                        |

### 进阶篇

| 序号 | 标题 | 时长 |
|------|------|------|
| 06 | 领域事件：解耦微服务的关键 | 17:38 |
| 07 | DDD分层架构：有效降低层与层之间的依赖 | 16:45 |
| 08 | 微服务架构模型：几种常见模型的对比和分析 | 16:34 |
| 09 | 中台：数字转型后到底应该共享什么？| 18:23 |
| 10 | DDD、中台和微服务：它们是如何协作的？| 14:32 |
| - | 答疑：有关3个典型问题的讲解 | 11:03 |

### 实战篇

| 序号  | 标题                         |                               |
| --- | -------------------------- | ----------------------------- |
| 11  | DDD实践：如何用DDD重构中台业务模型？|                               |
| 12  | 领域建模：如何用事件风暴构建领域模型？| 事件风暴说白了就是一起开个会，定义业务名词和对应的代码命名 |
| 13  | 代码模型（上）：如何使用DDD设计微服务代码模型？|                               |
| 14  | 代码模型（下）：如何保证领域模型与代码模型的一致性？|                               |
| 15  | 边界：微服务的各种边界在架构演进中的作用？|                               |
| 16  | 视图：如何实现服务和数据在微服务各层的协作？|                               |
| 17  | 从后端到前端：微服务后，前端如何设计？|                               |
| 18  | 知识点串讲：基于DDD的微服务设计实例        |                               |
| 19  | 总结（一）：微服务设计和拆分要坚持哪些原则？|                               |
| 20  | 总结（二）：分布式架构关键设计10问         |                               |

### 结束语

| 序号 | 标题 | 时长 |
|------|------|------|
| 1 | 结束语 \| 所谓高手，就是跨过坑和大海！| 08:18 |

### 专栏加餐

| 序号 | 标题 | 时长 |
|------|------|------|
| 1 | 基于DDD的微服务设计实例代码详解 | 26:01 |
| 2 | 抽奖｜《DDD实战课》沉淀成书了，感谢有你！| 09:26 |

### 期末测试

| 序号 | 标题 | 时长 |
|------|------|------|
| 1 | 期末测试 \| 有关DDD的内容，你掌握了多少呢？| 00:30 |
