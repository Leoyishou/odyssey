---
draw:
tags: []
title: String
date created: 2024-07-14
date modified: 2024-12-27
---

在 java 中 String 类为什么要设计成 final？- 胖君的回答 - 知乎  
https://www.zhihu.com/question/31345592/answer/114126087

com.google.common.base.Strings#nullToEmpty

## [不可变的](不可变的.md)

下面这个 n 次的循环，时间复杂度其实是 n 方

```java
public String statement() {
	String result = "";
	for (int i = 0; i < numItems(); i++)
		result += lineForItem(i); // String concatenation
		return result;
}
```

1. Java中的字符串是不可变的（immutable）。
2. 每次使用+操作符拼接字符串时，实际上会创建一个新的字符串对象。

步骤如下：

1. S1 + S2: 需要挪动长度为m的S1和长度为m的S2，总共2m个字符。
2. (S1 + S2) + S3: 需要挪动长度为2m的(S1 + S2)和长度为m的S3，总共3m个字符。
3. ((S1 + S2) + S3) + S4: 需要挪动长度为3m的((S1 + S2) + S3)和长度为m的S4，总共4m个字符。... n. 最后一步需要复制长度为(n-1)m的中间结果和长度为m的Sn，总共nm个字符。

$$
\sum_{i=1}^{n} im = m\sum_{i=1}^{n} i
$$

把 n 次做 sum，那么求和之后就是

$$
m\sum_{i=1}^{n} i = m \cdot \frac{n(n+1)}{2} \approx \frac{mn^2}{2}
$$

> [!最佳实践]
> 所以不要在 for 循环中用`+`来连接字符串

- StringBuffer线程安全，但是性能较低
- StringBuilder线程不安全，但是只有多个线程共用一个实例时才会产生
- 就像 HashTable 也是线程安全但是我们一般不用一样，因为性能低

```java
public class StringBuilderThreadUnsafeDemo {
    public static void main(String[] args) throws InterruptedException {
        StringBuilder sharedStringBuilder = new StringBuilder();
        
        Runnable task = () -> {
            for (int i = 0; i < 1000; i++) {
                sharedStringBuilder.append("a");
            }
        };
        
        Thread thread1 = new Thread(task);
        Thread thread2 = new Thread(task);
        
        thread1.start();
        thread2.start();
        
        thread1.join();
        thread2.join();
        
        System.out.println("StringBuilder长度: " + sharedStringBuilder.length());
        System.out.println("预期长度: 2000");
    }
}```
