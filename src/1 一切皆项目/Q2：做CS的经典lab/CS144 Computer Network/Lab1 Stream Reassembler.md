---
draw:
tags: []
title: Lab1 Stream Reassembler
date created: 2024-12-30
date modified: 2025-01-01
---

在这个 StreamReassembler 中，buffer 和 bitmap 是用来处理乱序数据的两个关键数据结构：

1. buffer（缓冲区）

```cpp
std::deque<char> buffer; //!< The unassembled strings
```

- 存储还未能按序重组的数据内容
- 使用 deque 是因为需要在两端进行操作（前端取出数据，后端添加数据）
- 大小为 capacity，未使用的位置用 '\0' 填充

1. bitmap（位图）

```cpp
std::deque<bool> bitmap; //!< buffer bitmap
```

- 标记 buffer 中对应位置是否已经存储了有效数据
- true 表示该位置已有数据
- false 表示该位置尚未收到数据
- 与 buffer 一一对应

工作原理：

1. 当收到新数据片段时：

```cpp
buffer[i + offset] = data[i];  // 存储数据
bitmap[i + offset] = true;     // 标记该位置已有数据
```

1. 检查连续数据时（check_contiguous）：

```cpp
while (bitmap.front()) {  // 如果第一个位置有数据
    tmp += buffer.front();  // 取出数据
    buffer.pop_front();     // 移除已处理数据
    bitmap.pop_front();     // 移除对应标记
    // 在末尾补充空位
    buffer.push_back('\0');
    bitmap.push_back(false);
}
```

这样设计的好处：

1. 可以处理乱序到达的数据片段
2. 避免重复处理相同位置的数据
3. 能够快速判断哪些位置的数据已经到达
4. 方便检查是否有连续的数据可以输出

例如，假设收到以下片段：

```Java
index: 5, data: "def"
index: 0, data: "abc"
```

buffer和bitmap会帮助正确组装成"abcdef"。

## 新数据在 unass 点后

让我用 ASCII 图解释 `min(len, _capacity - _output.buffer_size() - offset)` 这个计算：

```Java
【整体视图】
capacity = 10 (总容量)


假设收到新数据："HELLO" (len=5)，从位置8开始 (offset=2)：
|<-------------- capacity=10 ------------>|
+---+---+---+---+---+---+---+---+---+---+
| A | B | C | D | E | F |   |   | H | E |  新数据位置示意
+---+---+---+---+---+---+---+---+---+---+
|<--- buffer_size=6 --->|
                        |offset|
                               |-可用空间-|


计算可用空间：
_capacity = 10            (总容量)
_output.buffer_size() = 6 (已使用空间)
offset = 2               (新数据的偏移量)
可用空间 = 10 - 6 - 2 = 2  (只能放入 "HE"，"LLO" 放不下)

real_len = min(len=5, 可用空间=2) = 2
```

再举个例子：

```Java
情况1：缓冲区较空
capacity = 10
+---+---+---+---+---+---+---+---+---+---+
| A | B |   |   |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+
|<->| buffer_size=2
     |->| offset=1
        |<---- 可用空间=7 ---->|

len = 5 ("HELLO")
real_len = min(5, 7) = 5  // 全部数据都能放入


情况2：缓冲区较满
capacity = 10
+---+---+---+---+---+---+---+---+---+---+
| A | B | C | D | E | F | G | H |   |   |
+---+---+---+---+---+---+---+---+---+---+
|<-------- buffer_size=8 -------->|
                                  |->| offset=1
                                      |->| 可用空间=1

len = 5 ("HELLO")
real_len = min(5, 1) = 1  // 只能放入一个字符
```

这个计算就像是在玩俄罗斯方块：

1. `capacity` 是游戏屏幕的总高度
2. `buffer_size` 是已经堆积的方块高度
3. `offset` 是新方块要放的位置到顶部的距离
4. `real_len` 是实际能放下的新方块数量

这样可以确保：

1. 不会超过总容量
2. 考虑了已使用空间
3. 考虑了新数据的位置
4. 取最小值保证安全

### 新数据与 unass 有部分重叠的情况

```Java
假设当前状态：
unass_base = 5 (从位置5开始未重组)
收到数据："WORLD" (len=5) 从位置3开始 (index=3)

【初始状态】
         index=3
            ↓
+---+---+---+---+---+---+---+---+---+---+
| A | B | C | W | O | R | L | D |   |   |  新数据
+---+---+---+---+---+---+---+---+---+---+
            |<----- data="WORLD" ----->|
                    ↑
               unass_base=5
        
offset = unass_base - index = 5 - 3 = 2
意味着要跳过 "WO"，只处理 "RLD"

【计算实际可用长度】
len = 5 (WORLD的长度)
len - offset = 3 (要处理的实际长度，即"RLD")
real_len = min(3, capacity - buffer_size)

【处理后】
+---+---+---+---+---+---+---+---+---+---+
| A | B | C | D | E | R | L | D |   |   |
+---+---+---+---+---+---+---+---+---+---+
buffer:   数据内容
bitmap:  
  1   1   1   1   1   1   1   1   0   0
                      ↑   ↑   ↑
                  新存储的数据
```

再举个更复杂的例子：

```Java
【情况1：部分重叠，部分超出容量】
capacity = 10
unass_base = 5
收到："HELLOWORLD" (index=3)

+---+---+---+---+---+---+---+---+---+---+
| A | B | C | H | E | L | L | O | W | O |  当前缓冲区
+---+---+---+---+---+---+---+---+---+---+
            |<------ 新数据 "HELLOWORLD" ------>|
                    ↑
                unass_base=5
                
offset = 5 - 3 = 2 (跳过"HE")
len - offset = 8 (要处理"LLOWORLD")
real_len = min(8, 剩余容量)


【情况2：有重叠且有已存在数据】
+---+---+---+---+---+---+---+---+---+---+
| A | B | C | D | E | x | x | O | W | O |  当前缓冲区
+---+---+---+---+---+---+---+---+---+---+
bitmap:   1   1   1   1   1   0   0   1   1   1
                        ↑   ↑   ↑
                        要填充的空位
```

代码逻辑解释：

```cpp
// 1. 计算偏移量
offset = unass_base - index;  // 跳过重叠部分

// 2. 计算实际可处理长度
real_len = min(len - offset, _capacity - _output.buffer_size());

// 3. 如果无法完全存储，取消 EOF 标记
if (real_len < len - offset) {
    _eof = false;
}

// 4. 存储数据，但跳过已有数据
for (size_t i = 0; i < real_len; i++) {
    if (bitmap[i]) continue;  // 跳过已有数据
    buffer[i] = data[i + offset];  // 存储新数据
    bitmap[i] = true;  // 标记为已使用
    unass_size++;  // 更新未重组数据大小
}
```

这就像拼拼图：

1. 找到新片段应该放的位置
2. 跳过已经有的部分
3. 把新的部分放进去
4. 标记哪些位置已经填好了
