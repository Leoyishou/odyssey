---
date created: 2024-07-13
date modified: 2025-07-10
uid: d25aa953-1173-46ce-959d-0f9b2944637a
---

Spring 源码阅读指南 - 沈世钧的文章 - 知乎  
https://zhuanlan.zhihu.com/p/51393923

您理解得非常正确。问题的本质很可能是 `@RestControllerAdvice` 的切入点晚于 JSON 解析错误发生的位置。

Spring MVC 的请求处理流程大致如下：

1. HTTP 请求到达 Servlet 容器
2. 经过各种 Servlet 过滤器
3. 到达 Spring 的 `DispatcherServlet`
4. `DispatcherServlet` 将请求委托给 `HttpMessageConverter` 来解析请求体
5. **在这一步进行 JSON 反序列化**，将 JSON 转换为 Java 对象
6. 如果转换成功，请求才会被路由到控制器方法
7. `@RestControllerAdvice` 主要作用于控制器方法执行期间抛出的异常

您遇到的问题是在第 5 步发生的 - JSON 解析错误发生在请求到达控制器之前。虽然 `@RestControllerAdvice` 理论上也可以处理这些"前置"异常，但在某些配置或情况下可能失效。

解决这个问题的方法有：

1. **使用自定义的 HttpMessageConverter**：
   您可以创建一个自定义的消息转换器，在其中捕获异常并转换为您的自定义异常。

2. **实现 HandlerExceptionResolver**：
   这是一个更底层的接口，可以捕获到更早期的异常。

3. **扩展 ResponseEntityExceptionHandler**：
   如我之前提到的，让您的异常处理器继承这个类，它专门设计用来处理这些早期的框架异常。

如果您想快速验证这个问题，可以添加一个简单的测试：

```java
@GetMapping("/test-exception")
public void testException() {
    throw new HttpMessageNotReadableException("测试异常", new InvalidFormatException(
            null, "测试消息", "s", Long.class));
}
```

如果这个手动抛出的异常能被您的异常处理器捕获，但实际的 JSON 解析错误却不能，那就证实了问题确实出在捕获的时机上。

```Java
Diff:
* changes on com.qunar.dzs.hotelsearch.polaris.domain.entity.task.PreRunTask/ :
  - 'createTime' value 'Tue Feb 25 16:25:09 CST 2025' unset
  - 'createUser' value 'yishou.liu' unset
  - 'modifiedTime' value 'Tue Feb 25 16:26:29 CST 2025' unset
  - 'status' value '1' unset


Diff:
* changes on com.qunar.dzs.hotelsearch.polaris.domain.entity.task.PreRunTask/ :
  - 'basicGroupsTick' collection changes :
     1. 'com.qunar.dzs.hotelsearch.polaris.domain.entity.task.PreRunTask/#basicGroupsTick/5' added
     2. 'com.qunar.dzs.hotelsearch.polaris.domain.entity.task.PreRunTask/#basicGroupsTick/6' added
  - 'basicGroupsTick/5.canPreRun' = 'true'
  - 'basicGroupsTick/5.id' = '88'
  - 'basicGroupsTick/5.useModified' = 'true'
  - 'basicGroupsTick/6.canPreRun' = 'true'
  - 'basicGroupsTick/6.id' = '87'
  - 'basicGroupsTick/6.useModified' = 'true'
  - 'basicGroupsTickPriority' collection changes :
     1. '88' added
     2. '87' added
  - 'basicGroupsTickPriorityAcked' value 'true' unset
  - 'createTime' value 'Tue Feb 25 16:25:09 CST 2025' unset
  - 'createUser' value 'yishou.liu' unset
  - 'diffGroupsTickPriorityAcked' value 'true' unset
  - 'modifiedTime' value 'Tue Feb 25 16:25:20 CST 2025' unset
  - 'status' value '1' unset

```
