---
date created: 2025-09-01
date modified: 2025-09-01
uid: 5386b406-a12b-4f32-bbb5-ec050d5f3e3e
---

| 问题                   | 关键词/关键概念                                                                                                         |
| -------------------- | ---------------------------------------------------------------------------------------------------------------- |
| RabbitMQ如何防止重复消费？| **幂等设计**：唯一ID、数据库约束<br>**消息去重**：Redis/DB记录消息ID<br>**状态机**：业务状态控制<br>**分布式锁**：处理时加锁                               |
| RabbitMQ如何保证高可用？| **集群模式**：普通集群、镜像队列集群<br>**镜像队列**：多节点副本、主从同步<br>**Quorum队列(3.8+)**：Raft协议、强一致性<br>**Federation**：跨数据中心            |
| RabbitMQ如何保证消息不丢？| **生产端**：confirm机制、事务、备份交换机<br>**存储端**：持久化、镜像队列、集群<br>**消费端**：手动ACK、重试机制、死信队列                                     |
| RabbitMQ如何实现消费端限流？| **QoS设置**：basicQos(prefetchCount)<br>**预取数量**：控制未确认消息数<br>**手动ACK**：处理完再确认<br>**拒绝策略**：basicReject/basicNack     |
| RabbitMQ怎么做消息分发？| **轮询分发**：默认Round-Robin<br>**公平分发**：根据消费能力QoS<br>**消息路由**：Direct/Topic/Fanout/Headers<br>**优先级队列**：消息优先级          |
| RabbitMQ如何实现延迟消息？| **TTL+死信**：消息过期进死信队列<br>**延迟插件**：rabbitmq_delayed_message_exchange<br>**定时任务**：扫描数据库发送                           |
| RabbitMQ整体架构？| **核心组件**：Producer→Exchange→Queue→Consumer<br>**节点结构**：Connection→Channel→Virtual Host<br>**存储层**：Mnesia数据库、消息持久化 |
| 什么是死信队列？| **触发条件**：TTL过期、队列满、消费拒绝<br>**死信交换机**：x-dead-letter-exchange<br>**应用场景**：延迟队列、重试机制、异常处理                           |
| RabbitMQ事务机制？| **AMQP事务**：txSelect/txCommit/txRollback<br>**性能问题**：同步阻塞、吞吐量降250倍<br>**替代方案**：Publisher Confirm更高效               |
| 如何保障消息一定发送到RabbitMQ？| **Confirm机制**：异步确认<br>**Return机制**：路由失败回调<br>**备份交换机**：Alternate Exchange<br>**重试机制**：失败重发                       |

# RabbitMQ核心架构详解

## **1. 整体架构**

```Java
生产者(Producer)
    ↓ [Connection/Channel]
Exchange(交换机)
    ├── Direct Exchange (精确匹配)
    ├── Topic Exchange (模式匹配)
    ├── Fanout Exchange (广播)
    └── Headers Exchange (头部匹配)
    ↓ [Binding/Routing Key]
Queue(队列)
    ├── Classic Queue (经典队列)
    ├── Quorum Queue (仲裁队列)
    └── Stream Queue (流队列)
    ↓ [Push/Pull]
消费者(Consumer)

管理组件：
├── Virtual Host (虚拟主机隔离)
├── Connection (TCP连接)
├── Channel (信道复用)
└── Mnesia (元数据存储)
```

## **2. 高可用方案**

### **A. 镜像队列(Classic Mirrored)**

```bash
# 设置镜像策略
rabbitmqctl set_policy ha-all "^" \
  '{"ha-mode":"all","ha-sync-mode":"automatic"}'

# ha-mode选项：
- all: 所有节点
- exactly: 指定数量
- nodes: 指定节点
```

### **B. Quorum队列(推荐)**

```java
// 声明Quorum队列
Map<String, Object> args = new HashMap<>();
args.put("x-queue-type", "quorum");
args.put("x-quorum-initial-group-size", 3);  // 初始副本数

channel.queueDeclare("quorum-queue", true, false, false, args);

// 优势：
// - Raft协议保证一致性
// - 自动故障转移
// - 无需同步等待
```

### **C. 集群模式对比**

|模式|优点|缺点|适用场景|
|---|---|---|---|
|**普通集群**|简单、扩展读能力|队列单点、数据不同步|开发测试|
|**镜像队列**|数据冗余、自动故障转移|性能损耗、网络压力大|传统高可用|
|**Quorum队列**|强一致、性能好|3.8+版本、功能受限|生产推荐|
|**Federation**|跨数据中心、松耦合|最终一致、配置复杂|多数据中心|

## **3. 消息不丢失完整方案**

```java
// 生产端确认
public class ReliableProducer {
    // 1. 开启Confirm模式
    channel.confirmSelect();
    
    // 2. 添加确认监听器
    channel.addConfirmListener(new ConfirmListener() {
        @Override
        public void handleAck(long deliveryTag, boolean multiple) {
            // 消息成功到达Exchange
        }
        
        @Override
        public void handleNack(long deliveryTag, boolean multiple) {
            // 消息未到达，需要重发
            retrySend(deliveryTag);
        }
    });
    
    // 3. 开启Return机制
    channel.addReturnListener(new ReturnListener() {
        @Override
        public void handleReturn(int replyCode, String replyText,
                               String exchange, String routingKey,
                               AMQP.BasicProperties properties, byte[] body) {
            // 消息未路由到队列，处理失败
            handleRoutingFailure(body);
        }
    });
    
    // 4. 发送持久化消息
    AMQP.BasicProperties props = new AMQP.BasicProperties.Builder()
        .deliveryMode(2)  // 持久化
        .messageId(UUID.randomUUID().toString())
        .build();
        
    channel.basicPublish(exchange, routingKey, true, props, message);
}

// 消费端确认
public class ReliableConsumer {
    // 手动ACK模式
    channel.basicConsume(queueName, false, new DefaultConsumer(channel) {
        @Override
        public void handleDelivery(String consumerTag, Envelope envelope,
                                 AMQP.BasicProperties properties, byte[] body) {
            try {
                // 业务处理
                processMessage(body);
                // 手动确认
                channel.basicAck(envelope.getDeliveryTag(), false);
            } catch (Exception e) {
                // 处理失败，重新入队
                channel.basicNack(envelope.getDeliveryTag(), false, true);
            }
        }
    });
}
```

