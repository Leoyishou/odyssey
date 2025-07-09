---
draw:
title: PostgreSQL
date created: 2024-09-15
date modified: 2025-02-06
---

```Java

postgres=# \du
                                   List of roles
 Role name |                         Attributes                         | Member of 
-----------+------------------------------------------------------------+-----------
 postgres  | Superuser, Create role, Create DB, Replication, Bypass RLS | {}
 root      |                                                            | {}
```

```Java
postgresql://<用户名>:<密码>@<主机名>:<端口>/<数据库名>

postgresql://root   :root @localhost     /root

docker run -e POSTGRES_USER=<用户名> -e POSTGRES_PASSWORD=<密码> -e POSTGRES_DB=<数据库名> <镜像名>

docker run -e POSTGRES_USER=root -e POSTGRES_PASSWORD=root -e POSTGRES_DB=root postgres
```

`docker run -d -p 5432:5432 --name postgres -e POSTGRES_PASSWORD=postgres postgres:latest`  
![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F12%2F23%2F15-11-21-e7e4f32e293b763d1b3dc73acd22b215-202412231511540-b64210.png)

![image.png|1000](https://imagehosting4picgo.oss-cn-beijing.aliyuncs.com/imagehosting/fix-dir%2Fpicgo%2Fpicgo-clipboard-images%2F2024%2F12%2F11%2F22-41-58-36c45981869b5cefd5196851aa51fc47-202412112241647-f2b204.png)

## 数据结构

### JSSONB

使用 `JSONB` 类型的优势包括：

1. 灵活性：可以存储任意数量的例句，而不需要预先定义固定的列数。
2. 查询效率：PostgreSQL 提供了多种 JSON 操作符和函数，可以高效地查询和操作 JSON 数据。
3. 索引支持：可以在 JSONB 列上创建索引，提高查询性能。
	- [ ] 需要在数据库中为 `active_use_scenario` 列创建一个 GIN (Generalized Inverted Index) 索引。
4. 数据完整性：PostgreSQL 会验证插入的 JSON 数据的有效性。

```Java

```
