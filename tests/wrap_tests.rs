//! Contract tests for ANSI-aware word wrapping: the free [`wrap`] function,
//! [`Span::wrap`], [`Line::wrap`] and [`Panel::soft_wrap`].
//!
//! Every expectation is an exact string, so these tests pin down the wrapping
//! algorithm, the style-safety guarantees (no SGR/OSC 8 state leaks across
//! lines) and the CJK-aware column arithmetic.

use pretty_console::{Line, Panel, Span, WrapMode, visible_width, wrap};

// ---------------------------------------------------------------------------
// Free function: word mode
// ---------------------------------------------------------------------------

#[test]
fn word_wrap_basic() {
    assert_eq!(wrap("hello world", 5, WrapMode::Word), ["hello", "world"]);
}

#[test]
fn word_wrap_exact_fit_is_one_line() {
    assert_eq!(wrap("hello world", 11, WrapMode::Word), ["hello world"]);
    assert_eq!(wrap("hello world", 20, WrapMode::Word), ["hello world"]);
}

#[test]
fn word_wrap_breaks_at_latest_space_that_fits() {
    assert_eq!(
        wrap("the quick brown fox", 10, WrapMode::Word),
        ["the quick", "brown fox"]
    );
}

#[test]
fn word_wrap_drops_the_space_at_a_break() {
    assert_eq!(wrap("a b c", 3, WrapMode::Word), ["a b", "c"]);
}

#[test]
fn word_wrap_keeps_internal_space_runs() {
    assert_eq!(wrap("a  b", 4, WrapMode::Word), ["a  b"]);
}

#[test]
fn word_wrap_drops_space_run_at_a_break() {
    assert_eq!(wrap("a  b", 3, WrapMode::Word), ["a", "b"]);
}

#[test]
fn word_wrap_overlong_word_overflows_own_line() {
    assert_eq!(wrap("abcdefgh", 3, WrapMode::Word), ["abcdefgh"]);
    assert_eq!(wrap("ab cdefghij", 3, WrapMode::Word), ["ab", "cdefghij"]);
}

#[test]
fn word_wrap_never_splits_at_a_hard_break() {
    // The overflow rule applies per paragraph: "cdef" is one word wider than
    // the budget, so it overflows onto its own line unsplit.
    assert_eq!(wrap("ab\ncdef", 3, WrapMode::Word), ["ab", "cdef"]);
}

#[test]
fn word_wrap_empty_input_is_one_empty_line() {
    assert_eq!(wrap("", 5, WrapMode::Word), [""]);
}

#[test]
fn word_wrap_whitespace_only_is_one_empty_line() {
    assert_eq!(wrap("   ", 5, WrapMode::Word), [""]);
}

#[test]
fn zero_width_disables_wrapping() {
    assert_eq!(wrap("abc", 0, WrapMode::Word), ["abc"]);
    assert_eq!(wrap("a b", 0, WrapMode::Char), ["a b"]);
}

#[test]
fn newlines_are_hard_breaks() {
    assert_eq!(wrap("ab\ncdef", 10, WrapMode::Word), ["ab", "cdef"]);
    assert_eq!(wrap("ab\ncdef gh", 5, WrapMode::Word), ["ab", "cdef", "gh"]);
}

// ---------------------------------------------------------------------------
// Free function: char mode
// ---------------------------------------------------------------------------

#[test]
fn char_wrap_hard_breaks_every_budget() {
    assert_eq!(wrap("abcdefgh", 3, WrapMode::Char), ["abc", "def", "gh"]);
}

#[test]
fn char_wrap_wide_chars() {
    assert_eq!(wrap("日本語", 4, WrapMode::Char), ["日本", "語"]);
    assert_eq!(wrap("日本語", 2, WrapMode::Char), ["日", "本", "語"]);
}

#[test]
fn char_wrap_unbreakable_wide_cell_overflows() {
    // A single wide cell cannot be split: at a budget of 1 every cell goes
    // on its own (overwidth) line.
    assert_eq!(wrap("日本", 1, WrapMode::Char), ["日", "本"]);
}

// ---------------------------------------------------------------------------
// Free function: CJK awareness in word mode
// ---------------------------------------------------------------------------

