//! A fluent, zero-dependency API for styling terminal text with ANSI colors and attributes.
//!
//! Write beautifully formatted console output with an intuitive, chainable interface.
//! The crate has no runtime dependencies and compiles down to plain ANSI escape
//! sequences, so it works on every terminal that understands ANSI codes
//! (Windows 10+ included).
//!
//! # Highlights
//!
//! - **16 named colors**, the full **256-color** palette and **24-bit RGB**
//! - **Text attributes**: bold, dim, italic, underline, blink, reverse, hidden, strikethrough
//! - **Reusable styles**: build a [`Style`] once, apply it to many strings
//! - **Runtime color control**: honor `NO_COLOR`, `CLICOLOR`, `CLICOLOR_FORCE` and
//!   `FORCE_COLOR` via [`ColorChoice`]
//! - **`Styled` trait**: paint `&str` and `String` literals directly, `"hello".red().bold()`
//! - **Log-level helpers**: [`Console::info`], [`Console::warn`], [`Console::error`],
//!   [`Console::success`] and [`Console::debug`]
//! - **[`strip_ansi`]**: remove ANSI escape sequences from any string
//! - **Optional `no-color` feature**: compile out all styling entirely
//!
//! # Quick start
//!
//! ```
//! # #[cfg(not(feature = "no-color"))]
//! # {
//! use pretty_console::{Console, Styled};
//!
//! // Chainable builder API.
//! let error = Console::new("Error: file not found").red().bold();
//! assert_eq!(error.to_string(), "\x1b[1;38;5;1mError: file not found\x1b[0m");
//!
//! // Extension trait for string literals.
//! let ok = "Success!".green();
//! assert_eq!(ok.to_string(), "\x1b[38;5;2mSuccess!\x1b[0m");
//!
//! // Reusable style templates.
//! let heading = Console::new("").cyan().bold().underline();
//! assert_eq!(
//!     heading.with_text("Chapter 1").to_string(),
//!     "\x1b[1;4;38;5;6mChapter 1\x1b[0m"
//! );
//! # }
//! ```
//!
//! # Runtime color control
//!
//! [`write_to`](Console::write_to), [`print`](Console::print) and
//! [`println`](Console::println) consult the global [`ColorChoice`] before
//! emitting ANSI codes:
//!
//! | Choice        | Behavior                                                        |
//! |---------------|-----------------------------------------------------------------|
//! | [`Auto`]      | Honor `FORCE_COLOR`, `CLICOLOR_FORCE`, `NO_COLOR`, `CLICOLOR` and `TERM=dumb`, then fall back to stdout terminal detection |
//! | [`Always`]    | Always emit colors                                              |
//! | [`Never`]     | Never emit colors                                               |
//!
//! [`Auto`]: ColorChoice::Auto
//! [`Always`]: ColorChoice::Always
//! [`Never`]: ColorChoice::Never
//!
//! String rendering ([`to_string`](Console::to_string) and `Display`) is
//! deterministic and only affected by the compile-time `no-color` feature, so
//! formatted output never silently changes depending on where it is built.
//!
//! # Style ordering
//!
//! Within an ANSI sequence, attributes are emitted first, then the foreground
//! color, then the background color. This ordering is stable and covered by
//! snapshot tests.

use std::fmt;
#[cfg(not(feature = "no-color"))]
use std::io::IsTerminal;
use std::io::{self, Write};
use std::sync::atomic::{AtomicU8, Ordering};

/// The ANSI reset sequence appended after styled text.
const RESET: &str = "\x1b[0m";

// ===========================================================================
// Color
// ===========================================================================

/// A terminal color: a palette index or a 24-bit RGB value.
///
/// # Examples
///
/// ```
/// use pretty_console::Color;
///
/// let red = Color::RED;
/// assert_eq!(red.fg_code(), "38;5;1");
///
/// let sky = Color::rgb(64, 164, 255);
/// assert_eq!(sky.fg_code(), "38;2;64;164;255");
///
/// let amber = Color::hex(0xFFBF00);
/// assert_eq!(amber, Color::RGB(255, 191, 0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// A palette index into the 256-color table (0-255).
    Named(u8),
    /// A 24-bit RGB color.
    RGB(u8, u8, u8),
}

impl Color {
    /// Black (palette index 0).
    pub const BLACK: Color = Color::Named(0);
    /// Red (palette index 1).
    pub const RED: Color = Color::Named(1);
    /// Green (palette index 2).
    pub const GREEN: Color = Color::Named(2);
    /// Yellow (palette index 3).
    pub const YELLOW: Color = Color::Named(3);
    /// Blue (palette index 4).
    pub const BLUE: Color = Color::Named(4);
    /// Magenta (palette index 5).
    pub const MAGENTA: Color = Color::Named(5);
    /// Cyan (palette index 6).
    pub const CYAN: Color = Color::Named(6);
    /// White (palette index 7).
    pub const WHITE: Color = Color::Named(7);
    /// Bright black / gray (palette index 8).
    pub const BRIGHT_BLACK: Color = Color::Named(8);
    /// Bright red (palette index 9).
    pub const BRIGHT_RED: Color = Color::Named(9);
    /// Bright green (palette index 10).
    pub const BRIGHT_GREEN: Color = Color::Named(10);
    /// Bright yellow (palette index 11).
    pub const BRIGHT_YELLOW: Color = Color::Named(11);
    /// Bright blue (palette index 12).
    pub const BRIGHT_BLUE: Color = Color::Named(12);
    /// Bright magenta (palette index 13).
    pub const BRIGHT_MAGENTA: Color = Color::Named(13);
    /// Bright cyan (palette index 14).
    pub const BRIGHT_CYAN: Color = Color::Named(14);
    /// Bright white (palette index 15).
    pub const BRIGHT_WHITE: Color = Color::Named(15);

    /// Creates a color from a 256-color palette index.
    #[must_use]
    pub const fn indexed(index: u8) -> Self {
        Self::Named(index)
    }

    /// Creates a 24-bit RGB color.
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::RGB(red, green, blue)
    }

    /// Creates a 24-bit RGB color from a packed `0xRRGGBB` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Color;
    ///
    /// assert_eq!(Color::hex(0x00FF7F), Color::RGB(0, 255, 127));
    /// ```
    #[must_use]
    pub const fn hex(value: u32) -> Self {
        Self::RGB(
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        )
    }

    /// Returns the SGR parameters that select this color as the foreground.
    #[must_use]
    pub fn fg_code(&self) -> String {
        match self {
            Color::Named(index) => format!("38;5;{index}"),
            Color::RGB(r, g, b) => format!("38;2;{r};{g};{b}"),
        }
    }

    /// Returns the SGR parameters that select this color as the background.
    #[must_use]
    pub fn bg_code(&self) -> String {
        match self {
            Color::Named(index) => format!("48;5;{index}"),
            Color::RGB(r, g, b) => format!("48;2;{r};{g};{b}"),
        }
    }
}

impl From<u8> for Color {
    fn from(index: u8) -> Self {
        Self::Named(index)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::RGB(r, g, b)
    }
}

// ===========================================================================
// Attribute
// ===========================================================================

