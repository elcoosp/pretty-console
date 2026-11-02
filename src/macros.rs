//! Declarative macros for one-shot styling.
//!
//! - [`paint!`] — build a styled [`Span`] with `format!` text and a list of
//!   style methods
//! - [`style!`] — build a [`Style`] the same way
//! - [`line!`] — build a [`Line`] from spans
//!
//! # Examples
//!
//! ```
//! # #[cfg(not(feature = "no-color"))]
//! # {
//! use pretty_console::{paint, style};
//!
//! let span = paint!(red bold on_black, "count = {}", 42);
//! assert_eq!(span.to_string(), "\x1b[1;38;5;1;48;5;0mcount = 42\x1b[0m");
//!
//! let heading = style!(bold underline bright_blue);
//! assert_eq!(heading.attributes().bits(), 0b1001);
//! # }
//! ```
//!
//! The method list accepts any `Span`/`Style` builder method name: colors
//! (`red`, `bright_blue`), backgrounds (`on_white`), attributes (`bold`,
//! `underline`), or precision setters (`fg_rgb`, `fg_hex` take arguments —
//! use the regular builder chain for those).

/// Builds a [`Span`](crate::Span) from `format!` arguments and style methods.
///
/// ```rust
/// # use pretty_console::paint;
/// let msg = paint!(green bold, "{} tests passed", 12);
/// msg.println();
/// ```
#[macro_export]
macro_rules! paint {
    ($($method:ident)+, $($arg:tt)*) => {{
        #[allow(unused_mut)]
        let mut span = $crate::Span::new(format!($($arg)*));
        $crate::paint!(@apply span $($method)+);
        span
    }};
    (@apply $span:ident) => {};
    (@apply $span:ident $method:ident $($rest:ident)*) => {
        $span = $span.$method();
        $crate::paint!(@apply $span $($rest)*);
    };
}

/// Builds a [`Style`](crate::Style) from style method names.
///
/// ```rust
/// # use pretty_console::style;
/// let danger = style!(bold red on_white);
/// danger.paint("boom").println();
/// ```
#[macro_export]
macro_rules! style {
    ($($method:ident)+) => {{
        #[allow(unused_mut)]
        let mut style = $crate::Style::new();
        $crate::style!(@apply style $($method)+);
        style
    }};
    (@apply $style:ident) => {};
    (@apply $style:ident $method:ident $($rest:ident)*) => {
        $style = $style.$method();
        $crate::style!(@apply $style $($rest)*);
    };
}

/// Builds a [`Line`](crate::Line) from spans (or `&str`s).
///
/// ```rust
/// # use pretty_console::line;
/// let row = line!("cache ", "warm", " · 42 entries");
/// row.println();
/// ```
#[macro_export]
macro_rules! line {
    ($($span:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut line = $crate::Line::new();
        $( line = line.push($span); )*
        line
    }};
}
