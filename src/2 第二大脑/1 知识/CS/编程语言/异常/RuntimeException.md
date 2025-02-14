---
draw:
tags: []
title: RuntimeException
date created: 2024-12-01
date modified: 2024-12-27
---

## NPE

### for 循环的对象为空

```java
public static void main(String[] args) {  
    List<Integer> list = null;  
    for(Integer i : list) {  
        System.out.println(i);  
    }  
}
```

### BigDecimal 数组为空

```java
public static void main(String[] args) {  
    int[] arrInt = new int[3];  
    int i = arrInt[0] + 1;  
    System.out.println(i);               // 正常打印
  
    BigDecimal[] arr = new BigDecimal[3];  
    BigDecimal bigDecimal = arr[0].add(BigDecimal.ONE);  
    System.out.println(bigDecimal);      // 抛 NPE
}
```

### List 的 addAll

从语义上讲，向集合中添加 "nothing" (null) 是没有意义的。addAll 方法的目的是添加一个集合中的所有元素，而不是添加"无"。

```java
public boolean addAll(Collection<? extends E> c) {
    Objects.requireNonNull(c); // 快速检查，如果 c 为 null，立即抛出 NullPointerException
    boolean modified = false;
    for (E e : c) {
        if (add(e))
            modified = true;
    }
    return modified;
}
```

如果没有 `Objects.requireNonNull(c)` 这行代码，当 `c` 为 null 时，会在 for 循环中抛出 NullPointerException。这样的异常信息不够明确，可能会误导开发者认为问题出在循环内部的某个元素上。

## ClassCastException

```java
public static void main(String[] args) {  
    Map<String, Object> map = new HashMap<>();  
    map.put("addback", true);  
    System.out.println(Boolean.parseBoolean((String) map.get("addback")));  
}
```

```java
Object obj = new BigDecimal("123.45");

// 这是非法的，会抛出 ClassCastException
String str = (String) obj;
```

`()`的方式做强制类型转换，从 JVM 的视角，两个类的元数据要在多态的那套体系内，前者要兼容后者，像 String 和 BigDecimal 就属于 JVM 里面两个东西八竿子打不着一撇

### IndexOutOfBoundsException
