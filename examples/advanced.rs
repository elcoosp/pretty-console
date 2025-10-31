use pretty_console::{Attribute, Console, Style};

fn main() {
    // Creating a predefined style
    let heading_style = Style::new()
        .fg(pretty_console::Color::BRIGHT_BLUE)
        .bold()
        .underline();

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

    // Using in format strings
    let name = "Alice";
    let message = format!(
        "Hello {}, this is {}",
        name,
        Console::new("important").red().bold()
    );
    println!("{}", message);
}
