---
slug: guides/text-layout
type: how-to
audience: TUI and table authors aligning columns in a terminal
goal: measure, truncate and pad strings exactly as the terminal renders them, including styled and CJK strings
scope:
  in: visible_width, char_width, truncate with tail and reset semantics, pad with Align, format-width padding on Span/Line
  out: escape-scanner grammar (reference/text-tools), width-table approximation limits (reference/text-tools)
outline:
  - Prerequisites
  - Measure visible width correctly
  - Truncate to a column budget
  - Pad and align columns
  - Rely on format-width padding for styled values
source_refs:
  - src/text.rs (visible_width, char_width, truncate, pad, Align)
  - src/span.rs (fmt_with, Display)
  - src/line.rs (fmt_with, Display)
approved_by: self (autonomous pipeline)
