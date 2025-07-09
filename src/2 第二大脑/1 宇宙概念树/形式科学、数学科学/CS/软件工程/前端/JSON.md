---
comment_id: adbc7c6e
date created: 2024-06-20
date modified: 2025-02-06
draw: null
title: JSON
---
## 忽略null

JsonInclude.Include.NON_NULL 表示在序列化时会忽略值为 null 的字段，不会将它们包含在生成的 JSON 中。

JSON 数据中包括 [文字](2%20第二大脑/1%20宇宙概念树/形式科学、数学科学/数学/文字.md) 和 [数字 1](数字%201.md) 两类信息

"" 是唯一用来确定字符串 [文字](2%20第二大脑/1%20宇宙概念树/形式科学、数学科学/数学/文字.md) 的，如果 "" 中还有 "" 就会干扰 JSON 内容的识别，所以如果出现了 "" 嵌套，里面的 "" 一定要通过 [转义](转义.md) 成其他字符

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

下面这份讲解，会**一次性**帮你把「返回 JSON 对象」与「返回 JSON 字符串」这类常见的问题讲清楚、讲透彻，并通过几个不同的场景/语言举例，帮助你彻底弄懂。你可以在看完后对照自己项目里的实现方式，逐一排查并改进。

## Java 并不需要把所有 json 对应的对象都创建

可以通过 JSONObject 这个对象来做一个类似大 map 的东西

```java
JSONObject object = JSONObject.parseObject(encryptString);  
// 使用 header 中的 event_id 做去重  
String eventId = object.getJSONObject("header").getString("event_id");  
String cachedEventId = eventCache.getIfPresent(eventId);  
if (StringUtils.isNotEmpty(cachedEventId)) {  
    log.error("------飞书------:重复事件, event_id={}", eventId);  
    return "success";  
} else {  
    eventCache.put(eventId, eventId);  
}
```

---

## 一、问题背景：为什么会出现"嵌套字符串化 JSON"？

> 场景：飞书在验证回调地址时，需要你返回形如 `{ "challenge": "xxxx" }` 的响应。
> 结果：实际上飞书拿到的响应却变成了 `"{\"challenge\":\"xxxx\"}"`（加引号，里面又是一层 JSON）。

也就是说，==你本应该返回**一个 JSON 对象**，却变成了**返回 JSON 对象转成的字符串**==。对于任何 RESTful API（包括飞书的验证接口），如果它期望的响应是 JSON，那么我们就需要在响应中**直接提供 JSON 对象**，而不是一个字符串。

### 1.1 为什么会出现这层"嵌套的字符串"？

大多数情况下，这是因为：

1. **手动构造了 JSON 字符串** 然后又被框架（如 `Spring MVC` 的 `Jackson`）再次序列化了一次。
2. 或者在你的方法里本身返回了一个字符串类型的字段，比如 `data = "{\"challenge\":\"xxxx\"}"`，又被外层包装器（如自定义的返回对象、统一封装 `Result` 等）再做了一次序列化。

最终导致返回给客户端的内容里多了一层转义字符(`\"`)。

---

## 二、如何正确地返回 JSON 对象？

关键点是：**框架（Jackson 等）帮我们自动做序列化** 时，方法的返回值只需要是一个合适的"数据结构"（如 `Map`、自定义对象），它就会自动变成我们想要的 JSON 格式。

### 2.1 Spring Boot / Spring MVC 常见写法

1. **直接返回一个 Map：**

    ```java
    @RestController
    @RequestMapping("/callback")
    public class FeiShuController {
    
        @PostMapping("/verify")
        public Map<String, Object> verify(@RequestBody Map<String, Object> requestBody) {
            String challenge = (String) requestBody.get("challenge");
    
            // 构造返回 Map
            Map<String, Object> responseMap = new HashMap<>();
            responseMap.put("challenge", challenge);
    
            // Spring MVC 会自动将此 Map 序列化为 {"challenge": "xxx"} 返回
            return responseMap;
        }
    }
    ```

    - 优点：直观、简单、够用。
    - 缺点：如果返回字段很多，手写 Map 可能比较凌乱。
2. **返回自定义的 Java Bean：**

    ```java
    public class FeishuVerifyResponse {
        private String challenge;
    
        public FeishuVerifyResponse(String challenge) {
            this.challenge = challenge;
        }
        // getter & setter 略
    }
    
    @RestController
    @RequestMapping("/callback")
    public class FeiShuController {
        @PostMapping("/verify")
        public FeishuVerifyResponse verify(@RequestBody FeishuVerifyRequest request) {
            return new FeishuVerifyResponse(request.getChallenge());
        }
    }
    ```

    - `FeishuVerifyRequest` 就是你用来接收请求的实体类，里面包含 `challenge` 等字段。
    - 返回 `FeishuVerifyResponse`，框架会序列化成 `{"challenge":"xxxx"}`。
3. **如果你用的是 ResponseEntity：**

    ```java
    @PostMapping("/verify")
    public ResponseEntity<Map<String, String>> verify(@RequestBody Map<String, Object> requestBody) {
        String challenge = (String) requestBody.get("challenge");
    
        Map<String, String> response = new HashMap<>();
        response.put("challenge", challenge);
    
        return ResponseEntity.ok(response);
    }
    ```

    - 返回的依旧是一个 **JSON 对象**，而不是字符串。

### 2.2 Node.js / Express 常见写法

```js
app.post('/callback/verify', (req, res) => {
  // 从 req.body 里获取 challenge
  const { challenge } = req.body;
  
  // 以 JSON 格式返回
  res.json({ challenge });
});
```

