---
draw:
tags: []
title: HTTP
date created: 2025-01-05
date modified: 2025-01-08
---

`config.headers['Authorization']= 'Bearer ${token}'` 是设置 HTTP 请求的认证头部，用于向服务器证明请求者的身份。它遵循 Bearer token 认证方案。

这个头部的工作流程是：

1. 用户登录后，服务器生成一个 JWT (JSON Web Token)
2. 前端存储这个 token（通常在 localStorage 中）
3. 之后的每个请求都在 Authorization 头部带上这个 token
4. 服务器通过验证这个 token 来识别用户身份

实际请求中的样子：

```http
GET /api/user/profile HTTP/1.1
Host: example.com
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

其中：

- `Bearer` 是认证方案，表明这是一个持有者令牌
- 后面的字符串是实际的 JWT token

在你的代码中的作用是：

```javascript
// 拦截每个请求，添加认证信息
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('jwt_token');
  if (token) {
    config.headers['Authorization'] = `Bearer ${token}`;
  }
  return config;
});
```

这样服务器就能知道：

1. 请求来自哪个用户
2. 这个用户是否有权限访问请求的资源
3. token 是否已过期
4. token 是否被篡改
