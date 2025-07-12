---
date created: 2024-08-19
date modified: 2025-07-10
uid: e91c8480-263c-49a3-8c4d-ec1268bd909a
---
- uint8是8位（1字节），范围是0到255。是无符号的，只能表示非负数。
- int64是64位（8字节），范围是大约-9.2千万亿到+9.2千万亿。是有符号的，可以表示负数、零和正数。

好的，我们来看一下这五种语言中“数字（number）”类型所属的范畴和具体实现：

|                |                                                                                                                                                                                                |                                                                                                             |                                                                                                                                                                               |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **语言**         | **主要数字类型**                                                                                                                                                                                     | **范畴/本质**                                                                                                   | **关键特点**                                                                                                                                                                      |
| **Python**     | `int` (整数)<br>`float` (浮点数)<br>`complex` (复数)<br>*(还有 `Decimal`, `Fraction` 等模块类型)*                                                                                                            | **对象 (Objects)**<br>所有数字类型都是类 (`int`, `float`, `complex` 是内置类)。| `int` 支持**任意精度**，仅受内存限制。<br>`float` 通常是 64 位 IEEE 754 双精度。<br>类型之间在运算时会有一定的自动转换（例如 `int` 和 `float` 运算结果是 `float`）。|
| **Java**       | `byte`, `short`, `int`, `long` (整数)<br>`float`, `double` (浮点数)<br>*(还有 `BigInteger`, `BigDecimal` 类)*                                                                                          | **原始类型 (Primitive Types)**<br>这些是语言内置的[[基本数据类型]]，不是对象。<br>每个原始类型都有对应的**包装类** (`Integer`, `Double` 等)，它们是对象。| 整数类型有**固定的大小和范围** (`int` 是 32 位, `long` 是 64 位等)。<br>`float` 是 32 位，`double` 是 64 位 IEEE 754。<br>原始类型和包装类之间有自动装箱/拆箱机制。<br>`BigInteger`/`BigDecimal` 类用于任意精度计算。|
| **Go**         | `int8`, `int16`, `int32`, `int64`, `int`<br>`uint8` (`byte`), `uint16`, `uint32`, `uint64`, `uint`<br>`float32`, `float64`<br>`complex64`, `complex128`                                        | **基础类型 (Basic Types)**<br>这些是语言预定义的类型，表现为值类型，不是对象。| Go 语言是强类型语言，不允许不同类型的变量直接进行算术运算。<br>强制显式指定大小 (`int`/`uint` 大小依赖架构)。<br>不同数字类型间**不允许隐式转换**，必须显式转换 (如 `int64(myInt32)`)。<br>`float32`/`float64` 是 IEEE 754 标准。<br>整数类型有固定的大小和范围。|
| **C++**        | `short`, `int`, `long`, `long long` (有符号整数)<br>`unsigned short`, `unsigned int`, `unsigned long`, `unsigned long long` (无符号整数)<br>`float`, `double`, `long double` (浮点数)<br>*(还有 `char` 及其变体)* | **基本类型 (Fundamental Types)**<br>它们是语言内置的类型，不是类/对象。| 整数类型的确切大小**依赖于具体实现 (编译器/平台)**，但标准规定了最小范围。<br>浮点数通常遵循 IEEE 754，`long double` 精度可能更高，但也依赖实现。<br>存在较多的**隐式数值转换和提升**规则。|
| **JavaScript** | `number` (数字)<br>`bigint` (大整数)                                                                                                                                                                | **原始类型 (Primitive Types)**<br>`number` 和 `bigint` 都是原始数据类型。有对应的 `Number` 和 `BigInt` 包装对象（较少直接使用）。| **`number` 类型是唯一的**：它使用 64 位 IEEE 754 双精度表示**所有数字**，包括整数和浮点数。这意味着整数的安全表示范围有限 (到 `2^53 - 1`)。<br>`bigint` 用于表示**任意精度整数**，通过在数字后加 `n` (如 `10n`) 创建，不能与 `number` 直接混合运算。|

**总结来说：**

- **Python:** 将数字视为对象，整数精度是动态的。
- **Java:** 严格区分原始数字类型（固定大小）和用于任意精度计算的对象类。
- **Go:** 提供多种固定大小的基础数字类型，强调类型安全，无隐式转换。
- **C++:** 提供多种基本数字类型，其大小可能因平台而异，存在隐式转换规则。
- **JavaScript:** 主要使用单一的 `number` 类型（双精度浮点数）表示所有数字，导致整数精度有限；引入 `bigint` 解决大整数问题。

Nu

Numbers

in

JavaScript

1 exercise

About Numbers

Many programming languages have specific numeric types to represent different types of numbers, but JavaScript only has two:

  

number: a numeric data type in the double-precision 64-bit floating-point format (IEEE 754). Examples are -6, -2.4, 0, 0.1, 1, 3.14, 16.984025, 25, 976, 1024.0 and 500000.

bigint: a numeric data type that can represent integers in the arbitrary precision format. Examples are -12n, 0n, 4n, and 9007199254740991n.

If you require arbitrary precision or work with extremely large numbers, use the bigint type. Otherwise, the number type is likely the better option.

  

Rounding

There is a built-in global object called Math that provides various rounding functions. For example, you can round down (floor) or round up (ceil) decimal numbers to nearest whole numbers.

  

Math.floor(234.34); // => 234

Math.ceil(234.34); // => 235

Learn More

MDN: Number (Built-in object)

(external link)

MDN: Math (Built-in object)

(external link)

MDN: Lexical grammar for numeric literals

(external link)

MDN: Equality comparisons and sameness
