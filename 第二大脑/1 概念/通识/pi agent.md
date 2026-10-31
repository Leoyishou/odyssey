---
uid: f2731c74-c6b6-4e92-8f95-5704a6fddd72
date created: 2026-10-31
date modified: 2026-10-31
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 



# 🧩 Cues



# 🪞Notes

除了 `find` 和 `grep`，Pi 内置的还有这几个：⁠[![](https://img.notionusercontent.com/ext/https%3A%2F%2Fwww.google.com%2Fs2%2Ffavicons%3Fdomain%3Dpi.dev%26sz%3D64/raw?imgBuildSrc=createUnsignedImageVariantURL)](https://pi.dev/docs/latest/cli)⁠

| 工具           | 作用               | 要点                                                                    | 默认开启 |
| ------------ | ---------------- | --------------------------------------------------------------------- | ---- |
| `read`       | 读文件              | 图片会作为附件传给模型。文本默认读前 2000 行，单行超过 2000 字符会截断，大文件用 `offset` / `limit` 分段读 | ✅    |
| `write`      | 新建或覆盖整个文件        | 父目录不存在时会自动创建                                                          | ✅    |
| `edit`       | 对文件做精确的文本替换      | 要替换的原文必须**完全一致**，连空格都要对上。原文找不到、或者出现多次都会报错                             | ✅    |
| `bash`       | 执行 shell 命令      | 返回 stdout 和 stderr，可以设 `timeout`                                      | ✅    |


⁠[![](https://img.notionusercontent.com/ext/https%3A%2F%2Fwww.google.com%2Fs2%2Ffavicons%3Fdomain%3Dgithub.com%26sz%3D64/raw?imgBuildSrc=createUnsignedImageVariantURL)](https://github.com/nicobailon/pi-mono/blob/main/packages/coding-agent/README.md)⁠ ⁠[![](https://img.notionusercontent.com/ext/https%3A%2F%2Fwww.google.com%2Fs2%2Ffavicons%3Fdomain%3Dpi.dev%26sz%3D64/raw?imgBuildSrc=createUnsignedImageVariantURL)](https://pi.dev/docs/latest/cli)⁠

**按用途分成三组**

- **找**：`ls` 看目录结构，`find` 按路径找文