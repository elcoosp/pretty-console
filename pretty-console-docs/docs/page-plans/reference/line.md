---
slug: reference/line
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up Line members and the exact last-span styling semantics
scope:
  in: Line struct and all methods (construction, push family, accessors, with_style, map_span, when, last-span builders, render family, print family), trait impls
  out: Span internals (reference/span), composition guidance (guides/status-lines)
outline:
  - Overview
  - Construction
  - Pushing and editing spans
  - Whole-line operations
  - Rendering and printing
  - Derives and trait impls
source_refs:
  - src/line.rs (primary)
approved_by: self (autonomous pipeline)
