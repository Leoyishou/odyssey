---
comment_id: fec45974
date created: 2024-06-04
date modified: 2025-02-06
draw: null
title: HTTP 1.1
---
Web 开发必须掌握的三个技术：Token、Cookie、Session - 华为云开发者联盟的文章 - 知乎  
https://zhuanlan.zhihu.com/p/171787680  
还分不清 Cookie、Session、Token、JWT？- 老刘的文章 - 知乎  
https://zhuanlan.zhihu.com/p/164696755

[token](token.md) [cookie](cookie) 和 [session](session)、[JWT](JWT) 都是为了解决 http 无状态的问题。

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F10%2F02%2F16-27-45-2c1c8f819a61783f47d2aa147dac634c-202410021627346-e5533d.png)

## 为什么永远只用 POST

支持 POST 包裹rpc的方式与后台交互，一份api文档前后端通用。RESTful这玩意就是自 high，一结合业务就拧巴、纠结，碰上杠精挑刺更是痛不欲生，而且还割裂了前后端的语义。
曾经我也是 rest 的拥趸，谁用谁难受。接口调用的本质是 RPC。
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F11%2F03%2F22-04-39-550660c31548ff82acecc91e7b1ab380-202411032204948-cdf794.png)

GET 跟 POST 相比有什么优势呢？答曰：GET 接口返回的内容可以被缓存！（有人可能会说 GET 不能附加 body，其实这是不对的。GET 请求也可以像 POST 那样提交数据。）也就说，GET 比 POST 唯一的区别是 GET 请求的结果可以被缓存。这恰恰有很大的问题。我们对外提供接口，内容一般是动态的。如果中间节点缓存了结果，就可能出现意想不到的结果。所以说，在动态接口*领域，GET 的这个缓存特性反而是缺点，而非优点。

方法用 POST，路径表示接口，body 传输请求内容。从这个角度上讲，http 协议只不过是 RPC 调用的一种承载方式。RPC 要解决的问题无非就是接口映射+（也就调哪个接口）和编码问题（json/pb等）。至于具体的 http 请求类型，则不是很重要，只要不给大家带来麻烦就好。

而前面说过，GET 可能有一些问题，所以不如 POST。现在比较流行的grpc本质上也是 http post 请求

## 1.1的四个升级

1、HTTP/1.0 短链接 用一次建立一次 HTTP/1.1 长连接 Connection:keep-alive 一段时间内一直用一条连接  
2、更多的错误状态响应码:在HTTP1.1中新增了24个错误状态响应码  
3、更多可供选择的缓存头来表示相应的缓存策略  
4、允许只请求资源的某个部分，充分利⽤带宽和连接

1. HTTP 请求方式  
HTTP（Hypertext Transfer Protocol）是一种在计算机网络中用于传输超媒体文档的应用层协议。HTTP 协议定义了客户端和服务器之间的通信规则，并规定了客户端向服务器发送请求时需要采用的请求方法（请求方式）。

常见的 HTTP 请求方式有四种：

POST（添加）
GET（查询）
DELETE（删除）
PUT（修改）
下面将分别对这四种请求方式进行详细介绍。

1. POST 请求  
POST 请求用于向指定资源提交数据，通常会导致服务器端的状态发生变化。例如，在 Web 表单中填写用户信息并提交时，就是使用 POST 请求方式将表单数据提交到服务器存储。

使用 POST 请求方式提交的数据会被包含在请求体中，而不像 GET 请求方式那样包含在 URL 中。因此，POST 请求可以提交比 GET 更大的数据量，并且相对更安全。

2.1. 例子  
下面是一个 POST 请求的例子：

```Java
POST /api/user HTTP/1.1  
Host: example.com  
Content-Type: application/json  
Content-Length: 123

{  
   "name": "John Doe",  
   "email": "johndoe@example.com",  
   "age": 30  
}  
```

上述代码表示向 example.com 的 /api/user 资源发送一个 POST 请求，请求体中包含了一个 JSON 格式的用户信息。

2.2. 优缺点  
POST 请求的优点包括：

可以提交比 GET 更大的数据量。
相对更安全，因为请求参数不会被包含在 URL 中。
POST 请求的缺点包括：

对服务器性能的影响较大。
不适用于对同一资源进行多次操作。
2.3. 应用场景  
向服务器提交表单数据。
向服务器上传文件。
创建资源或提交数据到服务器。
3. GET 请求  
GET 请求用于向指定资源发出请求，请求中包含了资源的 URL 和请求参数。服务器端通过解析请求参数来返回相应的资源，不会修改服务器端的状态。

