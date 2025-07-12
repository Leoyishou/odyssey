---
date created: 2024-12-23
date modified: 2025-07-10
uid: dc5da772-9ef9-4285-aa0a-17f6e0733b09
---
```c
void
syscall(void)
{
  int num;
  struct proc *p = myproc();

  num = p->trapframe->a7;     // 从 a7 寄存器获取系统调用号
  if(num > 0 && num < NELEM(syscalls) && syscalls[num]) {
    p->trapframe->a0 = syscalls[num]();   // 执行系统调用并将返回值存在 a0
  } else {
    printf("%d %s: unknown sys call %d\n",
            p->pid, p->name, num);
    p->trapframe->a0 = -1;
  }
}
```

## 为什么从 a7 寄存器获取系统调用号？

这是用户程序和操作系统内核交互的关键接口。当用户程序需要进行系统调用时：

- 设置系统调用号到 a7
- 触发陷阱（trap）进入内核
- 内核执行这个 syscall() 函数
- 从 a0 获取返回值
