//! [`Line`]: ordered, differently-styled [`Span`]s that render as one row.

use std::fmt;
use std::fmt::Write as _;
use std::io;

use crate::attr::{Attribute, Effects};
use crate::choice::ColorLevel;
use crate::color::Color;
use crate::span::{
    Span, print_honoring_choice, println_honoring_choice, try_print_honoring_choice,
    try_println_honoring_choice, write_honoring_choice,
};
use crate::style::Style;
use crate::text::visible_width;

/// An ordered sequence of [`Span`]s that renders as a single row.
///
/// `Line` is the composition primitive v1 lacked: build status lines with
/// differently-styled segments without string concatenation.
///
/// The style methods on `Line` apply to the **last pushed span**, so a line
/// reads naturally left to right:
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::Line;
///
/// let line = Line::new().push("done ").push("in 42ms").green();
/// assert_eq!(
///     line.to_string(),
///     "done \x1b[38;5;2min 42ms\x1b[0m"
/// );
/// # }
/// ```
///
/// [`Line::with_style`] merges a style into **every** span instead.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Line {
    spans: Vec<Span>,
}

impl Line {
    /// Creates an empty line.
    #[must_use]
    pub const fn new() -> Self {
        Self { spans: Vec::new() }
    }

    /// Creates a line from an iterator of spans (or of `Into<Span>` values).
    #[must_use]
    pub fn from_spans(spans: impl IntoIterator<Item = Span>) -> Self {
        Self {
            spans: spans.into_iter().collect(),
        }
    }

    /// Pushes a span (or anything convertible into one, like `&str`) and
    /// returns the line for chaining.
    #[must_use]
    pub fn push(mut self, span: impl Into<Span>) -> Self {
        self.spans.push(span.into());
        self
    }

    /// Pushes a span in place, for use in loops.
    pub fn push_mut(&mut self, span: impl Into<Span>) {
        self.spans.push(span.into());
    }

    /// Pushes several spans at once.
    #[must_use]
    pub fn extend(mut self, spans: impl IntoIterator<Item = Span>) -> Self {
        self.spans.extend(spans);
        self
    }

    /// Returns the pushed spans.
    #[must_use]
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    /// Returns the total visible (CJK-aware) width of the line.
    #[must_use]
    pub fn visible_width(&self) -> usize {
        self.spans.iter().map(|s| visible_width(&s.text)).sum()
    }

    /// Merges a style into **every** span of the line.
    ///
    /// Colors set by `style` win over per-span colors; attribute sets are
    /// unioned.
    #[must_use]
    pub fn with_style(self, style: Style) -> Self {
        self.map_span(|mut span| {
            let merged = span.style().merge(style);
            span.set_style(merged);
            span
        })
    }

    /// Rewrites every span through `f`.
    #[must_use]
    pub fn map_span(self, f: impl FnMut(Span) -> Span) -> Self {
        Self {
            spans: self.spans.into_iter().map(f).collect(),
        }
    }

    /// Applies modifications to the whole line only when `condition` holds.
    #[must_use]
    pub fn when(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if condition { f(self) } else { self }
    }

    /// Reflows the line into rows of at most `max_width` visible columns,
    /// preserving each span's style and hyperlink.
    ///
    /// Spans are split at column boundaries (see [`WrapMode`](crate::WrapMode)
    /// for the two strategies); a row break inside a span produces adjacent
    /// fragments that carry the original style, so rendering re-opens the
    /// style on every row and no ANSI state leaks across lines.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "no-color"))]
    /// # {
    /// use pretty_console::{Line, WrapMode};
    ///
    /// let line = Line::new().push("total: ").push("12345 items").red();
    /// let rows = line.wrap(12, WrapMode::Word);
    /// assert_eq!(rows.len(), 2);
    /// assert_eq!(rows[0].to_string(), "total: \x1b[38;5;1m12345\x1b[0m");
    /// assert_eq!(rows[1].to_string(), "\x1b[38;5;1mitems\x1b[0m");
    /// # }
    /// ```
    #[must_use]
    pub fn wrap(&self, max_width: usize, mode: crate::text::WrapMode) -> Vec<Line> {
        let mut tokens: Vec<crate::text::Token> = Vec::new();
        for (index, span) in self.spans.iter().enumerate() {
            for mut token in crate::text::tokenize(&span.text, mode) {
                token.span = index;
                tokens.push(token);
            }
        }
        crate::text::pack_tokens(tokens, max_width)
            .into_iter()
            .map(|row| {
                let mut line = Line::new();
                let mut owner: Option<usize> = None;
                let mut text = String::new();
                for token in row {
                    if owner == Some(token.span) {
                        text.push_str(&token.text);
                    } else {
                        if let Some(index) = owner.take() {
                            line.push_mut(self.spans[index].with_text(std::mem::take(&mut text)));
                        }
                        owner = Some(token.span);
                        text = token.text;
                    }
                }
                if let Some(index) = owner {
                    line.push_mut(self.spans[index].with_text(text));
                }
                line
            })
            .collect()
    }

