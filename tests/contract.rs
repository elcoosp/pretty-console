//! Rendering contract tests: exact ANSI output for every core operation.
//!
//! These replace v1's insta snapshots with deterministic exact-string checks.

#![cfg(not(feature = "no-color"))]

use pretty_console::{Attribute, Color, ColorLevel, Console, Effects, Line, Span, Style, Styled};

const RESET: &str = "\x1b[0m";

#[test]
fn basic_color_and_attribute() {
    assert_eq!(
        Span::new("Hello, world!").red().bold().to_string(),
        "\x1b[1;38;5;1mHello, world!\x1b[0m"
    );
    assert_eq!(
        "Success!".green().to_string(),
        format!("\x1b[38;5;2mSuccess!{RESET}")
    );
}

#[test]
fn style_ordering_is_canonical() {
    // Attributes first (bit order), then foreground, then background —
    // regardless of the order the builder methods were called in.
    let style = Style::new()
        .bg(Color::BLUE)
        .fg(Color::RED)
        .underline()
        .bold();
    assert_eq!(style.to_ansi_start(), "\x1b[1;4;38;5;1;48;5;4m");

    let span = Span::new("x").dim().cyan().on_yellow().strikethrough();
    assert_eq!(span.to_string(), "\x1b[2;9;38;5;6;48;5;3mx\x1b[0m");
}

#[test]
fn duplicate_attributes_are_idempotent() {
    let span = Span::new("x").bold().bold().bold();
    assert_eq!(span.to_string(), "\x1b[1mx\x1b[0m");
    assert_eq!(span.style().attributes(), Effects::BOLD);
}

#[test]
fn plain_style_renders_no_escapes() {
    assert_eq!(Span::new("plain").to_string(), "plain");
    assert_eq!(Span::default().to_string(), "");
    assert_eq!(Style::new().to_ansi_start(), "");
}

#[test]
fn rgb_and_hex_colors() {
    let span = Span::new("RGB Text")
        .fg_rgb(255, 0, 128)
        .bg_rgb(0, 255, 0)
        .underline();
    assert_eq!(
        span.to_string(),
        "\x1b[4;38;2;255;0;128;48;2;0;255;0mRGB Text\x1b[0m"
    );

    let violet = Span::new("V").fg_hex(0x7C_4D_FF);
    assert_eq!(violet.to_string(), "\x1b[38;2;124;77;255mV\x1b[0m");
}

#[test]
fn indexed_colors() {
    let span = Span::new("x")
        .fg(Color::indexed(200))
        .bg(Color::indexed(21));
    assert_eq!(span.to_string(), "\x1b[38;5;200;48;5;21mx\x1b[0m");
}

type Paint = fn(Span) -> Span;

#[test]
fn all_foreground_colors() {
    let cases: [(Paint, Color, &str); 16] = [
        (Span::black, Color::BLACK, "38;5;0"),
        (Span::red, Color::RED, "38;5;1"),
        (Span::green, Color::GREEN, "38;5;2"),
        (Span::yellow, Color::YELLOW, "38;5;3"),
        (Span::blue, Color::BLUE, "38;5;4"),
        (Span::magenta, Color::MAGENTA, "38;5;5"),
        (Span::cyan, Color::CYAN, "38;5;6"),
        (Span::white, Color::WHITE, "38;5;7"),
        (Span::bright_black, Color::BRIGHT_BLACK, "38;5;8"),
        (Span::bright_red, Color::BRIGHT_RED, "38;5;9"),
        (Span::bright_green, Color::BRIGHT_GREEN, "38;5;10"),
        (Span::bright_yellow, Color::BRIGHT_YELLOW, "38;5;11"),
        (Span::bright_blue, Color::BRIGHT_BLUE, "38;5;12"),
        (Span::bright_magenta, Color::BRIGHT_MAGENTA, "38;5;13"),
        (Span::bright_cyan, Color::BRIGHT_CYAN, "38;5;14"),
        (Span::bright_white, Color::BRIGHT_WHITE, "38;5;15"),
    ];
    for (paint, color, code) in cases {
        let span = paint(Span::new("x"));
        assert_eq!(span.style().foreground(), Some(color));
        assert!(span.to_string().contains(code));
    }
}