#[test]
fn word_wrap_breaks_between_wide_chars() {
    assert_eq!(wrap("日本語", 4, WrapMode::Word), ["日本", "語"]);
}

#[test]
fn word_wrap_mixed_scripts() {
    assert_eq!(wrap("ab日本cd", 4, WrapMode::Word), ["ab日", "本cd"]);
}

// ---------------------------------------------------------------------------
// Free function: zero-width characters
// ---------------------------------------------------------------------------

#[test]
fn combining_mark_stays_with_its_base() {
    assert_eq!(wrap("e\u{0301}x y", 2, WrapMode::Word), ["e\u{0301}x", "y"]);
}

// ---------------------------------------------------------------------------
// Free function: style safety
// ---------------------------------------------------------------------------

#[test]
fn wrap_closes_and_reopens_open_styles() {
    // The crate's reset is the full `\x1b[0m` sequence.
    assert_eq!(
        wrap("\x1b[1mhello world\x1b[0m", 5, WrapMode::Word),
        ["\x1b[1mhello\x1b[0m", "\x1b[1mworld\x1b[0m"]
    );
}

#[test]
fn wrap_does_not_double_reset() {
    // The input's own reset lands before the break, so no synthetic reset is
    // added and the continuation line is not re-opened.
    assert_eq!(
        wrap("a\x1b[31mb\x1b[0m c", 2, WrapMode::Word),
        ["a\x1b[31mb\x1b[0m", "c"]
    );
}

#[test]
fn wrap_closes_dangling_style_on_every_line() {
    assert_eq!(
        wrap("\x1b[1mab cd", 2, WrapMode::Word),
        ["\x1b[1mab\x1b[0m", "\x1b[1mcd\x1b[0m"]
    );
}

#[test]
fn wrap_hard_break_closes_open_style() {
    assert_eq!(
        wrap("\x1b[1mab\ncd", 10, WrapMode::Word),
        ["\x1b[1mab\x1b[0m", "\x1b[1mcd\x1b[0m"]
    );
}

#[cfg(not(feature = "no-color"))]
#[test]
fn wrap_continues_hyperlinks_across_lines() {
    // A rendered Span splits mid-text: each wrapped line re-opens both the
    // OSC 8 hyperlink and the SGR style, and closes both at its end.
    let rendered = Span::new("hello world").red().link("http://x").render();
    assert_eq!(
        wrap(&rendered, 5, WrapMode::Word),
        [
            "\x1b]8;;http://x\x1b\\\x1b[38;5;1mhello\x1b[0m\x1b]8;;\x1b\\",
            "\x1b]8;;http://x\x1b\\\x1b[38;5;1mworld\x1b[0m\x1b]8;;\x1b\\",
        ]
    );
}

#[test]
fn wrap_plain_input_never_gains_escapes() {
    for line in wrap("hello world", 5, WrapMode::Word) {
        assert!(!line.contains('\x1b'));
    }
}

// ---------------------------------------------------------------------------
// Free function: width invariant
// ---------------------------------------------------------------------------

