# 0003 — Syntax highlighter: Shiki (the Starlight default)

## Decision

The site ships Shiki through Expressive Code with no custom configuration.
Rangi is not adopted.

## Why

Starlight's code blocks are powered by Expressive Code, whose supported
highlighter is Shiki: TextMate grammars, 180+ languages, build-time rendering
with zero client JavaScript, and native dual-theme integration with
Starlight's light/dark modes.

Rangi's own published comparison positions it differently:

| | Rangi | Shiki |
| --- | --- | --- |
| Bundle size | ~13 kB all-in | Larger (grammars) |
| Lexing | Regex tokenizer | TextMate grammars |
| Theming | Built-in themes / pairs | Dual themes + CSS vars |
| Transformers / Twoslash | No | Yes |
| Language coverage | ~46 | 180+ |
| Best for | Chat UIs, SSG, small apps | Docs sites, rich code samples |

A documentation site is the case Shiki is designated for. Rangi has no
Expressive Code integration; adopting it would mean maintaining an unsupported
custom highlighter and losing transformers, Twoslash and language coverage.

## Configuration

None beyond the Starlight defaults. The docs only need `rust`, `bash`,
`toml`, `text` and occasional `ansi` — all covered by the defaults, and the
default dual-theme setup already matches both Starlight color schemes.

## Related tooling decisions

- `starlight-links-validator` (0.27.0, peer-compatible with astro ≥ 7.2.10 and
  starlight ≥ 0.42.0) is enabled; broken internal links fail `pnpm build`.
- The remaining HiDeoo plugins were evaluated and omitted:
  - `starlight-typedoc` — generates API docs from TypeScript; this project is
    Rust, and docs.rs is the canonical API reference. N/A.
  - `starlight-blog` — no changelog section requested; CHANGELOG.md lives in
    the repository. Omitted.
  - `starlight-openapi` — no OpenAPI spec. N/A.
  - `starlight-image-zoom` — the site has no content images beyond the logo.
    Omitted.
  - `starlight-heading-badges` — no stability badges to convey yet (v2.0.0 is
    the only line). Omitted.
  - `starlight-versions` — single documented version (2.0.0). Omitted.
