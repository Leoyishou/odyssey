---
date created: 2025-09-01
date modified: 2025-09-01
uid: cde34b11-5f5b-431a-a09e-82a12fe169c5
---

| 问题                   | 关键词/关键概念                                                                                                                           |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Zookeeper是CP还是AP？| **CP系统**：强一致性、分区容错性<br>**牺牲可用性**：少数节点故障时不可用、选举期间不可用<br>**过半机制**：需要超过半数节点存活才能提供服务                                                   |
| Zookeeper选举机制？| **FastLeaderElection算法**<br>**投票规则**：epoch大优先→zxid大优先→myid大优先<br>**过半原则**：获得超过半数票当选Leader<br>**选举时机**：集群启动、Leader宕机                |
| Zookeeper数据结构？| **树形结构**：类似文件系统<br>**ZNode节点**：持久节点、临时节点、顺序节点、容器节点(3.5+)<br>**数据限制**：每个节点1MB<br>**路径表示**：斜杠分隔如/app/config                          |
|                      |                                                                                                                                    |
|                      |                                                                                                                                    |
| Zookeeper如何保证节点唯一？| **顺序节点**：自动追加10位递增序号<br>**原子性**：单线程处理、事务ID(zxid)<br>**版本控制**：数据版本(version)乐观锁<br>**路径唯一**：同级目录下节点名唯一                               |
| Zookeeper的watch机制？| **一次性触发**：触发后自动移除需重新注册<br>**事件类型**：NodeCreated/Deleted/DataChanged/ChildrenChanged<br>**异步通知**：服务端主动推送<br>**顺序保证**：先收到watch事件再看到变化 |
| Zookeeper典型应用场景？| **配置中心**：统一配置管理<br>**服务注册发现**：Dubbo注册中心<br>**分布式锁**：临时顺序节点<br>**集群管理**：节点上下线感知<br>**分布式队列**：FIFO队列<br>**命名服务**：全局唯一ID              |
| Zookeeper的缺点？| **性能瓶颈**：写操作都在Leader、不适合大量写<br>**选举风暴**：大规模集群选举耗时<br>**网络分区**：脑裂问题<br>**GC影响**：长时间GC导致超时<br>**不适合大数据存储**：1MB限制                     |
| Zookeeper集群角色？| **Leader**：处理写请求、事务请求、发起投票<br>**Follower**：处理读请求、转发写请求、参与投票<br>**Observer**：处理读请求、不参与投票、扩展读性能                                      |
| 什么是脑裂？如何解决？| **脑裂**：网络分区导致多个Leader<br>**ZK解决**：过半机制、epoch机制、少数分区不可用<br>**Fencing**：通过epoch/zxid隔离旧Leader                                        |
| 如何用Zookeeper实现分布式锁？| **步骤**：1.创建临时顺序节点 2.判断是否最小 3.不是则watch前一个 4.获得锁执行 5.删除节点释放<br>**公平锁**：按序号排队<br>**可重入**：记录持有者信息                                      |
| 怎样使用Zookeeper实现服务发现？| **服务注册**：创建临时节点存储服务信息<br>**服务发现**：获取子节点列表<br>**健康检查**：session超时自动删除<br>**变更通知**：watch子节点变化                                         |

# Zookeeper核心架构详解

## **整体架构**

```Java
客户端层
├── ZooKeeper Client API
├── Session管理
└── Watch管理

服务端集群
├── Leader（领导者）
│   ├── 处理写请求
│   ├── 事务提议
│   └── 数据同步
├── Follower（跟随者）
│   ├── 处理读请求
│   ├── 转发写请求
│   └── 参与投票
└── Observer（观察者）
    ├── 处理读请求
    └── 同步数据（不投票）

存储层
├── 内存数据库（DataTree）
└── 事务日志（磁盘）
```

## **ZNode节点类型**

