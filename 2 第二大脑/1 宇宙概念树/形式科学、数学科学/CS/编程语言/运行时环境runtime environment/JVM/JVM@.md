---
draw:
title: JVM@
date created: 2024-07-14
date modified: 2025-04-08
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F09%2F20%2F19-15-39-b5a27ea6955dff8eadca5d56b025dfcc-202409201915990-43d59b.png)

-Xmn：500M，就是说年轻代大小是500M（包括一个Eden和两个Survivor）
	-XX:SurvivorRatio=3，就是说Eden区与Survivor区的大小比值为3：1：1  
-XX:MaxPermSize：64M，就是说设置持久代最大值为64M   

  
-Xms：1G，就是说初始堆大小为1G  
-Xmx：2G，就是说最大堆大小为2G  
-XX:+UseConcMarkSweepGC，就是说使用使用CMS内存收集算法   

题目中所问的Eden区的大小是指年轻代的大小，直接根据-Xmn：500M和-XX:SurvivorRatio=3可以直接计算得出  
500M×(3/(3+1+1))  
=500M×（3/5）

=500M×0.6  
=300M  
所以Eden区域的大小为300M  
[堆](堆.md)

面试官 | JVM 为什么使用元空间替换了永久代？- 码上技术指导老师的文章 - 知乎  
https://zhuanlan.zhihu.com/p/111809384

虚拟机运行时数据区：2 和 3

虚拟机外：堆外/直接内存 1和 4

## 1、关于类的：类信息、常量、静态变量（都存在线程安全的问题）

方法区规范内还包括一个运行时常量池的规范，运行时常量池又包括一个字符串常量池的规范

![](https://cdn-a.markji.com/files/62fc6526889af996baeff5aa_hd.png?e=1725356282256&token=xX63b9jqTlDOcGmctt5K9254rV0LG8hS9BmDeFBy:qZatzy49ufFbQ36w0Pr7hPN0tHw=)

存的是c++中的一种数据结构 instanceKlass，其中有一个属性java_mirror，存在heap中，使得Klass可以暴露给java使用

JDK7以上版本，静态域存储于定义类型的Class对象中，Class对象如同堆中其他对象一样，存在于GC堆中。

## 2、关于对象的：堆的实现包括新生代和老年代

![](https://cdn-a.markji.com/files/62fc6571889af996baf0075c_hd.png?e=1725356282256&token=xX63b9jqTlDOcGmctt5K9254rV0LG8hS9BmDeFBy:-Z0uL7UIN9XcOHMAdm1qA1PbAyo=)

这是一个对象的内存图

## 3、关于线程的：栈的实现包括虚拟机栈、程序计数器、和本地方法栈

其中虚拟机栈的栈帧内又包括:"局部变量表"、"操作数栈"、"动态链接"以及"方法出口"四个部分。

操作数栈

局部变量表（八种基本数据类型、引用、返回值）

动态链接（主要服务一个方法需要调用其他方法的场景）

![](https://cdn-a.markji.com/files/62fc6454889af996baefcfea_hd.png?e=1725356282256&token=xX63b9jqTlDOcGmctt5K9254rV0LG8hS9BmDeFBy:x0H2GQDSSYsjYRx70iyedksc9iQ=)

## 4、直接内存

1、读写成本小

通过Java 堆中的 DirectByteBuffer操作，避免java堆和native堆之间的来回复制，也就是nio New Input

传统的阻塞io，得开辟两份缓冲区，操作系统管的内存开辟一份，java自己管的内存也开辟一份

第二张是new io只有一份缓冲区（java和操作系统都可管）

在内存建一个buffer，输入靠InputStream read, 输出靠 OutputStream write

[Pic#ID/5ZM9#]

2、回收成本大

直接内存也会导致内存溢出问题：

1、不受JVM内存回收管理，不会释放direct memory的内存。因为直接内存是属于操作系统的，java的内存回收不能回收他，

2、如果要回收，需要主动调用Unsafe对象的freeMemory方法，Unsafe是Java底层的类，一般不建议使用。

3、直接内存释放是借助了java的虚引用机制，ByteBuffer的实现类内部，使用了Cleaner（虚引用）来监测ByteBuffer对象，一旦ByteBuffer对象被垃圾回收，那么就会由ReferenceHandler线程通过Cleaner的clean方法调用freeMemory来释放直接内存。

内存布局：方法区位置的变化

1.8前在全部在永久代中实现。1.8在元空间（本地内存）上实现，其中的字符串常量池被安排在了堆中

另外还有一个直接内存是nio用来当缓冲区的，效率比较高

![](https://cdn-a.markji.com/files/62fc64922f8e2c97ce3712ab_hd.png?e=1725356282256&token=xX63b9jqTlDOcGmctt5K9254rV0LG8hS9BmDeFBy:MlasBp8RXmOfmk1W88b4uHhyrAM=)![](https://cdn-a.markji.com/files/62fc6498889af996baefde16_hd.png?e=1725356282256&token=xX63b9jqTlDOcGmctt5K9254rV0LG8hS9BmDeFBy:foXriQpYxyf_C2FN1rR0OcnemZs=)

threadlocal
