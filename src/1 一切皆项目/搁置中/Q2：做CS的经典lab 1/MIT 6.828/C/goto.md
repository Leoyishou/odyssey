---
comment_id: aa7e0842
date created: 2024-12-25
date modified: 2024-12-27
draw: null
tags: []
title: goto
---
这是一个很好的问题！这里使用 `goto found` 而不是直接写代码主要有两个原因：

1. **代码结构清晰**：

```c
static struct proc* allocproc(void) {
  struct proc *p;

  for(p = proc; p < &proc[NPROC]; p++) {
    acquire(&p->lock);
    if(p->state == UNUSED) {
      // 如果在这里写初始化代码：
      // p->pid = allocpid();
      // p->state = USED;
      // ... 20多行初始化代码 ...
      // 会让循环结构不清晰
    } else {
      release(&p->lock);
    }
  }
  return 0;

found:
  // 所有初始化代码集中在这里
  p->pid = allocpid();
  p->state = USED;
  // ... 其他初始化 ...
}
```

1. **避免重复的锁处理**：

```c
for(p = proc; p < &proc[NPROC]; p++) {
    acquire(&p->lock);
    if(p->state == UNUSED) {
      // 如果在这里写代码，需要考虑：
      // 1. 初始化失败时的锁释放
      // 2. 成功时的返回前锁处理
      // 会使锁的处理逻辑分散
    } else {
      release(&p->lock);
    }
}
```

使用 goto 的好处：

1. 搜索逻辑和初始化逻辑分离
2. 错误处理更集中
3. 锁的管理更清晰

就像：

```java
// 不好的方式
for(Parking spot : parkingLots) {
    spot.lock();
    if(spot.isEmpty()) {
        spot.setOwner(car);
        spot.markOccupied();
        spot.paintLines();
        spot.installSensor();
        // ... 很多初始化代码
        // 容易让循环逻辑变得混乱
    } else {
        spot.unlock();
    }
}

// 更好的方式
Parking foundSpot = null;
for(Parking spot : parkingLots) {
    spot.lock();
    if(spot.isEmpty()) {
        foundSpot = spot;
        break;  // 相当于 goto
    }
    spot.unlock();
}

if(foundSpot != null) {
    // 所有初始化代码集中在这里
    foundSpot.setOwner(car);
    foundSpot.markOccupied();
    foundSpot.paintLines();
    foundSpot.installSensor();
}
```

虽然通常不推荐使用 goto，但在这种特定场景下，它能让代码更清晰和易维护。

`found` 不是一个独立的方法，它是一个标签（label），是 goto 语句的跳转目标。

在 C 语言中：

```c
static struct proc* allocproc(void) {
  struct proc *p;

  for(p = proc; p < &proc[NPROC]; p++) {
    acquire(&p->lock);
    if(p->state == UNUSED) {
      goto found;  // 跳转到 found 标签
    } else {
      release(&p->lock);
    }
  }
  return 0;

found:   // 这是一个标签，不是方法
  p->pid = allocpid();
  // ... 更多初始化代码 ...
  return p;
}
```

Java 中没有 goto 语句，但有类似的替代方案：

1. **使用标签 break**：

```java
public Process allocProc() {
    searchBlock: {  // 定义标签
        for(Process p : processes) {
            p.lock();
            if(p.isUnused()) {
                break searchBlock;  // 跳出到标签
            }
            p.unlock();
        }
        return null;
    }
    // 相当于 C 中的 found 标签后的代码
    p.setPid(allocPid());
    // ... 更多初始化代码 ...
    return p;
}
```

2. **使用方法提取**：

```java
public Process allocProc() {
    Process p = findUnusedProcess();
    if (p == null) return null;
    
    // 初始化代码
    initializeProcess(p);
    return p;
}

private Process findUnusedProcess() {
    for(Process p : processes) {
        p.lock();
        if(p.isUnused()) {
            return p;
        }
        p.unlock();
    }
    return null;
}
```

3. **使用异常控制流**（不推荐）：

```java
public Process allocProc() {
    try {
        for(Process p : processes) {
            p.lock();
            if(p.isUnused()) {
                throw new FoundException(p);  // 跳转到 catch
            }
            p.unlock();
        }
        return null;
    } catch (FoundException e) {
        Process p = e.getProcess();
        // 初始化代码
        return p;
    }
}
```

总结：

- C 中的 goto + label 是一种低级的控制流机制
- Java 不支持 goto，但提供了其他更结构化的替代方案
- 在大多数情况下，重构为独立方法是更好的选择
