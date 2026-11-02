//! [`Span`]: the styled-string core, the [`Styled`] extension trait and the
//! printing helpers.

use std::fmt;
use std::fmt::Write as _;
use std::io::{self, Write as _};

use crate::attr::{Attribute, Effects};
use crate::color::Color;
use crate::style::Style;
use crate::text::visible_width;

// ---------------------------------------------------------------------------
// Shared IO helpers (used by Span, Line and Panel)
// ---------------------------------------------------------------------------

pub(crate) fn write_honoring_choice(
    rendered: &str,
    text: &str,
    writer: &mut impl io::Write,
) -> io::Result<()> {
    if crate::choice::is_color_enabled() {
        writer.write_all(rendered.as_bytes())
    } else {
        writer.write_all(text.as_bytes())
    }
}

/// Writes `payload` (plus an optional newline) to stdout or stderr.
fn emit(payload: &str, newline: bool, stderr: bool) -> io::Result<()> {
    let newline = if newline { Some(b"\n") } else { None };
    if stderr {
        let mut sink = io::stderr().lock();
        sink.write_all(payload.as_bytes())?;
        if let Some(nl) = newline {
            sink.write_all(nl)?;
        }
        sink.flush()
    } else {
        let mut sink = io::stdout().lock();
        sink.write_all(payload.as_bytes())?;
        if let Some(nl) = newline {
            sink.write_all(nl)?;
        }
        sink.flush()
    }
}

pub(crate) fn try_print_honoring_choice(
    rendered: &str,
    text: &str,
    stderr: bool,
) -> io::Result<()> {
    if crate::choice::is_color_enabled() {
        emit(rendered, false, stderr)
    } else {
        emit(text, false, stderr)
    }
}

pub(crate) fn try_println_honoring_choice(
    rendered: &str,
    text: &str,
    stderr: bool,
) -> io::Result<()> {
    if crate::choice::is_color_enabled() {
        emit(rendered, true, stderr)
    } else {
        emit(text, true, stderr)
    }
}

pub(crate) fn print_honoring_choice(rendered: &str, text: &str, stderr: bool) {
    let result = try_print_honoring_choice(rendered, text, stderr);
    if stderr {
        result.expect("failed to write to stderr");
    } else {
        result.expect("failed to write to stdout");
    }
}

pub(crate) fn println_honoring_choice(rendered: &str, text: &str, stderr: bool) {
    let result = try_println_honoring_choice(rendered, text, stderr);
    if stderr {
        result.expect("failed to write to stderr");
    } else {
        result.expect("failed to write to stdout");
    }
}

// ---------------------------------------------------------------------------
// Span
// ---------------------------------------------------------------------------

/// A styled string: text plus a [`Style`] plus an optional OSC 8 hyperlink.
///
/// `Span` replaces v1's `Console` (kept as a deprecated alias) and adds
/// hyperlinks, conditional styling and allocation-free rendering. `Display`
/// writes the SGR sequences straight into the destination formatter, so
/// `write!`/`print!` contexts never build intermediate strings.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::Span;
///
/// let warning = Span::new("deprecated").yellow().italic();
/// assert_eq!(warning.to_string(), "\x1b[3;38;5;3mdeprecated\x1b[0m");
/// # }
/// ```
///
/// # Style ordering
///
/// Attributes are emitted first (in canonical bold → strikethrough order),
/// then the foreground color, then the background color.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Span {
    pub(crate) text: String,
    pub(crate) style: Style,
    pub(crate) link: Option<Box<str>>,
}

