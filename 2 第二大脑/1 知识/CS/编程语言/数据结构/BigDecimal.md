---
draw:
title: BigDecimal
date created: 2024-07-16
date modified: 2025-03-27
---

ROUND_HALF_UP 四舍五入

compareTo

## int、float、double 转 BigDecimal

不要直接用 new 方法，而是用 value of

```java
public static void main(String[] args) {  
    double a = 0.05;  
    System.out.println(new BigDecimal(a));  
    System.out.println(BigDecimal.valueOf(a));  
}
```

## 典型回答

有区别，而且区别很大。

因为double是不精确的，所以使用一个不精确的数字来创建BigDecimal，得到的数字也是不精确的。如0.1这个数字，double只能表示他的近似值。

所以，**当我们使用new BigDecimal(0.1)创建一个BigDecimal 的时候，其实创建出来的值并不是正好等于0.1的。**

而是0.1000000000000000055511151231257827021181583404541015625。这是因为double自身表示的只是一个近似值。

而对于BigDecimal(String)，当我们使用new BigDecimal("0.1")创建一个BigDecimal 的时候，其实创建出来的值正好就是等于0.1的。

那么他的标度也就是1

## 扩展知识

在《阿里巴巴Java开发手册》中有一条建议，或者说是要求：

### BigDecimal如何精确计数？

如果大家看过BigDecimal的源码，其实可以发现，**实际上一个BigDecimal是通过一个"无标度值"和一个"标度"来表示一个数的。**

> **无标度值（Unscaled Value）**：这是一个整数，表示BigDecimal的实际数值。
>
> **标度（Scale）**：这是一个整数，表示小数点后的位数。
>
> BigDecimal的实际数值计算公式为：unscaledValue × 10^(-scale)。

假设有一个BigDecimal表示的数值是123.45，那么无标度值（Unscaled Value）是12345。标度（Scale）是2。因为123.45 = 12345 × 10^(-2)。

涉及到的字段就是这几个：

```java
public class BigDecimal extends Number implements Comparable<BigDecimal> {
    private final BigInteger intVal;
    private final int scale; 
    private final transient long intCompact;
}
```

关于无标度值的压缩机制大家了解即可，不是本文的重点，大家只需要知道BigDecimal主要是通过一个无标度值和标度来表示的就行了。

**那么标度到底是什么呢？**

除了scale这个字段，在BigDecimal中还提供了scale()方法，用来返回这个BigDecimal的标度。

```java
/**
 * Returns the <i>scale</i> of this {@code BigDecimal}.  If zero
 * or positive, the scale is the number of digits to the right of
 * the decimal point.  If negative, the unscaled value of the
 * number is multiplied by ten to the power of the negation of the
 * scale.  For example, a scale of {@code -3} means the unscaled
 * value is multiplied by 1000.
 *
 * @return the scale of this {@code BigDecimal}.
 */
public int scale() {
    return scale;
}
```

那么，scale到底表示的是什么，其实上面的注释已经说的很清楚了。

> 当标度为正数时，它表示小数点后的位数。例如，在数字123.45中，他的无标度值为12345，标度是2。
>
> 当标度为零时，BigDecimal表示一个整数。
>
> 当标度为负数时，它表示小数点向左移动的位数，相当于将数字乘以 10 的绝对值的次方。例如，一个数值为1234500，那么他可以用value是12345，scale为-2来表示，因为1234500 * 10^(-2) = 12345。（当需要处理非常大的整数时，可以使用负数的标度来指定小数点左侧的位数。这在需要保持整数的精度而又不想丢失尾部零位时很有用。）

**而二进制无法表示的0.1，使用BigDecimal就可以表示了，及通过无标度值1和标度1来表示。**

我们都知道，想要创建一个对象，需要使用该类的构造方法，在BigDecimal中一共有以下4个构造方法：

```java
BigDecimal(int)
BigDecimal(double) 
BigDecimal(long) 
BigDecimal(String)
```

以上四个方法，创建出来的BigDecimal的标度（scale）是不同的。

其中 BigDecimal(int)和BigDecimal(long) 比较简单，因为都是整数，所以他们的标度都是0。

而BigDecimal(double) 和BigDecimal(String)的标度就有很多学问了。

### BigDecimal(double)有什么问题

BigDecimal中提供了一个通过double创建BigDecimal的方法——BigDecimal(double)，但是，同时也给我们留了一个坑！

因为我们知道，double表示的小数是不精确的，如0.1这个数字，double只能表示他的近似值。

所以，**当我们使用new BigDecimal(0.1)创建一个BigDecimal 的时候，其实创建出来的值并不是正好等于0.1的。**

而是0.1000000000000000055511151231257827021181583404541015625。这是因为double自身表示的只是一个近似值。

**所以，如果我们在代码中，使用BigDecimal(double) 来创建一个BigDecimal的话，那么是损失了精度的，这是极其严重的。**

### 使用BigDecimal(String)创建

那么，该如何创建一个精确的BigDecimal来表示小数呢，答案是使用String创建。

而对于BigDecimal(String)，当我们使用new BigDecimal("0.1")创建一个BigDecimal 的时候，其实创建出来的值正好就是等于0.1的。

那么他的标度也就是1。

