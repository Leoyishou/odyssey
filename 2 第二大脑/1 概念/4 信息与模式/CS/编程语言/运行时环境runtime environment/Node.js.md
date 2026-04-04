---
date created: 2025-04-08
date modified: 2025-07-10
uid: 63668776-10ba-46e8-93b0-4047c5c409a5
---
# Summary

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2026%2F02%2F03%2F17-42-34-b4a1a41cd2a22946674157049b41dd31-202602031742228-87456d.png)

> [!info]
> JavaScript 是编程语言，Node.js 是运行环境。

更精确的类比：
Java : JVM $\approx$ JavaScript : V8
Node.js $\approx$ JVM + java.io + java.net + ...（打包成一个运行时）

# Cues

[[fnm 版本管理器]]


# Notes


| 维度 | Java + JVM | JavaScript + Node.js |
| :----------- | :-------------- | :------------------------------- |
| 语言 | Java | JavaScript |
| 运行时 | JVM | V8 引擎 (Node.js 的核心) |
| 编译 | 字节码 (.class) | 无（直接解释/JIT） |
| 运行环境数量 | 只有 JVM | 浏览器 + Node.js + Deno + Bun... |

关键区别：

1.  Java 只能在 JVM 上跑，而 JavaScript 有多个独立的运行环境（浏览器、Node.js、Deno 等）
2.  Node.js $\neq$ 纯粹的 JS 引擎。Node.js = V8 引擎 + 系统级 API（文件、网络等）。更准确的类比：
    -   V8 $\approx$ JVM（纯执行引擎）
    -   Node.js $\approx$ JVM + Java 标准库 + 系统绑定
3.  浏览器也能跑 JS，但不能跑 Java（除非用插件）

## 系统 API

Node.js 的"系统 API"是指底层操作系统（Linux/macOS/Windows）提供的能力，Node.js 通过 C++ 绑定将这些暴露给 JavaScript：

| Node.js 模块 | 对应的系统能力 |
| :------------ | :----------------------------------------- |
| fs | 文件系统调用（open, read, write, stat...） |
| http/net | 网络 socket（TCP/UDP） |
| child_process | 进程管理（fork, exec, spawn） |
| os | 操作系统信息（CPU、内存、用户） |
| path | 文件路径处理 |
| crypto | 加密库（OpenSSL） |

类比 Java：
Node.js 的 fs $\approx$ Java 的 java.io / java.nio
Node.js 的 http $\approx$ Java 的 java.net
Node.js 的 child_process $\approx$ Java 的 ProcessBuilder

为什么浏览器没有这些？

浏览器出于安全考虑，不能让网页直接访问用户的文件系统、启动进程。所以浏览器里的 JS 只有 DOM、fetch、localStorage 这些"沙盒内"的 API。

---

类比：
-   JavaScript $\approx$ 中文
-   Node.js $\approx$ 能说中文的场所（比如中国）
-   浏览器 $\approx$ 另一个能说中文的场所（比如新加坡）

具体关系：

| 维度 | JavaScript | Node.js |
| :------- | :-------------------- | :----------------------- |
| 本质 | 语言规范 (ECMAScript) | 运行时 (Runtime) |
| 运行位置 | 浏览器 / Node.js | 服务器 / 本地机器 |
| 能做什么 | 语法、逻辑、数据处理 | 文件读写、网络、进程管理 |

Node.js 额外提供的能力（浏览器没有）：
-   fs - 文件系统操作
-   child_process - 子进程（如 spawn、exec）
-   http/https - 创建服务器
-   path - 路径处理
-   os - 操作系统信息

所以 spawn() 和 unref() 是 Node.js 的 child_process 模块提供的，不是 JavaScript 语言本身的功能。浏览器里的 JavaScript 没有这些 API。

确实，"Node.js"这个名称容易让人误解为一门编程语言，这主要有几个原因：

1.  **命名方式相似**：许多编程语言都有".js"、".py"等后缀，如JavaScript、TypeScript、CoffeeScript，所以Node.js的命名格式类似于这些语言。
2.  **提法习惯**：人们常说"我用Node开发"或"这是Node项目"，就像说"我用Python开发"一样，这种表达方式容易混淆。
3.  **技术栈表述**：在招聘和技术讨论中，Node.js经常与编程语言并列出现（"精通Java、Python、Node.js"），强化了这种误解。
4.  **独特生态系统**：Node.js有自己的包管理器(npm)、特有API和开发模式，使其感觉像一个独立的编程平台而非仅仅是运行时。
5.  **身份多重性**：它既是运行时环境，也是一个平台和生态系统，这种多重身份增加了理解难度。

更准确的理解是：Node.js是一个让JavaScript能在服务器端运行的环境，而JavaScript才是实际使用的编程语言。这类似于说"我们使用JVM开发"，而实际上是用Java语言在JVM环境中开发。

---

是的，正式的学术或技术称谓上，Node.js 就是一个"JavaScript 运行时环境"(JavaScript runtime environment)。

这个术语"运行时环境"在计算机科学中是指一个程序或代码在执行时所需的所有东西的集合，包括：

-   代码执行引擎
-   标准库
-   内存管理
-   I/O 操作接口
-   其他支持程序运行所需的基础设施

对于 Node.js 来说，它使用了 Google Chrome 的 V8 JavaScript 引擎作为其核心执行引擎，然后围绕这个引擎构建了一整套服务器端功能的生态系统，让 JavaScript 能够在浏览器之外的环境中运行。

所以从技术角度来说，"JavaScript 运行时环境"确实是 Node.js 最准确的描述。