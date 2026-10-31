<div align="center">
  <img src="docs/logo.png" alt="pretty-console Logo" width="200"/>
  <p>
    <strong>A fluent, zero-cost API for styling terminal text with colors and attributes, written in Rust.</strong><br/>
    Zero runtime dependencies, a chainable builder, reusable styles, 256-color and true-color support, log-level helpers and runtime color detection — everything compiles down to plain ANSI escape codes that work in any modern terminal.
  </p>
  <p>
    <img src="https://img.shields.io/badge/Rust-1.85%2B%20%7C%202024-000000?style=flat-square&logo=rust" alt="Rust"/>
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License MIT"/>
    <img src="https://img.shields.io/crates/v/pretty_console?style=flat-square" alt="Crates.io"/>
    <img src="https://img.shields.io/docsrs/pretty_console?style=flat-square" alt="Docs.rs"/>
    <img src="https://img.shields.io/badge/Deps-Zero-brightgreen?style=flat-square" alt="Zero dependencies"/>
    <img src="https://img.shields.io/badge/Coverage-100%25-brightgreen?style=flat-square" alt="Coverage 100%"/>
    <img src="https://img.shields.io/badge/Build-Passing-brightgreen?style=flat-square&logo=githubactions" alt="Build"/>
  </p>
</div>

---

`pretty-console` makes terminal output pleasant to write and to read. Build styled strings with a fluent API, reuse them as templates, paint string literals directly with the [`Styled`] trait, and let the runtime honor standard conventions like `NO_COLOR` — all without pulling in a single dependency.

## Features

- **Fluent builder** — chain colors and attributes: `Console::new("hi").red().bold()`
- **Rich colors** — 16 named colors, the full 256-color palette, 24-bit RGB and `0xRRGGBB` hex
- **Text attributes** — bold, dim, italic, underline, blink, reverse, hidden, strikethrough
- **`Styled` trait** — paint literals in place: `"hello".green().underline()`
- **Reusable styles** — build a `Style` once, apply it to many strings, or merge styles together
- **Log-level helpers** — `info`, `warn`, `error`, `success` and `debug` constructors with tasteful defaults
- **Banners** — one-liner centered section headers for CLIs
- **Runtime color control** — `NO_COLOR`, `CLICOLOR`, `CLICOLOR_FORCE`, `FORCE_COLOR` and `TERM=dumb` support out of the box
- **ANSI stripping** — `strip_ansi` removes escape sequences from any string
- **Width-aware formatting** — `format!("{:>20}", styled)` pads by *visible* width, not by ANSI-decorated length
- **`no-color` feature** — compile out all styling entirely for embedded or minimal builds

## Quick start

```sh
cargo add pretty-console
```

```rust
use pretty_console::{Console, Styled};

fn main() {
    Console::new("Hello, world!").red().bold().println();
    Console::new("This is a warning").yellow().underline().println();
    Console::new("Success!").green().on_black().println();

    // Or paint a literal directly.
    println!("{}", "Ready to go".cyan());
}
```

## Usage

### Builder API and reusable styles

```rust
use pretty_console::Console;

// Simple color and attribute combinations.
Console::new("Error message").red().bold().println();

// A style template you can reuse with different text.
let error_style = Console::new("").red().bold();
error_style.with_text("Error: file not found").println();
error_style.with_text("Error: permission denied").println();
```

### Predefined styles, merging and hex colors

```rust
use pretty_console::{Console, Style, Color};

let heading = Style::new().bold().underline().fg(Color::BRIGHT_BLUE);
Console::new_with_style("Chapter 1", heading.clone()).println();

// Merge styles: the second style wins for colors, attributes are appended.
let emphasis = Style::new().fg(Color::RED);
Console::new_with_style("Important", heading.merge(emphasis)).println();

// True color, straight from hex.
Console::new("Violet").fg(Color::hex(0x7C_4D_FF)).println();
```

### Log-level helpers and banners

```rust
use pretty_console::Console;

Console::info("Compiling 42 modules").println();     // [INFO] …, cyan
Console::success("Tests passed").println();          // [OK] …, bold green
Console::warn("Deprecated API").println();           // [WARN] …, yellow
Console::error("Disk full").println();               // [ERROR] …, bold red
Console::debug("cache_size = 42").println();         // [DEBUG] …, dim

Console::banner("Build report", 60).println();       // ────── Build report ──────
```

### Runtime color control

Printing respects the global [`ColorChoice`], which you can set from your own CLI flags:

```rust
use pretty_console::{set_color_choice, ColorChoice};

// e.g. after parsing a --color flag from your CLI:
set_color_choice(ColorChoice::Never);
```

| Choice | Behavior |
| --- | --- |
| `ColorChoice::Auto` (default) | Honors the environment variables below, then falls back to stdout terminal detection |
| `ColorChoice::Always` | Always emit colors |
| `ColorChoice::Never` | Never emit colors |

| Variable | Effect |
| --- | --- |
| `FORCE_COLOR` | Set to a non-empty value other than `0` to force colors on; `FORCE_COLOR=0` forces them off |
| `CLICOLOR_FORCE` | Set to a non-empty value other than `0` to force colors on |
| `NO_COLOR` | Set to a non-empty value to disable colors |
| `CLICOLOR` | Set to `0` to disable colors |
| `TERM` | `dumb` disables colors |

> [!NOTE]
> String rendering (`to_string`, `Display`) is deterministic and never consults
> the environment or a terminal — only `print`, `println` and `write_to` do.
> Formatted output is identical whether it is produced on a CI runner or in an
> interactive terminal.

### Stripping ANSI codes

```rust
use pretty_console::{strip_ansi, Styled};

let styled = "hello".red().bold();
assert_eq!(strip_ansi(&styled.to_string()), "hello");
```

### The `no-color` feature

For environments where styling is never wanted:

```toml
[dependencies]
pretty-console = { version = "1.1.0", features = ["no-color"] }
```

All ANSI generation is compiled out; every method still exists and behaves as a pass-through.

## Comparison

| Feature | pretty_console | termcolor | colored |
| --- | --- | --- | --- |
| Fluent API | ✅ | ❌ | ✅ |
| True color | ✅ | ✅ | ✅ |
| Zero dependencies | ✅ | ❌ | ✅ |
| Reusable styles | ✅ | ✅ | ❌ |
| Runtime color detection | ✅ | ✅ | ✅ |
| 100% test coverage | ✅ | ❌ | ❌ |

## Performance

Styling is a builder over plain `String` formatting: there are no locks, no regexes and no dynamic dispatch in the render path. ANSI codes are only produced when a style is actually set, and empty styles render as the bare text.

## Platform support

- **Unix-like systems**: full support via ANSI escape codes
- **Windows 10+**: full support through virtual terminal sequences

[`Styled`]: https://docs.rs/pretty_console/latest/pretty_console/trait.Styled.html
[`ColorChoice`]: https://docs.rs/pretty_console/latest/pretty_console/enum.ColorChoice.html