impl Span {
    /// Creates a new `Span` with plain (unstyled) text.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
            link: None,
        }
    }

    /// Creates a new `Span` from text and a pre-built [`Style`].
    #[must_use]
    pub fn new_with_style(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
            link: None,
        }
    }

    /// Alias for [`new_with_style`](Span::new_with_style).
    #[must_use]
    pub fn with_style(text: impl Into<String>, style: Style) -> Self {
        Self::new_with_style(text, style)
    }

    /// Keeps the current style and link, replacing the text.
    ///
    /// Useful for reusing a configured style with different messages.
    #[must_use]
    pub fn with_text(&self, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: self.style,
            link: self.link.clone(),
        }
    }

    /// Returns the unstyled text held by this `Span`.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Replaces the text, keeping the style and link.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// Returns the [`Style`] held by this `Span`.
    #[must_use]
    pub const fn style(&self) -> &Style {
        &self.style
    }

    /// Replaces the style, keeping the text and link.
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Attaches an OSC 8 hyperlink to this span.
    ///
    /// Terminals that support hyperlinks make the text clickable; others
    /// ignore the sequence. The link is emitted around the styled text and is
    /// suppressed when the `no-color` feature is enabled or when printing with
    /// colors disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "no-color"))]
    /// # {
    /// use pretty_console::Span;
    ///
    /// let link = Span::new("docs").blue().underline().link("https://docs.rs");
    /// assert!(link.to_string().starts_with("\x1b]8;;https://docs.rs\x1b\\"));
    /// # }
    /// ```
    #[must_use]
    pub fn link(mut self, url: impl Into<Box<str>>) -> Self {
        self.link = Some(url.into());
        self
    }

    /// Removes any hyperlink from this span.
    #[must_use]
    pub fn unlink(mut self) -> Self {
        self.link = None;
        self
    }

    /// Returns the hyperlink URL, if any.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        self.link.as_deref()
    }

    // ----- style builders ---------------------------------------------------

    /// Replaces the whole style and returns the span.
    #[must_use]
    pub fn styled_with(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the foreground color.
    #[must_use]
    pub fn fg(mut self, color: Color) -> Self {
        self.style = self.style.fg(color);
        self
    }

    /// Sets the background color.
    #[must_use]
    pub fn bg(mut self, color: Color) -> Self {
        self.style = self.style.bg(color);
        self
    }

    /// Sets a 24-bit RGB foreground color.
    #[must_use]
    pub fn fg_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.fg(Color::RGB(r, g, b))
    }

    /// Sets a 24-bit RGB background color.
    #[must_use]
    pub fn bg_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.bg(Color::RGB(r, g, b))
    }

    /// Sets the foreground color from a packed `0xRRGGBB` value.
    #[must_use]
    pub fn fg_hex(self, value: u32) -> Self {
        self.fg(Color::hex(value))
    }

    /// Sets the background color from a packed `0xRRGGBB` value.
    #[must_use]
    pub fn bg_hex(self, value: u32) -> Self {
        self.bg(Color::hex(value))
    }

    /// Adds a single [`Attribute`].
    #[must_use]
    pub fn attr(mut self, attribute: Attribute) -> Self {
        self.style = self.style.attr(attribute);
        self
    }

    /// Unions a whole [`Effects`] set into the style.
    #[must_use]
    pub fn effects(mut self, effects: Effects) -> Self {
        self.style = self.style.effects(effects);
        self
    }

    /// Applies modifications only when `condition` holds.
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Span;
    ///
    /// let quiet = false;
    /// let span = Span::new("hi").bold().when(quiet, |s| s.dim());
    /// assert!(!span.style().attributes().has(pretty_console::Attribute::Dim));
    /// ```
    #[must_use]
    pub fn when(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if condition { f(self) } else { self }
    }

    /// Black foreground.
    #[must_use]
    pub fn black(self) -> Self {
        self.fg(Color::BLACK)
    }

    /// Red foreground.
    #[must_use]
    pub fn red(self) -> Self {
        self.fg(Color::RED)
    }

    /// Green foreground.
    #[must_use]
    pub fn green(self) -> Self {
        self.fg(Color::GREEN)
    }

    /// Yellow foreground.
    #[must_use]
    pub fn yellow(self) -> Self {
        self.fg(Color::YELLOW)
    }

    /// Blue foreground.
    #[must_use]
    pub fn blue(self) -> Self {
        self.fg(Color::BLUE)
    }

    /// Magenta foreground.
    #[must_use]
    pub fn magenta(self) -> Self {
        self.fg(Color::MAGENTA)
    }

    /// Cyan foreground.
    #[must_use]
    pub fn cyan(self) -> Self {
        self.fg(Color::CYAN)
    }

    /// White foreground.
    #[must_use]
    pub fn white(self) -> Self {
        self.fg(Color::WHITE)
    }

    /// Bright black foreground.
    #[must_use]
    pub fn bright_black(self) -> Self {
        self.fg(Color::BRIGHT_BLACK)
    }

    /// Bright red foreground.
    #[must_use]
    pub fn bright_red(self) -> Self {
        self.fg(Color::BRIGHT_RED)
    }

    /// Bright green foreground.
    #[must_use]
    pub fn bright_green(self) -> Self {
        self.fg(Color::BRIGHT_GREEN)
    }

    /// Bright yellow foreground.
    #[must_use]
    pub fn bright_yellow(self) -> Self {
        self.fg(Color::BRIGHT_YELLOW)
    }

    /// Bright blue foreground.
    #[must_use]
    pub fn bright_blue(self) -> Self {
        self.fg(Color::BRIGHT_BLUE)
    }

    /// Bright magenta foreground.
    #[must_use]
    pub fn bright_magenta(self) -> Self {
        self.fg(Color::BRIGHT_MAGENTA)
    }

    /// Bright cyan foreground.
    #[must_use]
    pub fn bright_cyan(self) -> Self {
        self.fg(Color::BRIGHT_CYAN)
    }

    /// Bright white foreground.
    #[must_use]
    pub fn bright_white(self) -> Self {
        self.fg(Color::BRIGHT_WHITE)
    }

    /// Black background.
    #[must_use]
    pub fn on_black(self) -> Self {
        self.bg(Color::BLACK)
    }

    /// Red background.
    #[must_use]
    pub fn on_red(self) -> Self {
        self.bg(Color::RED)
    }

    /// Green background.
    #[must_use]
    pub fn on_green(self) -> Self {
        self.bg(Color::GREEN)
    }

    /// Yellow background.
    #[must_use]
    pub fn on_yellow(self) -> Self {
        self.bg(Color::YELLOW)
    }

    /// Blue background.
    #[must_use]
    pub fn on_blue(self) -> Self {
        self.bg(Color::BLUE)
    }

    /// Magenta background.
    #[must_use]
    pub fn on_magenta(self) -> Self {
        self.bg(Color::MAGENTA)
    }

    /// Cyan background.
    #[must_use]
    pub fn on_cyan(self) -> Self {
        self.bg(Color::CYAN)
    }

    /// White background.
    #[must_use]
    pub fn on_white(self) -> Self {
        self.bg(Color::WHITE)
    }

    /// Bright black background.
    #[must_use]
    pub fn on_bright_black(self) -> Self {
        self.bg(Color::BRIGHT_BLACK)
    }

    /// Bright red background.
    #[must_use]
    pub fn on_bright_red(self) -> Self {
        self.bg(Color::BRIGHT_RED)
    }

    /// Bright green background.
    #[must_use]
    pub fn on_bright_green(self) -> Self {
        self.bg(Color::BRIGHT_GREEN)
    }

    /// Bright yellow background.
    #[must_use]
    pub fn on_bright_yellow(self) -> Self {
        self.bg(Color::BRIGHT_YELLOW)
    }

    /// Bright blue background.
    #[must_use]
    pub fn on_bright_blue(self) -> Self {
        self.bg(Color::BRIGHT_BLUE)
    }

    /// Bright magenta background.
    #[must_use]
    pub fn on_bright_magenta(self) -> Self {
        self.bg(Color::BRIGHT_MAGENTA)
    }

    /// Bright cyan background.
    #[must_use]
    pub fn on_bright_cyan(self) -> Self {
        self.bg(Color::BRIGHT_CYAN)
    }

    /// Bright white background.
    #[must_use]
    pub fn on_bright_white(self) -> Self {
        self.bg(Color::BRIGHT_WHITE)
    }

    /// Adds the bold attribute.
    #[must_use]
    pub fn bold(self) -> Self {
        self.attr(Attribute::Bold)
    }

    /// Adds the dim attribute.
    #[must_use]
    pub fn dim(self) -> Self {
        self.attr(Attribute::Dim)
    }

    /// Adds the italic attribute.
    #[must_use]
    pub fn italic(self) -> Self {
        self.attr(Attribute::Italic)
    }

    /// Adds the underline attribute.
    #[must_use]
    pub fn underline(self) -> Self {
        self.attr(Attribute::Underline)
    }

    /// Adds the blink attribute.
    #[must_use]
    pub fn blink(self) -> Self {
        self.attr(Attribute::Blink)
    }

    /// Adds the reverse attribute.
    #[must_use]
    pub fn reverse(self) -> Self {
        self.attr(Attribute::Reverse)
    }

    /// Adds the hidden attribute.
    #[must_use]
    pub fn hidden(self) -> Self {
        self.attr(Attribute::Hidden)
    }

    /// Adds the strikethrough attribute.
    #[must_use]
    pub fn strikethrough(self) -> Self {
        self.attr(Attribute::Strikethrough)
    }

    // ----- level helpers ------------------------------------------------------

    /// Builds an informational message (`[INFO]`, cyan).
    #[must_use]
    pub fn info(text: impl Into<String>) -> Self {
        Self::tagged("[INFO]", text, Color::CYAN, false)
    }

    /// Builds a warning message (`[WARN]`, yellow).
    #[must_use]
    pub fn warn(text: impl Into<String>) -> Self {
        Self::tagged("[WARN]", text, Color::YELLOW, false)
    }

    /// Builds an error message (`[ERROR]`, bold red).
    #[must_use]
    pub fn error(text: impl Into<String>) -> Self {
        Self::tagged("[ERROR]", text, Color::RED, true)
    }

    /// Builds a success message (`[OK]`, bold green).
    #[must_use]
    pub fn success(text: impl Into<String>) -> Self {
        Self::tagged("[OK]", text, Color::GREEN, true)
    }

    /// Builds a debug message (`[DEBUG]`, dim).
    #[must_use]
    pub fn debug(text: impl Into<String>) -> Self {
        Span::new(format!("[DEBUG] {}", text.into())).dim()
    }

    fn tagged(tag: &str, text: impl Into<String>, color: Color, bold: bool) -> Self {
        let span = Span::new(format!("{tag} {}", text.into())).fg(color);
        if bold { span.bold() } else { span }
    }

    // ----- decorations ---------------------------------------------------------

    /// Builds a full-width horizontal banner with a centered title.
    ///
    /// The banner is cyan and bold. When `width` is too small to fit the title
    /// with at least two dashes on each side, the plain title is returned
    /// styled instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "no-color"))]
    /// # {
    /// use pretty_console::Span;
    ///
    /// let banner = Span::banner("Setup", 21);
    /// assert_eq!(banner.to_string(), "\x1b[1;38;5;6m─────── Setup ───────\x1b[0m");
    /// # }
    /// ```
    #[must_use]
    pub fn banner(title: impl Into<String>, width: usize) -> Self {
        let title = title.into();
        let title_width = title.chars().count();
        let dashes = width.saturating_sub(title_width + 2);
        if dashes < 4 {
            return Span::new(title).cyan().bold();
        }
        let left = "─".repeat(dashes / 2);
        let right = "─".repeat(dashes - dashes / 2);
        Span::new(format!("{left} {title} {right}")).cyan().bold()
    }

    // ----- rendering -------------------------------------------------------------

    /// Writes the full styled body (link, start, text, reset) into `out`.
    pub(crate) fn write_body(
        &self,
        out: &mut impl fmt::Write,
        level: Option<crate::choice::ColorLevel>,
    ) -> fmt::Result {
        let style = match level {
            Some(lvl) => self.style.degraded(lvl),
            None => self.style,
        };
        #[cfg(not(feature = "no-color"))]
        if let Some(url) = &self.link {
            out.write_str("\x1b]8;;")?;
            out.write_str(url)?;
            out.write_str("\x1b\\")?;
        }
        style.write_start(out)?;
        out.write_str(&self.text)?;
        // Only emit a reset when a start sequence was actually written; a
        // plain style must render the bare text with no escapes at all.
        if !style.is_plain() {
            Style::write_reset(out)?;
        }
        #[cfg(not(feature = "no-color"))]
        if self.link.is_some() {
            out.write_str("\x1b]8;;\x1b\\")?;
        }
        Ok(())
    }

    /// Renders the styled text into a `String` (deterministic; never consults
    /// the environment).
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = String::with_capacity(self.text.len() + 24);
        let _ = self.write_body(&mut out, None);
        out
    }

    /// Renders the styled text, reducing colors to `level`.
    #[must_use]
    pub fn render_with(&self, level: crate::choice::ColorLevel) -> String {
        let mut out = String::with_capacity(self.text.len() + 24);
        let _ = self.write_body(&mut out, Some(level));
        out
    }

    fn fmt_with(
        &self,
        f: &mut fmt::Formatter<'_>,
        level: Option<crate::choice::ColorLevel>,
    ) -> fmt::Result {
        match f.width() {
            None => self.write_body(f, level),
            Some(width) => {
                let padding = width.saturating_sub(visible_width(&self.text));
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

    // ----- output ------------------------------------------------------------------

    /// Renders the styled text, honoring the global
    /// [`ColorChoice`](crate::ColorChoice).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn write_to(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write_honoring_choice(&self.render(), &self.text, writer)
    }

    /// Prints the styled text to stdout without a trailing newline.
    ///
    /// Printing honors the global [`ColorChoice`](crate::ColorChoice).
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails (for example on a broken pipe).
    pub fn print(&self) {
        print_honoring_choice(&self.render(), &self.text, false);
    }

    /// Prints the styled text to stdout followed by a newline.
    ///
    /// Printing honors the global [`ColorChoice`](crate::ColorChoice).
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails (for example on a broken pipe).
    pub fn println(&self) {
        println_honoring_choice(&self.render(), &self.text, false);
    }

    /// Prints the styled text to stderr without a trailing newline.
    ///
    /// # Panics
    ///
    /// Panics if writing to stderr fails.
    pub fn eprint(&self) {
        print_honoring_choice(&self.render(), &self.text, true);
    }

    /// Prints the styled text to stderr followed by a newline.
    ///
    /// # Panics
    ///
    /// Panics if writing to stderr fails.
    pub fn eprintln(&self) {
        println_honoring_choice(&self.render(), &self.text, true);
    }

    /// Fallible [`print`](Span::print).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_print(&self) -> io::Result<()> {
        try_print_honoring_choice(&self.render(), &self.text, false)
    }

    /// Fallible [`println`](Span::println).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_println(&self) -> io::Result<()> {
        try_println_honoring_choice(&self.render(), &self.text, false)
    }

    /// Fallible [`eprint`](Span::eprint).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_eprint(&self) -> io::Result<()> {
        try_print_honoring_choice(&self.render(), &self.text, true)
    }

    /// Fallible [`eprintln`](Span::eprintln).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn try_eprintln(&self) -> io::Result<()> {
        try_println_honoring_choice(&self.render(), &self.text, true)
    }
}

