---
slug: explanation/rendering-model
type: explanation
audience: developers who need to predict what their output will look like in logs and tests
goal: understand why string rendering is deterministic while printing honors the environment, and how ordering and idempotence follow
scope:
  in: render/Display path vs print path, ColorChoice scope, plain-style rendering, canonical SGR ordering, idempotent attributes, no-color feature behavior
  out: print-family signatures (reference/span)
outline:
  - Two paths, two contracts
  - What determinism buys you
  - The print family's environment contract
  - Canonical ordering and idempotence
  - Plain styles emit nothing
  - The no-color feature
source_refs:
  - src/lib.rs (rendering determinism, style ordering docs)
  - src/span.rs (write_body, write_honoring_choice)
  - src/style.rs (write_start, is_plain)
  - README.md (rendering note)
approved_by: self (autonomous pipeline)
