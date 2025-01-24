---
draw:
tags: []
title: Controller 层
date created: 2025-01-21
date modified: 2025-01-22
---

## 1. 统一异常处理

### 1.1 用切点或者注解

#### 切点

```plaintext
com.qunar.dzs.hotelsearch.polaris.interfaces.web.*Controller
```

#### 注解

在这个 `UnifiedExceptionHandler` 类中实际上并没有定义切点（Pointcut）。这是因为这个类使用的是 Spring 的 `@RestControllerAdvice` 注解，它是一个特殊的注解，不需要显式定义切点。

`@RestControllerAdvice` 是 Spring AOP 的一个特殊应用场景，它会自动处理所有 Controller 中抛出的异常。它的工作原理是：

1. `@RestControllerAdvice` 会自动对所有使用了 `@RestController` 或 `@Controller` 注解的类进行织入。
2. 通过 `@ExceptionHandler` 注解来指定具体要处理哪些异常类型。

### 1.2 那有了这样的切面，我自己实际写的 Controller 里还需要 try-catch 吗

在 Controller 层通常**不需要**写 try-catch，因为有了 `@RestControllerAdvice` 的统一异常处理。
总之，有了统一异常处理后，Controller 层应该专注于业务逻辑，把异常处理的责任交给 `@RestControllerAdvice`。只在确实需要特殊处理的场景下才使用 try-catch。

### 1.3 如果不 catch 的话，如何在异常发生时打印出外部参数呢

实际上，你的项目中已经通过 AOP 实现了参数打印的功能。让我们看一下之前的 `AopControllerLog` 类：

```java
log.info("用户:{} 请求的类：{} 请求的方法：{} 请求参数：{} 返回值：{} 响应时间： {} ",
         RequestHolder.getCurrentUser(),
         target,
         methodName,
         paramJson,
         JsonUtils.toJson(result),
         time);
```

---

## 2. 从 HTTP 报文中获取外部输入参数

### 2.1 怎么取

- **`@RequestBody` + JSON**  
    `@RequestBody` 的作用是用来**将请求体（Request Body）中的内容自动反序列化**为 Java 对象，并将其传给 Controller 方法的参数。通常用于接收 **JSON** 或 **XML** 等格式的数据，尤其在 RESTful 接口中，比较常见的用法是接收前端以 `application/json` 格式发送的请求。
    通过 `@RequestBody` 注解来告知 Spring："请从 HTTP Body 中读取并解析 JSON 并映射到我的参数对象上"。
    
    - **前端**：`POST /upsertTask`，请求头 `Content-Type: application/json`；请求体：`{ "id": 123, "name": "xx" }`。
    - **后端**：

        ```java
        @PostMapping("/upsertTask")
        public ApiResult<Long> upsertTask(@RequestBody PreRunTaskDetailVo taskDetail) {
            ...
        }
        ```

        这种写法可以让 `taskDetail` 自动绑定到 JSON 里的字段。

- **`@RequestParam`**  
    常见于处理简单的 `key=value` 请求参数（可放在 QueryString 或 form-data）：`?id=123&name=xxx`。
    
    - **后端例子**：

        ```java
        public ApiResult<Long> startTask(@RequestParam Long taskId) {
            ...
        }
        ```

- **`@PathVariable`**  
    一般用于 RESTful 风格的路径参数，如 `GET /user/123`，获取 `id = 123`。
    
    - **后端例子**：

        ```java
        @GetMapping("/user/{id}")
        public User getUserById(@PathVariable("id") Long id) {
            ...
        }
        ```

- **`@RequestHeader`**  
    用于获取请求头中的数据。
    

### 2.2 对应什么 Java 类型

基于你的示例，一般实践是：

- **分页场景**（`curPage`, `pageSize`）：用 `Integer` 更灵活，然后在代码或 `PageUtils` 里判空并填默认值。
- **必填业务主键**（`taskId`）：如果**一定要有**，可以直接用 `long`，并且在调用 `preRunService.startTask(taskId)` 时由框架/校验机制保证不会传 null（否则会 400 Bad Request）；或者用 `Long` 并结合 `@NotNull` 等校验注解，校验失败就返回 400。
- **tabType**：如果它是一个枚举或标识类型，而且也必须要有，那么用 `int`（或 `Integer` + 判空）都行；如果它也可以缺省，那就使用 `Integer` 并做判空处理。

**综上**，更通用的做法是：  
**外部输入参数大多数情况下都用包装类型**，因为这样可以显式处理 "没传/为 null" 的情况，让你的控制层入参语义更清晰、校验更灵活。如果某个参数 100% 必须有值且不允许为 null，可以用基本类型，并配合框架的必填校验或注解说明。

改写你的 `getTabVo` 用包装类型的例子：

```java
@PostMapping("/getTabData")
public ApiResult<PageDataVo<PreRunTabVo>> getTabVo(Long taskId,
                                                   Integer tabType,
                                                   Integer curPage,
                                                   Integer pageSize) {
    // 如果是可选参数，这里判空并给默认值
    if (tabType == null) {
        tabType = 1; // 默认 tab
    }
    if (curPage == null) {
        curPage = 1;
    }
    if (pageSize == null) {
        pageSize = 20;
    }
    // 其余逻辑 ...
    return null;
}
```

或者在你的 `PreRunTaskParam` 对象里用包装类型 `Integer curPage; Integer pageSize;`，同时在 Service 层或工具类里做默认值处理。

1. **包装类型**（`Integer`, `Long`）在 Controller 层更常见，能处理 "null == 未填写" 且能配合注解校验，灵活度更高。
2. **基本类型**（`int`, `long`）不支持 `null`，需要确保每个参数都传入并有意义，否则会出现绑定错误。
3. 选用哪一种，主要是看**参数是否允许为空**、**业务语义是否需要区分未传参**、以及**团队的统一习惯**或**使用注解默认值**的方式。
4. 性能差别对大多数 Web 应用来说并不显著，可忽略不计，更多是语义和可维护性优先。

---

以上即为在**尽量保留原文内容**的基础上，对文档排版、结构及可读性所做的整理示例。希望对你有所帮助。
