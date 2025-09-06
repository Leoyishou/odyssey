---
date created: 2025-05-26
date modified: 2025-09-01
uid: e14f5ca2-36d7-483f-9848-177d014623b9
---
：

| 问题                       | 关键词/关键概念                                                                                                                              |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| Tomcat与Web服务器(Apache)关系？| **Apache**：静态资源服务器、反向代理、负载均衡<br>**Tomcat**：Servlet容器、处理动态请求、Java应用服务器<br>**配合使用**：Apache处理静态资源+负载均衡、Tomcat处理业务逻辑、mod_jk/mod_proxy连接   |
| Tomcat的IO模型？| **BIO**：一请求一线程、阻塞IO、Tomcat8.5前默认<br>**NIO**：非阻塞、多路复用、Tomcat8.5+默认<br>**NIO2/AIO**：异步非阻塞<br>**APR**：Apache Portable Runtime、本地库优化      |
|                          |                                                                                                                                       |
| 为什么Tomcat线程数可设200而不是N+1？| **IO密集型**：大部分时间等待IO、CPU利用率低<br>**阻塞时间多**：数据库查询、网络调用、文件读写<br>**经验值**：线程数=CPU核数×(1+等待时间/计算时间)、Web应用等待时间远大于计算时间                          |
| Tomcat中有哪些类加载器？| **Bootstrap**：加载JVM核心类、rt.jar<br>**System**：加载CLASSPATH、tomcat启动类<br>**Common**：加载/lib共享类<br>**WebApp**：加载/WEB-INF/classes和lib、每个应用独立 |
| Tomcat处理请求的过程？| **流程**：Connector接收→创建Request/Response→Engine处理→Host匹配虚拟主机→Context定位应用→Wrapper找Servlet→Filter链→Servlet处理→返回响应                          |
| Tomcat的启动流程？| **步骤**：Bootstrap.main()→Catalina.start()→Server初始化→Service启动→Connector启动监听→Engine/Host/Context/Wrapper依次初始化→部署应用                      |
| Tomcat的类加载机制？| **破坏双亲委派**：WebAppClassLoader优先加载应用类<br>**隔离性**：不同应用类隔离<br>**共享性**：Common加载器共享<br>**顺序**：WEB-INF/classes→WEB-INF/lib→Common→System     |
| 过滤器和拦截器的区别？| **过滤器Filter**：Servlet规范、函数回调、依赖Servlet容器、作用所有请求<br>**拦截器Interceptor**：Spring框架、反射机制(AOP)、依赖Spring容器、作用Controller层                     |

# Tomcat架构详解

## **整体架构**

```Java
Server（服务器）
└── Service（服务）
    ├── Connector（连接器）- 处理网络连接
    │   ├── ProtocolHandler - 协议处理
    │   └── Adapter - 适配器
    └── Container（容器）- 处理业务逻辑
        └── Engine（引擎）
            └── Host（虚拟主机）
                └── Context（应用上下文）
                    └── Wrapper（Servlet包装器）
```

## **请求处理流程详细**

1. **接收请求**：Acceptor接收Socket连接
2. **协议解析**：ProtocolHandler解析HTTP协议
3. **创建对象**：创建Request/Response对象
4. **适配转换**：CoyoteAdapter适配到容器
5. **容器处理**：
    - Engine：顶层容器处理
    - Host：根据域名匹配虚拟主机
    - Context：根据URL路径匹配应用
    - Wrapper：定位具体Servlet
6. **过滤器链**：执行Filter链
7. **业务处理**：Servlet.service()处理
8. **返回响应**：逆向返回结果

## **类加载器层次**

```Java
       Bootstrap ClassLoader
              ↓
        System ClassLoader
              ↓
        Common ClassLoader
         ↙        ↘
    Catalina    Shared ClassLoader
    ClassLoader        ↓
                 WebApp ClassLoader
                       ↓
                  JSP ClassLoader
```

## **线程模型**

