//! [`Panel`]: bordered boxes with an inlaid title for CLI sections.

use std::fmt;
use std::io;

use crate::line::Line;
use crate::span::{Span, print_honoring_choice, println_honoring_choice};
use crate::style::Style;
use crate::text::{Align, pad, strip_ansi, truncate, visible_width};

/// A rounded, bordered box containing styled lines and an optional inlaid
/// title.
///
/// The border and title divider dashes are styled with [`border`](Panel::border);
/// content spans keep their own styles. Inner width is derived from the widest
/// line unless overridden with [`width`](Panel::width) (an explicit width is a
/// floor for the title and truncates over-wide lines with an ellipsis).
///
/// # Examples
///
/// ```
/// use pretty_console::{Line, Panel, Style};
///
/// let panel = Panel::new()
///     .title("Build report")
///     .border(Style::new().dim())
///     .line(Line::new().push("status: ").push("ok").green().bold());
/// panel.println();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Panel {
    title: Option<String>,
    lines: Vec<Line>,
    width: Option<usize>,
    padding: usize,
    border: Style,
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel {
    /// Creates an empty panel with padding 1, a plain border and no title.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            title: None,
            lines: Vec::new(),
            width: None,
            padding: 1,
            border: Style::new(),
        }
    }

    /// Sets the title, displayed inlaid into the top border.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Appends a content line (a [`Line`], a [`Span`], or a plain `&str`).
    #[must_use]
    pub fn line(mut self, line: impl Into<Line>) -> Self {
        self.lines.push(line.into());
        self
    }

    /// Appends several content lines at once.
    #[must_use]
    pub fn lines(mut self, lines: impl IntoIterator<Item = impl Into<Line>>) -> Self {
        self.lines.extend(lines.into_iter().map(Into::into));
        self
    }

    /// Fixes the inner content width; by default it fits the widest line and
    /// the title.
    #[must_use]
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the horizontal padding around content (default 1).
    #[must_use]
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Styles the border characters (and the title divider dashes).
    #[must_use]
    pub fn border(mut self, style: Style) -> Self {
        self.border = style;
        self
    }

    /// The computed inner width of the panel (between the vertical bars).
    ///
    /// This is `max(widest line) + 2 * padding` (or the explicit width plus
    /// padding), raised to fit the inlaid title when there is one.
    #[must_use]
    pub fn content_width(&self) -> usize {
        let title_min = self
            .title
            .as_ref()
            .map_or(0, |t| visible_width(t) + 2 * self.padding + 1);
        let content = self
            .lines
            .iter()
            .map(Line::visible_width)
            .max()
            .unwrap_or(0)
            + self.padding * 2;
        match self.width {
            Some(explicit) => (explicit + self.padding * 2).max(title_min),
            None => content.max(title_min),
        }
    }

    /// Renders the panel (deterministic; never consults the environment).
    #[must_use]
    pub fn render(&self) -> String {
        let inner = self.content_width();
        let body_width = inner.saturating_sub(self.padding * 2);
        let fixed = self.width.is_some();

        let mut out = String::new();
        // Top border with optional inlaid title: "─ Title " + dashes.
        out.push_str(&self.border.paint("╭").to_string());
        match &self.title {
            Some(title) => {
                let after = inner.saturating_sub(visible_width(title) + 3);
                out.push_str(&self.border.paint("─ ").to_string());
                out.push_str(&Span::new(title.clone()).to_string());
                out.push_str(&self.border.paint(" ").to_string());
                out.push_str(&self.border.paint("─".repeat(after)).to_string());
            }
            None => {
                out.push_str(&self.border.paint("─".repeat(inner)).to_string());
            }
        }
        out.push_str(&self.border.paint("╮").to_string());
        out.push('\n');

        // Content lines.
        let side = self.border.paint("│").to_string();
        let padding = " ".repeat(self.padding);
        for line in &self.lines {
            let rendered = line.render();
            let rendered = if fixed {
                truncate(&rendered, body_width, "…")
            } else {
                rendered
            };
            let body = pad(&rendered, body_width, Align::Left, ' ');
            out.push_str(&side);
            out.push_str(&padding);
            out.push_str(&body);
            out.push_str(&padding);
            out.push_str(&side);
            out.push('\n');
        }

        // Bottom border.
        out.push_str(&self.border.paint("╰").to_string());
        out.push_str(&self.border.paint("─".repeat(inner)).to_string());
        out.push_str(&self.border.paint("╯").to_string());
        out
    }

    /// The panel rendered without any ANSI escapes, useful for logs and tests.
    #[must_use]
    pub fn render_plain(&self) -> String {
        strip_ansi(&self.render())
    }

    /// Writes the rendered panel, honoring the global
    /// [`ColorChoice`](crate::ColorChoice).
    ///
    /// # Errors
    ///
    /// Returns any error reported by the underlying writer.
    pub fn write_to(&self, writer: &mut impl io::Write) -> io::Result<()> {
        let rendered = self.render();
        let plain = strip_ansi(&rendered);
        crate::span::write_honoring_choice(&rendered, &plain, writer)
    }

    /// Prints the panel to stdout.
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails.
    pub fn print(&self) {
        let rendered = self.render();
        let plain = strip_ansi(&rendered);
        print_honoring_choice(&rendered, &plain, false);
    }

    /// Prints the panel to stdout followed by a newline.
    ///
    /// # Panics
    ///
    /// Panics if writing to stdout fails.
    pub fn println(&self) {
        let rendered = self.render();
        let plain = strip_ansi(&rendered);
        println_honoring_choice(&format!("{rendered}\n"), &format!("{plain}\n"), false);
    }
}

impl fmt::Display for Panel {
    /// Renders the panel; a requested width pads the whole box.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.render())
    }
}
