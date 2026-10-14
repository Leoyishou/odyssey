---
uid: 35cd954f-770d-4edb-b0ad-2ea91b6ce408
---
## Superpowers × 产品研发流程

| Linear 状态    | 对应 Superpowers                 | 触发时机     |
| :----------- | :----------------------------- | :------- |
| Backlog/Todo | brainstorming                  | "做个XX功能" |
|              | writing-plans                  | 需求明确后    |
| In Progress  | using-git-worktrees            | 创建分支前    |
|              | test-driven-development        | 写代码前     |
|              | systematic-debugging           | 遇到 bug 时 |
|              | subagent-driven-development    | 多任务并行时   |
| In Review    | requesting-code-review         | PR 创建后   |
|              | receiving-code-review          | 收到反馈后    |
| 待发布          | verification-before-completion | 声称完成前    |
|              | finishing-a-development-branch | 准备合并时    |
| Done         | （无）                            |          |

模拟场景：「给 Viva 加个 HelloWorld 落地页」
                                                                                                                                                                 
  假设你在 ~/usr/pac/product-bu/viva 项目下，对我说：                                  

  "给 Viva 加个 HelloWorld 落地页"

  ---
  阶段 1: Brainstorming（需求分析）

  触发时机：我收到「加功能」的请求时

  实际动作：

  我：我正在使用 superpowers:brainstorming 技能来探索需求。

  [Claude 用 Skill 工具调用 brainstorming]
     ↓
  [PostToolUse Hook 触发]
     ↓
  [superpowers-tracker.js 检测到 brainstorming]
     ↓
  [异步启动 PMO Claude: 处理 superpower_event, phase=requirements]
     ↓
  [PMO 根据 sessionId 创建 Linear Issue: "【0204-15】Viva: HelloWorld 落地页"]

  然后我会开始问你问题：
  - 这个落地页的目的是什么？（展示产品 / 收集邮箱 / 下载引导）
  - 需要什么内容？（Hero Section / 功能介绍 / CTA 按钮）
  - 技术约束？（静态 HTML / React 组件 / 单独域名）

  ---
  阶段 2: Writing Plans（写实施计划）

  触发时机：需求明确后，我准备写计划时

  实际动作：

  我：需求已明确，正在使用 superpowers:writing-plans 编写实施计划。

  [Claude 用 Skill 工具调用 writing-plans]
     ↓
  [Hook 触发 → PMO agent]
     ↓
  [PMO 更新 Linear Issue description，追加计划内容]

  我会写一个 docs/plans/2026-02-04-helloworld-landing-design.md：

  ## 实施计划

  1. 创建 `src/pages/landing.tsx`
  2. 添加 Hero Section 组件
  3. 添加 CTA 按钮
  4. 配置路由 `/`
  5. 部署到 Vercel

  ---
  阶段 3: Using Git Worktrees（创建隔离环境）

  触发时机：计划确认后，准备开始写代码时

  实际动作：

  我：正在使用 superpowers:using-git-worktrees 创建隔离工作区。

  [Claude 执行以下步骤]
  6. 检查是否存在 .worktrees/ 目录
  7. 检查是否被 .gitignore 忽略
  8. 创建 worktree:
     git worktree add .worktrees/landing -b feat/P-123-landing-page
  9. cd .worktrees/landing
  10. npm install
  11. npm test (验证基线)

  [Hook 触发 → PMO agent]
     ↓
  [PMO 更新 Linear Issue 状态 → In Progress]

  输出：
  Worktree ready at /Users/liuyishou/usr/pac/product-bu/viva/.worktrees/landing
  Tests passing (47 tests, 0 failures)
  Ready to implement HelloWorld Landing Page

  ---
  阶段 4: 开发（TDD）

  触发时机：在 worktree 里写代码时

  实际动作：

  [Claude 调用 superpowers:test-driven-development]
     ↓
  [Hook 触发 → PMO 记录活动（可选）]

  先写测试：
  describe('Landing Page', () => {
    it('renders hero section', () => { ... });
    it('has CTA button', () => { ... });
  });

  再写实现：
  // src/pages/landing.tsx
  export default function Landing() {
    return (
      <div>
        <h1>Welcome to Viva</h1>
        <button>Get Started</button>
      </div>
    );
  }

  ---
  阶段 5: 验证 + 请求审查

  触发时机：代码写完，准备提交时

  实际动作：

  [Claude 调用 superpowers:verification-before-completion]
     ↓
  - 运行 npm test
  - 运行 npm run build
  - 确认无 lint 错误

  [Claude 调用 superpowers:requesting-code-review]
     ↓
  [Hook 触发 → PMO agent]
     ↓
  [PMO 更新 Linear Issue 状态 → In Review]
     ↓
  - 创建 PR: gh pr create --title "feat: add landing page"

  ---
  阶段 6: 完成分支

  触发时机：PR 合并后

  实际动作：

  [Claude 调用 superpowers:finishing-a-development-branch]
     ↓
  [Hook 触发 → PMO agent]
     ↓
  [PMO 更新 Linear Issue 状态 → 待发布 / Done]
     ↓
  - 清理 worktree: git worktree remove .worktrees/landing
  - 删除本地分支: git branch -d feat/P-123-landing-page

  ---
  总结：完整流程图

  用户: "给 Viva 加个 HelloWorld 落地页"
       │
       ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ 1. brainstorming                                            │
  │    • 探索需求，问问题                                        │
  │    • PMO: 创建 Linear Issue                                 │
  └─────────────────────────────────────────────────────────────┘
       │
       ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ 2. writing-plans                                            │
  │    • 写 docs/plans/xxx.md                                   │
  │    • PMO: 追加计划到 Issue                                   │
  └─────────────────────────────────────────────────────────────┘
       │
       ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ 3. using-git-worktrees                                      │
  │    • git worktree add .worktrees/landing -b feat/xxx        │
  │    • npm install && npm test                                │
  │    • PMO: Issue → In Progress                               │
  └─────────────────────────────────────────────────────────────┘
       │
       ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ 4. test-driven-development                                  │
  │    • 先写测试，再写实现                                       │
  └─────────────────────────────────────────────────────────────┘
       │
       ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ 5. verification-before-completion + requesting-code-review  │
  │    • 跑测试、构建、lint                                       │
  │    • 创建 PR                                                 │
  │    • PMO: Issue → In Review                                 │
  └─────────────────────────────────────────────────────────────┘
       │
       ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ 6. finishing-a-development-branch                           │
  │    • 合并 PR                                                 │
  │    • 清理 worktree                                           │
  │    • PMO: Issue → 待发布/Done                                │
  └─────────────────────────────────────────────────────────────┘

  这就是 superpowers + PMO 的完整工作流。每个关键节点都会自动触发 hook，启动 PMO Claude 同步状态到 Linear。

