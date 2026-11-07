---
slug: tutorials/getting-started
type: tutorial
audience: Rust developers who have never used pretty-console
goal: install the crate in a fresh project and print styled spans and one multi-segment line, seeing visible results at each step
scope:
  in: cargo new, cargo add pretty-console, Span builders, Styled literals, Line composition, println, cargo run, expected output
  out: reusable Style templates (guides/reusable-styles), runtime color control (guides/runtime-color-control), why rendering is deterministic (explanation/rendering-model)
outline:
  - What you will build
  - Prerequisites
  - Create a project
  - Add the crate
  - Style a single span
  - Style a literal in place
  - Compose a multi-segment line
  - Run and confirm the result
  - Next steps
source_refs:
  - Cargo.toml (crate name, version, examples)
  - README.md (quick start)
  - src/span.rs (Span::new, println, Styled)
  - src/line.rs (Line::new, push)
  - examples/demo.rs (vocabulary)
approved_by: self (autonomous pipeline)
