---
uid: 58e29855-11c7-4172-8dbf-2407b03b1dae
date created: 2025-07-19
date modified: 2025-08-10
---
# Summary



# Cues



# Notes

## Dry-run 模式的作用 🎯

Dry-run（演练/模拟运行）模式超级有用！它让你可以**预览脚本会做什么，但不真正执行**。

### 具体场景举例

想象你要同步代码到服务器，但担心：

- "我会不会不小心覆盖服务器上的重要文件？"
- "这次同步会传输多少文件？"
- "我的 exclude 规则写对了吗？"

### 使用对比

**没有 dry-run：**

```bash
# 直接执行，文件已经传过去了
$ bash sync.sh
Syncing code...
transferring 500 files...  # 哦豁，怎么这么多？！
# 完蛋，已经覆盖了... 😱
```

**有 dry-run：**

```bash
# 先模拟看看
$ bash sync.sh --dry-run
DRY RUN MODE
Would sync:
  - src/model.py
  - configs/new_config.yaml
  - DELETE: old_experiments/  # 等等，这个不应该删除！
  
# 发现问题，修改脚本后再真正执行
$ bash sync.sh  # 这次放心了
```

### 完整示例实现

```bash
#!/bin/bash

# 解析参数
DRY_RUN=false
if [[ "$1" == "--dry-run" || "$1" == "-n" ]]; then
    DRY_RUN=true
    echo -e "${YELLOW}🔍 DRY RUN MODE - 仅显示将要执行的操作${NC}"
fi

# rsync 命令
if $DRY_RUN; then
    # 模拟模式：加 --dry-run 参数
    rsync -avz --dry-run \
        --exclude='*.pyc' \
        $LOCAL_DIR/ $REMOTE_HOST:$REMOTE_DIR/
    
    # SSH 命令只显示不执行
    echo "Would execute: ssh $REMOTE_HOST 'mkdir -p $REMOTE_DIR'"
    echo "Would execute: pip install -r requirements.txt"
else
    # 真正执行
    rsync -avz \
        --exclude='*.pyc' \
        $LOCAL_DIR/ $REMOTE_HOST:$REMOTE_DIR/
    
    ssh $REMOTE_HOST "mkdir -p $REMOTE_DIR"
fi
```

### 实际好处

1. **安全检查** - 在生产环境操作前先验证
2. **调试脚本** - 看看脚本逻辑是否正确
3. **估算时间** - 了解要传输多少数据
4. **验证规则** - 确认 exclude/include 规则是否正确
5. **团队协作** - 让同事先 review 将要执行的操作

### 使用场景

```bash
# 常见用法
./deploy.sh --dry-run  # 先看看会部署什么
./backup.sh -n         # 检查会备份哪些文件
./cleanup.sh --dry-run # 确认不会误删重要文件
```

**小贴士：**

- 很多命令行工具都支持 dry-run（如 `rsync`、`make`、`terraform`）
- 这是 DevOps 最佳实践之一
- 特别是在处理生产环境时，dry-run 可以避免很多事故！

所以这个功能虽然只有几行代码，但能避免很多"手滑"导致的悲剧 😅