/// A text attribute (SGR parameter) such as bold or underline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute {
    /// Bold or increased intensity.
    Bold,
    /// Dim / decreased intensity.
    Dim,
    /// Italic.
    Italic,
    /// Underlined.
    Underline,
    /// Slow blink.
    Blink,
    /// Reverse video (swap foreground and background).
    Reverse,
    /// Hidden (concealed) text.
    Hidden,
    /// Strikethrough.
    Strikethrough,
}

impl Attribute {
    /// Returns the raw SGR parameter for this attribute.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Attribute::Bold => "1",
            Attribute::Dim => "2",
            Attribute::Italic => "3",
            Attribute::Underline => "4",
            Attribute::Blink => "5",
            Attribute::Reverse => "7",
            Attribute::Hidden => "8",
            Attribute::Strikethrough => "9",
        }
    }
}

// ===========================================================================
// Color choice
// ===========================================================================

/// Runtime color policy used when printing to a terminal.
///
/// The default is [`Auto`](ColorChoice::Auto), which inspects the standard
/// environment variables and detects whether stdout is a terminal.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ColorChoice {
    /// Detect automatically from the environment and stdout.
    #[default]
    Auto,
    /// Always emit colors, regardless of the environment.
    Always,
    /// Never emit colors, regardless of the environment.
    Never,
}

static COLOR_CHOICE: AtomicU8 = AtomicU8::new(0);

/// Sets the global [`ColorChoice`].
///
/// This affects [`Console::write_to`], [`Console::print`] and
/// [`Console::println`]. It does not affect string rendering.
pub fn set_color_choice(choice: ColorChoice) {
    let encoded = match choice {
        ColorChoice::Auto => 0,
        ColorChoice::Always => 1,
        ColorChoice::Never => 2,
    };
    COLOR_CHOICE.store(encoded, Ordering::Relaxed);
}

/// Returns the current global [`ColorChoice`].
#[must_use]
pub fn color_choice() -> ColorChoice {
    match COLOR_CHOICE.load(Ordering::Relaxed) {
        1 => ColorChoice::Always,
        2 => ColorChoice::Never,
        _ => ColorChoice::Auto,
    }
}

/// Returns whether colors should currently be emitted when printing.
///
/// With the `no-color` feature enabled this always returns `false`.
#[must_use]
pub fn is_color_enabled() -> bool {
    match color_choice() {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => env_enabled(|key| std::env::var(key).ok(), stdout_is_terminal()),
    }
}

#[cfg(not(feature = "no-color"))]
fn stdout_is_terminal() -> bool {
    io::stdout().is_terminal()
}

#[cfg(feature = "no-color")]
fn stdout_is_terminal() -> bool {
    false
}

#[cfg(feature = "no-color")]
fn env_enabled(_get: impl Fn(&str) -> Option<String>, _is_tty: bool) -> bool {
    false
}

/// Pure decision procedure for [`ColorChoice::Auto`], parameterized over the
/// environment for testability.
///
/// The checks, in order, are:
///
/// 1. `FORCE_COLOR` set to a non-empty value other than `0` forces colors on;
///    `FORCE_COLOR=0` forces them off.
/// 2. `CLICOLOR_FORCE` set to a non-empty value other than `0` forces colors on.
/// 3. `NO_COLOR` set to a non-empty value disables colors
///    (see <https://no-color.org/>).
/// 4. `CLICOLOR=0` disables colors.
/// 5. `TERM=dumb` disables colors.
/// 6. Otherwise colors are emitted only when stdout is a terminal.
#[cfg(not(feature = "no-color"))]
fn env_enabled(get: impl Fn(&str) -> Option<String>, is_tty: bool) -> bool {
    let non_empty = |key: &str| get(key).filter(|value| !value.trim().is_empty());

    if let Some(value) = non_empty("FORCE_COLOR") {
        return value != "0";
    }
    if non_empty("CLICOLOR_FORCE").is_some_and(|value| value != "0") {
        return true;
    }
    if non_empty("NO_COLOR").is_some() {
        return false;
    }
    if non_empty("CLICOLOR").is_some_and(|value| value == "0") {
        return false;
    }
    if non_empty("TERM").is_some_and(|value| value == "dumb") {
        return false;
    }
    is_tty
}

// ===========================================================================
// Style
// ===========================================================================

/// A reusable collection of colors and attributes.
///
/// Styles can be built fluently and applied to any number of strings:
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::{Console, Style, Color};
///
/// let heading = Style::new().bold().underline().fg(Color::BRIGHT_BLUE);
/// Console::new_with_style("Chapter 1", heading.clone()).println();
/// Console::new_with_style("Chapter 2", heading).println();
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    attributes: Vec<Attribute>,
}

impl Style {
    /// Creates an empty style with no colors or attributes.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            foreground: None,
            background: None,
            attributes: Vec::new(),
        }
    }

    /// Sets the foreground color.
    #[must_use]
    pub fn fg(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Sets the background color.
    #[must_use]
    pub fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Adds a single [`Attribute`].
    #[must_use]
    pub fn attr(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
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

    /// Returns the foreground color, if any.
    #[must_use]
    pub const fn foreground(&self) -> Option<Color> {
        self.foreground
    }

    /// Returns the background color, if any.
    #[must_use]
    pub const fn background(&self) -> Option<Color> {
        self.background
    }

    /// Returns the accumulated attributes.
    #[must_use]
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    /// Returns `true` when the style has no colors and no attributes.
    #[must_use]
    pub fn is_plain(&self) -> bool {
        self.foreground.is_none() && self.background.is_none() && self.attributes.is_empty()
    }

    /// Combines two styles into one; `self` first, then `other`.
    ///
    /// Colors set by `other` win, and `other`'s attributes are appended after
    /// `self`'s.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        if other.foreground.is_some() {
            self.foreground = other.foreground;
        }
        if other.background.is_some() {
            self.background = other.background;
        }
        self.attributes.extend(other.attributes);
        self
    }

    fn to_ansi_start(&self) -> String {
        #[cfg(feature = "no-color")]
        {
            let _ = self;
            String::new()
        }
        #[cfg(not(feature = "no-color"))]
        {
            let mut codes: Vec<String> = Vec::new();

            for attr in &self.attributes {
                codes.push(attr.code().into());
            }

            if let Some(fg) = &self.foreground {
                codes.push(fg.fg_code());
            }

            if let Some(bg) = &self.background {
                codes.push(bg.bg_code());
            }

            if codes.is_empty() {
                String::new()
            } else {
                format!("\x1b[{}m", codes.join(";"))
            }
        }
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::new().fg(color)
    }
}

// ===========================================================================
// Console
// ===========================================================================

/// A styled string plus a chainable builder API.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::Console;
///
/// let warning = Console::new("deprecated").yellow().italic();
/// assert_eq!(
///     warning.to_string(),
///     "\x1b[3;38;5;3mdeprecated\x1b[0m"
/// );
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Console {
    text: String,
    style: Style,
}

impl Console {
    /// Creates a new `Console` with plain (unstyled) text.
    #[must_use]
    pub fn new<T: Into<String>>(text: T) -> Self {
        Console {
            text: text.into(),
            style: Style::default(),
        }
    }

    /// Creates a new `Console` from text and a pre-built [`Style`].
    #[must_use]
    pub fn new_with_style<T: Into<String>>(text: T, style: Style) -> Self {
        Console {
            text: text.into(),
            style,
        }
    }

