# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Obsidian-based personal knowledge management system (PKM) called "Odyssey" that follows the PARA (Projects, Areas, Resources, Archives) methodology combined with the DIKW (Data-Information-Knowledge-Wisdom) model. The content is primarily in Chinese.

## Repository Structure

The repository contains four main directories:

- **1 一切皆项目/** - Projects directory (active, archived, and on-hold projects)
- **2 第二大脑/** - Second Brain knowledge base organized into:
  - **1 宇宙概念树/** - Concept tree covering various domains (humanities, sciences, technology)
  - **2 飞轮/** - Flywheel system with 6 subsystems (工具系统, 信息系统, 认知系统, 项目系统, 能力系统, 影响力系统)
- **3 博客/** - Blog posts and articles
- **4 复盘/** - Weekly/annual reviews and retrospectives

## Key Commands

### Markdown Processing
```bash
python merge_markdown.py
```
Combines all markdown files from the four main directories into `combined_markdown.txt`.

### Git Operations
The repository uses git for version control. Common operations:
```bash
git status
git add .
git commit -m "commit message"
git push
```

## Development Guidelines

### File Organization
- All content files are in Markdown format (.md)
- Use Chinese file names that clearly describe the content
- Files with `$` suffix (e.g., `项目$.md`) are typically index or overview files
- Files with `@` suffix (e.g., `管理学@.md`) are category or topic aggregation files

### Linking Convention
- Use Obsidian-style wikilinks: `[[filename]]` for internal references
- External links use standard markdown: `[text](url)`
- Image links follow the pattern: `![alt text|size](image_url)`

### Content Structure
- Each file typically starts with YAML frontmatter containing metadata
- Use hierarchical headings (##, ###) for content organization
- Tag system includes: `#第一性原理`, `#最佳实践`, `#comment`

### Working with Obsidian Features
- The repository is designed to work with Obsidian's graph view and backlinks
- Maintain the existing folder structure to preserve the knowledge hierarchy
- When creating new content, place it in the appropriate category folder

## Important Notes

- This is a personal knowledge base with content spanning multiple domains
- The primary language is Chinese, with some English technical terms
- The system follows specific organizational philosophies (PARA, DIKW, 飞轮系统)
- Content includes personal reflections, study notes, project documentation, and blog posts