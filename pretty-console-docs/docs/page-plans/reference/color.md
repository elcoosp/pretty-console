---
slug: reference/color
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the Color model, parsing, color math and SGR emission
scope:
  in: Color enum and 16 constants, indexed/rgb/hex constructors, from_hex_str and ParseColorError, fg_code/bg_code, rgb_value, luminance, readable_on, lerp, to_256, to_16, degrade_to, level, palette_rgb, From/FromStr impls
  out: capability detection (reference/color-control), degradation guidance (guides/degrade-colors)
outline:
  - Overview
  - Constants and constructors
  - Parsing hex strings
  - SGR parameter accessors
  - Color math (luminance, readable_on, lerp)
  - Degradation (to_256, to_16, degrade_to, level)
  - palette_rgb
source_refs:
  - src/color.rs (primary)
approved_by: self (autonomous pipeline)
