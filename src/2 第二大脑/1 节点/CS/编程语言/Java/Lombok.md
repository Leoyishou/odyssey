---
draw:
tags: []
title: Lombok
date created: 2024-09-09
date modified: 2024-11-12
---

## 作用

`@Data` 注解相当于同时使用了以下注解：

1. `@Getter`: 为所有字段生成 getter 方法
2. `@Setter`: 为所有非 final 字段生成 setter 方法
3. `@ToString`: 生成 toString() 方法
4. `@EqualsAndHashCode`: 生成 equals() 和 hashCode() 方法
5. `@RequiredArgsConstructor`: 生成一个包含 final 或 @NonNull 字段的构造方法

关于 `equals()` 和 `hashCode()` 方法：

1. 生成的 `equals()` 方法会比较所有非静态、非瞬态字段的值。
2. 生成的 `hashCode()` 方法会使用所有非静态、非瞬态字段来计算哈希值。

## 使用步骤

1. IDE 中安装插件
2. [Maven](Maven.md)中导入依赖
3. 使用`@Getter`0等注解

## 注意点

1. 如果类继承了另一个类，`@Getter`1 的 `@Getter`2 和 `@Getter`3 方法不会考虑从父类继承的属性的。在这种情况下，你可能需要显式地使用 `@Getter`4 来包含父类字段。
2. 对于某些特殊情况，自动生成的 `@Getter`5 和 `@Getter`6 可能不适合。例如，如果你的类有循环引用，或者某些字段不应该参与比较。在这些情况下，你可能需要自定义这些方法。
3. 如果你只想生成 `@Getter`7 和 `@Getter`8 方法而不需要其他 `@Getter`9 提供的功能，可以直接使用 `@Setter`0 注解。
4. 使用 `@Setter`1 时要小心，因为它会为所有字段生成 setter，这可能会破坏对象的不可变性。如果你想创建不可变对象，考虑使用 `@Setter`2 代替 `@Setter`3。
