---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-07-21
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# Summary

是个 [[Python]] 测试框架


# Cues



# Notes

好的，我们来简单介绍一下 `pytest` 这个 Python 测试框架。

## 什么是 `pytest`？

`pytest` 是一个非常流行、功能强大且易于使用的 **Python 测试框架**。它旨在让编写和运行各种规模的测试变得简单、高效。无论是简单的单元测试、复杂的集成测试还是功能测试，`pytest` 都能很好地胜任。

## 为什么选择 `pytest`？（主要优点）

1. **简单易上手：**
    
    - 你不需要继承任何特殊的测试类。普通的 Python 函数就可以作为测试函数。
        
    - 测试文件和测试函数的命名约定也很直观（通常以 `test_` 开头）。
        
2. **功能强大：**
    
    - **自动发现测试：** 只要按照约定命名，`pytest` 就能自动找到并运行你的测试。
        
    - **Fixtures（夹具）：** 这是 `pytest` 的一大亮点。它们提供了一种非常灵活、可重用的方式来设置测试环境（例如，创建数据库连接、临时文件、模拟对象）并在测试结束后进行清理。
        
    - **参数化测试：** 可以用不同的输入数据多次运行同一个测试函数，非常适合测试多种边界条件。
        
    - **断言重写：** 当你的 `assert` 语句失败时，`pytest` 会提供非常详细和有用的错误信息，告诉你具体是什么地方出了问题，而不是仅仅抛出一个 `AssertionError`。
        
    - **丰富的插件生态系统：** 有大量的第三方插件可以扩展 `pytest` 的功能，例如用于测试覆盖率、并发执行、报告生成等。
        
    - **详细的报告：** 运行测试后，`pytest` 会输出清晰的测试结果摘要。
        

## 如何使用 `pytest`？（基本步骤）

1. 安装：
    
    你只需要用 pip 安装它：
    
    Bash
    
    ```Java
    pip install pytest
    ```
    
2. 编写测试文件：
    
    创建一个 Python 文件，通常以 test_ 开头（例如 test_example.py）。
    
    在这个文件中，编写以 test_ 开头的函数。这些函数就是你的测试用例。
    
    **`test_example.py` 示例：**
    
    Python
    
    ```Java
    # 假设这是你要测试的函数
    def add(a, b):
        return a + b
    
    def subtract(a, b):
        return a - b
    
    # 这是你的测试函数
    def test_add_positive_numbers():
        assert add(2, 3) == 5
    
    def test_add_negative_numbers():
        assert add(-1, -1) == -2
    
    def test_subtract_numbers():
        assert subtract(5, 2) == 3
    
    def test_failing_example():
        # 这个测试会故意失败，展示pytest的错误报告
        assert add(1, 1) == 3
    ```
    
3. 运行测试：
    
    打开你的终端或命令行，进入到 test_example.py 文件所在的目录，然后直接输入 pytest 命令并回车：
    
    Bash
    
    ```Java
    pytest
    ```
    

## 运行结果示例（可能看起来像这样）

```Java
============================= test session starts ==============================
platform ... -- Python ...
plugins: ...
collected 4 items

test_example.py ....F                                                    [100%]

=================================== FAILURES ===================================
______________________________ test_failing_example ______________________________

    def test_failing_example():
>       assert add(1, 1) == 3
E       assert 2 == 3
E        +  where 2 = add(1, 1)

test_example.py:21: AssertionError
=========================== short test summary info ============================
FAILED test_example.py::test_failing_example - assert 2 == 3
========================= 1 failed, 3 passed in 0.05s ==========================
```

你会看到 `pytest` 自动找到了 4 个测试，其中 3 个通过，1 个失败。对于失败的测试，它提供了非常详细的错误信息（`assert 2 == 3` 和 `where 2 = add(1, 1)`），这比普通的 `AssertionError` 要有用得多。

**总结：**

`pytest` 是 Python 社区中非常受欢迎的测试工具，它通过其简洁的语法和强大的功能，极大地简化了测试的编写、管理和执行过程，是 Python 项目开发中推荐使用的测试框架。