但是需要注意的是，new BigDecimal("0.10000")和new BigDecimal("0.1")这两个数的标度分别是5和1，如果使用BigDecimal的equals方法比较，得到的结果是false。

那么，想要创建一个能精确的表示0.1的BigDecimal，请使用以下两种方式：

```java
BigDecimal recommend1 = new BigDecimal("0.1");
BigDecimal recommend2 = BigDecimal.valueOf(0.1);
```

这里，留一个思考题，BigDecimal.valueOf()是调用Double.toString方法实现的，那么，既然double都是不精确的，BigDecimal.valueOf(0.1)怎么保证精确呢？

### 总结

因为计算机采用二进制处理数据，但是很多小数，如0.1的二进制是一个无限循环小数，而这种数字在计算机中是无法精确表示的。

所以，人们采用了一种通过近似值的方式在计算机中表示，于是就有了单精度浮点数和双精度浮点数等。

所以，作为单精度浮点数的float和双精度浮点数的double，在表示小数的时候只是近似值，并不是真实值。

所以，当使用BigDecimal(Double)创建一个的时候，得到的BigDecimal是损失了精度的。

而使用一个损失了精度的数字进行计算，得到的结果也是不精确的。

想要避免这个问题，可以通过BigDecimal(String)的方式创建BigDecimal，这样的情况下，0.1就会被精确的表示出来。

其表现形式是一个无标度数值1，和一个标度1的组合。

## 典型回答

大家都知道，不能用Float和Double来表示金额，会存在丢失精度的问题。

[✅为什么不能用浮点数表示金额？](https://www.yuque.com/hollis666/vgoof0/vmrkz84g8c6ypu5s)

那么要表示金额，业内有两种做法：

1、单位为分，数据库存bigint，代码中用long。如100.16元，存储为10016（在不考虑多币种的情况下）。

2、单位为元，数据库用decimal，代码中用BigDecimal（<u>我们一般数据库存储的是decimal(18,6)</u>）。如100.16元，直接存成100.16

这两种，其实我们都用过，而且现在也还都在用，因为他们都有各自的优缺点以及适用场景。

首先说**BigDecimal**，`BigDecimal` 是 Java 中用于精确计算的类，特别适合于需要高精度数值计算的场景，如金融、计量和工程等领域。其特点如下：

- **精确度高**：`BigDecimal` 可以表示非常大或非常精确的小数，而不会出现浮点数那样的舍入误差。
- **灵活的数学运算**：它提供各种方法进行精确的算术操作，包括加减乘除和四舍五入等。
- **控制舍入行为**：在进行数学运算时，你可以指定舍入模式，这对于金融计算非常重要。



**所以，BigDecimal的适用场景是需要高精度计算的金融应用，如货币计算、利率计算等。比如我们的结算系统、支付系统、账单系统等，都是用BigDecimal的。**

其次，再说Long，`long` 是 Java 的一种基本数据类型，用于表示没有小数部分的整数。其特点如下：

- **性能高**：作为基本数据类型，`long` 在处理速度上比 `BigDecimal` 快很多。
- **容量限制**：`long` 可以表示的最大值为 (2^{63}-1)，最小值为 (-2^{63})。这在大多数应用程序中已经足够，但在表示非常大的数或需要小数的计算中则不适用。
- **不适合精确的小数运算**：`long` 无法处理小数，如果需要代表金额中的小数部分（如厘），则需要自行管理这一部分。



**所以，Long的适用场景是适合于不涉及小数计算的大整数运算，如某些计数应用或者金额以整数形式表示。比如我们的额度系统、积分系统等。**

****

很多人会有疑惑，什么情况下会出现需要比分还小的单位呢？其实就是在很多需要运算的场景，比如说金融的费率、利率、服务费的费率等等，这些都是很小的，一般都是万分之几或者千分之几。而一旦有一个单位为元的金额和一个"率"相乘的时候，就会出现小于分的单位。

那有人说，遇到分我就直接四舍五入不就行了么，反正结算也是按照分结算的。这样做会有问题，我举个例子。

我一笔账单，有两笔订单，金额都是1元，存储的时候按照分存储，即100分，然后我的服务费费率是0.004。

如果是以分为单位，long存储和表示的话，那么两笔订单分开算费率的话：100*0.004 = 0.4，四舍五入 0，两笔加在一起，收入的费率就是0分。

但是如说是以元为单位，bigdecimal存储和表示的话，那么两笔订单分开算费率的话：1*0.004 = 0.004，两笔加在一起0.008，当我要结算的时候，再做四舍五入就是0.01元，即1分钱。

所以，**因为long在计算和存储的过程中都会丢失掉小数部分，那就会导致每一次都被迫需要四舍五入。而decimal完全可以保留过程中的数据，再最终需要的时候做一次整体的四舍五入，这样结果就会更加精确！**

所以，如果你的应用需要处理小数点后的精确计算（如金融计算中常见的多位小数），则应选择 `BigDecimal`。

如果你的应用对性能要求极高，并且没有乘除类运算，不需要很小的精度时，那么使用 `long` 可能更合适。

<font style="background-color:#FBDE28;">总结来说，对于绝大多数涉及货币计算的应用，推荐使用 </font>BigDecimal、，因为它提供了必要的精度和灵活性，尽管牺牲了一些性能。如果确定不需要处理小数，并且对执行速度有极端要求，使用long 可能更适合
