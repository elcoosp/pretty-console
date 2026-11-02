//! ANSI-aware text utilities: stripping, visible width, truncation and padding.
//!
//! These helpers treat ANSI escape sequences as zero-width, skip zero-width
//! characters and count wide (CJK, emoji) characters as two columns, so they
//! line up with what a terminal actually renders.

use crate::color::RESET;

/// Text alignment used by [`pad`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Align {
    /// Content on the left, padding on the right.
    #[default]
    Left,
    /// Content centered, padding split on both sides.
    Center,
    /// Content on the right, padding on the left.
    Right,
}

/// Returns the end index (exclusive) of the escape sequence starting at byte
/// `i` of `input` (`input`'s byte at `i` must be `0x1B`).
///
/// Semantics match v1's character scanner:
/// - `ESC [` + any number of parameter/intermediate bytes (`0x20..=0x3F`) +
///   exactly one final byte (any byte), or truncated at end of input
/// - `ESC ]` + payload until `BEL` or `ESC \` (ST), or truncated
/// - any other escape consumes exactly one full character
fn escape_end(input: &str, i: usize) -> usize {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let Some(&next) = bytes.get(i + 1) else {
        return i + 1; // dangling ESC at end of input
    };
    match next {
        b'[' => {
            let mut j = i + 2;
            while j < len && (0x20..=0x3F).contains(&bytes[j]) {
                j += 1;
            }
            if j < len {
                j += 1; // final byte ends the sequence
            }
            j
        }
        b']' => {
            let mut j = i + 2;
            while j < len {
                match bytes[j] {
                    0x07 => return j + 1,            // BEL
                    0x1B => return (j + 2).min(len), // ESC \ (ST)
                    _ => j += 1,
                }
            }
            j
        }
        _ => {
            // Consume the ESC plus one full UTF-8 character.
            i + 1 + char_utf8_len(next)
        }
    }
}

/// The length in bytes of the UTF-8 character whose first byte is `lead`.
fn char_utf8_len(lead: u8) -> usize {
    if (0xC0..=0xDF).contains(&lead) {
        2
    } else if (0xE0..=0xEF).contains(&lead) {
        3
    } else if (0xF0..=0xF7).contains(&lead) {
        4
    } else {
        // ASCII, or an impossible continuation byte (input is valid UTF-8).
        1
    }
}

/// Removes ANSI escape sequences (CSI, OSC and single-character escapes) from
/// `input`, returning the plain text.
///
/// Truncated sequences at the end of the input are dropped.
///
/// # Examples
///
/// ```
/// use pretty_console::strip_ansi;
///
/// assert_eq!(strip_ansi("\x1b[1;31mhello\x1b[0m"), "hello");
/// assert_eq!(strip_ansi("\x1b]0;title\x07body"), "body");
/// assert_eq!(strip_ansi("plain"), "plain");
/// ```
#[must_use]
pub fn strip_ansi(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1B {
            i = escape_end(input, i);
            continue;
        }
        // Copy the whole run up to the next ESC. ESC is ASCII, so the slice
        // bounds are always char boundaries.
        let end = bytes[i..]
            .iter()
            .position(|&b| b == 0x1B)
            .map_or(bytes.len(), |rel| i + rel);
        out.push_str(&input[i..end]);
        i = end;
    }
    out
}

/// The number of terminal columns `c` occupies.
///
/// Control characters and common zero-width characters (combining marks,
/// variation selectors, zero-width joiners) count as 0; wide East Asian and
/// emoji characters count as 2; everything else counts as 1. This is a
/// practical approximation of the `unicode-width` reference tables, chosen to
/// keep the crate dependency-free.
///
/// # Examples
///
/// ```
/// use pretty_console::char_width;
///
/// assert_eq!(char_width('a'), 1);
/// assert_eq!(char_width('中'), 2);
/// assert_eq!(char_width('\u{0301}'), 0); // combining acute
/// ```
#[must_use]
// The arms intentionally repeat widths: the match is a readable width table.
#[allow(clippy::match_same_arms)]
pub fn char_width(c: char) -> usize {
    let cp = c as u32;
    match cp {
        // Control characters (including ESC) occupy no columns.
        0x00..=0x1F | 0x7F => 0,
        0x00..=0x7E => 1,
        // Combining diacritics and marks.
        0x0300..=0x036F
        | 0x0483..=0x0489
        | 0x0591..=0x05BD
        | 0x0610..=0x061A
        | 0x064B..=0x065F
        | 0x0E31..=0x0E3A
        | 0x1AB0..=0x1AFF
        | 0x1DC0..=0x1DFF
        | 0x20D0..=0x20F0 => 0,
        // Zero-width space, ZWNJ, ZWJ, directional marks.
        0x200B..=0x200F | 0x2060..=0x2064 | 0xFEFF => 0,
        // Variation selectors.
        0xFE00..=0xFE0F | 0xE0100..=0xE01EF => 0,
        // Hangul Jamo.
        0x1100..=0x115F => 2,
        // CJK radicals, punctuation, kana, common ideographs.
        0x2E80..=0x303E | 0x3041..=0x33FF | 0x3400..=0x4DBF | 0x4E00..=0x9FFF => 2,
        // Yi, Hangul compatibility jamo and syllables.
        0xA000..=0xA4CF | 0xA960..=0xA97F | 0xAC00..=0xD7A3 => 2,
        // CJK compatibility forms.
        0xF900..=0xFAFF | 0xFE10..=0xFE19 | 0xFE30..=0xFE6F => 2,
        // Fullwidth forms.
        0xFF00..=0xFF60 | 0xFFE0..=0xFFE6 => 2,
        // Emoji blocks.
        0x1F300..=0x1F64F | 0x1F900..=0x1F9FF | 0x1FA70..=0x1FAFF => 2,
        // CJK extensions B and C.
        0x20000..=0x2FFFD | 0x30000..=0x3FFFD => 2,
        _ => 1,
    }
}

