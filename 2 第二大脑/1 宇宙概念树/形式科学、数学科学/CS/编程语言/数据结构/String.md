---
comment_id: '97093596'
date created: 2024-07-14
date modified: 2025-03-27
draw: null
title: String
---
## 认知

两者都是[[不可变的]]字符序列。任何看起来修改字符串的操作实际上都是创建并返回一个**新**的字符串对象。

在 java 中 String 类为什么要设计成 final？- 胖君的回答 - 知乎  
https://www.zhihu.com/question/31345592/answer/114126087

Strings are [immutable](不可变的.md) sequences of Unicode code points -- individual "characters" or code points (strings of length 1)

## [CheatSheet](CheatSheet.md)

好的，在您提供的表格基础上，补充“字符串转数字”和“字符串转布尔”这两项：

|                         |                                                         |                                                                                         |                                                                                                                                |
| ----------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **目的 (Purpose)**        | **Python (str) 最简方式**                                   | **Java (String) 最简方式**                                                                  | **备注 (Remarks)**                                                                                                               |
| 获取长度                    | `len(my_str)`                                           | `myString.length()`                                                                     | Python 内置函数 vs Java 方法。|
| 字符串拼接                   | `str1 + str2` / f-string / `.join()`                    | `str1 + str2` / `StringBuilder` / `StringBuffer`                                        | 两者都可用 `+`。Java 中循环内用 `+` 可能效率低 (创建过多对象)，推荐 `StringBuilder`。Python `+` 通常优化较好。|
| 访问字符 (按索引)              | `my_str[index]`                                         | `myString.charAt(index)`                                                                | Python 用下标返回单字符字符串；Java 用方法返回 `char` 原始类型。|
| 获取子串 / 切片               | `my_str[start:stop:step]`                               | `myString.substring(beginIndex, endIndex)`                                              | Python 切片更灵活 (步长/负索引)；Java `substring` 结束索引不包含在内。都返回新字符串。|
| 检查子串是否存在                | `substring in my_str`                                   | `myString.contains(substring)`                                                          | Python 用 `in` 操作符；Java 用方法。|
| 查找子串索引                  | `my_str.find(sub)` / `.index(sub)`                      | `myString.indexOf(sub)`                                                                 | Python `find` 未找到返回-1, `index` 抛异常；Java `indexOf` 未找到返回-1。都查找首次出现。|
| 替换子串                    | `my_str.replace(old, new,[count])`                      | `.replace(old, new)` / `.replaceAll(regex, new)`                                        | 两者都有字面量替换。Java 区分字面量替换 (`replace`) 和正则替换 (`replaceAll`)。Python `replace` 可选替换次数。都返回新字符串。|
| 分割字符串                   | `my_str.split(sep=None)`                                | `myString.split(regex)`                                                                 | Python `split()` 默认按空白分割；Java `split()` **默认使用正则表达式**作为分隔符，需注意转义。都返回列表/数组。|
| 连接序列为字符串                | `separator.join(iterable)`                              | `String.join(separator, iterable)` (Java 8+)                                            | 句法相反（分隔符对象 vs 静态方法）。Java 8+ 后方便很多，旧版需手动 `StringBuilder`。|
| 大小写转换                   | `.lower()`, `.upper()`, `.title()`...                   | `.toLowerCase()`, `.toUpperCase()`                                                      | 两者都有基础大小写转换。Python 提供更多内置选项。都返回新字符串。|
| 去除首尾空白                  | `.strip()`, `.lstrip()`, `.rstrip()`                    | `.trim()` / `.strip()` (Java 11+)                                                       | Python 控制更细致。Java `.trim()` 历史悠久但行为与 Python 不同 (<= U+0020)；Java 11+ 的 `.strip()` 更符合 Unicode 空白定义，并提供 `stripLeading/Trailing`。|
| 检查前缀/后缀                 | `.startswith(prefix)`, `.endswith(suffix)`              | `.startsWith(prefix)`, `.endsWith(suffix)`                                              | 方法名和功能非常相似。|
|                         |                                                         |                                                                                         |                                                                                                                                |
| **字符串转数字**              | **`int(str)`**, **`float(str)`**                        | **`Integer.parseInt(str)`**, **`Float.parseFloat(str)`**, **`Double.parseDouble(str)`** | Python 用内置函数，Java 用包装类的静态方法。格式错误时都抛出异常 (Python: `ValueError`, Java: `NumberFormatException`)。|
| **字符串转布尔**              | **`bool(str)`** (判空) / **`str.lower() == 'true'`** (语义) | **`Boolean.parseBoolean(str)`**                                                         | Python `bool()` 仅检查是否为空串 (非空即 `True`)；Java `parseBoolean` 明确检查是否为 (不区分大小写) "true"，否则结果为 `false`。语义转换 Python 需手动比较。|
| **不可变性 (Immutability)** | **是**                                                   | **是**                                                                                   | **核心特性**：对象一旦创建，其内容（字符序列）不能被更改。所有修改操作都返回新的 String/str 对象。|
|                         |                                                         |                                                                                         |                                                                                                                                |
|                         |                                                         |                                                                                         |                                                                                                                                |

## python 中的 string

```Python
def remove_suffix_ness(word):
	# ... docstring ...
	new_word = word[:-4] # 移除 'ness'，对于 "heaviness"，new_word 是 "heavi"
	if new_word[-1] == 'i': # 检查最后一个字符是否是 'i'，这里是 True
		# 尝试将最后一个字符 'i' 替换为 'y'
		new_word[-1] = 'y'  # <--- 问题在这里！
	return new_word        # 返回 new_word
```
    
- **错误原因:** Python 中的**字符串 (string) 是不可变的 (immutable)**。这意味着你**不能**直接修改字符串中的某个字符。语句 `new_word[-1]= 'y'` 试图修改字符串，这是不允许的，它实际上不会产生任何效果（或者在某些情况下会直接报错 `TypeError`）。因此，即使 `if` 条件满足，`new_word` 仍然是 `"heavi"`，函数最后返回的就是 `"heavi"`。

## java 中的 string

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

>[!最佳实践]
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
