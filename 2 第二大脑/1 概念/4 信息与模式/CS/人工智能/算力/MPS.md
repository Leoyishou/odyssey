---
aliases: [Metal Performance Shaders]
date created: 2024-07-17
date modified: 2025-10-05
uid: 4cc40da3-f72f-45f7-8f1a-ca0dca497a7b
---
# 🪄 Summary

 Metal Performance Shaders 金属性能着色器

> **MPS = Apple 版 CUDA**  
    让你的 Mac GPU（尤其是 M 系列芯片）能跑深度学习。

|平台|GPU 框架|厂商|
|---|---|---|
|NVIDIA|CUDA|NVIDIA|
|AMD|ROCm|AMD|
|Apple|**MPS (Metal Performance Shaders)**|Apple|

🖥️ Mac 的 GPU（尤其是 M1 / M2 / M3 芯片）不是 NVIDIA 的，
👉 所以它不支持 CUDA。

那怎么办？苹果就自己搞了一个加速框架：**MPS**。


这是 Apple 为其 M 系列芯片（如您的 M2 Pro）提供的 GPU 加速框架。当您在 PyTorch 中使用 `device = torch.device("mps")` 时，您就是在利用这个 GPU 加速能力来加速深度学习计算。


# 💻 你的 Mac 硬件配置分析

|项目|内容|通俗解释|
|---|---|---|
|**Chip**|Apple M3 Pro|第三代 Apple Silicon，采用台积电 3nm 工艺，比 M2/M1 都快。支持 Metal（MPS）加速。|
|**CPU**|M3 Pro 内含 12 核（通常为 6 性能 + 6 能效核）|同时兼顾性能与续航，单核性能非常强，接近 i7-13700H。|
|**GPU**|M3 Pro 集成 GPU（最多 18 核）|用于图形渲染和机器学习计算，支持 Metal/MPS 加速。性能大约相当于 RTX 2060～3060 的 40–60%。|
|**Memory**|36 GB 统一内存（Unified Memory）|CPU 与 GPU 共享同一片高速内存，没有传统的“显存”瓶颈；36GB 在 Apple 芯片上非常奢侈。|
|**Startup disk**|Macintosh HD|系统主盘，NVMe 固态硬盘，读写速度通常超过 5 GB/s。|
|**macOS**|Sequoia 15.3|最新系统版本，包含对 M3 GPU、MPS、Metal 3 的优化。|

---

# ⚙️ 对开发 / 深度学习 的意义

|任务类型|体验|
|---|---|
|**前端/后端开发**|完全无压力，极快的编译速度。|
|**PyTorch / TensorFlow 训练**|可用 MPS 加速，中小模型（BERT-base、ResNet18 等）速度接近 RTX 2060。|
|**大模型推理 (LLM)**|可以运行 Qwen-1.8B / Llama-2-7B 的推理，但速度不如独显。|
|**视频/图像编辑**|Metal 加速极强，Final Cut / Photoshop 表现接近高端独显。|
|**虚拟机 / Docker / VSCode**|因为统一内存和高带宽 SSD，非常流畅。|

---

# 🔋 建议设置

1. ✅ 确保 PyTorch 是 2.2+ 版本（支持 MPS 完整特性）：
    
    `pip install torch torchvision torchaudio --upgrade`
    
2. ✅ 测试 GPU 是否可用：
    
    `import torch print(torch.backends.mps.is_available()) print(torch.backends.mps.is_built())`
    
3. ✅ 训练时指定设备：
    
    `device = torch.device("mps") model.to(device)`
    

---

简单总结一句话：

> 🧠 你的 M3 Pro（36GB 统一内存）是一台 **高性能轻薄本级别的深度学习开发机**，
> 可以流畅训练中小模型、跑推理、做数据分析、视频编辑。
> 如果将来要大规模训练，可以考虑远程连接 GPU 服务器（比如 A100/A800）。

要不要我帮你写一个 `check_mps.py` 小脚本，运行后会输出「是否支持 MPS、GPU 核心数、内存大小、PyTorch 版本、是否可用加速」？




---

# 🧩 一、问题背景：训练和推理很慢的根源

在机器学习中，训练模型（比如 PyTorch、TensorFlow）需要大量的**矩阵乘法**、**卷积计算**。
这些操作在 CPU 上跑会非常慢，所以我们希望把它们放到**GPU**上去跑。

但是：
🖥️ Mac 的 GPU（尤其是 M1 / M2 / M3 芯片）不是 NVIDIA 的，
👉 所以它不支持 CUDA。

那怎么办？苹果就自己搞了一个加速框架：**MPS**。

---

# ⚙️ 二、MPS 是什么？

**MPS = Metal Performance Shaders**

它是 Apple 官方提供的一个 **底层 GPU 计算框架**，
相当于 **苹果版的 CUDA**。

| 对比项    | NVIDIA 平台    | Apple 平台                        |
| ------ | ------------ | ------------------------------- |
| GPU 驱动 | CUDA / cuDNN | Metal                           |
| 加速接口   | `torch.cuda` | `torch.mps`                     |
| 底层库    | CUDA Kernel  | Metal Performance Shaders (MPS) |
| 可用设备   | NVIDIA GPU   | Apple Silicon (M1/M2/M3)        |

---

# 🚀 三、在 PyTorch 中如何“使用 MPS 加速”

当你在 Mac 上使用 PyTorch ≥1.12 时，可以直接写：

```python
import torch

device = torch.device("mps")  # 使用 Mac GPU
x = torch.randn(1000, 1000, device=device)
```

此时，PyTorch 会自动把计算图转发到 Apple 的 Metal GPU 上执行，
这就叫 **“MPS 加速”**。

---

# 🧠 四、它到底加速了什么？

✅ 加速的部分主要包括：

* 张量运算（矩阵乘法、卷积、激活函数）
* 模型前向与反向传播
* 一部分 PyTorch 的神经网络算子

⚠️ 但要注意：

* 有些算子还没完全支持，会 fallback 到 CPU。
* 性能不如 NVIDIA 的 CUDA GPU，但比纯 CPU 快很多（通常快 2～5 倍）。

---

# 🔋 五、简单理解一行总结

> 💡 **“MPS 加速” 就是让 PyTorch 把原本在 CPU 上跑的计算，转到 Mac 的 GPU（Metal 框架）上跑，从而更快。**

---

# 📊 举个例子（实际测试）

假设你在 M2 Mac 上跑：

```python
import torch, time
x = torch.randn(2000, 2000)
y = torch.randn(2000, 2000)

start = time.time()
for _ in range(100):
    z = x @ y
print("CPU time:", time.time() - start)
```

然后改成：

```python
x = x.to("mps")
y = y.to("mps")

start = time.time()
for _ in range(100):
    z = x @ y
print("MPS time:", time.time() - start)
```

你会发现 MPS 模式下的运行时间通常显著更短。

---

要不要我帮你画一张图，对比 **CPU → CUDA → MPS 三种计算路径**（展示它们各自的计算流、内存分配与加速层）？