- `res.json({...})` 会自动把对象序列化成 JSON 发送给客户端。

### 2.3 Python / Flask 常见写法

```python
from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/callback/verify', methods=['POST'])
def verify():
    data = request.get_json(force=True)
    challenge = data.get("challenge", "")
    return jsonify({"challenge": challenge})
```

- `jsonify(...)` 会把你传入的字典序列化成 JSON。

---

## 三、常见的错误写法举例

以下演示以 **Spring Boot** 为例，但原理各语言类似，都是**二次序列化**的问题。

### 3.1 在返回前自己用 `ObjectMapper` 转了 JSON，然后再被序列化

```java
@PostMapping("/verify")
public String verify(@RequestBody Map<String, Object> requestBody) {
    String challenge = (String) requestBody.get("challenge");
    
    // 错误做法：把对象手动转成了 JSON 字符串
    String jsonString = new ObjectMapper().writeValueAsString(Map.of("challenge", challenge));

    // 由于这里方法返回类型是 String，框架或你的控制器层又可能把这个字符串再装到一个统一的ResponseWrapper
    // 结果就会变成 "data": "{\"challenge\":\"xxxx\"}" 的形式

    return jsonString;
}
```

### 3.2 自定义一个统一返回对象，里面 `data` 字段是字符串

```java
public class ResponseWrapper {
    private int status;
    private String msg;
    private Object data;
    // getter / setter ...
}

@PostMapping("/verify")
public ResponseWrapper verify(@RequestBody Map<String, Object> requestBody) {
    String challenge = (String) requestBody.get("challenge");

    // 又手动做了序列化
    String challengeJson = new ObjectMapper().writeValueAsString(Map.of("challenge", challenge));

    // data 中保存的是字符串
    ResponseWrapper resp = new ResponseWrapper();
    resp.setStatus(0);
    resp.setData(challengeJson);

    // 外层序列化后 -> 
    // {
    //   "status":0, 
    //   "data":"{\"challenge\":\"xxxx\"}"
    // }
    return resp;
}
```

> 结果：飞书拿到的 `data` 只是一段字符串，它没法正常识别出 `{ "challenge": "xxxx" }` 这种 JSON 结构。

---

## 四、如何避免或修正这些错误？

1. **直接返回对象或 Map**（推荐做法）。不用自己去手动转 JSON 字符串，让框架（Jackson、Gson 等）自动做序列化。
    
2. 如果必须使用自定义的返回 Wrapper（如 `ResponseWrapper`），尽量让 `data` 字段可以接收任意对象，而不是仅仅字符串。不要在赋值给 `data` 时提前转成 JSON 字符串，让 `data` 保持"原始对象"，最后由统一序列化逻辑来处理。
    
    - **改进示例**：

        ```java
        ResponseWrapper resp = new ResponseWrapper();
        resp.setStatus(0);
        // 直接存放对象，不要预先转成字符串
        resp.setData(Map.of("challenge", challenge));
        // 返回后由 Jackson 一次性序列化 -> 
        // {
        //   "status": 0,
        //   "data": {
        //     "challenge":"xxxx"
        //   }
        // }
        ```

3. 确保请求头与返回头都符合 RESTful 习惯：
    
    - `Content-Type: application/json; charset=utf-8`
    - `Accept: application/json`
4. 如果你确实要返回原始字符串（比如你要自定义一些不规范的格式），可以指定 `produces="text/plain"` 并返回字符串。但飞书或其它平台明确要求 JSON 格式时，就不能这么做。
    

---

## 五、总结 & 拓展

1. **飞书的 URL 验证**：
    
    - 飞书在验证回调接口时，发送的请求体大致是：

        ```json
        {
          "challenge": "0c8d6139-b568-48bc-a6f4-c0bf75897682",
          "token": "K90clXMBsY5P4ejHpe0vQb1uZJSIJ0oj",
          "type": "url_verification"
        }
        ```

    - 你需要**1秒内**返回：

        ```json
        { 
          "challenge": "0c8d6139-b568-48bc-a6f4-c0bf75897682" 
        }
        ```

    - 如果你的接口返回的是：

        ```json
        {
          "status": 0,
          "data": "{\"challenge\":\"0c8d6139-b568-48bc-a6f4-c0bf75897682\"}"
        }
        ```

        那么就说明"有效的 JSON 数据"被再次包裹进了字符串，导致飞书识别不到正确的字段。

2. **通用原则**：
    
    - 当一个第三方平台或客户端要求返回 JSON 对象时，要让响应体**直接就是** JSON 对象本身。
    - 不要在 JSON 对象外面再加一层引号或转义后的字符串。
3. **调试技巧**：
    
    - Postman 或其他工具测试时，检查返回的数据内容；可以直接把返回值复制到 JSON 校验器（如 JSON Editor Online）里查看，如果发现它无法直接被解析为 JSON，或者里面有一堆 `\"` 之类的转义字符，你就要警惕"是否多了一层序列化"。

---

### TL;DR

- **错误做法**：返回了 `"{\"challenge\":\"xxxx\"}"` 这样的字符串，而非 JSON 对象。
- **正确做法**：让接口直接返回 `{"challenge":"xxxx"}` 对象（或者任何 key-value 形式），不要手动转 JSON 字符串，再让框架去二次处理。

掌握了以上这些，对付各种"JSON 嵌套序列化"都游刃有余了。祝你调试顺利，早日完成飞书回调验证！