#[test]
fn all_background_colors() {
    let cases: [(Paint, &str); 16] = [
        (Span::on_black, "48;5;0"),
        (Span::on_red, "48;5;1"),
        (Span::on_green, "48;5;2"),
        (Span::on_yellow, "48;5;3"),
        (Span::on_blue, "48;5;4"),
        (Span::on_magenta, "48;5;5"),
        (Span::on_cyan, "48;5;6"),
        (Span::on_white, "48;5;7"),
        (Span::on_bright_black, "48;5;8"),
        (Span::on_bright_red, "48;5;9"),
        (Span::on_bright_green, "48;5;10"),
        (Span::on_bright_yellow, "48;5;11"),
        (Span::on_bright_blue, "48;5;12"),
        (Span::on_bright_magenta, "48;5;13"),
        (Span::on_bright_cyan, "48;5;14"),
        (Span::on_bright_white, "48;5;15"),
    ];
    for (paint, code) in cases {
        assert!(paint(Span::new("x")).to_string().contains(code));
    }
}

#[test]
fn all_attributes() {
    let span = Span::new("x")
        .bold()
        .dim()
        .italic()
        .underline()
        .blink()
        .reverse()
        .hidden()
        .strikethrough();
    assert_eq!(span.to_string(), "\x1b[1;2;3;4;5;7;8;9mx\x1b[0m");
    assert_eq!(span.style().attributes(), Effects::ALL);
}

#[test]
fn reusable_styles() {
    let error_style = Span::new("").red().bold();
    assert_eq!(
        error_style.with_text("Error: File not found").to_string(),
        "\x1b[1;38;5;1mError: File not found\x1b[0m"
    );
    assert_eq!(
        error_style
            .with_text("Error: Permission denied")
            .to_string(),
        "\x1b[1;38;5;1mError: Permission denied\x1b[0m"
    );
    // The template itself is unchanged.
    assert_eq!(error_style.text(), "");
}

#[test]
fn style_paint_and_merge() {
    let heading = Style::new().bold().underline().fg(Color::BRIGHT_BLUE);
    assert_eq!(
        heading.paint("Chapter 1").to_string(),
        "\x1b[1;4;38;5;12mChapter 1\x1b[0m"
    );

    let emphasis = Style::new().fg(Color::RED);
    let merged = heading.merge(emphasis);
    // Other's foreground wins; attributes union.
    assert_eq!(merged.foreground(), Some(Color::RED));
    assert_eq!(merged.attributes(), Effects::BOLD.union(Effects::UNDERLINE));
    assert_eq!(
        merged.paint("Important").to_string(),
        "\x1b[1;4;38;5;1mImportant\x1b[0m"
    );
}

#[test]
fn display_width_uses_visible_width() {
    let span = Span::new("hi").red();
    let styled = span.to_string();
    assert_eq!(format!("{span:10}"), format!("{styled}        "));
    assert_eq!(format!("{span:>10}"), format!("        {styled}"));
    assert_eq!(format!("{span:^11}"), format!("    {styled}     "));
    assert_eq!(format!("{span:*^11}"), format!("****{styled}*****"));
    assert_eq!(format!("{span:2}"), styled);

    // Wide characters count as two columns.
    let cjk = Span::new("日本").cyan();
    assert_eq!(format!("{cjk:>6}"), format!("  {}", cjk.to_string()));

    let plain = Span::new("hi");
    assert_eq!(format!("{plain:>6}"), "    hi");
}

#[test]
fn empty_text_with_style() {
    let span = Span::new("").red().bold();
    assert_eq!(span.to_string(), "\x1b[1;38;5;1m\x1b[0m");
}

