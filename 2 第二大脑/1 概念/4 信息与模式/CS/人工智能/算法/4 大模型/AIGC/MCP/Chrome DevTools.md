---
uid: 0a2a4c61-31dc-48cb-afe5-b3458b5392c3
date created: 2025-07-19
date modified: 2025-10-04
---
# Summary

你这个截图里的 **Chrome DevTools MCP 插件工具列表**，其实就是通过 MCP（Model Context Protocol）把 Chrome DevTools 的调试能力暴露为一组可调用的“工具函数”。
这类问题本质上是 **“API 能力清单解读”** 的经典问题变体。下面我给你分组解释一下每个工具的作用：

---

## 🔎 页面与元素交互

- **click**：模拟点击元素。
    
- **drag**：拖拽元素。
    
- **hover**：鼠标悬停。
    
- **fill**：在输入框里填入文本。
    
- **fill_form**：批量填写表单。
    
- **upload_file**：模拟用户上传文件。
    
- **handle_dialog**：处理弹窗（alert/confirm/prompt）。
    

---

## 🌐 页面控制与导航

- **new_page**：新建一个标签页。
    
- **close_page**：关闭标签页。
    
- **list_pages**：列出所有打开的页面。
    
- **select_page**：切换到指定页面。
    
- **navigate_page**：跳转到某个 URL。
    
- **navigate_page_history**：在浏览器前进/后退历史中导航。
    
- **resize_page**：调整窗口大小。
    

---

## 📡 网络相关

- **list_network_requests**：列出所有网络请求。
    
- **get_network_request**：获取某个具体请求的详情。
    
- **emulate_network**：模拟网络环境（如 3G、离线等）。
    

---

## 🖥️ 硬件与环境模拟

- **emulate_cpu**：模拟 CPU 速度（比如降速测试性能瓶颈）。
    
- **emulate_network**：模拟带宽、延迟、丢包等情况。
    

---

## 📊 性能分析

- **performance_start_trace**：开始性能追踪。
    
- **performance_stop_trace**：停止性能追踪。
    
- **performance_analyze_insight**：分析性能追踪结果，给出瓶颈和优化建议。
    

---

## 📝 控制台与脚本

- **list_console_messages**：列出 console.log / error 等输出。
    
- **evaluate_script**：直接在页面上下文里执行 JS 脚本。
    

---

## 📷 截图与快照

- **take_screenshot**：截取当前页面截图。
    
- **take_snapshot**：保存页面 DOM/样式快照，便于调试状态。
    

---

## ⏳ 等待控制

- **wait_for**：等待某个元素/条件出现。
    

---

👉 总结来看，这个 MCP 插件等于是把 **Chrome DevTools 的“人工点点点”能力，转成了一个个 API/命令**。这样你就能在对话里、自动化脚本里调用这些工具，做自动化测试、性能分析、页面交互模拟。

要不要我帮你写一个 **典型场景示例**（比如“自动化登录并截图”），把几个工具串起来看看？

# Cues



# Notes
