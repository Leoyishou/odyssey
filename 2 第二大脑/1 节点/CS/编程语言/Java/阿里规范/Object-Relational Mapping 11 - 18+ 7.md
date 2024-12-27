---
draw:
title: Object-Relational Mapping 11 - 18+ 7
tags: [1 Dev, Java, 代码味道, 信息革命]
date created: 2024-04-17
date modified: 2024-12-27
---

excerpt

<!-- more -->

  

在后端开发中，OOP（面向对象编程）规约指的是一组实践原则和设计模式，

## 表数与表意

8.【强制】任何货币金额，均以最小货币单位且为整型类型进行存储。

7.【强制】所有整型包装类对象之间值的比较，全部使用 equals 方法比较。

> 因为整数常量池的存在

9.【强制】浮点数之间的等值判断，基本数据类型不能使用 == 进行比较，包装数据类型不能使用 equals 进行判断。

```java
(1)指定一个误差范围，两个浮点数的差值在此范围之内，则认为是相等的。

float a = 1.0F - 0.9F;
float b = 0.9F - 0.8F;
float diff = 1e-6F;
if (Math.abs(a - b) < diff) {
	System.out.println("true");
}

(2)使用 BigDecimal 来定义值，再进行浮点数的运算操作。
BigDecimal a = new BigDecimal("1.0");
BigDecimal b = new BigDecimal("0.9");
BigDecimal c = new BigDecimal("0.8");
BigDecimal x = a.subtract(b);
BigDecimal y = b.subtract(c);
if (x.compareTo(y) == 0) {
	System.out.println("true");
}
```

10.【强制】BigDecimal 的等值比较应使用 compareTo() 方法，而不是 equals() 方法。

12.【强制】禁止使用构造方法 BigDecimal(double) 的方式把 double 值转化为 BigDecimal 对象。

> 正例：优先推荐**入参为 String** 的构造方法，或使用 BigDecimal 的 valueOf 方法，此方法内部其实执行了 Double 的  
toString，而 Double 的 toString 按 double 的实际能表达的精度对尾数进行了截断。
BigDecimal recommend1 = new BigDecimal("0.1");  
BigDecimal recommend2 = BigDecimal.valueOf(0.1);

## POJO

16.【强制】构造方法里面禁止加入任何业务逻辑，如果有初始化逻辑，请放在 init 方法中。

13.【强制】所有的 POJO 类属性必须使用包装数据类型。

14.【强制】定义 DO / PO / DTO / VO 等 POJO 类时，不要设定任何属性默认值

18.【强制】禁止在 POJO 类中，同时存在对应属性 xxx 的 isXxx() 和 getXxx() 方法。

17.【强制】POJO 类必须写 toString 方法。使用 IDE 中的工具 source > generate toString 时，如果继承了另一个 POJO 类，注意在前面加一下 super.toString()。

```java
com.qunar.dzs.hotelsearch.polaris.domain.entity.price.MakePriceSubrule@7c29daf3

MakePriceSubrule(subruleType=null, priceRangeLowerLimit=null, priceRangeUpperLimit=null, beatRate=null, defRate=null, maxRate=null, minRate=null, beatMaxRate=null, beatMinRate=null, maxBeatMoney=null)
```

## DO

11.【强制】定义数据对象 DO 类时，属性类型要与数据库字段类型相匹配。

| MySQL Type                   | JDBCType                         | JavaType                   |
| ---------------------------- | -------------------------------- | -------------------------- |
| CHAR, VARCHAR                | CHAR, VARCHAR                    | String                     |
| *TEXT*, MEDIUMTEXT, LONGTEXT | *LONGVARCHAR*                    | String                     |
| DECIMAL, NUMERIC             | NUMERIC, DECIMAL                 | java.math.BigDecimal       |
| TINYINT(1)                   | BIT, BOOLEAN                     | boolean                    |
| TINYINT                      | TINYINT                          | byte                       |
| SMALLINT                     | SMALLINT                         | short                      |
| INT                          | INTEGER                          | int                        |
| BIGINT                       | BIGINT                           | long                       |
| FLOAT                        | REAL                             | float                      |
| FLOAT, DOUBLE                | FLOAT, DOUBLE                    | double                     |
| BINARY, VARBINARY            | BINARY, VARBINARY                | byte[]|
| BLOB, MEDIUMBLOB, LONGBLOB   | LONGVARBINARY                    | byte[]|
| BINARY, VARBINARY, BLOB      | BINARY, VARBINARY, LONGVARBINARY | byte[]|
| DATE                         | DATE                             | java.sql.Date              |
| TIME                         | TIME                             | java.sql.Time              |
| TIMESTAMP                    | TIMESTAMP                        | java.sql.Timestamp         |
| TEXT                         | CLOB                             | Clob                       |
| BLOB, MEDIUMBLOB, LONGBLOB   | BLOB                             | Blob                       |
|                              | ARRAY                            | Array                      |
|                              | DISTINCT                         | mapping of underlying type |
|                              | STRUCT                           | Struct                     |
|                              | REF                              | Ref                        |
|                              | DATALINK                         | java.net.URL               |

