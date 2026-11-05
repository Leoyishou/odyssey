#!/usr/bin/env python3
"""Build the 开始指南 sample library from shared-mock/guides/src.

usage: python3 shared-mock/guides/build.py <out_library_dir>

Each src/*.xml is one document (file name order = reading order). Placeholders:
  {{doc:key}}                       @文档 link to the guide whose KEYS entry is key
  {{reminder:YYYY-MM-DD HH:MM}}     timed date reminder, notified when it is due
  {{reminder-day:YYYY-MM-DD}}       whole-day reminder, notified 09:00 that day
  {{countdown:YYYY-MM-DD HH:MM}}    countdown block to that moment (blue)
  {{button:Action|url|text|color}}  inline button (OpenLink / DuplicatePage)
Assets under guides/assets are copied to <out>/assets/guides.
"""
import datetime as dt, json, re, shutil, sys, time, uuid
from pathlib import Path

here = Path(__file__).resolve().parent
out = Path(sys.argv[1]).resolve()
NS = uuid.UUID("6f7c1e9a-9d0b-4a51-8c7e-2a1d3b4c5e6f")
# file stem -> (key, parent key, pinned)
KEYS = {
    "01-welcome": ("welcome", None, False),
    "02-writing": ("writing", None, False),
    "03-blocks": ("blocks", None, False),
    "04-tables": ("tables", None, False),
    "05-code": ("code", None, False),
    "06-media": ("media", None, False),
    "07-organize": ("organize", None, False),
    "07a-reading-list": ("reading-list", "organize", False),
    "07b-weekly": ("weekly", "organize", False),
    "08-data": ("data", None, False),
    "09-personalize": ("personalize", None, False),
    "10-practice": ("practice", None, False),
}
ids = {k: str(uuid.uuid5(NS, k)) for k, _, _ in KEYS.values()}
srcs = {KEYS[p.stem][0]: p.read_text() for p in sorted((here / "src").glob("*.xml"))}
missing = set(ids) - set(srcs)
assert not missing, f"no source for {missing}"

def esc(s): return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
def plain_title(x):
    t = re.search(r"<title[^>]*>(.*?)</title>", x, re.S).group(1)
    return re.sub(r"<[^>]+>", "", t).replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")
titles = {k: plain_title(x) for k, x in srcs.items()}
local = lambda s: dt.datetime.strptime(s, "%Y-%m-%d %H:%M").astimezone()
ms = lambda d: int(d.timestamp() * 1000)
now = int(time.time())

def fill(x):
    def doc(m):
        k = m.group(1)
        assert k in ids, f"unknown doc {k}"
        return f'<cite type="doc" doc-id="{ids[k]}" file-type="docx">{esc(titles[k])}</cite>'
    def rem(m):
        d = local(m.group(1))
        return f'<time expire-time="{ms(d)}" notify-time="{ms(d)}" should-notify="true">{d:%Y-%m-%d %H:%M}</time>'
    def rem_day(m):
        d = local(m.group(1) + " 23:59")
        n = local(m.group(1) + " 09:00")
        return f'<time expire-time="{ms(d)}" notify-time="{ms(n)}" should-notify="true" is-whole-day="true">{d:%Y-%m-%d}</time>'
    def countdown(m):
        d = local(m.group(1))
        start = now * 1000
        dur = (ms(d) - start) // 1000 + 1
        return f'<countdown duration="{dur}" start-time="{start}" notify="true" color="#4E83FD" timing="date" date="{d:%Y-%m-%d}" time="{d:%H:%M}"/>'
    def button(m):
        action, url, text, color = m.group(1).split("|")
        src = f' src="{esc(url)}"' if url else ""
        return f'<button action="{action}"{src} background-color="{color}">{esc(text)}</button>'
    x = re.sub(r"\{\{doc:([a-z-]+)\}\}", doc, x)
    x = re.sub(r"\{\{reminder:([0-9-]+ [0-9:]+)\}\}", rem, x)
    x = re.sub(r"\{\{reminder-day:([0-9-]+)\}\}", rem_day, x)
    x = re.sub(r"\{\{countdown:([0-9-]+ [0-9:]+)\}\}", countdown, x)
    x = re.sub(r"\{\{button:([^}]+)\}\}", button, x)
    assert "{{" not in x, re.search(r"\{\{[^}]*\}\}", x).group(0)
    # A cover is a file in the library (the app copies a gallery pick into
    # assets): bring the app's gallery image along.
    def cover(m):
        name = m.group(1)
        covers.add(name)
        return f'cover="assets/guides/covers/{name}"'
    return re.sub(r'cover="covers/([^"]+)"', cover, x)

covers = set()

if out.exists() and any(out.iterdir()):
    sys.exit(f"{out} is not empty")
(out / "docs").mkdir(parents=True)
index = {}
order = list(KEYS.values())
for i, (k, parent, pinned) in enumerate(order):
    (out / "docs" / f"{ids[k]}.xml").write_text(fill(srcs[k]))
    # Reading order: the sidebar lists the most recently modified first, and
    # children oldest first.
    t = now - 60 * i
    meta = {"created": now - 3600 + i, "modified": t, "pinned": pinned, "opened": t}
    if parent:
        meta["parent"] = ids[parent]
    index[ids[k]] = meta
(out / "index.json").write_text(json.dumps(index, ensure_ascii=False, indent=2) + "\n")
(out / "format.json").write_text('{\n  "format": 1\n}')
shutil.copytree(here / "assets", out / "assets" / "guides")
(out / "assets" / "guides" / "covers").mkdir()
for name in sorted(covers):
    shutil.copy2(here.parent.parent / "assets" / "covers" / name, out / "assets" / "guides" / "covers" / name)
print(out)
