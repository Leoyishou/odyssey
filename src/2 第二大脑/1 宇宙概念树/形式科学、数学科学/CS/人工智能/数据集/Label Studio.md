---
date created: 2025-06-30
date modified: 2025-07-10
uid: 3b42a38d-4021-48c1-9f90-79d5e84e6af1
---

下图左侧的七个 **Project Settings** 子菜单控制的是「项目级」功能，作用分别如下（以自部署的开源 Label Studio v1.x 为例）：

| 菜单                     | 典型设置                                            | 作用简介                                                                                                                                                                                                                                                                                                                                           | 何时最常用                   |
| ---------------------- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| **General**            | 项目名称、描述、标签配置草稿、任务抽样策略                           | 项目的“总控台”。这里决定一个项目要标什么数据、用什么标注配置，以及给标注者看到的项目说明                                                                                                                                                                                                                                                                                                  | 创建新项目或调整标签体系            |
| **Labeling Interface** | XML-like 标签模板、界面预览                              | 可视化编辑/粘贴 Label Studio XML，实时预览标注 UI；保存后即刻生效                                                                                                                                                                                                                                                                                                    | 需要调整标注控件布局或增加标签时        |
| **Annotation**         | 共识策略（Consensus）、质检设置、标注分配                       | 定义多人标注时的分配规则、质检阈值、仲裁流程等，提高数据一致性                                                                                                                                                                                                                                                                                                                | 多人协作或需要 QA 流程的项目        |
| **Model**              | ML Backend URL、Webhook secret                   | 把你训练好的模型（REST 接口或 LS ML-Backend SDK）接入项目，实现**自动预标注**或**主动学习** workflow ([labelstud.io](https://labelstud.io/guide/project_settings?utm_source=chatgpt.com "Label Studio Documentation — Project settings"))                                                                                                                                    | 想用模型先打“草标签”再人工校正，或做主动学习 |
| **Predictions**        | 导入/导出 prediction 文件，查看对比                        | 管理模型推理结果：手动上传、API 推送，或审核、比对已有预测与人工标注                                                                                                                                                                                                                                                                                                           | 线下批量推理后再人工审核；或评估模型效果    |
| **Cloud Storage**      | S3/GCS/Azure Blob/本地文件夹：Source & Target         | 配置**源存储**同步原始任务，或**目标存储**自动回写标注结果，形成流水线 ([docs.humansignal.com](https://docs.humansignal.com/guide/project_settings_lse?utm_source=chatgpt.com "Label Studio Enterprise Documentation — Project settings"), [labelstud.io](https://labelstud.io/guide/storage?utm_source=chatgpt.com "Cloud and External Storage Integration - Label Studio")) | 数据量大、需要与对象存储打通时         |
| **Webhooks**           | 事件类型（task.created、annotation.completed…）、回调 URL | 每当任务/标注/预测等事件触发时，向外部服务发送 JSON payload，实现 CI/CD、主动学习或通知 ([labelstud.io](https://labelstud.io/blog/webhooks-in-label-studio-when-and-how-to-use-them/?utm_source=chatgpt.com "Webhooks in Label Studio: When And How To Use Them"))                                                                                                              | 要把标注动作接入下游流水线或消息系统      |
| **Danger Zone**        | Reset Cache、Drop Tabs、**Delete Project**        | 高危操作：重建缓存、清空 Data Manager 视图，或**彻底删除项目**（不可恢复）([labelstud.io](https://labelstud.io/guide/project_settings?utm_source=chatgpt.com "Label Studio Documentation — Project settings"), [docs.humansignal.com](https://docs.humansignal.com/guide/release_notes?utm_source=chatgpt.com "On-premises release notes for Label Studio Enterprise"))   | 出现配置失效、要重建缓存，或需要销毁项目时   |

**使用小贴士**

1. **先 General，后 Interface**：先拟好标签体系再去拖拽 UI，可以避免多次回滚。
    
2. **模型与预测分离**：在线推理就用 _Model_ 菜单；离线批量推理后再上传结果，就走 _Predictions_。
    
3. **云存储“双向同步”**：Source‐>Label Studio‐>Target，可接入训练管线自动增量学习。
    
4. **善用 Webhooks**：结合脚本可实现「新任务来就预处理」「标注完成即推送训练」等自动化。
    
5. Danger Zone 操作前务必备份；生产环境建议把该菜单权限仅授予管理员。
    

这样，你就能根据项目需求快速定位到对应的设置入口，搭建一条“数据 → 标注 → 模型 → 回流”的闭环。
