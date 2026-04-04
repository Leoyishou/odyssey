---
uid: a6c9b39d-84a1-48e1-bb65-6652660bbca5
date created: 2025-07-19
date modified: 2025-10-07
aliases: [Summary]
linter-yaml-title-alias: Summary
---
#  💡Summary  

> [!Info]
> 一句话说清楚
> 

  lark-cli 是飞书官方发布的编译好的原生二进制文件（Mach-O arm64），不是 Python 也不是纯 Node.js。                                                    
```
  包结构：                                                                                                                                           
  - npm 包：@larksuite/cli（飞书官方）     
  - 安装时通过 postinstall 脚本下载对应平台的预编译二进制                                                                                  
  - /opt/homebrew/lib/node_modules/@larksuite/cli/bin/lark-cli — 真正的可执行文件，编译后的原生代码
  - /opt/homebrew/bin/lark-cli — 只是一个 Node.js 薄包装脚本，转发参数给真正的二进制         
```


## 本质是个 HTTP 客户端                                                                                                        
  1. 管理 OAuth token（登录、刷新、缓存）                                                                                       
  2. 拼装请求（把 --params/--data 变成正确的 URL + JSON body）                                                                                       
  3. 发 HTTPS 请求到 https://open.feishu.cn/open-apis/...                                                                                            
  4. 解析响应返回给你                                                                                                                                
                                                                                                                                                     
  本质和你自己用 curl 调飞书 API 一样，只是省去了手动处理认证和参数拼装的麻烦。  
  
# 🧩 Cues



# 🪞Notes


  1. 底层：lark-cli 命令行工具                                                                                                                       
                                                                                                                                                     
  本地安装了 /opt/homebrew/bin/lark-cli，这是一个封装了飞书开放平台 REST API 的 CLI 工具。它处理了：                                                 
  - OAuth 认证（token 管理）                                                                                                                         
  - API 请求构建（params/data JSON 序列化）                                                                                                          
  - 分页、输出格式化等通用逻辑                                                                                                                    
                                                                                                                                                     
  2. 中层：Skill 文档（SKILL.md + references/）                                                                                                      
                                                                                                                                                     
  lark-sheets skill 本质上是一份结构化的使用手册，告诉 Claude：                                                                                      
  - 有哪些 shortcut 可用（+read, +write, +append, +create, +find, +export, +info）                                                                   
  - 每个命令的参数、用法、注意事项                                                                                                                   
  - 飞书的核心概念（token 类型、wiki 特殊处理、筛选操作流程）                                                                                     
  - 权限要求                                                                                                                                         
                                                                                                                                                     
  3. 执行层：Claude 读 Skill → 生成 Bash 命令                                                                                                        
                                                                                                                                                     
  当用户说"帮我读一下这个飞书表格"，Claude 会：                                                                                                      
  4. 加载 SKILL.md 获取用法说明                                                                                                                      
  5. 根据用户需求组装 lark-cli sheets +read --url "..." --range "..." 命令                                                                           
  6. 通过 Bash 工具执行，拿到结果        