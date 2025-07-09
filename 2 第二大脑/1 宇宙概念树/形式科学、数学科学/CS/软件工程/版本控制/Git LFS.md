---
draw:
title: Git LFS
date created: 2025-03-26
date modified: 2025-03-26
---

Git LFS（Large File Storage）是一个专门用于处理大文件的 Git 扩展工具。通常情况下，Git 很擅长管理文本文件，比如代码，但对于图片、视频、音频等大文件管理效率并不好。

## 为什么需要 Git LFS？

如果你直接用 Git 管理大文件：

- **仓库变得巨大**：每次提交大文件都会存储完整的副本，Git 仓库变得越来越大。
- **克隆速度缓慢**：新用户克隆项目时，必须下载整个历史记录中所有的大文件，下载慢且占用空间。

## Git LFS 是怎么解决的？

Git LFS 不会直接把大文件存进仓库，而是将这些大文件存储在服务器上，仓库中只保留一个小小的**指针文件**（pointer），告诉你：“这个大文件存放在另一个地方，你去那里取吧！”

也就是说：

- 仓库变小了，只保存文件引用（几百字节大小）。
- 大文件被单独存储，只有需要时才下载。

## 使用 Git LFS 的流程

1. **安装并启用 Git LFS**
   ```bash
   git lfs install
   ```

2. **标记需要管理的大文件类型**  
   假设你想让 Git LFS 管理所有的图片：
   ```bash
   git lfs track "*.png"
   git lfs track "*.jpg"
   ```

3. **添加 `.gitattributes` 文件并提交**
   ```bash
   git add .gitattributes
   git commit -m "Track images using Git LFS"
   ```

4. **正常提交大文件**
   ```bash
   git add my-photo.png
   git commit -m "add large photo"
   git push origin main
   ```

   当你推送时，Git LFS 会自动把真实的文件上传到 Git LFS 服务器，而 Git 仓库中仅保留一个小小的指针文件。

## 什么时候该用 Git LFS？

- 当你的仓库有大于几十 MB 的二进制文件，比如图片、视频、音频。
- 当项目频繁修改大文件时，使用 Git LFS 能有效减少仓库膨胀。

---

## 一个形象的比喻

想象你家很小，你的书房（Git 仓库）放不下太多大件家具。
使用 Git LFS 相当于你把这些大件家具都放进了租来的仓库（Git LFS服务器），你家里只放了小纸条，上面写着：“沙发在仓库A，冰箱在仓库B”。
每次你想使用这些大件家具时，再去对应的仓库取回来即可，不用把家塞得满满当当的。

这样你的家就能保持干净整洁，而不用因为大件物品而变得拥挤不堪。

---

以上就是 Git LFS 的通俗介绍，希望对你有所帮助！
