---
date created: 2024-01-15
date modified: 2025-07-10
uid: 610caa63-13f4-467e-96de-6cbbe2a58f1d
---
## 查询结果

 #最佳实践 通过`<resultMap`映射到某一个类，这样更解耦

```sql
<resultMap id="BaseResultMap" type="com.qunar.dzs.hotelsearch.polaris.domain.entity.task.PreRunTask">  
    <id column="id" property="id"/>  
    <result column="name" property="name"/>  
    <result column="pre_run_date" property="preRunDate"/>  
    <result column="status" property="status"/>  
    <result column="basic_groups" property="basicGroups"/>  
    <result column="diff_groups" property="diffGroups"/>  
    <result column="basic_rules" property="basicRules"/>  
    <result column="diff_rules" property="diffRules"/>  
    <result column="compound_metrics" property="compoundMetrics"/>  
    <result column="platform_proposals" property="platformProposals"/>  
    <result column="create_user" property="createUser"/>  
    <result column="modified_user" property="modifiedUser"/>  
    <result column="create_time" property="createTime"/>  
    <result column="update_time" property="modifiedTime"/>  
</resultMap>
```

```sql
<select id="list" resultMap="BaseResultMap">
	SELECT
	<include refid="Base_Column_List"/>
	FROM pre_run_task
	<where>
		<if test="status != null">
			AND status = #{status}
		</if>
	</where>
	ORDER BY create_time DESC
</select>
```

## 标签

### where

mybatis 会自动去掉多余的 and 吗？

```java
    <select id="count" resultType="java.lang.Integer">
        SELECT count(1) from pre_run_task
        <where>
            <if test="params.taskId != null">
                and id = #{params.taskId}
            </if>
            <if test="params.name != null">
                and name like concat('%', #{params.taskName}, '%')
            </if>           
        </where>
    </select>
```

是的，MyBatis 的 `<where>` 标签会自动处理这种情况。它会在生成最终 SQL 时，自动去掉最前面的 `and` 或 `or`，从而避免生成多余的逻辑运算符。也就是说，无论你的 `<if>` 条件中是否添加了 `and`，最终拼接出来的 SQL 都不会出现开头多余的 `and`。

## MyBatis 领域词汇表