#[test]
fn hyperlinks_render_osc8() {
    let link = Span::new("docs").blue().link("https://docs.rs");
    assert_eq!(
        link.to_string(),
        "\x1b]8;;https://docs.rs\x1b\\\x1b[38;5;4mdocs\x1b[0m\x1b]8;;\x1b\\"
    );
    assert_eq!(link.url(), Some("https://docs.rs"));
    assert_eq!(link.clone().unlink().url(), None);
    assert_eq!(link.clone().unlink().to_string(), "\x1b[38;5;4mdocs\x1b[0m");
    assert_eq!(link.url(), Some("https://docs.rs")); // original untouched
}

#[test]
fn level_helpers_match_v1() {
    let info = Span::info("starting");
    assert_eq!(info.text(), "[INFO] starting");
    assert_eq!(info.style().foreground(), Some(Color::CYAN));
    assert_eq!(info.style().attributes(), Effects::EMPTY);

    let warn = Span::warn("low disk");
    assert_eq!(warn.text(), "[WARN] low disk");
    assert_eq!(warn.style().foreground(), Some(Color::YELLOW));

    let error = Span::error("boom");
    assert_eq!(error.text(), "[ERROR] boom");
    assert_eq!(error.style().foreground(), Some(Color::RED));
    assert!(error.style().attributes().has(Attribute::Bold));

    let success = Span::success("done");
    assert_eq!(success.text(), "[OK] done");
    assert_eq!(success.style().foreground(), Some(Color::GREEN));
    assert!(success.style().attributes().has(Attribute::Bold));

    let debug = Span::debug("vars");
    assert_eq!(debug.text(), "[DEBUG] vars");
    assert!(debug.style().attributes().has(Attribute::Dim));
}

#[test]
fn banner_matches_v1() {
    let banner = Span::banner("Setup", 21);
    assert_eq!(banner.text(), "─────── Setup ───────");
    assert_eq!(banner.style().foreground(), Some(Color::CYAN));
    assert_eq!(
        banner.to_string(),
        "\x1b[1;38;5;6m─────── Setup ───────\x1b[0m"
    );

    let exact = Span::banner("ab", 8);
    assert_eq!(exact.text(), "── ab ──");

    let small = Span::banner("toolongtitle", 10);
    assert_eq!(small.text(), "toolongtitle");
    assert_eq!(small.style().foreground(), Some(Color::CYAN));
}

#[test]
fn v1_console_alias_works() {
    let error = Console::new("Error: file not found").red().bold();
    assert_eq!(
        error.to_string(),
        "\x1b[1;38;5;1mError: file not found\x1b[0m"
    );

    let style = Style::new().fg(Color::RED).bold();
    let console = Console::new_with_style("styled", style);
    assert_eq!(console.text(), "styled");
    assert_eq!(console.style().foreground(), Some(Color::RED));

    let info = Console::info("x");
    assert_eq!(info.text(), "[INFO] x");
}

#[test]
fn styled_trait_covers_all_methods() {
    assert_eq!("x".black().style().foreground(), Some(Color::BLACK));
    assert_eq!("x".on_white().style().background(), Some(Color::WHITE));
    assert_eq!(
        "x".bright_yellow().style().foreground(),
        Some(Color::BRIGHT_YELLOW)
    );
    assert_eq!(
        "x".strikethrough().style().attributes(),
        Effects::STRIKETHROUGH
    );
    let owned = String::from("owned");
    assert_eq!(owned.red().style().foreground(), Some(Color::RED));
    assert_eq!(owned.styled().text(), "owned");
}

