---
slug: guides/reusable-styles
type: how-to
audience: CLI authors who repeat the same styling in many places
goal: define a style once, apply it to many strings, and merge styles correctly
scope:
  in: Style::new in const context, Style::paint, Span::with_text, merge precedence, From<Color>/From<Attribute>/From<Effects>, Style::when
  out: multi-segment lines (guides/status-lines), full member tables (reference/style), design rationale (explanation/zero-cost-design)
outline:
  - Prerequisites
  - Define a style once
  - Apply it to different text
  - Merge two styles
  - Convert colors and attributes into styles
  - Adjust conditionally with when
source_refs:
  - src/style.rs (Style, merge, paint, when, From impls)
  - src/span.rs (with_text)
  - examples/advanced.rs (predefined styles, merging)
approved_by: self (autonomous pipeline)