fn fill(f: &mut fmt::Formatter<'_>, count: usize) -> fmt::Result {
    for _ in 0..count {
        f.write_char(f.fill())?;
    }
    Ok(())
}

impl fmt::Display for Span {
    /// Renders the styled text. When a width is requested, the padding is
    /// computed from the *visible* (CJK-aware) text width, not from the
    /// ANSI-decorated string, so `format!("{:>20}", span)` aligns as expected.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with(f, None)
    }
}

impl From<&str> for Span {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for Span {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// v1 compatibility alias for [`Span`].
pub type Console = Span;

// ---------------------------------------------------------------------------
// Styled trait
// ---------------------------------------------------------------------------

/// Extension trait that paints string-like values directly.
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::Styled;
///
/// assert_eq!("hello".red().bold().to_string(), "\x1b[1;38;5;1mhello\x1b[0m");
/// # }
/// ```
pub trait Styled {
    /// Wraps the value in a [`Span`] with a plain style.
    fn styled(&self) -> Span;

    /// Foreground black.
    fn black(&self) -> Span {
        self.styled().black()
    }

    /// Foreground red.
    fn red(&self) -> Span {
        self.styled().red()
    }

    /// Foreground green.
    fn green(&self) -> Span {
        self.styled().green()
    }

    /// Foreground yellow.
    fn yellow(&self) -> Span {
        self.styled().yellow()
    }

