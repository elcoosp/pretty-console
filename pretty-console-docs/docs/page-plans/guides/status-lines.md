---
slug: guides/status-lines
type: how-to
audience: CLI authors printing progress and summary rows
goal: compose one terminal row from differently styled segments without string concatenation
scope:
  in: Line::new/push/extend/from_spans, the last-span styling rule, push_mut in loops, with_style, visible_width, format-width alignment
  out: macro one-liners detail (reference/macros), Line member tables (reference/line)
outline:
  - Prerequisites
  - Push segments left to right
  - Understand the last-span rule
  - Build a line from an iterator
  - Restyle every span at once
  - Accumulate in a loop
  - Align a whole line with format widths
source_refs:
  - src/line.rs (Line, edit_last, with_style, Display)
  - src/span.rs (fmt_with visible-width padding)
  - examples/showcase.rs
approved_by: self (autonomous pipeline)
