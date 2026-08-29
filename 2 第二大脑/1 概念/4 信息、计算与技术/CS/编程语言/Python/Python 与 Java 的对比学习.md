---
date created: 2024-04-21
date modified: 2025-07-10
uid: 00b5c5f7-ccb8-47ff-8ede-7f2658c27e0e
---

![905063d6b84ffbcb1813ce99c82eea3c.png|400](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F04%2F21%2F15-45-05-d47b1fe071104587f73c8abde6155188-905063d6b84ffbcb1813ce99c82eea3c-3222f7.png?x-oss-process=image/resize,l_400)

> Life is short, you need Python  
>——Bruce Eckel

<!-- more -->

## 解释器与虚拟机

[都是解释执行，为什么跑java的叫虚拟机，而python ruby等只能叫解释器?](https://www.zhihu.com/question/647430973/answer/3440440306)？

[lua](https://www.zhihu.com/search?q=lua&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3440440306%7D) 也是脚本语言，人家 lua 的就叫虚拟机。

区别不是什么解释执行的问题，而是概念本身就很清晰明确。虚拟机，它为什么叫虚拟机？这才是你应该思考的问题。虚拟机，它本身就是对物理机的一个虚拟，就是在模拟 CPU 指令和执行的过程，虚拟机一般分为 [寄存器](https://www.zhihu.com/search?q=%E5%AF%84%E5%AD%98%E5%99%A8&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3440440306%7D) 虚拟机和栈虚拟机，这也是对物理机的模拟。Lua 就是寄存器虚拟机，Java 是栈虚拟机，一个是模拟寄存器操作，一个是模拟栈操作。

如果你运行时没有这一套机制，模仿指令和指令操作，你凭什么叫虚拟机？

一般的解释器，它没有这套机制，是直接执行 [ast](https://www.zhihu.com/search?q=ast&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3440440306%7D)，称作解释器，是实至名归的。

当然了，国外教材写得好的地方，就在于人家把前因后果，来龙去脉解释得清清楚楚。Python 和 [Ruby](https://www.zhihu.com/search?q=Ruby&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3440440306%7D) 一开始就是我上面说的，直接解释执行 ast，所以它确实就是解释器，这个命名没问题。但是随着时间发展，[解释器](https://www.zhihu.com/search?q=%E8%A7%A3%E9%87%8A%E5%99%A8&search_source=Entity&hybrid_search_source=Entity&hybrid_search_extra=%7B%22sourceType%22%3A%22answer%22%2C%22sourceId%22%3A3440440306%7D) 性能太差了，越来越不满足性能要求，为了提升性能，它们最终都发展成了虚拟机，其实内部早就引入虚拟机那一套机制，虚拟机指令和指令操作。但是他们内部命名没有改，还是沿用了之前的名字，称作解释器。

所以结论就是，Python 这些只是挂着解释器名称的虚拟机

## 基础语法

### 代码块和控制结构的表示

Java 使用大括号（{}）来表示代码块，例如：

```java
for (int i = 0; i < 5; i++) {
    System.out.println("Java");
}
```

Python 使用缩进来表示代码块，通常是四个空格，例如：

```python
for i in range(5):
    print("Python")
```

#### 条件语句

Java 中的条件语句使用 if-else 结构，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```

Python 中的条件语句使用 if-else 结构，但没有括号，并且使用缩进来表示代码块，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```

#### 循环结构

Java 中的常见循环结构是 for 循环和 while 循环，例如：

```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```

Python 中的常见循环结构是 for 循环和 while 循环，例如：

```python
for i in range(5):
    print(i)

i = 0
while i < 5:
    print(i)
    i += 1
```

### 变量和类型声明

Java 是一种静态类型语言，需要在变量声明时指定其类型，例如：

```java
int num = 10;
String name = "Java";
```

Python 是一种动态类型语言，变量的类型可以根据赋值自动推断，例如：

```python
num = 10
name = "Python"
```

#### 类型注解

在 Java 中，可以使用类型注解来提供额外的类型信息，以帮助编译器进行类型检查和类型推断，例如：

```java
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<@NonNull String> names = new ArrayList<>();
        names.add("John");
        names.add(null);  // 编译器会报错，不允许添加null值
    }
}
```

在 Python 中，从 Python 3.5 开始引入了类型注解的支持，可以使用类型注解来提供变量和函数的类型信息，例如：

```python
from typing import List

def greet(name: str) -> str:
    return "Hello, " + name

names: List[str] = ["John", "Jane"]
```

类型注解是可选的，不是强制要求的。在 Python 中，仍然可以编写没有类型注解的代码。但是，类型注解可以提供更好的代码可读性和可维护性，特别是在大型项目或多人协作的情况下。

类型注解可以使用 Python 3.5 引入的函数注解语法来实现，也可以使用类型注解库如 typing 库来提供更丰富的类型注解功能。以下是一个使用类型注解的示例：

```python
for i in range(5):
    print("Python")
```0

### 函数

#### 定义

Java 中定义函数需要使用关键字 void（表示无返回值）或指定返回类型，并使用 return 语句返回值，例如：

```python
for i in range(5):
    print("Python")
```1

Python 中定义函数使用关键字 def，不需要指定返回类型，并使用 return 语句返回值，例如：

```python
for i in range(5):
    print("Python")
```2

#### 静态方法和类方法

在 Java 中，可以使用 static 关键字定义静态方法和静态变量，静态方法可以直接通过类名调用，例如：

```python
for i in range(5):
    print("Python")
```3

在 Python 中，可以使用@staticmethod 装饰器定义静态方法，静态方法可以通过类名或实例调用，例如：

```python
for i in range(5):
    print("Python")
```4

#### 默认参数

在 Java 中，方法的参数没有默认值，每次调用方法都需要显式传递参数，例如：

```python
for i in range(5):
    print("Python")
```5

在 Python 中，方法的参数可以指定默认值，如果调用方法时没有传递对应参数，则使用默认值，例如：

```python
for i in range(5):
    print("Python")
```6

#### 函数调用和参数传递

##### 值传递方式

Java 中的参数传递方式是值传递，也就是说，当你将一个参数传递给一个函数时，函数会得到一个参数的副本，而不是参数本身。这意味着在函数内部修改参数的值不会影响传递给函数的原始值。

Python 中的参数传递方式是引用传递，也就是说，当你将一个参数传递给一个函数时，函数会得到一个指向参数的引用，而不是参数本身。这意味着在函数内部修改参数的值会影响传递给函数的原始值。

例如，在 Java 中，以下代码将打印出 "10"：

```python
for i in range(5):
    print("Python")
```7

而在 Python 中，以下代码将打印出 "20"：

```python
for i in range(5):
    print("Python")
```8

##### 对象传递方式

在 Java 中，对象是通过引用传递的，即传递的是对象的引用而不是对象本身，例如：

```python
for i in range(5):
    print("Python")
```9

在 Python 中，对象的传递方式是值传递，但对于可变对象（如列表、字典等），传递的是对象的引用，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```0

#### 匿名函数

在 Java 中，可以使用接口和匿名类来实现类似于匿名函数的功能，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```1

在 Python 中，可以使用 Lambda 表达式来创建匿名函数，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```2

#### 函数式编程

在 Java 中，函数式编程的支持较为有限，但可以使用 Lambda 表达式和 Stream API 来实现一些函数式编程的特性，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```3

在 Python 中，函数式编程是一种核心特性，支持函数作为一等公民，可以使用 Lambda 表达式、高阶函数和内置函数等来实现函数式编程的特性，例如：
使用 Lambda 表达式和内置函数进行函数式编程

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```4

### 数据结构

#### 基本数据类型

Java 中有 byte,short,int,long,float,double,是按照占用空间和空间存储来严格划分  
python 中就 int 和 float,代表一切了,这有点像 js 中的风格,业内称为弱引用

#### 类型

在 Python 中，内置了列表（List）、集合（Set）和字典（Dictionary）等集合类型，可以直接使用，例如：
列表 - Java 中的数组、列表  
元组 - Java 中的列表，但是是[不可变的](不可变的.md)的  
字典 - Java 中的 map  
set - Java 中的 set

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```5

#### 数组和列表

Java 中的数组是固定长度的，需要在声明时指定大小，并且只能存储相同类型的元素，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```6

在 Python 中，列表是动态长度的数据结构，可以根据需要动态添加或删除元素，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```7

列表切片  
在 Java 中，可以使用 List.subList() 方法来获取子列表，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```8

在 Python 中，可以使用切片（Slice）操作来获取子列表，例如：

```java
int num = 10;
if (num > 0) {
    System.out.println("Positive");
} else {
    System.out.println("Negative");
}
```9

#### 字典操作

在 Java 中，可以使用 Map 接口和相关的实现类来操作字典，例如获取、添加、删除和遍历字典的操作比较繁琐。

在 Python 中，字典的操作更加简洁和方便，可以直接使用字典的方法进行操作，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```0

#### 迭代器

在 Java 中，可以使用迭代器（Iterator）来遍历集合（如 List、Set 等），例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```1

在 Python 中，可以使用迭代器和生成器来遍历可迭代对象，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```2

#### 生成器

在 Python 中，可以使用生成器（Generator）来实现迭代器，使用 yield 关键字定义生成器函数，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```3

#### 列表推导式和集合推导

在 Java 中，没有直接的语法来创建列表或集合的推导式，需要使用循环和条件语句来实现。

在 Python 中，可以使用列表推导式和集合推导式来快速创建列表或集合，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```4

### 字符串操作

Java 中的字符串是不可变的，使用 + 操作符进行字符串拼接，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```5

Python 中的字符串是可变的，可以使用 + 操作符或 +=运算符进行字符串拼接，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```6

### 模块和导入

在 Java 中，代码通常组织在包（package）中，需要使用 import 语句导入其他包或类，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```7

在 Python 中，代码通常组织在模块（module）中，使用 import 语句导入其他模块或函数，例如：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```8

在 Python 中，```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```5 是一个目录（directory），但为了让 Python 解释器能够识别它并从中导入模块，它也被组织成一个 Python 包（package）。要将一个目录转换为 Python 包，你需要在该目录中包含一个名为 ```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```6 的文件。这个文件可以是空的，它的存在标志着该目录是一个 Python 包，这样你就可以使用类似 ```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```7 这样的导入语句。

因此，虽然 ```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```8 是一个目录，添加 ```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```9 文件后，它也起到了 Python 包的作用，使得目录内的所有模块都可以按照包的结构进行导入和组织。这种做法常见于 Python 项目中，特别是那些需要组织大量代码和模块的大型项目。这样的结构有助于维护测试代码的清晰和可管理性，尤其是当项目规模扩大，测试范围增加时。

### 特殊方法（魔术方法）

在 Java 中，没有像 Python 中的特殊方法（也称为魔术方法），这些方法以双下划线开头和结尾，用于实现对象的特定行为（如迭代、比较、运算符重载等）。

在 Python 中，特殊方法允许类自定义对象的行为，例如 **init**() 用于初始化对象，**str**() 用于返回对象的字符串表示等。

例如，在 Python 中定义一个类并重写特殊方法的示例：

```python
num = 10
if num > 0:
    print("Positive")
else:
    print("Negative")
```9

在 Python 中，以下是一些常见的以 __ 开头和以 __ 结尾的特殊方法：
**init**()：对象初始化方法，在创建对象时自动调用。
**str**()：返回对象的字符串表示，可以通过 str() 函数或 print() 函数调用。
**repr**()：返回对象的可打印字符串表示，可以通过 repr() 函数调用。
**len**()：返回对象的长度，可以通过 len() 函数调用。
**getitem**()：通过索引获取对象的元素，可以通过[]运算符调用。
**setitem**()：通过索引设置对象的元素，可以通过[]运算符调用。
**delitem**()：通过索引删除对象的元素，可以通过 del 关键字和[]运算符调用。
**iter**()：返回一个迭代器对象，用于支持对象的迭代。
**next**()：返回迭代器的下一个元素，用于支持对象的迭代。
**enter**() 和 **exit**()：用于定义上下文管理器，管理资源的获取和释放。
**call**()：使对象可以像函数一样被调用。

### 注释

在 Java 中，注释有单行注释（//）和多行注释（/*... */），例如：

```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```0

在 Python 中，注释使用井号（#、'''、"""），例如：

```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```1

### 异常类型

在 Java 中，异常类型需要在方法签名中声明或捕获，例如：

```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```2

在 Python 中，异常类型不需要在方法签名中声明，可以在 except 块中捕获特定的异常类型，例如：

```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```3

#### Python 常见的异常

ZeroDivisionError：除（或取模）零异常  
IndexError：序列中没有此索引  
KeyError：映射中没有这个键  
NameError：未声明/初始化对象（没有属性）
SyntaxError：Python 语法错误  
ValueError：传入无效的参数

#### Traceback 模块

打印异常信息：traceback.pringt_exc()

## IO

python 的比较简单

```java
for (int i = 0; i < 5; i++) {
    System.out.println(i);
}

int i = 0;
while (i < 5) {
    System.out.println(i);
    i++;
}
```4

| r  | 打开文件以进行读取（默认）。如果文件不存在，抛出异常。|
|----|--------------------------------------------------|
| w  | 打开文件以进行写入，如果文件存在则将其截断至零长度，如果文件不存在则创建新文件。|
| a  | 打开文件以进行追加；任何写入的数据将自动添加到文件的末尾。如果文件不存在，则创建新文件进行写入。|
| r+ | 打开文件进行读取和写入。文件指针放在文件的开头。如果文件不存在，抛出异常。|
| w+ | 打开文件进行读取和写入，相当于 w 模式，但也允许读取操作。|
| a+ | 打开文件进行读取和追加；与 a 相似，但文件打开后可进行读取。如果文件不存在，则创建新文件。|
| x  | 专为创建新文件而设计，如果文件已存在，返回 FileExistsError。|
| b  | 以二进制模式打开文件，而不是文本模式。此模式可以添加到其他模式中，如 rb、wb 或 ab+。|
| t  | 以文本模式打开文件（默认）。此模式可以添加到其他模式中，如 rt 或 xt。|

## 参考资料

              

原文链接：https://blog.csdn.net/qq_42440234/article/details/133107955