    /// Foreground blue.
    fn blue(&self) -> Span {
        self.styled().blue()
    }

    /// Foreground magenta.
    fn magenta(&self) -> Span {
        self.styled().magenta()
    }

    /// Foreground cyan.
    fn cyan(&self) -> Span {
        self.styled().cyan()
    }

    /// Foreground white.
    fn white(&self) -> Span {
        self.styled().white()
    }

    /// Foreground bright black.
    fn bright_black(&self) -> Span {
        self.styled().bright_black()
    }

    /// Foreground bright red.
    fn bright_red(&self) -> Span {
        self.styled().bright_red()
    }

    /// Foreground bright green.
    fn bright_green(&self) -> Span {
        self.styled().bright_green()
    }

    /// Foreground bright yellow.
    fn bright_yellow(&self) -> Span {
        self.styled().bright_yellow()
    }

    /// Foreground bright blue.
    fn bright_blue(&self) -> Span {
        self.styled().bright_blue()
    }

    /// Foreground bright magenta.
    fn bright_magenta(&self) -> Span {
        self.styled().bright_magenta()
    }

    /// Foreground bright cyan.
    fn bright_cyan(&self) -> Span {
        self.styled().bright_cyan()
    }

    /// Foreground bright white.
    fn bright_white(&self) -> Span {
        self.styled().bright_white()
    }

