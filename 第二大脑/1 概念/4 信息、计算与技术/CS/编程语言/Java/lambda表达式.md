---
date created: 2024-08-15
date modified: 2025-09-15
uid: e1f7b507-ca4f-4738-bc56-bd3debf7414c
---
# 本质

匿名内部类的简写方式

# 对象创建和垃圾回收

每个 Lambda 表达式都会创建一个实现函数接口的对象。频繁使用 Lambda 表达式可能导致大量对象的创建和销毁，从而增加垃圾回收的压力。这在高性能场景下可能成为一个性能瓶颈，因此在性能敏感的代码中应谨慎使用。

>[!最佳实践]

- **避免不必要的闭包使用**：尽量减少捕获外部变量的数量。
- **使用并行流**：在处理集合时，可以使用并行流来提高性能。
- **减少对象创建**：在性能敏感的代码路径中，考虑使用传统的循环或方法调用来替代 Lambda 表达式

# Notes

Lambda 表达式详解
1. 完整的 Lambda 语法
```java
// 完整写法
StreamingResponseBody responseBody = (OutputStream outputStream) -> {
    // 方法体
};

// 简化写法（类型推断）
StreamingResponseBody responseBody = outputStream -> {
    // 方法体
};
```

2. 等价于传统的[[匿名内部类]]
```java
// Lambda 表达式
StreamingResponseBody responseBody = outputStream -> {
    processMessageWithCallback(request.getMessage(), new EventCallback() {
        // ...
    });
};

// 等价的匿名内部类
StreamingResponseBody responseBody = new StreamingResponseBody() {
    @Override
    public void writeTo(OutputStream outputStream) throws IOException {
        processMessageWithCallback(request.getMessage(), new EventCallback() {
            // ...
        });
    }
};
```