| MySQL  | JDBCType      | JavaType                   |
| ------ | ------------- | -------------------------- |
|        | CHAR          | String                     |
|        | VARCHAR       | String                     |
| *Text* | LONGVARCHAR   | String                     |
|        | NUMERIC       | java.math.BigDecimal       |
|        | DECIMAL       | java.math.BigDecimal       |
|        | BIT           | boolean                    |
|        | BOOLEAN       | boolean                    |
|        | TINYINT       | byte                       |
|        | SMALLINT      | short                      |
|        | INTEGER       | int                        |
|        | BIGINT        | long                       |
|        | REAL          | float                      |
|        | FLOAT         | double                     |
|        | DOUBLE        | double                     |
|        | BINARY        | byte[]|
|        | VARBINARY     | byte[]|
|        | LONGVARBINARY | byte[]|
|        | DATE          | java.sql.Date              |
|        | TIME          | java.sql.Time              |
|        | TIMESTAMP     | java.sql.Timestamp         |
|        | CLOB          | Clob                       |
|        | BLOB          | Blob                       |
|        | ARRAY         | Array                      |
|        | DISTINCT      | mapping of underlying type |
|        | STRUCT        | Struct                     |
|        | REF           | Ref                        |
|        | DATALINK      | java.net.URL               |

---

1.【强制】在表查询中，一律不要使用 * 作为查询的字段列表，需要哪些字段必须明确写明。

2.【强制】POJO 类的布尔属性不能加 is，而数据库字段必须加 is_，要求在 resultMap 中进行字段与属  
性之间的映射。

3.【强制】不要用 resultClass 当返回参数，即使所有类属性名与数据库字段一一对应，也需要定义 resultMap；反过来，每一个表也必然有一个 resultMap 与之对应。

4.【强制】sql.xml 配置参数使用：#{}，#param# 不要使用 ${} 此种方式容易出现 SQL 注入。

5.【强制】iBATIS 自带的 queryForList(String statementName，int start，int size) 不推荐使用。

6.【强制】不允许直接拿 HashMap 与 Hashtable 作为查询结果集的输出。

> 道理类似于 extendMap 透传字段，和数据库交互用自定义的对象接

7.【强制】更新数据表记录时，必须同时更新记录对应的 update_time 字段值为当前时间。

## 字符串

占位符的问题

  

在 Java 标准库中，`String.format` 方法确实不支持 `{}` 作为占位符。这种占位符风格主要来自于诸如 SLF4J 这样的日志库，这些库在内部实现了自己的字符串格式化机制，允许使用 `{}` 作为占位符。如果你在代码中使用日志记录（比如使用 SLF4J 或 Logback），你确实可以直接使用 `{}` 占位符来格式化消息。但在普通的字符串处理中，如使用 `String.format`，你需要使用 `%s`（字符串）、`%d`（整数）、`%f`（浮点数）等格式化占位符

## 为什么一般不需要实现 serializable？什么时候才需要实现 serializable

1. 给前端的 VO，一般不需要实现 Serializable 接口。
我们返回给前端的数据，通常是 JSON 和 XML，但是实际上这是一个 String，String 是实现了 java。io。Serializable 的，所以我们的 DTO 是不需要实现 java。io。Serializable 接口的。

2. 给数据库的 DAO，一般需要实现 Serializable 接口  
除非你要将一个对象写入文件，此时就需要继承 Serializable。这个接口不用实现任何方法，就是一个标识作用，
Jvm 在做序列化操作的时候会去检查目标类有没有实现 java.io.Serializable 接口，没有实现的话，根据情况抛出异常：java.io.NotSerializableException