    /// Background black.
    fn on_black(&self) -> Span {
        self.styled().on_black()
    }

    /// Background red.
    fn on_red(&self) -> Span {
        self.styled().on_red()
    }

    /// Background green.
    fn on_green(&self) -> Span {
        self.styled().on_green()
    }

    /// Background yellow.
    fn on_yellow(&self) -> Span {
        self.styled().on_yellow()
    }

    /// Background blue.
    fn on_blue(&self) -> Span {
        self.styled().on_blue()
    }

    /// Background magenta.
    fn on_magenta(&self) -> Span {
        self.styled().on_magenta()
    }

    /// Background cyan.
    fn on_cyan(&self) -> Span {
        self.styled().on_cyan()
    }

    /// Background white.
    fn on_white(&self) -> Span {
        self.styled().on_white()
    }

    /// Background bright black.
    fn on_bright_black(&self) -> Span {
        self.styled().on_bright_black()
    }

    /// Background bright red.
    fn on_bright_red(&self) -> Span {
        self.styled().on_bright_red()
    }

    /// Background bright green.
    fn on_bright_green(&self) -> Span {
        self.styled().on_bright_green()
    }

    /// Background bright yellow.
    fn on_bright_yellow(&self) -> Span {
        self.styled().on_bright_yellow()
    }