    fn last_span(mut self) -> Self {
        if self.spans.is_empty() {
            self.spans.push(Span::new(String::new()));
        }
        self
    }

    fn edit_last(self, f: impl FnOnce(Span) -> Span) -> Self {
        let mut line = self.last_span();
        let last = line.spans.last_mut().expect("just ensured non-empty");
        *last = f(std::mem::take(last));
        line
    }

    /// Sets the foreground color of the last span.
    #[must_use]
    pub fn fg(self, color: Color) -> Self {
        self.edit_last(|span| span.fg(color))
    }

    /// Sets the background color of the last span.
    #[must_use]
    pub fn bg(self, color: Color) -> Self {
        self.edit_last(|span| span.bg(color))
    }

    /// Sets a 24-bit RGB foreground color on the last span.
    #[must_use]
    pub fn fg_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.edit_last(|span| span.fg_rgb(r, g, b))
    }

    /// Sets a 24-bit RGB background color on the last span.
    #[must_use]
    pub fn bg_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.edit_last(|span| span.bg_rgb(r, g, b))
    }

    /// Adds an [`Attribute`] to the last span.
    #[must_use]
    pub fn attr(self, attribute: Attribute) -> Self {
        self.edit_last(|span| span.attr(attribute))
    }

    /// Unions an [`Effects`] set into the last span.
    #[must_use]
    pub fn effects(self, effects: Effects) -> Self {
        self.edit_last(|span| span.effects(effects))
    }

    /// Bold on the last span.
    #[must_use]
    pub fn bold(self) -> Self {
        self.attr(Attribute::Bold)
    }

    /// Dim on the last span.
    #[must_use]
    pub fn dim(self) -> Self {
        self.attr(Attribute::Dim)
    }

    /// Italic on the last span.
    #[must_use]
    pub fn italic(self) -> Self {
        self.attr(Attribute::Italic)
    }

    /// Underline on the last span.
    #[must_use]
    pub fn underline(self) -> Self {
        self.attr(Attribute::Underline)
    }

    /// Blink on the last span.
    #[must_use]
    pub fn blink(self) -> Self {
        self.attr(Attribute::Blink)
    }

    /// Reverse video on the last span.
    #[must_use]
    pub fn reverse(self) -> Self {
        self.attr(Attribute::Reverse)
    }

    /// Hidden on the last span.
    #[must_use]
    pub fn hidden(self) -> Self {
        self.attr(Attribute::Hidden)
    }

    /// Strikethrough on the last span.
    #[must_use]
    pub fn strikethrough(self) -> Self {
        self.attr(Attribute::Strikethrough)
    }

    /// Black foreground on the last span.
    #[must_use]
    pub fn black(self) -> Self {
        self.fg(Color::BLACK)
    }

    /// Red foreground on the last span.
    #[must_use]
    pub fn red(self) -> Self {
        self.fg(Color::RED)
    }

    /// Green foreground on the last span.
    #[must_use]
    pub fn green(self) -> Self {
        self.fg(Color::GREEN)
    }

    /// Yellow foreground on the last span.
    #[must_use]
    pub fn yellow(self) -> Self {
        self.fg(Color::YELLOW)
    }

    /// Blue foreground on the last span.
    #[must_use]
    pub fn blue(self) -> Self {
        self.fg(Color::BLUE)
    }

    /// Magenta foreground on the last span.
    #[must_use]
    pub fn magenta(self) -> Self {
        self.fg(Color::MAGENTA)
    }

    /// Cyan foreground on the last span.
    #[must_use]
    pub fn cyan(self) -> Self {
        self.fg(Color::CYAN)
    }

    /// White foreground on the last span.
    #[must_use]
    pub fn white(self) -> Self {
        self.fg(Color::WHITE)
    }

    /// Bright black foreground on the last span.
    #[must_use]
    pub fn bright_black(self) -> Self {
        self.fg(Color::BRIGHT_BLACK)
    }

    /// Bright red foreground on the last span.
    #[must_use]
    pub fn bright_red(self) -> Self {
        self.fg(Color::BRIGHT_RED)
    }

    /// Bright green foreground on the last span.
    #[must_use]
    pub fn bright_green(self) -> Self {
        self.fg(Color::BRIGHT_GREEN)
    }

    /// Bright yellow foreground on the last span.
    #[must_use]
    pub fn bright_yellow(self) -> Self {
        self.fg(Color::BRIGHT_YELLOW)
    }

    /// Bright blue foreground on the last span.
    #[must_use]
    pub fn bright_blue(self) -> Self {
        self.fg(Color::BRIGHT_BLUE)
    }

    /// Bright magenta foreground on the last span.
    #[must_use]
    pub fn bright_magenta(self) -> Self {
        self.fg(Color::BRIGHT_MAGENTA)
    }

    /// Bright cyan foreground on the last span.
    #[must_use]
    pub fn bright_cyan(self) -> Self {
        self.fg(Color::BRIGHT_CYAN)
    }

    /// Bright white foreground on the last span.
    #[must_use]
    pub fn bright_white(self) -> Self {
        self.fg(Color::BRIGHT_WHITE)
    }

    /// Black background on the last span.
    #[must_use]
    pub fn on_black(self) -> Self {
        self.bg(Color::BLACK)
    }

    /// Red background on the last span.
    #[must_use]
    pub fn on_red(self) -> Self {
        self.bg(Color::RED)
    }

    /// Green background on the last span.
    #[must_use]
    pub fn on_green(self) -> Self {
        self.bg(Color::GREEN)
    }

    /// Yellow background on the last span.
    #[must_use]
    pub fn on_yellow(self) -> Self {
        self.bg(Color::YELLOW)
    }

    /// Blue background on the last span.
    #[must_use]
    pub fn on_blue(self) -> Self {
        self.bg(Color::BLUE)
    }

    /// Magenta background on the last span.
    #[must_use]
    pub fn on_magenta(self) -> Self {
        self.bg(Color::MAGENTA)
    }

    /// Cyan background on the last span.
    #[must_use]
    pub fn on_cyan(self) -> Self {
        self.bg(Color::CYAN)
    }

    /// White background on the last span.
    #[must_use]
    pub fn on_white(self) -> Self {
        self.bg(Color::WHITE)
    }

    /// Bright black background on the last span.
    #[must_use]
    pub fn on_bright_black(self) -> Self {
        self.bg(Color::BRIGHT_BLACK)
    }

    /// Bright red background on the last span.
    #[must_use]
    pub fn on_bright_red(self) -> Self {
        self.bg(Color::BRIGHT_RED)
    }

    /// Bright green background on the last span.
    #[must_use]
    pub fn on_bright_green(self) -> Self {
        self.bg(Color::BRIGHT_GREEN)
    }

    /// Bright yellow background on the last span.
    #[must_use]
    pub fn on_bright_yellow(self) -> Self {
        self.bg(Color::BRIGHT_YELLOW)
    }

    /// Bright blue background on the last span.
    #[must_use]
    pub fn on_bright_blue(self) -> Self {
        self.bg(Color::BRIGHT_BLUE)
    }

    /// Bright magenta background on the last span.
    #[must_use]
    pub fn on_bright_magenta(self) -> Self {
        self.bg(Color::BRIGHT_MAGENTA)
    }

    /// Bright cyan background on the last span.
    #[must_use]
    pub fn on_bright_cyan(self) -> Self {
        self.bg(Color::BRIGHT_CYAN)
    }

    /// Bright white background on the last span.
    #[must_use]
    pub fn on_bright_white(self) -> Self {
        self.bg(Color::BRIGHT_WHITE)
    }

    // ----- rendering ---------------------------------------------------------

    /// Writes the styled body into `out`, optionally degrading colors.
    fn write_body(&self, out: &mut impl fmt::Write, level: Option<ColorLevel>) -> fmt::Result {
        for span in &self.spans {
            span.write_body(out, level)?;
        }
        Ok(())
    }

    /// Renders the line into a `String` (deterministic).
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = String::new();
        let _ = self.write_body(&mut out, None);
        out
    }

    /// Renders the line, reducing colors to `level`.
    #[must_use]
    pub fn render_with(&self, level: ColorLevel) -> String {
        let mut out = String::new();
        let _ = self.write_body(&mut out, Some(level));
        out
    }

    fn fmt_with(&self, f: &mut fmt::Formatter<'_>, level: Option<ColorLevel>) -> fmt::Result {
        match f.width() {
            None => self.write_body(f, level),
            Some(width) => {
                let padding = width.saturating_sub(self.visible_width());
                if padding == 0 {
                    return self.write_body(f, level);
                }
                match f.align() {
                    Some(fmt::Alignment::Right) => {
                        fill(f, padding)?;
                        self.write_body(f, level)
                    }
                    Some(fmt::Alignment::Center) => {
                        let left = padding / 2;
                        fill(f, left)?;
                        self.write_body(f, level)?;
                        fill(f, padding - left)
                    }
                    _ => {
                        self.write_body(f, level)?;
                        fill(f, padding)
                    }
                }
            }
        }
    }

    // ----- output -------------------------------------------------------------

    /// Writes the styled line, honoring the global
    /// [`ColorChoice`](crate::ColorChoice).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn write_to(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write_honoring_choice(&self.render(), &self.plain(), writer)
    }

    /// Prints the line to stdout without a trailing newline.
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails (for example on a broken pipe).
    pub fn print(&self) {
        print_honoring_choice(&self.render(), &self.plain(), false);
    }

    /// Prints the line to stdout followed by a newline.
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails (for example on a broken pipe).
    pub fn println(&self) {
        println_honoring_choice(&self.render(), &self.plain(), false);
    }

    /// Prints the line to stderr without a trailing newline.
    ///
    /// # Panics
    ///
    /// Panics if writing to stderr fails.
    pub fn eprint(&self) {
        print_honoring_choice(&self.render(), &self.plain(), true);
    }

    /// Prints the line to stderr followed by a newline.
    ///
    /// # Panics
    ///
    /// Panics if writing to stderr fails.
    pub fn eprintln(&self) {
        println_honoring_choice(&self.render(), &self.plain(), true);
    }

    /// Fallible [`print`](Line::print).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_print(&self) -> io::Result<()> {
        try_print_honoring_choice(&self.render(), &self.plain(), false)
    }

    /// Fallible [`println`](Line::println).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_println(&self) -> io::Result<()> {
        try_println_honoring_choice(&self.render(), &self.plain(), false)
    }

    /// Fallible [`eprint`](Line::eprint).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_eprint(&self) -> io::Result<()> {
        try_print_honoring_choice(&self.render(), &self.plain(), true)
    }

    /// Fallible [`eprintln`](Line::eprintln).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_eprintln(&self) -> io::Result<()> {
        try_println_honoring_choice(&self.render(), &self.plain(), true)
    }

    fn plain(&self) -> String {
        let mut out = String::with_capacity(self.visible_width());
        for span in &self.spans {
            out.push_str(&span.text);
        }
        out
    }
}

fn fill(f: &mut fmt::Formatter<'_>, count: usize) -> fmt::Result {
    for _ in 0..count {
        f.write_char(f.fill())?;
    }
    Ok(())
}

impl fmt::Display for Line {
    /// Renders the line; a requested width pads by total visible width.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, None)
    }
}

impl FromIterator<Span> for Line {
    fn from_iter<T: IntoIterator<Item = Span>>(iter: T) -> Self {
        Self::from_spans(iter)
    }
}

impl From<Vec<Span>> for Line {
    fn from(spans: Vec<Span>) -> Self {
        Self { spans }
    }
}

impl From<&str> for Line {
    fn from(text: &str) -> Self {
        Self::new().push(text)
    }
}

impl From<String> for Line {
    fn from(text: String) -> Self {
        Self::new().push(text)
    }
}

impl From<Span> for Line {
    fn from(span: Span) -> Self {
        Self::new().push(span)
    }
}

impl IntoIterator for Line {
    type Item = Span;
    type IntoIter = std::vec::IntoIter<Span>;

    fn into_iter(self) -> Self::IntoIter {
        self.spans.into_iter()
    }
}
