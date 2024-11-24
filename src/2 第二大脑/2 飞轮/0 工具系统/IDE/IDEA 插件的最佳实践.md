---
draw:
title: IDEA 插件的最佳实践
tags: [最佳实践]
date created: 2024-04-20
date modified: 2024-11-12
---

excerpt

<!-- more -->

stackoverflow

## CamelCase- 多种命名格式之间切换

## RoboPOJOGenerator：JSON 转类对象

## Rainbow Brackets: 彩虹🌈括号

通过自定义一些 [后缀补全](后缀补全) [Live Templates](Live%20Templates)，来践行良好的编程习惯

Preferences | Editor | General | Postfix Completion
```


|                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ``// for 循环 for fori forr`<br>`<br> `List<String> list =` `new` ``ArrayList<>();`<br>`<br>``// 判空(非空).null.notnull.nn`<br>`<br>`Object o =` `new` ``Object();`<br>`<br>``// boolean 取反.if .not.if`<br>`<br>`boolean` `List<String> list =`0 ``List<String> list =`1`List<String> list =`2<br>`<br>``List<String> list =`3<br>`<br>`List<String> list =`4 `List<String> list =`5 ``List<String> list =`6<br>`<br>``List<String> list =`7<br>`<br>``List<String> list =`8<br>`<br>``List<String> list =`9<br>`<br>`new`0 `new`1 `new`2 |  
```

alibaba

Rainbow

file expander

gittoolbox

arthas idea

## 断点处添加 log

我们在在调试代码时都喜欢 `new`3 一些内容，这样看起来更直观，print 完之后又很容易忘记删除掉这些没用的内容，最终将代码提交到 `new`4 时又不得不删减这些内容重新提交，不但增加不必要的工作量，还会在线上打印一些没有价值的信息增加日志量。IntelliJ IDEA 提供 `new`5 功能恰巧可以帮助我们解决这个问题。