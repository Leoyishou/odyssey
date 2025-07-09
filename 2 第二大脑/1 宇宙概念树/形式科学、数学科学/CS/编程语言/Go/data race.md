---
aliases:
- 数据竞争
comment_id: 5969524a
date created: 2024-07-22
date modified: 2025-02-06
draw: null
title: data race
---
其实就是读写冲突或者写写冲突

- 什么是 data race? 数据竞争是在并发编程中出现的一个问题。它发生在两个或多个并发操作（比如 goroutines）同时访问同一块内存，并且至少有一个操作是写操作的时候。
- 用简单的比喻来理解：想象你和你的朋友在玩一个游戏，你们同时在一张纸上画画。如果你们都想在同一个位置画不同的东西，就会造成混乱，这就类似于 data race。