使用 GET 请求方式提交的数据会被包含在 URL 中，因此易于被缓存和浏览器保存，但也因此不适合用于提交敏感数据。

3.1. 例子  
下面是一个 GET 请求的例子：

GET /api/user?id=123 HTTP/1.1  
Host: example.com  
上述代码表示向 example.com 的 /api/user 资源发送一个 GET 请求，请求参数中包含了用户的 ID。

3.2. 优缺点  
GET 请求的优点包括：

可以被缓存和浏览器保存。
对服务器性能的影响较小。
GET 请求的缺点包括：

不适合用于提交敏感数据。
仅适用于对资源进行查询操作，不能修改服务器端的状态。
3.3. 应用场景  
获取资源信息。
对资源进行查询操作。

```http
请求行
GET /api/example?param1=value1&param2=value2 HTTP/1.1

请求头
Host: www.example.com
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36
## 表明数据类型
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8
## 表明语言
Accept-Language: en-US,en;q=0.5
## 表明压缩算法
Accept-Encoding: gzip, deflate, br
## 流式输出和固定长度输出，二选一
Transfer-Encoding: chunked” 
Content-Length：1000
## 表明字符集
Accept-Charset: gbk, utf-8

Connection: keep-alive
Upgrade-Insecure-Requests: 1
```

![image.png#pic_center|650](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F06%2F05%2F20-47-27-18d404bb7fff4dec8234fe518bf79e79-20240605204725-23c952.png)

## 请求头

```http
GET /index.html HTTP/1.1
Host: www.example.com
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Connection: keep-alive
Cookie: session_id=abc123; user_id=789
```

### 1. 通用头部

- **Host**: 指定请求的服务器域名和端口号
- **User-Agent**: 标识发送请求的客户端软件的应用类型、操作系统、软件开发商以及版本号
- **Accept**: 指定客户端能够接受的内容类型
- **Accept-Language**: 指定客户端可以接受的语言
- **Accept-Encoding**: 指定客户端可以接受的内容编码方式，如gzip
- **Connection**: 指定与连接相关的属性，如 Keep-Alive

### 2. 请求特定头部

- **Referer**: 标识请求的来源页面
- **Origin**: 在跨域请求中，标识请求的来源
- **Cookie**: 包含先前由服务器通过 Set-Cookie 发送的 HTTP cookie
- **Cache-Control**: 指定请求和响应遵循的缓存机制

### 3. 实体头部

- **Content-Type**: 请求体的 MIME 类型 (用于POST和PUT请求)
- **Content-Length**: 请求体的长度，以字节为单位

## 4. 安全相关头部

- **Authorization**: 包含用于验证用户代理身份的凭证
- **X-CSRF-Token**: 用于防止跨站请求伪造（CSRF）攻击

### 5. 其他常见头部

- **If-Modified-Since**: 只有当内容在指定日期之后被修改才返回
- **If-None-Match**: 比较实体标记（ETag）
- **Upgrade**: 要求服务器升级到另一个协议

[错误码](错误码.md)

## URL

需要对JSON数据进行URL编码（也称为百分号编码）处理主要有以下几个原因：

1. 特殊字符处理：
   URL中有许多字符是保留的或有特殊含义的，比如 `&`, `=`, `?`, `/` 等。JSON数据中可能包含这些字符，如果不进行编码，可能会导致URL解析错误。

2. 空格和换行符：
   JSON数据中的空格、换行符等在URL中是不允许直接使用的。编码可以将这些字符转换为合法的URL字符。

3. 非ASCII字符：
   如果JSON数据中包含非ASCII字符（如中文），编码可以确保这些字符被正确传输和解析。

4. 避免歧义：
   编码可以避免服务器在解析参数时产生歧义。例如，未编码的JSON中的冒号可能被误解为URL中的端口分隔符。

5. 安全性：
   编码可以防止一些基于注入的攻击，因为它会转义潜在的危险字符。

6. 兼容性：
   不同的服务器和客户端可能对URL中允许的字符有不同的限制。编码可以提高跨平台兼容性。

在你的具体例子中，JSON数据包含了大量的特殊字符（如 `{}`, `"`, `:` 等），这些都需要被编码以确保它们能作为一个完整的参数值被正确传递和解析。

虽然对于小型的、简单的数据，直接在URL中传递未编码的数据可能也能工作，但对于复杂的JSON数据，编码处理是一个更安全、更可靠的做法。这也是为什么在之前的回答中，我建议使用POST请求，将JSON数据放在请求体中，而不是直接放在URL中，因为这样可以避免URL长度限制，并且更适合传输大量复杂数据。
