# 性能测试

README 里的数字来自这里的测试流程。任何人都可以用同一套脚本在自己的机器上复现。

## 环境

| 项 | 值 |
|---|---|
| 日期 | 2026-11-03 |
| 源码 | `0e92b51`，`cargo build --release --locked` |
| 机器 | Apple M5 Pro / 48 GiB，macOS 26.5 |
| 窗口 | 1280 × 800 pt，2× 缩放，Craft 主题 |
| 文档 | [样例生成器](../tools/promo/gen_library.py) 生成的 1,001,307 字符、549 章，含段落、列表、表格和代码 |

## 方法

[harness](../tools/harness/README.md) 回放 [readme-performance.txt](../tools/harness/examples/readme-performance.txt)：打开百万字文档，在首段末尾依次插入 100 个 `x`，随后退格 100 次，再向下滚动 100 次。

`bench` 每发送一次操作后同步调用 `window.draw`，记录这一帧的处理耗时（日志里的 `frame`）。P50 为中位数，P95 为第 95 百分位。它统计应用内的帧处理时间，不包含屏幕呈现延迟，不等同于键盘到屏幕的端到端延迟。这是单机实测，不代表所有设备，也不是和其他应用的对比。

## 结果

| 操作 | 帧处理 P50 | 帧处理 P95 | 最大 |
|---|---:|---:|---:|
| 连续输入 | 5.06 ms | 5.78 ms | 6.94 ms |
| 连续退格 | 4.99 ms | 5.55 ms | 5.69 ms |
| 连续滚动 | 2.08 ms | 4.14 ms | 9.23 ms |

原始输出（ms）：

```text
bench "type x" x100: step [p50 0.03 p95 0.05 max 5.29] frame [p50 5.06 p95 5.78 max 6.94] editor render [p50 3.77 p95 4.31 max 4.99] ms
bench "key backspace" x100: step [p50 0.02 p95 0.05 max 3.91] frame [p50 4.99 p95 5.55 max 5.69] editor render [p50 3.64 p95 4.12 max 4.24] ms
bench "wheel 900 500 40" x100: step [p50 0.01 p95 0.02 max 0.02] frame [p50 2.08 p95 4.14 max 9.23] editor render [p50 0.02 p95 0.03 max 1.83] ms
```

## 复现

测试只使用临时文档库，不会碰 `~/Documents/Quiet.md`。

```sh
cargo build --release --locked
python3 tools/promo/gen_library.py /tmp/quiet-perf/library
printf '{"english":true,"theme":"craft"}\n' > /tmp/quiet-perf/library/preferences.json
python3 tools/harness/run.py tools/harness/examples/readme-performance.txt /tmp/quiet-perf/library /tmp/quiet-perf/run
grep -a bench /tmp/quiet-perf/run/log
```
