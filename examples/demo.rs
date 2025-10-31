use pretty_console::Console;

fn main() {
    // Basic colors
    Console::new("Hello, world!").red().bold().println();
    Console::new("This is a test")
        .blue()
        .on_yellow()
        .underline()
        .println();

    // RGB colors
    Console::new("RGB color!")
        .fg_rgb(255, 0, 0)
        .bg_rgb(0, 255, 0)
        .println();

    // Bright colors
    Console::new("Bright colors!")
        .bright_red()
        .on_bright_black()
        .println();

    // Reusing styles
    let error_style = Console::new("").red().bold();
    error_style
        .with_text("Error 1: Something went wrong")
        .println();
    error_style.with_text("Error 2: Another issue").println();

    // Multiple attributes
    Console::new("Multiple styles!")
        .magenta()
        .on_cyan()
        .bold()
        .italic()
        .strikethrough()
        .println();

    // Using the Display trait
    println!("Formatted: {}", Console::new("Styled text").green().bold());
}
