---
uid: 964cfbeb-fe4d-4291-999f-d62136106019
date created: 2025-07-19
date modified: 2025-10-07
---
# 💡Summary

1. 就是说命令行启动的 codex，无法用 ChatGPT 的 browse Use 插件，而是新开一个chrome是吧






[[CDP]]


| 路线                | 代表方案                                                                | 优势                                                                                                                       | 短板                                                                       | 适用场景                                                                                         |
| ------------------- | ----------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| ① 传统自动化        | Playwright、Puppeteer、Selenium                                         | 精准、确定、可复现；毫秒级、零 Token                                                                                       | 页面改版后维护选择器成本高；遇到未知页面要引入 AI                          | 稳定的端到端测试、固定自动化任务                                                                 |
| ② CDP + MCP/CLI     | Chrome DevTools MCP、Playwright MCP/CLI、agent-browser、Browser Harness | 生态成熟、开箱即用，不用自己维护浏览器内核                                                                                 | CDP 只是底层协议，不是完整产品；每步数据太多容易撑爆上下文                 | 把现成浏览器接给 Agent；Browser Harness 可接管本机已登录的 Chrome                                |
| ③ 浏览器插件        | OpenCLI、Kimi 扩展、ChatGPT 扩展、Claude in Chrome、Nanobrowser         | 门槛低，直接复用 Cookie 和登录态；OpenCLI 还能把高频网站封装成本地 CLI 命令                                                | 权限要求高，受宿主浏览器稳定性制约；高并发和长期无人值守需要额外防故障设计 | 接管日常已登录的 Chrome（OpenCLI、Browser Harness、Kimi 扩展）                                   |
| ④ 原生 Agent 浏览器 | Ego Lite、BrowserOS neo                                                 | 基于定制 Chromium，内嵌 Node.js，原生 Snapshot，独立 Space 供人和 Agent 并行，可人机接管验证码，用 Skill 对接 Coding Agent | Ego Lite 只开源 Harness 和 Skill，核心闭源；BrowserOS neo 完全开源         | 人和 Agent 在同一浏览器里并行协作                                                                |
| ⑤ AI 自主操作       | Browser Use、Stagehand、Skyvern、Jev Ultrafast                          | 灵活：Browser Use 全自主；Stagehand 固定步骤写代码、动态节点交给 AI；Skyvern 用视觉模型加工作流；Jev 压低延迟和成本        | 全自主 Token 开销大、结果有波动；Jev 能覆盖的网页复杂度有限                | 探索陌生站点（Browser Use）；做混合型自动化（Stagehand + Playwright）；企业长链路业务（Skyvern） |
| ⑥ 云端浏览器        | Browserbase、Steel Browser、Cloudflare Browser Run                      | 托管并发调度、资源隔离、会话持久化、代理轮换、防封禁                                                                       | 云端账单；多租户安全隔离复杂                                               | 服务器端高并发、多实例任务                                                                       |
| ⑦ 专业数据采集      | Crawl4AI、Firecrawl                                                     | 针对大规模抓取、动态渲染、去噪、转 Markdown/JSON 做过优化，吞吐高、成本低                                                  | 遇到强交互或复杂登录拦截的网站不如通用浏览器                               | 海量公开网页抓取、知识库清洗                                                                     |
| ⑧ 底层标准          | WebMCP（Google、微软推动）                                              | 网站主动通过标准接口暴露能力，Agent 不用猜按钮位置                                                                         | 还在早期试验，普及还很远                                                   | 长期方向，暂时不能直接用                                                                         |
|                     |                                                                         |                                                                                                                            |                                                                            |                                                                                                  |
|                     |                                                                         |                                                                                                                            |                                                                            |                                                                                                  |
--cdp 是「直接拿你本人去办事」

> [!Info]
> 一句话说清楚

三种认证模式对比：

| 模式    | 原理                               | 适用场景               |
| :------ | :--------------------------------- | :--------------------- |
| 默认    | 无认证                             | 公开网页               |
| --debug | Cookie 快照注入到新 Chromium       | 标准 cookie 认证的网站 |
| --cdp   | 直连 debug Chrome 进程             | Google 服务等复杂认证  |

✻ Crunched for 3m 32s


# 🧩 Cues

# 🪞Notes


Browser Use CLI 值得研究吗？截至 2026 年 3 月 21 日，这套工具已经从“玩具 Demo”进化成可接入真实浏览器、云浏览器和 MCP 的浏览器自动化工作台        
                                                                                                                                                   
🚀 核心定位已经不只是“命令行点网页”                                                                                                                
它本质上是“持久化浏览器会话 + 云端 API 管理”的双栈 CLI。                                                                                           
官方 CLI 文档把能力拆成两层：一层是本地/远程浏览器控制，另一层是 Cloud API 管理。                                                                  
这意味着它既能 open、click、type，也能直接管云端任务、Profile、Session、代理和计费，不只是个薄壳脚本。                                             
                                                                                                                                                   
🔗 https://docs.browser-use.com/open-source/browser-use-cli                                                                                        
🔗 https://github.com/browser-use/browser-use                                                                                                      
                                                                                                                                                   
