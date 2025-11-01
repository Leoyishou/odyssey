---
date created: 2025-09-01
date modified: 2025-09-01
uid: 8c82f697-9602-4825-a9e9-fd4e119ef7f4
---

| 问题                                              | 关键词/关键概念                                                                                        |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| 什么是深拷贝和浅拷贝？| 浅拷贝复制引用、深拷贝复制对象、递归复制                                                                            |
| Java中异常分哪两类？| [Throwable@](Throwable@.md)，Checked Exception（编译时异常）、Unchecked Exception（运行时异常RuntimeException）|
| Arrays.sort是使用什么排序算法实现的？| DualPivotQuicksort（双轴快排）、基本类型用快排、对象类型用TimSort（归并排序变种）|
| BigDecimal(double)和BigDecimal(String)有什么区别？| [基本数据类型](基本数据类型.md)，double构造器存在精度损失、String构造器精确、推荐使用String或valueOf                              |
|                                                 |                                                                                                 |
| BigDecimal和Long表示金额哪个更合适？| BigDecimal适合需要小数精度场景、Long适合分为单位整数场景、考虑精度需求和性能                                                   |
| String、StringBuilder和StringBuffer的区别？| [[String]]不可变、StringBuilder可变非线程安全、StringBuffer可变线程安全                                           |
| String为什么设计成不可变的？| 安全性、缓存hashCode、字符串常量池、线程安全                                                                      |
|                                                 |                                                                                                 |
|                                                 |                                                                                                 |
| ClassNotFoundException和NoClassDefFoundError的区别？| 前者是Exception运行时动态加载失败、后者是Error编译时存在但运行时找不到                                                      |
| JDK 9中对字符串的拼接做了什么优化？| invokedynamic指令、StringConcatFactory、减少StringBuilder创建                                           |
| JDK新版本中都有哪些新特性？| JDK21 LTS版本、虚拟线程、模式匹配、记录类型、密封类、文本块                                                              |
| Java中Timer实现定时调度的原理？| TaskQueue优先队列、TimerThread线程、wait/notify机制、单线程执行                                                 |
| Java中创建对象有哪些种方式？| new关键字、反射、clone()、反序列化、Unsafe.allocateInstance                                                  |
| Java中有了基本类型为什么还需要包装类？| 泛型需要、集合框架、null值表示、对象方法、自动装箱拆箱                                                                   |
| Java中的static都能用来修饰什么？| 变量、方法、代码块、内部类、导入（import static）|
| Java中的枚举有什么特点和好处？| 类型安全、单例、可添加方法、switch支持、线程安全                                                                     |
| Java和C++主要区别？| 内存管理（GC vs 手动）、指针、多继承、平台依赖性、编译方式                                                                |
| Java序列化的原理？| ObjectOutputStream/InputStream、Serializable接口、序列化ID、对象图遍历                                       |
| Java是值传递还是引用传递？| 值传递、基本类型传值、引用类型传引用的值（地址值）|
| Java注解的作用？| 元数据、编译检查、运行时处理、代码生成、框架配置                                                                        |
| Java的动态代理如何实现？| JDK代理（接口+InvocationHandler）、CGLIB（继承+MethodInterceptor）|
| Lambda表达式是如何实现的？| invokedynamic指令、函数式接口、LambdaMetafactory、匿名内部类优化                                                 |
| RPC接口返回中，使用基本类型还是包装类？| 包装类、可表示null、避免NPE、明确语义                                                                          |
| SimpleDateFormat是线程安全的吗？| 非线程安全、使用ThreadLocal、DateTimeFormatter（Java 8线程安全）|
| Stream的并行流一定比串行流更快吗？| 不一定、取决于数据量、操作复杂度、线程开销、CPU核心数                                                                    |
| String a="ab"; String b="a"+"b"; a==b吗？| true、编译期常量优化、字符串常量池                                                                             |
| String str=new String("hollis")创建了几个对象？| 1个或2个、堆中new对象、常量池中"hollis"（如果不存在）|
| String中intern的原理？| 字符串常量池、返回池中引用、JDK7后池在堆中                                                                         |
| String是如何实现不可变的？| final类、private final char[]数组、无setter方法、返回新对象                                                   |
| String有长度限制吗？| 编译期65535字符、运行期Integer.MAX_VALUE、常量池CONSTANT_Utf8限制                                              |
| char能存储中文吗？| 可以、UTF-16编码、2字节、BMP范围内中文                                                                        |
| finally中代码一定会执行吗？| 不一定、System.exit()、线程死亡、JVM崩溃时不执行                                                                |
| final、finally、finalize区别？| final修饰符、finally异常处理、finalize垃圾回收前调用                                                            |
| serialVersionUID作用？| 版本控制、反序列化兼容性检查、建议显式声明                                                                           |
| try return A, catch return B, finally return C？| 返回C、finally中return会覆盖、不建议finally中return                                                         |
| while(true)和for(;;)哪个性能好？| 性能相同、编译后字节码一致、语义差异                                                                              |
| 为什么JDK9把String的char[]改成byte[]？| 节省内存、Latin-1编码优化、Compact Strings、coder标记                                                        |
| 为什么Java不支持多继承？| 菱形继承问题、复杂性、接口多继承替代                                                                              |
| 为什么main方法必须是public static void？| JVM规范、无需实例化、访问权限、固定入口                                                                           |
| 为什么不建议使用异常控制业务流程？| 性能开销、语义不清、栈轨迹开销、代码可读性                                                                           |
| 为什么不能用BigDecimal的equals做等值比较？| equals比较值和精度、compareTo只比较值、1.0≠1.00用equals                                                      |
| 为什么不能用浮点数表示金额？| 精度丢失、IEEE 754标准、二进制表示限制                                                                         |
| 为什么负数取绝对值结果不一定是正数？| Integer.MIN_VALUE、补码表示、-2^31没有对应正数                                                              |
| 为什么建议多用组合少用继承？| 灵活性、低耦合、避免继承链、符合开闭原则                                                                            |
| 为什么建议自定义无参构造函数？| 反射需要、框架要求、子类调用、序列化                                                                              |
| 什么是AIO、BIO和NIO？| BIO同步阻塞、NIO同步非阻塞、AIO异步非阻塞                                                                       |
| 什么是SPI和API区别？| SPI服务发现机制、API应用编程接口、调用方向相反                                                                      |
| 什么是UUID？| 通用唯一标识符、128位、版本4随机、理论唯一实际极小概率重复                                                                 |
| 什么是反射机制？为什么慢？| 运行时动态获取类信息、安全检查、JIT优化失效、方法调用开销                                                                  |
| 什么是序列化与反序列化？| 对象转字节流、持久化、网络传输、深拷贝                                                                             |
| 什么是泛型？好处？| 参数化类型、类型安全、消除强转、代码复用                                                                            |
| 什么是类型擦除？| 编译期泛型信息擦除、运行时Object、桥接方法                                                                        |
| fastjson反序列化漏洞？| autoType功能、任意类实例化、RCE风险、版本升级                                                                    |
| 反射与封装矛盾？| 矛盾存在、SecurityManager、模块系统、访问控制                                                                  |
| 如何理解多态？| 编译时多态（重载）、运行时多态（重写）、向上转型                                                                        |
| 面向对象vs面向过程？| OOP封装继承多态、过程顺序执行、抽象vs具体、复用vs效率                                                                  |
| 字符串常量何时进入常量池？| 类加载时、编译期常量、运行时intern()                                                                          |
| 常见字符编码？| ASCII、UTF-8变长、UTF-16定长、GBK中文、Unicode字符集                                                         |
| 如何修改private String？| 反射Field.setAccessible(true)、JDK9+需要--add-opens                                                  |
| 接口vs抽象类？| 接口多实现、抽象类单继承、JDK8接口default方法、is-a vs can-do                                                     |
| 为什么需要hashCode？| HashMap/HashSet需要、equals相等hashCode必须相等、提高查找效率                                                   |
| 泛型中K T V E？| K-Key、T-Type、V-Value、E-Element、?通配符、Object根类                                                    |
| extends和super区别？| extends上界通配符（生产者）、super下界通配符（消费者）、PECS原则                                                        |
| 现在JDK最新版本？| JDK 23（2024年9月）、JDK 21 LTS长期支持版                                                                 |
| 常见语法糖？| 自动装箱拆箱、增强for循环、try-with-resources、泛型、变长参数、枚举、内部类                                                |
|                                                 |                                                                                                 |
|                                                 |                                                                                                 |
|                                                 |                                                                                                 |
|                                                 |                                                                                                 |

