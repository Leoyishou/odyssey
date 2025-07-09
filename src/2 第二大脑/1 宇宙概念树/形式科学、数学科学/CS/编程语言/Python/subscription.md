---
comment_id: bfb2c4eb
date created: 2025-05-02
date modified: 2025-05-02
draw: null
title: subscription
---
## Python 里的 “subscription” 到底是什么？

> **一句话**：`obj[something]` 这种方括号操作，在 Python 语法里就叫 **subscription（订阅 / 下标运算）**。
> 你可以把它理解成 “给对象一个 **下标**（sub-script）或 **键**，向它 **索取** 某样东西”。

---

## 1. “subscription” 这个词从哪儿来？

|关键词|解释|
|---|---|
|**sub-script**|在英语里本义是 “写在下面的小字”。早期编程语言把数组下标写在变量名右下角，叫 subscript。例如 `A₍ᵢⱼ₎`。后来方括号 `[]` 成了最常见的“下标记号”。|
|**subscription**|Python 官方语法文档把 `primary "[" expression_list "]"` 这一规则命名为 **subscription**。可以直译“下标式”，社区里也常叫“索引/切片操作”。|

> 所以它并不是“订阅 newsletter” 那个意思😂，只是历史上沿用了 _subscript_ → _subscription_ 的说法。

---

## 2. Python 如何实现 subscription？

1. **语法层面**  
    遇到 `obj[key]`，解释器会转成：
    
    ```python
    obj.__getitem__(key)        # 读取
    obj.__setitem__(key, value) # 赋值，用于 obj[key] = value
    ```
    

    只要一个类实现了这两个魔术方法，就能用 `[]` 像数组 / 字典那样访问。

    
2. **例子**
    
    ```python
    class MyList:
        def __init__(self, data):
            self.data = data
        def __getitem__(self, idx):
            return self.data[idx]
        def __setitem__(self, idx, val):
            self.data[idx] = val
    
    L = MyList([10, 20, 30])
    print(L[1])   # 触发 __getitem__ -> 20
    L[1] = 99     # 触发 __setitem__
    ```
    

---

## 3. 为什么 Java 没听过？

|对比点|Python|Java|
|---|---|---|
|**数组访问**|`arr[i]`（原生类型）；任何对象只要实现了 `__getitem__` 也能用|只有真·数组 `int[]`, `String[]` 能写 `arr[i]`|
|**集合访问**|`lst[i]`, `dict['k']` 等，都用 `[]`|列表、Map 必须调用方法：`list.get(i)`, `map.get(key)`|
|**语言特性**|Python 支持 **运算符重载**，类可自定义 `[]` 行为|Java 无运算符重载（除了 `+` 拼字符串），无法给自定义类新增 `[]` 语法糖|
|**官方术语**|_subscription_ / _subscript operator_|没有对应术语；通常说 **array access operator**|

因此，在 Java 世界里你只会听到 “调用 `get()` / `set()` 方法” 或 “数组下标运算”，而不会把 `[]` 扩展到别的对象，也就很少使用 “subscription” 这个词。

---

## 4. 在 pandas / NumPy 里的用法回看

- `df['A']` → `DataFrame.__getitem__('A')`
    
- `df.loc['row', 'col']` → `_LocIndexer.__getitem__((row, col))`
    
- `arr[1:5]` → `ndarray.__getitem__(slice(1,5,None))`
    

全部都依赖 **subscription** 机制，难怪一定要用 `[]` 而不是 `()`。

---

### 小结

- **subscription = “下标 / 索引 / 切片” 运算**，写成 `obj[...]`。
    
- 它本质上调用对象的 `__getitem__` / `__setitem__`。
    
- Java 没有运算符重载，所以只在原生数组里见过 `[]`，因此也鲜少提到 “subscription” 这个术语。
    

以后看到官方文档说 _subscription_，就把它翻译成 **“方括号索引”**，就不奇怪啦！
