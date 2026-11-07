# Visual QA report

Build: production (`pnpm build`), served with `pnpm preview --port 4321`,
inspected with `scripts/qa-screenshots.mjs` (Playwright Chromium) — every
route × 1440×900 and 390×844 × light and dark. `dist/404.html` screenshotted
once, excluded from the matrix per the standard. Gate result: **QA PASSED —
22 routes × 2 viewports × 2 themes** (zero console errors, zero page errors,
zero HTTP ≥ 400, zero horizontal overflow, exactly one `<h1>` per page, no
empty `<main>`, themes applied, no broken images, PNG logo at or below native
size on every page).

Legend: ✅ pass. Every cell is the programmatic probe plus the manual eyeball
where marked.

| Route | desktop-light | desktop-dark | mobile-light | mobile-dark | Notes |
| --- | --- | --- | --- | --- | --- |
| `/` | ✅ eyeballed | ✅ eyeballed | ✅ eyeballed | ✅ | Splash hero + CardGrid, PNG logo in header and hero, violet accent, no redirect |
| `/tutorials/getting-started/` | ✅ eyeballed | ✅ | ✅ | ✅ | `<Steps>` 1–5 with violet counters, `<Tabs>` (cargo add first), `<Aside>` note, `<LinkCard>` next step |
| `/guides/reusable-styles/` | ✅ | ✅ | ✅ | ✅ | Task steps, no teaching narrative; next steps → reference |
| `/guides/status-lines/` | ✅ | ✅ | ✅ | ✅ | Last-span rule explained as recipe; tables/code fit |
| `/guides/exact-colors/` | ✅ | ✅ | ✅ | ✅ | Error-variant table renders; code highlighted |
| `/guides/degrade-colors/` | ✅ | ✅ | ✅ | ✅ | — |
| `/guides/runtime-color-control/` | ✅ | ✅ | ✅ | ✅ | Env table fits; `--color` flag recipe |
| `/guides/decorations/` | ✅ | ✅ eyeballed | ✅ | ✅ | Rust highlighting multi-token, copy buttons, terminal frames on sh blocks; next steps → Panel reference |
| `/guides/text-layout/` | ✅ | ✅ | ✅ | ✅ | — |
| `/guides/migrate-from-v1/` | ✅ | ✅ | ✅ | ✅ | Mechanical-renames table fits |
| `/reference/span/` | ✅ eyeballed | ✅ eyeballed | ✅ eyeballed | ✅ | Longest page; all tables fit, no clipped columns; TOC tracks heading; pagination correct |
| `/reference/style/` | ✅ | ✅ | ✅ | ✅ | — |
| `/reference/color/` | ✅ | ✅ | ✅ | ✅ | — |
| `/reference/line/` | ✅ | ✅ | ✅ | ✅ | — |
| `/reference/panel/` | ✅ | ✅ | ✅ | ✅ | — |
| `/reference/decorations/` | ✅ | ✅ | ✅ | ✅ | — |
| `/reference/text-tools/` | ✅ | ✅ | ✅ | ✅ | Width-range table fits |
| `/reference/color-control/` | ✅ | ✅ | ✅ | ✅ | Two env tables fit; detection order as numbered list |
| `/reference/macros/` | ✅ | ✅ | ✅ | ✅ | `paint!`-style headings render (MD026 configured) |
| `/explanation/zero-cost-design/` | ✅ | ✅ eyeballed | ✅ | ✅ | Discursive register, const code block highlighted, next steps → rendering model |
| `/explanation/rendering-model/` | ✅ | ✅ | ✅ | ✅ | Two-path table renders |
| `/explanation/design-history/` | ✅ | ✅ | ✅ | ✅ | Sourced from docs/DESIGN.md only |
| `dist/404.html` (screenshot once) | — | ✅ eyeballed | — | — | Starlight built-in fallback: clean splash, no draft notice, no stray imports |

## Eyeball checklist result

- [x] Homepage is a splash landing page (hero + `<CardGrid>`), `/` is not a redirect or doc page
- [x] Logo is the project's PNG, sharp, legible in both themes, never upscaled (QA `logo-format` and `logo-upscale` guards pass on all routes)
- [x] Favicon resolves: `dist/favicon.png` (219,913 bytes) served with `<link rel="shortcut icon" href="/favicon.png" type="image/png"/>`
- [x] Brand accent (violet `#7c3aed` / `#c084fc`) on links, active sidebar item, focus rings, step counters — not Starlight blue
- [x] Code blocks syntax-highlighted (multiple token colors), copy button present, long lines scroll internally; terminal window frames on `sh` blocks (Expressive Code default)
- [x] Tables fit on every route, both viewports; no clipped columns
- [x] `<Aside>`, `<CardGrid>`, `<LinkCard>`, `<Tabs>`, `<Steps>` render with correct icons, spacing, borders
- [x] Sidebar: four Diátaxis groups in order, active state correct, nothing duplicated
- [x] On-page TOC present on long pages and highlighting the current heading
- [x] Pagination prev/next point to the adjacent page in sidebar order
- [x] Mobile: header collapses to search + menu icons, no clipping or overlap (overflow probe passes), tap targets usable
- [x] Dark mode: no pure-white panels, no invisible text, code background distinct from page background
- [x] Typography: no overlap, no orphaned headings, nothing hidden under the fixed header
- [x] Search: Pagefind modal opens with Ctrl K; "gradient" returns the decorations guide, its section heading and the reference page
- [x] Type fidelity: no page mixes quadrants (tutorial teaches one path; how-tos assume competence; reference states without directing; explanations argue without instructing)

## Defects found and fixed during QA

1. **Component JSX rendered as literal text on `.md` pages** — Next-steps
   `<LinkCard>` imports appeared as plain paragraphs. Fix: moved every page
   that uses Starlight components to `.mdx` and switched to the barrel import
   `@astrojs/starlight/components`. Re-verified: cards render on all pages.
2. **`<Tabs>`/`<TabItem>` silently dropped on the tutorial** — after the
   `.mdx` move MDX required explicit imports. Fix: import from the barrel.
   Re-verified: tab widget renders.
3. **Draft 404 entry rendered Starlight's draft notice** — a custom 404
   content entry with `draft: true` shows the "This content is a draft…"
   banner on the shipped 404 page; a non-draft one would publish a duplicate
   `/404/` route. Fix: removed the custom entry and shipped Starlight's
   built-in fallback 404; the probe warning is recorded in
   `docs/decisions/0005-known-warnings.md`.
4. **QA route mapper produced an empty slug for `/`** — `path.relative`
   returns `''` (not `'.'`) for `dist/index.html`, so homepage screenshots
   were named `--desktop--light.png` and captured via `//`. Fix:
   `rel === "." || rel === ""`. Re-verified: `index--*.png` captured for `/`.

Re-run rule honored: the QA sequence (`pnpm build` → `pnpm preview` →
`pnpm qa` + eyeball) ran after the last content, config and asset change.