```java
// BIO模型
Acceptor线程 → Worker线程池(固定大小)

// NIO模型  
Acceptor线程 → Poller线程 → Worker线程池

// 线程池配置
maxThreads="200"      // 最大线程数
minSpareThreads="10"  // 最小空闲线程
acceptCount="100"     // 等待队列长度
connectionTimeout="20000" // 连接超时
```

## **连接器配置优化**

```xml
<Connector port="8080"
    protocol="org.apache.coyote.http11.Http11NioProtocol"
    maxThreads="200"
    minSpareThreads="10"
    acceptCount="100"
    maxConnections="10000"
    connectionTimeout="20000"
    compression="on"
    compressionMinSize="2048"/>
```

## **性能调优要点**

### **JVM调优**

- 堆内存设置：-Xms/-Xmx
- 垃圾回收器选择
- JVM参数优化

### **连接器调优**

- 选择合适的IO模型(NIO/NIO2)
- 调整线程池参数
- 启用压缩
- 设置合理超时时间

### **应用优化**

- Session管理优化
- 静态资源分离
- 启用缓存
- 减少日志输出

## **Filter vs Interceptor对比**

|特性|Filter过滤器|Interceptor拦截器|
|---|---|---|
|**规范**|Servlet规范|Spring框架|
|**作用范围**|所有请求(含静态资源)|Controller请求|
|**执行顺序**|web.xml配置顺序|按Order注解/配置|
|**依赖**|Servlet容器|Spring容器|
|**实现方式**|函数回调|反射(AOP)|
|**生命周期**|容器启动销毁|Spring管理|
|**使用场景**|编码、CORS、认证|权限、日志、事务|

## **常见问题排查**

- **内存溢出**：调整堆内存、检查内存泄漏
- **线程池满**：增加maxThreads、优化业务处理时间
- **连接超时**：调整connectionTimeout、检查网络
- **404错误**：检查Context配置、应用部署路径
- **类加载冲突**：检查jar包版本、类加载器顺序






Tomcat 最大支持的连接数并不是一个固定的数字，它受到**多种因素**的综合影响，包括：

* **理论最大值 (NIO/APR)**：对于 NIO 或 APR 连接器，`maxConnections` 可以设置得非常高，例如 **20000、50000 甚至 100000+**。因为单个线程可以处理多个连接，所以实际的连接数上限主要受限于操作系统的文件描述符和服务器的内存。
* **实际可支持值**：在实际生产环境中，Tomcat 通常能稳定支持**数千到数万个并发连接**（取决于应用类型、资源和配置）。但更关键的指标是**并发处理的请求数 (TPS/QPS)**，这更多地受 `maxThreads` 和应用处理速度影响。

|       | 细节                                                |
| ----- | ------------------------------------------------- |
| 应用代码层 | 业务逻辑的处理时间、数据库 redis 依赖                            |
| 配置层   |                                                   |
| 操作系统层 | 每个 socket 连接都会占用一个文件描述符。操作系统的默认限制通常是 1024 或 65535 |
| 硬件层   | CPU、内存、带宽                                         |

1. **Tomcat 配置（Connector 配置）**：这是最直接也最关键的因素。
    * **`maxConnections`**: 这是 Tomcat Connector 最重要的一个参数，它定义了服务器在任何给定时间可以接受和处理的最大连接数。**这是对总连接数（包括正在处理请求的和空闲等待的连接）的硬性限制。**
    * **`maxThreads`**: 定义了处理请求的工作线程池的最大线程数。如果 `maxThreads` 小于 `maxConnections`，那么即使有大量连接，也只能由有限的线程来处理请求。对于阻塞 I/O (BIO) 连接器，`maxThreads` 实际上限制了并发处理能力。对于非阻塞 I/O (NIO/NIO2/APR)，这个参数更多地影响并发请求处理的能力，而不是连接本身。
    * **`acceptCount`**: 连接队列的长度，当所有 `maxThreads` 都在忙碌时，新的连接会在这个队列中等待。超过这个数量的连接会被拒绝。
    * **Connector 类型**:
        * **BIO (Blocking I/O)**: 每个连接一个线程，因此 `maxThreads` 直接决定了并发连接数上限。性能较差，不推荐用于高并发。
        * **NIO (Non-blocking I/O)**: 单个线程可以处理多个连接。`maxConnections` 可以远大于 `maxThreads`。NIO 是 Tomcat 8+ 的默认连接器。
        * **NIO2 (Async I/O)**: 基于 Java 7 的 AIO，性能与 NIO 类似。
        * **APR/Native**: 使用 JNI 调用 OS 底层特性，通常在 Linux 上性能更优，尤其在处理静态文件和 HTTPS 连接时。其 `maxConnections` 和 `maxThreads` 行为与 NIO 类似。

