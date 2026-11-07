---
slug: reference/color-control
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the runtime color control surface and the exact environment-variable precedence
scope:
  in: ColorChoice, ColorLevel, set_color_choice, color_choice, is_color_enabled (cache), supports_color_level (detection order), environment variables
  out: flag-wiring recipe (guides/runtime-color-control), degradation recipe (guides/degrade-colors)
outline:
  - Overview
  - ColorChoice
  - Setting and reading the choice
  - is_color_enabled and the cache
  - supports_color_level detection order
  - Environment variable tables
source_refs:
  - src/choice.rs (primary)
approved_by: self (autonomous pipeline)
