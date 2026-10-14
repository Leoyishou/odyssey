---
date created: 2025-09-01
date modified: 2025-09-01
uid: 31cb1cd6-f9c5-42e5-ae9c-31b19d5a719a
---

| 问题                    | 关键词/关键概念                                                                                                       |
| --------------------- | -------------------------------------------------------------------------------------------------------------- |
| Mybatis可以实现动态SQL？| **可以**：if、choose/when/otherwise、trim/where/set、foreach标签、OGNL表达式、SQL片段复用                                       |
| `#` 和`$`的区别？什么情况必须用$？| **#{}**：预编译、参数占位符、防SQL注入、自动类型转换<br>**${}**：字符串替换、直接拼接、SQL注入风险<br>**必须用$场景**：动态表名、列名、ORDER BY字段、动态SQL片段         |
| MyBatis-Plus有什么用？| **增强功能**：CRUD封装、条件构造器、分页插件、代码生成器、乐观锁、逻辑删除、自动填充、多租户、性能分析                                                        |
| MyBatis-Plus分页原理？| **拦截器机制**：PaginationInnerInterceptor、拦截SQL、自动添加limit、count查询、方言适配、物理分页                                         |
| MyBatis与Hibernate区别？| **MyBatis**：半ORM、SQL可控、轻量级、学习简单、灵活性高<br>**Hibernate**：全ORM、HQL、重量级、自动化程度高、数据库无关性好                              |
| Mybatis是否支持延迟加载？原理？| **支持**：lazyLoadingEnabled配置<br>**原理**：动态代理、CGLIB/Javassist、首次调用时触发查询、按需加载关联对象                                  |
| Mybatis插件运行原理？| **拦截器链**：Interceptor接口、@Intercepts注解、责任链模式、动态代理、可拦截Executor/StatementHandler/ParameterHandler/ResultSetHandler |
| Mybatis如何实现字段映射？| **自动映射**：驼峰命名转换、resultMap配置、列别名、TypeHandler类型处理器、@Results注解                                                    |
| Mybatis用什么连接池？| **三种类型**：UNPOOLED无连接池、POOLED内置连接池、JNDI容器连接池<br>**整合第三方**：Druid、HikariCP、C3P0                                   |
| Mybatis的优点？| **SQL灵活**、**轻量级**、**易学习**、**与Spring集成好**、**支持动态SQL**、**提供映射标签**、**可调优SQL**                                     |
| Mybatis工作原理？| **流程**：加载配置→创建SqlSessionFactory→获取SqlSession→获取Mapper代理→执行SQL→映射结果→返回结果                                        |
| Mybatis缓存机制？| **一级缓存**：SqlSession级别、默认开启、本地缓存、事务内有效<br>**二级缓存**：Mapper级别、需手动开启、跨SqlSession共享、可配置策略                           |
| PageHelper分页原理？| **ThreadLocal**保存分页参数、**拦截器**修改SQL、自动添加limit、count统计、方言识别、物理分页实现                                               |
| RowBounds分页原理？| **内存分页**：查询全部数据、内存中截取、逻辑分页、适合小数据量、性能差不推荐                                                                       |
| 什么是ORM？常用框架？| **ORM**：对象关系映射、对象与数据库表映射、简化数据库操作<br>**框架**：MyBatis、Hibernate、JPA、Spring Data JPA、JOOQ                          |
| 使用MyBatis如何实现分页？| **方式**：1.RowBounds逻辑分页 2.物理分页手写limit 3.PageHelper插件 4.MyBatis-Plus分页 5.拦截器自定义                                  |

# MyBatis核心概念详解

## **核心组件**

```Java
SqlSessionFactoryBuilder → SqlSessionFactory → SqlSession → Mapper
     构建器                    工厂              会话        映射器
```

## **执行流程**

1. **配置解析**：XML/注解配置解析
2. **代理创建**：MapperProxy动态代理
3. **SQL解析**：动态SQL生成
4. **参数处理**：ParameterHandler处理
5. **SQL执行**：Executor执行器
6. **结果映射**：ResultSetHandler处理

## **缓存架构**

```Java
一级缓存（默认开启）
├── 作用域：SqlSession
├── 存储：本地HashMap  
└── 失效：增删改/提交/关闭

二级缓存（需配置）
├── 作用域：namespace
├── 存储：可配置(EhCache/Redis)
└── 失效：增删改操作
```

## **插件机制（拦截器）**

```java
@Intercepts({
    @Signature(type = Executor.class, 
               method = "query",
               args = {...})
})
public class MyPlugin implements Interceptor {
    // 拦截逻辑
}
```

## **动态SQL标签**

- **条件判断**：`<if>` `<choose>` `<when>` `<otherwise>`
- **循环处理**：`<foreach>`
- **字符处理**：`<trim>` `<where>` `<set>`
- **SQL片段**：`<sql>` `<include>`

## **最佳实践**

1. **批量操作**：使用foreach或批处理
2. **懒加载**：合理配置延迟加载
3. **缓存策略**：根据业务选择缓存级别
4. **SQL优化**：使用explain分析
5. **连接池**：选择高性能连接池如HikariCP

## **性能优化**

- 合理使用缓存
- 避免N+1查询问题
- 使用批量操作
- 优化resultMap映射
- 选择合适的fetchSize

## **常见问题**

- **SQL注入**：使用#{}而非${}
- **缓存穿透**：合理设置缓存策略
- **大结果集**：使用流式查询
- **事务管理**：Spring集成事务
