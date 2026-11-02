//! A tour of the basics: colors, backgrounds, attributes and reusable styles.

use pretty_console::{Span, Styled};

fn main() {
    // Basic colors
    Span::new("Hello, world!").red().bold().println();
    Span::new("This is a test")
        .blue()
        .on_yellow()
        .underline()
        .println();

    // RGB colors
    Span::new("RGB color!")
        .fg_rgb(255, 0, 0)
        .bg_rgb(0, 255, 0)
        .println();

    // Bright colors
    Span::new("Bright colors!")
        .bright_red()
        .on_bright_black()
        .println();

    // Reusing styles
    let error_style = Span::new("").red().bold();
    error_style
        .with_text("Error 1: Something went wrong")
        .println();
    error_style.with_text("Error 2: Another issue").println();

    // Multiple attributes
    Span::new("Multiple styles!")
        .magenta()
        .on_cyan()
        .bold()
        .italic()
        .strikethrough()
        .println();

    // Attributes are idempotent: bold().bold() emits a single code.
    Span::new("Idempotent bold").bold().bold().bold().println();

    // Styled literals through the extension trait
    println!("{}", "Painted in place".cyan().underline());
}
