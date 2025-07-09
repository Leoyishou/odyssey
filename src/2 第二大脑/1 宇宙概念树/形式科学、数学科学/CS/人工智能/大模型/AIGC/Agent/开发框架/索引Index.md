---
draw:
title: 索引Index
date created: 2025-04-16
date modified: 2025-04-16
---

以下是 LlamaIndex 支持的主要索引类型及其特点，整理成 Markdown 表格，方便您对比和选择：

| 索引类型                             | 描述                                                       | 优点                          | 适用场景                    |
| -------------------------------- | -------------------------------------------------------- | --------------------------- | ----------------------- |
| 向量存储索引（Vector Store Index）| 将文档分割为节点，并使用嵌入模型将每个节点转换为向量，存储在向量数据库中。查询时，通过向量相似度检索相关节点。| 支持语义搜索，适用于大规模数据集，查询效率高。| 构建问答系统、知识库检索等需要语义理解的应用。|
| 摘要索引（Summary Index，原 List Index）| 将文档按顺序存储为节点列表。查询时，遍历所有节点，或结合嵌入进行筛选。| 实现简单，适用于小型数据集或对顺序有要求的文档。| 小型数据集或顺序敏感的文档。|
| 树索引（Tree Index）| 构建一个层次结构的树形索引，将节点组织为父子关系。查询时，从根节点开始，逐层选择相关子节点，最终定位到目标信息。| 适用于需要分层次摘要或结构化查询的场景。| 需要分层次摘要或结构化查询的场景。|
| 关键词表索引（Keyword Table Index）| 从每个节点中提取关键词，建立关键词到节点的映射表。查询时，根据输入中的关键词查找对应的节点。| 查询速度快，适用于关键词明确的查询。| 关键词明确的查询。|
| 属性图索引（Property Graph Index）| 构建包含实体和关系的知识图谱，节点表示实体，边表示实体之间的关系。可选地，为节点生成嵌入，以支持语义查询。| 适用于需要表示复杂关系和结构化数据的场景，如知识图谱。| 表示复杂关系和结构化数据的场景，如知识图谱。|
| Composable Graph Index           | 支持多种子索引并组合                                               |                             |                         |

根据您的具体需求，选择合适的索引类型可以提升查询效率和系统性能。

```Java
from llama_index.embeddings.huggingface import HuggingFaceEmbedding
from llama_index.core import Settings

embed_model = HuggingFaceEmbedding(model_name="sentence-transformers/all-MiniLM-L6-v2")
Settings.embed_model = embed_model

index = VectorStoreIndex.from_documents(documents)
query_engine = index.as_query_engine()
response = query_engine.query("这是自定义 embedding 后的提问")
print("回答：", response)

```
