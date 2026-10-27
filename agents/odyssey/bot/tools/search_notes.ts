import { defineTool } from "@grok/agent-sdk/tools";
import { z } from "zod";
import { rank, snapshot } from "../lib/notes.js";

export default defineTool({
  description: "检索 Odyssey main 分支公开笔记的标题、路径和全文。query 用空格分隔的 1–4 个关键词（匹配任意词），避免整句问句。返回原文片段、行号和不可变提交链接；先搜索再 read_note 核对上下文。首次或缓存过期会下载公开仓库，最长约 90 秒。",
  execution: "server",
  effect: "read",
  inputSchema: z.object({ query: z.string().trim().min(1).max(120), limit: z.number().int().min(1).max(10).default(5) }),
  async execute({ query, limit }) {
    const data = await snapshot();
    return { repository: "Leoyishou/odyssey", branch: "main", commit: data.commit, fetchedAt: data.fetchedAt,
      noteCount: data.notes.length, cacheMaxAgeSeconds: 300, results: rank(data.notes, query, limit, data.commit) };
  },
});
