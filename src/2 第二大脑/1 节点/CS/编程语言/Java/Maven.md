---
draw:
tags: []
title: Maven
date created: 2024-07-03
date modified: 2024-11-12
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F07%2F03%2F15-07-20-2317aa337084a11cf816a1c37968b75a-20240703150719-1f22aa.png)

## 全球共享的共有仓库

## 公司内共享的私有的仓库

Nexus 是一个软件仓库管理器,主要用于 Java 开发中。

1. 作为一个私有的 Maven 仓库,可以存放公司内部开Jar 包
2. 可以作为公共仓库的代理,加快依赖下载速度。
3. 帮助管理不同版本的组件。

## 依赖关系分析

### 显性关系 `mvn dependency:tree`

```Java
├── h_hprice_common
│   ├── xray.client
│   ├── wrapper-loader-client
│   ├── qabtest-client
│   ├── cbrs-over-hqmonitor
│   ├── fastutil
│   ├── algorithm-api
│   ├── commons-email
│   ├── h_hprice_base
│   ├── common-sentinel
│   └── magic-mirror-monitor
├── com.qunar.nl
├── com.qunar.mc
```

### 隐性关系 `

`

#### 当前使用但未声明

```Java
com.qunar.hotel.qta:qta-biz-common
org.apache.commons:commons-collections4
com.qunar.hotel:wrapper-loader-client
com.qunar.ucenter:ucenter-client-common
org.springframework:spring-web

```

#### 已声明但未使用

```Java
qunar.tc:qmq-api
qunar.common:common-lang
qunar.tc.qtracer:qtracer-client
xml-apis:xml-apis
qunar.tc.qconfig:qconfig-client
org.springframework:spring-core
commons-lang:commons-lang
org.hibernate:hibernate-validator
javax.validation:validation-api
org.springframework:spring-context
```
