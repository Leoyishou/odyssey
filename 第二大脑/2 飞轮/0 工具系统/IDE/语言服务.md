---
uid: b9d717ba-4d3b-46bd-ad32-0e364b16e202
date created: 2025-07-19
date modified: 2025-09-15
---
# Summary

## **常见的语言服务**
- Python → **Pylance**（微软开发的，基于 Pyright，非常快）、Jedi
- Java → **Language Support for Java(TM) by Red Hat**
- Go → **gopls**
- TypeScript/JavaScript → **内置 TypeScript Server**

# Cues



# Notes
你问的这个问题，本质上是**“工具为什么能理解代码” 的经典问题变体**。

---

## **语言服务（Language Server）的含义**

在 [[IDE]] / 编辑器里，**语言服务**指的是一套“理解代码语义”的后台程序。
它负责让编辑器知道：

- 一个变量是在哪里定义的；
- 一个函数需要什么参数；
- 一个类有哪些方法；
- 哪些地方有语法错误；
- 如何做智能补全、跳转、重构。

编辑器本身（像 VS Code、Cursor）只是个“外壳”，它**不懂 Python/Java 语法**。真正理解代码的就是 **语言服务**。

---

## **典型的语言服务协议：LSP**

业界有一个标准叫 **LSP（Language Server Protocol）**：
- **Client**：编辑器（VS Code、Cursor、NeoVim…）
- **Server**：语言服务（比如 Python 的 Pylance，Java 的 RedHat Java Language Server）

编辑器把你在界面上的操作（比如“点一个类名”）发给语言服务，语言服务返回“这个类定义在某个文件的第 42 行”。于是编辑器就能帮你跳转。

---

## **举个例子**

假设你写的是 Python：
- **没有语言服务** → 编辑器只会把代码当成普通文本，没法识别 import，也不能跳转。
- **有 Pylance 语言服务** →
    - 你输入 `os.pa` → 自动提示 `path`；
    - 你点 `os.path` → 能跳转到源码；
    - 写错 `pritn` → 会被标红提示语法错误。

---

## **常见的语言服务**
- Python → **Pylance**（微软开发的，基于 Pyright，非常快）、Jedi
- Java → **Language Support for Java(TM) by Red Hat**
- Go → **gopls**
- TypeScript/JavaScript → **内置 TypeScript Server**
    

---

👉 所以你在 Cursor 里遇到的问题，其实就是：**Python 的语言服务没启动，而 Java 的语言服务启动了**，导致编辑器表现得像是个 Java IDE。
