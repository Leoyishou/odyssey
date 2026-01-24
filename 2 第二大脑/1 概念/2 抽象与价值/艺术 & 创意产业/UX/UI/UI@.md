---
date created: 2025-04-14
date modified: 2025-08-30
uid: 02ac91bb-955a-4c9e-bc83-0e25b765894f
---
[[HIG]]
[[切图]]

```
Product_Design_System(产品设计体系)/
└── Design_Source(设计源文件)/
    ├── 1_Global_Foundations(全局视觉地基)/
    │   ├── Color_System(色彩系统)/
    │   │   ├── BrandColors(品牌色)✅
    │   │   ├── Semantic(语义色/Success, Error, Warning)✅
    │   │   └── DarkMode_Mapping(暗黑模式映射表)✅
    │   ├── Typography(排版系统)/
    │   │   ├── FontFamilies(字体族/PingFang, SF Pro, Segoe UI)
    │   │   └── TypeScale(字号阶梯/H1-H6, Body, Caption)
    │   ├── Grid_Layout(栅格与布局)/
    │   │   ├── Breakpoints(响应式断点)
    │   │   └── Spacing(间距/4px or 8px grid)✅
    │   └── Icons_Assets(图标与资产)/
    │       ├── SVG_Icons(矢量图标库)
    │       └── EmptyStates(空状态插画)
    │
    ├── 2_Component_Library(组件库/UI Kit)/
    │   ├── Atoms(原子组件)/
    │   │   ├── Buttons(按钮/Primary, Secondary, Ghost)
    │   │   ├── Inputs(输入框/Text, Checkbox, Toggle)
    │   │   └── Avatars(头像)
    │   ├── Molecules(分子组件)/
    │   │   ├── FormGroups(表单组)
    │   │   └── SearchBars(搜索栏)
    │   └── Organisms(组织组件)/
    │       ├── NavigationBar(导航栏)
    │       └── Cards(信息卡片)
    │
    ├── 3_Platform_Adaptation(多端适配规范)/✅
    │   ├── Desktop_OS/
    │   │   ├── macOS(遵循 Apple HIG)/
    │   │   │   ├── Vibrancy(毛玻璃/侧边栏通透感)
    │   │   │   └── TrafficLights(红绿灯窗口控制)
    │   │   └── Windows(遵循 Fluent Design)/
    │   │       ├── Mica/Acrylic(云母/亚克力材质)
    │   │       └── Segoe_UI(Win专用字体渲染)
    │   ├── Mobile_OS/
    │   │   ├── iOS(遵循 HIG)/
    │   │   │   ├── SafeArea(刘海屏/灵动岛避让)✅
    │   │   │   └── TouchTargets(最小点击区域 44pt)
    │   │   └── Android(遵循 Material 3)/
    │   │       ├── DynamicColor(壁纸取色)
    │   │       └── NavigationRail(侧边导航)
    │   └── Web_Extension/
    │       ├── Responsive(响应式/Flexbox布局)
    │       ├── HoverStates(鼠标悬停交互)
    │       └── Popup_Constraints(插件弹窗尺寸限制)
    │
    ├── 4_UX_Engineering(交互体验工程)/
    │   ├── UserFlows(用户流程图)
    │   ├── Interaction(微交互)/
    │   │   ├── Transitions(转场动画/Ease-in-out)
    │   │   └── Feedback(点击反馈/Haptic)
    │   └── Accessibility(无障碍/A11y)/
    │       ├── ContrastRatio(对比度检查)
    │       └── ScreenReader(读屏适配标签)
    │
    └── 5_Handoff_Delivery(交付与还原)/
        ├── DesignTokens(设计变量/JSON)/✅
        │   ├── global.tokens (颜色/间距)
        │   └── platform.tokens (特定平台差异)
        ├── Specs(标注说明)
        └── Slice_Export(切图导出)/
            ├── @2x_@3x (移动端倍图)
            └── SVG/WebP (Web端格式)
```
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2026%2F01%2F20%2F12-51-45-49f2306a6efe3426deaeb2b6a3318a10-202601201251432-e9dcaf.png)







实话实说，manus很大程度上的UI和UX，是非常细颗粒设计的，整个字体、颜色、动画切换，精细化到可以和medium、perplexity媲美的。

对于华尔街这些分析师和咨询而言，有一个不刻意精美的自然舒适的UI/UX，有闪光，有互动，有计时，能够让他们自己舒服操作，并且完全理直气壮地把成品分享给同事、manager、客户、会议观众，甚至不用任何包装就可以卖出去，是一个绝佳的必杀技。

  

绝大多数AI Agent产品，尤其是那些非SWE agent、面向普通人的产品，对这方面的感知是完全缺失的，甚至是瞧不起的——很多人拿一套tailwind就直接上手，甚至连serif和san serif都分不清，实际上恰恰缺失了这些最关键的设计因素。






- 设计系统与一致性原则：  
    现代化的产品往往依赖于系统化的设计语言和设计系统（如 Google 的 Material Design、Apple 的 Human Interface Guidelines）。这不仅包括视觉元素（颜色、字体、图标等）的一致性，还涉及组件、动画和交互模式的标准化。
    
- 响应式与无障碍设计：  
    在实现现代 UI 时必须同时考虑多设备（手机、平板、PC）的响应式设计，以及确保满足无障碍设计标准，使界面对各类用户（包括视力障碍用户）友好。
    
- 迭代与用户测试：  
    美化过程需要快速制作原型、进行 A/B 测试和用户反馈循环，确保设计改进在真实使用环境中具有积极作用。过于理想化的设计可能忽略真实场景的复杂性与多样性。
    
- 前沿设计趋势与微交互：  
    像 ChatGPT 和 Grok 这样的产品除了整体视觉风格外，其微交互、动效、反馈、加载状态等细节也极具吸引力和用户体验价值；这些细节往往需要在设计时提前规划好。


# UI 与动效
- UI 设计 (User Interface Design) → 广义上是界面设计，包括排版、色彩、图标、交互逻辑、组件规范等等。
- 动效 (Motion Design / Micro-interaction animation) → 是 UI 设计中的一个子概念/子领域。它关注的是元素在时间维度上的变化，用动画来传递信息、引导用户、增强体验。
## 🎬 实际例子：移动端 App 登录动效

1. 场景  
    用户在手机 App 上输入账号和密码，然后点击“登录”按钮。
2. UI 静态设计，如果只停留在静态 UI，用户点击后页面突然一闪就跳转，会感觉生硬。
	- 一个输入框
	- 一个按钮
	- 背景和 logo
3. 加入动效设计后的流程
	- 点击按钮 → 按钮微微缩小 10%，再回弹 → 给用户一个“被按下”的物理反馈。
	- 验证中 → 按钮变成一个 loading 圆圈，缓缓旋转 → 提示系统正在处理请求。
	- 成功登录 → 圆圈勾选动画（✅），然后页面内容淡出，主页从右侧轻轻滑入。
4. 效果
	- 用户感受到操作被接收（反馈）。
	- 通过旋转加载动画，用户知道系统正在工作（状态传递）。
	- 页面切换时的平滑过渡让体验更自然（引导注意力，降低认知负担）。
