---
date created: 2025-03-24
date modified: 2025-09-20
uid: 4e527a8d-9fac-4f78-aaf4-15c7b71e501f
---
# Summary

本质是[Transformers](Transformers.md)这种通用推理框架是针对所有[深度学习@](深度学习@.md)任务的，而 vllm 针对[LLM大模型](LLM大模型.md)这种特殊任务做了特殊优化，所以更快。
1. 自动批处理
2. [GPU Kernel 内核](GPU%20Kernel%20内核.md)
3. [PagedAttention](PagedAttention.md) 分页注意力

# Cues

你这个是**“为什么后端从 FlashAttention-2 降级到 xFormers 的经典问题变体”**。

简明答案：**因为你用的是 T4（Turing，SM_75）**。
FlashAttention-2 需要 **Ampere/Hopper 级别的 GPU（≥SM_80，如 A100/A10/L4/H100）** 才能启用其关键指令路径（如 `cp.async` 等）。在 **Volta(V100, SM_70)** 和 **Turing(T4, SM_75)** 上不满足硬件要求，vLLM 就会自动回退到 **xFormers** 后端，所以日志里写着：
# Notes

从零实现 vLLM (1.3）：如何加速 Attention 计算：[https://mp.weixin.qq.com/s/BdWG6_ZTaGRknmsbGfFkMQ…](https://t.co/J49S4ccB3g) 从零实现 vLLM (1.2）：如何实现张量并行：[https://mp.weixin.qq.com/s/8BVEVPPqDQhQ2l8L90dMNQ…](https://t.co/wnbHusOQwO)
**从零实现 vLLM (1.1)：并行词嵌入**https://mp.weixin.qq.com/s/h1cFYDNxcHC30APcarF47A

llm = LLM(
    model=MODEL_PATH,
    tokenizer=MODEL_PATH,
    trust_remote_code=True,
    dtype="half",
    quantization="gptq",          # [模型量化](模型量化.md)的方式
    max_model_len=1024,
    tensor_parallel_size=max(1, torch.cuda.device_count()),
    gpu_memory_utilization=0.9,
    tensor_parallel_size=1,        # [GPU 显卡](GPU%20显卡.md)并行数量
)

    llm_config = {
        "model": MODEL_PATH,
        "trust_remote_code": True,
        "dtype": "half",                 # FP16 权重
        "max_model_len": 512,            # 从 2048 降到 512
        "gpu_memory_utilization": 0.72,  # 从 0.90 降到 0.72
        "tensor_parallel_size": tensor_parallel_size,
        "disable_log_stats": True,
        "swap_space": 8,                 # 8 GiB CPU 交换空间
        "kv_cache_dtype": "fp8",         # FP8 KV Cache（大幅省显存）
        "max_num_batched_tokens": 512,   # 控制批量总 token
        "max_num_seqs": 16,              # 控制并发序列
        "enforce_eager": True,           # 避免 CUDA graph 额外占用
        "distributed_executor_backend": "mp",   # ★ 强制用 MP，绕开 Ray
    }


vLLM 是一个开源的 Python 库，用于快速、高效地部署大型语言模型（LLMs）。它的主要目标是显著提升 LLMs 的推理速度和吞吐量，使其更适用于生产环境。

以下是 vLLM 的一些关键特点和功能：
- **高效的内存管理：**
    - vLLM 使用一种称为 [[PagedAttention]] 的创新技术，它可以有效地管理注意力机制中的内存，避免浪费。
    - Paged Attention 允许 vLLM 连续处理更长的序列，并支持更高的吞吐量。
- **快速推理：**
    - 通过优化的内核和高效的内存管理，vLLM 实现了比传统 LLM 服务框架更高的推理速度。
    - 通过最优的batching和cache机制实现了更高级的推理效率。
- **易于使用：**
    - vLLM 提供了一个简单的 API，可以轻松地将 LLMs 部署为服务。
    - 它支持各种流行的 LLM 架构，并且可以与 Hugging Face Transformers 等库集成。
- **支持多种解码算法：**
    - 支持各种解码算法，其中包括贪婪解码，采样，beam search等等。
- **应用场景：**
    - vLLM 适用于需要高性能 LLM 推理的各种应用程序，例如聊天机器人、文本生成和问答系统。

简而言之，vLLM 的主要优势在于它能够显著提高大型语言模型推理的效率，从而降低延迟并提高吞吐量，这使得它在大规模部署 LLM 时非常有价值。
