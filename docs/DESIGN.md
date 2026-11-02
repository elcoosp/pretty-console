# pretty-console v2 — Design Document

Status: **implemented** (this document is the record of the brainstorm and the
decisions that shaped the 2.0 rewrite).

## 1. Deep-dive findings on v1.1.0

### Performance audit

| # | Finding | Cost |
| --- | --- | --- |
| P1 | `Style.attributes: Vec<Attribute>` | Heap allocation per style; every builder call clones the `Vec` (another alloc) |
| P2 | `Color::fg_code`/`bg_code` return `String` | One alloc per color, per render |
| P3 | `Style::to_ansi_start` builds a `Vec<String>` then `join`s | ≥ 3 allocs per styled render |
| P4 | `Console::render` does `format!` + `text.clone()`; `Display` renders through that temporary | 2 extra allocs even when writing straight into a formatter |
| P5 | `is_color_enabled()` re-reads env vars on **every** print | Syscalls per line printed |
| P6 | `Style` is neither `Copy` nor `const`-constructible | `.clone()` tax everywhere; no static style tables |

### DX audit (vs owo-colors, yansi, colored, anstyle, termcolor)

| # | Gap |
| --- | --- |
| D1 | One styled string per value — no multi-segment composition (a `Line` with differently-styled spans), the #1 thing CLI authors need |
| D2 | No hyperlinks (OSC 8) |
| D3 | No gradients / rainbow text |
| D4 | No terminal capability detection (`COLORTERM`, `FORCE_COLOR` levels) and no color *degradation* (truecolor → 256 → 16) |
| D5 | No hex-string parsing (`"#7c4dff"`) |
| D6 | No ANSI-aware text toolkit: visible width (CJK-aware), truncate-with-ellipsis, pad/align — every TUI needs these and gets them from heavyweight deps |
| D7 | No conditional styling (`when`) — forces `if let` noise in chains |
| D8 | No `paint!` / `line!` macros |
| D9 | No panels / boxed output |
| D10 | No contrast helper (pick a readable foreground) |
| D11 | `print` panics on broken pipe; no `try_*` variants; no `eprint*` at all |
| D12 | `Display` padding counts `chars()` — wrong for wide (CJK) glyphs |
| D13 | Duplicate attributes emit duplicated SGR codes (`bold().bold()` → `1;1`) |
| D14 | Naming: `Console` is not a console, it is a styled string |

## 2. Decisions

### 2.1 Core model

- **`Color`** stays a `Copy` enum (`Named(u8)` palette index, `RGB`) with the
  same 16 constants. Adds: hex-string parsing, luminance, `readable_on`,
  `lerp`, degradation (`to_256`, `to_16`, `degrade_to`), SGR writing **into a
  formatter** (no `String` intermediates).
- **`Effects`** — new `u16` bitset over the 8 attributes. Replaces the
  `Vec<Attribute>`. `Copy`, `const` everything, idempotent adds, canonical
  emission order (bold → strikethrough). `Attribute` remains as the enum for
  ergonomics/iteration.
- **`Style`** — `Copy`, `const`-constructible, zero heap. Builder methods like
  v1, `merge` now unions the bitset (no duplicates). Renders directly into
  `fmt::Write`.
- **`Span`** — the new primary type (replaces `Console`, which is kept as a
  type alias so most v1 user code still compiles): text + `Style` + optional
  OSC 8 hyperlink. `Display` writes start/text/reset **without building an
  intermediate `String`**. Width-aware `Display` padding uses visible (CJK
  aware) width.
- **`Line`** — ordered `Vec<Span>`; the composition primitive. Style methods
  apply to the *last* pushed span, so
  `Line::new().push("ok ").push("build").green()` reads naturally.
- Deterministic rendering philosophy kept: `to_string`/`Display` never consult
  the environment; only the `print*`/`write_to` family honors `ColorChoice`.

### 2.2 New capabilities (the "beat competitors" list)

| Feature | API |
| --- | --- |
| Multi-span lines | `Line`, `line!` |
| Hyperlinks | `Span::link(url)` (OSC 8) |
| Gradients | `gradient`, `gradient_bg`, `rainbow`, `rainbow_bg` |
| Capability detection | `ColorLevel` (`Ansi16`/`Ansi256`/`TrueColor`), `supports_color_level()`, honors `FORCE_COLOR=1/2/3` Node-style |
| Color degradation | `Color::to_256`, `Color::to_16`, `Span::render_with(level)`, `Line::render_with(level)` |
| Hex strings | `Color::from_hex_str("#7c4dff")`, `fg_hex`/`bg_hex` |
| ANSI-aware text kit | `visible_width`, `char_width`, `truncate(input, width, tail)`, `pad(input, width, align, fill)`, `Align` |
| Conditional styling | `Style::when(cond, f)`, `Span::when(cond, f)` |
| Macros | `paint!`, `style!`, `line!` |
| Panels | `Panel` builder (rounded box, inlaid title, padding, fixed width) |
| Contrast | `Color::luminance`, `Color::readable_on` |
| Stderr + fallible IO | `eprint`, `eprintln`, `try_print*`, `try_eprint*` |
| Cached env detection | detection result cached; zero syscalls on the hot print path |

### 2.3 Perf plan

1. `Style`/`Effects` are `Copy` — builder chains compile to register moves.
2. SGR emission via `fmt::Write` with hand-rolled digit writing: zero
   intermediate allocations in the `Display` path.
3. `strip_ansi` rewritten as a byte scanner (memchr-style ESC search, slice
   copies, full-char handling for non-ASCII) instead of `char`-by-`char`.
4. Global color decision cached in atomics; invalidated by `set_color_choice`.
5. `truncate`/`pad` operate with visible width in a single pass.

### 2.4 Compatibility

- MSRV stays **1.85**, edition 2024, `#![forbid(unsafe_code)]`, zero runtime
  dependencies.
- `Console = Span` alias, `Console::new_with_style`, `banner`, level helpers
  and the whole `Styled` trait keep their v1 signatures.
- **Breaking** (documented, allowed): attribute emission order is now
  canonical (bit order) instead of insertion order; `Style` internals changed
  (`attributes()` returns an iterator of a bitset view — the accessor is kept
  with the same `&[Attribute]`-free signature change noted in CHANGELOG);
  snapshot tests replaced by exact-string contract tests (insta dev-dep
  dropped).

### 2.5 Non-goals

- Window-size queries, cursor control, alternate screens (terminal-control
  crates' territory).
- Unicode width parity with the `unicode-width` crate (our table covers the
  common CJK/emoji/combining ranges with zero deps; documented approximation).
