---
slug: reference/span
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up any Span, Styled or Console member with signature, semantics and defaults
scope:
  in: Span struct and every public method (constructors, text/style accessors, style builders, level helpers, banner, link API, render family, print family), Styled trait surface, Console alias, trait impls and derives
  out: Style/Effects internals (reference/style), tutorials, why-methods (explanation/*)
outline:
  - Overview
  - Constructors
  - Text and style accessors
  - Style builder methods (tables)
  - Level helpers
  - Banner
  - Hyperlinks
  - Rendering
  - Printing
  - Styled extension trait
  - Derives and trait impls
source_refs:
  - src/span.rs (primary)
  - src/lib.rs (re-exports)
approved_by: self (autonomous pipeline)
