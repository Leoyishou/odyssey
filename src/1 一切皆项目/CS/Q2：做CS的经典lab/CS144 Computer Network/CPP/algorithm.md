---
draw:
tags: []
title: algorithm
date created: 2024-12-31
date modified: 2024-12-31
---

`<algorithm>` 是 C++ 标准库中的一个头文件，提供了大量有用的算法函数。以下是一些常用的算法：

```cpp
#include <algorithm>

// 1. min/max 相关
std::min(a, b);              // 返回较小值
std::max(a, b);              // 返回较大值
std::min({1, 2, 3, 4});      // 返回集合中最小值
std::max({1, 2, 3, 4});      // 返回集合中最大值

// 2. 查找相关
std::find(begin, end, value);           // 查找元素
std::find_if(begin, end, predicate);    // 条件查找
std::binary_search(begin, end, value);  // 二分查找

// 3. 排序相关
std::sort(begin, end);                  // 排序
std::reverse(begin, end);               // 反转

// 4. 复制和填充
std::copy(src_begin, src_end, dest);    // 复制
std::fill(begin, end, value);           // 填充

// 5. 计数和比较
std::count(begin, end, value);          // 计数
std::equal(begin1, end1, begin2);       // 比较
```

在 ByteStream 的实现中，可能会用到：

```cpp
// 例如在 write 函数中限制写入长度
size_t ByteStream::write(const string &data) {
    // 计算可写入的字节数
    size_t len = std::min(data.length(), remaining_capacity());
    // ...
}

// 或在 read 函数中
string ByteStream::read(const size_t len) {
    // 计算实际可读取的字节数
    size_t actual_len = std::min(len, buffer_size());
    // ...
}
```

相当于 Java 中的：

```java
// Java 等价功能
import java.util.Collections;  // 对应一部分算法
import java.util.Arrays;      // 对应另一部分算法

// 使用示例
Math.min(a, b);              // 对应 std::min
Collections.min(list);       // 对应 std::min 的集合版本
Arrays.sort(array);         // 对应 std::sort
Collections.reverse(list);  // 对应 std::reverse
```