## **4. 消费端限流实现**

```java
// QoS设置
channel.basicQos(
    0,      // prefetchSize: 消息大小限制，0表示不限制
    10,     // prefetchCount: 未确认消息数量
    false   // global: false表示Consumer级别，true表示Channel级别
);

// 实际效果：
// - 每个消费者最多处理10条未确认消息
// - 超过限制后，RabbitMQ停止推送
// - 确认后继续推送
```

## **5. 防止重复消费**

```java
@Component
public class IdempotentConsumer {
    @Autowired
    private RedisTemplate<String, String> redisTemplate;
    
    @RabbitListener(queues = "order.queue")
    public void handleMessage(Message message) {
        String messageId = message.getMessageProperties().getMessageId();
        
        // 1. 分布式锁防并发
        String lockKey = "lock:" + messageId;
        Boolean locked = redisTemplate.opsForValue()
            .setIfAbsent(lockKey, "1", 10, TimeUnit.SECONDS);
            
        if (!locked) {
            return;  // 其他实例正在处理
        }
        
        try {
            // 2. 幂等性检查
            String processKey = "processed:" + messageId;
            Boolean isNew = redisTemplate.opsForValue()
                .setIfAbsent(processKey, "1", 24, TimeUnit.HOURS);
                
            if (!isNew) {
                return;  // 已处理过
            }
            
            // 3. 业务处理
            processOrder(message);
            
        } finally {
            redisTemplate.delete(lockKey);
        }
    }
}
```

## **6. 消息分发策略**

```java
// 1. 轮询分发(默认)
channel.basicConsume(queueName, autoAck, consumer);

// 2. 公平分发
channel.basicQos(1);  // 每次只发1条
channel.basicConsume(queueName, false, consumer);  // 手动ACK

// 3. 基于优先级
Map<String, Object> args = new HashMap<>();
args.put("x-max-priority", 10);  // 最大优先级
channel.queueDeclare("priority-queue", true, false, false, args);

// 发送带优先级的消息
AMQP.BasicProperties props = new AMQP.BasicProperties.Builder()
    .priority(5)  // 消息优先级
    .build();
```

## **7. 延迟消息实现**

### **方案1: TTL + 死信队列**

```java
// 1. 声明死信交换机和队列
channel.exchangeDeclare("dlx.exchange", "direct");
channel.queueDeclare("dlx.queue", true, false, false, null);

// 2. 声明延迟队列(带TTL和死信配置)
Map<String, Object> args = new HashMap<>();
args.put("x-message-ttl", 10000);  // 10秒过期
args.put("x-dead-letter-exchange", "dlx.exchange");
args.put("x-dead-letter-routing-key", "dlx.routing.key");
channel.queueDeclare("delay.queue", true, false, false, args);

// 3. 发送延迟消息
channel.basicPublish("", "delay.queue", props, message);
```

### **方案2: 延迟插件**

```java
// 安装插件：rabbitmq-plugins enable rabbitmq_delayed_message_exchange

// 声明延迟交换机
Map<String, Object> args = new HashMap<>();
args.put("x-delayed-type", "direct");
channel.exchangeDeclare("delayed.exchange", "x-delayed-message", true, false, args);

// 发送延迟消息
AMQP.BasicProperties props = new AMQP.BasicProperties.Builder()
    .headers(Collections.singletonMap("x-delay", 5000))  // 延迟5秒
    .build();
channel.basicPublish("delayed.exchange", routingKey, props, message);
```

## **8. 死信队列(DLX)详解**

```java
// 死信触发条件：
// 1. 消息TTL过期
// 2. 队列达到最大长度
// 3. 消费者拒绝(requeue=false)

// 配置示例
Map<String, Object> args = new HashMap<>();
args.put("x-dead-letter-exchange", "dlx.exchange");
args.put("x-dead-letter-routing-key", "dlx.key");
args.put("x-message-ttl", 60000);  // 60秒过期
args.put("x-max-length", 1000);    // 最大1000条

channel.queueDeclare("business.queue", true, false, false, args);
```

## **9. 事务 vs Confirm对比**

|特性|事务机制|Confirm机制|
|---|---|---|
|**性能**|极差(降250倍)|好(异步)|
|**同步性**|同步阻塞|异步回调|
|**批量处理**|不支持|支持批量确认|
|**使用复杂度**|简单|稍复杂|
|**推荐度**|不推荐|生产推荐|

## **10. 最佳实践**

```yaml
# 生产端最佳实践
1. 开启Publisher Confirm
2. 开启Publisher Return  
3. 消息持久化
4. 设置消息ID去重
5. 实现重试机制

# 消费端最佳实践
1. 手动ACK确认
2. 设置QoS限流
3. 幂等性设计
4. 异常重试策略
5. 死信队列兜底

# 运维最佳实践
1. 使用Quorum队列
2. 监控队列积压
3. 设置内存/磁盘告警
4. 定期备份数据
5. 合理的TTL策略
```
