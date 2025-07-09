---
draw:
title: NFTurbo
date created: 2024-10-27
date modified: 2025-05-03
---

## 目标

熟悉一下[ElasticSearch](ElasticSearch.md)，对电商中除了报价之外的其他模块的代码实现有一个思路

## 模块

| 难度等级 | 适用人群 |
|----------|----------|
| ⭐☆☆☆☆ | 适合在校生、实习生 |
| ⭐⭐☆☆☆ | 适合实习生、应届生 |
| ⭐⭐⭐☆☆ | 适合应届生，工作1年内的人 |
| ⭐⭐⭐⭐☆ | 适合工作3年内的人 |
| ⭐⭐⭐⭐⭐ | 适合工作8年内的人 |

| 模块   | 模块名                  | 难度    | 难点&亮点                                                   | 技术列表                                                                               |
| ---- | -------------------- | ----- | ------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| 消息通知 | nft-turbo-notice     | ⭐☆☆☆☆ | 并发控制、CRUD、外部对接                                          | 滑动窗口、短信服务                                                                          |
| 网关   | nft-turbo-gateway    | ⭐⭐☆☆☆ | gateway的理解、统一鉴权、负载均衡、统一限流                               | spring-cloud-gateway、loadbalancer、nacos、sentinel、sa-token、switch表达式                |
| 认证   | nft-turbo-auth       | ⭐⭐☆☆☆ | 统一认证、分布式session                                         | sa-token、redis、dubbo、springmvc                                                     |
| 区块链  | nft-turbo-chain      | ⭐⭐⭐☆☆ | 外部对接、多chain切换、限流降级                                      | 策略+工厂模式、文昌链对接、sentinel、rocketmq                                                    |
| 用户   | nft-turbo-user       | ⭐⭐⭐☆☆ | 用户信息缓存、用户名判重、数据脱敏、数据加解密、分布式锁、短信发送、滑动窗口限流、缓存一致性、实名认证、排行榜 | caffeine、redis、jetcache、redisson、bloomfilter、oss、dubbo、虚拟线程                        |
| 藏品   | nft-turbo-collection | ⭐⭐⭐⭐☆ | 搜索、ES和DB动态切换、限流、防超卖                                     | elasticsearch、canal、redis、caffeine、jetcache、dubbo                                  |
| 订单   | nft-turbo-order      | ⭐⭐⭐⭐☆ | 订单重复提交、订单到期关闭、分库分表                                      | xxl-job、token、rocketmq、springevent、shardingjdbc、雪花算法、dubbo、责任链模式、虚拟线程、textblock    |
| 支付   | nft-turbo-pay        | ⭐⭐⭐⭐☆ | 重复支付、数据一致性、支付回调、数据对账、多付退款、渠道对接                          | seata、微信支付对接、xxl-job、分布式锁、限流、分布式事务                                                 |
| 交易   | nft-turbo-trade      | ⭐⭐⭐⭐⭐ | 秒杀、幂等控制、事务消息、分布式ID、库存扣减                                 | xxl-job、token、rocketmq、springevent、dubbo、责任链模式、虚拟线程、lua脚本、textblock、inventory hint |
| 后台管理 | nft-trubo-admin      | ⭐⭐⭐☆☆ | 改价、修改库存、用户冻结、下架                                         | sa-token、redis、延迟双删、缓存失效、鉴权                                                        |
| 玩法   |                      | ⭐⭐⭐⭐⭐ |                                                         |                                                                                    |
|      |                      |       |                                                         |                                                                                    |
