---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
#  💡Summary  

> [!Info]
> Schema 就是数据库里的"文件夹"          
> 



# 🧩 Cues



# 🪞Notes

```
  PostgreSQL 数据库                                                                                                                                               
  ├── public/          ← 默认 schema                                                                                                                              
  │   ├── users                                                                                                                                                   
  │   └── posts                                                   
  ├── pac/             ← 你自定义的 schema
  │   ├── pmo_events
  │   ├── content_posts
  │   └── investment_trades
  └── auth/            ← Supabase 内置 schema
      ├── users
      └── sessions

```                                                                                                    


  类比：
  - 数据库 = 硬盘
  - Schema = 文件夹
  - 表 = 文件

  Schema 隔离的好处：

  1. 命名不冲突：pac.users 和 auth.users 可以同时存在
  2. 权限控制：可以给不同 schema 设置不同的访问权限
  3. 逻辑清晰：按业务领域分组，而不是所有表堆在 public 里

  你的当前结构：
  -- 访问 pac schema 的表
  SELECT * FROM pac.pmo_events;

  -- 访问 public schema 的表（默认可以省略 public）
  SELECT * FROM users;
  SELECT * FROM public.users;  -- 等价

  所以「一个 Supabase + 多 Schema」就是用一个数据库实例，通过不同文件夹来隔离不同业务的数据。