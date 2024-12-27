```
```
## 一、System call tracing 思路

### 1. 为什么要做系统调用追踪

在后续调试各种功能（尤其是内核相关功能）时，如果能在内核态“看见”当前进程调用了哪些系统调用、返回值是什么，会非常方便排查 Bug。因此，Lab2 的第一个子实验要求你为 xv6 添加一个 `trace` 系统调用，用于打开/关闭对应进程的“系统调用追踪”开关，并在追踪打开时打印出所有被追踪的系统调用的**名称**、**返回值**以及**发起调用的进程 PID**。

### 2. 实验大致做法

1. **添加系统调用 `trace`**
    
    - 在 `kernel/syscall.h` 中定义 `SYS_trace` 的编号。
    - 在 `kernel/syscall.c` 中把新的编号映射到你写好的内核处理函数 `sys_trace()`。
    - 在 `kernel/sysproc.c` 里实现 `sys_trace()` 函数，核心逻辑就是给当前进程（`myproc()`）的 `syscall_trace` 赋值。
    - 在用户态则需要在 `usys.pl`、`user.h` 等处为 `trace` 添加跳板函数和声明，如此才能从用户态方便地调用 `trace(mask)`。
2. **在 `proc.h` 中为进程添加一个 `syscall_trace` 字段**  
    这个字段用来存储“需要追踪哪些系统调用”的 bitmask，比如如果你传入的 mask 的某一位是 1，就代表需要追踪对应编号的系统调用。
    
    - 父进程在 fork 的时候会把这个 `syscall_trace` 传给子进程，这样父进程设置了 trace 之后，子进程也自动开启对应的追踪。
3. **在 `syscall()` 这个“统一处理入口”里去打印日志**  
    xv6 中所有系统调用都会在内核态走到 `syscall(void)` 函数，所以要做“哪几个系统调用被追踪并打印”的工作，自然就放在这里最好。
    
    - 先根据寄存器 `a7` 拿到系统调用编号 `num`。
    - 调用正确的内核处理函数，得到系统调用的返回值 (存入 `p->trapframe->a0`)。
    - 如果进程的 `syscall_trace` 对应 bit 置位，就通过 `printf` 打印出 `pid`, `syscall name`, `return value` 等信息。

## trace 的实现效果

在你完成 **trace** 系统调用并编译好 xv6 之后，你的 shell 里会多出一个用户态的可执行程序（通常叫 **`trace`**），用法大致是：

```
trace <mask> <command> [<args>...]
```

- **`<mask>`**：一个整数，里面的各个 bit 控制是否追踪对应编号（`SYS_xxx`）的系统调用。
- **`<command>`**：要执行的命令或程序。
- **`<args>...`**：该命令需要的参数。

**当 trace 启动 `<command>` 时，只要 `<mask>` 对某个系统调用的编号进行“位掩码”匹配，这个系统调用就会被跟踪，并在命令执行期间每次系统调用返回前打印一行**。输出格式类似：

```
<进程ID>: syscall <系统调用名> -> <返回值>
```

下面给你举一些在 xv6 shell 里可能看到的例子，方便了解 trace 实际如何工作。

---

### 1. 只追踪一个系统调用（比如 `read`）

假设在 xv6 内核的 `kernel/syscall.h` 中，`SYS_read` 的编号是 5，那么它的位掩码就是 `1 << 5 = 32`。于是：

```sh
$ trace 32 grep hello README
```

这会只追踪 `SYS_read`。示例输出可能是：

```
3: syscall read -> 1023
3: syscall read -> 966
3: syscall read -> 70
3: syscall read -> 0
3: syscall close -> 0
```

> 这里显示进程 PID 为 3，`read` 每次返回了多少字节，最后还有一次 `close`（如果你也把 `close` 的掩码打开，就会显示它的踪迹）。

由于这里只设置了掩码 `32`（即只跟踪 `read`），所以 `exec`、`open` 等其他系统调用并不会输出。

---

### 2. 跟踪所有系统调用

如果你想看某个程序的**所有系统调用**，可以把 31 位都置为 1，比如 `2147483647 = 0x7fffffff`，这是一个常见的“所有位都打开”的掩码：

```sh
$ trace 2147483647 grep hello README
```

输出示例（省略部分）：

```
4: syscall trace -> 0
4: syscall exec -> 3
4: syscall open -> 3
4: syscall read -> 1023
4: syscall read -> 966
4: syscall read -> 70
4: syscall read -> 0
4: syscall close -> 0
...
```

> 可以看到这时会打印 `trace`, `exec`, `open`, `read`, `close` 等所有调用及它们的返回值。

---

### 3. 不追踪任何系统调用

如果 `<mask>` 是 0，那么不会打印任何跟踪信息：

```sh
$ trace 0 grep hello README
```

此时 grep 照常执行，但不会输出任何 “syscall … -> …” 行。

---

### 4. 追踪 fork 及其子进程的 fork

假设 `SYS_fork` 的编号是 1，那么掩码就是 `1 << 1 = 2`。 例如，有个测试程序叫 `forkforkfork`（会连续地 fork 好几次）：

```sh
$ trace 2 usertests forkforkfork
usertests starting
test forkforkfork: 
407: syscall fork -> 408
408: syscall fork -> 409
409: syscall fork -> 410
410: syscall fork -> 411
409: syscall fork -> 412
410: syscall fork -> 413
409: syscall fork -> 414
411: syscall fork -> 415
...
```

因为你在父进程中打开了 trace，同时在 `fork()` 的实现里会把 `trace mask` 复制到子进程，所以后面所有子进程也继续带着同样的跟踪掩码——只要它们调用 `fork`，就会打出类似的行。

---

### 5. 追踪多个系统调用

如果你想一次追踪多个系统调用，比如 `fork` (`SYS_fork`) 和 `read` (`SYS_read`)，可以把对应的位掩码“或”起来。举例：

- `SYS_fork` 编号是 1 → `1 << 1 = 2`
- `SYS_read` 编号是 5 → `1 << 5 = 32`

那么 “2 | 32 = 34” 就会同时追踪 `fork` 和 `read`。你可以在 shell 中：

```sh
$ trace 34 somecommand ...
```

---

### 总结

- **`trace <mask> <command>`** 会启动 `<command>` 并对 `<mask>` 指定的系统调用进行跟踪。
- **输出格式**：`PID: syscall xxx -> return_value`。
- **子进程继承**：一旦某进程调用了 `trace(mask)`, 该进程后续的子进程也会带着相同的 `mask`。

通过这些示例，你就能看到 **trace** 在 xv6 shell 里具体如何工作、以及它如何帮助你调试或观察系统调用的执行状况。祝你实验顺利!