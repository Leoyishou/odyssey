---
uid: 7f84ff59-7433-4ecb-9f2b-b7fd9f8b7398
---


> [!info]
> 
> 你可以把它理解为视频编辑领域的 `Vim` 键位——一旦学会，你的左手在键盘上“起舞”，右手甚至不需要碰鼠标。
> 


视频+音频+图片+文本

```
CapCut_剪映/
└── Project(工程)/
    ├── Workspace(工作区)/
    │   ├── MediaLibrary(媒体库/素材区)/
    │   │   ├── Import(导入)/
    │   │   │   ├── LocalFiles(本地文件)
    │   │   │   ├── Album(相册)
    │   │   │   └── Record(录制/录音)
    │   │   ├── Assets(素材资产)/
    │   │   │   ├── Video(视频))✅
    │   │   │   ├── Audio(音频))✅
    │   │   │   ├── Image(图片))✅
    │   │   │   └── TextMaterials(字体/文本素材))✅
    │   │   ├── Templates(模板/草稿同款)/
    │   │   └── Favorites(收藏)
    │   │
    │   ├── Timeline(时间线)/
    │   │   ├── Sequence(序列/主时间轴)/
    │   │   │   ├── Track(轨道)* /
    │   │   │   │   ├── VideoTrack(视频轨)✅
    │   │   │   │   ├── AudioTrack(音频轨)✅
    │   │   │   │   ├── TextTrack(文本轨)✅
    │   │   │   │   ├── StickerTrack(贴纸/元素轨)
    │   │   │   │   └── EffectTrack(特效/滤镜轨)
    │   │   │   │
    │   │   │   │   └── Clip(片段)* /
    │   │   │   │       ├── SourceRef(引用的源素材)
    │   │   │   │       ├── Trim(裁剪)/
    │   │   │   │       │   ├── InOut(入点/出点)
    │   │   │   │       │   └── Split(分割)✅
    │   │   │   │       ├── Transform(变换)/
    │   │   │   │       │   ├── Position(位置)
    │   │   │   │       │   ├── Scale(缩放)
    │   │   │   │       │   ├── Rotate(旋转)
    │   │   │   │       │   └── Crop(裁切)
    │   │   │   │       ├── Speed(变速)/
    │   │   │   │       │   ├── Normal(常规变速)
    │   │   │   │       │   └── Curve(曲线变速)
    │   │   │   │       ├── Opacity(不透明度)
    │   │   │   │       ├── Blend(混合模式)
    │   │   │   │       ├── Mask(蒙版)/
    │   │   │   │       │   ├── Shape(形状蒙版)
    │   │   │   │       │   └── Invert/Feather(反向/羽化)
    │   │   │   │       ├── Keyframes(关键帧)* /
    │   │   │   │       │   └── PropertyTimeline(属性随时间的曲线)
    │   │   │   │       └── EffectsOnClip(作用于片段的效果)* /
    │   │   │   │           ├── Filter(滤镜)
    │   │   │   │           ├── Adjust(调节/调色)/
    │   │   │   │           │   ├── Exposure/Contrast/Saturation...
    │   │   │   │           │   └── HSL/Curves/LUT(若支持)
    │   │   │   │           ├── VideoEffect(视频特效)
    │   │   │   │           └── Motion(动效/入出场)
    │   │   │   │
    │   │   │   ├── Transition(转场)* /
    │   │   │   │   ├── BetweenClips(片段之间)
    │   │   │   │   └── Duration(时长)
    │   │   │   └── GlobalOverlays(全局层)/
    │   │   │       ├── Canvas(画布)/
    │   │   │       │   ├── Ratio(比例)
    │   │   │       │   ├── Background(背景)
    │   │   │       │   └── SafeArea(安全区)
    │   │   │       └── GlobalEffects(全局效果)/
    │   │   │           ├── GlobalFilter(全局滤镜)
    │   │   │           └── GlobalAdjust(全局调节)
    │   │   │
    │   │   └── AudioPipeline(音频处理链)/
    │   │       ├── Volume(音量/增益)
    │   │       ├── FadeInOut(淡入淡出))✅
    │   │       ├── NoiseReduction(降噪)✅
    │   │       ├── VoiceEffects(变声/音效))✅
    │   │       └── BeatMarkers(节拍点/卡点辅助)
    │   │
    │   └── Inspector(属性面板/检查器)/
    │       ├── SelectionContext(选中对象上下文)✅
    │       └── PropertyEditors(属性编辑器集合)
    │
    ├── Operations(操作/命令)/
    │   ├── Add(添加到时间线)
    │   ├── Split(分割)✅
    │   ├── Trim(裁剪)✅
    │   ├── Ripple(波纹编辑/联动推拉)
    │   ├── Group(编组/多选)
    │   ├── CopyPaste(复制粘贴/复制属性)✅
    │   └── UndoRedo(撤销/重做)✅
    │
    ├── Output(输出)/
    │   ├── Export(导出)/✅
    │   │   ├── Resolution(分辨率)✅
    │   │   ├── FPS(帧率)✅
    │   │   ├── Bitrate(码率)✅
    │   │   └── Format(格式/封装)✅
    │   └── Publish(发布/分享)/
    │
    └── Metadata(元信息)/
        ├── ProjectSettings(工程设置)
        ├── Cache(缓存/代理/预览渲染)
        └── Versions(草稿版本/历史)✅

```

