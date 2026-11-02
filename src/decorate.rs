//! Decorative rendering: color gradients and rainbow text.
//!
//! # Examples
//!
//! ```
//! use pretty_console::{gradient, Color};
//!
//! let fade = gradient("pretty console", Color::hex(0xFF3E6C), Color::hex(0x4DC4FF));
//! fade.println();
//! ```

use crate::color::Color;
use crate::line::Line;
use crate::span::Span;

/// Paints `text` with a foreground color gradient from `from` to `to`.
///
/// Each character gets its own interpolated color. The result is a [`Line`],
/// so it honors [`ColorChoice`](crate::ColorChoice) when printing and can be
/// degraded with [`render_with`](Line::render_with) like any other line.
///
/// # Examples
///
/// ```
/// use pretty_console::{gradient, Color};
///
/// let fade = gradient("ok", Color::BLACK, Color::WHITE);
/// assert_eq!(fade.spans().len(), 2);
/// ```
#[must_use]
pub fn gradient(text: &str, from: Color, to: Color) -> Line {
    per_char(text, |c, t| {
        Span::new(c.to_string()).fg(Color::lerp(from, to, t))
    })
}

/// Paints `text` with a background color gradient from `from` to `to`.
#[must_use]
pub fn gradient_bg(text: &str, from: Color, to: Color) -> Line {
    per_char(text, |c, t| {
        Span::new(c.to_string()).bg(Color::lerp(from, to, t))
    })
}

/// Paints `text` with the full rainbow spectrum across its characters.
///
/// # Examples
///
/// ```
/// use pretty_console::rainbow;
///
/// let arc = rainbow("wow");
/// assert_eq!(arc.spans().len(), 3);
/// ```
#[must_use]
pub fn rainbow(text: &str) -> Line {
    per_char(text, |c, t| Span::new(c.to_string()).fg(hue_color(t)))
}

/// Paints `text` with the full rainbow spectrum as background colors.
#[must_use]
pub fn rainbow_bg(text: &str) -> Line {
    per_char(text, |c, t| Span::new(c.to_string()).bg(hue_color(t)))
}

/// Applies `paint` to every character of `text`; `t` is the character's
/// position on the `0.0..=1.0` interpolation axis.
fn per_char(text: &str, mut paint: impl FnMut(char, f32) -> Span) -> Line {
    let count = text.chars().count();
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            let t = if count <= 1 {
                0.0
            } else {
                i as f32 / (count - 1) as f32
            };
            paint(c, t)
        })
        .collect()
}

/// Converts a position on the `0.0..=1.0` axis into a fully saturated color
/// cycling the entire hue wheel.
fn hue_color(t: f32) -> Color {
    let (r, g, b) = hsv_to_rgb(t * 360.0, 1.0, 1.0);
    Color::rgb(r, g, b)
}

/// Standard HSV → RGB conversion (`h` in degrees, `s`/`v` in `0.0..=1.0`).
///
/// The single-letter names follow the standard algorithm notation.
#[must_use]
#[allow(clippy::many_single_char_names)]
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h.rem_euclid(360.0) / 60.0;
    let i = h.floor();
    let f = h - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));
    let (r, g, b) = match i as u8 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}
