---
date created: 2025-04-17
date modified: 2025-07-10
uid: 45833656-5e51-4606-9f36-923bdebd1d1d
---
**Interpreting the question**

Yishou’s message, "类似 java 中的撒谎," seems like a typo. I think they meant "类似 Java 中的啥," or "similar to what in Java?" Given their earlier question about Python's `Literal` type, they may be asking for the Java equivalent. I’m guessing they’re referring to Java features like enums or sealed interfaces with constant values, which may align with Python's `Literal`. I'll provide that as the answer.

Thought for 5 seconds

在 Java 里，**最接近 `typing.Literal` 概念的就是 `enum`（枚举类型）**——都用“列举固定值”的方式，让编译器在编译期帮你做合法性检查。

|功能点|Python `Literal`|Java `enum`|说明|
|---|---|---|---|
|定义方式|`Literal["A", "B"]`|`enum Mode { A, B }`|把所有可选值一次性写死|
|编译期检查|✅ 静态类型检查器（mypy/pyright 等）|✅ Java 编译器|写错值直接报错|
|运行时开销|与普通常量几乎一样|每个枚举值是一个单例对象（轻量）|二者都可放心使用|
|代码提示|IDE 只补全列出的字面量|IDE 只补全枚举常量|提升开发体验|
|额外能力|仅限定值本身|可为常量添加字段/方法|Java `enum` 功能更丰富|

> **一句话理解：**  
> `Literal` 在 Python 的类型系统里扮演“**迷你枚举**”的角色，而 Java 自带的 `enum` 就是完整体。

---

## 小对照示例

Python（`Literal`）👇

```python
from typing import Literal

def set_mode(mode: Literal["TRAIN", "EVAL"]) -> None:
    ...
```

Java（`enum`）👇

```java
public enum Mode { TRAIN, EVAL }

public void setMode(Mode mode) {
    ...
}
```

- 若传入 `set_mode("Traiin")` 或 `setMode(null)`，编译器 / 类型检查器都会立刻报错。
    
- 在 Java 17+，你还可以用 **`sealed interface` + `record`** 实现更复杂、可扩展的“受限取值”模式，但最常见、最直观的还是 `enum`。
    

---

### 什么时候选哪一个？

|语言|典型场景|
|---|---|
|Python|只想在类型检查阶段限制字符串拼写，且无需运行时对象（轻量）|
|Java|需要强类型、希望方法签名里就看见枚举；或者枚举值本身要携带逻辑/属性|

---

**总结**

- 把 `Literal` 当成“轻量枚举”就好。
    
- 在 Java 里，你几乎总是用 `enum` 来达成同样的“只允许这几个值”效果。
