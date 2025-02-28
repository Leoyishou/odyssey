---
draw:
tags: []
title: Lab1 Unix utilities
date created: 2024-12-11
date modified: 2025-01-25
---

## 任务

| 测试项目                       | 状态  | 分数    | 问题描述                      |
| -------------------------- | --- | ----- | ------------------------- |
| sleep, no arguments        | 通过  | 5/5   | 无                         |
| sleep, returns             | 通过  | 5/5   | 无                         |
| sleep, makes syscall       | 通过  | 10/10 | 无                         |
| pingpong                   | 通过  | 20/20 | 无                         |
| primes                     | 失败  | 0/20  | exec 失败，未输出素数             |
| find, in current directory | 通过  | 10/10 | 无                         |
| find, recursive            | 通过  | 10/10 | 无                         |
| xargs                      | 失败  | 0/19  | 'hello' 出现次数错误（期望3次，实际0次）|
| time                       | 失败  | 0/1   | 缺少 time.txt 文件            |

## 知识点

1. `exit(0)`:

- 表示程序正常退出
- 是约定俗成的成功状态码
- 通常意味着程序完成了它的任务且没有遇到错误

1. `exit(1)` (或任何非零值):

- 表示程序异常退出
- 是错误状态码
- 通常用于表示程序遇到了错误或异常情况

## 思想

[用户态、内核态](用户态、内核态.md)

## 在[Makefile](Makefile.md)添加条目

在 MIT 6.828 的 Lab1 中，你需要在 Makefile 里加入一些条目（targets、规则和变量）来编译和打包你要在 xv6 操作系统里运行的用户态程序。这么做的原因是：

1. **统一编译流程**：Makefile 是一个自动化编译工具。在 Lab1 里，你不光有内核代码（kernel code），也有用户态的测试程序（比如 pingpong、primes、xargs 等）。如果不在 Makefile 中指定这些程序的编译规则，`make` 就不会自动帮你编译、链接这些用户程序，也无法将它们打包进 xv6 的文件系统镜像中。换句话说，Makefile 里的改动让编译器和链接器知道该如何处理、产出对应的可执行文件。
2. **生成可执行文件和镜像文件系统**：xv6 的用户程序需要在实验给的 "用户态文件系统镜像" 中出现，这样系统在启动后才能从内核的 shell 中直接运行这些程序。Makefile 中增加的新内容会规定：
    - 如何从 `.c` 源文件生成可执行文件（如 `xargs`、`pingpong` 等）。
    - 如何将编译好的用户程序打包进 xv6 的文件系统镜像（`fs.img`）。
   没有这些信息，`make` 不会把用户程序放进系统镜像里，启动后你就无法在 xv6 中运行这些命令。

3. **实验要求与流程**：MIT 6.828 实验要求你不断丰富 xv6 的功能。早期的 Lab（如 Lab1）就会让你添加一些简单的用户态程序或工具，以测试、展示或检验你对内核和系统调用的理解。这些工具如果不通过 Makefile 进行适当配置，编译系统不清楚要处理哪些文件、生成什么目标文件，以及如何将其整合到可用的文件系统中。

总结来说，给 Makefile 增加相应条目是为了让你的用户态代码自动被编译、链接，并打包进 xv6 的镜像，从而在 xv6 环境中正常运行。这与 Java 项目中使用 `javac` 编译、`jar` 打包的流程是类似的，只不过这里是 C 程序和系统镜像的生成，而 Makefile 起到的作用有点类似于自动化构建脚本，让整个构建和打包过程自动化、可重复、可维护。

## 具体实现

### argv 和 argv 是什么的简称

argc 和 argv 是 C 语言中 main 函数的标准参数：

- argc = "argument count"
- 表示命令行参数的数量
- 至少为1（因为程序名本身就是第一个参数）
- argv = "argument vector"
- 是一个指向字符串数组的指针
- 存储所有的命令行参数
- argv[0] 是程序名
- argv[1] 到 argv[argc-1] 是实际的命令行参数

让我帮你分析一下 Lab1 Unix utilities 的实现思路。根据文档，这个 lab 包含几个主要任务:

1. sleep (easy)
2. pingpong (easy)
3. primes (moderate)
4. find (moderate)
5. xargs (moderate)

让我们一个一个来实现:

## 1. sleep (easy)

这是最简单的任务,需要实现一个类似 UNIX sleep 命令的程序。实现步骤:

1. 创建文件 `user/sleep.c`:

```c
#include "kernel/types.h"
#include "kernel/stat.h"
#include "user/user.h"

int main(int argc, char *argv[]) {
    if(argc != 2) {
        fprintf(2, "Usage: sleep ticks\n");
        exit(1);
    }
    
    int ticks = atoi(argv[1]);
    sleep(ticks);
    exit(0);
}
```

1. 在 Makefile 的 UPROGS 中添加 _sleep

### 1. 为什么 argc 要等于 2?

- argc 表示命令行参数的数量，包括程序名称
- 当我们运行 `sleep 10` 时:
  - argv[0]是 "sleep" (程序名)
  - argv[1]是 "10" (参数)
  - 所以总共应该有 2 个参数
