//! Tests for the `Panel` builder.
#![cfg_attr(feature = "no-color", allow(unused_imports))]

use pretty_console::{Line, Panel, Span, Style};

fn plain_panel() -> Panel {
    Panel::new().title("Report").line("alpha").line("beta")
}

#[test]
fn panel_structure() {
    let out = plain_panel().render_plain();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 4);

    // Title inlaid into the top border; inner width raised to fit it
    // (title 6 + 2*padding + 1 = 9 > content 5 + 2 = 7).
    assert_eq!(lines[0], "╭─ Report ╮");
    assert_eq!(lines[1], "│ alpha   │");
    assert_eq!(lines[2], "│ beta    │");
    assert_eq!(lines[3], "╰─────────╯");
}

#[test]
fn panel_widens_for_long_lines() {
    let panel = Panel::new().line("a").line("a much longer line");
    let out = panel.render_plain();
    let lines: Vec<&str> = out.lines().collect();
    // No title: inner width is the longest line + 2 * padding.
    assert_eq!(lines[0], "╭────────────────────╮");
    assert_eq!(lines[1], "│ a                  │");
    assert_eq!(lines[3], "╰────────────────────╯");
}

#[test]
fn panel_fixed_width_truncates() {
    let panel = Panel::new()
        .width(12)
        .line("short")
        .line("this line is definitely too long");
    let out = panel.render_plain();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[1], "│ short        │");
    // Over-wide lines are truncated with an ellipsis to fit.
    assert_eq!(lines[2], "│ this line i… │");
    assert_eq!(lines[3], "╰──────────────╯");
}

#[test]
#[cfg(not(feature = "no-color"))]
fn panel_styled_border() {
    let panel = plain_panel().border(Style::new().dim());
    let out = panel.render();
    // Border characters carry the dim code, content does not.
    assert!(out.contains("\x1b[2m╭"));
    assert!(out.contains("alpha"));
    assert!(out.contains("\x1b[2m╯"));
}

#[test]
fn panel_styled_content_and_cjk() {
    let panel = Panel::new()
        .title("日本")
        .line(Span::new("ok").green())
        .line(Line::new().push("wide ").push("日本").cyan());
    let out = panel.render_plain();
    let lines: Vec<&str> = out.lines().collect();
    // Every rendered row has the same visible width.
    for line in &lines[1..3] {
        assert_eq!(
            pretty_console::visible_width(line),
            pretty_console::visible_width(lines[0])
        );
        assert_eq!(
            pretty_console::visible_width(line),
            pretty_console::visible_width(lines[3])
        );
    }
    assert!(lines[1].contains("ok"));
}

#[test]
#[cfg(not(feature = "no-color"))]
fn panel_write_to_respects_choice() {
    use pretty_console::{ColorChoice, set_color_choice};

    set_color_choice(ColorChoice::Never);
    let mut buffer = Vec::new();
    plain_panel()
        .border(Style::new().red())
        .write_to(&mut buffer)
        .unwrap();
    let text = String::from_utf8(buffer).unwrap();
    assert!(!text.contains('\x1b'));
    assert!(text.contains("alpha"));

    set_color_choice(ColorChoice::Always);
    let mut buffer = Vec::new();
    plain_panel()
        .border(Style::new().red())
        .write_to(&mut buffer)
        .unwrap();
    let text = String::from_utf8(buffer).unwrap();
    assert!(text.contains("\x1b[38;5;1m╭"));

    set_color_choice(ColorChoice::Auto);
}

#[test]
fn panel_defaults_and_accessors() {
    let panel = Panel::default();
    assert_eq!(panel.content_width(), 2);
    let empty = panel.render_plain();
    assert_eq!(empty, "╭──╮\n╰──╯");

    let panel = Panel::new().title("T").padding(2).line("hello");
    // Content 5 + 2*2 = 9; title minimum 1 + 2*2 + 1 = 6.
    assert_eq!(panel.content_width(), 9);
    let out = panel.render_plain();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[1], "│  hello  │");
}

#[test]
fn panel_display_impl() {
    let panel = plain_panel();
    let rendered = format!("{panel}");
    assert_eq!(rendered, panel.render());
}
