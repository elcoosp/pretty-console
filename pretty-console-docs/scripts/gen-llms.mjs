// Generates public/llms.txt and public/llms-full.txt from the docs source.
// Run after content changes, before `pnpm build` (public/ is copied to dist/).
import { readdir, readFile, writeFile, mkdir } from 'node:fs/promises';
import path from 'node:path';

const ROOT = 'src/content/docs';

async function walk(dir) {
        const entries = await readdir(dir, { withFileTypes: true });
        const files = [];
        for (const e of entries) {
                const p = path.join(dir, e.name);
                if (e.isDirectory()) files.push(...(await walk(p)));
                else if (e.name.endsWith('.md') || e.name.endsWith('.mdx')) files.push(p);
        }
        return files;
}

function parseFrontmatter(text) {
        if (!text.startsWith('---')) return {};
        const end = text.indexOf('\n---', 3);
        const block = text.slice(4, end);
        const meta = {};
        for (const line of block.split('\n')) {
                const m = line.match(/^(title|description|type):\s*(.*)$/);
                if (m) meta[m[1]] = m[2].replace(/^["']|["']$/g, '');
        }
        return meta;
}

const files = (await walk(ROOT)).filter(
        (f) => !/(^|\/)404\.mdx?$/.test(f) && !/(^|\/)_/.test(path.basename(f)),
);
const pages = [];
for (const file of files) {
        const raw = await readFile(file, 'utf8');
        const meta = parseFrontmatter(raw);
        const rel = path.relative(ROOT, file).replace(/\.mdx?$/, '');
        const url = rel === 'index' ? '/' : `/${rel}/`;
        const body = raw.replace(/^---\n[\s\S]*?\n---\n/, '');
        pages.push({ url, meta, body: body.trim(), file: path.relative('.', file) });
}
pages.sort((a, b) => a.url.localeCompare(b.url));

const site = 'pretty-console — the fluent, zero-cost terminal styling toolkit for Rust.';
let llms = `# ${site}\n\n`;
llms += 'Diátaxis-structured documentation: tutorials, how-to guides, reference and explanation.\n\n';
for (const p of pages) {
        llms += `- [${p.meta.title ?? p.url}](${p.url}): ${p.meta.description ?? ''}\n`;
}
await mkdir('public', { recursive: true });
await writeFile('public/llms.txt', llms);

let full = `# ${site}\n\n`;
for (const p of pages) {
        full += `---\n\n<!-- ${p.file} -->\n# ${p.meta.title ?? p.url}\n\n${p.body}\n\n`;
}
await writeFile('public/llms-full.txt', full);
console.log(`llms.txt (${pages.length} pages) and llms-full.txt written`);
