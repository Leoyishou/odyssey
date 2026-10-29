import assert from "node:assert/strict";
import test from "node:test";
import { rank, readNote, sourceUrl, validPath } from "../bot/lib/notes.js";

const commit = "a".repeat(40);
test("全文可命中标题中不存在的概念，片段保留真实行号", () => {
  const results = rank([{ path: "第二大脑/常青笔记.md", title: "常青笔记", text: "引言\n\n知识复利来自持续迭代\n正文" }], "知识复利", 3, commit);
  assert.equal(results.length, 1);
  assert.match(results[0].snippet, /3: 知识复利来自持续迭代/);
  assert.match(results[0].url, /#L1-L4$/);
  assert.equal(rank([], "知识复利", 3, commit).length, 0);
});
test("无依据返回空结果，多个词可分别命中", () => {
  const notes = [{ path: "第二大脑/常青笔记.md", title: "常青笔记", text: "持续迭代" }];
  assert.equal(rank(notes, "不存在的证据", 3, commit).length, 0);
  assert.equal(rank(notes, "常青 迭代", 3, commit).length, 1);
});
test("拒绝穿越、私有路径和其他仓库", async () => {
  for (const path of ["../../.env", "第二大脑/../.env.md", "private/notes.md", "第二大脑/a\\b.md", "第二大脑/a\n.md", "https://example.com/a.md"]) {
    assert.equal(validPath(path), false);
    await assert.rejects(readNote(path, commit, 1, 5), /Invalid/);
  }
  await assert.rejects(readNote("README.md", "main", 1, 5), /Invalid/);
});
test("来源链接编码中文、空格和井号，固定提交", () => {
  assert.equal(sourceUrl("第二大脑/AI #1.md", commit, 3, 8),
    `https://github.com/Leoyishou/odyssey/blob/${commit}/%E7%AC%AC%E4%BA%8C%E5%A4%A7%E8%84%91/AI%20%231.md#L3-L8`);
});
