---
slug: reference/text-tools
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the ANSI-aware text functions, their parameters and edge-case behavior
scope:
  in: strip_ansi (sequence grammar), char_width (range table summary), visible_width, truncate (tail/reset semantics), pad, Align
  out: recipes (guides/text-layout)
outline:
  - Overview
  - strip_ansi
  - char_width
  - visible_width
  - truncate
  - pad and Align
source_refs:
  - src/text.rs (primary)
approved_by: self (autonomous pipeline)