#[test]
fn every_wrapped_line_fits_the_budget() {
    // No token in the input is wider than 6 columns, so word-mode budgets of
    // 7+ never trigger the unbreakable-overflow rule.
    let input = "日本語 テスト abc defg styled \x1b[1mbold text\x1b[0m here";
    for width in [7usize, 10, 12, 20] {
        for line in wrap(input, width, WrapMode::Word) {
            assert!(
                visible_width(&line) <= width,
                "word width {width}: line {line:?} is {} columns",
                visible_width(&line)
            );
        }
    }
    // Char mode hard-breaks every cell, so budgets of 3+ (wide cells are 2
    // columns) also never overflow.
    for width in [3usize, 5, 8] {
        for line in wrap(input, width, WrapMode::Char) {
            assert!(
                visible_width(&line) <= width,
                "char width {width}: line {line:?} is {} columns",
                visible_width(&line)
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Span::wrap
// ---------------------------------------------------------------------------

#[test]
fn span_wrap_preserves_style_and_link() {
    let span = Span::new("hello world").red().bold().link("http://x");
    let fragments = span.wrap(5, WrapMode::Word);
    assert_eq!(fragments.len(), 2);
    assert_eq!(fragments[0].text(), "hello");
    assert_eq!(fragments[1].text(), "world");
    for fragment in &fragments {
        assert_eq!(fragment.style(), span.style());
        assert_eq!(fragment.url(), Some("http://x"));
    }
}

#[test]
fn span_wrap_char_mode() {
    let span = Span::new("abcde").red();
    let fragments = span.wrap(3, WrapMode::Char);
    assert_eq!(fragments.len(), 2);
    assert_eq!(fragments[0].text(), "abc");
    assert_eq!(fragments[1].text(), "de");
    for fragment in &fragments {
        assert_eq!(fragment.style(), span.style());
    }
}

#[test]
fn span_wrap_never_returns_empty() {
    let fragments = Span::new("").red().wrap(5, WrapMode::Word);
    assert_eq!(fragments.len(), 1);
}

// ---------------------------------------------------------------------------
// Line::wrap
// ---------------------------------------------------------------------------

#[test]
fn line_wrap_word_mode_splits_between_spans() {
    let line = Line::new().push("ab ").push("cdefg").red();
    let rows = line.wrap(4, WrapMode::Word);
    assert_eq!(rows.len(), 2);
    // The trailing space is trimmed at the break.
    assert_eq!(rows[0].spans().len(), 1);
    assert_eq!(rows[0].spans()[0].text(), "ab");
    assert_eq!(rows[1].spans().len(), 1);
    assert_eq!(rows[1].spans()[0].text(), "cdefg");
    assert_eq!(rows[1].spans()[0].style(), line.spans()[1].style());
}

#[test]
fn line_wrap_char_mode_splits_inside_a_span() {
    let line = Line::new().push("abcde").red();
    let rows = line.wrap(3, WrapMode::Char);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].spans().len(), 1);
    assert_eq!(rows[0].spans()[0].text(), "abc");
    assert_eq!(rows[0].spans()[0].style(), line.spans()[0].style());
    assert_eq!(rows[1].spans()[0].text(), "de");
    assert_eq!(rows[1].spans()[0].style(), line.spans()[0].style());
}

#[test]
fn line_wrap_mixed_styles_across_a_break() {
    let line = Line::new().push("ab").red().push("cdef");
    let rows = line.wrap(3, WrapMode::Char);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].spans().len(), 2);
    assert_eq!(rows[0].spans()[0].text(), "ab");
    assert_eq!(rows[0].spans()[0].style(), line.spans()[0].style());
    assert_eq!(rows[0].spans()[1].text(), "c");
    assert_eq!(rows[0].spans()[1].style(), line.spans()[1].style());
    assert_eq!(rows[1].spans().len(), 1);
    assert_eq!(rows[1].spans()[0].text(), "def");
}

#[test]
fn line_wrap_returns_one_row_when_it_fits() {
    let line = Line::new().push("hi ").push("there").green();
    let rows = line.wrap(20, WrapMode::Word);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0], line);
}

#[test]
fn line_wrap_always_returns_at_least_one_row() {
    let rows = Line::new().wrap(5, WrapMode::Word);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].spans(), []);
}

// ---------------------------------------------------------------------------
// Panel::soft_wrap
// ---------------------------------------------------------------------------

#[test]
fn panel_soft_wrap_reflows_instead_of_truncating() {
    let panel = Panel::new()
        .width(8)
        .soft_wrap(true)
        .line("one two three four");
    let plain = panel.render_plain();
    assert!(!plain.contains('…'));
    let rows: Vec<&str> = plain
        .lines()
        .filter(|l| l.contains("one two") || l.contains("three") || l.contains("four"))
        .collect();
    assert_eq!(rows.len(), 3, "expected three reflowed rows, got {plain:?}");
    assert!(rows[0].contains("one two"));
    assert!(rows[1].contains("three"));
    assert!(rows[2].contains("four"));
    // Every rendered row keeps the panel geometry intact.
    let widths: std::collections::HashSet<usize> =
        plain.lines().map(str::chars).map(Iterator::count).collect();
    assert_eq!(widths.len(), 1, "ragged panel rows: {plain:?}");
}

#[test]
fn panel_without_soft_wrap_still_truncates() {
    let panel = Panel::new().width(8).line("one two three four");
    let plain = panel.render_plain();
    assert!(plain.contains('…'));
    assert!(!plain.contains("four"));
}
