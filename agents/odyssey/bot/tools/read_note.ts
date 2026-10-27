import { defineTool } from "@grok/agent-sdk/tools";
import { z } from "zod";
import { readNote, validPath } from "../lib/notes.js";

export default defineTool({
  description: "读取 search_notes 命中的原始 Markdown，使用其完整 path 和 commit，核实观点和上下文。返回真实行号及引用链接；长文按 nextLine 翻页。只支持 Odyssey 公开 README 和第二大脑下的 Markdown。",
  execution: "server",
  effect: "read",
  inputSchema: z.object({ path: z.string().max(1500).refine(validPath), commit: z.string().regex(/^[a-f0-9]{40}$/),
    startLine: z.number().int().min(1).default(1), count: z.number().int().min(1).max(160).default(100) }),
  async execute({ path, commit, startLine, count }) { return readNote(path, commit, startLine, count); },
});
