---
draw:
title: PG 数据导出迁移
tags: [信息革命, 数据, 数据库最佳实践]
date created: 2024-05-01
date modified: 2024-11-12
---

excerpt

<!-- more -->

PostgreSQL（通常称为 PG 库）的数据实际上存储在文件系统的一组文件中，这些文件位于服务器上的一个特定目录中。这个目录被称为数据目录，通常在 PostgreSQL 配置文件（`postgresql.conf`）中设置为 `data_directory`。

## 数据存储位置

1. **数据文件**：表数据、索引等都存储在数据目录中的文件里。
2. **事务日志（WAL）文件**：这些是用于数据恢复和确保数据完整性的日志文件。
3. **配置文件**：如 `postgresql.conf` 和 `pg_hba.conf` 等配置文件也位于此目录。

## 数据导出与迁移

迁移 PostgreSQL 数据库通常涉及数据的备份与恢复。下面是一些常用的方法：

### 1. SQL 转储（pg_dump 和 pg_dumpall）

- **pg_dump**：用于导出一个或多个数据库的数据和结构到一个文件中。这个文件可以是 SQL 语句的集合，也可以是二进制格式。
    

    bash

    

    Copy code

    

    `pg_dump -U username -W -F c -b -v -f "/path/to/your/file.backup" dbname`

    

    这条命令会提示你输入密码，然后创建一个格式化的备份文件。

    
- **pg_dumpall**：导出整个数据库服务器的所有数据库和角色数据。
    

    bash

    

    Copy code

    

    `pg_dumpall -U username -W -f "/path/to/your/file.sql"`

    

    这会生成一个 SQL 脚本文件，包括创建数据库和用户的命令。

    

### 2. 物理备份（文件系统级备份）

- **直接复制数据目录**：在数据库停机的情况下，可以直接复制整个数据目录到另一位置或另一服务器。
- **使用工具如 rsync**：对于较大的