/// The number of terminal columns `input` occupies when printed.
///
/// ANSI escape sequences are treated as zero-width, so styled strings are
/// measured correctly. Wide characters count as two columns.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::{visible_width, Styled};
///
/// assert_eq!(visible_width("héllo"), 5);
/// assert_eq!(visible_width("日本語"), 6);
/// assert_eq!(visible_width(&"hi".red().to_string()), 2);
/// # }
/// ```
#[must_use]
pub fn visible_width(input: &str) -> usize {
    if input.as_bytes().contains(&0x1B) {
        let plain = strip_ansi(input);
        return visible_width(&plain);
    }
    if input.is_ascii() {
        return input.bytes().filter(|b| (0x20..=0x7E).contains(b)).count();
    }
    input.chars().map(char_width).sum()
}

/// Truncates `input` to at most `max_width` visible columns, appending `tail`
/// (typically an ellipsis) when truncation happens.
///
/// Escape sequences are preserved and do not count toward the width. If any
/// escape sequence was emitted, a reset is appended after the tail so the
/// truncation cannot leak styles into later output. When `max_width` is
/// smaller than the tail, the tail itself overruns the requested width.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::truncate;
///
/// assert_eq!(truncate("hello world", 8, "…"), "hello w…");
/// assert_eq!(truncate("short", 8, "…"), "short");
/// # }
/// ```
#[must_use]
pub fn truncate(input: &str, max_width: usize, tail: &str) -> String {
    if visible_width(input) <= max_width {
        return input.to_string();
    }
    let tail_width = visible_width(tail);
    let budget = max_width.saturating_sub(tail_width);

    let bytes = input.as_bytes();
    let mut out = String::with_capacity(input.len() + tail.len() + RESET.len());
    let mut width = 0usize;
    let mut styled = false;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == 0x1B {
            let end = escape_end(input, i);
            out.push_str(&input[i..end]);
            styled = true;
            i = end;
            continue;
        }
        let Some(ch) = input[i..].chars().next() else {
            break;
        };
        let cw = char_width(ch);
        if width + cw > budget {
            break;
        }
        width += cw;
        out.push(ch);
        i += ch.len_utf8();
    }

    out.push_str(tail);
    if styled {
        out.push_str(RESET);
    }
    out
}

/// Pads `input` to exactly `width` visible columns using `fill`, without
/// counting ANSI escape sequences.
///
/// # Examples
///
/// ```
/// use pretty_console::{pad, Align};
///
/// assert_eq!(pad("hi", 4, Align::Left, '.'), "hi..");
/// assert_eq!(pad("hi", 4, Align::Right, '.'), "..hi");
/// assert_eq!(pad("hi", 5, Align::Center, '.'), ".hi..");
/// ```
#[must_use]
pub fn pad(input: &str, width: usize, align: Align, fill: char) -> String {
    let content = visible_width(input);
    let Some(padding) = width.checked_sub(content) else {
        return input.to_string();
    };
    if padding == 0 {
        return input.to_string();
    }
    match align {
        Align::Left => format!("{input}{}", fill_str(fill, padding)),
        Align::Right => format!("{}{input}", fill_str(fill, padding)),
        Align::Center => {
            let left = padding / 2;
            let right = padding - left;
            format!("{}{input}{}", fill_str(fill, left), fill_str(fill, right))
        }
    }
}

fn fill_str(fill: char, count: usize) -> String {
    std::iter::repeat_n(fill, count).collect()
}
