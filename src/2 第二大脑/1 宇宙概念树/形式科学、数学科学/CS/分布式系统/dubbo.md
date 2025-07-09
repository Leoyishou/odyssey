---
comment_id: 1ab68fa3
date created: 2024-09-04
date modified: 2025-02-06
draw: null
title: dubbo
---
## 通信协议

dubbo协议（默认）：端口20880，使用 [[Netty]] 通信

```Java
<dubbo:protocol name="dubbo" port="20800" threads="600" queues="1000"/>
```

## 序列化协议

```Java
如果没有特别配置，Dubbo 默认使用 hessian2 序列化。
```

服务/引用级别  
`<dubbo:service>`（服务提供方）或 `<dubbo:reference>`（服务消费方）标签配置

全局级别  
`<dubbo:provider>`、`<dubbo:consumer>` 或 `<dubbo:protocol>`

方法级别  
`<dubbo:method>`
