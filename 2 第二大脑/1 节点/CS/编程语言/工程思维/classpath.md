---
draw:
title: classpath
tags: [1 Dev, Java, 信息革命, 工程思维]
date created: 2024-04-28
date modified: 2024-12-27
---

excerpt

<!-- more -->

项目中，有很多地方需要配置文件路径，总是分不清 classpath 的用途，这里进行一个总结。

平时我们使用 idea 时，有很多细节都会屏蔽了，当我们使用 idea 启动项目时他会设置一个 classpath，这个参数有 jdk 下的 jar 和项目中 target/classes 目录

我们的应用启动前，生成的.class 文件会被放到 target/classes 目录下，resources 目录下的文件也会被复制到 target/classes 目录下，如上图所示。

之前也会遇到一个问题就是，我明明在代码里修改了相应的属性或方法，但是却没有生效，其实这就有可能是因为 target/classes 目录下的内容没有更新，当出现问题时可以先查看对应的文件是否一致。并可以通过 idea 的 rebuild 命令解决。

**classpath 代表什么？**

它其实代表的就是 target/classes 目录，我们可以通过代码进行验证（使用 spring 的 ResourceUtils 类）：

|   |
|---|
|``//添加classpath和不加classspath获取的路径效果`<br>`<br>``URL url1 = ResourceUtils.getURL(``"classpath:"``);`<br>`<br>``System.out.println(url1.getPath());`<br>`<br>``URL url2 = ResourceUtils.getURL(``""``);`<br>`<br>`System.out.println(url2.getPath());`|

结果如下：

![](https://wiki.corp.qunar.com/download/attachments/475858871/image2021-8-3_22-18-28.png?version=1&modificationDate=1628000308000&api=v2)

还有一个是 classpath*，它不仅会扫描 target/classes 目录，也会扫描 jar 文件中的 class 路径。
