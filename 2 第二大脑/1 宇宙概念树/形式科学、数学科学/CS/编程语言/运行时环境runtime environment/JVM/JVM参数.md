---
comment_id: b8063adf
date created: 2023-12-26
date modified: 2025-02-06
draw: null
title: JVM参数
---
## 标准参数

标准参数是 jvm 所有版本都是支持的参数。

- 具体信息
    

    |参数|说明|备注|简写|  
    |---|---|---|---|  
    |参数|说明|备注|简写|  
    |---|---|---|---|  
    |-agentlib:libname[=options]|加载指定的本机代理库。特定的库的选项列表使用逗号分割|-agentlib:hprof=cpu=samples,interval=20,depth=3||  
    |-agentpath:pathname[=options]|加载绝对路径名称的本机代理库，跟上面的 -agentlib 功能一致，区别在于使用完整的路径和文件名|||  
    |-client|选择 JAVA Hotspot 客户端 VM 编译，加载||64 位版本的 jdk 默认使用 -server|  
    |-Dproperty=value|设置系统属性值||属性名称是一个不能带空格的字符串；属性值如果有空字符串使用双引号括住|  
    |-d32|声明应用是在 32 位的环境运行||默认使用 32 位|  
    |-d64|||目前只有 Java HotSpot Server VM 支持 64 位操作，并且 -server 选项在使用 -d64 时是隐含的。使用 -d64 会忽略 -client 选项。这在未来的版本中可能会发生变化。|  
    |-disableassertions[:[packagename]...|:classname]|禁用断言。默认情况下，所有包和类中的断言都被禁用。|要在特定包或类中显式启用断言，请使用 -enableassertions (-ea) 选项。|  
    |-disablesystemassertions|禁用所有系统类中的断言。||-dsa|  
    |-enableassertions[:[packagename]...|:classname]|启用断言。默认情况下，所有包和类中的断言都被禁用。||  
    |-enablesystemassertions|在所有系统类中启用断言。||-esa|  
    |-help|||-?|  
    |-jar filename|执行在 JAR 文件中的程序。||filename 参数是带有清单的 JAR 文件的名称，其中包含格式为 Main-Class:classname 的行，该行定义了具有公共静态 void main(String[]args) 方法的类，该方法用作应用程序的起点。|  
    |-javaagent:jarpath[=options]|加载指定的 Java 编程语言代理||[http://docs.oracle.com/javase/8/docs/api/java/lang/instrument/package-summary.html](http://docs.oracle.com/javase/8/docs/api/java/lang/instrument/package-summary.html)|  
    |-jre-restrict-search|在版本搜索中包括用户私有 JRE。|||  
    |-no-jre-restrict-search|从版本搜索中排除用户私有 JRE。|||  
    |-server|选择 server 虚拟机|||  
    |-showversion|显示版本信息并继续执行应用程序。|||  
    |-splash:imgname|使用 imgname 指定的图像显示启动画面|-splash:images/splash.gif||  
    |-verbose:class|显示有关每个已加载类的信息。|||  
    |-verbose:gc|显示有关每个垃圾回收 (GC) 事件的信息。|||  
    |-verbose:jni|显示有关使用本机方法和其他 Java 本机接口 (JNI) 活动的信息。|||  
    |-version|显示版本信息然后退出。|||  
    |-version:release|指定用于运行应用程序的发行版本|||
