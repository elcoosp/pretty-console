---
slug: guides/runtime-color-control
type: how-to
audience: CLI authors wiring a --color flag or respecting no-color conventions
goal: control whether printed output carries colors at runtime, honoring the standard environment variables
scope:
  in: ColorChoice, set_color_choice, color_choice, is_color_enabled, env precedence table, stderr and try_* printers, broken-pipe handling
  out: capability detection and degradation (guides/degrade-colors), detection internals (reference/color-control), determinism rationale (explanation/rendering-model)
outline:
  - Prerequisites
  - Default behavior
  - The environment variables pretty-console honors
  - Wire a --color flag
  - Print to stderr
  - Handle broken pipes with try_*
source_refs:
  - src/choice.rs (ColorChoice, set_color_choice, is_color_enabled, env_enabled)
  - src/span.rs (print family, write_honoring_choice)
  - README.md (runtime color control tables)
approved_by: self (autonomous pipeline)
