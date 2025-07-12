---
date created: 2024-11-29
date modified: 2025-07-10
uid: ef4aeda4-ff9f-4431-b8da-27f8a4aa11cc
---

让我来解释一下 SDK (Software Development Kit) 和 API (Application Programming Interface) 的主要区别：

1. 抽象层级不同：
- API 是一个接口规范，定义了软件组件之间如何交互的规则和方法
- SDK 是一整套开发工具集，通常包含了 API、开发工具、文档、示例代码、调试工具等

1. 使用方式不同：
- API 更像是一个协议或规范，你需要按照规定的格式发送请求和处理响应
- SDK 提供了更便捷的封装，通常包含了预设的类和方法，能直接在代码中调用

1. 使用场景举例：
假设你要接入支付功能：
- 只用 API 的话：需要自己处理 HTTP 请求、加密、签名等细节

```python
# 使用纯 API 的方式
import requests

def make_payment(amount):
    url = "https://api.payment.com/v1/pay"
    headers = {"Authorization": "Bearer " + get_token()}
    data = {"amount": amount, "currency": "CNY"}
    response = requests.post(url, headers=headers, json=data)
    return response.json()
```

- 使用 SDK 的话：这些都被封装好了

```python
# 使用 SDK 的方式
from payment_sdk import PaymentClient

client = PaymentClient("api_key")
result = client.create_payment(amount=100)
```

1. 开发便利性：
- API 更灵活但需要自己处理更多细节
- SDK 使用更简单，学习成本更低，但灵活性可能受限

1. 依赖关系：
- API 通常只依赖基础的网络请求库
- SDK 可能带来更多的依赖包，但提供了更完整的功能

简单来说：

- API 就像是一个插座的规范，定义了电压、相数等标准
- SDK 则是一个工具箱，除了插座规范，还包含了插头、电线、使用说明书等配套工具

大多数情况下，如果有官方提供的 SDK，建议优先使用 SDK，因为它能帮你处理很多细节问题，让开发更高效。但如果你需要更细粒度的控制，或者想要更轻量级的实现，直接使用 API 也是个不错的选择。