    /// Keeps the current style and replaces the text.
    ///
    /// Useful for reusing a configured style with different messages.
    #[must_use]
    pub fn with_text<T: Into<String>>(&self, text: T) -> Self {
        Console {
            text: text.into(),
            style: self.style.clone(),
        }
    }

    /// Returns the unstyled text held by this `Console`.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the [`Style`] held by this `Console`.
    #[must_use]
    pub const fn style(&self) -> &Style {
        &self.style
    }

    // ----- color setters ---------------------------------------------------

    /// Sets the foreground color.
    #[must_use]
    pub fn fg(self, color: Color) -> Self {
        Console {
            style: self.style.fg(color),
            ..self
        }
    }

    /// Sets the background color.
    #[must_use]
    pub fn bg(self, color: Color) -> Self {
        Console {
            style: self.style.bg(color),
            ..self
        }
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

    // ----- attribute setters -----------------------------------------------

    /// Adds a single [`Attribute`].
    #[must_use]
    pub fn attr(self, attribute: Attribute) -> Self {
        Console {
            style: self.style.attr(attribute),
            ..self
        }
    }

    /// Adds the bold attribute.
    #[must_use]
    pub fn bold(self) -> Self {
        Console {
            style: self.style.bold(),
            ..self
        }
    }

    /// Adds the dim attribute.
    #[must_use]
    pub fn dim(self) -> Self {
        Console {
            style: self.style.dim(),
            ..self
        }
    }

    /// Adds the italic attribute.
    #[must_use]
    pub fn italic(self) -> Self {
        Console {
            style: self.style.italic(),
            ..self
        }
    }

    /// Adds the underline attribute.
    #[must_use]
    pub fn underline(self) -> Self {
        Console {
            style: self.style.underline(),
            ..self
        }
    }

    /// Adds the blink attribute.
    #[must_use]
    pub fn blink(self) -> Self {
        Console {
            style: self.style.blink(),
            ..self
        }
    }

    /// Adds the reverse attribute.
    #[must_use]
    pub fn reverse(self) -> Self {
        Console {
            style: self.style.reverse(),
            ..self
        }
    }

    /// Adds the hidden attribute.
    #[must_use]
    pub fn hidden(self) -> Self {
        Console {
            style: self.style.hidden(),
            ..self
        }
    }

    /// Adds the strikethrough attribute.
    #[must_use]
    pub fn strikethrough(self) -> Self {
        Console {
            style: self.style.strikethrough(),
            ..self
        }
    }

    // ----- log level helpers ------------------------------------------------

    /// Builds an informational message (`[INFO]`, cyan).
    #[must_use]
    pub fn info<T: Into<String>>(text: T) -> Self {
        Self::tagged("[INFO]", text, Color::CYAN, false)
    }

    /// Builds a warning message (`[WARN]`, yellow).
    #[must_use]
    pub fn warn<T: Into<String>>(text: T) -> Self {
        Self::tagged("[WARN]", text, Color::YELLOW, false)
    }

    /// Builds an error message (`[ERROR]`, bold red).
    #[must_use]
    pub fn error<T: Into<String>>(text: T) -> Self {
        Self::tagged("[ERROR]", text, Color::RED, true)
    }

    /// Builds a success message (`[OK]`, bold green).
    #[must_use]
    pub fn success<T: Into<String>>(text: T) -> Self {
        Self::tagged("[OK]", text, Color::GREEN, true)
    }

    /// Builds a debug message (`[DEBUG]`, dim).
    #[must_use]
    pub fn debug<T: Into<String>>(text: T) -> Self {
        Console::new(format!("[DEBUG] {}", text.into())).dim()
    }

    fn tagged<T: Into<String>>(tag: &str, text: T, color: Color, bold: bool) -> Self {
        let console = Console::new(format!("{tag} {}", text.into())).fg(color);
        if bold { console.bold() } else { console }
    }

    // ----- decorations -------------------------------------------------------

    /// Builds a full-width horizontal banner with a centered title.
    ///
    /// The banner is cyan and bold. When `width` is too small to fit the title
    /// with at least two dashes on each side, the plain title is returned
    /// styled instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Console;
    ///
    /// # #[cfg(not(feature = "no-color"))]
    /// # {
    /// use pretty_console::Console;
    ///
    /// let banner = Console::banner("Setup", 21).to_string();
    /// assert_eq!(banner, "\x1b[1;38;5;6m─────── Setup ───────\x1b[0m");
    /// # }
    /// ```
    #[must_use]
    pub fn banner<T: Into<String>>(title: T, width: usize) -> Self {
        let title = title.into();
        let title_width = title.chars().count();
        let dashes = width.saturating_sub(title_width + 2);
        if dashes < 4 {
            return Self::new(title).cyan().bold();
        }
        let left = "─".repeat(dashes / 2);
        let right = "─".repeat(dashes - dashes / 2);
        Self::new(format!("{left} {title} {right}")).cyan().bold()
    }

    // ----- output ------------------------------------------------------------

    /// Renders the styled text, honoring the global [`ColorChoice`].
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if is_color_enabled() {
            writer.write_all(self.render().as_bytes())?;
        } else {
            writer.write_all(self.text.as_bytes())?;
        }
        Ok(())
    }

    /// Prints the styled text to stdout without a trailing newline.
    ///
    /// Printing honors the global [`ColorChoice`].
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails (for example on a broken pipe).
    pub fn print(&self) {
        let mut stdout = io::stdout().lock();
        self.write_to(&mut stdout)
            .expect("failed to write to stdout");
    }

    /// Prints the styled text to stdout followed by a newline.
    ///
    /// Printing honors the global [`ColorChoice`].
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails (for example on a broken pipe).
    pub fn println(&self) {
        let mut stdout = io::stdout().lock();
        self.write_to(&mut stdout)
            .expect("failed to write to stdout");
        stdout.write_all(b"\n").expect("failed to write to stdout");
    }

    fn render(&self) -> String {
        let start = self.style.to_ansi_start();
        if start.is_empty() {
            self.text.clone()
        } else {
            format!("{start}{}{RESET}", self.text)
        }
    }
}

impl fmt::Display for Console {
    /// Renders the styled text. When a width is requested, the padding is
    /// computed from the *visible* text length, not from the ANSI-decorated
    /// string, so `format!("{:>20}", console)` aligns as expected.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body = self.render();
        let Some(width) = f.width() else {
            return f.write_str(&body);
        };
        let padding = width.saturating_sub(self.text.chars().count());
        if padding == 0 {
            return f.write_str(&body);
        }
        let fill = f.fill().to_string();
        match f.align() {
            Some(fmt::Alignment::Right) => write!(f, "{}{body}", fill.repeat(padding)),
            Some(fmt::Alignment::Center) => {
                let left = padding / 2;
                write!(
                    f,
                    "{}{body}{}",
                    fill.repeat(left),
                    fill.repeat(padding - left)
                )
            }
            _ => write!(f, "{body}{}", fill.repeat(padding)),
        }
    }
}

// ===========================================================================
// Styled trait
// ===========================================================================

