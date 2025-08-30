---
date created: 2024-04-21
date modified: 2025-07-10
uid: 923bcf1c-0f97-4d9a-a306-c15b9022679c
---

![c9a1ec04e68122ea0faf7daf4175f9c5.png|400](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fliuyishou%2Ftmp%2F2024%2F04%2F22%2F00-07-37-1835db2ac20f78f8a3f859bcd1fd4b57-c9a1ec04e68122ea0faf7daf4175f9c5-240236.png?x-oss-process=image/resize,l_400)

> odyssey - an intellectual or spiritual wandering or quest

<!-- more -->

## 表设计

- [ ] word 字段需要加唯一索引 ⏰ 2024-04-24📅 2024-04-25

![](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F04%2F21%2F22-18-27-4b89d65204db38c122dc8bb9efb6a989-20240421221826-a8c757.png)

## Anki 中插入 oss 的音频

<audio src="audiofile.mp3" autoplay> Your browser does not support the audio element. </audio>

```html
<meta charset="UTF-8">
<title>Custom Audio Player</title>
<style>
    audio {
        width: 100%;
        margin-top: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        overflow: hidden; /* Helps contain the inner elements */
    }

    /* Chrome, Safari, Edge, Opera */
    audio::-webkit-media-controls-panel {
        background-color: white;
        color: black;
    }

    /* Firefox */
    audio::-moz-focus-inner {
        border: 0;
    }

    audio::-moz-range-track {
        background-color: white;
    }

    /* Might not fully change the background but attempts to lighten it */
    audio::--webkit-media-controls-enclosure {
        background-color: white !important;
    }
</style>


<audio controls="" autoplay="">
    <source src="https://odyssey-liugongzi.oss-cn-beijing.aliyuncs.com/Her%20(brilliant)%20idea%20solved%20the%20problem..mp3" type="audio/mpeg">
    Your browser does not support the audio element.
</audio>

```