- 如果 argc!= 2，说明用户输入格式不对，比如:
  - `sleep` (缺少参数，argc = 1)
  - `sleep 10 20` (参数过多，argc = 3)

### 2. 为什么要用 atoi 而不直接用 argv?

- argv 中的参数都是字符串类型 (char*)
- sleep 系统调用需要的是整数类型的 ticks
- atoi 函数将字符串转换为整数，比如:
  - "10" -> 10
  - "100" -> 100

### 3. 为什么两个分支都需要 exit?

- exit(1) 表示程序异常退出
  - 用于参数错误的情况
  - 返回非零值表示错误
- exit(0) 表示程序正常退出
  - 用于程序完成正常功能后退出
  - 返回 0 表示成功
- 如果不调用 exit:
  - 程序会继续执行到 main 函数结束
  - 可能导致未定义行为
  - 父进程无法知道子进程是否正确执行

### 4. 为什么需要 include 这三个

有了他们，下面的代码才拥有 1.使用基本数据类型 2.使用[[文件系统]] 3.调用[[system call]]的能力

```c
// 定义了基本数据类型，比如:uint (无符号整型)uint64 (64位无符号整型)uchar (无符号字符) 这些是系统编程常用的类型定义

#include "kernel/types.h"

// 定义了文件系统相关的结构体和常量, 包含文件状态信息(比如文件大小、创建时间等), 虽然在 sleep 程序中没有直接使用，但这是 xv6 用户程序的标准包含文件

#include "kernel/stat.h"

//定义了用户程序可以使用的系统调用接口, 包含了 sleep、fork、exit 等函数的声明

#include "user/user.h"
```

## 2. pingpong (easy)

需要实现一个使用管道在父子进程间传递字节的程序:

```c
#include "kernel/types.h"
#include "kernel/stat.h"
#include "user/user.h"

int main() {
    int p1[2], p2[2]; // Two pipes
    char buf[1];
    
    pipe(p1);
    pipe(p2);
    
    if(fork() == 0) {
        // Child process
        close(p1[1]);
        close(p2[0]);
        
        read(p1[0], buf, 1);
        printf("%d: received ping\n", getpid());
        
        write(p2[1], "x", 1);
        exit(0);
    } else {
        // Parent process  
        close(p1[0]);
        close(p2[1]);
        
        write(p1[1], "x", 1);
        read(p2[0], buf, 1);
        printf("%d: received pong\n", getpid());
        
        wait(0);
        exit(0);
    }
}
```

### 为啥 fork() == 0是子进程反之是父

fork() 的特点：调用一次，返回两次

- 在父进程中返回子进程的 PID（大于 0）
- 在子进程中返回 0

### read 和 write 方法的第三个参数啥意思，为啥都是 1

在 read 和 write 函数中，第三个参数表示要读取或写入的字节数。

```c
// 从文件描述符 fd 读取最多 n 个字节到 buf
ssize_t read(int fd, void *buf, size_t n);

// 向文件描述符 fd 写入 buf 中的 n 个字节
ssize_t write(int fd, const void *buf, size_t n);
```

## 3. find (moderate)

实现一个简单版本的 UNIX find 命令。根据文档([1](https://pdos.csail.mit.edu/6.S081/2020/labs/util.html))的提示:

1. 参考 `user/ls.c` 来了解如何读取目录
2. 使用递归来遍历子目录
3. 不要递归进入 "." 和 ".."
4. 使用 strcmp() 来比较字符串

```c
#include "kernel/types.h"
#include "kernel/stat.h"
#include "user/user.h"
#include "kernel/fs.h"

void find(char *path, char *name) {
    char buf[512], *p;
    int fd;
    struct dirent de;
    struct stat st;

    if((fd = open(path, 0)) < 0) {
        fprintf(2, "find: cannot open %s\n", path);
        return;
    }

    if(fstat(fd, &st) < 0) {
        fprintf(2, "find: cannot stat %s\n", path);
        close(fd);
        return;
    }

    switch(st.type) {
        case T_FILE:
            if(strcmp(path + strlen(path) - strlen(name), name) == 0) {
                printf("%s\n", path);
            }
            break;
        case T_DIR:
            if(strlen(path) + 1 + DIRSIZ + 1 > sizeof buf) {
                printf("find: path too long\n");
                break;
            }
            strcpy(buf, path);
            p = buf+strlen(buf);
            *p++ = '/';
            while(read(fd, &de, sizeof(de)) == sizeof(de)) {
                if(de.inum == 0)
                    continue;
                if(strcmp(de.name, ".") == 0 || strcmp(de.name, "..") == 0)
                    continue;
                memmove(p, de.name, DIRSIZ);
                p[DIRSIZ] = 0;
                find(buf, name);
            }
            break;
    }
    close(fd);
}

int main(int argc, char *argv[]) {
    if(argc != 3) {
        fprintf(2, "Usage: find path name\n");
        exit(1);
    }
    find(argv[1], argv[2]);
    exit(0);
}
```

我建议按照这个顺序逐个实现,每完成一个就测试一下。文档中提供了详细的测试用例([1](https://pdos.csail.mit.edu/6.S081/2020/labs/util.html))。

需要我继续说明 xargs 和 primes 的实现吗？
