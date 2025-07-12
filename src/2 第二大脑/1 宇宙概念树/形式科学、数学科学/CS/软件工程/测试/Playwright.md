---
date created: 2025-03-13
date modified: 2025-07-10
uid: fd28c536-b228-4ebc-bb7d-64adf0621067
---
**Playwright** 是一个由微软开发的开源的浏览器自动化测试框架，专门用来做网页和前端自动化测试。

---

## 它能做什么？

- **自动化测试**
    
    - 用代码模拟真实用户，自动执行网页点击、填写表单、上传文件、截图等操作。
- **跨浏览器兼容性测试**
    
    - 同时支持 Chromium（Chrome、Edge）、Firefox 和 WebKit（Safari），可以轻松检查网页在不同浏览器上的表现。
- **网页抓取（Web Scraping）**
    
    - 快速便捷地获取动态加载网页数据。
- **自动化任务执行**
    
    - 比如自动提交表单、自动监测网页变化、自动登录等。

---

## 特点和优势

1. **跨浏览器支持**
    
    - 支持三大主流浏览器：Chromium、Firefox、WebKit。
2. **高效易用的API**
    
    - 提供简单易读的JavaScript、TypeScript、Python、Java、C#接口。
3. **自动等待机制**
    
    - 内置智能等待，比如自动等待元素加载完成，不需要手动写等待逻辑。
4. **支持无头（Headless）模式**
    
    - 可以在后台执行，无需打开浏览器窗口。
5. **强大的调试能力**
    
    - 提供交互式的调试工具，可以实时监控网页状态、查看快照等。

---

## Playwright 的基本使用示例（以Python为例）

```python
# 安装
# pip install playwright
# playwright install

from playwright.sync_api import sync_playwright

with sync_playwright() as p:
    browser = p.chromium.launch(headless=False)  # 启动浏览器（非无头模式）
    page = browser.new_page()                   # 新建页面
    page.goto("https://example.com")            # 打开网页
    print(page.title())                         # 打印网页标题
    page.screenshot(path="example.png")         # 截图
    browser.close()                             # 关闭浏览器
```

---

## 和其他工具的比较

|功能/特性|Playwright|Selenium|Puppeteer|
|---|---|---|---|
|跨浏览器支持|✅优秀（内置支持）|✅（需单独安装驱动）|❌（只支持Chrome）|
|无头模式|✅|✅|✅|
|易用性|✅ 非常简单易懂|❌ 相对复杂|✅ 简单易懂|
|自动等待|✅ 自动处理|❌ 手动设置等待|✅ 较完善|
|执行速度|✅ 较快|❌ 较慢|✅ 较快|
|开发者|微软|社区开源|谷歌|

- **Playwright** 相比 **Selenium** 更易于上手，性能更高。
- 相比 **Puppeteer**，Playwright 提供了跨浏览器能力，适合更多测试场景。

---

## 总结

如果你想做跨浏览器的自动化测试或网页抓取，**Playwright** 是目前非常优秀且流行的选择之一。
