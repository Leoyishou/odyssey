---
comment_id: '98634191'
date created: 2025-02-04
date modified: 2025-02-06
draw: null
title: CR
---
## 一、题目

[第五届CR大赛-鲲鹏试卷.pdf](https://wiki.corp.qunar.com/download/attachments/905739467/%E7%AC%AC%E4%BA%94%E5%B1%8ACR%E5%A4%A7%E8%B5%9B-%E9%B2%B2%E9%B9%8F%E8%AF%95%E5%8D%B7.pdf?version=1&modificationDate=1717568963000&api=v2)

[第五届CR大赛-鸿鹄试卷.pdf](https://wiki.corp.qunar.com/download/attachments/905739467/%E7%AC%AC%E4%BA%94%E5%B1%8ACR%E5%A4%A7%E8%B5%9B-%E9%B8%BF%E9%B9%84%E8%AF%95%E5%8D%B7.pdf?version=1&modificationDate=1717568975000&api=v2)

## 二、采分点

### 鲲鹏

|      |                                                                                                                                                                                                                                                                                                                                                                                                             |
| ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 题目   | 采分点                                                                                                                                                                                                                                                                                                                                                                                                         |
| 第 1 题  | 1、Thread.sleep(100); 可能导致线程中断异常未正确处理，应该重新设置中断状态。【1 分】<br>2、consumption 方法在账户余额不足时，没有抛出异常或返回特定值，仅打印信息，会导致调用方无法正确识别业务逻辑。【0.5 分】<br>3、金额一般使用 BigDecimal 格式，尽量避免直接使用 double，会可能出现精度丢失的情况。【0.5 分】|
| 第 2 题  | 1、if (APP_CODE_VERSION_CACHE.size() == 0) {  <br>            cacheAppCodeVersion();  <br>        }   <br>在这段代码逻辑判断中，当缓存到期的时，size 并不会变为 0，所以这段代码逻辑只有第一次机器启动的时候会执行，后续再也不会执行。导致 String tcVersion = APP_CODE_VERSION_CACHE.getIfPresent(appCode); 获取的 tcVersion 永远为 null。【2 分】|
| 第 3 题  | 1、personList.add(person)，线程安全问题【1 分】<br>2、使用 Collections.synchronizedList 解决，并指出返回值 UOE 问题【1 分】|
| 第 4 题  | 1、data.qData.trim(), data.cData.trim() 会有 NPE【1 分】<br>2、String qData; String cData; 命名方式为 xXXX 不合理，应该为 xxxXxx【1 分】|
| 第 5 题  | 1、当线程池无法处理大量任务时会抛出 RejectedExecutionException,需要对此异常捕获并做相应处理。【1 分】<br>2、coundownLatch.countDown() 需要至少保证无论正常还是异常结束都被执行到，本程序当发生异常情况时 coundownLatch.countDown() 不会被执行到。可修改为在 catch 中也调用 coountDown()  或者将 countDown() 放在 finally 中。【1 分】<br>3、exception 缺少监控发现执行异常,如果得分未达到题目得分上限，可酌情给予【0.5 分】|
| 第 6 题  | 1、异常没有监控 or 错误日志没有打印 key 方便定位问题。【0.5 分】<br>2、setnx 和 expire 复合操作可以合并成原子操作 SET key value[EX seconds][PX milliseconds][NX\|XX] 【0.5 分】<br>3、syncDelete 进行删除的时候 应该根据 value 判断是否是当前线程获得的锁再进行删除【1 分】|
| 第 7 题  | 1、qconfig 的对象是可变的，方法 B 会往 wrapperManualBanList 里新增数据，造成数据污染，应该是返回 Collections.unmodifiableSet【1 分】<br>2、在高并发情况下方法 B 操作 wrapperManualBanList 会造成集合的数据缺失【1 分】|
| 第 8 题  | 1、isPaymentReceived 有并发问题，多个线程同时进入支付处理流程，造成订单状态更新的竞态条件，导致已支付金额不正确【1.5 分】<br>2、catch (Exception e)，在 catch 块中，捕获了 Exception，但是并没有对具体的异常进行处理，这里应该处理异常，比如回滚事务或更新订单状态【0.5 分】|
| 第 9 题  | 1、actionBean.setNextExecuteTime(new Date());，数据库里字段 next_execute_time 不匹配，会丢失毫秒数【2 分】<br>2、`create_time` timestamp 使用 timestamp 类型，会丢失毫秒数【1 分】<br>3、`next_execute_time` timestamp 使用 timestamp 类型，会丢失毫秒数【1 分】<br>4、总分未达上限的情况下，提到缺少参数【1 分】|
| 第 10 题 | 1、extValue.put(AUTO_PRICE_THRESHOLD, adaptPriceThreshold);【0.5 分】多线程有覆盖问题  <br>extValue.put(AUTO_PRICE_DECREASE_AMOUNT,Math.min(autoPriceDecreaseAmount, adaptPriceThreshold));【0.5 分】多线程有覆盖问题  <br>extValue.put(AUTO_PRICE_DECREASE_AMOUNT, 0.0f);【0.5 分】多线程有覆盖问题  <br>三个点都找全 2 分，找到 2 个 1 分，找到 1 个 0.5 分  <br>2、if (targetBasePrice > 0) {}【2 分】当 targetBasePrice 小于等于 0 时，应该有明确的处理，特别是对于调价结果的设置，否则存在错误调价业务风险，同时也无法完整体现业务逻辑 |
| 第 11 题 | 1、jedis.set(key, newStatus);  <br>【1 分】并发场景会有状态覆盖问题  <br>2、if (status == null) {return "UNKNOWN"; }【1 分】未获取到订单状态时可能会有多种原因，比如订单未创建、redis 暂时不可用、状态缓存过期等，统一返回 "unknown" 无法体现差异，对调用方基于订单状态做后续处理不友好  <br>3、jedis.set(statusKey, "CREATED");  <br>【1 分】在并发场景下会存在订单重复创建但系统没有感知  <br>4.return status;【1 分】没有做状态枚举检查，存在给返回非法状态且无感知的风险                                                                                     |
| 第 12 题 | 1、cityInfos.size() 不够严谨，需要加入其他合理限制条件，改动合理的情况下可以给【2 分】，目前看最保守的合理值是 min(510,cityInfos.size())，<br>2、newRejectedExecutionHandlerMonitor 中因为内容不能丢，所以需要修改 AbortPolicy 为 CallerRunsPolicy 补偿执行逻辑【1 分】<br>3、监控报警 给【0.5 分】<br>4、对 hotelInfoStoragePackages 判空，给【0.5 分】|
| 第 13 题 | 1、内循环中新建线程池或在循环外新建另一共享线程池可以给【2 分】，<br>2、空指针判断给【1 分】<br>3、严重写法问题，使用 slf4j 打日志给【1 分】|
| 第 14 题 | 1、new JSONObject(location) 是一个浅拷贝，在 newLocation.getJSONObject("rule").getJSONArray("condition_rule").add(routerMatchRule); 修改 newLocation 的 rule 的时候，依然会更改 location 的值【4 分】|
| 第 15 题 | 常量无法在 config 配置发生变更的时候进行值更新，有两处分别为 queryPreHours 和 limit【各 2 分】|
| 第 16 题 | 1、【问题点】按现有代码写法，在 qconfig 修改 corePoolSize 和 maxPoolSize 时，没法一次修改成功。假设初始配置 corePoolSize=4，maxPoolSize=4，若需要调大线程数，修改配置文件为 corePoolSize=8，maxPoolSize=8，行 12 会报错 IllegalArgumentException（核心线程数不能大于最大线程数）<br>【修复方案】在设置 corePoolSize 前判断下 maxPoolSize 是否满足>=corePoolSize 的条件，如果不满足需要先设置 maxPoolSize。同样，在设置 maxPoolSize 之前判断下 corePoolSize 是否满足<=maxPoolSize 的条件，如果不满足需要先设置 corePoolSize。【4 分】|
| 第 17 题 | 如果处理过程下游强依赖服务不可用抛异常，qmq 会一直不停本地重试，可能耗尽单机资源，影响业务，正确处理方式之一：throw new NeedRetryException(System.currentTimeMillis() + 30 * 1000, JSON.toJSONString(message));【4 分】<br>1、如答出业务方法上抛异常可能导致本地一直重试，消耗本机资源，给【2 分】；<br>2、如在 1 的基础上，给出使用 new NeedRetryException(System.currentTimeMillis() + 30 * 1000, JSON.toJSONString(message)); 这类用法，给【4 分】；<br>3、完全没有理解，给【0 分】；|
| 第 18 题 | 1、构造函数中开始消费消息，但是消息处理的 service 是 spring 注入的，因此可能导致抛出 NPE【5 分】|
| 第 19 题 | /fetch1 和/fetch2 均返回空报文。令人迷惑的点在这里，如果 controller 的方法返回 null 是不会触发 ResponseBodyAdvice 的，即使你用 ResponseEntity 把 null 包起来也不行，所以这里不能将 null 转化为{"success":true,"data":null,"message":null}。<br>/fetch1 回答正确得 2 分，/fetch2 回答正确得 3 分                                                                                                                                                                                                   |
| 第 20 题 | 1、redis 循环查询没有去重  【2.5 分】<br>2、redis 取值没有判空【1.5 分】<br>3、qps 过高方法中不建议 JSONObject.toJSONString(activityNoList) 打印【1 分】<br>4、如果提到走缓存失败查库或者查库成功加缓存，在总分不超题目上限的情况下可以【酌情给 1～2 分】|
| 第 21 题 | 1、semaphore 获取与释放令牌没有成对出现，可能由于令牌不释放，导致阻塞。<br>如：27、35、36 行，都有可能导致程序退出，但没有释放令牌。【2 分】<br>给出优化建议加【1 分】<br>2、向线程池提交任务时，阻塞等待执行结果（43 行），导致了两个问题，一）qmq 线程被阻塞。二）整体处理线程不会多于 qmq 线程，并发机制失效。【1 分】<br>3、线程池拒绝策略里只是打了一个监控，一旦触发拒绝策略，future 只能等待超时。【1 分】|
| 第 22 题 | 1、QMQ 消费逻辑异步，但未开启手动 ack【1 分】<br>2、父子任务提交到同一个线程池中，短时间提交多个父任务可能导致线程池线程耗尽，子任务永远无法执行。【1 分】<br>3、采用 Executors.newFixedThreadPool 线程池是无界队列，有 oom 的风险【1 分】<br>4、使用 countDownLatch,在 submit 处要有异常处理流程，并在异常处理中 countdown,防止如果拒绝策略是抛异常方式，无法 countdown 导致超时【1 分】<br>5、使用 countDownLatch.await() 要有超时控制【1 分】|
| 第 23 题 | 1、类型异常问题：【1 分】<br>接收到的外部属性 items 虽然有判空，但是并不一定是数组，因此直接使用 map 方法可能会导致找不到 map 的崩溃问题。<br>2、key 属性不可为空问题：【0.5 分】<br>key 属性为 null 时，使用时需要有兜底处理，如 item?.key \| index，确保健壮性。<br>3、item?.avatar  或 item.name：【0.5 分】<br>item?.avatar 或 item.name 需要有兜底处理，确保健壮性。|
| 第 24 题 | 1、cardData 的默认值会被 null 覆盖【4 分】|
| 第 25 题 | const formatToday = JSON.stringify(now).replace(/T.+\|"/g, '')  <br>此行有误，原因：JSON.stringify(new Date()) 会把时间转为 utc 格式，北京时间下每天 0 点 -8 点之间都会被转为上一天  <br>1、指出 formatToday 生成方式错误，【2 分】<br>2、写出正确的生成日期格式代码，【2 分】|
| 第 26 题 | 将 setTimeout 中的 === 改成 >= 和 <=,当自动轮播期间，手动滚动的时机恰逢 duration 期间时，currentIndex 会被无限加 1 或减 1，导致轮播图因找不到而出现空白  <br>1、=== 改成 >=，【2 分】<br>2、第二个=== 改成 <=，【2 分】<br>【总分不变；提到 clearTimeout 1 分；提到 this.currentIndex = 0，1 分】|
| 第 27 题 | 1、components[index]= JSON.parse(JSON.stringify(component).replace(new RegExp(mId, 'g'), generateMid));    得分【2 分】<br>2、去掉没必要的 if-else 层级  得分【1 分】<br>3、JSON.parse(JSON.stringify()) 要加 try catch 得分【1 分】，【总分不变，detailList 判空和应该放在!existMid 判断后使用，1 分，mid 判空 1 分】|
| 第 28 题 | 1、forEach 中的 await 无法保障执行顺序：【3 分】<br>forEach 方法是一个同步操作，它在遍历数组时会依次执行提供的回调函数。回调函数中的代码会立即执行，而不会等待异步操作完成。可使用 for of 代替 forEach 保障执行顺序。<br>2、数值字符串类型不可直接使用：【2 分】<br>尽管后端返回的数据使用 Number 包装后是数值，但实际在 render 赋值时没有使用 Number() 转换。当 '100' 这种数值字符串给 width, height 赋值时，RN 将不被识别，直接报类型异常崩溃。|

### 鸿鹄

|        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 题目     | 采分点                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| 第 1 题  | 1、useraccount 命名不符合驼峰命名，userAccount【0.5 分】<br>2、Thread.sleep(100); 可能导致线程中断异常未正确处理，应该重新设置中断状态。【1 分】<br>3、System.out.println("Insufficient funds"); 在多线程环境下不应直接使用 System.out 打印，可能会导致日志错乱。【0.5 分】<br>4、consumption 方法在账户余额不足时，没有抛出异常或返回特定值，仅打印信息，会导致调用方无法正确识别业务逻辑。【0.5 分】<br>5、金额一般使用 BigDecimal 格式，尽量避免直接使用 double，会可能出现精度丢失的情况。【0.5 分】|
| 第 2 题  | 1、if (APP_CODE_VERSION_CACHE.size() == 0) {  <br>            cacheAppCodeVersion();  <br>        }   <br>在这段代码逻辑判断中，当缓存到期的时，size 并不会变为 0，所以这段代码逻辑只有第一次机器启动的时候会执行，后续再也不会执行。导致 String tcVersion = APP_CODE_VERSION_CACHE.getIfPresent(appCode); 获取的 tcVersion 永远为 null。【3 分】|
| 第 3 题  | 1、personList.add(person)，线程安全问题【1.5 分】<br>2、使用 Collections.synchronizedList 解决，并指出返回值 UOE 问题【1.5 分】|
| 第 4 题  | 1、data.qData.trim(), data.cData.trim() 会有 NPE【1 分】<br>2、String qData; String cData; 命名方式为 xXXX 不合理，应该为 xxxXxx【2 分】|
| 第 5 题  | 1、exception 缺少监控发现执行异常【0.5 分】<br>2、当线程池无法处理大量任务时会抛出 RejectedExecutionException,需要对此异常捕获并做相应处理。【1 分】<br>3、coundownLatch.countDown() 需要至少保证无论正常还是异常结束都被执行到，本程序当发生异常情况时 coundownLatch.countDown() 不会被执行到。可修改为在 catch 中也调用 coountDown()  或者将 countDown() 放在 finally 中。【1.5 分】|
| 第 6 题  | 1、异常没有监控 or 错误日志没有打印 key 方便定位问题。【0.5 分】<br>2、setnx 和 expire 复合操作可以合并成原子操作 SET key value[EX seconds][PX milliseconds][NX\|XX] 【1 分】<br>3、syncDelete 进行删除的时候 应该根据 value 判断是否是当前线程获得的锁再进行删除【1.5 分】|
| 第 7 题  | 1、qconfig 的对象是可变的，方法 B 会往 wrapperManualBanList 里新增数据，造成数据污染，应该是返回 Collections.unmodifiableSet【1 分】<br>2、在高并发情况下方法 B 操作 wrapperManualBanList 会造成集合的数据缺失【2 分】|
| 第 8 题  | 1、空指针：List<PNRSegmentInfo> segmentInfos = readPNRResponse.getSegmentInfos(); 遍历获取的 segmentInfos 时，需要进行判空操作；【0.5 分】<br>2、使用 segmentInfos.stream() 解决遍历获取的 segmentInfos 的 NPE 问题【0.5 分】<br>3、DateUtil.parse 可能出异常，此场景会将异常抛给上游。【0.5 分】<br>4、不可变集合的 sort 排序，会抛出异常 java.lang.UnsupportedOperationException: null  <br>at java.util.Collections$UnmodifiableList.sort(Collections.java:1333) ~[na:1.8.0_331]，可以把这个集合变成可变的集合 list = new ArrayList<>(list);【1.5 分】|
| 第 9 题  | 1、actionBean.setNextExecuteTime(new Date());，数据库里字段 next_execute_time 不匹配，会丢失毫秒数，会导致没有立刻执行【3 分】<br>2、`create_time` timestamp 使用 timestamp 类型，会丢失毫秒数【1 分】<br>3、`next_execute_time` timestamp 使用 timestamp 类型，会丢失毫秒数【1 分】|
| 第 10 题 | 1、extValue.put(AUTO_PRICE_THRESHOLD, adaptPriceThreshold);【0.5 分】多线程有覆盖问题  <br>extValue.put(AUTO_PRICE_DECREASE_AMOUNT,Math.min(autoPriceDecreaseAmount, adaptPriceThreshold));【0.5 分】多线程有覆盖问题  <br>extValue.put(AUTO_PRICE_DECREASE_AMOUNT, 0.0f);【0.5 分】多线程有覆盖问题  <br>以上三个点都找全 1.5 分，找到 2 个 1 分，找到 1 个 0.5 分  <br>2、if (targetBasePrice > 0) {}【2 分】当 targetBasePrice 小于等于 0 时，应该有明确的处理，特别是对于调价结果的设置，否则存在错误调价业务风险，同时也无法完整体现业务逻辑  <br>3、logger.error("Error processing fareDataKey: {}", fareDataKey, ex);【0.5 分】缺少异常监控  <br>4、if (MapUtils.isEmpty(packagePriceMap)) {return;}【1 分】对于非法入参没有监控和日志，缺少发现手段 |
| 第 11 题 | 1.jedis.set(key, newStatus);  <br>【1 分】并发场景会有状态覆盖问题  <br>【1 分】订单状态机流转缺少可靠前置状态检查  <br>2. if (status == null) {return "UNKNOWN"; }【1 分】未获取到订单状态时可能会有多种原因，比如订单未创建、redis 暂时不可用、状态缓存过期等，统一返回 "unknown" 无法体现差异，对调用方基于订单状态做后续处理不友好  <br>3. jedis.set(statusKey, "CREATED");  <br>【1 分】在并发场景下会存在订单重复创建但系统没有感知  <br>4.return status;【1 分】没有做状态枚举检查，存在给返回非法状态且无感知的风险                                                                                                                                                                                                                                                |
| 第 12 题 | 1、cityInfos.size() 不够严谨，需要加入其他合理限制条件，改动合理的情况下可以给【3 分】，目前看最保守的合理值是 min(510,cityInfos.size())，<br>2、newRejectedExecutionHandlerMonitor 中因为内容不能丢，所以需要修改 AbortPolicy 为 CallerRunsPolicy 补偿执行逻辑【1 分】<br>3、监控报警 给【0.5 分】<br>4、对 hotelInfoStoragePackages 判空，给【0.5 分】|
| 第 13 题 | 1、内循环中新建线程池或在循环外新建另一共享线程池可以给【3 分】，<br>2、空指针判断给【1 分】<br>3、严重写法问题，使用 slf4j 打日志给【1 分】|
| 第 14 题 | 1、new JSONObject(location) 是一个浅拷贝，在 newLocation.getJSONObject("rule").getJSONArray("condition_rule").add(routerMatchRule); 修改 newLocation 的 rule 的时候，依然会更改 location 的值【5 分】|
| 第 15 题 | 1、常量无法在 config 配置发生变更的时候进行值更新，有两处分别为 queryPreHours 和 limit，【各 2.5 分】|
| 第 16 题 | 1、【问题点】按现有代码写法，在 qconfig 修改 corePoolSize 和 maxPoolSize 时，没法一次修改成功。假设初始配置 corePoolSize=4，maxPoolSize=4，若需要调大线程数，修改配置文件为 corePoolSize=8，maxPoolSize=8，行 12 会报错 IllegalArgumentException（核心线程数不能大于最大线程数）【2 分】<br>【修复方案】在设置 corePoolSize 前判断下 maxPoolSize 是否满足>=corePoolSize 的条件，如果不满足需要先设置 maxPoolSize。同样，在设置 maxPoolSize 之前判断下 corePoolSize 是否满足<=maxPoolSize 的条件，如果不满足需要先设置 corePoolSize。【3 分】|
| 第 17 题 | 1、如答出业务方法上抛异常可能导致本地一直重试，消耗本机资源，给【2 分】；<br>2、如在 1 的基础上，给出使用 new NeedRetryException(System.currentTimeMillis() + 30 * 1000, JSON.toJSONString(message)); 这类用法，给【5 分】；<br>3、完全没有理解，给【0 分】；|
| 第 18 题 | 1、e.printStackTrace(); 日志打印方式错误，需要使用 LOGGER。【1 分】<br>2、构造函数中开始消费消息，但是消息处理的 service 是 spring 注入的，因此可能导致抛出 NPE【6 分】|
| 第 19 题 | /fetch1 和/fetch2 均返回空报文。令人迷惑的点在这里，如果 controller 的方法返回 null 是不会触发 ResponseBodyAdvice 的，即使你用 ResponseEntity 把 null 包起来也不行，所以这里不能将 null 转化为{"success":true,"data":null,"message":null}。<br>/fetch1 回答正确得【3 分】，/fetch2 回答正确得【4 分】|
| 第 20 题 | 1. 类型异常问题：【1 分】<br>接收到的外部属性 items 虽然有判空，但是并不一定是数组，因此直接使用 map 方法可能会导致找不到 map 的崩溃问题。<br>2. key 属性不可为空问题：【1 分】<br>key 属性为 null 时，会直接导致 crash。使用时需要有兜底处理，如 item?.key \| index，确保健壮性。<br>3. item?.avatar 或 item?.name 兜底问题【1 分】<br>item?.avatar 和  item?.name  需要有兜底处理，确保健壮性。|
| 第 21 题 | 1、cardData 的默认值会被 null 覆盖    【5 分】|
| 第 22 题 | const formatToday = JSON.stringify(now).replace(/T.+\|"/g, '')  <br>此行有误，原因：JSON.stringify(new Date()) 会把时间转为 utc 格式，北京时间下每天 0 点 -8 点之间都会被转为上一天  <br>1、指出 formatToday 生成方式错误，【2 分】<br>2、写出正确的生成日期格式代码，【3 分】|
| 第 23 题 | 将 setTimeout 中的 === 改成 >= 和 <=,当自动轮播期间，手动滚动的时机恰逢 duration 期间时，currentIndex 会被无限加 1 或减 1，导致轮播图因找不到而出现空白  <br>1、=== 改成 >=，【2 分】<br>2、第二个=== 改成 <=，【2 分】<br>3、this.currentIndex = 1 改为 0，【1 分】|
| 第 24 题 | 1、timer 不能是一个变量，【3 分】<br>2、useEffect 没有设置 cleanup【2 分】|
|        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

[](https://wiki.corp.qunar.com/pages/viewpage.action?pageId=905739467)

- 并发和线程安全问题
    - 多线程环境下的数据竞争和覆盖
    - 线程池使用不当
    - 锁使用不正确
- 异常处理不当
    - 异常捕获不完整或处理不当
    - 缺少必要的异常监控和日志记录
- 性能问题
    - 不必要的对象创建
    - 缓存使用不当
    - 循环中的低效操作
- 命名和代码风格问题
    - 不符合驼峰命名规范
    - 变量命名不清晰或不合理
- 数据类型和精度问题
    - 使用不适当的数据类型 (如用 double 表示金额)
    - 时间精度丢失问题
- 空指针和边界条件处理
    - 缺少必要的 null 检查
    - 边界条件考虑不周全
- API 使用不当
    - 误用或滥用某些 API 功能
    - 没有充分利用 API 提供的功能
- 配置和可维护性问题
    - 硬编码常量
    - 配置更新机制不完善
- 数据库操作问题
    - SQL 语句优化
    - 事务处理不当
- 缓存和分布式问题
    - Redis 操作原子性问题
    - 分布式锁使用不当
