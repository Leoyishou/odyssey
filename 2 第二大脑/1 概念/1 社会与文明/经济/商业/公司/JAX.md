---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-25
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  

和 TensorFlow / PyTorch 的直觉对比：
- JAX：更 NumPy、更函数式，变换（`grad/jit/vmap/pjit`）是“第一公民”，在 TPU 上体验突出。
- PyTorch：动态图、生态与工程工具链更成熟（推理部署、ONNX、社区模型多）。
- TensorFlow：图/静态执行与企业部署链条强，但研究端近年更多人转向 JAX 或 PyTorch。



JAX 是 Google Research 推出的**高性能数值计算与自动微分库**。它有三件“看家本领”：

1. **NumPy 风格**：API 和 `numpy` 几乎一致（用的是 `jax.numpy`），上手成本低。
    
2. **可组合的程序变换**：
    

- `jax.grad` 自动求导
    
- `jax.jit` 一键加速（用 XLA 编译到 CPU/GPU/TPU）
    
- `jax.vmap` 自动向量化（把“对一个样本的函数”变成“对一批样本的函数”）
    
- `jax.pmap` / `jax.pjit` 跨多卡/多机并行与分片
    

3. **纯函数/函数式风格**：强调不可变状态，更利于推理、并行与编译优化。
    

为什么研究和大模型爱用它：

- **快**：`jit` + XLA 编译，算子融合、内存优化到位。
    
- **简**：把“写清楚数学函数”交给你，把“怎么并行、怎么编译”交给 JAX 的变换器。
    
- **大规模**：`pmap/pjit` + SPMD 分片，天然适合 TPU/GPU 集群训练。
    
- **生态**：常见搭子包括
    
    - 模型框架：**Flax**、**Haiku**、**Equinox**
        
    - 优化器：**Optax**
        
    - 概率编程：**NumPyro**
        
    - 图与工具：**Jraph**、**Chex**、**Orbax** 等  
（例如 AlphaFold、T5X/Pax 等研究系统都基于 JAX/其生态的变体。）
        


# 🧩 Cues



# 🪞Notes
