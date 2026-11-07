//! A fluent, zero-dependency API for styling terminal text with ANSI colors,
//! attributes, gradients, panels, hyperlinks and ANSI-aware text tools.
//!
//! Write beautifully formatted console output with an intuitive, chainable
//! interface. The crate has no runtime dependencies and compiles down to plain
//! ANSI escape sequences, so it works on every terminal that understands ANSI
//! codes (Windows 10+ included).
//!
//! # Highlights
//!
//! - **16 named colors**, the full **256-color** palette and **24-bit RGB**
//! - **Text attributes**: bold, dim, italic, underline, blink, reverse,
//!   hidden, strikethrough — stored in a [`Copy`] [`Effects`] bitset
//! - **Zero-allocation rendering**: styles are `Copy`, and [`Display`] writes
//!   SGR codes straight into the destination formatter
//! - **Reusable styles**: build a [`Style`] once (even in `const` context),
//!   apply it to many strings
//! - **Composition**: [`Line`] joins differently-styled [`Span`]s into one
//!   row — the multi-segment output CLIs actually need
//! - **Decorations**: [`gradient`], [`rainbow`], [`Panel`] boxes and OSC 8
//!   hyperlinks ([`Span::link`])
//! - **Adaptive colors**: detect terminal capability with
//!   [`supports_color_level`] and degrade truecolor output via
//!   [`Span::render_with`](Span::render_with)
//! - **ANSI-aware text tools**: [`strip_ansi`], [`visible_width`],
//!   [`truncate`], [`pad`], [`wrap`]
//! - **Word wrapping**: [`wrap`], [`Span::wrap`](Span::wrap) and
//!   [`Line::wrap`](Line::wrap) reflow styled, CJK-aware text without
//!   leaking ANSI state across lines; [`Panel::soft_wrap`](Panel::soft_wrap)
//!   reflows fixed-width panels
//! - **Runtime color control**: honor `NO_COLOR`, `CLICOLOR`,
//!   `CLICOLOR_FORCE`, `FORCE_COLOR`, `TERM=dumb` and `COLORTERM` via
//!   [`ColorChoice`], with the detection result cached
//! - **Log-level helpers**: [`Span::info`], [`Span::warn`],
//!   [`Span::error`], [`Span::success`] and [`Span::debug`]
//! - **Macros**: [`paint!`], [`style!`] and [`line!`]
//! - **Optional `no-color` feature**: compile out all styling entirely
//!
//! # Quick start
//!
//! ```
//! # #[cfg(not(feature = "no-color"))]
//! # {
//! use pretty_console::{Line, Span, Styled};
//!
//! // Chainable builder API.
//! let error = Span::new("Error: file not found").red().bold();
//! assert_eq!(error.to_string(), "\x1b[1;38;5;1mError: file not found\x1b[0m");
//!
//! // Extension trait for string literals.
//! let ok = "Success!".green();
//! assert_eq!(ok.to_string(), "\x1b[38;5;2mSuccess!\x1b[0m");
//!
//! // Reusable style templates.
//! let heading = Span::new("").cyan().bold().underline();
//! assert_eq!(
//!     heading.with_text("Chapter 1").to_string(),
//!     "\x1b[1;4;38;5;6mChapter 1\x1b[0m"
//! );
//!
//! // Compose a multi-segment line.
//! let line = Line::new()
//!     .push("build ").dim()
//!     .push("done in 42ms").green().bold();
//! # }
//! ```
//!
//! # Rendering determinism
//!
//! String rendering ([`to_string`](Span::render) and `Display`) is
//! deterministic and only affected by the compile-time `no-color` feature, so
//! formatted output never silently changes depending on where it is built.
//! Only the printing methods (`print`, `println`, `eprint`, `eprintln`,
//! `try_*` and `write_to`) honor the runtime [`ColorChoice`].
//!
//! # Style ordering
//!
//! Within an ANSI sequence, attributes are emitted first (in canonical
//! bold → strikethrough order), then the foreground color, then the
//! background color. This ordering is stable and covered by contract tests.

#![cfg_attr(docsrs, feature(doc_cfg))]
// Color math and SGR emission work in small, test-covered integer ranges (u8
// components, palette indices, f32 interpolation), so the blanket cast lints
// are relaxed for the whole crate.
#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

pub mod attr;
pub mod choice;
pub mod color;
pub mod decorate;
pub mod line;
#[macro_use]
pub mod macros;
pub mod panel;
pub mod span;
pub mod style;
pub mod text;

pub use attr::{Attribute, EffectIter, Effects};
pub use choice::{
    ColorChoice, ColorLevel, color_choice, is_color_enabled, set_color_choice, supports_color_level,
};
pub use color::{Color, ParseColorError, palette_rgb};
pub use decorate::{gradient, gradient_bg, hsv_to_rgb, rainbow, rainbow_bg};
pub use line::Line;
pub use panel::Panel;
pub use span::{Console, Span, Styled};
pub use style::Style;
pub use text::{Align, WrapMode, char_width, pad, strip_ansi, truncate, visible_width, wrap};