    /// Background bright blue.
    fn on_bright_blue(&self) -> Span {
        self.styled().on_bright_blue()
    }

    /// Background bright magenta.
    fn on_bright_magenta(&self) -> Span {
        self.styled().on_bright_magenta()
    }

    /// Background bright cyan.
    fn on_bright_cyan(&self) -> Span {
        self.styled().on_bright_cyan()
    }

    /// Background bright white.
    fn on_bright_white(&self) -> Span {
        self.styled().on_bright_white()
    }

    /// Bold attribute.
    fn bold(&self) -> Span {
        self.styled().bold()
    }

    /// Dim attribute.
    fn dim(&self) -> Span {
        self.styled().dim()
    }

    /// Italic attribute.
    fn italic(&self) -> Span {
        self.styled().italic()
    }

    /// Underline attribute.
    fn underline(&self) -> Span {
        self.styled().underline()
    }

    /// Blink attribute.
    fn blink(&self) -> Span {
        self.styled().blink()
    }

    /// Reverse attribute.
    fn reverse(&self) -> Span {
        self.styled().reverse()
    }

    /// Hidden attribute.
    fn hidden(&self) -> Span {
        self.styled().hidden()
    }

    /// Strikethrough attribute.
    fn strikethrough(&self) -> Span {
        self.styled().strikethrough()
    }
}

impl<T: AsRef<str> + ?Sized> Styled for T {
    fn styled(&self) -> Span {
        Span::new(self.as_ref())
    }
}
