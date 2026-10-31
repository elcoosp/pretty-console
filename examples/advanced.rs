//! Advanced usage: predefined styles, merging, hex colors and the `Styled` trait.

use pretty_console::{Attribute, Color, Console, Style, Styled};

fn main() {
    // Creating a predefined style
    let heading_style = Style::new().fg(Color::BRIGHT_BLUE).bold().underline();

    // Using the style with different text
    Console::new_with_style("Chapter 1: Introduction", heading_style.clone()).println();
    Console::new_with_style("Chapter 2: Getting Started", heading_style).println();

    // Complex styling
    Console::new("Warning!")
        .yellow()
        .blink()
        .attr(Attribute::Reverse)
        .println();

    // Building step by step
    let mut console = Console::new("Step by step");
    console = console.red();
    console = console.on_white();
    console = console.bold();
    console.println();

    // Merging styles
    let base = Style::new().fg(Color::RED);
    let emphasis = Style::new().bold().underline();
    Console::new_with_style("Merged style", base.merge(emphasis)).println();

    // Hex and indexed colors
    Console::new("True color from hex")
        .fg(Color::hex(0x7C_4D_FF))
        .println();
    Console::new("Indexed palette color")
        .fg(Color::indexed(202))
        .println();

    // The Styled extension trait on literals
    println!("{}", "painted directly".green().on_black());

    // Using in format strings
    let name = "Alice";
    let message = format!(
        "Hello {}, this is {}",
        name,
        Console::new("important").red().bold()
    );
    println!("{message}");
}
