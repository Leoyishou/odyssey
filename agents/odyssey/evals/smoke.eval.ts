import { defineEval, includes } from "@grok/agent-sdk/evals";

export default defineEval({
  tags: ["smoke"],
  description: "依据真实笔记回答，并提供固定提交与行号的来源。",
  async test(t) {
    await t.send("根据 Odyssey 公开笔记，为什么需要第二大脑？请给出带行号的原文来源。");
    t.succeeded();
    t.calledTool("search_notes");
    t.calledTool("read_note");
    t.check(t.reply, includes(/https:\/\/github\.com\/Leoyishou\/odyssey\/blob\/[a-f0-9]{40}\/[^\s)]+#L\d+/));
  },
});
