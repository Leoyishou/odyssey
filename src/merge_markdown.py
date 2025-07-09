import os
import glob

# Directories to search
directories = [
    '1 一切皆项目',
    '2 第二大脑',
    '3 博客',
    '4 复盘'
]

# Output file
output_file = 'combined_markdown.txt'

# Function to process all markdown files in a directory
def process_directory(directory):
    content = []
    # Find all markdown files recursively
    md_files = glob.glob(f'{directory}/**/*.md', recursive=True)
    
    for file_path in md_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as file:
                file_content = file.read()
                # Add file path as header and content with separator
                content.append(f"\n\n{'=' * 80}\n{file_path}\n{'=' * 80}\n\n")
                content.append(file_content)
                content.append("\n\n")
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
    
    return content

# Main process
all_content = []
for directory in directories:
    print(f"Processing directory: {directory}")
    dir_content = process_directory(directory)
    all_content.extend(dir_content)
    print(f"Found {len(dir_content) // 3} markdown files in {directory}")

# Write all content to the output file
with open(output_file, 'w', encoding='utf-8') as out_file:
    out_file.writelines(all_content)

print(f"\nAll markdown files have been combined into {output_file}")
print(f"Total number of markdown files processed: {len(all_content) // 3}") 