/// Extension trait that paints string-like values directly.
///
/// ```
/// use pretty_console::Styled;
///
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::Styled;
///
/// assert_eq!("hello".red().bold().to_string(), "\x1b[1;38;5;1mhello\x1b[0m");
/// # }
/// ```
pub trait Styled {
    /// Wraps the value in a [`Console`] with a plain style.
    fn styled(&self) -> Console;

    /// Foreground black.
    fn black(&self) -> Console {
        self.styled().black()
    }

    /// Foreground red.
    fn red(&self) -> Console {
        self.styled().red()
    }

    /// Foreground green.
    fn green(&self) -> Console {
        self.styled().green()
    }

    /// Foreground yellow.
    fn yellow(&self) -> Console {
        self.styled().yellow()
    }

    /// Foreground blue.
    fn blue(&self) -> Console {
        self.styled().blue()
    }

    /// Foreground magenta.
    fn magenta(&self) -> Console {
        self.styled().magenta()
    }

    /// Foreground cyan.
    fn cyan(&self) -> Console {
        self.styled().cyan()
    }

    /// Foreground white.
    fn white(&self) -> Console {
        self.styled().white()
    }

    /// Foreground bright black.
    fn bright_black(&self) -> Console {
        self.styled().bright_black()
    }

    /// Foreground bright red.
    fn bright_red(&self) -> Console {
        self.styled().bright_red()
    }

    /// Foreground bright green.
    fn bright_green(&self) -> Console {
        self.styled().bright_green()
    }

    /// Foreground bright yellow.
    fn bright_yellow(&self) -> Console {
        self.styled().bright_yellow()
    }

    /// Foreground bright blue.
    fn bright_blue(&self) -> Console {
        self.styled().bright_blue()
    }

    /// Foreground bright magenta.
    fn bright_magenta(&self) -> Console {
        self.styled().bright_magenta()
    }

    /// Foreground bright cyan.
    fn bright_cyan(&self) -> Console {
        self.styled().bright_cyan()
    }

    /// Foreground bright white.
    fn bright_white(&self) -> Console {
        self.styled().bright_white()
    }

    /// Background black.
    fn on_black(&self) -> Console {
        self.styled().on_black()
    }

    /// Background red.
    fn on_red(&self) -> Console {
        self.styled().on_red()
    }

    /// Background green.
    fn on_green(&self) -> Console {
        self.styled().on_green()
    }

    /// Background yellow.
    fn on_yellow(&self) -> Console {
        self.styled().on_yellow()
    }

    /// Background blue.
    fn on_blue(&self) -> Console {
        self.styled().on_blue()
    }

    /// Background magenta.
    fn on_magenta(&self) -> Console {
        self.styled().on_magenta()
    }

    /// Background cyan.
    fn on_cyan(&self) -> Console {
        self.styled().on_cyan()
    }

    /// Background white.
    fn on_white(&self) -> Console {
        self.styled().on_white()
    }

    /// Background bright black.
    fn on_bright_black(&self) -> Console {
        self.styled().on_bright_black()
    }

    /// Background bright red.
    fn on_bright_red(&self) -> Console {
        self.styled().on_bright_red()
    }

    /// Background bright green.
    fn on_bright_green(&self) -> Console {
        self.styled().on_bright_green()
    }

    /// Background bright yellow.
    fn on_bright_yellow(&self) -> Console {
        self.styled().on_bright_yellow()
    }

    /// Background bright blue.
    fn on_bright_blue(&self) -> Console {
        self.styled().on_bright_blue()
    }

    /// Background bright magenta.
    fn on_bright_magenta(&self) -> Console {
        self.styled().on_bright_magenta()
    }

    /// Background bright cyan.
    fn on_bright_cyan(&self) -> Console {
        self.styled().on_bright_cyan()
    }

    /// Background bright white.
    fn on_bright_white(&self) -> Console {
        self.styled().on_bright_white()
    }

    /// Bold attribute.
    fn bold(&self) -> Console {
        self.styled().bold()
    }

    /// Dim attribute.
    fn dim(&self) -> Console {
        self.styled().dim()
    }

    /// Italic attribute.
    fn italic(&self) -> Console {
        self.styled().italic()
    }

    /// Underline attribute.
    fn underline(&self) -> Console {
        self.styled().underline()
    }

    /// Blink attribute.
    fn blink(&self) -> Console {
        self.styled().blink()
    }

    /// Reverse attribute.
    fn reverse(&self) -> Console {
        self.styled().reverse()
    }

    /// Hidden attribute.
    fn hidden(&self) -> Console {
        self.styled().hidden()
    }

    /// Strikethrough attribute.
    fn strikethrough(&self) -> Console {
        self.styled().strikethrough()
    }
}

impl<T: AsRef<str> + ?Sized> Styled for T {
    fn styled(&self) -> Console {
        Console::new(self.as_ref())
    }
}

