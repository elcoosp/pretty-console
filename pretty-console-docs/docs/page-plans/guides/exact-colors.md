---
slug: guides/exact-colors
type: how-to
audience: developers matching a design system or parsing colors from configuration
goal: apply exact RGB colors from packed values or hex strings and pick readable foregrounds
scope:
  in: Color::hex, fg_hex/bg_hex, Color::from_hex_str, FromStr, ParseColorError handling, Color::indexed, Color::rgb, readable_on, luminance
  out: degradation to 256/16 colors (guides/degrade-colors), full Color tables (reference/color)
outline:
  - Prerequisites
  - Use a packed hex value
  - Parse a hex string from config or CLI
  - Use 256-palette entries
  - Pick a readable foreground for a background
source_refs:
  - src/color.rs (hex, from_hex_str, ParseColorError, readable_on, luminance)
  - examples/advanced.rs (hex colors, theme accent parsing)
approved_by: self (autonomous pipeline)
