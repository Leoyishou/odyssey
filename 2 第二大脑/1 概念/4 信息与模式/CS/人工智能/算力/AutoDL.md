---
date created: 2024-10-22
date modified: 2025-08-11
uid: a268d76d-d7a9-4c22-8631-088255801390
---
# Summary

Autodl（Auto Deep Learning）是一个在中国比较流行的深度学习云平台，主要提供GPU算力租用服务。以下是它的主要特点：

# Notes

```shell
# 开梯子
source /etc/network_turbo


# 清空回收站
rm -rf /root/autodl-tmp/.Trash-0/*

# 使用 HuggingFace 镜像
export HF_ENDPOINT=https://hf-mirror.com
export HF_HOME=/root/autodl-tmp/cache

# 或者使用 ModelScope 镜像
export HF_ENDPOINT=https://modelscope. 

```

## 监控

### 1. 在新终端监控GPU使用

```bash
# 实时监控GPU
watch -n 1 nvidia-smi

# 或者更详细的监控
nvidia-smi dmon -s pucvmet -d 5
```

### 2. 查看训练日志

```bash
# 如果有输出日志文件
tail -f outputs/training.log
```

### 3. 检查内存使用

bash

```bash
# 查看系统资源
htop
```

1. 硬件资源：
   - 提供各种型号的 NVIDIA GPU（如 $3090$、$4090$、$A100$ 等）
   - 按小时计费的租用模式
   - 可以选择不同配置的显卡和内存组合

2. 主要功能：
   - Jupyter Notebook 环境
   - SSH 远程连接
   - 常用深度学习框架预装（PyTorch, TensorFlow 等）
   - 数据持久化存储
   - 自定义镜像部署

3. 使用场景：
   - 深度学习模型训练
   - 大规模数据处理
   - AI 研究开发
   - 学术研究

4. 价格优势：
   - 比自建服务器成本低
   - 按需付费，不使用不计费
   - 经常有优惠活动和学生优惠

5. 基本使用流程：

```bash
# 1. 选择实例并启动
# 2. 连接到实例（SSH或Jupyter）
ssh -p <端口> root@<实例IP>

# 3. 上传数据（可以用scp或者在线下载）
scp -P <端口> 本地文件 root@<实例IP>:目标路径

# 4. 运行训练脚本
python train.py

# 5. 下载结果
scp -P <端口> root@<实例IP>:结果文件 本地路径
```

6. 常见使用建议：
   - 使用前先估算好需要的时长和配置
   - 重要数据要及时备份
   - 不用时记得停止实例以节省费用
   - 可以使用自动关机脚本避免超支
   - 使用永久存储来保存模型和数据集

7. 替代选择：
   - Google Colab（免费但限制多）
   - AWS/GCP（功能全但价格较贵）
   - 阿里云/腾讯云（国内服务但价格较高）
   - 本地搭建（前期成本高但长期使用划算）

如果你是初学深度学习或者只是临时需要GPU算力，Autodl是一个不错的选择。它的使用门槛相对较低，价格也比较实惠。不过要注意合理规划使用时间，避免产生不必要的费用。
