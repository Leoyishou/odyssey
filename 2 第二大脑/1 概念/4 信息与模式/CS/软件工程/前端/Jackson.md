---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡 Summary  

加了 @JsonRawValue 之后，Jackson 会把这个字符串当成「已经序列化好的 JSON 片段」直接嵌进去，不再加外面那一层引号。


### @JsonRawValue

**`@JsonRawValue` = "这个字段里装的就是 JSON，直接拿出来用，别给我加引号转义"**

就像你买了一个礼盒装的商品，普通情况下 Jackson 会把它再包一层；加了 `@JsonRawValue` 后，Jackson 直接把礼盒里的东西拿出来，不再多此一举地包装。
```
数据库存：   "# 测试周末游玩 \n ## 第一天 \n ### 吃饭 \n ## 第二天\n ### 喝酒"
用了 JsonRawValue     "contentList": "# 测试周末游玩 \n ## 第一天 \n ### 吃饭 \n ## 第二天\n ### 喝酒"
不用JsonRawValue       "contentList": "\"# 测试周末游玩 \\n ## 第一天 \\n ### 吃饭 \\n ## 第二天\\n ### 喝酒\""
```


## 换行

```
这样：

1. DB 存的是真正的换行符 \n（ASCII 10），而不是 \ + n；

2. Java 读出来是带换行的字符串；

3. Jackson 序列化后，JSON 里会看到 \n（这是换行符的转义表示）；

4. 前端解析后，JS 字符串里就是“真正的换行”，放进 textarea 或你的 parseMarkdownToData 就能正常按行解析。
```
# 🧩 Cues



# 🪞Notes
