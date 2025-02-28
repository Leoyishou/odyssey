---
draw:
title: Google Cloud Platform
date created: 2024-11-24
date modified: 2025-02-06
---

console.cloud.google.com 是 Google Cloud Platform (GCP) 的控制台网址，主要用于：

1. 云资源管理：
   - 创建和管理虚拟机
   - 设置网络和存储
   - 管理数据库实例
   - 配置 Kubernetes 集群

2. 开发者工具：
   - Cloud Shell（云端命令行）
   - Cloud Code（代码编辑器）
   - Cloud Build（CI/CD 工具）
   - Container Registry（容器镜像仓库）

3. AI/ML 服务：
   - Vertex AI（机器学习平台）
   - Cloud Vision API（图像识别）
   - Cloud Natural Language（自然语言处理）

4. 监控和运维：
   - Cloud Monitoring（监控指标）
   - Cloud Logging（日志管理）
   - Error Reporting（错误跟踪）
   - Cloud Trace（性能分析）

5. 安全和身份：
   - IAM（身份和访问管理）
   - Security Command Center（安全中心）
   - Cloud KMS（密钥管理）

相比你现在使用的腾讯云，GCP：

- 全球基础设施更完善
- AI/ML 服务更丰富
- 开发者工具更完整
- 但国内访问可能需要特殊网络环境

## Google 登录

是的，如果你要在网站中使用 Google 登录功能，确实需要在 Google Cloud Platform (GCP) 控制台进行配置，主要是：

1. 在 GCP 配置 OAuth 2.0：
   - 创建项目（如果没有）
   - 配置 OAuth 同意屏幕
   - 创建 OAuth 2.0 客户端 ID
   - 添加授权来源 URL（你的腾讯云域名）
   - 添加授权重定向 URL

2. 你需要获取的关键信息：
   - Client ID（客户端 ID）
   - Client Secret（客户端密钥）
   
3. 需要在 GCP 配置的内容：
   - 允许的 JavaScript 来源（比如 https://viva.liugongzi.org）
   - 授权重定向 URI（用户登录后的跳转地址）
   - 设置应用程序类型（Web 应用）

4. 安全注意事项：
   - Client Secret 要保密
   - 正确设置允许的域名
   - 谨慎管理 OAuth 权限范围

需要我详细说明如何在 GCP 配置 Google 登录吗？
