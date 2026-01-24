---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
# 💡Summary

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2026%2F01%2F22%2F16-49-48-cb2c2d1d763229b1239194dd3e9cf08f-202601221649200-6b12b2.png)

## 三个代码级别


- **关于选项 1 (WebView / H5)：**
    - 这就是开发同学说的 **"套壳"**。
    - **优势**：开发速度极快，一份代码跑天下（Web、iOS、Android）。
    - **劣势**：性能相对较差，操作手感不如原生 App 流畅（比如滑动、转场动画），且上架 App Store 时如果被苹果审核发现只是简单的网页套壳，**有被拒审的风险**。

- **关于选项 3 (Capacitor)：**
    - 这也是 **"H5"** 的一种变体，但比纯 WebView 更高级一点。它提供了一个更标准的“壳”，让网页能更容易地调用手机功能（如相机、震动等）。
    - 对于 Web 项目转换来说，**Capacitor 往往比自己手写 WebView (选项1) 更稳健**。

- **关于选项 2 (RN)：**
    - 虽然也是用 React 写，但它是 **"渲染成原生组件"**。
    - 这意味着用户看到的按钮、列表是真正的 iOS 控件，而不是网页画出来的。
    - **关键点**：如果你的 Web 项目本身就是用 React 写的，选 RN 重构会比重写 Native 省事，但依然比直接套壳（选项 1/3）麻烦得多。

> [!Info]
> 一句话说清楚



作为新手 Apple 开发者，建议的流程是：

1. **Expo Go** → 日常开发调试
2. **TestFlight** → 邀请朋友帮忙测试（可邀请最多 10,000 人）
3. **App Store** → 正式上架
# 🧩 Cues

# 🪞Notes

## 如果只是自己用：
  - 完全可行：把自己加为"内部测试员"，无需任何审核
  - 唯一麻烦：每 90 天要重新 build 并上传一次（可以自动化）
  - 优势：不用管 App Store 的各种合规要求、截图、描述等

  更省事的替代方案

  如果你觉得 90 天续期麻烦，还有几个选择：

| 方案                   | 特点                                                        |
| :--------------------- | :---------------------------------------------------------- |
| TestFlight（当前方案） | 最简单，90天续期                                            |
| Ad Hoc 分发            | 直接装 .ipa，不过也要每年续签 provisioning profile          |
| PWA（添加到主屏幕）    | 零成本，不需要 Apple Developer 账号，但功能受限（无推送等） |
| 发布 App Store         | 一劳永逸，不过要填写信息过审核                              |

  对你这个"每日句子"App 来说，如果不需要推送通知、后台刷新等原生能力，其实 PWA 也是一个选择——把 web 版部署后，Safari
  打开添加到主屏幕就行，完全不需要付费账号。

  你想继续用 TestFlight 的方案，还是换成 PWA？