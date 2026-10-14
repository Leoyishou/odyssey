---
date created: 2024-12-25
date modified: 2025-07-10
uid: 0fabc65d-49ce-4814-a8a1-32d20dc821fb
---
```Java
┌─────────────────────────────────────────────────────────────────────────────┐
│                  父进程 (在用户态) 调用 fork()                             │
└─────────────────────────────────────────────────────────────────────────────┘
              |
              | (触发系统调用，进入内核)
              v
┌─────────────────────────────────────────────────────────────────────────────┐
│  (父进程内核态) 执行 fork():                                              │
│                                                                           │
│  1) p = myproc();                   // 拿到当前(父)进程 p                 │
│  2) np = allocproc();               // 分配并初始化子进程 np              │
│  3) uvmcopy(...);                   // 拷贝父进程内存给子进程             │
│  4) np->trapframe->a0 = 0;          // "子进程"的返回值寄存器设为 0       │
│  5) for(i=0..NOFILE) filedup(...)   // 复制文件描述符给子进程            │
│     np->cwd = idup(p->cwd);         // 复制工作目录                      │
│     safestrcpy(np->name, p->name);  // 子进程继承名字                     │
│  6) pid = np->pid;                  // 记录子进程的PID                    │
│  7) np->state = RUNNABLE;           // 子进程置为就绪                      │
│  8) return pid;                     // 内核态返回 pid → 父进程用户态      │
└─────────────────────────────────────────────────────────────────────────────┘
              |
              | (切换回父进程用户态)
              v
┌─────────────────────────────────────────────────────────────────────────────┐
│ 父进程用户态得到 fork() == pid                                            │
│ (父进程继续执行自己的后续代码)                                            │
└─────────────────────────────────────────────────────────────────────────────┘


                             ---- (分割线) ----


┌─────────────────────────────────────────────────────────────────────────────┐
│    子进程此时并没有"执行 fork() 函数"                 │
│    它只是被内核放到了可运行队列 (RUNNABLE)，并还没真正运行。             │
└─────────────────────────────────────────────────────────────────────────────┘
                             |
                             | (调度器切换到子进程)
                             v
┌─────────────────────────────────────────────────────────────────────────────┐
│ 子进程"上 CPU"时，从用户态视角看，自己就像"从 fork() 返回"：              │
│                                                                           │
│    trapframe->a0 = 0   -->  fork() 的返回值==0                            │
│                                                                           │
│ 子进程其实是在"用户态的 fork() 调用点之后"继续执行，                       │
│ 自己看到 fork() 的返回值是 0                                             │
└─────────────────────────────────────────────────────────────────────────────┘

```

## 父进程视角

```c
// Create a new process, copying the parent.
// Sets up child kernel stack to return as if from fork() system call.
int
fork(void)
{
  int i, pid;
  struct proc *np;
  struct proc *p = myproc();  //Process p = Process.currentProcess();

  // Allocate process.
  if((np = allocproc()) == 0){
    return -1;  //返回 -1 表示错误
  }

  // Copy user memory from parent to child.
  // 复制内存空间 np.setMemory(p.getMemory().clone());
  if(uvmcopy(p->pagetable, np->pagetable, p->sz) < 0){
    freeproc(np);
    release(&np->lock);
    return -1;
  }
  np->sz = p->sz;

  np->parent = p; //np.setParent(p);

  np->mask = p->mask; //np.setMask(p.getMask());

  // copy saved user registers.
  *(np->trapframe) = *(p->trapframe);

  // Cause fork to return 0 in the child.
  // 修改子进程这边的返回值，
  // 父进程拿到的 fork() 返回值是 pid，子进程则是 0。这是 Unix fork() 的经典语义。
  np->trapframe->a0 = 0; 

  // increment reference counts on open file descriptors.
  for(i = 0; i < NOFILE; i++)
    if(p->ofile[i])
	  // child.openFiles[i] = new FileDescriptor(parent.openFiles[i])
      np->ofile[i] = filedup(p->ofile[i]);
  np->cwd = idup(p->cwd);

  safestrcpy(np->name, p->name, sizeof(p->name));

  pid = np->pid;

  np->state = RUNNABLE;

  release(&np->lock);

  return pid;
}
```

## 子进程视角

```c
// 在子进程 trapframe 中将 a0 设为 0
np->trapframe->a0 = 0;

```

这一切都只需要内核在**父进程的"fork"代码**里把子进程的运行现场（trapframe）"摆弄"好即可，子进程**不会**在内核里再去"走一遍"后续的赋值、循环、return 等 C 代码。

## 为什么子进程拿到的 fork() 返回值 0

1. RISC-V 架构中函数返回值保存在 `a0`
2. 这就让子进程的寄存器 `a0` 被强行置成 `0`。
3. 在 Unix 的进程模型中，"子进程"诞生时仅仅是「复制了父进程的地址空间和内核上下文」，自己**还没真正获得 CPU 执行时间**。它只有在内核把它标记为 `RUNNABLE` 之后，调度器才能在将来某个时刻"切换"到子进程，让它**从用户态**跑起来。
4. 等子进程要"返回到用户态"时，会把 `a0` 的值带到子进程的 C 调用栈环境，**从而子进程看到的 `fork()` 返回值是 0**。

也就是说，这些"拷贝内存、设置寄存器、设置返回值"的动作，**本质是父进程在干**，子进程并不需要"亲力亲为"。

因此，仅凭

```c
np->trapframe->a0 = 0;   // 对子进程设置返回值
return pid;             // 对父进程设置返回值
```

就能在父子两个进程中体现不同的返回值，达成 Unix `fork()` 的"一次调用、两次返回"效果。
