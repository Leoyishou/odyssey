---
draw:
title: TDD
date created: 2024-01-18
date modified: 2025-04-05
---
**代码实现的留痕似乎不如测试过程的留痕更有意义**

## Spring Boot applications are typically tested at three main levels

1. Unit Testing: This level involves testing individual components or classes in isolation. You can use frameworks like JUnit or TestNG for unit tests.
2. Integration Testing: Integration tests verify the interactions between different parts of your application, such as testing how various components work together. Spring provides tools like `@SpringBootTest` for integration testing.
3. End-to-End (E2E) Testing: E2E tests validate the entire application, simulating real user scenarios. Tools like Selenium and Cucumber are often used for E2E testing.

## Strategies for Effective Testing

1. **Test-Driven Development (TDD):** Consider adopting TDD, where you write tests before implementing the corresponding code. This approach helps define clear requirements and encourages modular code design.
2. **Use Mocking:** In unit and integration tests, use mocking frameworks like Mockito to simulate dependencies and interactions with external systems.
3. **Test Data Management:** Use libraries like Testcontainers or tools like Spring Test DBUnit to manage test data and databases for integration tests.
4. **Parameterized Tests:** Leverage parameterized tests to run the same test with different inputs, ensuring comprehensive coverage.
5. **Continuous Integration (CI):** Integrate testing into your CI/CD pipeline, running tests automatically on code changes to catch issues early.
6. **Code Coverage Analysis:** Employ code coverage tools like JaCoCo to measure the percentage of code covered by tests and identify untested areas.

## Best Practices

1. **Keep Tests Isolated:** Tests should not rely on the state of other tests. Each test should be independent and self-contained.
2. **Naming Conventions:** Follow a consistent naming convention for test methods to improve readability and understanding.
3. **Documentation:** Write descriptive test case names and add comments where necessary to clarify test intentions.
4. **Maintainable Tests:** Avoid duplicating test code. Create utility methods and helper classes to keep tests maintainable.
5. **Regular Updates:** As your [application](https://www.sayonetech.com/services/webapp-development-company-usa/) evolves, update and maintain your tests to ensure they remain relevant and accurate.

## Tools for Testing

1. **JUnit:** The de facto standard for writing unit tests in Java.
2. **Spring Boot Test:** Spring Boot provides a suite of test annotations and utilities to simplify testing.
3. **Mockito:** A popular mocking framework for simulating dependencies in tests.
4. **Testcontainers:** A library for managing Docker containers in tests, useful for integration testing.
5. **Selenium:** An E2E testing framework for web applications.
6. **Cucumber:** A tool for behavior-driven development (BDD) that helps create readable, executable specifications.

我已为老师想好下一个专栏的名称《TDD 项目实战》，很多人一提到 TDD 第一感觉就是听过、没见过。耗子叔早年写过《TDD 并不是看上去的那么美》https://coolshell.cn/articles/3649.html，我对 TDD 还是很感兴趣的，一直自己摸索感觉还是不得其法。老师在极客时间的其实为 TDD 做了不少铺垫，是时候综合运用一下来一个中型的实践项目，把 ThoughtWorks 的 TDD 实战经验在实际项目中展示一下。

以下是我自我摸索出的想要玩转 TDD 需要的能力/技能：

1. 需求分解《10X 程序员工作法》-- 解决测试用例从哪里来的问题？
2. 设计能力《软件设计之美》-- 设计/抽象/建模让产品代码更具可测试性，同时完成第一创造
3. 测试能力《程序员的测试课》--- 扬长避短，写出好的测试代码，别让测试成为负资产
4. 编程能力 --- 熟练掌握所用语言的核心语法、常用标准库、第三方库等（含数据结构与算法），能写出符合该语言规范的代码，判断标准：在 TDD 循环中尽量不卡壳、少用 Google 减少打断循环的次数。
5. 转换能力 -- TDD 循环是在两个思维层次来回跳转，写测试时抽象需求 What，写代码时具体细节 How
6. 重构能力《代码之丑》-- 识别坏味道，运用恰当重构手法提高代码质量
7. 工具能力 -- 熟练操作 Git、IDE、及其他自动化工具，使其能跟上 TDD 的节奏  
（比如，编写测试时，产品代码还没实现但写出其接口后可以利用 IDE 快捷键自动生成产品代码的框架。重构时，运用 IDE 提供的重构快捷键，快速修改名称、抽取方法、内联函数等。完成几次 TDD 循环后可以用 Git 提交，设置 " 游戏存档 " 随时重新开始，如果一切顺利合并多个提交为一个然后提交）
 

最后就是各种场景实战，小型场景练习并熟练上述技能，中型场景强化上述技能的同时，培养在工期压力下完成任务的能力，大型场景就是在实际工作中实践。

我没把 TDD 当银弹，希望能达到 " 对于同一个业务功能，大多数人用 X 天完成产品代码然后提测，我同样用 X 天完成产品代码和测试代码然后提测 " 的标准就可以了。希望这种工作方式能给我带来更多自信（对代码也对自己）。谢谢！
