---
date created: 2025-03-23
date modified: 2025-07-10
uid: 27ee99fa-e7aa-4f17-b477-f0dbfaefe01b
---

是的，**checkpoint**的最大好处之一就是能从上次训练中断的位置继续训练，无需重新开始。

具体而言，checkpoint 保存了：

1. **模型的权重（weights）**
2. **优化器状态（optimizer states）**
3. **学习率调度器状态（scheduler states）**
4. **训练过程的状态信息（trainer_state.json，如当前迭代次数、训练进度等）**

这样，在训练意外中断（如发生OOM错误、程序中断、服务器重启、断电等）时，不用从头开始重新训练，只需从最近保存的checkpoint恢复训练即可。

具体使用方法（以Hugging Face的Trainer为例）：

```bash
--resume_from_checkpoint data/path_to_your_checkpoint
```

此时训练会：

- 自动载入之前保存的权重与优化器状态
- 学习率会自动恢复到中断时的状态
- 训练迭代步数会从之前停止的位置继续计算，而不是重新开始

这样能节省大量的时间和资源，避免浪费之前的训练成果。

---

## 📌 **注意事项：**

- 尽量定期保存checkpoint（例如每几百或几千步保存一次）
- checkpoint文件占空间较大，注意及时清理不需要的checkpoint文件
- 从checkpoint恢复时，必须确保checkpoint的目录和文件都是完整的，否则可能会导致恢复失败

因此你当前使用checkpoint-2000恢复训练，是比较稳妥和推荐的做法。
