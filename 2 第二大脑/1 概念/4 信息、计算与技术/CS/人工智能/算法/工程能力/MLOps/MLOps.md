---
date created: 2025-03-24
date modified: 2025-10-14
uid: ac446d7d-eef8-4b8a-99b6-55a2f81b2b0b
---


# Cues

[[数据工程 数据管线]]

# Notes

| 经典问题变体 (Problem) | MLOps 对应解决方案 (Solution) |
|:--- |:--- |
| **模型训练无法复现** | 使用版本控制（Git + DVC）和容器化（Docker）确保代码、数据和环境的一致性。|
| **数据变化导致模型失效** | 实施数据版本控制（Data Versioning）和数据漂移监控（Data Drift Detection）。|
| **模型部署流程复杂** | 通过 CI/CD 流水线（Pipeline）自动完成模型的打包、测试和部署（例如使用 Kubeflow, MLflow, Airflow）。|
| **实验记录混乱** | 统一使用实验追踪工具（如 MLflow, Weights & Biases）来记录和管理所有实验的参数、指标和产物。|
| **模型上线后难以监控性能** | 建立完善的模型监控体系（Model Monitoring & Alerting），实时追踪线上模型的性能指标并设置告警。|
| **团队协作困难** | 采用标准化流程和工具（如特征商店 Feature Store, 流水线模板 Pipeline Template）打通研发、运维和数据团队。|

🧰 四、MLOps 核心组件（Pipeline）

1. 数据层（DataOps）：数据收集、清洗、特征存储（Feature Store）

2. 实验层（ModelOps）：模型训练、追踪、管理（MLflow/W&B）

3. 部署层（Deployment）：API服务化、A/B测试、灰度发布

4. 监控层（Monitoring）：监控性能、数据漂移、异常报警

5. 反馈层（Feedback Loop）：自动触发再训练和版本更新

  
