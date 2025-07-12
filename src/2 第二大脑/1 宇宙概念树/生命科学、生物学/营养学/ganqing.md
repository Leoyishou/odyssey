---
date created: 2025-06-05
date modified: 2025-07-10
uid: 76d5d063-1af2-4b99-8c9d-bad2396067c7
---
```mermaid

flowchart LR
    A[抓取帖子] --> B[统一清洗]
    B --> C{规则粗筛}
    C -->|命中| D[LLM 细判]
    C -->|未命中| H[非成型路线链路]
    D -->|Itinerary| E[路线标准化]
    D -->|Non| H
    E --> F[指标计算 & 分级]
    F --> G[写 Redis 缓存]
    H --> I[目的地/POI 抽取+挂靠]
    I --> G
    G --> J[Rest/GraphQL API]

```
