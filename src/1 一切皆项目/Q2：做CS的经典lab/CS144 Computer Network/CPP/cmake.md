---
draw:
tags: []
title: cmake
date created: 2024-12-30
date modified: 2024-12-30
---

cmake 预处理，[make](2%20第二大脑/1%20节点/CS/编程语言/C/make.md)实际编译。

`cmake..` 它相当于一个"预处理"步骤，规划好如何编译整个项目  
`..` 表示 CMakeLists.txt 文件在上一级目录

- cmake 会读取 CMakeLists.txt 中的配置，检查依赖，生成相应的构建规则  
类似于在 Java 中使用：
- Maven 的 `mvn generate-sources`
- 或者 Gradle 的 `gradle build -x test`
- 它们都是在"准备构建环境"，解析项目配置（如 pom.xml 或 build.gradle）
- 类似于你在 IDEA 中第一次导入项目时，它要加载和配置项目的过程

`make` 实际执行编译过程，类似于

- Maven 的 `mvn compile`
- 或者在 IDEA 中点击"Build Project"按钮
- 就是真正的编译代码，生成可运行的程序
- 它会根据 cmake 生成的 Makefile 中的规则，调用编译器（g++）
- 编译源文件，链接库文件，最终生成可执行文件
- 它会自动判断哪些文件需要重新编译（根据文件修改时间）

总的来说：

- CMakeLists.txt 就像 Java 的 pom.xml 或 build.gradle
- cmake 就像 Maven/Gradle 这样的构建工具
- make 就像 IDEA 中的编译按钮

一个典型的工作流程是：

```bash
mkdir build      # 创建构建目录
cd build         # 进入构建目录
cmake ..         # 生成构建规则
make             # 执行编译
```

这种方式的好处是：

- 把构建文件与源代码分开（放在 build 目录）
- 便于清理（直接删除 build 目录即可）
- 支持多种构建配置（可以有多个不同的 build 目录）

## cmake 是 cpp 独有的吗

cmake 不是 C++ 独有的，它是一个通用的构建系统生成器，可以用于多种编程语言：

1. 最常用于：
- C
- C++
- Objective-C
- Fortran

1. 也支持其他语言：
- Java
- Python
- Swift
- Rust
- 等等

但在实际开发中：

- C/C++ 项目最常用 cmake
- Java 项目通常用 Maven/Gradle
- Python 项目通常用 pip/setuptools
- Rust 项目通常用 Cargo

cmake 之所以在 C/C++ 中流行，是因为：

1. C/C++ 需要处理复杂的依赖关系
2. 需要跨平台编译
3. 需要管理不同的编译器选项
4. 历史上缺乏统一的包管理工具

所以虽然 cmake 可以用于其他语言，但它主要还是在 C/C++ 生态系统中占主导地位。你在 CS144 中会大量使用它来管理项目构建。
