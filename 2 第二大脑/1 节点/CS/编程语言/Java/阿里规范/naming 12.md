---
draw:
title: naming 12
tags: [1 Dev, Java, 代码味道, 信息革命]
date created: 2024-04-17
date modified: 2024-11-12
---

> There are only two hard things in Computer Science: cache invalidation and naming things.  
—Phil Karlton

<!-- more -->

1.【强制】所有编程相关的命名均不能以下划线或美元符号开始，也不能以下划线或美元符号结束。

> easy

2.【强制】所有编程相关的命名严禁使用拼音与英文混合的方式，更不允许直接使用中文的方式。

> 想到一个 mianSha 报价的命名

3.【强制】代码和注释中都要避免使用任何人类语言中的种族歧视性或侮辱性词语。

> 别写 fuck

4.【强制】类名使用 UpperCamelCase 风格，以下情形例外：DO / PO / DTO / BO / VO / UID 等。

> 正例：ForceCode / UserDO / HtmlDTO / XmlService / TcpUdpDeal / TaPromotion  
  反例：forcecode / UserDo / HTMLDto / XMLService / TCPUDPDeal / TAPromotion  
  Polaris 中所有的 VO 都是 Vo

5.【强制】方法名、参数名、成员变量、局部变量都统一使用 lowerCamelCase 风格。

> easy

6.【强制】常量命名应该全部大写，单词间用下划线隔开，力求语义表达完整清楚，不要嫌名字长。

> 比如很多开关 switch 的命名，尽量长一点解释清晰一点

7.【强制】抽象类命名使用 Abstract 或 Base 开头；异常类命名使用 Exception 结尾，测试类命名以它要 测试的类的名称开始，以 Test 结尾。

> easy

8.【强制】类型与中括号紧挨相连来定义数组。

> 自己一直就是这个习惯

9.【强制】POJO 类中的任何布尔类型的变量，都不要加 is 前缀，否则部分框架解析会引起序列化错误。

> 在命名、ORM、数据库等多处规范中都有出现

10.【强制】包名统一使用小写，点分隔符之间有且仅有一个自然语义的英语单词。包名统一使用单数形式，但是类名如果有复数含义，类名可以使用复数形式。

> 包名只单数，类名可复数

11.【强制】避免在子父类的成员变量之间、或者不同代码块的局部变量之问采用完全相同的命名，使可理解性降低。

12.【强制】杜绝完全不规范的英文缩写，避免望文不知义。

> easy
