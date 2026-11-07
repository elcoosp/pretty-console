---
slug: guides/degrade-colors
type: how-to
audience: developers whose output must survive on 256-color or 16-color terminals (CI logs, ssh, IDE consoles)
goal: detect the terminal capability level and render degraded colors accordingly
scope:
  in: ColorLevel, supports_color_level, Span::render_with, Line::render_with, Color::degrade_to/to_256/to_16
  out: turning colors fully off (guides/runtime-color-control), degradation math (reference/color)
outline:
  - Prerequisites
  - Detect the terminal capability
  - Render a span or line at a level
  - Degrade a single color for a palette swatch
  - Produce stable output for CI logs
source_refs:
  - src/choice.rs (supports_color_level, ColorLevel)
  - src/color.rs (degrade_to, to_256, to_16)
  - src/span.rs (render_with)
  - src/line.rs (render_with)
  - README.md (adaptive colors section)
approved_by: self (autonomous pipeline)
