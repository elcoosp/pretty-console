# 0001 — TypeScript version: pin to 6.x

## Decision

`docs-site` pins `typescript` to `~6.0.3` (the latest 6.x release at build time).

## Context

`astro check` (run by `pnpm check`) does not support TypeScript 7. Running it with
TypeScript 7 installed fails with:

```
astro check does not currently support TypeScript 7.0
```

The registry's current latest TypeScript is `7.0.2` (verified with
`pnpm view typescript version` on 2026-11-08). Without a pin, a fresh install
resolves TypeScript 7 and the quality gate breaks.

## Consequences

- `pnpm check` runs on TypeScript 6.0.3 with zero errors and zero warnings.
- The pin uses `~6.0.3`, so future 6.x patch releases are picked up but the 7.x
  boundary is never crossed.
- When `astro check` gains TypeScript 7 support, this decision is reversed and
  the pin is removed.
