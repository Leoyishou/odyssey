---
draw:
title: 递归比较对象，记录 diff 日志
date created: 2024-04-18
date modified: 2025-02-14
---

excerpt

<!-- more -->

```java
package com.qunar.dzs.hotelsearch.polaris.util;  
  
import com.google.common.collect.ImmutableSet;  
import com.qunar.dzs.hotelsearch.polaris.domain.annotation.CompareDesc;  
import com.qunar.flight.qmonitor.QMonitor;  
import java.beans.PropertyDescriptor;  
import java.lang.reflect.Field;  
import java.lang.reflect.Method;  
import java.math.BigDecimal;  
import java.util.Collection;  
import java.util.Collections;  
import java.util.Date;  
import java.util.HashSet;  
import java.util.Map;  
import java.util.Objects;  
import java.util.Set;  
import java.util.concurrent.ConcurrentHashMap;  
import org.apache.commons.collections4.CollectionUtils;  
import org.apache.commons.collections4.MapUtils;  
import org.apache.commons.lang3.ClassUtils;  
import org.joda.time.DateTime;  
import org.joda.time.format.DateTimeFormat;  
import org.joda.time.format.DateTimeFormatter;  
import org.reflections.ReflectionUtils;  
import org.slf4j.Logger;  
import org.slf4j.LoggerFactory;  
  
import static io.netty.util.internal.StringUtil.LINE_FEED;  
  
/**  
 * 对象属性对比工具  
 **/  
public class CompareUtil {  
  
    private static final Logger logger = LoggerFactory.getLogger(CompareUtil.class);  
  
    private static final ConcurrentHashMap<Class, Set<Field>> FIELD_CACHE = new ConcurrentHashMap<>();  
  
    private static final DateTimeFormatter FORMAT = DateTimeFormat.forPattern("yyyy-MM-dd HH:mm:ss");  
  
    private static final ImmutableSet<Class<? extends Comparable>> COMMON_CLASS_SET = ImmutableSet.of(String.class, Date.class, BigDecimal.class);  
  
    public static void compareObject(Object oldObject, Object newObject, StringBuffer diffMessage) {  
       //有对象为空，则直接返回  
       if (Objects.isNull(oldObject) || Objects.isNull(newObject)) {  
          return;  
       }  
       //如果两个对象为不同类型，则直接返回  
       if (!oldObject.getClass().equals(newObject.getClass())) {  
          return;  
       }  
       Set<Field> allFields = getFields(oldObject.getClass());  
       for (Field field : allFields) {  
          CompareDesc annotation = field.getAnnotation(CompareDesc.class);  
          if (annotation == null) {  
             continue;  
          }  
          String desc = annotation.desc();  
          try {  
             PropertyDescriptor pd = new PropertyDescriptor(field.getName(), oldObject.getClass());  
             Method getMethod = pd.getReadMethod();  
             Object o1 = getMethod.invoke(oldObject);  
             Object o2 = getMethod.invoke(newObject);  
             if (o1 instanceof Collection && o2 instanceof Collection) {  
                compareCollection(desc, (Collection<?>) o1, (Collection<?>) o2, diffMessage);  
             }else if (o1 instanceof Map && o2 instanceof Map) {  
                compareMap(desc, (Map<?, ?>) o1, (Map<?, ?>) o2, diffMessage);  
             }else {  
                if (!(o1 instanceof Comparable) || !(o2 instanceof Comparable)) {  
                   continue;  
                }  
                Comparable oldObjectValue = (Comparable) o1;  
                Comparable newObjectValue = (Comparable) o2;  
                // 如果不是类中定义的普通类型对象, 就继续递归检查其子成员变量  
                if (!isPrimitive(field)) {  
                   compareObject(oldObjectValue, newObjectValue, diffMessage);  
                   continue;  
                }  
                if (oldObjectValue != null && newObjectValue == null) {  
                   diffMessage.append("删除").append(desc).append(parseDataValue(oldObjectValue)).append(LINE_FEED);  
                } else if (oldObjectValue == null && newObjectValue != null) {  
                   diffMessage.append("新增").append(desc).append(parseDataValue(newObjectValue)).append(LINE_FEED);  
                } else if (oldObjectValue != null && newObjectValue.compareTo(oldObjectValue) != 0) {  
                   diffMessage.append(desc).append(parseDataValue(oldObjectValue)).append("变为了").append(parseDataValue(newObjectValue))  
                      .append(LINE_FEED);  
                }  
             }  
          } catch (Exception e) {  
             logger.error("变更对比出错", e);  
             QMonitor.recordOne("diffError");  
          }  
       }  
    }  
  
    /**  
     * 生成Collection字段的diff信息  
     */  
    private static void compareCollection(String collectionType, Collection<?> oldCollection, Collection<?> newCollection, StringBuffer diffMsg) {  
       if (CollectionUtils.isEmpty(oldCollection)) {  
          oldCollection = Collections.emptySet();  
       }  
       if (CollectionUtils.isEmpty(newCollection)) {  
          newCollection = Collections.emptySet();  
       }  
       // 使用 HashSet 来处理可能的重复元素问题，并提高性能  
       Set<?> oldSet = new HashSet<>(oldCollection);  
       Set<?> newSet = new HashSet<>(newCollection);  
       // 找出新增的元素  
       Set<?> added = new HashSet<>(newSet);  
       added.removeAll(oldSet);  
       // 找出删除的元素  
       Set<?> removed = new HashSet<>(oldSet);  
       removed.removeAll(newSet);  
       // 追加记录到 diffMsg       if (CollectionUtils.isNotEmpty(removed)) {  
          diffMsg.append("被删除的: ").append(collectionType).append(": ").append(JsonUtils.toJson(removed)).append(LINE_FEED);  
       }  
       if (CollectionUtils.isNotEmpty(added)) {  
          diffMsg.append("新增的:  ").append(collectionType).append(": ").append(JsonUtils.toJson(added)).append(LINE_FEED);  
       }  
    }  
  
    private static void compareMap(String mapDesc, Map<?, ?> oldMap, Map<?, ?> newMap, StringBuffer diffMsg) {  
       if (MapUtils.isEmpty(oldMap)) {  
          if (MapUtils.isNotEmpty(newMap)) {  
             diffMsg.append("新增的: ").append(mapDesc).append(": ").append(JsonUtils.toJson(newMap)).append(LINE_FEED);  
             return;  
          }  
       } else if (MapUtils.isEmpty(newMap)) {  
          diffMsg.append("被删除的: ").append(mapDesc).append(": ").append(JsonUtils.toJson(oldMap)).append(LINE_FEED);  
          return;  
       }  
       diffMsg.append(mapDesc).append(parseDataValue(oldMap)).append("变为了").append(parseDataValue(newMap))  
          .append(LINE_FEED);  
    }  
  
    private static String parseDataValue(Object data) {  
       if (data instanceof Date) {  
          return new DateTime(data).toString(FORMAT);  
       } else {  
          return data.toString();  
       }  
    }  
  
    //获取某个类的所有属性，使用了ConcurrentHashMap做缓存，提升了系统性能  
    private static Set<Field> getFields(Class clazz) {  
       Set<Field> fields = FIELD_CACHE.get(clazz);  
       if (CollectionUtils.isEmpty(fields)) {  
          Set<Field> allFields = ReflectionUtils.getAllFields(clazz);  
          FIELD_CACHE.put(clazz, allFields);  
          return allFields;  
       }  
       return fields;  
    }  
  
  
    // 判断某个对象是否是普通类型或者String,Date,BigDecimal这种可以打印的类型  
    private static boolean isPrimitive(Field field) {  
       Class<?> type = field.getType();  
       if (ClassUtils.isPrimitiveOrWrapper(type)) {  
          return true;  
       }  
       for (Class clazz : COMMON_CLASS_SET) {  
          if (clazz.equals(type)) {  
             return true;  
          }  
       }  
       return false;  
    }  
}
```