⚡ 它最实用的点是“浏览器常驻”，不是每次冷启动                                                                                                      
Browser Use CLI 的关键体验优势，在于把浏览器保活成一个后台 daemon。                                                                                
官方文档写得很明确：首次命令会拉起后台 daemon，后续命令通过 socket 通信，典型交互延迟约 50ms。                                                     
如果你平时用脚本式 Playwright，每次开进程、拉浏览器、再执行一步，Browser Use CLI 在“边看边操作”的迭代感上会明显更顺。                              
                                                                                                                                                   
🔗 https://docs.browser-use.com/open-source/browser-use-cli                                                                                        
                                                                                                                                                   
🛠️ 上手门槛不高，安装路径也比早期清晰很多                                                                                                          
现在最标准的安装路径已经收敛成“一行安装”或 uv 安装。                                                                                               
官方推荐的 macOS / Linux 安装命令是 curl -fsSL https://browser-use.com/cli/install.sh | bash；手动装则是 uv pip install browser-use 后再           
browser-use install。                                                                                                                              
装完先跑 browser-use doctor，再决定要不要 browser-use setup。这比很多 agent 工具“装完还得自己猜依赖”友好不少。                                     
                                                                                                                                                   
🔗 https://docs.browser-use.com/open-source/browser-use-cli                                                                                        
🔗 https://docs.browser-use.com/open-source/quickstart                                                                                             
                                                                                                                                                   
🧭 它强在三种浏览器模式：本地无头、真实 Chrome、云浏览器                                                                                           
同一套命令，既能连本地 Chromium，也能直接接管你已经登录过的 Chrome。                                                                               
官方支持默认 headless Chromium，也支持 --profile "Default" 直接复用本地 Chrome 登录态，还支持 --cdp-url / cloud connect 连远程浏览器。             
这点很关键，因为很多真实任务卡在登录态、Cookie、2FA、Cloudflare 和地区限制，Browser Use 的设计明显是朝“真实业务流程”靠的。                         
                                                                                                                                                   
🔗 https://docs.browser-use.com/open-source/browser-use-cli                                                                                        
🔗 https://docs.browser-use.com/open-source/customize/browser/authentication                                                                       
🔗 https://docs.browser-use.com/open-source/customize/browser/all-parameters                                                                       
                                                                                                                                                   
🤖 和 Coding Agent 的结合度很高，MCP 也是现成的                                                                                                    
如果你是 Claude Code / Cursor / Copilot 用户，它不是单独的工具，而是 agent 的浏览器手脚。                                                          
官方一方面提供 Skill 安装方式，另一方面提供 MCP Server。                                                                                           
本地可直接 uvx browser-use --mcp，云端则可接 https://api.browser-use.com/mcp。这意味着你既可以手敲 CLI，也可以把它挂进 agent                       
工作流，让模型调用浏览器工具链。                                                                                                                   
                                                                                                                                                   
🔗 https://docs.browser-use.com/open-source/browser-use-cli                                                                                        
🔗 https://docs.browser-use.com/customize/integrations/mcp-server                                                                                  
🔗 https://docs.browser-use.com/quickstart_llm                                                                                                     
                                                                                                                                                   
📈 热度和更新速度都很高，短期内还在快速演进                                                                                                        
截至 2026 年 3 月 21 日，browser-use/browser-use GitHub 星标约 81.5k，最新 release 是 0.12.3，发布时间是 2026 年 3 月 20 日。                      
这说明它不是“论文项目停更”，而是高频发布状态。                                                                                                     
从 release 记录看，最近还在持续补 CLI、模型支持、CDP、认证和文档体验，适合追新，但也意味着 API 和最佳实践可能继续变化。                            
                                                                                                                                                   
🔗 https://github.com/browser-use/browser-use                                                                                                      
🔗 https://github.com/browser-use/browser-use/releases                                                                                             
                                                                                                                                                   
⚠️ 真正的风险不在 CLI 本身，而在“高权限浏览器代理”这个范式                                                                                         
一旦接入真实登录态、邮箱、支付或后台系统，Browser Use 就不再是普通爬虫，而是高权限执行器。                                                         
官方文档已经给出防护建议：限制 allowed_domains、敏感页面关闭视觉输入、按域隔离凭据。                                                               
进一步说，这是我基于官方安全建议和近期 web-agent 安全论文做出的判断：Browser Use CLI 很适合自动化，但不适合“无隔离地替你操作所有敏感系统”。        
                                                                                                                                                   
🔗 https://docs.browser-use.com/open-source/customize/browser/authentication                                                                       
🔗 https://arxiv.org/abs/2504.18575                                                                                                                
🔗 https://arxiv.org/abs/2505.13076                                                                                                                
                                                                                                                                                   
🎯 一句落地判断：适合谁，不适合谁                                                                                                                  
适合“要把浏览器能力接到 agent/终端工作流里的人”；不适合“只想做稳定回归测试的人”。                                                                  
适合的场景：表单流转、后台运营、带登录态的信息检索、Agent 辅助调试、MCP 浏览器工具、云端任务编排。                                                 
不适合的场景：严格可重复的前端回归测试。这类任务 Playwright 原生脚本通常 still 更稳、更可审计。                                                    
                                                                                                                                                   
🔗 https://docs.browser-use.com/open-source/browser-use-cli                                                                                        
🔗 https://playwright.dev/docs/test-cli 这个讲了个啥                                                                                               

⏺ 这是一篇关于 Browser Use CLI 的技术调研