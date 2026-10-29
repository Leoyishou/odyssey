import { execFile } from "node:child_process";
import { mkdtemp, readdir, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, relative, sep } from "node:path";
import { promisify } from "node:util";

const exec = promisify(execFile);
const REPO = "https://github.com/Leoyishou/odyssey";
const TTL = 5 * 60_000;
const MAX_NOTE_BYTES = 16 * 1024 * 1024;
type Note = { path: string; title: string; text: string };
type Snapshot = { commit: string; fetchedAt: string; notes: Note[] };
let current: Snapshot | undefined;
let loading: Promise<Snapshot> | undefined;

export function validPath(path: string) {
  return (path === "README.md" || path.startsWith("第二大脑/")) && path.endsWith(".md") &&
    !path.split("/").some(p => p === ".." || p === "." || !p) && !/[\\\x00-\x1f]/u.test(path);
}

export function sourceUrl(path: string, commit: string, start = 1, end = start) {
  return `${REPO}/blob/${commit}/${path.split("/").map(encodeURIComponent).join("/")}#L${start}-L${end}`;
}

async function download(): Promise<Snapshot> {
  const scratch = await mkdtemp(join(tmpdir(), "odyssey-public-"));
  const checkout = join(scratch, "repo");
  try {
    // Fixed public repository; no shell or caller-controlled command arguments.
    await exec("git", ["-c", "credential.helper=", "clone", "--depth", "1", "--no-tags", "--single-branch",
      "--branch", "main", `${REPO}.git`, checkout], {
      timeout: 90_000, maxBuffer: 2 * 1024 * 1024,
      env: { ...process.env, GIT_CONFIG_NOSYSTEM: "1", GIT_CONFIG_GLOBAL: "/dev/null",
        GIT_TERMINAL_PROMPT: "0", GIT_ASKPASS: "false" },
    });
    const { stdout } = await exec("git", ["-C", checkout, "rev-parse", "HEAD"], { timeout: 5000 });
    const commit = stdout.trim();
    if (!/^[a-f0-9]{40}$/.test(commit)) throw new Error("Invalid repository commit");
    const notes: Note[] = [];
    let total = 0;
    async function visit(dir: string) {
      for (const entry of await readdir(dir, { withFileTypes: true })) {
        if (entry.name.startsWith(".")) continue;
        const file = join(dir, entry.name);
        if (entry.isDirectory()) await visit(file);
        else if (entry.isFile() && entry.name.endsWith(".md")) {
          const path = relative(checkout, file).split(sep).join("/");
          if (!validPath(path)) continue;
          const bytes = await readFile(file);
          if (bytes.length > MAX_NOTE_BYTES) throw new Error(`Note too large: ${path}`);
          total += bytes.length;
          if (notes.length >= 20_000 || total > 64 * 1024 * 1024) throw new Error("Repository exceeds corpus limit");
          notes.push({ path, title: entry.name.replace(/\.md$/, ""), text: bytes.toString("utf8") });
        }
      }
    }
    await visit(join(checkout, "第二大脑"));
    notes.push({ path: "README.md", title: "Odyssey README", text: await readFile(join(checkout, "README.md"), "utf8") });
    return { commit, fetchedAt: new Date().toISOString(), notes };
  } finally {
    await rm(scratch, { recursive: true, force: true });
  }
}

export async function snapshot() {
  if (current && Date.now() - Date.parse(current.fetchedAt) < TTL) return current;
  if (!loading) loading = download().then(data => (current = data)).finally(() => { loading = undefined; });
  return loading;
}

export function rank(notes: Note[], query: string, limit: number, commit: string) {
  const terms = [...new Set(query.normalize("NFKC").toLowerCase().split(/\s+/u).filter(Boolean))];
  return notes.map(note => {
    const title = note.title.normalize("NFKC").toLowerCase();
    const path = note.path.replace(/^第二大脑\//u, "").normalize("NFKC").toLowerCase();
    const body = note.text.normalize("NFKC").toLowerCase();
    let score = 0;
    for (const term of terms) {
      if (title === term) score += 80;
      if (title.includes(term)) score += 30;
      if (path.includes(term)) score += 5;
      if (body.includes(term)) score += 4;
    }
    return { note, score };
  }).filter(hit => hit.score > 0).sort((a, b) => b.score - a.score || a.note.path.localeCompare(b.note.path))
    .slice(0, limit).map(({ note, score }) => {
      const lines = note.text.split("\n");
      const hitLine = lines.findIndex(line => terms.some(term => line.normalize("NFKC").toLowerCase().includes(term)));
      const startLine = Math.max(1, hitLine + 1 - 2);
      const endLine = Math.min(lines.length, startLine + 7);
      return { path: note.path, title: note.title, score, startLine, endLine,
        snippet: lines.slice(startLine - 1, endLine).map((line, i) => `${startLine + i}: ${line.slice(0, 600)}`).join("\n"),
        url: sourceUrl(note.path, commit, startLine, endLine) };
    });
}

export async function readNote(path: string, commit: string, startLine: number, count: number) {
  if (!validPath(path) || !/^[a-f0-9]{40}$/.test(commit)) throw new Error("Invalid public note path or commit");
  let text = current?.commit === commit ? current.notes.find(note => note.path === path)?.text : undefined;
  if (text === undefined) {
    const url = `https://raw.githubusercontent.com/Leoyishou/odyssey/${commit}/${path.split("/").map(encodeURIComponent).join("/")}`;
    const response = await fetch(url, { signal: AbortSignal.timeout(20_000), redirect: "error" });
    if (!response.ok) throw new Error(`GitHub read failed: HTTP ${response.status}`);
    const reader = response.body!.getReader();
    const parts: Uint8Array[] = [];
    let size = 0;
    while (true) {
      const part = await reader.read();
      if (part.done) break;
      size += part.value.length;
      if (size > MAX_NOTE_BYTES) { await reader.cancel(); throw new Error("Note exceeds read limit"); }
      parts.push(part.value);
    }
    text = Buffer.concat(parts).toString("utf8");
  }
  const lines = text.split("\n");
  if (startLine > lines.length) throw new Error(`startLine exceeds ${lines.length} lines`);
  const endLine = Math.min(lines.length, startLine + count - 1);
  const selected = lines.slice(startLine - 1, endLine);
  let remaining = 24_000;
  const content: string[] = [];
  for (const [i, line] of selected.entries()) {
    if (remaining < 50) break;
    const clipped = line.slice(0, Math.min(2000, remaining));
    const rendered = `${startLine + i}: ${clipped}${clipped.length < line.length ? " [本行截断]" : ""}`;
    content.push(rendered);
    remaining -= rendered.length;
  }
  const actualEnd = startLine + content.length - 1;
  return { path, commit, startLine, endLine, totalLines: lines.length,
    content: content.join("\n"), returnedEndLine: actualEnd,
    url: sourceUrl(path, commit, startLine, actualEnd), nextLine: actualEnd < lines.length ? actualEnd + 1 : null };
}
