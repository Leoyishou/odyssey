---
comment_id: bef85915
date created: 2024-12-25
date modified: 2024-12-27
draw: null
tags: []
title: Lab3 Page tables
---
## ## A kernel page table per process ([hard](https://pdos.csail.mit.edu/6.S081/2020/labs/guidance.html))

### 为什么要对进程的页表做四次映射

```c
  kvmmapkern(pagetable, UART0, UART0, PGSIZE, PTE_R | PTE_W);
  kvmmapkern(pagetable, VIRTIO0, VIRTIO0, PGSIZE, PTE_R | PTE_W);
  kvmmapkern(pagetable, CLINT, CLINT, 0x10000, PTE_R | PTE_W);
  kvmmapkern(pagetable, PLIC, PLIC, 0x400000, PTE_R | PTE_W);

  return pagetable;
```

这四次映射是为了让进程能访问不同的硬件设备，每个映射对应一个不同的设备：
UART0 (Universal Asynchronous Receiver/Transmitter)  
kvmmapkern(pagetable, UART0, UART0, PGSIZE, PTE_R | PTE_W);  
串口通信设备  
用于控制台输入输出  
比如 printf 就需要通过 UART 输出

VIRTIO0  
kvmmapkern(pagetable, VIRTIO0, VIRTIO0, PGSIZE, PTE_R | PTE_W);  
虚拟 I/O 设备接口  
用于磁盘操作  
比如文件读写需要通过这个接口

CLINT (Core Local Interruptor)  
kvmmapkern(pagetable, CLINT, CLINT, 0x10000, PTE_R | PTE_W);

- 处理器本地中断控制器  
用于定时器中断  
比如进程调度需要用到

PLIC (Platform-Level Interrupt Controller)  
kvmmapkern(pagetable, PLIC, PLIC, 0x400000, PTE_R | PTE_W);  
平台级中断控制器  
处理外部设备中断  
比如键盘、磁盘等设备的中断

这些映射是必需的，因为：

1. 每个设备都有其特定的物理地址
2. 内核需要通过这些地址访问设备
3. 没有这些映射，内核就无法与设备通信

就像一个城市需要：

- 港口（UART）用于对外通信
- 仓库（VIRTIO）用于存储
- 警报系统（CLINT）用于定时
- 交通控制（PLIC）用于协调  
每个设备都有其特定用途，缺一不可。
