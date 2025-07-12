---
date created: 2025-03-23
date modified: 2025-07-10
uid: c3a087b7-bf8a-4fb1-97a7-510d1b3c1e0e
---

这属于 **Python虚拟环境（Virtual Environment）的创建和激活操作**。

---

具体解释如下：

| 命令                         | 含义              | 说明                    |
| -------------------------- | --------------- | --------------------- |
| `python -m venv venv`      | 创建名为`venv`的虚拟环境 | 通过venv模块创建独立的Python环境 |
| `source venv/bin/activate` | 激活虚拟环境          | 激活后，安装包会安装到这个环境中      |
| `deactivate`               | 退出虚拟环境          | 完成工作后退出当前虚拟环境         |

---

**用途：**

- **隔离不同项目的Python环境**
- 避免不同项目依赖版本冲突
- 更好地进行项目管理和部署
