---
draw:
tags: []
title: protobuf
date created: 2024-12-10
date modified: 2024-12-27
---

你有一份「秘密文件」（`byte[]` 的二进制数据），想把它变成一本「故事书」（你的自定义Java对象DailySortPrice）。
可是这个二进制数据是用「特别的语言」写的，别人不一定看得懂。
Protobuf 就像是一本「词典」，按照 `.proto` 文件的定义将二进制数据翻译成一种「标准语言」（Protobuf对象）。
等你拿到这份标准语言描述的东西后，再按照你的业务逻辑和习惯，把它转换成你的「故事书」（DailySortPrice 对象）格式。

## run

1. protobuf
	1. protobuf 中新增标签号 `repeated string propertyValueTags=119;`
	2. `protoc -I=$SRC_DIR --java_out=$DST_DIR $SRC_DIR/PriceEntryProtoBuf.proto`
	3. `PriceEntryProtoBufUtils.java`

这是一条使用 `protoc`（Protocol Buffers 编译器）编译 `.proto` 文件的命令。下面对命令各部分进行解释：

```Java
protoc -I={src_dir}/com/qunar/hotel/protobuf/definition \
       --java_out={src_dir} \
       {src_dir}/com/qunar/hotel/protobuf/definition/PriceEntryProtoBuf.proto
```

1. **protoc**：
    `protoc` 是 Google Protocol Buffers 的编译器程序，用于将 `.proto` 文件生成对应语言的代码文件（如 Java、C++、Python 等）。
    
2. **-I={src_dir}/com/qunar/hotel/protobuf/definition**：
    `-I` 是 `--proto_path` 的缩写形式，代表的是 "include path" 或 "import path"。在 `protoc` 命令中，通过 `-I` 或 `--proto_path` 指定一个目录作为 `.proto` 文件的搜索路径。当你的 `.proto` 文件中使用 `import` 语句导入其他 `.proto` 文件时，编译器会从指定的 `-I` 目录开始查找相应的文件。
    在本例中，`-I` 指定了 `src_dir/com/qunar/hotel/protobuf/definition` 为包含 `.proto` 文件的目录，这样 `protoc` 就能在这个路径下找到要编译的 `.proto` 文件。
    
3. **--java_out={src_dir}**：
    `--java_out` 参数指定生成的 Java 源代码文件要输出到哪个目录。
    在此处，`{src_dir}` 为你的源代码根目录，编译器会在该目录下自动创建对应的 Java 包目录结构，并将生成的 `.java` 文件写入其中。
    
4. **{src_dir}/com/qunar/hotel/protobuf/definition/PriceEntryProtoBuf.proto**：
    这是要编译的 `.proto` 文件的路径。
    该 `.proto` 文件中定义了数据结构（如消息类型、枚举）等，当执行此命令后，`protoc` 会依据其定义生成相应的 Java 类文件。
    

## 为什么不直接从 byte 到业务对象

[适配器模式](适配器模式.md)的思想

- **Input（byte[]）**：原始数据（难以直接理解）
- **Protobuf对象**：通过 Protobuf 规范解析出的中间模型（统一标准格式）
- **业务对象**：适配和转换后的最终对象，适合直接在业务逻辑中使用
- **Protobuf对象是中间翻译层**：
    Protobuf 编译器根据 `.proto` 文件生成的代码里有 `parseFrom()` 等方法，这些方法可以直接把 byte[]按照 `.proto` 里定义的格式解析成一个清晰的、可访问的对象（`DailySortPriceProtoBuf.DailySortPrice`）。
    如果你跳过这一步，`DailySortPrice` 对象就只能看到一串没头没尾的 byte[]，不知道应该怎样拆分和解析每个字段。
- **自定义对象可能与协议数据结构不同**：
    有时候，你的业务对象 `DailySortPrice` 的属性名、结构、类型可能和 Protobuf 定义的结构有点差异。通过先获得一个 Protobuf 对象，你就可以按照自己的需求有序地「拷贝」和「转换」数据，灵活处理数据类型、默认值、数据清洗等逻辑。这样业务对象不一定要和 Protobuf 定义一模一样的字段和类型。
![image.png|300](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F12%2F15%2F02-19-19-81bf42ee709d520ea6750faa9d5a540d-202412150219859-ac6f8c.png)
