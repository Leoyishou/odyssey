---
comment_id: 90ad4216
date created: 2025-05-03
date modified: 2025-06-15
draw: null
title: gin
---
Thought for 5 seconds

的确，上面那段「Hello Cursor」示例只演示了**单用户**调用 API。

若你想用 Cursor 一次生成 **支持注册 / 登录 / 每用户独立背词进度（四技能 + SuperMemo）** 的完整骨架，可把下面 **升级版 Prompt** 粘进 Cursor AI 面板，让它自动写出 _多用户全栈_ 项目。

## 项目目标

生成一个最小但可跑的“多人背单词”网站骨架，技术栈 **Vue-3 + Pinia + Vite (+ TypeScript)** 前端，**FastAPI + SQLModel + SQLite** 后端。功能：

1. 用户注册 / 登录（JWT 持久化）
2. 词书+单词公共数据只读
3. “四技能” 独立间隔复习 (SuperMemo-2)
4. 每个用户可以：
   - 拉取今天待复习队列 `GET /api/queue?skill=listening`
   - 提交复习结果 `POST /api/review`
   - 查看学习统计 `GET /api/dashboard`
5. 前端含 **登录页** + **学习页** 两个路由

## 数据库（SQLModel → SQLite）
```Java

user(id, username, password_hash, created_at)  
skill(skill_id PK, name) -- listening / reading / speaking / writing  
word(word_id PK, spelling, cn_def, phonetic, en_audio, ...)  
review_state(id PK, user_id FK, word_id FK, skill_id FK,  
status, ease_factor, repetition, interval, due_at, last_review_at)

```

## 项目结构
```Java

/frontend  
├─ index.html  
├─ src/  
│ ├─ main.ts  
│ ├─ router.ts # /login /study  
│ ├─ store/user.ts # Pinia，存 JWT  
│ ├─ api/auth.ts # login / register  
│ ├─ api/learn.ts # queue / review / dashboard  
│ └─ pages/  
│ ├─ Login.vue  
│ └─ Study.vue  
└─ package.json

/backend  
├─ main.py # FastAPI 实例 + CORS  
├─ database.py # SQLModel + SessionLocal  
├─ models.py # 上述表  
├─ routers/  
│ ├─ auth.py # /auth/*  
│ ├─ learn.py # /queue /review /dashboard  
│ └─ seed.py # 初始导入 word & skill  
├─ security.py # JWT & pwd hashing  
└─ requirements.txt

README.md

```

## 关键要求
- **认证**  
  - `POST /auth/register`：用户名 + 密码 → 写库  
  - `POST /auth/login`：返回 `access_token` (JWT)  
  - 统一在 `Authorization: Bearer <token>` 头校验
- **间隔复习逻辑**  
  - 取队列：`due_at <= now()` + `skill_id`  
  - 提交 `grade(0-5)` 后按 SM-2 更新 `ease_factor / interval / due_at`
- **前端**  
  - Axios 请求自动带 `Authorization`（拦截器）
  - `Study.vue`：进入后先 `GET /queue?skill=reading` 拉 1 条；答题后 POST /review，再拉下一条
- **脚本**  
  - `npm run dev` → Vite(5173)  
  - `uvicorn main:app --reload` → API(8000)
- **README**：一步步启动＋生成种子词库 `python -m backend.routers.seed`
- **代码风格**  
  - Vue `<script setup lang="ts">`  
  - 后端每个路由函数带 `@router.post()/get()` 注解并返回 Pydantic 模型
- **额外 (可选)**  
  - dashboard 返回今日新学 / 待复习 / 已掌握数量  
  - 对未登录用户自动 302 到 `/login`

## 输出格式
- 每个文件前加注释 `# path:...` 或 `// path:...`  
- 按项目结构顺序输出所有文件  
- 最后用 “✅ Done” 结尾


---

> **使用方法**
>
> 1. 在空目录运行 `cursor.` 并打开 AI 面板。
>
> 2. 粘贴整段 Prompt → 回车。等待文件生成。
>
> 3. `cd backend && pip install -r requirements.txt && uvicorn main:app --reload`
>
> 4. 另开终端 `cd frontend && npm i && npm run dev`
>
> 5. 浏览器 [http://localhost:5173](http://localhost:5173/)——先注册，再开始四技能背词体验。
>

如需进一步细化（SM-2 公式、前端 UI 美化、Docker 一键部署等），随时告诉我！
