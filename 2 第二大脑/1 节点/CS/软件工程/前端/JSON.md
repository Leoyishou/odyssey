---
draw:
title: JSON
tags: []
date created: 2024-06-20
date modified: 2024-12-27
---

JSON 数据中包括 [文字](文字.md) 和 [数字](数字.md) 两类信息

"" 是唯一用来确定字符串 [文字](文字.md) 的，如果 "" 中还有 "" 就会干扰 JSON 内容的识别，所以如果出现了 "" 嵌套，里面的 "" 一定要通过 [转义](转义.md) 成其他字符

## JSON 只支持下面这六种类型

```json
{
  "number": 42,                               // 数字 number
  "string": "Hello, JSON",                    // 字符串
  "boolean": true,                            //  布尔值
  "nullValue": null,                          //  空值
  "array": [1, "two", false, null],           //  数组 Array
  "object": {                                //  Object
    "nestedNumber": 3.14,
    "nestedString": "Nested value"
  }
}
```
