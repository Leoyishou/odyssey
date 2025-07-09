---
comment_id: 3d3b1033
date created: 2023-12-04
date modified: 2025-02-06
draw: null
title: K8s
---
## 配置细节

[Setup Your K8s Cluster with AWS EC2 | by Milinda Nandasena | Medium](https://milindasenaka96.medium.com/setup-your-k8s-cluster-with-aws-ec2-3768d78e7f05)

## 三层网络

1. ingress 层 对外的总代理、总网关
2. service 层网络 k8s init 的时候指定
3. pod 层网络 k8s init 的时候指定

apiServer  
proxy  
etcd  
scheduler  
controller

DAU 是什么  
服务缺陷率  
工单的全 cheng 

**kubeadm init --apiserver-advertise-address=INSTANCE_PRIVATE_IP --pod-network-cidr=192.168.0.0/16 --ignore-preflight-errors=all**
