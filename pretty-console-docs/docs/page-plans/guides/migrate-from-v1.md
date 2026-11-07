---
slug: guides/migrate-from-v1
type: how-to
audience: maintainers upgrading a codebase from pretty-console 1.x
goal: move to 2.0 knowing exactly what breaks and what keeps compiling
scope:
  in: Console alias status, canonical attribute ordering and dedup, Style::attributes() returning Effects, new_with_style spelling, contract tests replacing insta
  out: full v2 API (reference/*), the reasoning behind the redesign (explanation/design-history)
outline:
  - Prerequisites
  - Bump the dependency
  - Keep or rename Console
  - Fix exact-output tests
  - Update attributes() consumers
  - Adopt the Span-native spellings
source_refs:
  - CHANGELOG.md (2.0.0 breaking changes)
  - README.md (migrating from 1.x)
  - src/span.rs (Console alias, canonical ordering)
  - docs/DESIGN.md (2.4 compatibility)
approved_by: self (autonomous pipeline)
