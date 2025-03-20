---
draw:
title: rime输入法
date created: 2025-03-14
date modified: 2025-03-14
---

```Java
/Library/Input\ Methods/Squirrel.app/Contents/MacOS/rime_dict_manager
options:
	-l|--list
	-s|--sync
	-b|--backup dict_name
	-r|--restore xxx.userdb.txt
	-e|--export dict_name export.txt
	-i|--import dict_name import.txt
```

"雾凇拼音"与"鼠须管"都是基于 Rime 输入法引擎的产品，但它们扮演着不同的角色：

- **鼠须管**：
    这是 Rime 输入法在 macOS 上的一个客户端，实现了图形界面和相关交互。也就是说，鼠须管是一个运行 Rime 引擎的输入法应用程序。
    
- **雾凇拼音**：
    这是一套 Rime 的输入方案（schema），包含了具体的词库、编码规则和候选词排序等配置。您可以把它看作是 Rime 引擎中的一种配置风格，用来决定拼音输入时的候选词和使用体验。
    

**关系**：
在鼠须管中，您可以选择使用不同的输入方案，如"雾凇拼音"、"朙月拼音"等。也就是说，雾凇拼音是鼠须管（以及其他 Rime 客户端）可选的配置之一。选择雾凇拼音后，您会获得该方案特有的输入体验和词库，但输入法的底层仍然由鼠须管（Rime 引擎）提供支持。

---


- **背景与初衷**  
    佛振自幼对符号和编程产生浓厚兴趣，大学毕业后边工作边探索速录机键盘原理，最早推出《宫保拼音》。他希望为输入法爱好者打造一个智能、灵活且高度可定制的平台，从而萌生了研发Rime输入法的想法。
    
- **项目发展与开源实践**  
    自2009年开始设计输入法，佛振在2008年离职后全力投入研发，并在2010年加入百度工作。在百度工作期间，他利用大公司的严谨流程和丰富资源开发百度PC输入法，同时利用休息时间推动Rime的跨平台版本（Windows版"小狼毫"、Mac版"鼠须管"、Linux版"中州韵"）。Rime不仅仅是一款输入法，更是一个支持多种编码（拼音、双拼、注音、五笔、仓颉等）和多种语言、方言的输入法算法框架。
    
- **技术与设计理念**  
    为了解决不同语音系统的差异，佛振设计了通用的音节切分算法和基于正则表达式的"拼写运算"，使输入法能处理简拼、模糊音、双拼等多种变体。他强调代码如同文学作品，需要追求艺术表现力，简洁明了却功能强大。
    
- **团队合作与社区贡献**  
    在百度团队中，他学习了严谨的测试流程和自动化构建的重要性，也将这些经验反哺到开源项目中。Rime的开发过程中吸收了许多第三方开源库，同时也得到广大用户和开发者的补丁、反馈和新方案支持，形成了一个活跃的开源生态。
    
- **未来展望**  
    佛振计划继续优化Rime核心算法，尤其针对移动平台进行改进，并完善API、文档及辅助工具，让更多开发者能够轻松构建和定制各类输入法，推动Rime在开源领域取得更大成功。
    

这篇采访不仅展现了一个技术匠人对输入法设计的深刻思考，也体现了他对技术与艺术、团队协作与开源精神的独到理解。

---

我将研究如何在 macOS 上使用 Rime 鼠须管记录所有输入的中文文本，并提供一个可以手动导出的方案。这可能涉及 Rime 的日志功能、Lua 插件或外部脚本方法。我会整理出最合适的方法，并告诉您如何配置和使用它。请稍等，我会尽快回复您。

## 利用 Rime 鼠须管记录所有输入的中文文本方案

### 使用 Rime 自带日志功能 (有限方案)

