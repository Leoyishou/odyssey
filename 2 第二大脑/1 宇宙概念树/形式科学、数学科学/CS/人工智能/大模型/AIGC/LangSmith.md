---
date created: 2025-07-02
date modified: 2025-07-10
uid: 4a4b8764-f846-4309-a03b-6f13899e8aba
---

| 入口 | 核心作用 | 典型使用场景 | 快速上手提示 |
|------|----------|--------------|--------------|
| Trace an Application | **对接代码 → 采集 Trace → 可视化调试**<br>查看每次链调用 / LLM 请求的调用树、耗时、Token 用量、错误栈 | • 本地跑脚本或启动 API 服务<br>• 想定位 Prompt 拼装、Tool 调用或外部依赖的瓶颈 | ① 在代码里 `pip install langsmith`<br>② 设置 `LANGSMITH_TRACING=true` 等环境变量<br>③ 运行程序，后台自动生成 Trace |
| Test Prompts in the Playground | **交互式试 Prompt**，支持切换模型、参数，左边输 Prompt，右边看结果 | • 还在迭代 Prompt，想先手动尝试<br>• 快速对比不同模型 / 温度 | ① 直接点击进入编辑器<br>② 写 System / User Prompt<br>③ 选择模型并运行 |
| Run an Evaluation | **批量跑评测**：把同一数据集交给不同 Prompt/LLM，自动打分并对比 | • 回归测试：新 Prompt 是否退化？<br>• 横向评估：挑选最佳模型 | ① 在 UI 或 SDK 创建 Dataset<br>② 选 Target Function & Evaluator<br>③ 点击 “Run” 生成 Experiments |
| Build and Deploy Agents | **用 LangGraph 设计、托管 Agent**，并连接到 LangSmith 监控 | • 需要多步对话或工具调用的 Agent<br>• 计划把原型部署到生产 | ① 安装 `langgraph`<br>② 编写节点与边，生成 Graph<br>③ 一键部署到 LangGraph Cloud 并连到 Trace |

在 LangSmith 后台查看

1. 打开浏览器进入 https://smith.langchain.com
2. 侧边栏 **Projects → taurus-chat**↳
3. 默认展示 **Traces** 标签页：
    - **Run Tree**：父子节点 + Token/Latency 统计
    - **Inputs / Outputs**：精确还原 prompt & 结果
    - **Errors**：堆栈、重试次数一目了然
4. 若要做整体监控，切到 **Dashboards**：可以改成你自定义的仪表盘或官方「LLM Latency / Token Usage」模板。[docs.smith.langchain.com](https://docs.smith.langchain.com/observability/how_to_guides/dashboards?utm_source=chatgpt.com)[muegenai.com](https://muegenai.com/docs/data-science/building-llm-powered-applications-with-langchain-langgraph/module-7-deployment-scaling/using-langsmith-for-tracing-and-debugging/?utm_source=chatgpt.com)

LangSmith 带来的“Prompt/模型版本控制＋CI”

**“创建数据集 + 跑 Experiment + 对比结果”**  
就是把 Prompt / LLM 的改动转成可量化、可回溯、可视化的实验流程，帮你快速定位最佳方案、避免回归，并为上线提供客观依据。
