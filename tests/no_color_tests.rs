//! Behavior of the `no-color` feature: every method exists and every
//! renderer is a pass-through.
#![cfg(feature = "no-color")]

use pretty_console::{
    Color, ColorLevel, Line, Panel, Span, Style, Styled, gradient, pad, rainbow, strip_ansi,
    truncate, visible_width,
};

#[test]
fn spans_render_plain() {
    assert_eq!(
        Span::new("Hello, world!").red().bold().to_string(),
        "Hello, world!"
    );
    assert_eq!(
        Span::new("RGB Text")
            .fg_rgb(255, 0, 128)
            .bg_rgb(0, 255, 0)
            .underline()
            .to_string(),
        "RGB Text"
    );
    assert_eq!(Span::new("x").fg_hex(0x00FF_0000).to_string(), "x");
    assert_eq!(Span::new("").red().bold().to_string(), "");
}

#[test]
fn render_with_still_plain() {
    let span = Span::new("x").fg(Color::rgb(255, 0, 0));
    for level in [
        ColorLevel::Ansi16,
        ColorLevel::Ansi256,
        ColorLevel::TrueColor,
    ] {
        assert_eq!(span.render_with(level), "x");
    }
}

#[test]
fn hyperlinks_suppressed() {
    let link = Span::new("docs").blue().link("https://docs.rs");
    assert_eq!(link.to_string(), "docs");
}

#[test]
fn helpers_and_banners_plain() {
    assert_eq!(Span::info("msg").to_string(), "[INFO] msg");
    assert_eq!(Span::error("msg").to_string(), "[ERROR] msg");
    assert_eq!(
        Span::banner("Setup", 21).to_string(),
        "─────── Setup ───────"
    );
}

#[test]
fn lines_and_panels_plain() {
    let line = Line::new().push("a ").push("b").red();
    assert_eq!(line.to_string(), "a b");

    let panel = Panel::new().title("T").line("hello");
    let plain = panel.render_plain();
    assert_eq!(plain, panel.render());
    assert!(plain.contains("hello"));
}

#[test]
fn decorations_plain() {
    let fade = gradient("abc", Color::BLACK, Color::WHITE);
    assert_eq!(fade.render(), "abc");
    let arc = rainbow("abc");
    assert_eq!(arc.render(), "abc");
}

#[test]
fn text_tools_unaffected() {
    assert_eq!(strip_ansi("\x1b[1mx\x1b[0m"), "x");
    assert_eq!(visible_width("日本"), 4);
    assert_eq!(truncate("hello world", 8, "…"), "hello w…");
    assert_eq!(pad("hi", 4, pretty_console::Align::Left, '.'), "hi..");
    // Styled input is already plain, and stays plain.
    let styled = "hi".red();
    assert_eq!(
        pad(&styled.to_string(), 4, pretty_console::Align::Left, '.'),
        "hi.."
    );
}

#[test]
fn choice_stays_disabled() {
    use pretty_console::{ColorChoice, set_color_choice};
    for choice in [ColorChoice::Auto, ColorChoice::Always, ColorChoice::Never] {
        set_color_choice(choice);
        assert!(!pretty_console::is_color_enabled());
    }
    assert!(pretty_console::supports_color_level().is_none());
}

#[test]
fn styles_are_copy_and_const() {
    const HEADING: Style = Style::new().bold().underline().fg(Color::BRIGHT_BLUE);
    let a = HEADING;
    let b = HEADING;
    assert_eq!(a, b);
    assert_eq!(a.paint("x").to_string(), "x");
}
