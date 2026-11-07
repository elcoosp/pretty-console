---
slug: guides/decorations
type: how-to
audience: CLI authors building banners, report boxes or clickable output
goal: produce gradient and rainbow text, rounded panels with titles, and OSC 8 hyperlinks
scope:
  in: gradient/gradient_bg, rainbow/rainbow_bg, Panel builder (title, line/lines, width, padding, border), fixed-width truncation, Span::link/unlink/url
  out: function signatures (reference/decorations, reference/panel), span member tables (reference/span)
outline:
  - Prerequisites
  - Fade a foreground gradient across text
  - Paint a rainbow
  - Build a titled panel
  - Fix the panel width (and what happens to wide lines)
  - Make text clickable with OSC 8
source_refs:
  - src/decorate.rs (gradient, gradient_bg, rainbow, rainbow_bg)
  - src/panel.rs (Panel builder, render, content_width)
  - src/span.rs (link, unlink, url)
  - examples/showcase.rs, examples/advanced.rs
approved_by: self (autonomous pipeline)