1. 需求阶段（Backlog → Todo）

当你说"给 Viva 加个XX功能"时，我应该先用 brainstorming 探索：
- 你真正想要什么
- 有哪些方案
- 技术约束

然后用 writing-plans 输出实施计划。

2. 开发阶段（In Progress）

- 用 using-git-worktrees 创建隔离环境
- 用 test-driven-development 先写测试再写实现
- 遇到问题用 systematic-debugging 而不是乱猜

3. 收尾阶段（In Review → Done）

- 用 verification-before-completion 确保真的能跑
- 用 requesting-code-review 让另一个 session 审查
- 用 finishing-a-development-branch 决定合并策略

| 类别       | Skill                          | 说明                                                    |
| :--------- | :----------------------------- | :------------------------------------------------------ |
| 协作流程   | brainstorming                  | 🔥 苏格拉底式需求澄清，分段验收设计                     |
|            | writing-plans                  | 拆解任务（每个 2-5 分钟），含文件路径+完整代码+验证步骤 |
|            | executing-plans                | 批量执行 + 人工 checkpoint                              |
|            | subagent-driven-development    | 🔥 两阶段 review：先验规格，再验代码质量                |
| 测试       | test-driven-development        | 🔥 强制 RED-GREEN-REFACTOR，写代码前先写测试            |
| 调试       | systematic-debugging           | 4 阶段根因分析                                          |
|            | verification-before-completion | 修完必须验证                                            |
| Git 工作流 | using-git-worktrees            | 创建隔离分支工作区                                      |
|            | finishing-a-development-branch | merge/PR/discard 决策                                   |
| 代码审查   | requesting-code-review         | 提交前自查清单                                          |
|            | receiving-code-review          | 响应 review 反馈                                        |
| 元         | writing-skills                 | 创建新 skill 的最佳实践                                 |