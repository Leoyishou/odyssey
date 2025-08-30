---
uid: 7455356a-b1b6-49a2-a6ac-d227ad737715
aliases: [Cues]
linter-yaml-title-alias: Cues
date created: 2025-07-20
date modified: 2025-07-20
---
> 所谓的 source root 感觉就是为了弄清楚命名空间的起点



# Cues

Source Root
# Notes

没错，你的直觉很对：把某个目录标记为 Source Root，本质上就是告诉 IDE（同时也等价于把它加入 PYTHONPATH）——“从这里往下的文件夹，都是命名空间的起点，可以作为顶级包来 [导包机制 import](导包机制%20import.md)。

## 为何必须显式声明“起点”？

|症结|运行期为什么能 import|IDE 为什么还报红|
|---|---|---|
|Python 解释器 在启动时 会把“当前工作目录”自动塞进 `sys.path`（再加上一些 `site‑packages`）|pytest / `python main.py` 往往从项目根启动，所以能找到顶级包|IDE 只会索引它知道的 Interpreter Paths + Source Roots，默认并不知道“你要把哪个文件夹当工作目录”|

> 因此：你一旦标 Source Root，PyCharm 会把对应的绝对路径加入自己的搜索路径，静态分析器就能解析 `from cs336_basics import...`，红线自然消失。

## 一个最小示例

```Java
project/
├── assignment1-basics/      ← 标记为 Source Root
│   └── cs336_basics/
│       ├── __init__.py
│       └── nn.py
└── tests/
    └── adapters.py
```

- 执行侧：`pytest` 从 `project` 跑，解释器把它当作 cwd，所以能 import。
    
- IDE 侧：只有当 `assignment1-basics` 被设成 Source Root，`tests/adapters.py` 里的
    
    ```python
    from cs336_basics.nn import linear
    ```
    
    才能解析到正主。
    

## 小贴士

1. 一个项目可有多个 Source Root——例如 `src/`、`examples/`；IDE 会把它们全部加入搜索。
    
2. 可编辑安装 (`pip install -e.`) 实际做的就是把包路径写进 `site‑packages/…‑dist-info.pth`，效果与标 Source Root 类似，但对命令行、其他 IDE 也通用。
    
3. 若仍报红，可重建索引：`File → Invalidate Caches / Restart…`。
    

总结：Source Root = 显式声明“我的顶级包从这里开始”，既方便 IDE 补全跳转，也避免跨环境跑脚本时因为路径不同而找不到包。
