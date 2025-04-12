---
draw:
title: meridian
date created: 2025-04-08
date modified: 2025-04-08
---

https://github.com/iliane5/meridian

>**Presidential-level intelligence briefings, built with AI, tailored for you.**

#有趣 _Built because we live in an age of magic, and we keep forgetting to use it._

这些环境变量是项目正常运行的必要配置，每个都有特定的用途：

1. **DATABASE_URL**：连接到PostgreSQL数据库，存储项目的所有数据，包括文章、分析结果和生成的简报。没有数据库连接，系统无法存储或检索任何信息。
2. **SECRET_KEY**：用于加密和安全目的，保护系统中敏感数据的传输和存储。这是任何web应用程序的基本安全需求。
3. **GOOGLE_API_KEY**：连接到Google的Gemini AI服务。根据项目README，Meridian依赖Gemini模型进行文章分析和简报生成。没有这个密钥，AI分析功能无法工作。
4. **WORKER_API**：前端需要知道后端API的地址，以便获取数据。这是前端和后端通信的必要配置。

这个项目本质上是一个AI驱动的新闻分析系统，它通过抓取新闻源、分析内容并生成个性化简报来工作。所有这些环境变量都是支持这些核心功能的基础设施配置。

如果你只是想简单地查看项目的前端界面，可能只需要配置前端相关的变量，但要获得完整功能，所有这些配置都是必要的。
