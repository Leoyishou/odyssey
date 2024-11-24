import os
import re

def wrap_generic_types(content):
    # 保存所有代码块和它们的占位符
    code_blocks = {}
    placeholder_counter = 0
    
    # 临时替换代码块
    def replace_code_block(match):
        nonlocal placeholder_counter
        placeholder = f"CODEBLOCK_{placeholder_counter}"
        code_blocks[placeholder] = match.group(0)
        placeholder_counter += 1
        return placeholder
    
    # 先替换多行代码块
    content = re.sub(r'```.*?```', replace_code_block, content, flags=re.DOTALL)
    # 再替换行内代码
    content = re.sub(r'`[^`]+`', replace_code_block, content)
    
    # 处理泛型标记
    content = re.sub(
        r'(?<!\w)(\w+<[\w\s,]+>)(?!\w)', 
        r'`\1`', 
        content
    )
    
    # 还原所有代码块
    for placeholder, code_block in code_blocks.items():
        content = content.replace(placeholder, code_block)
    
    return content

def process_markdown_files(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.md'):
                file_path = os.path.join(root, file)
                print(f"Processing {file_path}")
                
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()
                    
                    # 处理内容
                    modified_content = wrap_generic_types(content)
                    
                    # 如果内容有变化才写入
                    if modified_content != content:
                        print(f"Found and wrapped generic types in {file_path}")
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.write(modified_content)
                
                except Exception as e:
                    print(f"Error processing {file_path}: {str(e)}")

# 使用脚本
directory = "/Users/liuyishou/usr/projects/odyssey/src"
process_markdown_files(directory)