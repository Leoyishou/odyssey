---
date created: 2024-11-19
date modified: 2025-07-10
uid: 79dea59a-f397-4b14-aade-c578e5f0cc39
---

#handsDirty

https://colab.research.google.com/drive/1SmjpWrz8NzU7UyEDvbv9qEqGm_2keWEq#scrollTo=h3tA6La8Bhxl

## 版本

|             |            |                           |
| ----------- | ---------- | ------------------------- |
| 版本号         | 发布时间       | 关键更新简述                    |
| v0.1.0      | 2022/11/9  | 初次开源，提供基础索引和 LLM 接入功能     |
| v0.10.0     | 2024/2/12  | 架构大改：核心包分离、集成模块独立、弃用旧接口   |
| v0.11.0     | 2024年9月    | 引入 Workflows、增强可观测性、优化图索引 |
| v0.12.0     | 2024/11/17 | 去除 Python 3.8 支持，依赖兼容性更新  |
| v0.12.30 🏆 | 2025/4/10  | 最新稳定版本，常规功能增强与 bug 修复     |
|             |            |                           |

过期特性

https://docs.llamaindex.ai/en/stable/changes/deprecated_terms/#servicecontext

## 核心逻辑

1. 准备索引  document 进来，通过 embedding 建立[索引Index](索引Index.md)，然后 save_to_disk
2. 用户发起 query
3. 从 load_from_disk 到的索引中
4. LLM 根据从 index 中拿到的 chunks 来回答问题，生成 Response

| **概念**              | **通俗解释**                                                                    | **在 LlamaIndex 中的作用**                                                       |
| ------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| **Document** ✅      | raw 资料                                                                      | LlamaIndex 构建索引的基本单位；每个 Document 会被切分成更小的段落或 chunks，存入索引以便检索时快速定位。|
| **[[索引Index]]** ✅   | 存放文本内容和它们对应 embedding（向量表示）或层次结构的「数据库」。<br/>就像给一本书做的「目录索引」，方便快速查找章节位置。| 用于在查询时进行相似度匹配或分层检索，提升查询效率；常见有向量索引（GPTSimpleVectorIndex）和树索引（GPTTreeIndex）等。|
| **Chunk / Node**    | Document 切分后得到的较小文本块。<br/>好比一本书中的一节内容，这样做是为了更好地将上下文拆分，利于向量检索和后续的 Prompt 拼接。| 作为检索与检索后上下文拼接（Prompt）的基础单元；向量索引会基于这些 chunks 的嵌入向量来查找最匹配的文本。|
| **insert / update** | 向现有索引中增量添加或更新文档的操作。<br/>就像在图书馆的索引卡里增加新书籍条目，而无需重新建一整套索引。| 使索引内容保持最新，不用重新构建全部索引；尤其对经常新增内容的场景很重要，可以节省计算资源和时间。|

| **概念**       | **通俗解释**                                                                     | **在 LlamaIndex 中的作用**                                            |
| ------------ | ---------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **LLMs**     | 封装了 LLM（如 OpenAI GPT-3.5、GPT-4）调用的对象。<br/>包括模型名称、请求参数（温度等），以及如何向模型发送 prompt。| 在索引构建或查询回答时，需要用 LLM 来做总结、回答生成等；LLMs 就是把这些调用逻辑集中起来。|
| **Settings** | 为 LlamaIndex 提供「统一配置」的上下文，比如模型类型（LLMPredictor）、嵌入模型（Embedding）、超参数等。| 允许我们一次性地把 LLM、Embedding、提示模板等打包成一个上下文，避免在多处重复写配置；构建索引或查询时可共享此配置。|

## Settings 能配置啥

| 配置项             | 描述                             | 示例代码                                                                                           |
| --------------- | ------------------------------ | ---------------------------------------------------------------------------------------------- |
| LLM ✅           | 用于响应提示和查询，负责生成自然语言回答           | `Settings.llm = OpenAI(model="gpt-3.5-turbo", temperature=0.1)`                                |
| Embed Model ✅   | 用于将文本转换为数值表示，用于计算相似度和 top-k 检索 | `Settings.embed_model = OpenAIEmbedding(model="text-embedding-3-small", embed_batch_size=100)` |
| Text Splitter   | 用于将文档解析成更小的块，称为节点              | `Settings.text_splitter = SentenceSplitter(chunk_size=1024)`                                   |
| Chunk Size      | 文本分块的大小                        | `Settings.chunk_size = 512`                                                                    |
| Chunk Overlap   | 文本分块的重叠部分                      | `Settings.chunk_overlap = 20`                                                                  |
| Transformations | 在摄取过程中应用于文档的转换                 | `Settings.transformations =[SentenceSplitter(chunk_size=1024)]`                                |
| Tokenizer       | 用于计算令牌数量，应与使用的 LLM 匹配          | `Settings.tokenizer = tiktoken.encoding_for_model("gpt-3.5-turbo").encode`                     |
| Callbacks       | 设置全局回调管理器，用于观察和消费整个代码生成的事件     | `Settings.callback_manager = CallbackManager([token_counter])`                                 |
| Context Window  | LLM 的最大输入大小                    | `Settings.context_window = 4096`                                                               |
| Num Output      | 为文本生成保留的令牌数量                   | `Settings.num_output = 256`                                                                    |

这些配置可以全局设置，也可以在特定接口中通过参数进行局部覆盖。

#cheatsheet

以下是 LlamaIndex v0.12.30 中常用 API 的速查表，涵盖了从数据加载、索引构建到查询的核心操作

---

## 📄 文档加载与构建


````python
from llama_index.core import SimpleDirectoryReader, Document

# 从目录加载文档
documents = SimpleDirectoryReader("data").load_data()

# 手动创建文档对象
doc = Document(text="示例文本", id_="doc_1", metadata={"author": "张三"})
```


## 🧠 索引构建与管理
```python
from llama_index.core import VectorStoreIndex

# 从文档构建向量索引
index = VectorStoreIndex.from_documents(documents)

# 插入新文档
index.insert(Document(text="新文档内容", id_="doc_new"))

# 删除文档
index.delete_ref_doc("doc_1", delete_from_docstore=True)

# 更新文档
doc.text = "更新后的内容"
index.update_ref_doc(doc)

# 刷新索引
index.refresh_ref_docs([doc])
```


## 🔍 查询与问


```python
# 创建查询引擎
query_engine = index.as_query_engine()

# 提出查询
response = query_engine.query("什么是 LlamaIndex？")

# 使用聊天引擎
chat_engine = index.as_chat_engine()
response = chat_engine.chat("请介绍一下 LlamaIndex 的功能。")
``

## 🧩 节点解析与元数据理


```python
from llama_index.node_parser import SimpleNodeParser

# 使用节点解析器
parser = SimpleNodeParser()
nodes = parser.get_nodes_from_documents(documents)

# 设置文档 ID
doc = Document(text="内容", id_="custom_id")

# 自动设置文件名为 ID
reader = SimpleDirectoryReader("data", filename_as_id=True)
``


## 🧪 高级功能扩展





```python
from llama_index.embeddings import MixedbreadAIEmbedding
from llama_index.core import Settings
from llama_index.llms import OpenAI
from llama_index.agent import FunctionCallingAgent

# 使用自定义嵌入模型
Settings.embed_model = MixedbreadAIEmbedding(api_key="your_api_key", model_name="model_name")

# 使用自定义 LLM
Settings.llm = OpenAI(model="gpt-3.5-turbo", temperature=0.1)

# 使用代理（Agent）
agent = FunctionCallingAgent.from_tools([query_engine])
response = agent.chat("请查询相关信息。")
```
