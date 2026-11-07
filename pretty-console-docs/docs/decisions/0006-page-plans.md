# 0006 — Page plans

Every documentation page has a binding plan in `docs/page-plans/` containing
`type`, `audience`, `goal`, `scope.in`, `scope.out`, `outline` and
`source_refs`, written before the page was drafted. This record aggregates the
final set.

| # | Page | Type | Goal |
| --- | --- | --- | --- |
| 1 | `/` (splash homepage) | landing (no Diátaxis type) | Route the reader into the correct quadrant within one screen |
| 2 | tutorials/getting-started | tutorial | Install the crate and print styled spans and one multi-segment line, seeing results at each step |
| 3 | guides/reusable-styles | how-to | Define a style once, apply it to many strings, merge styles correctly |
| 4 | guides/status-lines | how-to | Compose one terminal row from differently styled segments |
| 5 | guides/exact-colors | how-to | Apply exact RGB colors from packed values or hex strings, pick readable foregrounds |
| 6 | guides/degrade-colors | how-to | Detect terminal capability and render degraded colors |
| 7 | guides/runtime-color-control | how-to | Control whether printed output carries colors at runtime |
| 8 | guides/decorations | how-to | Produce gradients, rainbows, titled panels and OSC 8 links |
| 9 | guides/text-layout | how-to | Measure, truncate and pad strings exactly as the terminal renders them |
| 10 | guides/migrate-from-v1 | how-to | Move a 1.x codebase to 2.0 knowing exactly what breaks |
| 11 | reference/span | reference | Look up any Span/Styled/Console member with signature and semantics |
| 12 | reference/style | reference | Look up Style, Attribute, Effects and EffectIter members |
| 13 | reference/color | reference | Look up the Color model, parsing, color math and SGR emission |
| 14 | reference/line | reference | Look up Line members and last-span semantics |
| 15 | reference/panel | reference | Look up Panel members, defaults and width computation |
| 16 | reference/decorations | reference | Look up gradient/rainbow signatures and interpolation details |
| 17 | reference/text-tools | reference | Look up ANSI-aware text functions and edge-case behavior |
| 18 | reference/color-control | reference | Look up the runtime color control surface and env precedence |
| 19 | reference/macros | reference | Look up paint!, style! and line! syntax and constraints |
| 20 | explanation/zero-cost-design | explanation | Understand how styled output renders without heap allocation or syscalls |
| 21 | explanation/rendering-model | explanation | Understand deterministic string rendering vs environment-honoring printing |
| 22 | explanation/design-history | explanation | Understand the v1 problems, v2 decisions and non-goals |

Total: 22 pages (1 landing + 1 tutorial + 8 how-to + 9 reference + 3
explanation). Documentation debt identified in Phase 1 and its resolution:

- The README mixes all four quadrants in one file → decomposed above.
- `docs/DESIGN.md` was repository-internal prose → distilled into
  explanation/design-history with citation.
- No dedicated docs site existed → this site.
- No light/dark logo variants → resolved per `0002-logo-format.md`.
