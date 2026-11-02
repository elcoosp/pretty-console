//! Log-level helpers, banners, multi-segment lines and panels: the building
//! blocks of a CLI's output.

use pretty_console::{Align, Color, Line, Panel, Span, Style, pad};

fn main() {
    // Log levels with tasteful defaults.
    Span::info("Compiling 42 modules").println();
    Span::success("Tests passed").println();
    Span::warn("Deprecated API").println();
    Span::error("Disk full").println();
    Span::debug("cache_size = 42").println();
    println!();

    // Banners for section headers.
    Span::banner("Build report", 60).println();
    println!();

    // Multi-segment lines: differently-styled parts of one row.
    Line::new()
        .push("  ✓ build ")
        .dim()
        .push("cargo build --release")
        .bold()
        .push(" finished in ")
        .push("4.2s")
        .green()
        .println();
    Line::new()
        .push("  ✗ test ")
        .dim()
        .push("parse_config")
        .yellow()
        .push(" failed")
        .red()
        .eprintln();
    println!();

    // Width-aware alignment of styled rows (CJK counts as 2 columns).
    let rows = [
        ("ok", "unit tests", "1.2s"),
        ("warn", "clippy", "3.4s"),
        ("ok", "ドキュメント", "0.8s"),
    ];
    for (status, name, time) in rows {
        let status_span = if status == "ok" {
            Span::new(" OK ").green().bold()
        } else {
            Span::new("WARN").yellow().bold()
        };
        let line = Line::new()
            .push("  ")
            .push(status_span)
            // `pad` is visible-width aware: CJK names stay aligned.
            .push(pad(name, 14, Align::Left, ' '))
            .push(time)
            .bright_black();
        println!("{line}");
    }
    println!();

    // Panels: boxed sections with an inlaid title.
    Panel::new()
        .title("Summary")
        .border(Style::new().bright_black())
        .line(
            Line::new()
                .push("passed: ")
                .dim()
                .push("128")
                .green()
                .bold(),
        )
        .line(Line::new().push("failed: ").dim().push("0").red().bold())
        .line(Line::new().push("total:  ").dim().push("128").bold())
        .println();
    println!();

    // Decorations: gradients and rainbows (truecolor terminals).
    pretty_console::gradient(
        "pretty console gradient",
        Color::hex(0x00FF_3E6C),
        Color::hex(0x004D_C4FF),
    )
    .println();
    pretty_console::rainbow("rainbow across the whole spectrum!").println();
    pretty_console::rainbow_bg(" BACKGROUND RAINBOW ").println();
}
