# 0004 — Visual QA: screenshot and eyeball gate

## Decision

Every built route is screenshot at 1440×900 and 390×844 in both light and dark
themes (`scripts/qa-screenshots.mjs`, Playwright Chromium) and a human-style
eyeball pass inspects the images before release. The gate is blocking:
`pnpm qa` exits non-zero on any console error, page error, HTTP ≥ 400,
horizontal overflow, missing or duplicate `<h1>`, near-empty `<main>`,
theme-application failure, broken image, non-PNG logo, or a logo rendered
larger than its native pixels.

## Route enumeration

Routes are enumerated by walking `dist/` for `index.html` — never
hand-maintained. `dist/404.html` is screenshot once but excluded from the
eyeball matrix.

## Theme application

Each context sets `localStorage['starlight-theme']` before navigation and the
probe asserts `document.documentElement.dataset.theme` matches; if the inline
prefers-color-scheme path wins, the probe clicks the theme toggle and
re-checks.

## Optional axe pass

`@axe-core/playwright` was considered and omitted: the QA script already
asserts the structural properties axe would flag first (empty content,
missing headings, overflow), and the manual pass covers contrast on the
accent tokens (computed values recorded in `0002-logo-format.md`). Adding axe
remains a listed follow-up if the site grows dynamic components.

## Process

1. `pnpm build` (also runs starlight-links-validator)
2. `pnpm preview --port 4321`
3. `pnpm qa`
4. Eyeball: homepage, one page per section, longest page — both viewports,
   both themes
5. Record results in `docs/qa-report.md`

Any config, CSS, logo, favicon, theme or content change after a pass
invalidates it; the full sequence re-runs before packaging.
