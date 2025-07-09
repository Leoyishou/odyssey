#!/usr/bin/env python3
"""
为所有 Markdown 文件添加唯一的 comment_id
这样即使文件重命名或移动，评论也会保留
"""

import os
import re
import uuid
import yaml
from pathlib import Path


def generate_comment_id():
    """生成一个短的唯一 ID"""
    return str(uuid.uuid4())[:8]


def has_comment_id(content):
    """检查文件是否已有 comment_id"""
    # 匹配 YAML frontmatter
    match = re.match(r"^---\s*\n(.*?)\n---\s*\n", content, re.DOTALL)
    if match:
        try:
            frontmatter = yaml.safe_load(match.group(1))
            return frontmatter and "comment_id" in frontmatter
        except:
            pass
    return False


def add_comment_id_to_file(filepath):
    """为单个文件添加 comment_id"""
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    if has_comment_id(content):
        print(f"跳过（已有 comment_id）: {filepath}")
        return False

    # 生成新的 comment_id
    comment_id = generate_comment_id()

    # 检查是否有 frontmatter
    match = re.match(r"^---\s*\n(.*?)\n---\s*\n(.*)", content, re.DOTALL)

    if match:
        # 有 frontmatter，添加 comment_id
        try:
            frontmatter = yaml.safe_load(match.group(1)) or {}
            frontmatter["comment_id"] = comment_id

            new_content = f"---\n{yaml.dump(frontmatter, allow_unicode=True)}---\n{match.group(2)}"
        except:
            # YAML 解析失败，手动添加
            new_content = f"---\n{match.group(1)}\ncomment_id: {comment_id}\n---\n{match.group(2)}"
    else:
        # 没有 frontmatter，创建新的
        new_content = f"---\ncomment_id: {comment_id}\n---\n\n{content}"

    with open(filepath, "w", encoding="utf-8") as f:
        f.write(new_content)

    print(f"已添加 comment_id: {filepath} -> {comment_id}")
    return True


def process_directory(root_dir):
    """处理目录下所有 Markdown 文件"""
    processed = 0
    skipped = 0

    # 需要处理的目录
    target_dirs = ["1 一切皆项目", "2 第二大脑", "3 博客", "4 复盘"]

    for target_dir in target_dirs:
        dir_path = Path(root_dir) / target_dir
        if not dir_path.exists():
            print(f"目录不存在: {dir_path}")
            continue

        for filepath in dir_path.rglob("*.md"):
            if add_comment_id_to_file(filepath):
                processed += 1
            else:
                skipped += 1

    print(f"\n处理完成！")
    print(f"- 已处理: {processed} 个文件")
    print(f"- 已跳过: {skipped} 个文件")


if __name__ == "__main__":
    # 使用当前目录作为根目录
    root_directory = "."

    print("开始为 Markdown 文件添加 comment_id...")
    print(f"根目录: {os.path.abspath(root_directory)}\n")

    process_directory(root_directory)
