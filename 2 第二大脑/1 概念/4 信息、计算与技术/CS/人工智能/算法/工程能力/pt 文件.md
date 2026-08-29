---
uid: 617bb9fc-9aa4-49f6-b500-17397d34d7ea
date created: 2025-07-19
date modified: 2025-07-20
---
# Summary

`.pt` 是 **PyTorch 的标准文件格式**，用于保存模型、权重和张量数据。

# Cues

[[safetensor]]



# Notes
##  model.pt - 预训练模型权重文件 (5.4MB)

这是一个已经训练好的标准Transformer模型的所有权重，包含：

- Token embedding权重

- 每一层的attention权重 (W_q, W_k, W_v, W_o)

- 每一层的FFN权重 (W1, W2, W3)

- LayerNorm的权重

- 输出层权重




# 基本信息

| 特性 | 说明 |
|------|------|
| **全称** | PyTorch Tensor file |
| **底层格式** | Pickle + ZIP (Python序列化) |
| **用途** | 保存模型权重、完整模型、张量数据 |
| **读取方式** | `torch.load()` |
| **保存方式** | `torch.save()` |

# 在代码中的使用

```python
# 保存模型权重
torch.save(model.state_dict(), "model.pt")

# 加载模型权重  
state_dict = torch.load("model.pt", map_location="cpu")

# 在我们的测试中：
state_dict = torch.load(FIXTURES_PATH / "ts_tests" / "model.pt", map_location="cpu")
```

# 文件内容结构

一个 `.pt` 文件通常包含：

```python
# model.pt 内部结构示例
{
    "token_embeddings.weight": tensor([[0.1, -0.3, ...], ...]),      # Token嵌入层权重
    "layers.0.attn.q_proj.weight": tensor([[...], ...]),             # 第0层Q投影权重  
    "layers.0.attn.k_proj.weight": tensor([[...], ...]),             # 第0层K投影权重
    "layers.0.attn.v_proj.weight": tensor([[...], ...]),             # 第0层V投影权重
    "layers.0.attn.output_proj.weight": tensor([[...], ...]),        # 第0层输出投影权重
    "layers.0.ln1.weight": tensor([1.2, 0.8, ...]),                 # 第0层LayerNorm权重
    "layers.0.ffn.w1.weight": tensor([[...], ...]),                 # 第0层FFN权重
    "layers.0.ffn.w2.weight": tensor([[...], ...]),
    "layers.0.ffn.w3.weight": tensor([[...], ...]),
    # ... 重复每一层的权重
    "layers.1.attn.q_proj.weight": tensor([[...], ...]),
    # ... 
    "output.weight": tensor([[...], ...]),                           # 输出层权重
}
```

# 与其他格式的对比

| 格式 | 框架 | 特点 |
|------|------|------|
| **`.pt` / `.pth`** | PyTorch | PyTorch原生，包含完整上下文 |
| **`.safetensors`** | Hugging Face | 更安全，跨框架兼容 |
| **`.ckpt`** | 通用 | Checkpoint格式，可能包含优化器状态 |
| **`.h5`** | TensorFlow/Keras | HDF5格式 |
| **`.onnx`** | ONNX | 跨框架标准格式 |

# 在CS336项目中的作用

```python
# 这个5.4MB的model.pt文件包含了：
state_dict = torch.load("model.pt")
print(state_dict.keys())

# 输出类似：
# dict_keys([
#     'token_embeddings.weight',           # [10000, 64] - 词汇表到向量的映射
#     'layers.0.attn.q_proj.weight',       # [64, 64] - 第0层注意力Q投影
#     'layers.0.attn.k_proj.weight',       # [64, 64] - 第0层注意力K投影  
#     'layers.0.attn.v_proj.weight',       # [64, 64] - 第0层注意力V投影
#     'layers.0.attn.output_proj.weight',  # [64, 64] - 第0层注意力输出投影
#     'layers.0.ln1.weight',               # [64] - 第0层LayerNorm1
#     'layers.0.ln2.weight',               # [64] - 第0层LayerNorm2
#     'layers.0.ffn.w1.weight',            # [64, 128] - 第0层FFN第1层
#     'layers.0.ffn.w2.weight',            # [128, 64] - 第0层FFN第2层
#     'layers.0.ffn.w3.weight',            # [64, 128] - 第0层FFN门控层
#     # ... 第1层、第2层的所有权重
#     'output.weight'                       # [64, 10000] - 输出到词汇表的映射
# ])
```

# 文件大小分析

5.4MB 对于这个模型来说是合理的：
- 词汇表大小：10,000
- 模型维度：64  
- 层数：3层
- 主要权重：token_embeddings (10000×64) + layers + output (64×10000)

**总参数量**大约几百万个浮点数 × 4字节/浮点数 ≈ 几MB

所以 `.pt` 文件就是 **PyTorch模型权重的标准存储容器**，类似于其他语言中的序列化文件格式。
