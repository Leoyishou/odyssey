---
draw:
tags: []
title: Optional
date created: 2024-08-02
date modified: 2024-12-27
---

```java

Map<String, List> map =
if (map.get("good") == null) {

} else {

}

Optional(map.get("good"))
	.map()
	.orElse()
```

```java
WrapperInfoProxy res = wrapperInfoProxyList.stream()
	.filter(wrapperInfoProxy -> StringUtils.equals(wrapperInfoProxy.getWrapperId(), codebase))
	.findFirst()
	.orElseGet(WrapperInfoProxy::new);
```
