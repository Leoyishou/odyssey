---
draw:
title: Untitled
tags: [1 Dev, Java, TroubleShootin, 信息革命]
date created: 2024-04-28
date modified: 2024-11-12
---

excerpt

<!-- more -->

## 成功运行

```java
Map<Object, Object> rooms = Maps.newHashMap();  
try {  
    ExtendListenableFuture<Map<Object, Object>> hgetall = redisAsyncClient.hgetall(REDIS_KEY_PREFIX + hotelSeq);  
    rooms = hgetall.get(hotelQHStatsDataConfig.getAdrPriceFutureTimeout(), TimeUnit.MILLISECONDS);  
} catch (Exception e) {  
    log.error(ADR_PRICE_REDIS_SERVICE + "查询某酒店 adr 价格失败, hotelSeqs:{}", hotelSeq, e);  
    QMonitor.recordOne(ADR_PRICE_REDIS_SERVICE + "del_record_exception");  
}  
  
List<String> needDeletePrices = Lists.newArrayList();  
for (Map.Entry<Object, Object> room : rooms.entrySet()) {  
    String adr = new String((byte[]) room.getValue());  
    String[] split = adr.split("_");  
    if (split.length == 2) {  
        long createTime = NumberUtils.toLong(split[1]);  
        if (System.currentTimeMillis() - createTime > hotelQHStatsDataConfig.getAdrPriceExpireTime()) {  
            needDeletePrices.add(new String((byte[]) room.getKey()));  
        }  
    }  
}
```

本质是因为，往 [redis](redis.md) 写的时候经过了一层 encode

```java
@Override  
public void encode(ByteBuf buf, Object value) {  
    if (value == null) {  
        throw new NullPointerException();  
    }  
  
    // string  
    if (value instanceof String) {  
        byte[] bytes = Charsets.encode(value.toString(), Charsets.UTF8_CHARSET_NAME);  
        ProtocolUtil.write(buf, bytes);  
        return;  
    }  
  
    if (value instanceof Number) {  
        encodeAscii(buf, value.toString());  
        return;  
    }  
  
    if (value instanceof byte[]) {  
        ProtocolUtil.write(buf, (byte[]) value);  
        return;  
    }  
  
    writeObject(buf, value);  
}
```

用 new String((byte[]) room.getKey())，可以享受一种默认的 decode 规则

```java

public String(byte bytes[], int offset, int length) {  
    checkBounds(bytes, offset, length);  
    this.value = StringCoding.decode(bytes, offset, length);  
}

java.lang.StringCoding#decode(byte[], int, int)

static char[] decode(byte[] ba, int off, int len) {  
    String csn = Charset.defaultCharset().name();  
    try {  
        // use charset name decode() variant which provides caching.  
        return decode(csn, ba, off, len);  
    } catch (UnsupportedEncodingException x) {  
        warnUnsupportedCharset(csn);  
    }  
    try {  
        return decode("ISO-8859-1", ba, off, len);  
    } catch (UnsupportedEncodingException x) {  
        // If this code is hit during VM initialization, MessageUtils is  
        // the only way we will be able to get any kind of error message.        MessageUtils.err("ISO-8859-1 charset not available: "  
                         + x.toString());  
        // If we can not find ISO-8859-1 (a required encoding) then things  
        // are seriously wrong with the installation.        System.exit(1);  
        return null;  
    }  
}


java.nio.charset.Charset#defaultCharset

public static Charset defaultCharset() {  
    if (defaultCharset == null) {  
        synchronized (Charset.class) {  
            String csn = AccessController.doPrivileged(  
                new GetPropertyAction("file.encoding"));  
            Charset cs = lookup(csn);  
            if (cs != null)  
                defaultCharset = cs;  
            else  
                defaultCharset = forName("UTF-8");  
        }  
    }  
    return defaultCharset;  
}
```
