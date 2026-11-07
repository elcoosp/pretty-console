---
slug: reference/style
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up Style, Attribute, Effects and EffectIter members and their exact semantics
scope:
  in: Style methods and merge rule, Attribute variants with SGR codes, Effects bitset operations, EffectIter, From impls
  out: Span methods (reference/span), const-context rationale (explanation/zero-cost-design)
outline:
  - Overview
  - Style methods
  - Merge semantics
  - Attribute variants and SGR codes
  - Effects operations
  - EffectIter
source_refs:
  - src/style.rs (primary)
  - src/attr.rs (primary)
approved_by: self (autonomous pipeline)
