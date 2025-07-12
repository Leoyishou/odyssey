---
date created: 2024-11-25
date modified: 2025-07-10
uid: 4eba3fc6-5849-49ad-9db2-86a57a0cce0a
---
## 公网 IP

`curl ifconfig.me`

### 网络接口顺序

1. **物理网卡**：`eth0`
2. **本地回环接口**：`lo`
3. **Docker默认网桥**：`docker0`
4. **Docker自定义网桥**：`br-xxx`
5. **Docker容器虚拟网卡**：`veth-xxx`

```mermaid
flowchart TD
    subgraph Host["Host (物理服务器)"]
        eth0["eth0\n10.5.0.12\n(物理网卡)"]
        lo["lo\n127.0.0.1\n(本地回环)"]
    end

    subgraph DockerNetwork["Docker 网络环境"]
        docker0["docker0\n172.17.0.1\n(默认网桥)"]
        br["br-dff855d3e4cc\n172.24.0.1\n(自定义网桥)"]
        
        subgraph Containers["容器"]
            veth1["veth0cbf08d\n(容器虚拟网卡1)"]
            veth2["vethff390ca\n(容器虚拟网卡2)"]
        end
    end

    Internet["外部网络"] --> eth0

    eth0 --> docker0
    eth0 --> br

    docker0 --> veth1
    br --> veth2

    lo <--> lo
    
    classDef networkBridge fill:#f9f,stroke:#333,stroke-width:2px
    classDef container fill:#bbf,stroke:#333,stroke-width:2px
    classDef physical fill:#bfb,stroke:#333,stroke-width:2px
    
    class docker0,br networkBridge
    class veth1,veth2 container
    class eth0,lo physical
```

## 网络统计信息

```Java
eth0:
RX packets 2377130  bytes 3372009777 (3.3 GB)  # 已接收的数据
TX packets 322799  bytes 32328691 (32.3 MB)    # 已发送的数据
```

这表明您的服务器已经接收了约 3.3GB 的数据，发送了约 32.3MB 的数据。

- RX = Receive (接收)
- TX = Transmit (发送)