`CompareUtil` 类的实现清晰且结构良好，用于比较两个对象的状态并以结构化格式报告差异。然而，还有几个潜在的改进和优化的地方：

## 1. **优化反射的使用**

- **缓存改进**：虽然已经使用 `FIELD_CACHE` 缓存类的字段，你还可以考虑缓存每个字段的 `PropertyDescriptor` 实例，以避免在每次调用 `compareObject` 时重新构造它们。每次比较时为每个字段创建 `PropertyDescriptor` 可能相当耗费资源。
- **字段访问性**：如果安全约束允许，你可以设置 `field.setAccessible(true)` 来加速反射访问。这应谨慎使用，并且只在安全策略允许的环境中使用。

## 2. **错误处理**

- **异常处理**：当前你在发生异常时记录错误并记录监控事件。也许还可以更优雅地处理这些错误，比如通过返回包含任何失败信息的结果或状态对象。
- **日志和监控**：考虑添加更详细的日志或调试信息，特别是在关键决策点（如类型比较和递归调用）周围。

## 3. **类型比较**

- **原始类型处理**：`isPrimitive` 方法检查是否属于预定义的简单或 " 原始 " 类型集合。可以考虑扩展此集合以处理更多类型，或通过外部配置这些类型集合。
- **Comparable 处理**：你检查对象是否为 `Comparable` 的实例，但有些对象可能有自然排序，这不是通过 `Comparable` 捕获的。可以通过处理特定情况或使用比较器来增强这一点。

## 4. **代码清晰度和可维护性**

- **魔术字符串和常量**：使用常量替代重复字符串，如 `CompareUtil`0、`CompareUtil`1 和 `CompareUtil`2，以避免打字错误并简化未来的修改。
- **方法分解**：`CompareUtil`3 方法功能较多。考虑将其拆分为更小、更专注的方法，以提高可读性和可维护性。

## 5. **性能考虑**

- **集合比较**：比较集合时，将它们转换为 `CompareUtil`4 可能对大型集合来说效率不高。此外，这种方法会丢失元素的顺序。根据业务需求，考虑是否需要维护元素顺序或以不同方式处理重复项。
- **避免不必要的包装**：在集合和映射比较方法中，有包装到新的 `CompareUtil`5 或类似结构的操作，这可以优化或避免，如果底层集合已经是集合或如果这种包装不是必需的。
