---
comment_id: 92a49914
date created: 2025-05-26
date modified: 2025-05-26
draw: null
title: 八股-Tomcat
---
[[八股-Tomcat]]/✅Tomcat与Web服务器（如Apache）之间的关系是什么？.md  
Tomcat/✅Tomcat中有哪些类加载器_.md  
Tomcat/✅Tomcat处理请求的过程是怎么样的？.md  
Tomcat/✅Tomcat的启动流程是怎样的？.md  
Tomcat/✅Tomcat的类加载机制是怎么样的？.md  
Tomcat/✅为什么Tomcat可以把线程数设置为200，而不是N+1？.md  
Tomcat/✅介绍一下Tomcat的IO模型？.md  
Tomcat/✅过滤器和拦截器的区别是什么？.md  

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
