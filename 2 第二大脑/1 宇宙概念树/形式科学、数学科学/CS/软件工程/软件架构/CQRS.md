---
draw:
title: CQRS
date created: 2025-05-06
date modified: 2025-05-13
---
> 举个例子，<<刑事诉讼法>>是很少的写，很多的读，比如教材里面、法院的法官、律师们的参考资料等等

当然知道！CQRS（Command Query Responsibility Segregation，命令-查询职责分离）是一种在后端架构中常用来改进可伸缩性、性能与复杂度管理的设计模式。下面给你一个结构化、落地的总览，方便快速掌握与实践。

---

## 1 核心理念

|关注点|Command（写）|Query（读）|
|---|---|---|
|意图|改变系统状态|仅获取数据|
|模型|领域模型（具备业务不变式和行为）|投影 / 视图模型（面向 UI 或外部 API 的只读结构）|
|事务性|强一致 / 事务支持|最终一致 / 可读优化|
|典型技术|DDD 聚合、事件溯源|物化视图、缓存、ELK / OLAP|

**一句话**：把“改数据”和“查数据”彻底拆开，各自用最合适的代码和存储，避免“一刀切”的巨型表/巨型服务。

---

## 2 为什么要用 CQRS？

1. **读写负载不对称**  
    电商、社交 Feed 等场景常是 90%+ 读取，10%- 写入。拆分后可为读侧做缓存或只读副本，为写侧保留严格事务。
    
2. **不同的性能特征**  
    写侧往往需要复杂验证与事务，读侧只需查询快。不同索引、Schema、甚至不同数据库类型（如写在 PostgreSQL，读走 Elasticsearch）能各司其职。
    
3. **复杂领域逻辑隔离**  
    写侧可遵循 DDD，保持领域纯净；读侧模型可针对 UI 展平、去规范化。
    
4. **演进与扩展**  
    新增报表、不停机热读库迁移等，都只改读侧。写侧不动即可保证业务一致性。
    

---

## 3 典型实施步骤

> 以 Java Spring + Kafka + PostgreSQL + Elasticsearch 为例

1. **划分接口**
    - `POST /orders` → Command（创建订单）
    - `GET /orders/{id}/detail` → Query（查看订单详情）
        
2. **实现写模型（Command Side）**
    - 设计聚合 `Order`，封装业务规则。
    - 写请求进入应用层 → 调用 `OrderService.placeOrder(cmd)` → 持久化到 **写库**。
    - 同时发布领域事件 `OrderPlacedEvent` 到 Kafka。
        
3. **事件消费者更新读模型（Projection / Read Side）**
    - `OrderProjectionConsumer` 订阅 `OrderPlacedEvent`。
    - 把所需字段写入 **读库**（Elastic 或分库分表的只读 PostgreSQL），结构可直接满足前端页面。
        
4. **查询服务**
    - `OrderQueryService` 仅访问读库，返回 DTO 给客户端。
    - 读侧可加 Redis / CDN 缓存，保证毫秒级延迟。
        
5. **一致性策略**
    - 采用“最终一致”：写成功即返回 200，读侧稍后可见。
    - 业务若需“同步可见”，可让客户端轮询或使用 WebSocket 通知“订单已创建”。
        
6. **监控与补偿**
    - 监控 Kafka 消费滞后。
    - 失败重放或使用幂等指令模式防止重复写读库。
        

---

## 4 落地示例代码片段（简化）

```java
// 1. Command - 创建订单
public record PlaceOrderCmd(UUID userId, List<Item> items) {}

@Service
public class OrderService {
    @Transactional
    public UUID placeOrder(PlaceOrderCmd cmd) {
        Order order = Order.create(cmd);
        orderRepo.save(order);
        eventBus.publish(new OrderPlacedEvent(order));
        return order.getId();
    }
}
```

```java
// 2. Projection - 更新读侧
@Component
public class OrderProjectionConsumer {
    @KafkaListener(topics = "order-placed")
    public void on(OrderPlacedEvent evt) {
        OrderView view = OrderView.from(evt);
        orderViewRepo.save(view);           // 存 Elastic / Mongo / 分表
    }
}
```

```java
// 3. Query - 查询订单
@RestController
public class OrderQueryController {
    @GetMapping("/orders/{id}/detail")
    public OrderViewDetailDTO get(@PathVariable UUID id) {
        return orderViewRepo.findDetail(id)
               .orElseThrow(() -> new NotFoundException("订单不存在"));
    }
}
```

---

## 5 常见陷阱与权衡

|陷阱|说明与应对|
|---|---|
|一致性难以保证|业务要求“写后立刻读到”时，要用同步投影或事务内双写，但会削弱并行度|
|事件溯源复杂度|事件模型好处大，但引入 Kafka / Pulsar 需治理幂等、重放、版本迁移|
|读模型爆炸|每个界面都建一张表→维护成本高；用通用查询引擎或 GraphQL 聚合可减缓|
|过度设计|小型 CRUD 应用无需 CQRS。先单模型，等性能瓶颈或复杂度真出现再拆|

---

## 6 什么时候值得上 CQRS？

- **读写 QPS 差异 ≥ 10 倍**，且写侧需要强事务。
    
- **领域规则复杂、聚合大**：单表无法既满足写侧事务又满足读侧性能。
    
- **微服务拆分**需稳定 API，CQRS 让读写职责天然解耦，演进风险低。
    
- **需要事件驱动**（Audit、异步工作流、实时分析）。
    

---

## 7 快速验证思路

1. **基准测试**：先对传统单库做压测，记录读写延迟；再切 CQRS 观测指标。
    
2. **灰度上线**：读侧先切只读副本，写侧保持不变；观察一致性告警。
    
3. **指标监控**：写库 TPS、读库 QPS、Kafka lag、投影延迟 (ms)。
    
4. **恢复演练**：模拟 Kafka 消息丢失、读模型重建，确保事件可追溯。
    

---

### 小结

CQRS 不是银弹，但在**高读写不对称、业务行为复杂、需要事件驱动**的场景下能显著提升可维护性与性能。建议先从“**接口层读写分离**”做起，再逐步引入“事件 + 投影”的完整架构。

如果你具体想在现有 Java / Spring / MyBatis 或其他栈里改造，或者需要示例项目结构、脚手架配置，随时告诉我！
