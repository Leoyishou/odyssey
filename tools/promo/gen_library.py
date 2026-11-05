#!/usr/bin/env python3
"""Demo library for the promo video: a handful of everyday team documents
plus a 1,000,000-character knowledge-base compilation for the scrolling shot.

    python3 tools/promo/gen_library.py /tmp/fdl-promo/lib

All content is synthetic. Deterministic (fixed seed)."""
import json, os, random, sys, time
from xml.sax.saxutils import escape as esc

R = random.Random(20261023)
out = sys.argv[1] if len(sys.argv) > 1 else "/tmp/fdl-promo/lib"
os.makedirs(f"{out}/docs", exist_ok=True)
os.makedirs(f"{out}/assets", exist_ok=True)

# ---------------------------------------------------------------- prose
SUBJ = ["我们", "团队", "产品组", "设计侧", "研发同学", "这一版", "新方案", "用户", "核心链路", "编辑器", "数据层", "增长小组", "客服团队", "运营同学"]
VERB = ["梳理了", "重新评估了", "验证了", "收敛了", "补齐了", "简化了", "拆解了", "对齐了", "复盘了", "优化了", "量化了", "跑通了"]
OBJ = ["首屏加载的关键路径", "新用户的上手流程", "文档协作的冲突处理", "长文档的滚动性能", "表格的列宽策略", "导出 Markdown 的格式细节",
       "移动端的阅读体验", "搜索结果的排序规则", "离线状态下的保存逻辑", "块级菜单的交互", "评论与提及的通知节奏", "权限模型的边界",
       "模板中心的分类方式", "图片上传的压缩参数", "快捷键的冲突列表", "目录大纲的折叠规则", "版本历史的保留周期"]
TAIL = ["，结论是先保留现状、下个迭代再看数据", "，整体耗时下降了约 {n}%", "，并在周三的评审会上达成一致", "，详细记录见附表",
        "，目前还有 {k} 个边界情况没有覆盖", "，灰度一周后再全量", "，用户反馈明显变好", "，需要设计再出一版对比稿",
        "，风险点已同步给相关负责人", "，下周一前给出排期", "", "", ""]
LEAD = ["从数据上看，", "需要注意的是，", "另外，", "换个角度说，", "回到最初的目标，", "具体来说，", "值得一提的是，", "综合下来，", "", "", ""]
CLAUSE = ["真正影响体验的往往不是功能数量，而是每一步是否顺手", "任何需要用户停下来思考的地方，都是可以继续打磨的地方",
          "我们更愿意把复杂度留在系统内部，把简单留给使用者", "好的默认值比十个配置项更有用", "先把最常见的路径做到极致，再考虑长尾",
          "一致性本身就是一种功能", "可感知的速度比基准测试的数字更重要", "文档是团队的长期记忆，写清楚比写得多更重要",
          "把问题拆小，每一步都能验证，进度自然就稳了", "对齐的成本远低于返工的成本"]
NAMES = ["林晓", "周默", "陈一帆", "王可", "赵青", "孙启明", "李遥", "吴悠", "郑南", "冯舒", "许安", "何川"]

def sentence():
    if R.random() < 0.25:
        return R.choice(LEAD) + R.choice(CLAUSE) + "。"
    t = R.choice(TAIL).format(n=R.randint(12, 64), k=R.randint(2, 9))
    return R.choice(LEAD) + R.choice(SUBJ) + R.choice(VERB) + R.choice(OBJ) + t + "。"

def para(lo=2, hi=5):
    return "".join(sentence() for _ in range(R.randint(lo, hi)))

def styled(text):
    """Sprinkle inline styles over a paragraph."""
    parts = text.split("，")
    out = []
    for p in parts:
        r = R.random()
        if r < 0.06: out.append(f"<b>{esc(p)}</b>")
        elif r < 0.09: out.append(f'<span text-color="blue">{esc(p)}</span>')
        elif r < 0.11: out.append(f'<span background-color="light-yellow">{esc(p)}</span>')
        elif r < 0.13: out.append(f"<u>{esc(p)}</u>")
        else: out.append(esc(p))
    return "，".join(out)

def p(text=None):
    return f"<p>{styled(text or para())}</p>"

def ul(n=None):
    return "<ul>" + "".join(f"<li>{esc(sentence())}</li>" for _ in range(n or R.randint(3, 5))) + "</ul>"

def ol(n=None):
    return "<ol>" + "".join(f"<li>{esc(sentence())}</li>" for _ in range(n or R.randint(3, 5))) + "</ol>"