#[test]
fn write_to_respects_color_choice() {
    use pretty_console::{ColorChoice, set_color_choice};

    set_color_choice(ColorChoice::Never);
    let mut buffer = Vec::new();
    Span::new("plain").red().write_to(&mut buffer).unwrap();
    assert_eq!(String::from_utf8(buffer).unwrap(), "plain");

    set_color_choice(ColorChoice::Always);
    let mut buffer = Vec::new();
    Span::new("colored").red().write_to(&mut buffer).unwrap();
    assert_eq!(
        String::from_utf8(buffer).unwrap(),
        "\x1b[38;5;1mcolored\x1b[0m"
    );

    set_color_choice(ColorChoice::Auto);
}

#[test]
fn render_with_degrades_colors() {
    let span = Span::new("x").fg(Color::rgb(255, 0, 0));

    // Truecolor keeps the exact color.
    assert_eq!(
        span.render_with(ColorLevel::TrueColor),
        "\x1b[38;2;255;0;0mx\x1b[0m"
    );
    // 256 and 16 levels degrade.
    assert_eq!(
        span.render_with(ColorLevel::Ansi256),
        "\x1b[38;5;196mx\x1b[0m"
    );
    assert_eq!(span.render_with(ColorLevel::Ansi16), "\x1b[38;5;9mx\x1b[0m");

    // A 256-palette color degrades to 16 when needed.
    let indexed = Span::new("y").fg(Color::indexed(200));
    assert_eq!(
        indexed.render_with(ColorLevel::Ansi16),
        "\x1b[38;5;13my\x1b[0m"
    );
    // Truecolor keeps palette indices intact.
    assert_eq!(
        indexed.render_with(ColorLevel::TrueColor),
        "\x1b[38;5;200my\x1b[0m"
    );
}

#[test]
fn when_applies_conditionally() {
    let verbose = true;
    let span = Span::new("x").bold().when(verbose, |s| s.dim().cyan());
    assert_eq!(span.to_string(), "\x1b[1;2;38;5;6mx\x1b[0m");

    let span = Span::new("x").bold().when(!verbose, Span::dim);
    assert_eq!(span.to_string(), "\x1b[1mx\x1b[0m");
}

#[test]
fn line_composition() {
    let line = Line::new().push("done ").push("in 42ms").green();
    assert_eq!(line.to_string(), "done \x1b[38;5;2min 42ms\x1b[0m");

    let line = Line::new().push("build ").dim().push("ok").green().bold();
    assert_eq!(
        line.to_string(),
        "\x1b[2mbuild \x1b[0m\x1b[1;38;5;2mok\x1b[0m"
    );

    // Styling the first segment then pushing more.
    let line = Line::new().push(Span::new("A").red()).push("B").blue();
    assert_eq!(line.to_string(), "\x1b[38;5;1mA\x1b[0m\x1b[38;5;4mB\x1b[0m");
}

#[test]
fn line_width_and_with_style() {
    use pretty_console::Style;

    let line = Line::new().push("ab").push("cd").bold();
    assert_eq!(line.visible_width(), 4);
    assert_eq!(format!("{line:>6}"), format!("  {}", line.to_string()));

    // with_style merges into every span.
    let all = Line::new()
        .push(Span::new("a").red())
        .push("b")
        .with_style(Style::new().bold());
    assert_eq!(all.to_string(), "\x1b[1;38;5;1ma\x1b[0m\x1b[1mb\x1b[0m");
}

#[test]
fn macros_build_expected_values() {
    let span = pretty_console::paint!(red bold on_black, "count = {}", 42);
    assert_eq!(span.text(), "count = 42");
    assert_eq!(span.to_string(), "\x1b[1;38;5;1;48;5;0mcount = 42\x1b[0m");

    let heading = pretty_console::style!(bold underline bright_blue);
    assert_eq!(
        heading.attributes(),
        Effects::BOLD.union(Effects::UNDERLINE)
    );
    assert_eq!(heading.foreground(), Some(Color::BRIGHT_BLUE));

    let row = pretty_console::line!("a ", "b");
    assert_eq!(row.spans().len(), 2);
    assert_eq!(row.visible_width(), 3);
}
