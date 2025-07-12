---
date created: 2025-04-16
date modified: 2025-07-10
uid: 1f8812e2-59d1-43e9-ad66-ace0d33ac0bd
---

下面给出一个参考示例教程，整体风格与前面 PyTorch 基础教程类似，分步骤介绍 **LlamaIndex**（原名 GPT Index）的核心概念与常用功能。每一步包含「**理论介绍**」、「**编程任务**」和「**验证**」三部分，帮助读者循序渐进地掌握 LlamaIndex 的使用。该教程的目标是让你在大约 10~15 小时内，对 LlamaIndex 的主流功能（数据接入、索引构建、查询、与 LLM 结合等）形成整体认识，并能完成简单项目。**每步的示例代码可在 Jupyter Notebook、Colab 或任何 Python 环境运行。**

> **假设前置条件**：读者对 Python 基础较熟悉，对语言模型（LLM）或向量检索有基本概念即可。**关于模型和接口**：本教程在示例中以 OpenAI 接口为例，其他模型或私有 API 的使用方式与此类似，只需替换相关的配置和调用方式。**关于环境**：如果在 Colab 环境使用，请先 `!pip install llama-index openai`。**教程结构总览**：
>
> 1. [第1步：LlamaIndex 安装与快速体验](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step1)
>
> 2. [第2步：加载与预处理文档](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step2)
>
> 3. [第3步：构建索引](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step3)
>
> 4. [第4步：查询索引](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step4)
>
> 5. [第5步：自定义嵌入与向量检索](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step5)
>
> 6. [第6步：结合 Prompt 与 LLM](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step6)
>
> 7. [第7步：增量更新与多文档索引](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step7)
>
> 8. [第8步：进阶功能与评估](https://chatgpt.com/c/67ff1e59-c2f8-8010-854c-a2b4fab487d9#step8)
>

|                               |                                                                              |                                                                                |
| ----------------------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| 概念                            | 通俗解释                                                                         | 在 LlamaIndex 中的作用                                                              |
| Document                      | 一段可以被索引的文本资料，可能是从文件或其他数据源获得的「文章」。想象一下：如果你有一份长篇的新闻稿，这就是一个 Document。| LlamaIndex 构建索引的基本单位；每个 Document 会被切分成更小的段落或 chunks，存入索引以便检索时快速定位。|
| Chunk / Node                  | Document 切分后得到的较小文本块。<br/>好比一本书中的一节内容，这样做是为了更好地将上下文拆分，利于向量检索和后续的 Prompt 拼接。| 作为检索与检索后上下文拼接（Prompt）的基础单元；向量索引会基于这些 chunks 的嵌入向量来查找最匹配的文本。|
| Index                         | 存放文本内容和它们对应 embedding（向量表示）或层次结构的「数据库」。<br/>就像给一本书做的「目录索引」，方便快速查找章节位置。| 用于在查询时进行相似度匹配或分层检索，提升查询效率；常见有向量索引（GPTSimpleVectorIndex）和树索引（GPTTreeIndex）等。|
| Embedding                     | 把文本转换成向量（数字列表）的过程和结果，向量可代表文本的语义。<br/>比如，把「我喜欢猫」转换成 768 维向量，用来和其他句子做比较。| 在向量索引中，用于计算文本的相似度，找出与查询最相关的文本块；可选择用 OpenAI 的模型或自定义 HuggingFace 模型。|
| ServiceContext                | 为 LlamaIndex 提供「统一配置」的上下文，比如模型类型（LLMPredictor）、嵌入模型（Embedding）、超参数等。| 允许我们一次性地把 LLM、Embedding、提示模板等打包成一个上下文，避免在多处重复写配置；构建索引或查询时可共享此配置。|
| LLMPredictor                  | 封装了 LLM（如 OpenAI GPT-3.5、GPT-4）调用的对象。<br/>包括模型名称、请求参数（温度等），以及如何向模型发送 prompt。| 在索引构建或查询回答时，需要用 LLM 来做总结、回答生成等；LLMPredictor 就是把这些调用逻辑集中起来。|
| Prompt / Prompt Template      | 向 LLM 提供的提示文本（系统提示 + 用户问题 + 上下文等），让模型知道「自己是谁、要做什么」。<br/>可以理解为跟 AI 聊天时的背景/要求。| 指导 LLM 怎样使用检索到的文本块回答问题；可自定义格式、风格，如让模型给出参考文档、简洁回答或详细推理等。|
| Query                         | 用户发起的自然语言问题或任务请求，输入给 LlamaIndex 以得到回答。<br/>就像向搜索引擎输入关键词，只是这里是大语言模型可理解的问题。| 触发索引检索和 LLM 回答的过程；index.query(...) 会对 Query 进行向量检索，并将结果打包成 Prompt 发给 LLM 生成回复。|
| Response                      | LLM 根据检索得到的上下文和用户查询，输出的最终回答。<br/>可能包含所用到的引用文本、理由等，具体取决于 Prompt 设计与模型能力。| 是向用户反馈的文本答案；可以手动查看或再做其他处理，如格式化成 JSON、进一步分析等。|
| insert / update               | 向现有索引中增量添加或更新文档的操作。<br/>就像在图书馆的索引卡里增加新书籍条目，而无需重新建一整套索引。| 使索引内容保持最新，不用重新构建全部索引；尤其对经常新增内容的场景很重要，可以节省计算资源和时间。|
| save_to_disk / load_from_disk | 将索引保存到本地文件/从本地文件加载的过程。<br/>类似把「目录索引卡」持久化到硬盘，下次就不用再手动重新生成。| 方便在多次运行间复用索引，不必重复构建；可将索引分享给他人或部署到服务器后直接加载使用。|

---

## 第1步：LlamaIndex 安装与快速体验

### 理论介绍

[LlamaIndex](https://github.com/jerryjliu/llama_index)（原名 GPT Index）是一个将文本数据统一为可查询知识库，并通过大语言模型（LLM）进行自然语言查询与推理的框架。它核心做了两件事：

1. **索引构建**：将原始文档（文本、PDF、网页等）整理为一种便于检索的结构化形式（例如树索引、向量索引），并保存到内部存储或外部向量数据库中；
    
2. **LLM 查询**：接收用户的自然语言问题，通过调用 LLM（如 OpenAI GPT-3.5/GPT-4、LLaMA 等）结合索引中的内容进行**语义检索**与**上下文增强**的回答。
    

LlamaIndex 能够屏蔽很多底层细节，如分块（chunking）、embedding 生成、提示工程（prompt engineering）等，让我们能快速搭建一个「阅读+问答」式应用。

在本练习中，我们先完成安装并做一个最简单的体验——给 LlamaIndex 一段文本，让它根据这段文本来回答一个问题。

### 编程任务

1. **安装 LlamaIndex**（如果尚未安装）：执行 `pip install llama-index openai`。
    
2. **初始化一个最简单的索引**：从纯文本字符串构建索引。
    
3. **调用索引进行一次查询**。
    
4. 打印查询结果，看是否能根据文本作答。
    

请在 Python 环境中运行以下代码，并根据注释进行补充：

```python
# 第1步：检查安装并导入
# 如果你在本地环境，需要先 pip install llama-index openai
# 这里以 openai API 为例，需要先设置 openai.api_key
import os
import openai
openai.api_key = "YOUR_OPENAI_API_KEY"  # TODO: 替换为你的 OpenAI Key
from llama_index import SimpleDirectoryReader, GPTSimpleVectorIndex, Document
# 如果在 Notebook 中，可以直接写:
# !pip install llama-index openai
# 准备一段文本并构建索引
sample_text = (
    "PyTorch 是一个深度学习框架，提供了灵活的动态计算图和自动微分机制。"
    "LlamaIndex 则更专注于让语言模型对文档进行检索式问答。"
)
# 将文本包装为 Document 对象
documents = [Document(sample_text)]
# 使用最简单的向量索引方式
index = GPTSimpleVectorIndex.from_documents(documents)
# 执行一次查询
query = "什么是 LlamaIndex？"
response = index.query(query)
print("问题：", query)
print("回答：", response)
```

### 验证

观察输出结果，如果索引能根据文本回答一些与 **“LlamaIndex 是什么”** 相关的信息（可能不够完善，但能反映出它从文本中提取/理解了信息），则说明构建和查询索引已成功。

可以使用一个简要的断言（**非严格**）来判断回答是否包含“检索式问答”或“文档”这样的关键词：

```Python
answer_str = str(response).lower()
assert ("检索" in answer_str) or ("文档" in answer_str), "回答似乎没有从文本提取到正确概念！"
print("第1步通过：LlamaIndex 索引构建与查询的简单演示完成。")
```

当输出“第1步通过”时，表明你已经成功安装并初步使用了 LlamaIndex。

---

## 第2步：加载与预处理文档

### 理论介绍

在真实场景中，我们常需要从文件系统、网络等位置加载文本。LlamaIndex 提供了一些辅助类，比如 `SimpleDirectoryReader` 可以从本地目录批量读取文本文件。更多格式（PDF、Notion、API 数据源等）可以使用官方/社区的插件或自行编写加载逻辑。**预处理（Preprocessing）** 指的是对加载到的文本进行初步清理、分段（chunking）等操作，以便后续构建索引时更高效。LlamaIndex 默认会做基本的分块，你也可以自定义切分规则，例如按段落、按固定令牌数等。

本练习中，我们将演示如何从目录中加载多个文本文件，并观察文档列表和它们的元信息。例如，当我们想索引一组 Markdown 或 txt 文件时，可以直接使用 `SimpleDirectoryReader` 一次性读取整个目录中的文件。

### 编程任务

1. 创建一个名为 `my_texts/` 的文件夹，里面放两个 `.txt` 文件（例如 `doc1.txt`, `doc2.txt`）。每个文件写几行文字说明，或你自己感兴趣的文本。
    
2. 使用 `SimpleDirectoryReader` 读取该文件夹并生成 `Document` 列表。
    
3. 打印加载到的文档，查看它们的 `text`、`doc_id` 等属性。
    

```python
import os
from llama_index import SimpleDirectoryReader
# 假设你已经在本地新建了 my_texts 文件夹，并放入 doc1.txt, doc2.txt
# doc1.txt 内容示例: "这是文档1的文本，用于测试LlamaIndex加载功能。"
# doc2.txt 内容示例: "这是文档2的文本，用于测试多文档索引和检索。"
# 使用 SimpleDirectoryReader
# documents = SimpleDirectoryReader('my_texts').load_data()
print(f"共加载到 {len(documents)} 个文档。")
# 打印每个 Document 的基本信息
for i, doc in enumerate(documents):
    print(f"\n文档 {i+1} 的内容前100字符:")
    print(doc.text[:100])
    print("文档 ID:", doc.doc_id)
```

### 验证

如果 `my_texts` 文件夹确实有两个 `.txt` 文件，你应该能看到如下输出：

- “共加载到 2 个文档。”
    
- 每个文档的文本前 100 个字符，以及自动生成的 doc_id。
    

可以写个断言简单检查加载文档的数量是否正确：

```Python
assert len(documents) == 2, "文档数与预期不符，请检查是否放置了两个txt文件。"
print("第2步通过：成功加载并预览文档内容。")
```

当看到“第2步通过”并正确打印出文档内容，说明我们能够使用 LlamaIndex 的读取器来处理外部文本文件，为后续索引打下基础。

---

## 第3步：构建索引

### 理论介绍

LlamaIndex 有多种索引结构可选，包括：

- **GPTSimpleVectorIndex**：将文档切分后，生成文本 chunk 的 embedding 并存入向量索引，以向量检索方式查找相关 chunk。
    
- **GPTTreeIndex**：递归地把文档 chunk 组成一棵层级树，查询时自顶向下缩小范围。
    
- **GPTListIndex**：将所有 chunk 组成一个列表，依次检索。
    
- **其他高级索引**（例如 Composable Graph Index，支持多种子索引并组合）……
    

最常用、直观的往往是 `GPTSimpleVectorIndex`，因为它结合了向量检索的高效与可扩展性。本练习我们就用 `GPTSimpleVectorIndex` 体验如何构建索引，并查看切分好的 chunk 信息。

### 编程任务

1. 在上一步骤加载的 `documents` 基础上，使用 `GPTSimpleVectorIndex.from_documents(documents)` 构建一个向量索引。
    
2. 观察索引内部生成的 chunk 情况。可以打印索引对象中的 `index_struct.nodes`，看看文档被如何切分。
    
3. 将构建好的索引保存到磁盘（JSON 或者 pkl 文件），以便后续载入。
    

```python
from llama_index import GPTSimpleVectorIndex
# 构建向量索引
index = GPTSimpleVectorIndex.from_documents(documents)
# 观察索引结构
print("索引构建完成，查看索引中的节点数量：", len(index.index_struct.nodes))
# 保存索引到本地
index.save_to_disk("my_index.json")
print("索引已保存到 my_index.json")
```

### 验证

我们可以检查一下：

- `index.index_struct.nodes` 的长度应 > 0，说明至少有一个 chunk。
    
- 确认磁盘路径下生成了 `my_index.json`。
    

```Python
assert len(index.index_struct.nodes) > 0, "索引中没有任何节点，文档可能没有被切分？"
import os
assert os.path.exists("my_index.json"), "索引文件未成功保存，请检查 save_to_disk 调用。"
print("第3步通过：索引构建与保存成功。")
```

当输出“第3步通过”且本地目录确实有 `my_index.json` 文件时，说明索引已经创建完毕并持久化。我们下一步就可以使用它进行查询了。

---

## 第4步：查询索引

### 理论介绍

一旦索引构建好，就可以通过 `index.query(...)` 方法发起查询。LlamaIndex 会根据你的问题做如下事情：

1. 利用向量检索或其他结构检索到最相关的文本 chunk；
    
2. 将检索到的 chunk 作为上下文，拼接到大语言模型的 Prompt 中；
    
3. 调用 LLM（如 OpenAI GPT-3.5）得到最后回答。
    

这里可以控制很多参数，例如：

- `top_k`: 检索返回多少条文本片段；
    
- `response_mode`: 使用单轮回答，还是逐段生成回答等；
    
- 以及更多提示模板（prompt template）等高级设置。
    

### 编程任务

1. 加载上一步骤保存的索引 `my_index.json`。
    
2. 编写一个循环，让用户可以多次输入问题，然后调用 `index.query(user_input)` 来获取回答并打印。
    
3. 在此基础上，试着添加额外的 `top_k=3`、`response_mode="compact"` 等参数，感受回答会有何差异。
    

```python
from llama_index import GPTSimpleVectorIndex
# 从磁盘加载已保存的索引
# index = GPTSimpleVectorIndex.load_from_disk("my_index.json")
while True:
    user_query = input("\n请输入问题(或输入 'exit' 退出): ")
    if user_query.lower() == "exit":
        print("结束查询。")
        break
    response = index.query(
        user_query,
        top_k=3,
        response_mode="compact"
    )
    print("回答：", response)
```

### 验证

你可以提出一些与文本内容相关的问题，比如「文档1主要说了什么？」「这是哪一步的示例？」等，若能得到较为匹配文本含义的回答则说明查询流程正常。若回答与上下文无关，或 API 调用失败，请检查 API Key、网络、索引文件是否正确。

无法用严格断言去校验回答的正确性，但至少可以检测 `response` 是否为空：

```Python
assert response is not None and len(str(response)) > 0, "查询结果似乎为空，请检查索引是否正确。"
print("第4步通过：成功使用索引回答问题。")
```

如果输出“第4步通过”且回答与文档内容相符，就说明我们的索引查询工作流走通了！

---

## 第5步：自定义嵌入与向量检索

### 理论介绍

**嵌入（Embedding）** 是将文本转换为向量的核心过程，在向量索引中是基础。LlamaIndex 默认使用 OpenAI 的文本嵌入模型（如 `text-embedding-ada-002`），也可替换成其他开源模型（例如 Hugging Face 模型）。在向量检索流程中，我们通常会：

1. 对每个文本块生成向量 (embedding)；
    
2. 存入向量数据库或内存；
    
3. 用户提问时，将问题也转换为向量，然后在向量空间里找最近的文档块；
    
4. 选出最相关的文本并供 LLM 参考生成回答。
    

本练习将演示如何自定义 `ServiceContext` 里的 embedding 模型，并指定给 LlamaIndex 使用。注意：若要用本地模型，需要先用 `pip install sentence-transformers` 等。

### 编程任务

1. **自定义 Embedding**：使用示例的 `HuggingFaceEmbedding`（如果你想用 HuggingFace），并通过 `ServiceContext` 将其传给 LlamaIndex。
    
2. 重建索引并验证查询是否正常。
    

```python
from llama_index import ServiceContext, LLMPredictor
from llama_index.embeddings import HuggingFaceEmbedding
from llama_index import GPTSimpleVectorIndex
from transformers import AutoModel, AutoTokenizer
# 示例：使用 sentence-transformers/all-MiniLM-L6-v2 作为Embedding
embed_model = HuggingFaceEmbedding(model_name="sentence-transformers/all-MiniLM-L6-v2")
# 若要自行下载本地模型，请确保已经安装 transformers 等依赖
service_context = ServiceContext.from_defaults(embed_model=embed_model)
# 从前面的 documents 或从磁盘再次加载都可
index_custom_embed = GPTSimpleVectorIndex.from_documents(
    documents,
    service_context=service_context
)
response = index_custom_embed.query("这是自定义embedding后的提问", top_k=2)
print("回答：", response)
```

### 验证

如果使用 Hugging Face 模型，索引在构建时会下载或加载相应的权重文件。查询时如能顺利得到回答，即说明嵌入生成和向量检索流程正常。可做一些断言检测 `response` 不为空：

```Python
assert response is not None and len(str(response)) > 0, "自定义嵌入后查询结果为空，请检查配置。"
print("第5步通过：成功使用自定义向量模型生成答案。")
```

如果输出“第5步通过”，表示你已完成在 LlamaIndex 中替换默认 embedding 的操作，这能给你更多灵活性、甚至离线部署的能力。

---

## 第6步：结合 Prompt 与 LLM

### 理论介绍

LlamaIndex 在检索到相关文本后，会将这些文本与**提示（Prompt）**拼接后传递给 LLM，以生成回答。默认有一套内置提示模板，但有时我们需要自定义 prompt 来决定回答风格、输出格式等。

常用方式：

- 修改系统提示 (system prompt) 或添加前缀/后缀；
    
- 使用 `LLMPredictor` 指定不同的 LLM 模型和参数（如 temperature）；
    
- 在调用 `index.query(...)` 时传入自定义的 prompt template。
    

### 编程任务

1. 创建自定义 Prompt，例如要求返回答案时附带引用文档 ID；
    
2. 构建新的索引或在 query 时传入这个自定义 Prompt；
    
3. 对文档进行一次查询，看看是否携带了文档引用等信息。
    

下面是一种简易做法，利用 `response_mode="compact"` + `include_summary=True`：

```Python
from llama_index import LLMPredictor, GPTSimpleVectorIndex
import openai
openai.api_key = "YOUR_OPENAI_API_KEY"  # 替换为你的Key
custom_llm_predictor = LLMPredictor(
    llm_kwargs={
        "temperature": 0.7,
        "max_tokens": 512,
    }
)
index_with_prompt = GPTSimpleVectorIndex.from_documents(
    documents,
    llm_predictor=custom_llm_predictor
)
query = "请简要说明文档的主要内容，并给出引用来源"
response = index_with_prompt.query(
    query,
    response_mode="compact",
    include_summary=True
)
print("回答：", response)
```

### 验证

若回答中出现了文档的引用（例如“(Document: xxx)”），或至少能较详细地引用文档内容，就说明 prompt 生效了。我们可以通过关键词检查：

```Python
assert "Document" in str(response), "回答中似乎没有引用文档信息，请检查 prompt 或参数设置。"
print("第6步通过：自定义 Prompt + LLM 参数成功。")
```

当输出“第6步通过”时，你已理解了如何控制提示与 LLM 参数，从而定制回答格式。

---

## 第7步：增量更新与多文档索引

### 理论介绍

在实际生产场景中，我们往往不断有新文档到来，需要对索引进行**增量更新**（Incremental Update）。对于 `GPTSimpleVectorIndex` 而言，可以在已有索引基础上直接插入新的 Document，而不用重建整个索引。对于多文档管理，你也可以选择使用**组合索引**（ComposableGraph Index），让多个索引协同工作。增量更新可以避免大规模重复计算，提升效率。下面我们演示**向现有索引添加新文档**。

### 编程任务

1. 载入我们之前的 `index`；
    
2. 准备一个新文本文件（如 `doc3.txt`），用 `SimpleDirectoryReader` 或其他方式读入并转换成 `Document`；
    
3. 调用 `index.insert(Document(...))` 将新文档增量添加到索引；
    
4. 再次查询，验证是否可以检索到新文档的内容。
    

```python
# 假设已有 index
index = GPTSimpleVectorIndex.load_from_disk("my_index.json")
# 新增文档
new_document = Document("这是一份新增文档的内容，用于演示增量更新。")
index.insert(new_document)
# 验证增量后的索引节点
print("新增文档后，索引节点数：", len(index.index_struct.nodes))
# 保存更新后的索引
index.save_to_disk("my_index_updated.json")
print("已保存增量更新后的索引到 my_index_updated.json")
# 试着查询新文档内容
resp = index.query("增量更新文档提到什么？")
print("回答：", resp)
```

### 验证

- 检查 `len(index.index_struct.nodes)` 是否比之前更多；
    
- 查询时回答应能包含 “新增文档” 之类的信息。
    

```Python
assert len(index.index_struct.nodes) > 1, "索引节点数未增加，增量插入可能失败！"
print("第7步通过：索引增量更新与多文档支持成功。")
```

当输出“第7步通过”且回答中确实能体现新文档内容，就表示我们掌握了对索引进行动态更新的基本方法。

---

## 第8步：进阶功能与评估

### 理论介绍

到这一步，我们已经掌握了 LlamaIndex 的基础使用流程：**加载文档 → 构建索引 → 查询**，并能自定义 Prompt、Embedding、增量更新多文档。对于更复杂的需求，LlamaIndex 还提供：

1. **Composable Graph**：可将多种索引结构组合在一起，让同一查询在不同索引间路由；
    
2. **高级 Prompt 模板**：使用 PromptHelper 自定义分割策略、上下文拼接策略；
    
3. **外部向量数据库**：如 Milvus, Faiss, Pinecone 等，方便大规模向量存储；
    
4. **性能评估**：在没有明确答案的生成式任务中，可以结合评分模型或人工评审；若是有确定性答案的 Q&A，可以对比回答与标准答案的匹配度。
    

本步骤简单演示如何做一个最小的**自动化评估**：给定一些 “问题-正确答案” 测试对，可以让 LlamaIndex 逐个查询并计算回答的平均得分（如 BLEU/ROUGE 简易实现）。

### 编程任务

1. 假设我们有一组测试问答对 test_data =[("问题1","答案1"),...]；
    
2. 用索引对每个问题进行 query，获得回答 answer；
    
3. 用一种简单匹配算法评估回答的准确度；
    
4. 统计并输出平均分数。
    

示例代码（仅供参考）：

```python
# 假设已经有 index
test_data = [
    ("这是第一个问题?", "这是第一个问题的答案"),
    ("这是第二个问题?", "这是第二个问题的答案"),
]
def simple_eval_score(pred: str, gold: str):
    """
    简易打分逻辑：如果 gold 在 pred 中出现，就给1分，否则0分。
    真实应用中可用更复杂算法，如相似度计算、BLEU等。
    """
    return 1.0 if gold in pred else 0.0
total_score = 0.0
for q, gold_ans in test_data:
    response = index.query(q)
    pred_ans = str(response)
    score = simple_eval_score(pred_ans, gold_ans)
    total_score += score
    print(f"Q: {q}\nPred: {pred_ans}\nGold: {gold_ans}\nScore: {score}\n---")
avg_score = total_score / len(test_data)
print("平均得分:", avg_score)
```

### 验证

若只要确认评估脚本能正常执行，无报错即可。更严格的自动化评测需要更完善的打分算法和足够多的测试对。

```Python
assert avg_score >= 0.0, "评估脚本出现错误，得分应当 >= 0"
print("第8步通过：完成基本的查询结果评估流程演示。")
```

当输出“第8步通过”且打印了每条测试的对比情况，就说明你已经知道了如何在 LlamaIndex 中做基本的自动化评估流程（虽然实际项目会更复杂）。

---

## 总结

**恭喜你！** 完成以上 8 步后，你已经完成了一个由浅入深的 **LlamaIndex** 上手教程。在这个类似 Exercism 风格的实践中，我们依次学习了：

1. LlamaIndex 的安装和最小示例；
    
2. 如何加载并预处理文本文档；
    
3. 构建索引、保存加载索引的流程；
    
4. 发起查询，获取与文档内容相关的回答；
    
5. 自定义 embedding，使用开源或自有模型进行向量检索；
    
6. 调整 Prompt 与 LLM 参数，定制回答风格；
    
7. 为索引增量插入新文档，支持多文档场景；
    
8. 简单地评估回答质量。
    

通过这些步骤，你已经具备了用 LlamaIndex 做文档问答、知识库构建的核心能力。接下来，你可以尝试更多高级功能，比如组合索引、外部存储、更加精细的 Prompt 或大规模数据分布式处理等，也可以探索在实际项目中结合前端、API，构建一个可交互的「文档聊天机器人」。

**祝你在大语言模型应用的道路上越走越远，玩转 LlamaIndex！**
