# AGENTS.md — working rules for this documentation site

This is the Astro Starlight documentation site for
[`pretty-console`](https://github.com/elcoosp/pretty-console), a Rust crate.
The information architecture is
[Diátaxis](https://diataxis.fr/): one page, one type.

## Layout

- `src/content/docs/` — all pages. `index.mdx` is the splash homepage (not a
  Diátaxis page). Quadrant folders: `tutorials/`, `guides/`, `reference/`,
  `explanation/`.
- `docs/page-plans/` — the binding plan for every page (type, audience, goal,
  scope, outline, `source_refs`). Revise the plan, then the page.
- `docs/decisions/` — ADRs (TypeScript pin, logo handling, highlighter,
  visual QA, page-plan aggregate, toolchain).
- `docs/qa-report.md` — the visual QA record of the last passing build.
- `scripts/qa-screenshots.mjs` — the Playwright screenshot gate.

## Commands

```sh
pnpm install        # once
pnpm dev            # local dev server
pnpm build          # production build; starlight-links-validator fails on broken internal links
pnpm check          # astro check (TypeScript is pinned to 6.x — see docs/decisions/0001)
pnpm lint           # markdownlint-cli2 over src/content/docs
pnpm qa             # screenshots every route × 2 viewports × 2 themes (needs `pnpm preview` on :4321)
```

## Content rules

- Never mix Diátaxis types: a tutorial teaches one path with `<Steps>`; a
  how-to assumes competence; a reference states, it does not instruct; an
  explanation argues, it does not direct.
- Every claim about crate behavior must be traceable to `src/` in the
  upstream repository. When README and code disagree, the code wins — record
  the discrepancy in `docs/decisions/`.
- `type` frontmatter (`tutorial | how-to | reference | explanation`) is
  mandatory on every page except the homepage.
- Every fenced code block declares a language. Package-manager tabs put pnpm
  first.
- Voice: second person, present tense, active, imperative for instructions.
  Never "simply", "just", "easy", "obviously".

## Release gate

`pnpm build`, `pnpm check` and `pnpm lint` emit zero warnings;
`pnpm qa` exits 0 across all routes; the eyeball pass in `docs/qa-report.md`
is complete. Re-run everything after any change, before packaging.