|节点类型|持久性|顺序性|用途|
|---|---|---|---|
|PERSISTENT|持久|否|普通数据存储|
|PERSISTENT_SEQUENTIAL|持久|是|分布式队列|
|EPHEMERAL|临时|否|服务注册、会话标识|
|EPHEMERAL_SEQUENTIAL|临时|是|分布式锁|
|CONTAINER(3.5+)|持久|否|容器节点|
|TTL(3.5+)|持久|否|超时自动删除|

## **选举算法(FastLeaderElection)**

```java
// 投票结构
class Vote {
    long id;      // 服务器ID (myid)
    long zxid;    // 事务ID
    long epoch;   // 选举轮次
}

// 选举流程
1. 初始投票给自己
2. 交换投票信息
3. 比较规则：
   if (vote.epoch > my.epoch) 更新
   else if (vote.epoch == my.epoch) {
       if (vote.zxid > my.zxid) 更新
       else if (vote.zxid == my.zxid && vote.id > my.id) 更新
   }
4. 统计投票，超过半数则选举成功
```

## **ZAB协议(原子广播)**

### **两种模式**

1. **恢复模式**：选举Leader、数据同步
2. **广播模式**：处理客户端请求

### **消息广播流程**

```Java
Client → Leader → Proposal → Followers
                → ACK(过半) → Commit → Followers
```

## **分布式锁实现**

```java
// 获取锁
public boolean lock(String lockPath) {
    // 1. 创建临时顺序节点
    String myNode = zk.create(lockPath + "/lock-", 
                              data, 
                              EPHEMERAL_SEQUENTIAL);
    
    // 2. 获取所有子节点
    List<String> children = zk.getChildren(lockPath);
    Collections.sort(children);
    
    // 3. 判断是否是最小节点
    if (myNode.equals(children.get(0))) {
        return true; // 获得锁
    }
    
    // 4. 监听前一个节点
    String prevNode = getPrevNode(myNode, children);
    zk.exists(prevNode, watcher);
    
    // 5. 等待通知
    wait();
}

// 释放锁
public void unlock() {
    zk.delete(myNode);
}
```

## **Watch机制原理**

```Java
注册Watch
Client → Server: 注册监听器
              ↓
         WatchManager存储

触发Watch  
数据变更 → WatchManager查找 → 异步通知Client
                           → 移除Watch(一次性)

重要特性：
- 一次性触发
- 异步通知
- 先收到事件，后看到新数据
- 不会丢失事件
```

## **性能优化**

### **集群规模建议**

- **3节点**：测试环境，容忍1个节点故障
- **5节点**：生产环境推荐，容忍2个节点故障
- **7节点**：大型集群，容忍3个节点故障
- 避免偶数节点（无优势）

### **配置优化**

```properties
# 会话超时（避免频繁重连）
tickTime=2000
minSessionTimeout=4000
maxSessionTimeout=40000

# 同步限制
initLimit=10  # Leader选举超时
syncLimit=5   # 数据同步超时

# 快照和日志
dataDir=/data/zookeeper
dataLogDir=/datalog/zookeeper  # 分离日志
autopurge.snapRetainCount=3
autopurge.purgeInterval=1
```

## **常见问题及解决**

|问题|原因|解决方案|
|---|---|---|
|**连接超时**|网络延迟、GC|调整sessionTimeout|
|**选举风暴**|大量节点同时选举|使用Observer节点|
|**性能下降**|写请求过多|读写分离、增加Observer|
|**脑裂**|网络分区|过半机制自动处理|
|**数据不一致**|异步同步|使用sync()强制同步|

## **与其他组件对比**

|对比项|Zookeeper|Etcd|Consul|
|---|---|---|---|
|**一致性协议**|ZAB|Raft|Raft|
|**语言**|Java|Go|Go|
|**性能**|中|高|高|
|**Watch**|一次性|持续|持续|
|**使用复杂度**|高|中|低|
|**主要用途**|配置、协调|K8s存储|服务发现|
