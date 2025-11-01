---
uid: c02991b2-7d6b-406b-b2e0-147a992466ad
date created: 2025-02-14
date modified: 2025-10-28
---
**KVM（Kernel‑Based Virtual Machine）** 是 Linux 内核自带的虚拟化技术。它把 Linux 本身变成了一个 **裸金属（type‑1）hypervisor**，让一台物理服务器可以同时运行多个相互隔离的 **虚拟机（VM）**，每个虚拟机里可以装不同的操作系统（Linux、Windows、BSD …）。


https://portal.corp.qunar.com/portal/apptree#!/my-appcode/host?app_code=pf_taurus&appcode_filter=taur&binded=all&app_level=all APPcode 下有 kvm 机器
1. 是主机-KVM
2. 容器 是 [[Docker]]
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2025%2F10%2F28%2F15-57-00-6822a7b798a7117806773c24813f96f8-202510281557467-5230c1.png)
