---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

## Wireshark 的不可替代性

在以下场景，Surge 是做不到的：

- **排查底层网络问题：** 比如丢包、TCP 重传、三次握手失败、DNS 解析耗时等。
    
- **分析非标准协议：** 比如视频监控的 RTSP、工业协议、或者某些私有加密协议。
    
- **物理层分析：** 查看 Wi-Fi 信号强度、以太网帧头等。

# 🧩 Cues



# 🪞Notes

|**功能特性**|**Surge (HTTP Map / Dashboard)**|**Wireshark**|
|---|---|---|
|**主要层级**|应用层 (Layer 7)，侧重 HTTP/HTTPS/WebSocket|全层级 (Layer 2-7)，包括 TCP/UDP, ICMP, DNS 等|
|**HTTPS 解密**|**极其简单**。内置 MITM 证书配置，一键解密查看明文|**非常复杂**。需要配置 SSLKEYLOGFILE 等环境变量导入密钥|
|**易读性**|**高**。自动解析 JSON、图片、表单，界面像开发者工具|**低**。全是原始数据包，需要极强的网络协议知识|
|**设备支持**|仅限 macOS / iOS|跨平台（Windows, macOS, Linux）|
|**抓取范围**|主要抓取本机或接管网关后的流量|只要网卡能“看”到的包都能抓（混杂模式）|