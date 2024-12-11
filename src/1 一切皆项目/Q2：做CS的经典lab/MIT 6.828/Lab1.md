如果你已经成功安装并配置好实验环境，并且代码也准备好了（如已完成下面这些步骤）：

1. 克隆 xv6-labs-2021 仓库并切换到 `util` 分支：
    
    ```bash
    git clone git://g.csail.mit.edu/xv6-labs-2021
    cd xv6-labs-2021
    git checkout util
    ```
    
2. 确保工具链（riscv64-unknown-elf-gcc、qemu 等）已正确安装。
    

那么快速启动并看到 lab1 效果的方式是：

1. 在 xv6-labs-2021 目录下运行：
    
    ```bash
    make qemu
    ```
    
    这会编译 xv6，并在 QEMU 仿真器中启动 xv6 系统。
    
2. 当你看到类似以下输出时：
    
    ```
    xv6 kernel is booting
    init: starting sh
    $
    ```
    
    表示 xv6 已经运行并进入 shell。
    
3. 此时你可以在 xv6 的 shell 中输入命令（如 `ls`、`cat`、`echo`）查看效果。例如：
    
    ```bash
    $ ls
    ```
    
    会列出当前文件系统中的文件，如 README、cat、echo、sh 等。
    
4. 若你已完成 lab1 中的部分修改（如实现了 `sleep` 程序），只需要在本地编译完成后再次运行：
    
    ```bash
    make qemu
    ```
    
    启动 QEMU，就能在 xv6 shell 中执行：
    
    ```bash
    $ sleep 10
    ```
    
    等待片刻后，命令将返回 shell 提示符。这说明你的 `sleep` 程序已生效。
    
5. 如果想要测试你的程序是否满足 lab 的要求，可以运行：
    
    ```bash
    make grade
    ```
    
    或使用指定测试项的命令（根据文档说明）来查看你实现的功能是否通过测试。
    

总结来说，只需在 xv6-labs-2021 目录中 `make qemu` 启动虚拟机系统，然后在 xv6 shell 内执行相应的命令，即可快速看到你的 lab1 改动所带来的效果。