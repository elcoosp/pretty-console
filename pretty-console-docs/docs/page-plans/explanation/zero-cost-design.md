---
slug: explanation/zero-cost-design
type: explanation
audience: Rust developers evaluating the crate or curious about its performance model
goal: understand how the crate renders styled output without heap allocation or repeated syscalls
scope:
  in: Copy styles and const construction, Effects bitset, direct fmt::Write emission, hand-rolled digit writing, cached environment detection, single-pass visible width
  out: how to use the API (tutorials, guides), measured benchmark tables (README)
outline:
  - What zero-cost means here
  - Styles are values, not builders with state
  - The Effects bitset
  - Writing codes straight into the formatter
  - One environment decision, cached
  - Visible width in a single pass
  - What this does not cover
source_refs:
  - docs/DESIGN.md (perf plan)
  - src/style.rs, src/attr.rs, src/color.rs (write_u8), src/choice.rs, src/text.rs
approved_by: self (autonomous pipeline)