Rime 输入法引擎本身具备日志记录功能，但默认情况下鼠须管不会主动输出详细日志 ([鼠须管输入法配置 – 哈呜.王](https://www.hawu.me/others/2666#:~:text=,%E6%97%A9%E6%9C%9F%E7%89%88%E6%9C%AC%C2%A0%60%E7%94%A8%E6%88%B7%E9%85%8D%E7%BD%AE%E7%9B%AE%E5%BD%95%2Frime.log))。Rime 在 macOS 下的日志文件位于 `$TMPDIR/rime.squirrel.INFO` ([鼠须管输入法配置 – 哈呜.王](https://www.hawu.me/others/2666#:~:text=,%E6%97%A9%E6%9C%9F%E7%89%88%E6%9C%AC%C2%A0%60%E7%94%A8%E6%88%B7%E9%85%8D%E7%BD%AE%E7%9B%AE%E5%BD%95%2Frime.log))。如果开启调试日志，里面**可能**包含输入法处理的相关信息（包括上屏文本）([ENABLE_LOGGING 不应该默认开启 · Issue #254 · rime/librime · GitHub](https://github.com/rime/librime/issues/254#:~:text=%E6%84%8F%E5%A4%96%E5%8F%91%E7%8E%B0%20%2Ftmp%20%E4%B8%8B%E6%9C%89%E4%B8%80%E4%B8%AA%E5%BE%88%E5%A4%A7%EF%BC%88200M%2B%EF%BC%89%E7%9A%84%E6%97%A5%E5%BF%97%E6%96%87%E4%BB%B6%20%2Ftmp%2Frime.fcitx,rime%20%E9%A1%B9%E7%9B%AE%E6%9B%BE%E6%9C%89%E4%BA%BA%E6%8A%A5%E5%91%8A%E8%BF%87))。然而，官方发行版一般关闭了详细日志（出于性能和隐私考虑），普通用户无法直接通过配置打开**完整键入内容**的记录 ([ENABLE_LOGGING 不应该默认开启 · Issue #254 · rime/librime · GitHub](https://github.com/rime/librime/issues/254#:~:text=%E5%85%B6%E5%AE%83%E5%B9%B3%E5%8F%B0%E4%B9%9F%E6%B2%A1%E6%9C%89%E5%9C%A8%20release%20%E4%B8%8B%E8%BE%93%E5%87%BA%20debug%20%E6%97%A5%E5%BF%97%E7%9A%84%E9%97%AE%E9%A2%98%EF%BC%8C%E8%99%BD%E7%84%B6%E4%B8%8D%E6%B8%85%E6%A5%9A,build%20%E4%BA%A7%E7%94%9F%E7%9A%84%20release%20%E5%BA%93%E4%B9%9F%E6%B2%A1%E6%9C%89%E8%BF%87%E9%97%AE%E9%A2%98%E3%80%82%20%E5%BB%BA%E8%AE%AE%E5%8F%8D%E9%A6%88%E5%8F%91%E8%A1%8C%E7%89%88%E3%80%82))。除非自行编译启用日志或使用特殊环境变量，否则此方法难以实现持续记录。同时，大量日志数据会迅速膨胀，占用存储 ([ENABLE_LOGGING 不应该默认开启 · Issue #254 · rime/librime · GitHub](https://github.com/rime/librime/issues/254#:~:text=%E6%84%8F%E5%A4%96%E5%8F%91%E7%8E%B0%20%2Ftmp%20%E4%B8%8B%E6%9C%89%E4%B8%80%E4%B8%AA%E5%BE%88%E5%A4%A7%EF%BC%88200M%2B%EF%BC%89%E7%9A%84%E6%97%A5%E5%BF%97%E6%96%87%E4%BB%B6%20%2Ftmp%2Frime.fcitx,rime%20%E9%A1%B9%E7%9B%AE%E6%9B%BE%E6%9C%89%E4%BA%BA%E6%8A%A5%E5%91%8A%E8%BF%87))。因此，不建议将 Rime 日志作为长期记录方案，仅适合作为调试参考。若尝试此法，可在强制重启输入法后检查上述日志文件并手动提取中文文本，但操作繁琐，不便于手动导出整理。

### 通过用户词典记录输入文本

Rime 默认会通过**用户词典**来学习用户输入的新词和调整词频。也就是说，用户输入过的内容会记录在用户词典数据库中 ([Rime 配置：雾凇拼音 - Dvel's Blog](https://dvel.me/posts/rime-ice/#:~:text=))。在默认配置下，用户词典 (`enable_user_dict`) 已开启，存储为二进制格式 (`db_class: userdb`) ([Rime 配置：雾凇拼音 - Dvel's Blog](https://dvel.me/posts/rime-ice/#:~:text=))。要方便地导出查看输入过的中文文本，可将用户词典改为**文本格式**。具体步骤：

1. **修改配置开启文本词典**：打开鼠须管的用户配置目录（通常为`~/Library/Rime/`，可通过菜单"用户设置"打开 ([Configuring Rime Input Method on macOS | Bdim](https://blog.bdim.moe/posts/configuring-rime-input-method-on-macos/#:~:text=In%20macOS%2C%20Rime%E2%80%99s%20configuration%20path,into%20this%20folder))）。新建或编辑对应输入方案的自定义配置文件，如使用朙月拼音则编辑`luna_pinyin.custom.yaml`。在文件中添加如下内容，将用户词典切换为文本模式：

    ```yaml
    patch:
      translator:
        enable_user_dict: true        # 确保启用用户词典（默认已启用）
        db_class: tabledb             # 将用户词典格式改为文本 (tabledb)
    ```

    上述配置开启用户词典并指定格式为`tabledb`（文本），替代默认的`userdb`二进制格式 ([超全超详细Rime中州韵输入法配置指南_rime输入法的玩法-CSDN博客](https://blog.csdn.net/qq_43108090/article/details/122759647#:~:text=db_class%3A%20tabledb%20%20,%E5%AD%97%E7%AC%A6%E9%9B%86%E8%BF%87%E6%BB%A4%EF%BC%8C%E4%BD%8E%E9%87%8D%E5%BD%A2%E7%A0%81%E7%94%A8%E4%B8%8D%E7%9D%80))。保存文件后，重新部署输入法（在菜单中选择"重新部署"以应用配置）。

    
2. **查找导出文本文件**：部署成功后，Rime 会将用户词典以**可读文本**形式保存在用户配置目录下。例如，对于朙月拼音方案，会生成类似`luna_pinyin.userdb.txt`的文件（或直接在`luna_pinyin.tabledb`目录下生成文本文件）。该文件包含用户上屏过的词条。文本格式每行通常由"词语\t编码\t频次"三部分组成 ([Rime 那些事儿](https://aituyaa.com/rime-%E9%82%A3%E4%BA%9B%E4%BA%8B%E5%84%BF/#:~:text=%E5%B0%86%E7%94%A8%E6%88%B7%E8%AF%8D%E5%85%B8%E5%AF%BC%E5%87%BA%E4%B8%BA%E6%96%87%E6%9C%AC%E7%A0%81%E8%A1%A8%EF%BC%8C%E4%BE%BF%E6%96%BC%E7%9B%B4%E6%8E%A5%E9%98%85%E8%AF%BB%EF%BC%8C%E6%88%96%E6%89%B9%E9%87%8F%E6%B7%BB%E5%8A%A0%E8%AF%8D%E6%9D%A1%E5%88%B0%20))。您可以用文本编辑器打开此文件，提取所有上屏的中文词语列。如需定期保存，可在重新部署或执行同步后获取最新内容；Rime 也会定期自动备份用户词典为`.userdb.txt`快照 ([Rime 那些事儿](https://aituyaa.com/rime-%E9%82%A3%E4%BA%9B%E4%BA%8B%E5%84%BF/#:~:text=%E2%9D%B7%20%E5%A4%87%E4%BB%BD%E5%8F%8A%E5%90%88%E4%BD%B5%E8%AF%8D%E5%85%B8%E5%BF%AB%E7%85%A7))。
    
3. **手动导出整理**：由于文本词典包含编码和频率信息，若只需纯中文文本，可用脚本过滤每行的第一列。例如，用 shell 命令提取词列：

    ```bash
    cut -f1 luna_pinyin.userdb.txt > my_input_words.txt
    ```

    这样即可得到所有记录的中文词语列表。需要注意，用户词典记录以**词/短语**为单位，可能不保留重复次数（频次会累计在一起）且不保证原始顺序。它更适合用于统计常用词或调整词频 ([Rime 配置：雾凇拼音 - Dvel's Blog](https://dvel.me/posts/rime-ice/#:~:text=))。如果要求**逐次、完整**地记录每一次输入，还需要借助 Lua 插件方案。

    

### 使用 Lua 插件实时记录输入 (推荐方案)

鼠须管从 0.12.0 版本开始内置了 **librime-lua** 插件 ([鼠须管输入法配置 – 哈呜.王](https://www.hawu.me/others/2666#:~:text=%E8%BF%9B%E9%98%B6%EF%BC%9ALua%20%E6%89%A9%E5%B1%95%E8%84%9A%E6%9C%AC))。这允许用户通过编写 Lua 脚本定制输入法的处理流程，包括在每次**上屏**时执行自定义操作 ([鼠须管输入法配置 – 哈呜.王](https://www.hawu.me/others/2666#:~:text=%E8%BF%9B%E9%98%B6%EF%BC%9ALua%20%E6%89%A9%E5%B1%95%E8%84%9A%E6%9C%AC))。我们可以利用 Lua 插件捕获每次上屏的文本并写入日志文件，从而完整记录所有输入的中文内容。

**实现思路**：借助 Rime 暴露的 `commit_notifier` 接口，在每次用户确认上屏时触发回调，获取上屏文本并追加保存到本地文件 ([能获取前一次上屏的内容吗？ · Issue #23 · hchunhui/librime-lua · GitHub](https://github.com/hchunhui/librime-lua/issues/23#:~:text=,%E7%9A%84%E4%B8%80%E4%B8%AA%E6%88%90%E5%91%98%E4%B8%AD%E6%9D%A5%E5%AE%9E%E7%8E%B0%EF%BC%89%EF%BC%8C%E5%90%A6%E5%88%99%20lua%20%E5%9C%A8%20gc%20%E7%9A%84%E6%97%B6%E5%80%99%E5%9B%9E%E8%B0%83%E4%BC%9A%E6%96%AD%E6%8E%89%E3%80%82))。具体实现步骤如下：

1. **编写 Lua 脚本**：在用户配置目录下新建文件`rime.lua`（此路径为 Rime 默认加载的 Lua 脚本入口 ([鼠须管输入法配置 – 哈呜.王](https://www.hawu.me/others/2666#:~:text=match%20at%20L847%20Rime%20%E9%BB%98%E8%AE%A4%E5%8A%A0%E8%BD%BD,%E3%80%82))）。编写一个 Lua 模块（处理器）监听上屏事件。示例如下：

    ```lua
    -- rime.lua
    local commit_logger = {}                     -- 定义模块表
    function commit_logger.init(env)             -- 初始化时执行
        env.logged = env.engine.context.commit_notifier:connect(function(ctx)
            local text = ctx:get_commit_text()   -- 获取上屏文本 ([如何进行输入统计？ · Issue #90 · rime/librime · GitHub](https://github.com/rime/librime/issues/90#:~:text=local%20function%20on_commit,time))
            -- 打开日志文件（位于用户目录），追加写入本次上屏内容和时间戳
            local log_path = env.engine.schema.config:get_string("commit_log_path")
                            or (os.getenv("HOME").."/Library/Rime/input_log.txt")
            local f = io.open(log_path, "a")
            if f then
                f:write(os.date("%Y-%m-%d %H:%M:%S").." "<<span style="color:blue"></span> text .. "\n")
                f:close()
            end
        end)
    end
    function commit_logger.fini(env)             -- 输入法卸载时执行
        if env.logged then env.logged:disconnect() end
    end
    function commit_logger.func(key_event, env)  -- 处理器主函数
        return 2  -- kNoop, 不截断事件，继续让后续组件处理 ([如何进行输入统计？ · Issue #90 · rime/librime · GitHub](https://github.com/rime/librime/issues/90#:~:text=function%20Module,kNoop%20end))
    end
    return commit_logger
    ```

    **说明**：上述脚本定义了一个名为`commit_logger`的 Lua **处理器**模块。在`init`阶段，它通过`context.commit_notifier:connect()`注册了一个回调函数，每当有文本上屏(commit)时触发 ([能获取前一次上屏的内容吗？ · Issue #23 · hchunhui/librime-lua · GitHub](https://github.com/hchunhui/librime-lua/issues/23#:~:text=,%E7%9A%84%E4%B8%80%E4%B8%AA%E6%88%90%E5%91%98%E4%B8%AD%E6%9D%A5%E5%AE%9E%E7%8E%B0%EF%BC%89%EF%BC%8C%E5%90%A6%E5%88%99%20lua%20%E5%9C%A8%20gc%20%E7%9A%84%E6%97%B6%E5%80%99%E5%9B%9E%E8%B0%83%E4%BC%9A%E6%96%AD%E6%8E%89%E3%80%82))。回调中使用`ctx:get_commit_text()`获取上屏的中文文本 ([如何进行输入统计？ · Issue #90 · rime/librime · GitHub](https://github.com/rime/librime/issues/90#:~:text=local%20function%20on_commit,time))，并附加当前时间戳写入指定的日志文件（这里默认为用户目录下`input_log.txt`，也可通过在配置中设置`commit_log_path`自定义路径）。`fini`阶段确保在输入法卸载时断开连接，避免资源泄露。`func`函数返回`2`表示不拦截按键（即让输入流程正常进行）([如何进行输入统计？ · Issue #90 · rime/librime · GitHub](https://github.com/rime/librime/issues/90#:~:text=function%20Module,kNoop%20end))。请确保脚本文件保存为 UTF-8 编码。
    
2. **加载 Lua 模块**：编辑输入方案配置，使上述 Lua 处理器生效。可以在全局`default.custom.yaml`中为所有方案添加处理器，或在特定方案的自定义配置中加入。以朙月拼音为例，打开`luna_pinyin.custom.yaml`，添加引擎处理器设置：

    ```yaml
    patch:
      engine:
        processors:
          - lua_processor@commit_logger   # 插入自定义Lua处理器
          - ascii_composer
          - recognizer
          - ...  (后续列出该方案原有的处理器组件)
    ```

    将`lua_processor@commit_logger`放在处理器列表开头，以便优先初始化监听器 ([如何进行输入统计？ · Issue #90 · rime/librime · GitHub](https://github.com/rime/librime/issues/90#:~:text=%E8%BF%99%E4%B8%AA%20processor%20%E9%A1%BB%E6%94%BE%E5%9C%A8%20processor%20%E5%88%97%E8%A1%A8%E8%BE%83%E5%BC%80%E5%A4%B4%E7%9A%84%E4%BD%8D%E7%BD%AE%E3%80%82))。后面的`ascii_composer`等为原方案默认的处理器，需一并保留，以确保输入法正常工作（可从原始`schema.yaml`中复制processors列表并在首位加入一行）。保存配置后，重新部署输入法。

    
3. **验证效果**：重新部署并切换到鼠须管输入中文。每输入并上屏一个词或字，在用户目录下都会看到`input_log.txt`不断增长。打开该文件，即可看到按时间顺序记录的所有上屏中文文本，以及时间戳。这样就实现了对所有输入中文的完整日志记录。**导出**时，直接使用该文本文件即可（手动复制或备份）。此方案数据完全保存在本地，可随时查看或导出，无需特殊工具。
    

**优点**：Lua 插件法能够**逐条**记录每次上屏内容，包括重复输入，保留顺序和时间信息，满足全面记录的需求。而且配置生效后自动记录，无需手动干预。
**注意**：日志文件可能包含敏感信息，请妥善保管，不要同步到不安全的位置。若不再需要记录，可移除配置并删除日志文件。

### 使用外部脚本收集输入内容 (可选方案)

除了利用 Rime 自身功能外，也可以借助系统脚本或第三方工具捕获输入内容。然而，此类方法往往更复杂，有一定风险，并非最佳选择：

- **利用 Rime 数据文件**：可以编写Shell或Python脚本定期读取Rime用户词典文本（如上述`*.userdb.txt`或Lua日志文件），将其中的中文词条汇总整理。例如使用 Python 脚本扫描新增行并存储到自定义文件或数据库。这相当于对Rime已记录的数据做二次处理，工作量较小且不会破坏输入法。用户只需在需要导出时运行脚本收集整理即可。
    
- **UI 自动化截取文本**：通过AppleScript或其他macOS辅助功能，可以监视当前输入窗格的文本内容变化，从而推断用户新输入的内容。这类似于Windows下网友提到的用 AutoHotkey 监听光标位置和文本变化的方法 ([有什么办法可以记录自己所有输入过的文本信息？ - 问题求助 - 小众软件官方论坛](https://meta.appinn.net/t/topic/38984#:~:text=%E6%88%91%E6%8F%90%E4%BE%9B%E4%B8%80%E4%B8%AAautohotkey%E6%96%B9%E6%A1%88%EF%BC%9A%20%E9%80%9A%E8%BF%87%E8%BF%99%E4%B8%AA%E5%B8%96%E5%AD%90%E9%87%8C%E7%9A%84%E5%87%A0%E7%A7%8D%E6%96%B9%E6%B3%95%EF%BC%8C%E7%9B%91%E5%90%AC%E8%BE%93%E5%85%A5%E5%85%89%E6%A0%87%E7%9A%84%E4%BD%8D%E7%BD%AE%EF%BC%9A%20%E5%A6%82%E4%BD%95%E8%8E%B7%E5%8F%96Windows%E5%90%84%E7%B1%BB%E8%BD%AF%E4%BB%B6%E7%9A%84%E2%80%9C%E6%8F%92%E5%85%A5%2F%E8%BE%93%E5%85%A5%E5%85%89%E6%A0%87%E3%80%96caret%E3%80%97%E2%80%9D%E7%9A%84x%E3%80%81y%E5%9D%90%E6%A0%87%EF%BC%9F%20,net))。具体做法是在短时间间隔内获取前台窗口中光标所在控件的文本，对比前后差异来捕获新输入。然而，此方式实现复杂，通用性差，而且频繁轮询可能影响性能。更重要的是，出于安全考虑，这种**模拟键盘监听**的行为可能被视为恶意（键盘记录器），导致安全警告 ([有什么办法可以记录自己所有输入过的文本信息？ - 问题求助 - 小众软件官方论坛](https://meta.appinn.net/t/topic/38984#:~:text=%E6%88%96%E8%80%85%E5%8F%AA%E8%AE%B0%E5%BD%95%E6%B1%89%E5%AD%97%E4%B9%9F%E8%A1%8C%E3%80%82%20%E4%B8%BB%E8%A6%81%E6%98%AF%E6%83%B3%E6%A0%B9%E6%8D%AE%E8%87%AA%E5%B7%B1%E7%9A%84%E8%BE%93%E5%85%A5%E8%B0%83%E6%95%B4%E8%AF%8D%E9%A2%91%EF%BC%8C%E7%94%A8%E7%9A%84%E8%BE%93%E5%85%A5%E6%B3%95%E6%98%AF%E5%B0%8F%E7%8B%BC%E6%AF%AB%EF%BC%88RIME%EF%BC%89%E6%8C%82%E8%BD%BD%E6%98%9F%E7%A9%BA%E9%94%AE%E9%81%936%E3%80%82%20%E6%83%B3%E4%B8%80%E6%83%B3%E7%AC%AC%E4%B8%89%E6%96%B9%E8%BD%AF%E4%BB%B6%E5%A4%A7%E6%A6%82%E4%BC%9A%E5%9B%A0%E4%B8%BA%E8%AE%B0%E5%BD%95%E6%95%8F%E6%84%9F%E8%BE%93%E5%85%A5%E8%80%8C%E8%A2%AB%E5%88%A4%E5%AE%9A%E4%B8%BA%E7%97%85%E6%AF%92%EF%BC%9F%20%E5%8F%AA%E8%83%BD%E8%87%AA%E5%B7%B1%E6%89%8B%E6%92%B8%EF%BC%9F))。因此不太推荐，除非有特殊需求且了解系统辅助权限设置。
    
- **键盘事件捕获**：利用macOS的事件tap接口编写底层监听程序，截获所有键盘按键，再自行按照输入法逻辑组词。这几乎需要构造一个简易"键盘记录器"，不仅开发量大，而且由于Rime有自己的输入编辑上下文，外部纯按键流未必能正确复原中文上屏结果（尤其涉及拼音组合与选词）。故这种方案并不实际。
    

综上，外部脚本方案在**可行性**和**安全性**上都不如直接使用Rime提供的扩展接口。若仅为调整词频或保存输入记录，优先考虑前述用户词典或Lua日志方法，而非从操作系统层面截获输入。

### 方案总结与导出

在 macOS 上记录所有中文输入文本，**推荐使用 Rime 内置机制**实现：要么启用用户词典的文本导出功能，要么使用Lua插件捕获上屏事件。前者（文本用户词典）配置简洁，能够累积用户输入的**词汇和频率** ([Rime 配置：雾凇拼音 - Dvel's Blog](https://dvel.me/posts/rime-ice/#:~:text=))，适合定期手动导出关注高频词；后者（Lua 日志）则详尽记录**每次输入**，包括时间和顺序，适合完整保存输入文本日志。两种方案都可在本地生成可手动获取的文本文件，确保数据安全可控。

**实施步骤概览**：先修改 Rime 配置（YAML 文件）启用所需功能，然后重新部署鼠须管使配置生效，最后从用户目录提取产生的文本记录文件。无论选择哪种方案，整个过程均在本机完成，不依赖云端。根据需求选择合适的方法，即可方便地手动导出所有输入过的中文文本记录。

**参考资料：**

- Rime 官方文档及配置指南 ([Rime 配置：雾凇拼音 - Dvel's Blog](https://dvel.me/posts/rime-ice/#:~:text=)) ([超全超详细Rime中州韵输入法配置指南_rime输入法的玩法-CSDN博客](https://blog.csdn.net/qq_43108090/article/details/122759647#:~:text=db_class%3A%20tabledb%20%20,%E5%AD%97%E7%AC%A6%E9%9B%86%E8%BF%87%E6%BB%A4%EF%BC%8C%E4%BD%8E%E9%87%8D%E5%BD%A2%E7%A0%81%E7%94%A8%E4%B8%8D%E7%9D%80))
- 鼠须管日志和用户词典机制说明 ([鼠须管输入法配置 – 哈呜.王](https://www.hawu.me/others/2666#:~:text=,%E6%97%A9%E6%9C%9F%E7%89%88%E6%9C%AC%C2%A0%60%E7%94%A8%E6%88%B7%E9%85%8D%E7%BD%AE%E7%9B%AE%E5%BD%95%2Frime.log)) ([ENABLE_LOGGING 不应该默认开启 · Issue #254 · rime/librime · GitHub](https://github.com/rime/librime/issues/254#:~:text=%E6%84%8F%E5%A4%96%E5%8F%91%E7%8E%B0%20%2Ftmp%20%E4%B8%8B%E6%9C%89%E4%B8%80%E4%B8%AA%E5%BE%88%E5%A4%A7%EF%BC%88200M%2B%EF%BC%89%E7%9A%84%E6%97%A5%E5%BF%97%E6%96%87%E4%BB%B6%20%2Ftmp%2Frime.fcitx,rime%20%E9%A1%B9%E7%9B%AE%E6%9B%BE%E6%9C%89%E4%BA%BA%E6%8A%A5%E5%91%8A%E8%BF%87))
- librime-lua 插件开发讨论 ([如何进行输入统计？ · Issue #90 · rime/librime · GitHub](https://github.com/rime/librime/issues/90#:~:text=local%20function%20on_commit,time)) ([能获取前一次上屏的内容吗？ · Issue #23 · hchunhui/librime-lua · GitHub](https://github.com/hchunhui/librime-lua/issues/23#:~:text=,%E7%9A%84%E4%B8%80%E4%B8%AA%E6%88%90%E5%91%98%E4%B8%AD%E6%9D%A5%E5%AE%9E%E7%8E%B0%EF%BC%89%EF%BC%8C%E5%90%A6%E5%88%99%20lua%20%E5%9C%A8%20gc%20%E7%9A%84%E6%97%B6%E5%80%99%E5%9B%9E%E8%B0%83%E4%BC%9A%E6%96%AD%E6%8E%89%E3%80%82))
- 小众软件论坛用户经验分享
