---
uid: 8369fe4a-eac7-4d3d-b25d-c645116552c9
date created: 2025-07-19
date modified: 2025-09-29
---
# Summary

| 对比点 | [[JSON]]                    | JSON Schema           |
| --- | --------------------------- | --------------------- |
| 本质  | 数据内容                        | 数据结构和约束               |
| 角色  | “实例”                        | “蓝图/规范”               |
| 含义  | 存放了实际值（title、content、url 等）| 定义这些值应该是什么类型、必填不必填    |
| 类比  | 一份填好的表格                     | 表格的模板（告诉你要填哪些字段、什么类型）|

# Cues

``

# Notes

```json
{
  "type": "object",
  "properties": {
    "title": { "type": "string" },
    "content": { "type": "string" },
    "images": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "url": { "type": "string", "format": "uri" },
          "fileid": { "type": "string" }
        },
        "required": ["url", "fileid"]
      }
    }
  },
  "required": ["title", "content", "images"]
}

```
