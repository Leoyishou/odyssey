---
uid: 21bab621-d415-4ca0-ad5c-92c612d2defd
---

## Iphone

1. iPhone 镜像
2. **Xcode 方案（XCUITest / WebDriverAgent）：**
	1. 用 Xcode 把一个测试 App（WebDriverAgent）签名后装到 iPhone 上⁠[![](https://img.notionusercontent.com/ext/https%3A%2F%2Fwww.google.com%2Fs2%2Ffavicons%3Fdomain%3Dgithub.com%26sz%3D64/raw?imgBuildSrc=createUnsignedImageVariantURL)](https://github.com/ykdojo/iphone-control)⁠。
	2. 它以 UI 测试的身份运行，用的是苹果的 XCUITest 框架，所以有权限读取界面元素、截屏，还能模拟点击、滑动和输入。
	3. 它在手机上开一个本地服务，电脑上的 Agent 通过 Appium 给它发指令⁠[![](https://img.notionusercontent.com/ext/https%3A%2F%2Fwww.google.com%2Fs2%2Ffavicons%3Fdomain%3Dgithub.com%26sz%3D64/raw?imgBuildSrc=createUnsignedImageVariantURL)](https://github.com/ykdojo/iphone-control)⁠。

## 安卓

1. **ADB：**从电脑端控制。手机通过 USB 或 Wi‑Fi 连到电脑，电脑发命令截屏、点击、输入⁠需要开 USB 调试，通常离不开电脑。
2. **无障碍：**从手机里控制。手机上装一个 App，给它无障碍权限，它就能直接读界面元素（按钮、文字）并执行操作⁠[![](https://app.notion.com/image/https%3A%2F%2Fcdn.prod.website-files.com%2F67e6b7885e182e1054b556db%2F6821e1e956d1d88bb85f74fd_favicon.png?userId=&cache=v2&imgBuildSrc=resolvePublicImageUrl)](https://www.callstack.com/blog/agent-device-ai-native-mobile-automation-for-ios-android)⁠。不需要电脑。



![CleanShot 2026-02-04 at 15.31.17@2x.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fmedia%2Fmedia_VNRQpeMUTr%2F2026%2F02%2F04%2F15-32-17-72fe912096043c2d7fdac36f1a0ff6e3-CleanShot%202026-02-04%20at%2015.31.17-2x-6d3c5e.png)