def todos(n=None):
    return "".join(f'<checkbox done="{str(R.random() < .45).lower()}">{esc(R.choice(NAMES))}：{esc(R.choice(VERB)[:-1])}{esc(R.choice(OBJ))}</checkbox>'
                   for _ in range(n or R.randint(3, 5)))

CALLOUTS = [("💡", "light-yellow", "yellow"), ("📌", "light-blue", "blue"), ("⚠️", "light-orange", "orange"), ("✅", "light-green", "green"), ("🎯", "light-purple", "purple")]
def callout():
    e, bg, bd = R.choice(CALLOUTS)
    return f'<callout emoji="{e}" background-color="{bg}" border-color="{bd}"><p>{styled(R.choice(CLAUSE) + "。" + sentence())}</p></callout>'

def quote():
    return f"<blockquote><p>{esc(R.choice(CLAUSE))}。—— {esc(R.choice(NAMES))}</p></blockquote>"

CODE = [("rust", "fn visible_rows(&self, top: f32, height: f32) -> Range<usize> {\n    let first = self.heights.partition_point(|&y| y < top);\n    let last = self.heights.partition_point(|&y| y < top + height);\n    first..(last + 1).min(self.len())\n}"),
        ("typescript", "export async function save(doc: Doc) {\n  const xml = serialize(doc);\n  await fs.writeFile(`${doc.id}.xml.tmp`, xml);\n  await fs.rename(`${doc.id}.xml.tmp`, `${doc.id}.xml`);\n}"),
        ("python", "def p95(samples):\n    s = sorted(samples)\n    return s[int(len(s) * 0.95)]"),
        ("sql", "SELECT date, COUNT(DISTINCT user_id) AS dau\nFROM events\nWHERE action = 'open_doc'\nGROUP BY date\nORDER BY date;")]
def code():
    lang, src = R.choice(CODE)
    return f'<pre lang="{lang}"><code>{esc(src)}</code></pre>'

def table(rows=None):
    heads = R.choice([["模块", "负责人", "进度", "备注"], ["指标", "上周", "本周", "变化"], ["需求", "优先级", "状态", "预计上线"]])
    body = []
    for _ in range(rows or R.randint(3, 5)):
        if heads[0] == "指标":
            a, b = R.randint(1200, 9800), R.randint(1200, 9800)
            cells = [R.choice(["日活", "新建文档", "平均停留", "分享次数", "搜索次数", "导出次数"]), f"{a:,}", f"{b:,}", f"{(b - a) / a:+.1%}"]
        elif heads[0] == "模块":
            cells = [R.choice(["编辑器", "表格", "搜索", "导出", "模板", "侧栏", "评论"]), R.choice(NAMES), f"{R.randint(20, 100)}%", R.choice(["按计划", "需要支援", "已提测", "待评审"])]
        else:
            cells = [R.choice(OBJ), R.choice(["P0", "P1", "P2"]), R.choice(["开发中", "设计中", "已完成", "排期中"]), f"{R.randint(11, 12)} 月 {R.randint(1, 28)} 日"]
        body.append(cells)
    w = [180, 120, 120, 200]
    colg = "".join(f'<col width="{x}"/>' for x in w)
    th = "".join(f'<th background-color="light-gray"><p>{esc(h)}</p></th>' for h in heads)
    tb = "".join("<tr>" + "".join(f"<td><p>{esc(c)}</p></td>" for c in r) + "</tr>" for r in body)
    return f"<table><colgroup>{colg}</colgroup><thead><tr>{th}</tr></thead><tbody>{tb}</tbody></table>"

def grid():
    return ('<grid><column width-ratio="0.5">' + f"<h3>{esc(R.choice(['优势', '目标', '现状']))}</h3>" + ul(3) +
            '</column><column width-ratio="0.5">' + f"<h3>{esc(R.choice(['风险', '下一步', '待讨论']))}</h3>" + ul(3) + "</column></grid>")

