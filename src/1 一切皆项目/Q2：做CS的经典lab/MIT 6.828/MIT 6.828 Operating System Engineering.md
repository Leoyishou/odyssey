## 配环境
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F12%2F11%2F20-16-18-49f660603551ec40cb3d2de84564db4c-202412112016393-088440.png)

| 命令行                                                                                                                        | 作用                        | 说明                                                                                                                                                                                                                                                                                                |
| :------------------------------------------------------------------------------------------------------------------------- | :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `sudo apt-get install git build-essential gdb-multiarch qemu-system-misc gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu` | 安装基础编译与开发工具链              | 通过 APT 包管理器以管理员权限（sudo）安装：<br>- **git**：版本控制工具<br>- **build-essential**：编译所需的基础工具（如 gcc、g++、make）<br>- **gdb-multiarch**：支持多种架构的 GDB 调试器<br>- **qemu-system-misc**：QEMU 仿真器的一组工具，用于模拟多种硬件架构<br>- **gcc-riscv64-linux-gnu**：RISC-V 架构交叉编译的编译器<br>- **binutils-riscv64-linux-gnu**：RISC-V 架构的二进制工具集 |
| `sudo apt-get remove qemu-system-misc`                                                                                     | 移除已安装的 qemu-system-misc 包 | 在需要安装特定版本的 QEMU 之前，先移除已有的 qemu-system-misc 版本，以防冲突。                                                                                                                                                                                                                                               |
| `sudo apt-get install qemu-system-misc=1:4.2-3ubuntu6`                                                                     | 安装指定版本的 qemu-system-misc  | 利用 apt-get 安装特定版本（`1:4.2-3ubuntu6`）的 qemu-system-misc，以满足特定的环境要求和依赖关系，确保后续实验顺利运行。                                                                                                                                                                                                                 |


## 找好的代码答案

参考思路：
https://blog.miigon.net/categories/mit6-s081/


MIT 6.S081 2020 操作系统 实验环境搭配详解及问题处理 - 北海草鱼的文章 - 知乎  
https://zhuanlan.zhihu.com/p/331492444

MIT6.828- 神级 OS 课程 - 要是早遇到，我还会是这种 five 系列 - 一丁点儿的文章 - 知乎  
https://zhuanlan.zhihu.com/p/74028717

notebook：
https://notebooklm.google.com/notebook/0d26bbfc-c5d1-4ff3-9bb0-3823aa2d956c?_gl=1*1j7d54k*_ga*MTk1MTE0NzAxMC4xNzI3NTc3NTUw*_ga_W0LDH41ZCB*MTczMzkyNDYzMC4yLjEuMTczMzkyNDYzMC42MC4wLjA.
参考书籍：
https://pdos.csail.mit.edu/6.828/2021/xv6/book-riscv-rev2.pdf
参考代码：
https://github.com/PKUFlyingPig/MIT6.S081-2020fall
课程原视频：
https://www.youtube.com/watch?v=J3LCzufEYt0&list=PLTsf9UeqkReZHXWY9yJvTwLJWYYPcKEqK
官方地址：
https://pdos.csail.mit.edu/6.S081/2020/