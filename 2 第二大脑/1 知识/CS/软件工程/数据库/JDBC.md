---
draw:
tags: []
title: JDBC
date created: 2025-01-18
date modified: 2025-01-18
---

| MySQL | JDBCType      | JavaType                   |
| ----- | ------------- | -------------------------- |
|       | CHAR          | String                     |
|       | VARCHAR       | String                     |
| Text  | LONGVARCHAR   | String                     |
|       | NUMERIC       | java.math.BigDecimal       |
|       | DECIMAL       | java.math.BigDecimal       |
|       | BIT           | boolean                    |
|       | BOOLEAN       | boolean                    |
|       | TINYINT       | byte                       |
|       | SMALLINT      | short                      |
|       | INTEGER       | int                        |
|       | BIGINT        | long                       |
|       | REAL          | float                      |
|       | FLOAT         | double                     |
|       | DOUBLE        | double                     |
|       | BINARY        | byte[]|
|       | VARBINARY     | byte[]|
|       | LONGVARBINARY | byte[]|
|       | DATE          | java.sql.Date              |
|       | TIME          | java.sql.Time              |
|       | TIMESTAMP     | java.sql.Timestamp         |
|       | CLOB          | Clob                       |
|       | BLOB          | Blob                       |
|       | ARRAY         | Array                      |
|       | DISTINCT      | mapping of underlying type |
|       | STRUCT        | Struct                     |
|       | REF           | Ref                        |
|       | DATALINK      | java.net.URL               |
