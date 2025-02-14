---
draw:
title: Throwable@
date created: 2024-08-20
date modified: 2025-02-06
---

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F08%2F20%2F11-27-07-48ce5245af8e69526d54787f51fab89d-202408201127685-ec753b.png)

```mermaid
classDiagram
    Throwable <|-- Error
    Throwable <|-- Exception
    Exception <|-- RuntimeException
    Exception <|-- CheckedException
    Error --> UncheckedException
    RuntimeException --> UncheckedException
    
    class Throwable{
        <<Abstract>>
    }
    class Error{
        <<Error>>
    }
    class Exception{
        +All subclasses except RuntimeException
        are Checked Exceptions
    }
    class RuntimeException{
        <<RuntimeException>>
    }
    class CheckedException{
        <<Checked>>
    }
    class UncheckedException{
        <<Unchecked>>
    }
```

[Throwable@](Throwable@.md)

- [[Error]]（非受检异常）
	- [OOM](OOM.md)
	- [StackOverFlow](StackOverFlow.md)
- [[RuntimeException]]（非受检异常）
- [[CheckedException]]（受检异常）

> [!第一性原理]  
> 受检和不受检，指的是受不受编译器的检查