// ===========================================================================
// ANSI stripping
// ===========================================================================

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
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(current) = chars.next() {
        if current != '\x1b' {
            out.push(current);
            continue;
        }
        match chars.next() {
            // CSI: parameters (0x30-0x3F) and intermediates (0x20-0x2F),
            // terminated by a final byte (0x40-0x7E).
            Some('[') => loop {
                match chars.next() {
                    Some(next) if ('0'..='?').contains(&next) || (' '..='/').contains(&next) => {}
                    // Final byte ends the sequence; anything else (including
                    // end of input) is treated as a truncated sequence.
                    _ => break,
                }
            },
            // OSC: terminated by BEL or by ST (ESC \).
            Some(']') => loop {
                match chars.next() {
                    Some('\x07') | None => break,
                    Some('\x1b') => {
                        let _ = chars.next();
                        break;
                    }
                    Some(_) => {}
                }
            },
            // Any other escape sequence consumes exactly one more character;
            // a dangling ESC at end of input is simply dropped.
            _ => {}
        }
    }
    out
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "no-color"))]
    use std::collections::HashMap;
    use std::sync::Mutex;

    use super::*;

    /// Serializes tests that touch the global [`ColorChoice`] state.
    static GLOBAL_LOCK: Mutex<()> = Mutex::new(());

    /// One environment-detection test case: env snapshot, decision with a tty
    /// stdout, and decision without a tty stdout.
    #[cfg(not(feature = "no-color"))]
    type EnvCase = (&'static [(&'static str, &'static str)], bool, bool);

    /// A paint function from the [`Styled`] trait plus its expected color.
    type PaintCase = (fn(&str) -> Console, Color);

    // ----- Color -----------------------------------------------------------

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::BLACK, Color::Named(0));
        assert_eq!(Color::RED, Color::Named(1));
        assert_eq!(Color::GREEN, Color::Named(2));
        assert_eq!(Color::YELLOW, Color::Named(3));
        assert_eq!(Color::BLUE, Color::Named(4));
        assert_eq!(Color::MAGENTA, Color::Named(5));
        assert_eq!(Color::CYAN, Color::Named(6));
        assert_eq!(Color::WHITE, Color::Named(7));
        assert_eq!(Color::BRIGHT_BLACK, Color::Named(8));
        assert_eq!(Color::BRIGHT_RED, Color::Named(9));
        assert_eq!(Color::BRIGHT_GREEN, Color::Named(10));
        assert_eq!(Color::BRIGHT_YELLOW, Color::Named(11));
        assert_eq!(Color::BRIGHT_BLUE, Color::Named(12));
        assert_eq!(Color::BRIGHT_MAGENTA, Color::Named(13));
        assert_eq!(Color::BRIGHT_CYAN, Color::Named(14));
        assert_eq!(Color::BRIGHT_WHITE, Color::Named(15));
    }

    #[test]
    fn test_color_constructors() {
        assert_eq!(Color::indexed(200), Color::Named(200));
        assert_eq!(Color::rgb(1, 2, 3), Color::RGB(1, 2, 3));
        assert_eq!(Color::from(200_u8), Color::Named(200));
        assert_eq!(Color::from((9, 8, 7)), Color::RGB(9, 8, 7));
        assert_eq!(Color::hex(0x11_22_33), Color::RGB(0x11, 0x22, 0x33));
        assert_eq!(Color::hex(0x00_00_00), Color::RGB(0, 0, 0));
    }

    #[test]
    fn test_color_codes() {
        let color = Color::RED;
        assert_eq!(color.fg_code(), "38;5;1");
        assert_eq!(color.bg_code(), "48;5;1");

        let rgb = Color::RGB(255, 128, 0);
        assert_eq!(rgb.fg_code(), "38;2;255;128;0");
        assert_eq!(rgb.bg_code(), "48;2;255;128;0");

        assert_eq!(Color::BLACK.fg_code(), "38;5;0");
        assert_eq!(Color::BRIGHT_WHITE.fg_code(), "38;5;15");
        assert_eq!(Color::Named(200).bg_code(), "48;5;200");
    }

    // ----- Attribute --------------------------------------------------------

    #[test]
    fn test_attribute_codes() {
        assert_eq!(Attribute::Bold.code(), "1");
        assert_eq!(Attribute::Dim.code(), "2");
        assert_eq!(Attribute::Italic.code(), "3");
        assert_eq!(Attribute::Underline.code(), "4");
        assert_eq!(Attribute::Blink.code(), "5");
        assert_eq!(Attribute::Reverse.code(), "7");
        assert_eq!(Attribute::Hidden.code(), "8");
        assert_eq!(Attribute::Strikethrough.code(), "9");
    }

    // ----- Color choice -----------------------------------------------------

    #[cfg(not(feature = "no-color"))]
    fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<&str, String> = pairs
            .iter()
            .map(|(key, value)| (*key, (*value).to_string()))
            .collect();
        move |key: &str| map.get(key).cloned()
    }

    #[test]
    #[cfg(not(feature = "no-color"))]
    fn test_env_detection() {
        let cases: &[EnvCase] = &[
            (&[("FORCE_COLOR", "1")], true, true),
            (&[("FORCE_COLOR", "0")], false, false),
            (&[("FORCE_COLOR", "")], true, false),
            (&[("CLICOLOR_FORCE", "1")], true, true),
            (&[("CLICOLOR_FORCE", "0")], true, false),
            (&[("NO_COLOR", "1")], false, false),
            (&[("NO_COLOR", "")], true, false),
            (&[("CLICOLOR", "0")], false, false),
            (&[("TERM", "dumb")], false, false),
            (&[], true, false),
            (
                &[("FORCE_COLOR", "0"), ("CLICOLOR_FORCE", "1")],
                false,
                false,
            ),
        ];

        for (env, with_tty, without_tty) in cases {
            assert_eq!(
                env_enabled(env_of(env), true),
                *with_tty,
                "env {env:?} with tty"
            );
            assert_eq!(
                env_enabled(env_of(env), false),
                *without_tty,
                "env {env:?} without tty"
            );
        }
    }

    #[test]
    fn test_color_choice_roundtrip() {
        let _guard = GLOBAL_LOCK.lock().unwrap();

        assert_eq!(color_choice(), ColorChoice::Auto);
        let _ = is_color_enabled();

        set_color_choice(ColorChoice::Always);
        assert_eq!(color_choice(), ColorChoice::Always);
        #[cfg(not(feature = "no-color"))]
        assert!(is_color_enabled());

        set_color_choice(ColorChoice::Never);
        assert_eq!(color_choice(), ColorChoice::Never);
        assert!(!is_color_enabled());

        set_color_choice(ColorChoice::Auto);
        assert_eq!(color_choice(), ColorChoice::Auto);
    }

    // ----- Style ------------------------------------------------------------

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .fg(Color::RED)
            .bg(Color::BLUE)
            .bold()
            .underline();

        #[cfg(not(feature = "no-color"))]
        {
            let ansi = style.to_ansi_start();
            assert!(ansi.contains('1')); // bold
            assert!(ansi.contains('4')); // underline
            assert!(ansi.contains("38;5;1")); // red foreground
            assert!(ansi.contains("48;5;4")); // blue background
        }

        #[cfg(feature = "no-color")]
        assert_eq!(style.to_ansi_start(), "");
    }

    #[test]
    fn test_style_empty() {
        assert_eq!(Style::new().to_ansi_start(), "");
    }

    #[test]
    fn test_style_only_foreground() {
        let style = Style::new().fg(Color::GREEN);

        #[cfg(not(feature = "no-color"))]
        assert_eq!(style.to_ansi_start(), "\x1b[38;5;2m");
        #[cfg(feature = "no-color")]
        assert_eq!(style.to_ansi_start(), "");
    }

    #[test]
    fn test_style_only_background() {
        let style = Style::new().bg(Color::YELLOW);

        #[cfg(not(feature = "no-color"))]
        assert_eq!(style.to_ansi_start(), "\x1b[48;5;3m");
        #[cfg(feature = "no-color")]
        assert_eq!(style.to_ansi_start(), "");
    }

    #[test]
    fn test_style_only_attributes() {
        let style = Style::new().bold().italic();

        #[cfg(not(feature = "no-color"))]
        {
            let ansi = style.to_ansi_start();
            assert!(ansi.contains('1'));
            assert!(ansi.contains('3'));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(style.to_ansi_start(), "");
    }

    #[test]
    fn test_style_rgb_colors() {
        let style = Style::new()
            .fg(Color::RGB(255, 0, 0))
            .bg(Color::RGB(0, 255, 0));

        #[cfg(not(feature = "no-color"))]
        {
            let ansi = style.to_ansi_start();
            assert!(ansi.contains("38;2;255;0;0"));
            assert!(ansi.contains("48;2;0;255;0"));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(style.to_ansi_start(), "");
    }

    #[test]
    fn test_style_accessors() {
        let style = Style::new();
        assert!(style.is_plain());
        assert_eq!(style.foreground(), None);
        assert_eq!(style.background(), None);
        assert_eq!(style.attributes(), &[]);

        let styled = style.fg(Color::RED).bg(Color::BLUE).bold();
        assert!(!styled.is_plain());
        assert_eq!(styled.foreground(), Some(Color::RED));
        assert_eq!(styled.background(), Some(Color::BLUE));
        assert_eq!(styled.attributes(), &[Attribute::Bold]);
    }

    #[test]
    fn test_style_merge() {
        let base = Style::new().fg(Color::RED).underline();
        let replacement = Style::new().fg(Color::BLUE).bold();

        let merged = base.clone().merge(replacement);
        assert_eq!(merged.foreground(), Some(Color::BLUE));
        assert_eq!(
            merged.attributes(),
            &[Attribute::Underline, Attribute::Bold]
        );

        let merged_with_empty = base.clone().merge(Style::new());
        assert_eq!(merged_with_empty.foreground(), Some(Color::RED));
        assert_eq!(merged_with_empty.attributes(), &[Attribute::Underline]);

        let background_merge = base.merge(Style::new().bg(Color::GREEN));
        assert_eq!(background_merge.background(), Some(Color::GREEN));
        assert_eq!(background_merge.foreground(), Some(Color::RED));

        assert!(Style::new().merge(Style::new()).is_plain());
    }

    #[test]
    fn test_style_from_color() {
        let style = Style::from(Color::RED);
        assert_eq!(style.foreground(), Some(Color::RED));
        assert_eq!(style.background(), None);
        assert_eq!(style.attributes(), &[]);
    }

    #[test]
    fn test_color_and_attribute_copy() {
        let color = Color::RED;
        let attr = Attribute::Bold;

        let style1 = Style::new().fg(color).attr(attr);
        let style2 = Style::new().fg(color).attr(attr);

        assert_eq!(style1.foreground, style2.foreground);
        assert_eq!(style1.attributes, style2.attributes);
    }

    // ----- Console basics -----------------------------------------------------

    #[test]
    fn test_console_creation() {
        let console = Console::new("test");
        assert_eq!(console.text, "test");

        let console_from_string = Console::new(String::from("test"));
        assert_eq!(console_from_string.text, "test");
    }

    #[test]
    fn test_console_new_with_style() {
        let style = Style::new().fg(Color::RED).bold();
        let console = Console::new_with_style("styled", style.clone());

        assert_eq!(console.text(), "styled");
        assert_eq!(console.style(), &style);
    }

    #[test]
    fn test_console_accessors_and_default() {
        let console = Console::default();
        assert_eq!(console.text(), "");
        assert!(console.style().is_plain());

        let styled = Console::new("data").blue();
        assert_eq!(styled.text(), "data");
        assert_eq!(styled.style().foreground(), Some(Color::BLUE));
    }

    #[test]
    fn test_console_with_text() {
        let base = Console::new("").red().bold();
        let derived = base.with_text("new text");

        assert_eq!(derived.text, "new text");
        assert_eq!(derived.style.foreground, base.style.foreground);
        assert_eq!(derived.style.attributes, base.style.attributes);
    }

    #[test]
    fn test_console_color_methods() {
        let console = Console::new("test").red();
        assert_eq!(console.style.foreground, Some(Color::RED));

        let console = Console::new("test").on_blue();
        assert_eq!(console.style.background, Some(Color::BLUE));

        let console = Console::new("test").fg_rgb(255, 0, 0);
        assert_eq!(console.style.foreground, Some(Color::RGB(255, 0, 0)));

        let console = Console::new("test").bg_rgb(0, 255, 0);
        assert_eq!(console.style.background, Some(Color::RGB(0, 255, 0)));

        let console = Console::new("test").fg(Color::CYAN).bg(Color::MAGENTA);
        assert_eq!(console.style.foreground, Some(Color::CYAN));
        assert_eq!(console.style.background, Some(Color::MAGENTA));
    }

    #[test]
    fn test_console_attribute_methods() {
        let console = Console::new("test")
            .bold()
            .dim()
            .italic()
            .underline()
            .blink()
            .reverse()
            .hidden()
            .strikethrough();

        let attrs = &console.style.attributes;
        assert!(attrs.contains(&Attribute::Bold));
        assert!(attrs.contains(&Attribute::Dim));
        assert!(attrs.contains(&Attribute::Italic));
        assert!(attrs.contains(&Attribute::Underline));
        assert!(attrs.contains(&Attribute::Blink));
        assert!(attrs.contains(&Attribute::Reverse));
        assert!(attrs.contains(&Attribute::Hidden));
        assert!(attrs.contains(&Attribute::Strikethrough));

        let console = Console::new("test").attr(Attribute::Reverse);
        assert!(console.style.attributes.contains(&Attribute::Reverse));
    }

    #[test]
    fn test_all_named_colors() {
        let colors = [
            (Console::new("").black(), Color::BLACK),
            (Console::new("").red(), Color::RED),
            (Console::new("").green(), Color::GREEN),
            (Console::new("").yellow(), Color::YELLOW),
            (Console::new("").blue(), Color::BLUE),
            (Console::new("").magenta(), Color::MAGENTA),
            (Console::new("").cyan(), Color::CYAN),
            (Console::new("").white(), Color::WHITE),
            (Console::new("").bright_black(), Color::BRIGHT_BLACK),
            (Console::new("").bright_red(), Color::BRIGHT_RED),
            (Console::new("").bright_green(), Color::BRIGHT_GREEN),
            (Console::new("").bright_yellow(), Color::BRIGHT_YELLOW),
            (Console::new("").bright_blue(), Color::BRIGHT_BLUE),
            (Console::new("").bright_magenta(), Color::BRIGHT_MAGENTA),
            (Console::new("").bright_cyan(), Color::BRIGHT_CYAN),
            (Console::new("").bright_white(), Color::BRIGHT_WHITE),
        ];

        for (console, expected_color) in colors {
            assert_eq!(console.style.foreground, Some(expected_color));
        }
    }

    #[test]
    fn test_all_background_colors() {
        let backgrounds = [
            (Console::new("").on_black(), Color::BLACK),
            (Console::new("").on_red(), Color::RED),
            (Console::new("").on_green(), Color::GREEN),
            (Console::new("").on_yellow(), Color::YELLOW),
            (Console::new("").on_blue(), Color::BLUE),
            (Console::new("").on_magenta(), Color::MAGENTA),
            (Console::new("").on_cyan(), Color::CYAN),
            (Console::new("").on_white(), Color::WHITE),
            (Console::new("").on_bright_black(), Color::BRIGHT_BLACK),
            (Console::new("").on_bright_red(), Color::BRIGHT_RED),
            (Console::new("").on_bright_green(), Color::BRIGHT_GREEN),
            (Console::new("").on_bright_yellow(), Color::BRIGHT_YELLOW),
            (Console::new("").on_bright_blue(), Color::BRIGHT_BLUE),
            (Console::new("").on_bright_magenta(), Color::BRIGHT_MAGENTA),
            (Console::new("").on_bright_cyan(), Color::BRIGHT_CYAN),
            (Console::new("").on_bright_white(), Color::BRIGHT_WHITE),
        ];

        for (console, expected_color) in backgrounds {
            assert_eq!(console.style.background, Some(expected_color));
        }
    }

    #[test]
    fn test_console_clone() {
        let original = Console::new("original").red().bold();
        let cloned = original.clone();

        assert_eq!(original.text, cloned.text);
        assert_eq!(original.style.foreground, cloned.style.foreground);
        assert_eq!(original.style.background, cloned.style.background);
        assert_eq!(original.style.attributes, cloned.style.attributes);
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_multiple_identical_attributes() {
        let console = Console::new("test").bold().bold().bold();

        let bold_count = console
            .style
            .attributes
            .iter()
            .filter(|&&attr| attr == Attribute::Bold)
            .count();
        assert_eq!(bold_count, 3);
    }

    #[test]
    #[cfg(not(feature = "no-color"))]
    fn test_ansi_code_ordering() {
        // Attributes first, then foreground, then background.
        let style = Style::new()
            .bg(Color::BLUE)
            .fg(Color::RED)
            .underline()
            .bold();

        let ansi = style.to_ansi_start();
        // Insertion order: underline was added before bold.
        assert_eq!(ansi, "\x1b[4;1;38;5;1;48;5;4m");
    }

    // ----- Console rendering ---------------------------------------------------

    #[test]
    fn test_console_display_trait() {
        let console = Console::new("hello").red().bold();
        let output = format!("{console}");

        #[cfg(not(feature = "no-color"))]
        {
            assert!(output.starts_with("\x1b["));
            assert!(output.contains("hello"));
            assert!(output.ends_with("\x1b[0m"));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(output, "hello");
    }

    #[test]
    fn test_console_to_string() {
        let console = Console::new("test").blue().underline();
        let string = console.to_string();

        #[cfg(not(feature = "no-color"))]
        {
            assert!(string.contains("test"));
            assert!(string.contains("38;5;4")); // blue
            assert!(string.contains('4')); // underline
        }
        #[cfg(feature = "no-color")]
        assert_eq!(string, "test");
    }

    #[test]
    fn test_console_complex_styling() {
        let console = Console::new("complex")
            .fg_rgb(128, 64, 255)
            .on_bright_white()
            .bold()
            .italic()
            .underline();

        let output = console.to_string();

        #[cfg(not(feature = "no-color"))]
        {
            assert!(output.contains("complex"));
            assert!(output.contains("38;2;128;64;255"));
            assert!(output.contains("48;5;15"));
            assert!(output.contains('1'));
            assert!(output.contains('3'));
            assert!(output.contains('4'));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(output, "complex");
    }

    #[test]
    fn test_console_method_chaining() {
        let console = Console::new("test").red().on_white().bold().underline();

        assert_eq!(console.style.foreground, Some(Color::RED));
        assert_eq!(console.style.background, Some(Color::WHITE));
        assert!(console.style.attributes.contains(&Attribute::Bold));
        assert!(console.style.attributes.contains(&Attribute::Underline));
    }

    #[test]
    fn test_console_empty_text() {
        let console = Console::new("").red().bold();
        let output = console.to_string();

        #[cfg(not(feature = "no-color"))]
        {
            assert!(output.starts_with("\x1b["));
            assert!(output.ends_with("\x1b[0m"));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(output, "");
    }

    #[test]
    fn test_display_width_and_alignment() {
        let console = Console::new("hi").red();
        let styled = console.to_string();

        assert_eq!(format!("{console:10}"), format!("{styled}        "));
        assert_eq!(format!("{console:>10}"), format!("        {styled}"));
        assert_eq!(format!("{console:^11}"), format!("    {styled}     "));
        assert_eq!(format!("{console:*^11}"), format!("****{styled}*****"));
        assert_eq!(format!("{console:2}"), styled);

        let plain = Console::new("hi");
        assert_eq!(format!("{plain:>6}"), "    hi");

        let wide = Console::new("longer text");
        assert_eq!(format!("{wide:5}"), wide.to_string());
    }

    #[test]
    fn test_console_write_to() {
        let _guard = GLOBAL_LOCK.lock().unwrap();

        set_color_choice(ColorChoice::Always);
        let mut buffer = Vec::new();
        Console::new("write test")
            .green()
            .write_to(&mut buffer)
            .unwrap();
        set_color_choice(ColorChoice::Auto);
        let output = String::from_utf8(buffer).unwrap();

        #[cfg(not(feature = "no-color"))]
        {
            assert!(output.contains("write test"));
            assert!(output.contains("38;5;2")); // green
            assert!(output.ends_with("\x1b[0m"));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(output, "write test");
    }

    #[test]
    fn test_write_to_respects_color_choice() {
        let _guard = GLOBAL_LOCK.lock().unwrap();

        set_color_choice(ColorChoice::Never);
        let mut buffer = Vec::new();
        Console::new("plain").red().write_to(&mut buffer).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), "plain");

        set_color_choice(ColorChoice::Always);
        let mut buffer = Vec::new();
        Console::new("colored").red().write_to(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        #[cfg(not(feature = "no-color"))]
        assert_eq!(output, "\x1b[38;5;1mcolored\x1b[0m");
        #[cfg(feature = "no-color")]
        assert_eq!(output, "colored");

        set_color_choice(ColorChoice::Auto);
    }

    #[test]
    fn test_print_and_println() {
        let _guard = GLOBAL_LOCK.lock().unwrap();

        set_color_choice(ColorChoice::Never);
        Console::new("stdout out").red().print();
        Console::new("stdout line").blue().println();

        set_color_choice(ColorChoice::Always);
        Console::new("styled out").green().print();
        Console::new("styled line").yellow().println();

        set_color_choice(ColorChoice::Auto);
    }

    // ----- Log level helpers -----------------------------------------------------

    #[test]
    fn test_log_level_helpers() {
        let info = Console::info("starting");
        assert_eq!(info.text(), "[INFO] starting");
        assert_eq!(info.style().foreground(), Some(Color::CYAN));
        assert_eq!(info.style().attributes(), &[]);

        let warn = Console::warn("low disk");
        assert_eq!(warn.text(), "[WARN] low disk");
        assert_eq!(warn.style().foreground(), Some(Color::YELLOW));
        assert_eq!(warn.style().attributes(), &[]);

        let error = Console::error("boom");
        assert_eq!(error.text(), "[ERROR] boom");
        assert_eq!(error.style().foreground(), Some(Color::RED));
        assert!(error.style().attributes().contains(&Attribute::Bold));

        let success = Console::success("done");
        assert_eq!(success.text(), "[OK] done");
        assert_eq!(success.style().foreground(), Some(Color::GREEN));
        assert!(success.style().attributes().contains(&Attribute::Bold));

        let debug = Console::debug("vars");
        assert_eq!(debug.text(), "[DEBUG] vars");
        assert!(debug.style().attributes().contains(&Attribute::Dim));
    }

    #[test]
    fn test_banner() {
        let banner = Console::banner("Setup", 21);
        assert_eq!(banner.text(), "─────── Setup ───────");
        assert_eq!(banner.style().foreground(), Some(Color::CYAN));
        assert!(banner.style().attributes().contains(&Attribute::Bold));

        let exact = Console::banner("ab", 8);
        assert_eq!(exact.text(), "── ab ──");

        let small = Console::banner("toolongtitle", 10);
        assert_eq!(small.text(), "toolongtitle");
        assert_eq!(small.style().foreground(), Some(Color::CYAN));
    }

    // ----- Styled trait ---------------------------------------------------------

    #[test]
    fn test_styled_trait_foreground_methods() {
        let cases: Vec<PaintCase> = vec![
            (<str as Styled>::black, Color::BLACK),
            (<str as Styled>::red, Color::RED),
            (<str as Styled>::green, Color::GREEN),
            (<str as Styled>::yellow, Color::YELLOW),
            (<str as Styled>::blue, Color::BLUE),
            (<str as Styled>::magenta, Color::MAGENTA),
            (<str as Styled>::cyan, Color::CYAN),
            (<str as Styled>::white, Color::WHITE),
            (<str as Styled>::bright_black, Color::BRIGHT_BLACK),
            (<str as Styled>::bright_red, Color::BRIGHT_RED),
            (<str as Styled>::bright_green, Color::BRIGHT_GREEN),
            (<str as Styled>::bright_yellow, Color::BRIGHT_YELLOW),
            (<str as Styled>::bright_blue, Color::BRIGHT_BLUE),
            (<str as Styled>::bright_magenta, Color::BRIGHT_MAGENTA),
            (<str as Styled>::bright_cyan, Color::BRIGHT_CYAN),
            (<str as Styled>::bright_white, Color::BRIGHT_WHITE),
        ];

        for (paint, expected) in cases {
            let console = paint("x");
            assert_eq!(console.style().foreground(), Some(expected));
            assert_eq!(console.text(), "x");
        }
    }

    #[test]
    fn test_styled_trait_background_methods() {
        let cases: Vec<PaintCase> = vec![
            (<str as Styled>::on_black, Color::BLACK),
            (<str as Styled>::on_red, Color::RED),
            (<str as Styled>::on_green, Color::GREEN),
            (<str as Styled>::on_yellow, Color::YELLOW),
            (<str as Styled>::on_blue, Color::BLUE),
            (<str as Styled>::on_magenta, Color::MAGENTA),
            (<str as Styled>::on_cyan, Color::CYAN),
            (<str as Styled>::on_white, Color::WHITE),
            (<str as Styled>::on_bright_black, Color::BRIGHT_BLACK),
            (<str as Styled>::on_bright_red, Color::BRIGHT_RED),
            (<str as Styled>::on_bright_green, Color::BRIGHT_GREEN),
            (<str as Styled>::on_bright_yellow, Color::BRIGHT_YELLOW),
            (<str as Styled>::on_bright_blue, Color::BRIGHT_BLUE),
            (<str as Styled>::on_bright_magenta, Color::BRIGHT_MAGENTA),
            (<str as Styled>::on_bright_cyan, Color::BRIGHT_CYAN),
            (<str as Styled>::on_bright_white, Color::BRIGHT_WHITE),
        ];

        for (paint, expected) in cases {
            let console = paint("x");
            assert_eq!(console.style().background(), Some(expected));
            assert_eq!(console.text(), "x");
        }
    }

    #[test]
    fn test_styled_trait_attribute_methods() {
        let cases: Vec<fn(&str) -> Console> = vec![
            <str as Styled>::bold,
            <str as Styled>::dim,
            <str as Styled>::italic,
            <str as Styled>::underline,
            <str as Styled>::blink,
            <str as Styled>::reverse,
            <str as Styled>::hidden,
            <str as Styled>::strikethrough,
        ];
        let attributes = [
            Attribute::Bold,
            Attribute::Dim,
            Attribute::Italic,
            Attribute::Underline,
            Attribute::Blink,
            Attribute::Reverse,
            Attribute::Hidden,
            Attribute::Strikethrough,
        ];

        for (paint, expected) in cases.into_iter().zip(attributes) {
            let console = paint("x");
            assert_eq!(console.style().attributes(), &[expected]);
            assert_eq!(console.text(), "x");
        }
    }

    #[test]
    fn test_styled_trait_on_owned_string() {
        let text = String::from("owned");
        assert_eq!(text.red().style().foreground(), Some(Color::RED));
        assert_eq!(text.styled().text(), "owned");
        assert_eq!("literal".green().text(), "literal");
    }

    // ----- strip_ansi -------------------------------------------------------

    #[test]
    fn test_strip_ansi() {
        assert_eq!(strip_ansi("plain"), "plain");
        assert_eq!(strip_ansi(""), "");
        assert_eq!(strip_ansi("\x1b[1;31mred\x1b[0m"), "red");
        assert_eq!(strip_ansi("\x1b[38;2;1;2;3mrgb\x1b[0m"), "rgb");
        assert_eq!(strip_ansi("\x1b[?25hvisible"), "visible");
        assert_eq!(strip_ansi("text\x1b[31"), "text");
        assert_eq!(strip_ansi("\x1b]0;window title\x07rest"), "rest");
        assert_eq!(
            strip_ansi("\x1b]8;;https://example.com\x1b\\link\x1b]8;;\x1b\\"),
            "link"
        );
        assert_eq!(strip_ansi("a\x1bMb"), "ab");
        assert_eq!(strip_ansi("done\x1b"), "done");
        assert_eq!(strip_ansi("\x1b[1;31ma\x1b[0mb\x1b[32mc\x1b[0m"), "abc");
    }

    // ----- Snapshot tests (default features) ----------------------------------

    #[cfg(not(feature = "no-color"))]
    #[test]
    fn test_basic_colors() {
        let console = Console::new("Hello, world!").red().bold();
        insta::assert_yaml_snapshot!(console.to_string());
    }

    #[cfg(not(feature = "no-color"))]
    #[test]
    fn test_rgb_colors() {
        let console = Console::new("RGB Text")
            .fg_rgb(255, 0, 128)
            .bg_rgb(0, 255, 0)
            .underline();
        insta::assert_yaml_snapshot!(console.to_string());
    }

    #[cfg(not(feature = "no-color"))]
    #[test]
    fn test_complex_styling() {
        let console = Console::new("Complex Style")
            .bright_red()
            .on_bright_white()
            .bold()
            .italic()
            .blink();
        insta::assert_yaml_snapshot!(console.to_string());
    }

    #[cfg(not(feature = "no-color"))]
    #[test]
    fn test_style_combinations() {
        let styles = [
            Console::new("Error style").red().bold(),
            Console::new("Warning style").yellow().italic(),
            Console::new("Success style").green().bold(),
        ];

        let outputs: Vec<String> = styles.iter().map(Console::to_string).collect();
        insta::assert_yaml_snapshot!(outputs);
    }

    #[cfg(not(feature = "no-color"))]
    #[test]
    fn test_reusable_style() {
        let error_style = Console::new("").red().bold();
        let messages = vec![
            error_style.with_text("Error: File not found").to_string(),
            error_style
                .with_text("Error: Permission denied")
                .to_string(),
        ];

        insta::assert_yaml_snapshot!(messages);
    }

    // ----- Snapshot equivalents (no-color feature) ----------------------------

    #[cfg(feature = "no-color")]
    #[test]
    fn test_no_color_disables_all_output() {
        assert_eq!(
            Console::new("Hello, world!").red().bold().to_string(),
            "Hello, world!"
        );
        assert_eq!(
            Console::new("RGB Text")
                .fg_rgb(255, 0, 128)
                .bg_rgb(0, 255, 0)
                .underline()
                .to_string(),
            "RGB Text"
        );
        assert_eq!(
            Console::new("Complex Style")
                .bright_red()
                .on_bright_white()
                .bold()
                .italic()
                .blink()
                .to_string(),
            "Complex Style"
        );
        assert_eq!(
            Console::new("Complex")
                .bright_red()
                .on_bright_white()
                .to_string(),
            "Complex"
        );
        assert_eq!(
            Console::banner("Setup", 21).to_string(),
            "─────── Setup ───────"
        );
    }
}
