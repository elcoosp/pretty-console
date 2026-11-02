//! Showcase of everything v2 adds: composition, gradients, panels,
//! hyperlinks, adaptive colors and the ANSI-aware text toolkit.

use pretty_console::{
    Color, ColorLevel, Line, Panel, Span, gradient, gradient_bg, is_color_enabled, pad, paint,
    rainbow, rainbow_bg, style, supports_color_level, truncate, visible_width,
};

fn main() {
    let colors_on = is_color_enabled();
    println!("Color capability detected: {:?}", supports_color_level());
    println!("Colors enabled for printing: {colors_on}");
    println!();

    composition_section();
    macros_section();
    hyperlinks_section();
    text_toolkit_section();
    adaptive_section();
    decorations_section();
    panel_section(colors_on);
}

fn composition_section() {
    // --- Composition ---------------------------------------------------------
    Span::banner("Composition", 60).println();
    Line::new()
        .push("  status: ")
        .dim()
        .push("2.1.0 ")
        .bold()
        .push("(latest)")
        .green()
        .println();
    Line::new()
        .push("  specs:  ")
        .dim()
        .push("42")
        .cyan()
        .push(" passed, ")
        .push("0")
        .red()
        .push(" failed")
        .println();
    println!();
}

fn macros_section() {
    // --- Macros ---------------------------------------------------------------
    Span::banner("Macros", 60).println();
    paint!(bold magenta, "{} crates compiled in {}", 12, "3.1s").println();
    paint!(yellow, "warning: unused import in main.rs").println();
    style!(dim italic).paint("press q to quit").println();
    println!();
}

fn hyperlinks_section() {
    // --- Hyperlinks -------------------------------------------------------------
    Span::banner("Hyperlinks", 60).println();
    Span::new("→ documentation")
        .blue()
        .underline()
        .link("https://docs.rs/pretty-console")
        .println();
    Span::new("→ repository")
        .blue()
        .underline()
        .link("https://github.com/elcoosp/pretty-console")
        .println();
    println!();
}

fn text_toolkit_section() {
    // --- Text toolkit -----------------------------------------------------------
    Span::banner("ANSI-aware text tools", 60).println();
    let header = pad("package", 14, pretty_console::Align::Left, ' ')
        + &pad("version", 10, pretty_console::Align::Right, ' ')
        + "  status";
    println!("{}", style!(bold).paint(header));
    let rows = [
        ("serde", "1.0.219", "up to date"),
        ("tokio", "1.45.1", "outdated"),
        ("quote", "1.0.40", "up to date"),
    ];
    for (name, version, status) in rows {
        let status_span = if status == "up to date" {
            Span::new(status).green()
        } else {
            Span::new(status).yellow()
        };
        let row = Line::new()
            .push(pad(name, 14, pretty_console::Align::Left, ' '))
            .push(pad(version, 10, pretty_console::Align::Right, ' '))
            .push("  ")
            .push(status_span);
        println!("{row}");
    }
    let long = "the quick brown fox jumps over the lazy dog";
    println!("truncate(20): {}", truncate(long, 20, "…"));
    println!("visible_width of 日本語: {}", visible_width("日本語"));
    println!();
}

fn adaptive_section() {
    // --- Adaptive colors ----------------------------------------------------------
    Span::banner("Adaptive colors", 60).println();
    let fancy = Span::new("truecolor text")
        .fg(Color::hex(0x7C_4D_FF))
        .bold();
    println!("  truecolor : {fancy}");
    for level in [ColorLevel::Ansi256, ColorLevel::Ansi16] {
        println!("  {level:?}: {}", fancy.render_with(level));
    }
    println!();
}

fn decorations_section() {
    // --- Decorations ----------------------------------------------------------------
    Span::banner("Decorations", 60).println();
    gradient(
        "  sunrise fade from coral to sky blue",
        Color::hex(0x00FF_3E6C),
        Color::hex(0x004D_C4FF),
    )
    .println();
    gradient_bg(
        "  background gradient ",
        Color::hex(0x001E_1B4B),
        Color::hex(0x0031_2E81),
    )
    .println();
    rainbow("  the full spectrum, one hue per character").println();
    rainbow_bg("  RAINBOW BACKGROUND ").println();
    println!();
}

fn panel_section(colors_on: bool) {
    // --- Panels ------------------------------------------------------------------------
    let panel = Panel::new()
        .title("System report")
        .border(style!(bright_black))
        .line(Line::new().push("os       ").dim().push("Linux 6.9"))
        .line(Line::new().push("shell    ").dim().push("fish 4.0"))
        .line(Line::new().push("term     ").dim().push("kitty"))
        .line(
            Line::new()
                .push("colors   ")
                .dim()
                .push("truecolor")
                .green(),
        );
    panel.print();
    println!();

    // Everything degrades gracefully when colors are off (NO_COLOR, piped...).
    if !colors_on {
        println!("(colors are currently disabled — plain output above)");
    }
}
