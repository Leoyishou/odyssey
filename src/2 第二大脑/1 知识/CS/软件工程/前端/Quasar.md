---
draw:
title: Quasar
date created: 2025-04-07
date modified: 2025-04-07
---

Quasar 的音标是 `/ˈkweɪzɑːr/`

|选项|用途|是否完整应用|是否需要 Quasar CLI|典型场景|
|---|---|---|---|---|
|App with Quasar CLI|构建完整应用（多平台支持）|是|是|开发网站、移动端或桌面应用|
|AppExtension (AE)|为 Quasar 项目添加扩展功能|否|是|创建插件或功能模块|
|Quasar UI kit|提供 UI 组件给 Vue 项目使用|否|否|增强已有 Vue 项目的界面|

如何选择？

- 如果你想从零开始开发一个跨平台应用（如网站+移动端），选 App with Quasar CLI，然后根据需求挑模式（比如 SPA + Capacitor）。
    
- 如果你想为现有 Quasar 项目添加特定功能（比如集成 GraphQL），选 AppExtension。
    
- 如果你只想要 Quasar 的漂亮组件，用于非 Quasar 的 Vue 项目，选 Quasar UI kit。

你提到的选项是 Quasar Framework 提供的不同构建模式或工具，它们各自有不同的用途和适用场景。以下是对这三个选项的具体解释，帮助你理解它们的区别：

1. App with Quasar CLI (spa/pwa/ssr/bex/electron/capacitor/cordova)

这是使用 Quasar CLI（命令行界面）创建完整应用程序的选项。它允许你基于一个代码库构建不同类型的应用，具体取决于你选择的“模式”（mode）。这些模式包括：

- SPA (Single Page Application): 单页应用，适用于纯前端网页应用，运行在浏览器中，页面切换无需刷新。
    
- PWA (Progressive Web App): 渐进式网页应用，具备离线功能、可安装到桌面或手机等特性，接近原生应用体验。
    
- SSR (Server-Side Rendering): 服务端渲染，服务器生成 HTML 后发送给客户端，适合需要 SEO（搜索引擎优化）或快速首屏加载的场景。
    
- BEX (Browser Extension): 浏览器扩展，允许你用 Quasar 开发 Chrome、Firefox 等浏览器的插件。
    
- Electron: 桌面应用，通过 Electron 将你的网页应用打包为 Windows、macOS 或 Linux 的桌面程序。
    
- Capacitor: 混合移动应用，基于 Web 技术开发 iOS 和 Android 应用，Ionic 团队维护，现代替代 Cordova。
    
- Cordova: 混合移动应用，类似 Capacitor，但历史更悠久，生态更成熟，支持 iOS 和 Android。
    

区别: 这个选项是构建完整应用的起点，涵盖了从网页到移动端、桌面端的全平台开发。你可以根据项目需求选择一种或多种模式（例如同时支持 SPA 和 PWA）。它包含完整的项目结构（如路由、状态管理、UI 组件等）。

适用场景: 想开发一个功能完整的应用（网站、移动端或桌面端），需要 Quasar 的全部功能支持。

---

2. AppExtension (AE) for Quasar CLI

这是 Quasar 的“应用扩展”（App Extension），用于为现有的 Quasar 项目添加额外的功能或自定义模块，而不是创建一个完整的应用。它更像是一个插件系统，允许开发者扩展 Quasar CLI 的能力。

- 特点:
    
    - 可以添加自定义的启动脚本（boot files）、组件、指令等。
        
    - 通常用于封装第三方库（如 Axios、Apollo）或自定义功能，供其他 Quasar 项目复用。
        
    - 有两种套件：AE 套件（无 UI，纯逻辑扩展）和 UI 套件（含 UI 组件或指令）。
        
- 与 App with Quasar CLI 的区别:
    
    - AE 不是独立的应用程序，而是附加到现有 Quasar 项目上的功能扩展。
        
    - 创建 AE 时，你的目标是开发一个可分享的 npm 包，而不是直接运行的应用。
        

适用场景: 你想为 Quasar 项目开发一个可复用的功能模块，或者为社区贡献扩展包（比如集成某个第三方服务）。

---

3. Quasar UI kit

这是 Quasar 的 UI 组件库（Quasar UI），独立于 Quasar CLI，仅提供基于 Vue.js 的 UI 组件和样式，不包含完整的应用开发工具（如 CLI 的构建系统）。

- 特点:
    
    - 提供丰富的 Material Design 风格组件（如按钮、表格、对话框等）。
        
    - 可以集成到任何 Vue.js 项目中，不限于 Quasar CLI 创建的项目。
        
    - 支持多种使用方式：通过 Vite 插件、Vue CLI 或 UMD（直接通过 `<script>` 标签引入）。
        
