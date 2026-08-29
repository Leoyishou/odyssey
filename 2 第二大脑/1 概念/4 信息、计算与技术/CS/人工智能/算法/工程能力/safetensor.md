---
uid: 30583d35-2e41-4fe0-9d6d-7ceac32d3e1c
date created: 2025-07-19
date modified: 2025-07-20
---
# Summary

- **学术/教学**: 仍然使用`.pt`格式 (如CS336)
- **工业/开源**: 主流是`.safetensors`格式 (如现代Qwen)
- **趋势**: `.safetensors`正在成为事实标准，因为更安全、更快


# Cues



# Notes

Ran tool

是的，但**现代开源Qwen模型主要使用`.safetensors`格式**，而不是传统的`.pt`格式！

## 现代Qwen模型的文件格式

从Hugging Face上的Qwen模型可以看到：

### 主流格式：`.safetensors`
```Java
Qwen/Qwen2-72B/
├── model-00001-of-00037.safetensors  (3.76 GB)
├── model-00002-of-00037.safetensors  (4 GB)
├── ...
├── model-00037-of-00037.safetensors  (3.46 GB)
└── model.safetensors.index.json      (索引文件)
```

### 为什么用`.safetensors`而不是`.pt`？

| 特性 | `.pt` (PyTorch) | `.safetensors` (Hugging Face) |
|------|----------------|----------------------------|
| **安全性** | ❌ 可执行任意Python代码 | ✅ 只存储张量数据，无代码执行 |
| **跨框架** | ❌ 仅限PyTorch | ✅ 支持PyTorch/TensorFlow/JAX |
| **加载速度** | 较慢 | ✅ 更快的加载速度 |
| **内存映射** | 有限支持 | ✅ 零拷贝内存映射 |
| **文件完整性** | ❌ 无校验 | ✅ 内置校验和 |

## 实际使用对比

### 传统方式 (CS336作业用的)
```python
# 加载.pt文件
state_dict = torch.load("model.pt", map_location="cpu")
```

### 现代开源模型
```python
# 加载.safetensors格式
from transformers import AutoModelForCausalLM
model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen2.5-7B-Instruct")
```

## Qwen模型的完整生态

现代Qwen模型提供多种格式：

| 格式 | 用途 | 文件扩展名 |
|------|------|-----------|
| **Safetensors** | 🔥 主流，安全快速 | `.safetensors` |
| **GGUF** | CPU/移动端推理 | `.gguf` |
| **AWQ** | 4-bit量化推理 | `.safetensors` (量化) |
| **ONNX** | 跨平台部署 | `.onnx` |

## 为什么CS336课程还用`.pt`？

1. **教学简单性**: `.pt`格式更直接，学生容易理解
2. **完全控制**: 课程需要测试学生的底层实现，不用高级库
3. **历史原因**: 课程设计时`.safetensors`还不够普及

所以虽然CS336用`.pt`来教学，但实际生产环境中，你更可能遇到`.safetensors`格式的Qwen模型！