- Arrays.sort是使用什么排序算法实现的？
- BigDecimal(double)和BigDecimal(String)有什么区别？
- BigDecimal和Long表示金额哪个更合适，怎么选择？
- ClassNotFoundException和NoClassDefFoundError的区别是什么？
- JDK 9中对字符串的拼接做了什么优化？
- JDK新版本中都有哪些新特性？
- Java中Timer实现定时调度的原理是什么？
- Java中创建对象有哪些种方式
- Java中异常分哪两类，有什么区别？
- Java中有了基本类型为什么还需要包装类？
- Java中的static都能用来修饰什么？
- Java中的枚举有什么特点和好处
- Java和C++主要区别有哪些？各有哪些优缺点？
- Java序列化的原理是啥
- Java是值传递还是引用传递？
- Java注解的作用是啥
- Java的动态代理如何实现？
- Lambda表达式是如何实现的？
- RPC接口返回中，使用基本类型还是包装类？
- SimpleDateFormat是线程安全的吗？使用时应该注意什么？
- Stream的并行流一定比串行流更快吗？
- String a = "ab"; String b = "a" + "b"; a == b 吗？
- String str=new String("hollis")创建了几个对象？
- String、StringBuilder和StringBuffer的区别？
- String中intern的原理是什么？
- String为什么设计成不可变的？
- String是如何实现不可变的？
- String有长度限制吗？是多少？
- char能存储中文吗？
- finally中代码一定会执行吗？
- final、finally、finalize有什么区别
- serialVersionUID 有何用途? 如果没定义会有什么问题？
- try中return A，catch中return B，finally中return C，最终返回值是什么？
- while(true)和for(;;)哪个性能好？
- 为什么JDK 9中把String的$char[]$改成了$byte[]$？
- 为什么Java不支持多继承？
- 为什么Java中的main方法必须是public static void的？
- 为什么不建议使用异常控制业务流程
- 为什么不能用BigDecimal的equals方法做等值比较？
- 为什么不能用浮点数表示金额？
- 为什么对Java中的负数取绝对值结果不一定是正数？
- 为什么建议多用组合少用继承？
- 为什么建议自定义一个无参构造函数
- 为什么这段代码在JDK不同版本中结果不同
- 什么是AIO、BIO和NIO？
- 什么是SPI，和API有啥区别
- 什么是UUID，能保证唯一吗？
- 什么是反射机制？为什么反射慢？
- 什么是序列化与反序列化
- 什么是泛型？有什么好处？
- 什么是深拷贝和浅拷贝？
- 什么是类型擦除？
- 以下关于异常处理的代码有哪些问题
- 你知道fastjson的反序列化漏洞吗
- 反射与封装是否矛盾？如何解决反射破坏封装不安全的问题？
- 如何理解Java中的多态？
- 如何理解面向对象和面向过程？
- 字符串常量是什么时候进入到字符串常量池的？
- 常见的字符编码有哪些？有什么区别？
- 怎么修改一个类中的private修饰的String参数的值
- 接口和抽象类的区别，如何选择？
- 有了equals为啥需要hashCode方法？
- 泛型中K T V E？Object等分别代表什么含义。
- 泛型中上下界限定符extends 和 super有什么区别？
- 现在JDK的最新版本是什么？
- 说几个常见的语法糖？