| 术语           | 含义                                                         |
|---------------|-------------------------------------------------------------|
| 映射器文件     | 定义 SQL 语句和 Java 对象之间映射关系的 XML 文件                |
| SqlSession    | MyBatis 的核心接口，用于执行 SQL 命令、获取映射器和管理事务      |
| 会话工厂       | 用于生成 SqlSession 的工厂类                                   |
| 数据源         | 数据库连接的来源，配置 JDBC 数据库连接信息                     |
| 连接池         | 数据库连接的缓存池，提高数据库连接效率                         |
| 事务管理       | 控制事务提交、回滚和关闭的管理接口                             |
| 动态 SQL       | 根据条件动态生成 SQL 语句                                      |
| 映射接口       | 映射器文件对应的 Java 接口，用于定义 SQL 操作方法              |
| 输入映射       | 将 Java 对象映射为 SQL 语句的输入参数                          |
| 输出映射       | 将 SQL 查询结果映射回 Java 对象                                |
| 参数占位符     | 在 SQL 语句中用于代替实际参数值的占位符                        |
| 结果集处理     | 处理从数据库查询返回的结果集                                   |
| 选择器         | 特定的 SQL 语句，用于从数据库中选择数据                        |
| 插入器         | 特定的 SQL 语句，用于将数据插入数据库                          |
| 更新器         | 特定的 SQL 语句，用于更新数据库中的数据                        |
| 删除器         | 特定的 SQL 语句，用于从数据库中删除数据                        |
| 绑定方法       | 将方法调用绑定到指定的 SQL 操作                                |
| 类型别名       | 为 Java 类型设置一个简短的名字，简化配置文件中的类型引用        |
| 类型处理器     | 处理 Java 类型与数据库类型之间的转换                           |
| 拦截器         | 可插拔的组件，用于拦截核心执行的操作（如 SQL 执行）|
| 插件           | 自定义修改 MyBatis 核心行为的方法，通常用于扩展功能            |
| 缓存           | 存储已执行 SQL 的结果，以提高查询效率                          |
| 本地缓存       | SqlSession 级别的缓存，用于缓存一个会话中的数据                |
| 二级缓存       | 映射器级别的缓存，跨多个 SqlSession 共享                       |
| 延迟加载       | 延迟初始化对象属性，直到真正使用时才查询数据库                 |
| SQL 构建器     | 程序化方式构建 SQL 语句的工具                                 |
| XML 构建器     | 解析映射器 XML 文件的工具                                      |
| 注解           | 用于在接口方法上定义 SQL，替代 XML 映射文件的方式              |
| 会话绑定       | 将 SqlSession 绑定到特定的 Java 对象或生命周期中                |
| 会话跟踪       | 跟踪和管理 SqlSession 的状态和事务行为                         |
| 事务隔离级别   | 定义数据库操作的事务隔离级别                                  |
| 事务传播行为   | 定义事务方法调用的行为模式                                    |
| 配置解析       | 解析 MyBatis 配置文件并构建运行环境                            |
| 字段映射       | 将数据库表的字段映射到 Java 对象的属性中                       |
| 集合映射       | 处理 SQL 查询结果为集合类型的映射                              |
| 级联映射       | 映射涉及到对象关联的复杂 SQL 结构                             |
| 分步查询       | 将复杂的查询拆分为多个步骤执行，通常用于优化性能               |
| 标识生成       | 自动生成主键值的策略                                          |
| SQL 片段       | 重用 SQL 代码的片段，通过 `<include>` 标签包含在其他 SQL 中    |
| 查询缓存       | 用于缓存查询结果，减少数据库访问次数                          |
| 条件构造       | 动态构建 SQL 中的条件表达式                                   |
| 语句构造器     | 构建动态 SQL 语句的组件                                       |
| 执行器         | 负责执行 SQL 命令并返回处理结果的组件                          |
| 环境配置       | 定义 MyBatis 运行环境的设置，如数据源和事务管理的配置          |
| 日志配置       | 配置 MyBatis 日志行为，用于调试和监控                          |
| 映射注册       | 在配置中注册和管理 SQL 映射的过程                              |
| 实体类         | 代表数据库表中数据的 Java 类                                  |
| 分页查询       | 对查询结果进行分页处理的技术                                  |
| 优化查询       | 提高查询效率的策略，例如使用合适的索引                        |
| 查询参数       | 传递给 SQL 语句的参数                                        |
| 返回值处理     | 处理 SQL 查询或命令的返回结果                                 |
| SQL 审计       | 跟踪和记录执行的 SQL 语句，用于性能分析和安全监控              |
| 会话超时       | 设置 SqlSession 的超时时间，影响事务的持续时间                |
| 配置验证       | 启动时检查 MyBatis 配置的正确性                               |
| 数据完整性     | 确保数据库操作不破坏数据的准确性和一致性                      |
| 错误处理       | 管理和响应 SQL 执行过程中发生的错误                           |
| 性能监控       | 跟踪 SQL 执行时间和资源使用，用于优化                          |
| 安全控制       | 防止 SQL 注入等安全威胁的策略                                 |
| 代码生成       | 自动生成基于 MyBatis 的数据访问层代码                         |
| 批处理         | 同时执行多条 SQL 语句以提高性能                               |
| 事务日志       | 记录事务操作的详细信息，用于故障恢复                          |
| 数据迁移       | 将数据从一个数据库迁移到另一个数据库的过程                    |
| 数据校验       | 在执行 SQL 之前验证数据的合法性                               |
| 语句重用       | 在多个查询中重用相同的 SQL 语句                               |
| 资源管理       | 管理数据库连接、语句和结果集的资源                            |
| 数据映射       | 将数据库中的数据转换为应用程序中的数据结构                    |
| 应用集成       | 将 MyBatis 集成到企业应用架构中                              |
| 依赖注入       | 自动将依赖关系注入到组件中，简化配置和管理                    |
| 模块化设计     | 将系统分解为多个模块，各自独  

画一个 java-spring-mybatis-jdbc-pg 的图

The _where_ element knows to only insert "WHERE" if there is any content returned by the containing tags. Furthermore, if that content begins with "AND" or "OR", it knows to strip it off.  
If the _where_ element does not behave exactly as you like, you can customize it by defining your own trim element. For example, the trim equivalent to the _where_ element is:
