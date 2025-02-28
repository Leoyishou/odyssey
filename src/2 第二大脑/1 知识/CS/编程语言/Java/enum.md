---
draw:
title: enum
date created: 2024-07-03
date modified: 2025-02-06
---

 枚举是 [JVM@](JVM@.md) 里一张不会变的参考表，就好像 [汉谟拉比法典](汉谟拉比法典)  
 所以枚举的 field 应该是 final 的，不然会出这种警告 `Non-final field 'code' in enum 'HomogenizationSelectorType'`

## 反编译后的枚举代码

values() 方法是 Java 编译器在编译时自动为每个枚举类合成的静态方法。当我们定义一个枚举类时，编译器会自动添加这些特殊的方法。

让我来详细解释一下：

1. 编译器合成的方法：

```java
// 这些方法在编译后会被自动添加到枚举类中
public static BedroomCountEnum[] values(); 
public static BedroomCountEnum valueOf(String name);
```

1. 反编译一个枚举类看看实际实现：

```java
// 原始代码
public enum BedroomCountEnum {
    ONE(1, "1间卧室"),
    TWO(2, "2间卧室");
    
    private final int code;
    private final String desc;
    
    BedroomCountEnum(int code, String desc) {
        this.code = code;
        this.desc = desc;
    }
}

// 反编译后的主要部分
public final class BedroomCountEnum extends Enum<BedroomCountEnum> {
    // 1. 每个枚举常量都被转换为静态final字段
    public static final BedroomCountEnum ONE = new BedroomCountEnum("ONE", 0, 1, "1间卧室");
    public static final BedroomCountEnum TWO = new BedroomCountEnum("TWO", 1, 2, "2间卧室");
    
    private final int code;
    private final String desc;
    
    // 2. 编译器生成的values()方法实现
    private static final BedroomCountEnum[] $VALUES;
    static {
        $VALUES = new BedroomCountEnum[]{ONE, TWO};
    }
    
    public static BedroomCountEnum[] values() {
        return (BedroomCountEnum[])$VALUES.clone();
    }
    
    // 3. 编译器生成的valueOf()方法实现
    public static BedroomCountEnum valueOf(String name) {
        return (BedroomCountEnum)Enum.valueOf(BedroomCountEnum.class, name);
    }
    
    // 构造方法
    private BedroomCountEnum(String name, int ordinal, int code, String desc) {
        super(name, ordinal);
        this.code = code;
        this.desc = desc;
    }
}
```

关键点解释：

1. values() 方法的实现：
   - 编译器会创建一个私有的静态数组 $VALUES 存储所有枚举常量
   - values() 方法返回这个数组的克隆，防止外部修改原数组
   - 返回的数组顺序与枚举常量定义顺序一致

2. 性能考虑：

```java
// 每次调用values()都会克隆数组
BedroomCountEnum[] values = BedroomCountEnum.values();  // 会创建新数组

// 如果需要频繁使用，最好缓存结果
public class BedroomCountEnumUtils {
    private static final BedroomCountEnum[] VALUES = BedroomCountEnum.values();
    
    public static BedroomCountEnum[] getValues() {
        return VALUES;  // 直接返回缓存的数组
    }
}
```

1. 为什么要克隆数组？

```java
// 如果直接返回原数组，可能会被修改
BedroomCountEnum[] values = BedroomCountEnum.values();
values[0] = null;  // 如果不是克隆的数组，这会影响原始数据

// 克隆确保了枚举值的不可变性
BedroomCountEnum[] values1 = BedroomCountEnum.values();
BedroomCountEnum[] values2 = BedroomCountEnum.values();
values1[0] = null;  // 不会影响values2
```

所以，当我们使用 values() 方法时，要注意：

1. 它是编译器自动生成的
2. 每次调用都会创建新的数组（克隆）
3. 如果需要频繁使用，建议缓存结果
4. 返回顺序与定义顺序一致
5. 实际是基于一个私有的静态数组实现的

这也就解释了为什么前面讨论的遍历方式会比 Map 方式慢 - 因为每次调用 values() 都会克隆数组，而且还要遍历这个数组。相比之下，Map 的查找是 O(1) 的，且不需要克隆操作。

## 根据 code 反查 enum

这两种实现方式各有优劣，我来分析一下：

1. 遍历方式 (values())：

```java
public static BedroomCountEnum getByCode(int code) {
    for (BedroomCountEnum value : values()) {
        if (value.getCode() == code) {
            return value;
        }
    }
    return null;
}
```

优点：

- 代码简单直观，易于理解和维护
- 内存占用少，不需要额外的数据结构
- 适合枚举值较少的情况（一般小于10个）
- 类加载时无需额外初始化

缺劣：

- 时间复杂度为 O(n)，需要遍历所有枚举值
- 频繁调用时性能较差

1. Map缓存方式：

```java
private static final Map<Integer, BedroomCountEnum> CODE_MAP;
static {
    CODE_MAP = Maps.newHashMap();
    for (BedroomCountEnum value : values()) {
        CODE_MAP.put(value.getCode(), value);
    }
}

public static BedroomCountEnum getByCode(int code) {
    return CODE_MAP.get(code);
}
```

优点：

- 查找时间复杂度为 O(1)，性能好
- 适合频繁查询的场景
- 适合枚举值较多的情况

缺点：

- 需要额外的内存存储 Map
- 类加载时需要初始化 Map
- 代码稍微复杂一些

选择建议：

1. 使用遍历方式的场景：
- 枚举值较少（比如少于10个）
- 查询频率不高
- 内存资源紧张
- 追求代码简洁性

1. 使用 Map 缓存的场景：
- 枚举值较多
- 查询频繁
- 对查询性能要求高
- 内存资源充足

在你的例子中，因为是房间数量枚举，通常不会有太多值，所以两种实现方式差异不大。但如果这个方法会被频繁调用（比如在循环中或批量处理数据时），建议使用 Map 方式。

补充一个性能对比的简单示例：

```java
// 假设有1000000次查询
@Test
public void performanceTest() {
    long start = System.currentTimeMillis();
    
    for (int i = 0; i < 1000000; i++) {
        BedroomCountEnum.getByCode(2); // 遍历方式
    }
    
    long end1 = System.currentTimeMillis();
    
    for (int i = 0; i < 1000000; i++) {
        BedroomCountEnum.codeOf(2); // Map方式
    }
    
    long end2 = System.currentTimeMillis();
    
    System.out.println("遍历方式耗时：" + (end1 - start) + "ms");
    System.out.println("Map方式耗时：" + (end2 - end1) + "ms");
}
```

实际项目中，如果这个枚举类使用频率高，建议使用 Map 方式。如果使用频率低，两种方式都可以，选择你觉得更容易维护的方式即可。
