import { defineAgent } from "@grok/agent-sdk";

export default defineAgent({
  name: "Odyssey 第二大脑",
  description: "检索 Odyssey 公开笔记，依据原文回答并引用提交版本与行号。",
  tools: [],
  concurrency: { maxRunningTurns: 2 },
  hosting: { egressDomains: ["github.com", "raw.githubusercontent.com"] },
  model: {
    id: "grok-4.5",
    params: [
      { id: "effort", value: "high" },
      { id: "fast", value: "true" },
    ],
  },
});
