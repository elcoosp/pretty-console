//! Tests for the ANSI-aware text toolkit.

use pretty_console::{Align, Styled, pad, strip_ansi, truncate, visible_width};

#[test]
fn strip_ansi_v1_semantics() {
    assert_eq!(strip_ansi("plain"), "plain");
    assert_eq!(strip_ansi(""), "");
    assert_eq!(strip_ansi("\x1b[1;31mred\x1b[0m"), "red");
    assert_eq!(strip_ansi("\x1b[38;2;1;2;3mrgb\x1b[0m"), "rgb");
    assert_eq!(strip_ansi("\x1b[?25hvisible"), "visible");
    assert_eq!(strip_ansi("text\x1b[31"), "text");
    assert_eq!(strip_ansi("\x1b]0;window title\x07rest"), "rest");
    assert_eq!(
        strip_ansi("\x1b]8;;https://example.com\x1b\\link\x1b]8;;\x1b\\"),
        "link"
    );
    assert_eq!(strip_ansi("a\x1bMb"), "ab");
    assert_eq!(strip_ansi("done\x1b"), "done");
    assert_eq!(strip_ansi("\x1b[1;31ma\x1b[0mb\x1b[32mc\x1b[0m"), "abc");
}

#[test]
fn strip_ansi_keeps_unicode() {
    assert_eq!(strip_ansi("héllo wörld"), "héllo wörld");
    assert_eq!(strip_ansi("日本語"), "日本語");
    // ESC followed by a multibyte character consumes exactly that character.
    assert_eq!(strip_ansi("a\x1béb"), "ab");
    assert_eq!(strip_ansi("\x1b[1m日本\x1b[0m"), "日本");
}

#[test]
fn strip_styled_output() {
    let styled = "hello".red().bold();
    assert_eq!(strip_ansi(&styled.to_string()), "hello");
}

#[test]
fn char_width_rules() {
    assert_eq!(visible_width(""), 0);
    assert_eq!(visible_width("abc"), 3);
    assert_eq!(visible_width("héllo"), 5);
    assert_eq!(visible_width("日本語"), 6);
    assert_eq!(visible_width("한국"), 4);
    // Combining marks are zero-width.
    assert_eq!(visible_width("e\u{0301}"), 1);
    // Control characters are zero-width.
    assert_eq!(visible_width("a\nb"), 2);
    assert_eq!(visible_width("\u{1F600}"), 2); // emoji
}

#[test]
fn visible_width_ignores_ansi() {
    let styled = "hi".red().bold();
    assert_eq!(visible_width(&styled.to_string()), 2);
    assert_eq!(visible_width("\x1b[1mx\x1b[0m"), 1);
    assert_eq!(visible_width(&"日本".cyan().to_string()), 4);
}

#[test]
fn truncate_basics() {
    assert_eq!(truncate("hello world", 8, "…"), "hello w…");
    assert_eq!(truncate("short", 8, "…"), "short");
    assert_eq!(truncate("", 4, "…"), "");
    // Width smaller than the tail: tail wins (documented overrun).
    assert_eq!(truncate("abcdef", 0, "…"), "…");
    assert_eq!(truncate("abcdef", 1, "..."), "...");
    // Exact width is not truncated.
    assert_eq!(truncate("abcd", 4, "…"), "abcd");
    // Wide characters are never split across the boundary.
    assert_eq!(truncate("a日本語", 4, ""), "a日");
    assert_eq!(truncate("a日本語", 3, ""), "a日");
}

#[test]
#[cfg(not(feature = "no-color"))]
fn truncate_preserves_styles_and_resets() {
    let styled = "hello world".red().to_string();
    let cut = truncate(&styled, 5, "…");
    assert_eq!(visible_width(&cut), 5);
    // The color code is preserved...
    assert!(cut.contains("\x1b[38;5;1m"));
    // ...and a reset is appended so the style cannot leak.
    assert!(cut.ends_with("\x1b[0m"));
    assert_eq!(strip_ansi(&cut), "hell…");
}

#[test]
#[cfg(not(feature = "no-color"))]
fn truncate_multibyte_tail() {
    let styled = "日本語テスト".cyan().to_string();
    let cut = truncate(&styled, 6, "…");
    assert!(visible_width(&cut) <= 6);
    assert!(cut.ends_with("\x1b[0m"));
}

#[test]
fn pad_basics() {
    assert_eq!(pad("hi", 4, Align::Left, '.'), "hi..");
    assert_eq!(pad("hi", 4, Align::Right, '.'), "..hi");
    assert_eq!(pad("hi", 5, Align::Center, '.'), ".hi..");
    assert_eq!(pad("hi", 6, Align::Center, '.'), "..hi..");
    // Content wider than the target is returned unchanged.
    assert_eq!(pad("hello", 3, Align::Left, '.'), "hello");
    assert_eq!(pad("", 3, Align::Left, ' '), "   ");
}

#[test]
fn pad_is_ansi_aware() {
    let styled = "hi".green().to_string();
    assert_eq!(pad(&styled, 4, Align::Right, ' '), format!("  {styled}"));
    let cjk = "日本".cyan().to_string();
    assert_eq!(pad(&cjk, 6, Align::Left, ' '), format!("{cjk}  "));
}