2. **操作系统限制**：
    * **文件描述符限制 (File Descriptor Limit)**：在 Linux/Unix 系统中，每个 socket 连接都会占用一个文件描述符。操作系统的默认限制通常是 1024 或 65535。如果 Tomcat 配置的 `maxConnections` 很高，你需要相应地提高操作系统的文件描述符限制（`ulimit -n`）。
    * **TCP/IP 协议栈参数**：如 `net.ipv4.tcp_tw_reuse`、`net.ipv4.tcp_fin_timeout`、`net.ipv4.tcp_max_syn_backlog` 等，这些参数会影响 TCP 连接的建立和关闭效率。

3. **硬件资源**：
    * **CPU**: 足够的 CPU 核心数来处理请求和上下文切换。
    * **内存**: 用于线程栈、连接缓冲区、HttpSession、应用数据等。每个连接、每个线程、每个会话都会消耗内存。
    * **网络带宽**: 足够的带宽来传输数据。

4. **应用代码和业务逻辑**：
    * **请求处理时间**: 如果每个请求的处理时间很长（例如，涉及复杂计算或大量数据库查询），那么即使 `maxConnections` 设置得很高，实际的并发处理能力也会下降。
    * **后端依赖**: 数据库连接池、外部服务调用等，如果这些依赖成为瓶颈，也会限制 Tomcat 的并发能力。
    * **Session 大小**: 如果 Session 存储大量数据，会消耗更多内存。

**理论上和实际中：**

* **理论最大值 (NIO/APR)**：对于 NIO 或 APR 连接器，`maxConnections` 可以设置得非常高，例如 **20000、50000 甚至 100000+**。因为单个线程可以处理多个连接，所以实际的连接数上限主要受限于操作系统的文件描述符和服务器的内存。
* **实际可支持值**：在实际生产环境中，Tomcat 通常能稳定支持**数千到数万个并发连接**（取决于应用类型、资源和配置）。但更关键的指标是**并发处理的请求数 (TPS/QPS)**，这更多地受 `maxThreads` 和应用处理速度影响。

**如何确定合适的连接数？**

没有一个万能的数字。最佳实践是：

1. **基准测试和压力测试**：使用工具（如 JMeter, ApacheBench, Locust）模拟真实负载，逐步增加连接数和请求并发量，观察 Tomcat 的 CPU、内存、网络、响应时间、错误率等指标。
2. **监控**：在测试过程中，监控 Tomcat 内部指标（如线程池状态、连接数）、JVM 状态和操作系统资源使用情况。
3. **逐步优化**：根据测试结果，调整 `maxConnections`、`maxThreads`、`acceptCount` 以及操作系统的相关参数。

**示例配置（Tomcat `server.xml`）：**

```xml
<Connector port="8080" protocol="org.apache.coyote.http11.Http11NioProtocol"
           connectionTimeout="20000"
           redirectPort="8443"
           maxConnections="20000"  maxThreads="500"      acceptCount="100" />   ```

总之，Tomcat 支持的连接数是一个动态且可配置的指标，需要根据具体的应用场景、硬件资源和压力测试结果来确定最合理的值。
