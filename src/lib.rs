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
    fn test_color_codes() {
        let color = Color::RED;
        assert_eq!(color.to_fg_code(), "38;5;1");

        let rgb = Color::RGB(255, 128, 0);
        assert_eq!(rgb.to_fg_code(), "38;2;255;128;0");
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .fg(Color::RED)
            .bg(Color::BLUE)
            .bold()
            .underline();

        #[cfg(not(feature = "no-color"))]
        assert!(style.to_ansi_start().contains("1"));
        #[cfg(not(feature = "no-color"))]
        assert!(style.to_ansi_start().contains("4"));
        #[cfg(not(feature = "no-color"))]
        assert!(style.to_ansi_start().contains("38;5;1"));
        #[cfg(not(feature = "no-color"))]
        assert!(style.to_ansi_start().contains("48;5;4"));
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
}
