//! Reusable, zero-allocation styles.

use std::fmt;

use crate::attr::{Attribute, Effects};
use crate::color::Color;
#[cfg(not(feature = "no-color"))]
use crate::color::RESET;

/// A reusable collection of colors and attributes.
///
/// A `Style` is [`Copy`] and can be built in `const` contexts — building,
/// merging and cloning styles never allocates. Rendering writes SGR codes
/// straight into the destination formatter.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "no-color"))]
/// # {
/// use pretty_console::{Color, Style};
///
/// let heading = Style::new().bold().underline().fg(Color::BRIGHT_BLUE);
/// assert_eq!(
///     heading.paint("Chapter 1").to_string(),
///     "\x1b[1;4;38;5;12mChapter 1\x1b[0m"
/// );
/// # }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Style {
    pub(crate) foreground: Option<Color>,
    pub(crate) background: Option<Color>,
    pub(crate) effects: Effects,
}

impl Style {
    /// Creates an empty style with no colors or attributes.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            foreground: None,
            background: None,
            effects: Effects::EMPTY,
        }
    }

    /// Sets the foreground color.
    #[must_use]
    pub const fn fg(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Sets the background color.
    #[must_use]
    pub const fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Adds a single [`Attribute`].
    #[must_use]
    pub const fn attr(mut self, attribute: Attribute) -> Self {
        self.effects = self.effects.insert(attribute);
        self
    }

    /// Unions a whole [`Effects`] set into the style.
    #[must_use]
    pub const fn effects(mut self, effects: Effects) -> Self {
        self.effects = self.effects.union(effects);
        self
    }

    /// Adds the bold attribute.
    #[must_use]
    pub const fn bold(self) -> Self {
        self.attr(Attribute::Bold)
    }

    /// Adds the dim attribute.
    #[must_use]
    pub const fn dim(self) -> Self {
        self.attr(Attribute::Dim)
    }

    /// Adds the italic attribute.
    #[must_use]
    pub const fn italic(self) -> Self {
        self.attr(Attribute::Italic)
    }

    /// Adds the underline attribute.
    #[must_use]
    pub const fn underline(self) -> Self {
        self.attr(Attribute::Underline)
    }

    /// Adds the blink attribute.
    #[must_use]
    pub const fn blink(self) -> Self {
        self.attr(Attribute::Blink)
    }

    /// Adds the reverse attribute.
    #[must_use]
    pub const fn reverse(self) -> Self {
        self.attr(Attribute::Reverse)
    }

    /// Adds the hidden attribute.
    #[must_use]
    pub const fn hidden(self) -> Self {
        self.attr(Attribute::Hidden)
    }

    /// Adds the strikethrough attribute.
    #[must_use]
    pub const fn strikethrough(self) -> Self {
        self.attr(Attribute::Strikethrough)
    }

    /// Black foreground.
    #[must_use]
    pub const fn black(self) -> Self {
        self.fg(Color::BLACK)
    }

    /// Red foreground.
    #[must_use]
    pub const fn red(self) -> Self {
        self.fg(Color::RED)
    }

    /// Green foreground.
    #[must_use]
    pub const fn green(self) -> Self {
        self.fg(Color::GREEN)
    }

    /// Yellow foreground.
    #[must_use]
    pub const fn yellow(self) -> Self {
        self.fg(Color::YELLOW)
    }

    /// Blue foreground.
    #[must_use]
    pub const fn blue(self) -> Self {
        self.fg(Color::BLUE)
    }

    /// Magenta foreground.
    #[must_use]
    pub const fn magenta(self) -> Self {
        self.fg(Color::MAGENTA)
    }

    /// Cyan foreground.
    #[must_use]
    pub const fn cyan(self) -> Self {
        self.fg(Color::CYAN)
    }

    /// White foreground.
    #[must_use]
    pub const fn white(self) -> Self {
        self.fg(Color::WHITE)
    }

    /// Bright black foreground.
    #[must_use]
    pub const fn bright_black(self) -> Self {
        self.fg(Color::BRIGHT_BLACK)
    }

    /// Bright red foreground.
    #[must_use]
    pub const fn bright_red(self) -> Self {
        self.fg(Color::BRIGHT_RED)
    }

    /// Bright green foreground.
    #[must_use]
    pub const fn bright_green(self) -> Self {
        self.fg(Color::BRIGHT_GREEN)
    }

    /// Bright yellow foreground.
    #[must_use]
    pub const fn bright_yellow(self) -> Self {
        self.fg(Color::BRIGHT_YELLOW)
    }

    /// Bright blue foreground.
    #[must_use]
    pub const fn bright_blue(self) -> Self {
        self.fg(Color::BRIGHT_BLUE)
    }

    /// Bright magenta foreground.
    #[must_use]
    pub const fn bright_magenta(self) -> Self {
        self.fg(Color::BRIGHT_MAGENTA)
    }

    /// Bright cyan foreground.
    #[must_use]
    pub const fn bright_cyan(self) -> Self {
        self.fg(Color::BRIGHT_CYAN)
    }

    /// Bright white foreground.
    #[must_use]
    pub const fn bright_white(self) -> Self {
        self.fg(Color::BRIGHT_WHITE)
    }

    /// Black background.
    #[must_use]
    pub const fn on_black(self) -> Self {
        self.bg(Color::BLACK)
    }

    /// Red background.
    #[must_use]
    pub const fn on_red(self) -> Self {
        self.bg(Color::RED)
    }

    /// Green background.
    #[must_use]
    pub const fn on_green(self) -> Self {
        self.bg(Color::GREEN)
    }

    /// Yellow background.
    #[must_use]
    pub const fn on_yellow(self) -> Self {
        self.bg(Color::YELLOW)
    }

    /// Blue background.
    #[must_use]
    pub const fn on_blue(self) -> Self {
        self.bg(Color::BLUE)
    }

    /// Magenta background.
    #[must_use]
    pub const fn on_magenta(self) -> Self {
        self.bg(Color::MAGENTA)
    }

    /// Cyan background.
    #[must_use]
    pub const fn on_cyan(self) -> Self {
        self.bg(Color::CYAN)
    }

    /// White background.
    #[must_use]
    pub const fn on_white(self) -> Self {
        self.bg(Color::WHITE)
    }

    /// Bright black background.
    #[must_use]
    pub const fn on_bright_black(self) -> Self {
        self.bg(Color::BRIGHT_BLACK)
    }

    /// Bright red background.
    #[must_use]
    pub const fn on_bright_red(self) -> Self {
        self.bg(Color::BRIGHT_RED)
    }

    /// Bright green background.
    #[must_use]
    pub const fn on_bright_green(self) -> Self {
        self.bg(Color::BRIGHT_GREEN)
    }

    /// Bright yellow background.
    #[must_use]
    pub const fn on_bright_yellow(self) -> Self {
        self.bg(Color::BRIGHT_YELLOW)
    }

    /// Bright blue background.
    #[must_use]
    pub const fn on_bright_blue(self) -> Self {
        self.bg(Color::BRIGHT_BLUE)
    }

    /// Bright magenta background.
    #[must_use]
    pub const fn on_bright_magenta(self) -> Self {
        self.bg(Color::BRIGHT_MAGENTA)
    }

    /// Bright cyan background.
    #[must_use]
    pub const fn on_bright_cyan(self) -> Self {
        self.bg(Color::BRIGHT_CYAN)
    }

    /// Bright white background.
    #[must_use]
    pub const fn on_bright_white(self) -> Self {
        self.bg(Color::BRIGHT_WHITE)
    }

    /// Applies modifications only when `condition` holds.
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::{Color, Style};
    ///
    /// let verbose = true;
    /// let style = Style::new().when(verbose, |s| s.fg(Color::CYAN).dim());
    /// assert_eq!(style.foreground(), Some(Color::CYAN));
    /// ```
    #[must_use]
    pub fn when(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if condition { f(self) } else { self }
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

    /// Returns the attribute set.
    #[must_use]
    pub const fn attributes(&self) -> Effects {
        self.effects
    }

    /// Returns `true` when the style has no colors and no attributes.
    #[must_use]
    pub const fn is_plain(&self) -> bool {
        self.foreground.is_none() && self.background.is_none() && self.effects.is_empty()
    }

    /// Combines two styles; colors set by `other` win and attribute sets are
    /// unioned (no duplicates can appear).
    #[must_use]
    pub const fn merge(self, other: Self) -> Self {
        Self {
            foreground: match other.foreground {
                Some(color) => Some(color),
                None => self.foreground,
            },
            background: match other.background {
                Some(color) => Some(color),
                None => self.background,
            },
            effects: self.effects.union(other.effects),
        }
    }

    /// Reduces both colors to the richest representation `level` supports.
    #[must_use]
    pub fn degraded(&self, level: crate::choice::ColorLevel) -> Self {
        Self {
            foreground: self.foreground.map(|c| c.degrade_to(level)),
            background: self.background.map(|c| c.degrade_to(level)),
            effects: self.effects,
        }
    }

    /// Paints `text` with this style, producing a [`Span`](crate::Span).
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "no-color"))]
    /// # {
    /// use pretty_console::{Color, Style};
    ///
    /// let style = Style::new().fg(Color::RED).bold();
    /// assert_eq!(style.paint("boom").to_string(), "\x1b[1;38;5;1mboom\x1b[0m");
    /// # }
    /// ```
    #[must_use]
    pub fn paint(self, text: impl Into<String>) -> crate::Span {
        crate::Span::new_with_style(text, self)
    }

    /// Writes the SGR start sequence, or nothing for a plain style.
    // Under the `no-color` feature the body is a no-op, so `self` is unused
    // and the `fmt::Result` is trivially `Ok`.
    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    pub(crate) fn write_start(&self, f: &mut impl fmt::Write) -> fmt::Result {
        #[cfg(feature = "no-color")]
        {
            let _ = f;
            Ok(())
        }
        #[cfg(not(feature = "no-color"))]
        {
            if self.is_plain() {
                return Ok(());
            }
            f.write_str("\x1b[")?;
            let mut first = true;
            for attr in self.effects.attributes() {
                crate::color::write_sep(f, &mut first)?;
                f.write_str(attr.code())?;
            }
            if let Some(fg) = self.foreground {
                crate::color::write_sep(f, &mut first)?;
                fg.write_fg(f)?;
            }
            if let Some(bg) = self.background {
                crate::color::write_sep(f, &mut first)?;
                bg.write_bg(f)?;
            }
            f.write_str("m")
        }
    }

    /// Writes the SGR reset sequence; a no-op with the `no-color` feature.
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn write_reset(f: &mut impl fmt::Write) -> fmt::Result {
        #[cfg(feature = "no-color")]
        {
            let _ = f;
            Ok(())
        }
        #[cfg(not(feature = "no-color"))]
        f.write_str(RESET)
    }

    /// Renders the start sequence into a `String`; plain styles render empty.
    #[must_use]
    pub fn to_ansi_start(&self) -> String {
        let mut out = String::new();
        let _ = self.write_start(&mut out);
        out
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::new().fg(color)
    }
}

impl From<Effects> for Style {
    fn from(effects: Effects) -> Self {
        Self::new().effects(effects)
    }
}

impl From<Attribute> for Style {
    fn from(attribute: Attribute) -> Self {
        Self::new().attr(attribute)
    }
}
