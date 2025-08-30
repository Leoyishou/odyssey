---
date created: 2024-08-02
date modified: 2025-07-10
uid: c3611b3d-d3c0-4aee-a51a-6f9aeecdb0ed
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
