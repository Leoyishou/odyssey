---
date created: 2024-11-25
date modified: 2025-07-10
uid: a72f5403-cfdd-494e-a884-d97c87d04749
---
## 本质

一条`git push`触发一连串命令，比如

- 构建你的 VuePress 项目
- 运行测试
- 部署到 GitHub Pages 或其他服务器

##

一条`git push`触发一连串命令，比如

- 构建你的 VuePress 项目
- 运行测试
- 部署到 GitHub Pages 或其他服务器

是的，这里的"本地"指的是 GitHub Actions 的运行环境（也就是 GitHub 提供的临时虚拟机）。让我解释一下整个流程：

1. 当你推送代码到 GitHub 时，GitHub Actions 会：
   - 创建一个全新的虚拟机（Ubuntu）
   - 克隆你的代码到这个虚拟机
   - 按照 workflow 文件中的步骤执行操作

2. 在你的 workflow 中：

```yaml:.github/workflows/deploy.yml
steps:
  - name: Checkout                    # 1. 克隆代码到虚拟机
    uses: actions/checkout@v3
    
  - name: Install dependencies        # 2. 安装依赖
    run: npm install
    
  - name: Build VuePress site        # 3. 构建网站，生成 dist 目录
    run: npm run build
    
  - name: Deploy to Server           # 4. 将构建好的文件从虚拟机传到你的服务器
    uses: appleboy/scp-action@master
    with:
      source: "docs/.vuepress/dist/*"  # 这里的路径是相对于虚拟机上项目根目录的路径
      target: "/home/lighthouse/odyssey"
```

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F11%2F25%2F02-20-23-6363abbdf25db55720c61601816fd813-202411250220402-26a47b.png)

`source` 路径就是指 GitHub Actions 虚拟机上的文件路径，这些文件是通过前面的构建步骤生成的。
