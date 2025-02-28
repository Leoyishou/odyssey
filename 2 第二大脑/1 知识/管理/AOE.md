---
aliases: [Activity On Edge, 活动边网络]
draw: 
title: AOE
date created: 2024-08-24
date modified: 2025-02-06
---

```mermaid
graph LR
    1((1)) --> |需求分析 3天| 2((2))
    1 --> |系统设计 4天| 3((3))
    2 --> |数据库设计 2天| 4((4))
    3 --> |后端开发 6天| 5((5))
    4 --> |前端开发 5天| 6((6))
    5 --> |集成测试 3天| 6
    6 --> |用户验收测试 2天| 7((7))

    style 1 fill:#lightblue,stroke:#333,stroke-width:2px
    style 2 fill:#lightblue,stroke:#333,stroke-width:2px
    style 3 fill:#lightblue,stroke:#333,stroke-width:2px
    style 4 fill:#lightblue,stroke:#333,stroke-width:2px
    style 5 fill:#lightblue,stroke:#333,stroke-width:2px
    style 6 fill:#lightblue,stroke:#333,stroke-width:2px
    style 7 fill:#lightblue,stroke:#333,stroke-width:2px
```
