---
date created: 2025-03-13
date modified: 2025-10-25
uid: b40a744d-7dc2-4de4-a874-4bbdafbb732a
---

postman可以解决测试繁琐的问题


# 环境配置

```javascript
// ✅ 推荐：base_url 不带尾部斜杠
base_url = "https://api.example.com"

// 接口路径带前导斜杠
GET /users
GET /orders/123

// ❌ 如果 base_url 带斜杠
base_url = "https://api.example.com/"
path = "/users"
结果: https://api.example.com//users  // 双斜杠

// ✅ 不带斜杠就没问题
base_url = "https://api.example.com"
path = "/users"
结果: https://api.example.com/users
```
