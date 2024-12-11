import{_ as n}from"./plugin-vue_export-helper-DlAUqK2U.js";import{c as a,f as i,o as e}from"./app-DyPYfXuN.js";const l={};function p(d,s){return e(),a("div",null,s[0]||(s[0]=[i(`<p>FastAPI 是一个可快速构建 API 服务的 Web 框架，可与 NodeJS 和 Go 比肩的极高性能（归功于 Starlette 和 Pydantic），是最快的 Python Web 框架之一。更多详情见官网 <a href="https://link.zhihu.com/?target=https%3A//fastapi.tiangolo.com/zh/" target="_blank" rel="noopener noreferrer">FastAPI官网地址</a></p><p>本文对 FastAPI 的开发部署以及生产环境部署做一个记录。</p><h2 id="开发部署" tabindex="-1"><a class="header-anchor" href="#开发部署"><span>开发部署</span></a></h2><p>安装 \`\`\`python3 if <strong>name</strong> == &#39;<strong>main</strong>&#39;: uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</p><div class="language-0 line-numbers-mode" data-highlighter="shiki" data-ext="0" data-title="0" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span></span></span>
<span class="line"><span>\`\`\`text</span></span>
<span class="line"><span>pip install uvicorn</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p>例：main.py</p><div class="language-python3 line-numbers-mode" data-highlighter="shiki" data-ext="python3" data-title="python3" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p>nvicorn main:app --reload 开发模式下运行 热加载</p><p>如果是想在 Pycharm 等 IDE 中直接运行，可以在代码中加入</p><div class="language-python3 line-numbers-mode" data-highlighter="shiki" data-ext="python3" data-title="python3" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>if __name__ == &#39;__main__&#39;:</span></span>
<span class="line"><span>    uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div></div></div><p>即可直接 run 运行，不需要使用命令行，但只适合用于开发环境。</p><h2 id="生产环境部署" tabindex="-1"><a class="header-anchor" href="#生产环境部署"><span>生产环境部署</span></a></h2><p>生产环境：CentOS7.x +Nginx</p><p>1.安装 Gunicorn</p><p>Gunicorn 是一个 unix 上被广泛使用的高性能的 Python WSGI UNIX HTTP Server，和大多数的 Web 框架兼容，并具有实现简单，轻量级，高性能等特点。</p><p>使用 gunicorn 启动应用程序的好处是，它可以处理大量的并发连接,，并且其使用的是预派生子进程的方式，这意味着它能够更好地利用多核 CPU。</p><p>安装命令</p><div class="language-text line-numbers-mode" data-highlighter="shiki" data-ext="text" data-title="text" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>pip install uvicorn</span></span>
<span class="line"><span>pip install gunicorn</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div></div></div><p>Shell 中执行 \`\`\`python3 if <strong>name</strong> == &#39;<strong>main</strong>&#39;: uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</p><div class="language-1 line-numbers-mode" data-highlighter="shiki" data-ext="1" data-title="1" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span></span></span>
<span class="line"><span>  </span></span>
<span class="line"><span>2.以配置文件方式启动应用</span></span>
<span class="line"><span></span></span>
<span class="line"><span>创建 gunicorn.py 文件，里面包含下列内容</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>import os</span></span>
<span class="line"><span></span></span>
<span class="line"><span># 设置守护进程</span></span>
<span class="line"><span>daemon=True</span></span>
<span class="line"><span># 监听内网端口8000</span></span>
<span class="line"><span>bind=&#39;0.0.0.0:8000&#39;</span></span>
<span class="line"><span># 设置进程文件目录</span></span>
<span class="line"><span>pidfile=&#39;./gunicorn.pid&#39;</span></span>
<span class="line"><span>chdir=&#39;./&#39; # 工作目录</span></span>
<span class="line"><span># 工作模式</span></span>
<span class="line"><span>worker_class=&#39;uvicorn.workers.UvicornWorker&#39;</span></span>
<span class="line"><span># 并行工作进程数 核心数*2+1个</span></span>
<span class="line"><span>workers=3  #multiprocessing.cpu_count()+1</span></span>
<span class="line"><span># 指定每个工作者的线程数</span></span>
<span class="line"><span>threads=2</span></span>
<span class="line"><span># 设置最大并发量</span></span>
<span class="line"><span>worker_connections = 2000</span></span>
<span class="line"><span>loglevel=&#39;debug&#39; # 错误日志的日志级别</span></span>
<span class="line"><span>access_log_format = &#39;%(t)s %(p)s %(h)s &quot;%(r)s&quot; %(s)s %(L)s %(b)s %(f)s&quot; &quot;%(a)s&quot;&#39;</span></span>
<span class="line"><span># 设置访问日志和错误信息日志路径</span></span>
<span class="line"><span>log_dir = &quot;./log&quot;</span></span>
<span class="line"><span>if not os.path.exists(log_dir):</span></span>
<span class="line"><span>    os.makedirs(log_dir)</span></span>
<span class="line"><span>accesslog = &quot;./log/gunicorn_access.log&quot;</span></span>
<span class="line"><span>errorlog = &quot;./log/gunicorn_error.log&quot;</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><h2 id="运行项目" tabindex="-1"><a class="header-anchor" href="#运行项目"><span>运行项目</span></a></h2><p>1.上面的 gunicorn.py 文件配置好后，使用如下命令：</p><div class="language-text line-numbers-mode" data-highlighter="shiki" data-ext="text" data-title="text" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>gunicorn main:app -c gunicorn.py</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div></div></div><p>在 gunicorn_error.log 文件中看到日志输出表示启动成功。</p><p>2.当然也可以不用配置文件，直接执行下面命令启动应用</p><div class="language-text line-numbers-mode" data-highlighter="shiki" data-ext="text" data-title="text" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>gunicorn main:app -b 0.0.0.0:8000 -w 4 -k uvicorn.workers.UvicornWorker --daemon</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div></div></div><p>这是一个启动命令，主要作用是使用 gunicorn 作为应用部署服务器，并指定服务器的启动参数：</p><ul><li><div class="language-python3 line-numbers-mode" data-highlighter="shiki" data-ext="python3" data-title="python3" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span></span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div></div></div></li></ul><p>if <strong>name</strong> == &#39;<strong>main</strong>&#39;: uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</p><div class="language-2 line-numbers-mode" data-highlighter="shiki" data-ext="2" data-title="2" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>- \`\`\`python3</span></span>
<span class="line"><span>if __name__ == &#39;__main__&#39;:</span></span>
<span class="line"><span>    uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</span></span>
<span class="line"><span>\`\`\`3 指定了服务器的 IP 地址和端口；</span></span>
<span class="line"><span>- \`\`\`python3</span></span>
<span class="line"><span>if __name__ == &#39;__main__&#39;:</span></span>
<span class="line"><span>    uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</span></span>
<span class="line"><span>\`\`\`4 指定了 Gunicorn 使用 4 个工作进程同时处理请求；</span></span>
<span class="line"><span>- \`\`\`python3</span></span>
<span class="line"><span>if __name__ == &#39;__main__&#39;:</span></span>
<span class="line"><span>    uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</span></span>
<span class="line"><span>\`\`\`5 指定使用 UvicornWorker 作为工作进程的类型；</span></span>
<span class="line"><span>- \`\`\`python3</span></span>
<span class="line"><span>if __name__ == &#39;__main__&#39;:</span></span>
<span class="line"><span>    uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</span></span>
<span class="line"><span>\`\`\`6 参数表示将服务器以守护进程 (daemon) 模式启动（后台运行）。</span></span>
<span class="line"><span></span></span>
<span class="line"><span>## 停止项目</span></span>
<span class="line"><span></span></span>
<span class="line"><span>如果想要停止 Gunicorn 项目，可以先通过查看进程树找到进程 pid，然后使用 kill 命令结束进程</span></span>
<span class="line"><span></span></span>
<span class="line"><span>1.获取 gunicorn 进程树</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`text</span></span>
<span class="line"><span>pstree -ap | grep gunicorn</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div><p>2.终止 gunicorn 任务</p><div class="language-text line-numbers-mode" data-highlighter="shiki" data-ext="text" data-title="text" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>kill -HUP 进程pid</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div></div></div><p>3.如果使用了多进程，那么执行了上述命令后还会有子进程在运行，可以使用如下命令杀死</p><div class="language-text line-numbers-mode" data-highlighter="shiki" data-ext="text" data-title="text" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>kill -9 进程pid</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div></div></div><p>当然，这样一个一个关闭太繁琐了，我们可以编写一个 Shell 脚本，来实现一键杀死所有主进程和子进程的功能。</p><p>脚本示例：</p><div class="language-python3 line-numbers-mode" data-highlighter="shiki" data-ext="python3" data-title="python3" style="--shiki-light:#383A42;--shiki-dark:#abb2bf;--shiki-light-bg:#FAFAFA;--shiki-dark-bg:#282c34;"><pre class="shiki shiki-themes one-light one-dark-pro vp-code"><code><span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`0</span></span>
<span class="line"><span></span></span>
<span class="line"><span>这个脚本可以完成以下操作：</span></span>
<span class="line"><span></span></span>
<span class="line"><span>1. 查找 gunicorn 主进程 PID</span></span>
<span class="line"><span>2. 给主进程发送 SIGINT 信号请求正常关闭</span></span>
<span class="line"><span>3. 睡眠 5 秒等待主进程结束</span></span>
<span class="line"><span>4. 使用 \`\`\`python3</span></span>
<span class="line"><span>if __name__ == &#39;__main__&#39;:</span></span>
<span class="line"><span>    uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</span></span>
<span class="line"><span>\`\`\`7 命令查找所有 gunicorn 子进程 PID</span></span>
<span class="line"><span>5. 使用 \`\`\`python3</span></span>
<span class="line"><span>if __name__ == &#39;__main__&#39;:</span></span>
<span class="line"><span>    uvicorn.run(&quot;main:app&quot;, host=&quot;127.0.0.1&quot;, port=8000, log_level=&quot;info&quot;)</span></span>
<span class="line"><span>\`\`\`8 命令杀死所有子进程</span></span>
<span class="line"><span></span></span>
<span class="line"><span>您只需要将以上代码保存到一个文件中 (例如 stop_gunicorn.sh)，并确保该文件有执行权限，然后在命令行界面运行该脚本，即可一次性停止 gunicorn 进程和所有子进程：</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`1</span></span>
<span class="line"><span></span></span>
<span class="line"><span>## 配置开机自启 (可选）</span></span>
<span class="line"><span></span></span>
<span class="line"><span>配置 gunicorn.service 服务开机自启动</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`2</span></span>
<span class="line"><span></span></span>
<span class="line"><span>依次执行下面命令</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`3</span></span>
<span class="line"><span></span></span>
<span class="line"><span>查看服务状态</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`4</span></span>
<span class="line"><span></span></span>
<span class="line"><span>## 配置 Nginx 代理访问 (可选）</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`5</span></span>
<span class="line"><span></span></span>
<span class="line"><span>运行 nginx -s reload，至此就完成了 FastAPI 的生产部署！</span></span>
<span class="line"><span></span></span>
<span class="line"><span>## **docs/redoc**文档显示空白问题</span></span>
<span class="line"><span></span></span>
<span class="line"><span>有时候我们用不同设备或者浏览器会出现 docs/redoc 文档空白不能显示的问题，这是因为模板文件用的是外国的 cdn，所以国内访问会比较慢，出现访问失败的情况，导致显示空白。</span></span>
<span class="line"><span></span></span>
<span class="line"><span>解决办法：</span></span>
<span class="line"><span></span></span>
<span class="line"><span>修改 FastAPI 源文件，把模板文件保存到本地，然后修改文件路径让其从本地访问。</span></span>
<span class="line"><span></span></span>
<span class="line"><span>1.首先修改源文件，模板文件路径如下：</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`6</span></span>
<span class="line"><span></span></span>
<span class="line"><span>把 docs.py 文件里的下面这三行</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`7</span></span>
<span class="line"><span></span></span>
<span class="line"><span>修改为</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`8</span></span>
<span class="line"><span></span></span>
<span class="line"><span>2.修改我们的项目代码，添加下面这段代码</span></span>
<span class="line"><span></span></span>
<span class="line"><span>\`\`\`python3</span></span>
<span class="line"><span>from fastapi import FastApi</span></span>
<span class="line"><span></span></span>
<span class="line"><span>app = FastApi()</span></span>
<span class="line"><span></span></span>
<span class="line"><span></span></span>
<span class="line"><span>@app.get(&#39;/hello&#39;)</span></span>
<span class="line"><span>async def hello():</span></span>
<span class="line"><span>    return {&#39;message&#39;: &#39;hello World&#39;}</span></span>
<span class="line"><span>\`\`\`9</span></span>
<span class="line"><span></span></span>
<span class="line"><span>注意：directory 路径需要根据自己的实际情况设置。</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  </span></span>
<span class="line"><span></span></span>
<span class="line"><span>3.把我们的 static 模板文件放置到项目根目录下，静态文件我放网盘了 [百度云文件](https://link.zhihu.com/?target=https%3A//pan.baidu.com/s/1nWFSSKtCHA0QEkhEiJTh1w%3Fpwd%3D0516)，有需要可自行下载。</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  </span></span>
<span class="line"><span></span></span>
<span class="line"><span>最后重新运行我们的项目，便大功告成！</span></span>
<span class="line"><span></span></span>
<span class="line"><span>  </span></span>
<span class="line"><span></span></span>
<span class="line"><span>  </span></span>
<span class="line"><span>2022.11.06  </span></span>
<span class="line"><span>如需转载请详细注明来源</span></span></code></pre><div class="line-numbers" aria-hidden="true" style="counter-reset:line-number 0;"><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div><div class="line-number"></div></div></div>`,37)]))}const t=n(l,[["render",p],["__file","FastAPi部署.html.vue"]]),v=JSON.parse('{"path":"/2%20%E7%AC%AC%E4%BA%8C%E5%A4%A7%E8%84%91/1%20%E8%8A%82%E7%82%B9/CS/%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F/linux/FastAPi%E9%83%A8%E7%BD%B2.html","title":"FastAPi部署","lang":"zh-CN","frontmatter":{"draw":null,"tags":[],"title":"FastAPi部署","date created":"2023-11-22T00:00:00.000Z","date modified":"2024-11-12T00:00:00.000Z","description":"FastAPI 是一个可快速构建 API 服务的 Web 框架，可与 NodeJS 和 Go 比肩的极高性能（归功于 Starlette 和 Pydantic），是最快的 Python Web 框架之一。更多详情见官网 FastAPI官网地址 本文对 FastAPI 的开发部署以及生产环境部署做一个记录。 开发部署 安装 ```python3 if n...","head":[["meta",{"property":"og:url","content":"https://vuepress-theme-hope-docs-demo.netlify.app/2%20%E7%AC%AC%E4%BA%8C%E5%A4%A7%E8%84%91/1%20%E8%8A%82%E7%82%B9/CS/%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F/linux/FastAPi%E9%83%A8%E7%BD%B2.html"}],["meta",{"property":"og:site_name","content":"转了码的刘公子"}],["meta",{"property":"og:title","content":"FastAPi部署"}],["meta",{"property":"og:description","content":"FastAPI 是一个可快速构建 API 服务的 Web 框架，可与 NodeJS 和 Go 比肩的极高性能（归功于 Starlette 和 Pydantic），是最快的 Python Web 框架之一。更多详情见官网 FastAPI官网地址 本文对 FastAPI 的开发部署以及生产环境部署做一个记录。 开发部署 安装 ```python3 if n..."}],["meta",{"property":"og:type","content":"article"}],["meta",{"property":"og:locale","content":"zh-CN"}],["meta",{"property":"og:updated_time","content":"2024-11-24T17:46:49.000Z"}],["meta",{"property":"article:modified_time","content":"2024-11-24T17:46:49.000Z"}],["script",{"type":"application/ld+json"},"{\\"@context\\":\\"https://schema.org\\",\\"@type\\":\\"Article\\",\\"headline\\":\\"FastAPi部署\\",\\"image\\":[\\"\\"],\\"dateModified\\":\\"2024-11-24T17:46:49.000Z\\",\\"author\\":[{\\"@type\\":\\"Person\\",\\"name\\":\\"转了码的刘公子\\",\\"url\\":\\"https://mister-hope.com\\"}]}"]]},"headers":[{"level":2,"title":"开发部署","slug":"开发部署","link":"#开发部署","children":[]},{"level":2,"title":"生产环境部署","slug":"生产环境部署","link":"#生产环境部署","children":[]},{"level":2,"title":"运行项目","slug":"运行项目","link":"#运行项目","children":[]}],"git":{"createdTime":1732465042000,"updatedTime":1732470409000,"contributors":[{"name":"Luis","email":"liuysh20@gmail.com","commits":2}]},"readingTime":{"minutes":5.38,"words":1614},"filePathRelative":"2 第二大脑/1 节点/CS/操作系统/linux/FastAPi部署.md","localizedDate":"2024年11月25日","autoDesc":true}');export{t as comp,v as data};