- 与前两者的区别:
    
    - 与 App with Quasar CLI: UI kit 不包括项目脚手架（如路由、状态管理），只提供组件库；而 CLI 模式是全栈解决方案。
        
    - 与 AppExtension: UI kit 是现成的组件集合，无需自己开发；AE 是开发者自定义扩展的工具。
        

适用场景: 你已经有一个 Vue.js 项目，只想引入 Quasar 的 UI 组件，而不需要完整的 Quasar CLI 生态。

---

总结对比

希望这个解释能帮你理清它们的区别！你有具体想构建的项目吗？可以告诉我，我帮你推荐最适合的选项！

___

## 部署为移动应用程序

要将这个 Quasar 项目[[部署]]为移动应用程序（iOS 或 Android），你需要使用 Quasar 的 Capacitor 模式（推荐）或 Cordova 模式。这两种模式可以将你的 Web 代码打包成一个可以在移动设备上运行的原生应用。

以下是使用 **Capacitor** 模式部署的基本步骤：

1. **添加 Capacitor 模式:**
    * 在你的项目根目录（`quasar-project`）下运行命令来添加 Capacitor 模式：
        ```bash
        npx quasar mode add capacitor
        ```
    * 这会安装必要的依赖并在你的项目中创建一个 `src-capacitor` 目录。

2. **安装原生开发环境:**
    * **Android:**
        * 安装 Android Studio。
        * 安装必要的 Android SDK 和构建工具（Android Studio 通常会引导你完成此过程）。
        * 确保你的环境变量（如 `ANDROID_SDK_ROOT` 或 `ANDROID_HOME`）已正确设置。
    * **iOS (需要 macOS):**
        * 安装 Xcode。
        * 安装 CocoaPods (`sudo gem install cocoapods`)。
        * 确保你的 Apple Developer 账户已设置好（如果需要发布到 App Store）。

3. **配置应用标识符:**
    * 打开 `quasar.config.js` 文件，在 `capacitor` 部分修改 `appId`。这个 ID 是你应用的唯一标识符，通常采用反向域名格式（例如 `com.yourcompany.yourappname`）。

4. **构建 Web 资源:**
    * 运行以下命令来构建你的 Web 应用代码，并指定目标平台：
        * 构建 Android 应用：
            ```bash
            npx quasar build -m capacitor -T android
            ```
        * 构建 iOS 应用：
            ```bash
            npx quasar build -m capacitor -T ios
            ```
    * 这个命令会：
        * 执行标准的 Web 应用构建 (`quasar build`)。
        * 将构建好的 Web 资源复制到 `src-capacitor/www`。
        * 运行 `npx cap sync[android|ios]` 来同步 Web 资源和插件到原生项目中。

5. **在原生 IDE 中打开项目:**
    * **Android:**
        * 运行 `npx cap open android`。
        * 这将在 Android Studio 中打开位于 `src-capacitor/android` 的原生 Android 项目。
    * **iOS:**
        * 运行 `npx cap open ios`。
        * 这将在 Xcode 中打开位于 `src-capacitor/ios` 的原生 iOS 项目。

6. **配置原生项目 (可选但通常需要):**
    * 在 Android Studio 或 Xcode 中，你可能需要：
        * 配置应用图标和启动画面 (Splash Screen)。Quasar 提供了 [Icon Genie](https://quasar.dev/icongenie/introduction) 工具来简化这个过程。
        * 配置签名证书，以便进行真机测试和发布。
        * 调整权限 (Permissions) 或其他原生设置。

7. **构建和运行应用:**
    * **Android Studio:** 使用菜单 `Build > Build Bundle(s) / APK(s)` 来构建 APK 或 App Bundle。使用 `Run > Run 'app'` 在模拟器或连接的设备上运行。
    * **Xcode:** 选择你的目标设备（模拟器或连接的设备），然后点击 "Run" 按钮 (▶) 来构建和运行应用。使用 `Product > Archive` 来构建用于发布的 `.ipa` 文件。

**关键点:**

* **原生环境:** 最复杂的部分通常是正确安装和配置 Android Studio 或 Xcode 及其依赖项。请务必遵循 Quasar 和 Capacitor 的官方文档。
* **Capacitor 配置:** `capacitor.config.json` 文件（位于项目根目录）和原生项目本身的配置文件（如 `AndroidManifest.xml` 或 `Info.plist`）包含重要的应用设置。
* **Quasar 文档:** Quasar 官方文档有非常详细的关于 Capacitor 开发的指南：
    * [Quasar Capacitor 开发](https://quasar.dev/quasar-cli-vite/developing-capacitor-apps/introduction)
    * [Android 设置](https://capacitorjs.com/docs/android/environment-setup)
    * [iOS 设置](https://capacitorjs.com/docs/ios/environment-setup)

这个过程可能需要一些时间和耐心，特别是第一次设置原生环境时。
