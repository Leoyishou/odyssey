2. protobuf
	1. protobuf 中新增标签号 `repeated string propertyValueTags=119;`
	2. `protoc -I=$SRC_DIR --java_out=$DST_DIR $SRC_DIR/PriceEntryProtoBuf.proto`
	3. `PriceEntryProtoBufUtils.java`


这是一条使用 `protoc`（Protocol Buffers 编译器）编译 `.proto` 文件的命令。下面对命令各部分进行解释：

```
protoc -I={src_dir}/com/qunar/hotel/protobuf/definition \
       --java_out={src_dir} \
       {src_dir}/com/qunar/hotel/protobuf/definition/PriceEntryProtoBuf.proto
```

1. **protoc**：  
    `protoc` 是 Google Protocol Buffers 的编译器程序，用于将 `.proto` 文件生成对应语言的代码文件（如 Java、C++、Python 等）。
    
2. **-I={src_dir}/com/qunar/hotel/protobuf/definition**：  
    `-I`（或 `--proto_path`）参数指定 `.proto` 文件的搜索路径。  
    在本例中，`-I` 指定了 `src_dir/com/qunar/hotel/protobuf/definition` 为包含 `.proto` 文件的目录，这样 `protoc` 就能在这个路径下找到要编译的 `.proto` 文件。
    
3. **--java_out={src_dir}**：  
    `--java_out` 参数指定生成的 Java 源代码文件要输出到哪个目录。  
    在此处， `{src_dir}` 为你的源代码根目录，编译器会在该目录下自动创建对应的 Java 包目录结构，并将生成的 `.java` 文件写入其中。
    
4. **{src_dir}/com/qunar/hotel/protobuf/definition/PriceEntryProtoBuf.proto**：  
    这是要编译的 `.proto` 文件的路径。  
    该 `.proto` 文件中定义了数据结构（如消息类型、枚举）等，当执行此命令后，`protoc` 会依据其定义生成相应的 Java 类文件。
    

**整体功能**：  
这条命令从指定的定义文件 (`PriceEntryProtoBuf.proto`) 中读取 Protocol Buffers 的消息结构，然后使用 `protoc` 将其编译生成对应的 Java 类文件，以便在 Java 项目中通过类型化类与方法访问和操作定义的消息数据结构。