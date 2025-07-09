#!/bin/bash

# 为新的 Markdown 文件自动添加 comment_id
# 可以手动运行，也可以作为 git hook 使用

# 获取所有未跟踪和已修改的 .md 文件
MD_FILES=$(find . -name "*.md" -type f | grep -E '^\./((1 一切皆项目)|(2 第二大脑)|(3 博客)|(4 复盘))/')

echo "🔍 检查 Markdown 文件..."

PROCESSED=0
for file in $MD_FILES; do
    # 跳过已有 comment_id 的文件
    if grep -q "^comment_id:" "$file" 2>/dev/null; then
        continue
    fi
    
    echo "📝 处理: $file"
    
    # 生成唯一 ID
    COMMENT_ID=$(uuidgen | tr '[:upper:]' '[:lower:]' | cut -c1-8)
    
    # 读取文件内容
    CONTENT=$(<"$file")
    
    # 检查是否已有 frontmatter
    if [[ "$CONTENT" =~ ^--- ]]; then
        # 在现有 frontmatter 后添加 comment_id
        # 使用 awk 在第一个 --- 后插入
        awk -v id="$COMMENT_ID" '
            /^---/ && !found {found=1; print; next}
            /^---/ && found && !done {print "comment_id: " id; done=1}
            {print}
        ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
    else
        # 创建新的 frontmatter
        {
            echo "---"
            echo "comment_id: $COMMENT_ID"
            echo "---"
            echo ""
            echo "$CONTENT"
        } > "$file"
    fi
    
    PROCESSED=$((PROCESSED + 1))
    echo "  ✅ 已添加 comment_id: $COMMENT_ID"
done

if [ $PROCESSED -eq 0 ]; then
    echo "✅ 所有文件都已有 comment_id"
else
    echo "✨ 已为 $PROCESSED 个文件添加了 comment_id"
fi