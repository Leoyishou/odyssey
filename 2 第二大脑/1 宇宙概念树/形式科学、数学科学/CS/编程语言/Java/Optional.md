---
comment_id: f481a7d5
date created: 2024-08-02
date modified: 2025-02-06
draw: null
title: Optional
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