# ---------------------------------------------------------------- chart image
def chart(path):
    from PIL import Image, ImageDraw, ImageFont
    W, H = 1360, 600
    im = Image.new("RGB", (W, H), (255, 255, 255))
    d = ImageDraw.Draw(im)
    try:
        f = ImageFont.truetype("/System/Library/Fonts/Hiragino Sans GB.ttc", 32)
        fs = ImageFont.truetype("/System/Library/Fonts/Hiragino Sans GB.ttc", 26)
    except OSError:
        f = fs = ImageFont.load_default()
    d.text((60, 40), "周活跃用户（千人）", fill=(31, 35, 41), font=f)
    vals = [42, 47, 51, 49, 58, 63, 61, 70, 76, 83, 88, 97]
    x0, y0, bw, gap = 80, 520, 70, 34
    for i, v in enumerate(vals):
        x = x0 + i * (bw + gap)
        h = v * 4
        d.rounded_rectangle((x, y0 - h, x + bw, y0), 8, fill=(51, 112, 255) if i == len(vals) - 1 else (186, 206, 253))
        d.text((x + bw / 2, y0 + 30), f"W{i + 1}", fill=(100, 106, 115), font=fs, anchor="mm")
        d.text((x + bw / 2, y0 - h - 22), str(v), fill=(100, 106, 115), font=fs, anchor="mm")
    d.line((60, y0, W - 40, y0), fill=(222, 224, 227), width=2)
    im.save(path)

chart(f"{out}/assets/weekly-active.png")

# ---------------------------------------------------------------- documents
docs = []
def doc(title, body):
    docs.append((title, body))

doc("2026 Q4 产品规划", "".join([
    f'<callout emoji="🎯" background-color="light-blue" border-color="blue"><p><b>本季度目标</b>：把长文档编辑体验做到行业第一梯队，七日留存提升到 45%。</p></callout>',
    "<h1>一、背景</h1>", p(), p(),
    '<img href="assets/weekly-active.png" width="680" caption="图 1 · 近 12 周活跃用户"/>',
    "<h1>二、重点项目</h1>", table(5),
    "<h2>2.1 长文档性能</h2>", p(), ul(4),
    "<h2>2.2 表格与数据</h2>", p(), grid(),
    "<h1>三、里程碑</h1>", todos(5), quote(), p(),
]))
doc("周会纪要 · 10 月 20 日", "".join([
    f"<p>参会：{esc('、'.join(R.sample(NAMES, 6)))}</p>", "<h2>上周进展</h2>", ol(4), "<h2>数据</h2>", table(4),
    "<h2>讨论</h2>", p(), callout(), "<h2>待办</h2>", todos(5)]))
doc("技术方案：本地存储与原子写入", "".join([
    "<h1>目标</h1>", p(), callout(), "<h1>设计</h1>", p(), code(), p(), code(), "<h2>边界情况</h2>", ul(5), table(3)]))
doc("新人入职指南", "".join([
    callout(), "<h1>第一周</h1>", todos(6), "<h1>常用资源</h1>", ul(5), "<h1>团队约定</h1>", ol(4), quote()]))
doc("用户访谈汇总", "".join(["<h1>访谈概况</h1>", table(4)] + [f"<h2>用户 {c}</h2>" + p() + quote() + ul(3) for c in "ABCDE"]))
doc("读书笔记：《设计心理学》", "".join(["<h1>核心观点</h1>", ol(5), p(), quote(), "<h1>摘录</h1>", p(), p(), callout()]))

# The million-character compilation: chapters of mixed blocks.
TOPICS = ["编辑器", "表格", "搜索", "导出", "协作", "模板", "移动端", "性能", "数据", "设计系统", "增长", "客服"]
def text_len(xml):
    import re
    return len(re.sub(r"<[^>]+>", "", xml).replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">"))

parts, n, ch = [], 0, 0
while n < 1_000_000:
    ch += 1
    sec = [f"<h1>第 {ch} 章 · {R.choice(TOPICS)}{R.choice(['复盘', '方案', '纪要', '调研', '规范', '手册'])}</h1>", p()]
    for s in range(1, R.randint(3, 5)):
        sec.append(f"<h2>{ch}.{s} {esc(R.choice(OBJ))}</h2>")
        for _ in range(R.randint(2, 4)):
            sec.append(p())
            sec.append(R.choice([ul, ol, callout, quote, todos, code, table, p, p, p])())
    x = "".join(sec)
    parts.append(x)
    n += text_len(x)
doc("产品知识库 · 合订本（100 万字）", "".join(parts))

index = {}
t0 = int(time.time()) - 600
for i, (title, body) in enumerate(docs):
    did = f"d0c0{i:04d}-0000-4000-8000-{i:012d}"
    with open(f"{out}/docs/{did}.xml", "w") as f:
        f.write(f"<title>{esc(title)}</title>\n{body}\n")
    t = t0 - i * 5400
    index[did] = {"created": t - 86400 * 3, "modified": t, "pinned": False, "opened": t}
json.dump(index, open(f"{out}/index.json", "w"), indent=2)
print(f"{len(docs)} docs; big doc {n:,} chars in {ch} chapters")
