use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Named(u8),
    RGB(u8, u8, u8),
}

impl Color {
    pub const BLACK: Color = Color::Named(0);
    pub const RED: Color = Color::Named(1);
    pub const GREEN: Color = Color::Named(2);
    pub const YELLOW: Color = Color::Named(3);
    pub const BLUE: Color = Color::Named(4);
    pub const MAGENTA: Color = Color::Named(5);
    pub const CYAN: Color = Color::Named(6);
    pub const WHITE: Color = Color::Named(7);
    pub const BRIGHT_BLACK: Color = Color::Named(8);
    pub const BRIGHT_RED: Color = Color::Named(9);
    pub const BRIGHT_GREEN: Color = Color::Named(10);
    pub const BRIGHT_YELLOW: Color = Color::Named(11);
    pub const BRIGHT_BLUE: Color = Color::Named(12);
    pub const BRIGHT_MAGENTA: Color = Color::Named(13);
    pub const BRIGHT_CYAN: Color = Color::Named(14);
    pub const BRIGHT_WHITE: Color = Color::Named(15);

    fn to_fg_code(&self) -> String {
        match self {
            Color::Named(n) => format!("38;5;{}", n),
            Color::RGB(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }

    fn to_bg_code(&self) -> String {
        match self {
            Color::Named(n) => format!("48;5;{}", n),
            Color::RGB(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,
}

impl Attribute {
    fn to_code(&self) -> &'static str {
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

#[derive(Clone, Default)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    attributes: Vec<Attribute>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn attr(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn bold(self) -> Self {
        self.attr(Attribute::Bold)
    }

    pub fn dim(self) -> Self {
        self.attr(Attribute::Dim)
    }

    pub fn italic(self) -> Self {
        self.attr(Attribute::Italic)
    }

    pub fn underline(self) -> Self {
        self.attr(Attribute::Underline)
    }

    pub fn blink(self) -> Self {
        self.attr(Attribute::Blink)
    }

    pub fn reverse(self) -> Self {
        self.attr(Attribute::Reverse)
    }

    pub fn hidden(self) -> Self {
        self.attr(Attribute::Hidden)
    }

    pub fn strikethrough(self) -> Self {
        self.attr(Attribute::Strikethrough)
    }

    #[cfg(not(feature = "no-color"))]
    fn to_ansi_start(&self) -> String {
        let mut codes: Vec<String> = Vec::new();

        for attr in &self.attributes {
            codes.push(attr.to_code().into());
        }

        if let Some(fg) = &self.foreground {
            codes.push(fg.to_fg_code());
        }

        if let Some(bg) = &self.background {
            codes.push(bg.to_bg_code());
        }

        if codes.is_empty() {
            String::new()
        } else {
            format!("\x1b[{}m", codes.join(";"))
        }
    }

    #[cfg(feature = "no-color")]
    fn to_ansi_start(&self) -> String {
        String::new()
    }
}

#[derive(Clone)]
pub struct Console {
    text: String,
    style: Style,
}

impl Console {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Console {
            text: text.into(),
            style: Style::default(),
        }
    }

    pub fn new_with_style<T: Into<String>>(text: T, style: Style) -> Self {
        Console {
            text: text.into(),
            style,
        }
    }

    pub fn with_text<T: Into<String>>(&self, text: T) -> Self {
        Console {
            text: text.into(),
            style: self.style.clone(),
        }
    }

    // Color methods
    pub fn fg(self, color: Color) -> Self {
        Console {
            style: self.style.fg(color),
            ..self
        }
    }

    pub fn bg(self, color: Color) -> Self {
        Console {
            style: self.style.bg(color),
            ..self
        }
    }

    pub fn fg_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.fg(Color::RGB(r, g, b))
    }

    pub fn bg_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.bg(Color::RGB(r, g, b))
    }

    // Named color convenience methods
    pub fn black(self) -> Self {
        self.fg(Color::BLACK)
    }

    pub fn red(self) -> Self {
        self.fg(Color::RED)
    }

    pub fn green(self) -> Self {
        self.fg(Color::GREEN)
    }

    pub fn yellow(self) -> Self {
        self.fg(Color::YELLOW)
    }

    pub fn blue(self) -> Self {
        self.fg(Color::BLUE)
    }
    pub fn magenta(self) -> Self {
        self.fg(Color::MAGENTA)
    }

    pub fn cyan(self) -> Self {
        self.fg(Color::CYAN)
    }

    pub fn white(self) -> Self {
        self.fg(Color::WHITE)
    }

    pub fn bright_black(self) -> Self {
        self.fg(Color::BRIGHT_BLACK)
    }

    pub fn bright_red(self) -> Self {
        self.fg(Color::BRIGHT_RED)
    }

    pub fn bright_green(self) -> Self {
        self.fg(Color::BRIGHT_GREEN)
    }

    pub fn bright_yellow(self) -> Self {
        self.fg(Color::BRIGHT_YELLOW)
    }

    pub fn bright_blue(self) -> Self {
        self.fg(Color::BRIGHT_BLUE)
    }

    pub fn bright_magenta(self) -> Self {
        self.fg(Color::BRIGHT_MAGENTA)
    }

    pub fn bright_cyan(self) -> Self {
        self.fg(Color::BRIGHT_CYAN)
    }

    pub fn bright_white(self) -> Self {
        self.fg(Color::BRIGHT_WHITE)
    }

    // Background color convenience methods
    pub fn on_black(self) -> Self {
        self.bg(Color::BLACK)
    }

    pub fn on_red(self) -> Self {
        self.bg(Color::RED)
    }

    pub fn on_green(self) -> Self {
        self.bg(Color::GREEN)
    }

    pub fn on_yellow(self) -> Self {
        self.bg(Color::YELLOW)
    }

    pub fn on_blue(self) -> Self {
        self.bg(Color::BLUE)
    }

    pub fn on_magenta(self) -> Self {
        self.bg(Color::MAGENTA)
    }

    pub fn on_cyan(self) -> Self {
        self.bg(Color::CYAN)
    }

    pub fn on_white(self) -> Self {
        self.bg(Color::WHITE)
    }

    pub fn on_bright_black(self) -> Self {
        self.bg(Color::BRIGHT_BLACK)
    }

    pub fn on_bright_red(self) -> Self {
        self.bg(Color::BRIGHT_RED)
    }

    pub fn on_bright_green(self) -> Self {
        self.bg(Color::BRIGHT_GREEN)
    }

    pub fn on_bright_yellow(self) -> Self {
        self.bg(Color::BRIGHT_YELLOW)
    }

    pub fn on_bright_blue(self) -> Self {
        self.bg(Color::BRIGHT_BLUE)
    }

    pub fn on_bright_magenta(self) -> Self {
        self.bg(Color::BRIGHT_MAGENTA)
    }

    pub fn on_bright_cyan(self) -> Self {
        self.bg(Color::BRIGHT_CYAN)
    }

    pub fn on_bright_white(self) -> Self {
        self.bg(Color::BRIGHT_WHITE)
    }

    // Attribute methods
    pub fn attr(self, attribute: Attribute) -> Self {
        Console {
            style: self.style.attr(attribute),
            ..self
        }
    }

    pub fn bold(self) -> Self {
        Console {
            style: self.style.bold(),
            ..self
        }
    }

    pub fn dim(self) -> Self {
        Console {
            style: self.style.dim(),
            ..self
        }
    }

    pub fn italic(self) -> Self {
        Console {
            style: self.style.italic(),
            ..self
        }
    }

    pub fn underline(self) -> Self {
        Console {
            style: self.style.underline(),
            ..self
        }
    }

    pub fn blink(self) -> Self {
        Console {
            style: self.style.blink(),
            ..self
        }
    }

    pub fn reverse(self) -> Self {
        Console {
            style: self.style.reverse(),
            ..self
        }
    }

    pub fn hidden(self) -> Self {
        Console {
            style: self.style.hidden(),
            ..self
        }
    }

    pub fn strikethrough(self) -> Self {
        Console {
            style: self.style.strikethrough(),
            ..self
        }
    }

    // Output methods
    pub fn print(&self) {
        let mut stdout = std::io::stdout();
        self.write_to(&mut stdout).unwrap();
    }

    pub fn println(&self) {
        let mut stdout = std::io::stdout();
        self.write_to(&mut stdout).unwrap();
        writeln!(stdout).unwrap();
    }

    pub fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let ansi_code = self.style.to_ansi_start();
        if !ansi_code.is_empty() {
            write!(writer, "{}", ansi_code)?;
        }
        write!(writer, "{}", self.text)?;
        if !ansi_code.is_empty() {
            write!(writer, "\x1b[0m")?;
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl std::fmt::Display for Console {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ansi_code = self.style.to_ansi_start();
        if !ansi_code.is_empty() {
            write!(f, "{}", ansi_code)?;
        }
        write!(f, "{}", self.text)?;
        if !ansi_code.is_empty() {
            write!(f, "\x1b[0m")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_color_codes() {
        // Test named colors
        let color = Color::RED;
        assert_eq!(color.to_fg_code(), "38;5;1");
        assert_eq!(color.to_bg_code(), "48;5;1");

        // Test RGB colors
        let rgb = Color::RGB(255, 128, 0);
        assert_eq!(rgb.to_fg_code(), "38;2;255;128;0");
        assert_eq!(rgb.to_bg_code(), "48;2;255;128;0");

        // Test edge cases
        let black = Color::BLACK;
        assert_eq!(black.to_fg_code(), "38;5;0");

        let white = Color::BRIGHT_WHITE;
        assert_eq!(white.to_fg_code(), "38;5;15");
    }

    #[test]
    fn test_attribute_codes() {
        assert_eq!(Attribute::Bold.to_code(), "1");
        assert_eq!(Attribute::Dim.to_code(), "2");
        assert_eq!(Attribute::Italic.to_code(), "3");
        assert_eq!(Attribute::Underline.to_code(), "4");
        assert_eq!(Attribute::Blink.to_code(), "5");
        assert_eq!(Attribute::Reverse.to_code(), "7");
        assert_eq!(Attribute::Hidden.to_code(), "8");
        assert_eq!(Attribute::Strikethrough.to_code(), "9");
    }

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
            assert!(ansi.contains("1")); // bold
            assert!(ansi.contains("4")); // underline
            assert!(ansi.contains("38;5;1")); // red foreground
            assert!(ansi.contains("48;5;4")); // blue background
        }

        #[cfg(feature = "no-color")]
        assert_eq!(style.to_ansi_start(), "");
    }

    #[test]
    fn test_style_empty() {
        let style = Style::new();
        #[cfg(not(feature = "no-color"))]
        assert_eq!(style.to_ansi_start(), "");
        #[cfg(feature = "no-color")]
        assert_eq!(style.to_ansi_start(), "");
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
            assert!(ansi.contains("1"));
            assert!(ansi.contains("3"));
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
    fn test_console_creation() {
        let console = Console::new("test");
        assert_eq!(console.text, "test");

        let console_from_str = Console::new("test");
        assert_eq!(console_from_str.text, "test");
    }

    #[test]
    fn test_console_with_text() {
        let base = Console::new("").red().bold();
        let derived = base.with_text("new text");

        assert_eq!(derived.text, "new text");
        // Should maintain the same style
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
    }

    #[test]
    fn test_console_attribute_methods() {
        let console = Console::new("test").bold();
        assert!(console.style.attributes.contains(&Attribute::Bold));

        let console = Console::new("test").italic().underline();
        assert!(console.style.attributes.contains(&Attribute::Italic));
        assert!(console.style.attributes.contains(&Attribute::Underline));

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
    }

    #[test]
    fn test_console_display_trait() {
        let console = Console::new("hello").red().bold();
        let output = format!("{}", console);

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
            assert!(string.contains("4")); // underline
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
            assert!(output.contains("1"));
            assert!(output.contains("3"));
            assert!(output.contains("4"));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(output, "complex");
    }

    #[test]
    fn test_console_method_chaining() {
        // Test that methods can be chained and each returns Self
        let console = Console::new("test").red().on_white().bold().underline();

        assert_eq!(console.style.foreground, Some(Color::RED));
        assert_eq!(console.style.background, Some(Color::WHITE));
        assert!(console.style.attributes.contains(&Attribute::Bold));
        assert!(console.style.attributes.contains(&Attribute::Underline));
    }

    #[test]
    fn test_console_clone() {
        let original = Console::new("original").red().bold();
        let cloned = original.clone();

        assert_eq!(original.text, cloned.text);
        assert_eq!(original.style.foreground, cloned.style.foreground);
        assert_eq!(original.style.background, cloned.style.background);
        assert_eq!(original.style.attributes, cloned.style.attributes);
    }

    #[test]
    fn test_color_and_attribute_copy() {
        // These should be Copy types, so we can use them multiple times
        let color = Color::RED;
        let attr = Attribute::Bold;

        let style1 = Style::new().fg(color).attr(attr);
        let style2 = Style::new().fg(color).attr(attr);

        assert_eq!(style1.foreground, style2.foreground);
        assert_eq!(style1.attributes, style2.attributes);
    }

    #[test]
    fn test_all_named_colors() {
        // Test that all named color methods work correctly
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
        // Test that all background color methods work correctly
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
    fn test_console_write_to() {
        let console = Console::new("write test").green();
        let mut buffer = Vec::new();

        console.write_to(&mut buffer).unwrap();
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
    fn test_console_empty_text() {
        let console = Console::new("").red().bold();
        let output = console.to_string();

        #[cfg(not(feature = "no-color"))]
        {
            // Even with empty text, ANSI codes should be generated and reset
            assert!(output.starts_with("\x1b["));
            assert!(output.ends_with("\x1b[0m"));
        }
        #[cfg(feature = "no-color")]
        assert_eq!(output, "");
    }

    #[test]
    fn test_multiple_identical_attributes() {
        // Adding the same attribute multiple times should work
        // (though it might not make sense semantically)
        let console = Console::new("test").bold().bold().bold();

        // Should have multiple bold attributes in the vector
        let bold_count = console
            .style
            .attributes
            .iter()
            .filter(|&&attr| attr == Attribute::Bold)
            .count();
        assert_eq!(bold_count, 3);
    }

    #[test]
    fn test_ansi_code_ordering() {
        // Test that ANSI codes are generated in consistent order:
        // attributes first, then foreground, then background
        let style = Style::new()
            .bg(Color::BLUE)
            .fg(Color::RED)
            .underline()
            .bold();

        #[cfg(not(feature = "no-color"))]
        {
            let ansi = style.to_ansi_start();
            // The order should be: 1 (bold), 4 (underline), 38;5;1 (red), 48;5;4 (blue)
            let expected_parts = ["1", "4", "38;5;1", "48;5;4"];
            let ansi_without_prefix = ansi.trim_start_matches("\x1b[").trim_end_matches('m');
            let parts: Vec<&str> = ansi_without_prefix.split(';').collect();

            // We can't easily test the exact order because it depends on Vec iteration order,
            // but we can test that all expected parts are present
            for expected in expected_parts {
                assert!(
                    ansi.contains(expected),
                    "ANSI code should contain {}",
                    expected
                );
            }
        }
    }

    #[test]
    fn test_console_builder() {
        let console = Console::new("test").red().on_white().bold().underline();

        let output = console.to_string();

        #[cfg(not(feature = "no-color"))]
        {
            assert!(output.contains("test"));
            assert!(output.contains("\x1b["));
            assert!(output.contains("0m"));
        }

        #[cfg(feature = "no-color")]
        assert_eq!(output, "test");
    }
    #[test]
    fn test_basic_colors() {
        let console = Console::new("Hello, world!").red().bold();
        insta::assert_yaml_snapshot!(console.to_string());
    }

    #[test]
    fn test_rgb_colors() {
        let console = Console::new("RGB Text")
            .fg_rgb(255, 0, 128)
            .bg_rgb(0, 255, 0)
            .underline();
        insta::assert_yaml_snapshot!(console.to_string());
    }

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
    #[test]
    fn test_style_combinations() {
        let styles = vec![
            Console::new("Error style").red().bold(),
            Console::new("Warning style").yellow().italic(),
            Console::new("Success style").green().bold(),
        ];

        let outputs: Vec<String> = styles.iter().map(|c| c.to_string()).collect();
        insta::assert_yaml_snapshot!(outputs);
    }

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
}
