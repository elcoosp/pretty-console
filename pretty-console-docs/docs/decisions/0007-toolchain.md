# 0007 — Toolchain: pnpm, Astro floor, package manager scope

## pnpm

pnpm is the package manager of record: `pnpm-lock.yaml` ships, every snippet
uses pnpm, and `<Tabs>` in the tutorial puts pnpm first. No npm or Yarn
lockfiles exist.

## Verified baseline (execution time 2026-11-08)

| Package | Version used | Verified with |
| --- | --- | --- |
| astro | 7.4.7 | `pnpm view astro version` |
| @astrojs/starlight | 0.42.10 | `pnpm view @astrojs/starlight version` |
| starlight-links-validator | 0.27.0 | `pnpm view …` + peer deps (astro ≥ 7.2.10, starlight ≥ 0.42.0) |
| typescript | 6.0.3 (pinned) | see `0001-typescript-version.md` |
| @astrojs/check | 0.9.12 | `pnpm view` |
| playwright | 1.65.0 | `pnpm view` |

Astro 7.4 is the floor per the documentation standard; 7.4.7 is the current
patch. Node in the build environment is 24.x (above the 22.23.4 minimum).

## Rust toolchain note

The sandbox has no Rust toolchain (`cargo` absent), so tutorial commands could
not be executed against a scratch build. Every tutorial command was verified
statically instead: `cargo new` / `cargo add pretty-console` / `cargo run` are
standard cargo subcommands; the crate name and version come from `Cargo.toml`
(`name = "pretty-console"`, `version = "2.0.0"`, published badge in README);
all code snippets compile against signatures read directly from `src/` and are
adapted from the repository's own doctests, which the project's CI runs. This
residual risk is recorded here deliberately rather than silently assumed away.