## Cues

## 导出

- **分辨率** 决定画面的“大小”和细节程度。  
- **帧率** 决定画面运动的“流畅度”。  
- **码率** 决定画面细节的“保真度”，同时影响文件大小和播放时的带宽需求。  

| 概念                  | 简单比喻             | 常见取值举例                                                   |
| ------------------- | ---------------- | -------------------------------------------------------- |
| **Resolution（分辨率）** | 栅格数量             | 3840×2160（4K）<br>1920×1080（Full HD）<br>1280×720（HD）      |
| **FPS（帧率）**         | 像翻书一样，每翻一页就是一帧。  | 24 fps（电影常用）<br>30 fps（常规视频）<br>60 fps（游戏、运动）            |
| **Bitrate（码率）**     | 栅格格子里的信息量 (数据维度) | 2 Mbps（低清）<br>5 Mbps（1080p 30 fps）<br>15 Mbps（4K 30 fps） |




## Notes
## 一、视频轨
### 1. 读数据：J-K-L 大法

可变速的遍历：J-K-L 大法

**`J-K-L`**：就像 **Debug 模式下的步进控制**。
- `L` 是 `Continue`。
- `K` 是 `BreakPoint`。    
- `K+L` 是 `Step Over` (单步执行)。


，学会 `Ripple Delete`（波纹删除）。这就像写代码时的重构，把没用的冗余代码（废片）删掉。
    - 多轨道管理： 理解视频轨（V1, V2...）和音频轨（A1, A2...）的层级关系。
    - 关键帧 (Keyframes)： 理解属性随时间变化的逻辑（类似于 CSS 动画），这是做缩放、移动效果的基础。

### 2. 删除数据：波纹删除

波纹删除，注意不要导致声音轨道和视频轨道的不同步

想象你往平静的水面上扔了一块石头，波纹会向外扩散。 在剪辑里，这个“波纹”是指：当你删掉 1 秒钟的素材，这 1 秒钟的变动会像波纹一样**传递**到整条时间轴的末尾——**所有后续的素材都会向前移动 1 秒**。

1. 当你在**主视频轨道**上选中一段，按下 `Backspace` (Mac 上的 `Delete`) 时，**它默认执行的就是波纹删除**。
2. **在剪映里**：确保开启 **“主轨吸附”** 或 **“联动”**（通常默认开启），这样删视频时，对应的音频也会跟着动。

### 3. 修改数据：J-Cut 和 L-Cut

J-Cut：未闻其人，先闻其声

```
时间轴流动方向 ---------------------------------------------->

[视频轨道 V1]  |===== Clip A (你的脸) =====|========== Clip B (代码画面) ==========|
                                           ^
                                           | 视觉切点 (Video Cut)
                                           |
[音频轨道 A1]  |===== Clip A 声音: "来，我带你看一下...它是怎么工作的" ===|=== Clip B 声音 ===|
                                                                  ^
                                                                  | 听觉切点 (Audio Cut)
```


在现实世界，声音和画面是同步的，J-Cut 和 L-Cut是通过音画的一种错位来给观众造成一种视觉效果
如果你把视频和音频在同一个时间点切断（| |），这对观众的大脑来说是一个**双重信号中断**。大脑会下意识地意识到：“哦，换场景了/被打断了”。这就像网页跳转时的“白屏”。


## 快捷键

按 **`↓` (下箭头)**：光标会自动**瞬移**到下一个视频片段的切点（这是关键，剪映默认支持）。



## 概念



**“文稿剪辑” (Text-Based Editing)** 功能中的 **“智能去水词” (Smart Filler Word Removal)** 能力。

最佳实践：删除「语气词」，保留「停顿」和「重复」
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2026%2F01%2F17%2F18-50-47-13127f68e45cb916201d8fbf3c945f5f-202601171850798-a4439f.png)

### “切点” (The Cut Point)

想象你的视频时间轴是一个数组 `[Video_Clip_A, Video_Clip_B, Video_Clip_C]`。

- **切点**就是 `Video_Clip_A` 结束、`Video_Clip_B` 开始的那个**毫秒级的时间戳**（Boundary）。
    
- 在剪映的界面上，它就是**两段视频中间的那条竖线**。





![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2026%2F01%2F17%2F18-47-58-13127f68e45cb916201d8fbf3c945f5f-202601171847221-7ff647.png)
