//! Advanced usage: predefined styles, merging, hex colors, hyperlinks and
//! conditional styling.

use pretty_console::{Attribute, Color, Span, Style, paint, style};

fn main() {
    // Creating a predefined style
    let heading_style = Style::new().fg(Color::BRIGHT_BLUE).bold().underline();

    // Using the style with different text
    heading_style.paint("Chapter 1: Introduction").println();
    heading_style.paint("Chapter 2: Getting Started").println();

    // Complex styling
    Span::new("Warning!")
        .yellow()
        .blink()
        .attr(Attribute::Reverse)
        .println();

    // Building step by step
    let mut span = Span::new("Step by step");
    span.set_style(span.style().red().on_white().bold());
    span.println();

    // Merging styles: colors of the second win, attribute sets union.
    let base = Style::new().fg(Color::RED);
    let emphasis = Style::new().bold().underline();
    Span::new_with_style("Merged style", base.merge(emphasis)).println();

    // Hex colors: packed values...
    Span::new("Violet").fg(Color::hex(0x7C_4D_FF)).println();
    // ...and parsed from strings (handy for config files).
    let theme_color = "#22d3ee";
    match Color::from_hex_str(theme_color) {
        Ok(color) => Span::new("Theme accent").fg(color).println(),
        Err(err) => eprintln!("bad color: {err}"),
    }

    // Hyperlinks (OSC 8): clickable in supporting terminals.
    Span::new("pretty-console on GitHub")
        .blue()
        .underline()
        .link("https://github.com/elcoosp/pretty-console")
        .println();

    // Conditional styling keeps chains flat.
    let verbose = std::env::var("VERBOSE").is_ok();
    Span::new("details")
        .when(verbose, |s| s.dim().italic())
        .println();

    // The macros: one-shot styling without hand-written format strings.
    paint!(green bold, "{} files synced", 42).println();
    style!(dim).paint("(end of demo)").println();
}
