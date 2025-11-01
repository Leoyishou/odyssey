---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-28
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  



# 🧩 Cues



# 🪞Notes
其中，DeepSeek OCR和 Nanonets都是3B，PaddleOCR更是只有 0.9B，部署起来非常轻松。从好友刘聪的实测来看，在文字识别精度上，百度PaddleOCR的表现要好于DeepSeek OCR。机打纯文字，两者都能够做到一字不差。但稍微上点难度（比如形近字、手写字、竖牌字、数学公式、表格），DeepSeek OCR就开始出错了。长期深耕OCR领域的飞飞桨Paddle，采用的是传统结构：一个模型识别文档的结构，一个模型识别文档的内容，两者结合进行结构化输出。如果是企业生产级使用OCR，建议首选这个模型。DeepSeek OCR则是全新的实验路线，试图将文本转为图像token进行压缩，做到所谓“一图胜干言”。DeepSeek志不在OCR，这个模型是他们对“文压图“新路线的实验性探索，在OCR 领域验证通过了，然后就顺手把自己的内部工具放出来了。有意思的是，几乎同一时间，智谱和清华大学也提出了类似想法—Glyph架构，试图将文本转为图像，再通过视觉语言模型*（VLM）进行处理。DeepSeek和智谱，都在试图探索上下文窗口的上限。DeepSeek 说，可以压缩10倍，精度97%；智谱说，可以压缩3-4倍，保持与前沿LLM的精度不变。这真的太有趣了，国内模型厂商都在自己最擅长的领域，一同去探索AI的上限。而且，他们还都开源。
