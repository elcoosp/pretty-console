//! Runtime color control: the global [`ColorChoice`], environment detection
//! and terminal capability levels.

#[cfg(not(feature = "no-color"))]
use std::io::IsTerminal;
use std::sync::atomic::{AtomicU8, Ordering};

/// Runtime color policy used when printing to a terminal.
///
/// The default is [`Auto`](ColorChoice::Auto), which inspects the standard
/// environment variables and detects whether stdout is a terminal.
///
/// # Examples
///
/// ```
/// use pretty_console::{set_color_choice, ColorChoice};
///
/// // e.g. after parsing a --color flag from your CLI:
/// set_color_choice(ColorChoice::Never);
/// set_color_choice(ColorChoice::Auto);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ColorChoice {
    /// Detect automatically from the environment and stdout.
    #[default]
    Auto,
    /// Always emit colors, regardless of the environment.
    Always,
    /// Never emit colors, regardless of the environment.
    Never,
}

static COLOR_CHOICE: AtomicU8 = AtomicU8::new(0);
/// Cached decision for `Auto`: 0 = unknown, 1 = false, 2 = true.
static CACHED_ENABLED: AtomicU8 = AtomicU8::new(0);

/// Sets the global [`ColorChoice`].
///
/// This affects `print`, `println`, `eprint`, `eprintln`, `try_print*`,
/// `try_eprint*` and `write_to`. It does not affect string rendering.
pub fn set_color_choice(choice: ColorChoice) {
    let encoded = match choice {
        ColorChoice::Auto => 0,
        ColorChoice::Always => 1,
        ColorChoice::Never => 2,
    };
    COLOR_CHOICE.store(encoded, Ordering::Relaxed);
    CACHED_ENABLED.store(0, Ordering::Relaxed);
}

/// Returns the current global [`ColorChoice`].
#[must_use]
pub fn color_choice() -> ColorChoice {
    match COLOR_CHOICE.load(Ordering::Relaxed) {
        1 => ColorChoice::Always,
        2 => ColorChoice::Never,
        _ => ColorChoice::Auto,
    }
}

/// Returns whether colors should currently be emitted when printing.
///
/// With the `no-color` feature enabled this always returns `false`.
///
/// The environment detection for [`ColorChoice::Auto`] runs once and is then
/// cached; [`set_color_choice`] invalidates the cache.
#[must_use]
pub fn is_color_enabled() -> bool {
    #[cfg(feature = "no-color")]
    {
        false
    }
    #[cfg(not(feature = "no-color"))]
    {
        match color_choice() {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => match CACHED_ENABLED.load(Ordering::Relaxed) {
                1 => false,
                2 => true,
                _ => {
                    let enabled = env_enabled(
                        |key| std::env::var(key).ok(),
                        std::io::stdout().is_terminal(),
                    );
                    CACHED_ENABLED.store(if enabled { 2 } else { 1 }, Ordering::Relaxed);
                    enabled
                }
            },
        }
    }
}

/// Pure decision procedure for [`ColorChoice::Auto`], parameterized over the
/// environment for testability.
///
/// The checks, in order, are:
///
/// 1. `FORCE_COLOR` set to a non-empty value other than `0` forces colors on;
///    `FORCE_COLOR=0` forces them off.
/// 2. `CLICOLOR_FORCE` set to a non-empty value other than `0` forces colors on.
/// 3. `NO_COLOR` set to a non-empty value disables colors
///    (see <https://no-color.org/>).
/// 4. `CLICOLOR=0` disables colors.
/// 5. `TERM=dumb` disables colors.
/// 6. Otherwise colors are emitted only when stdout is a terminal.
#[cfg_attr(feature = "no-color", allow(dead_code))]
pub(crate) fn env_enabled(get: impl Fn(&str) -> Option<String>, is_tty: bool) -> bool {
    let non_empty = |key: &str| get(key).filter(|value| !value.trim().is_empty());

    if let Some(value) = non_empty("FORCE_COLOR") {
        return value != "0";
    }
    if non_empty("CLICOLOR_FORCE").is_some_and(|value| value != "0") {
        return true;
    }
    if non_empty("NO_COLOR").is_some() {
        return false;
    }
    if non_empty("CLICOLOR").is_some_and(|value| value == "0") {
        return false;
    }
    if non_empty("TERM").is_some_and(|value| value == "dumb") {
        return false;
    }
    is_tty
}

/// The richest color mode the terminal can display exactly.
///
/// Used with [`Span::render_with`](crate::Span::render_with) and
/// [`Line::render_with`](crate::Line::render_with) to degrade colors for
/// limited terminals.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorLevel {
    /// Only the 16 base ANSI colors.
    #[default]
    Ansi16,
    /// The full 256-color palette.
    Ansi256,
    /// 24-bit RGB.
    TrueColor,
}

/// Detects the terminal's color capability from the environment.
///
/// Returns `None` when no colors should be emitted at all. Detection order:
///
/// 1. `FORCE_COLOR`: `0` → `None`, `1` → `Ansi16`, `2` → `Ansi256`, anything
///    else → `TrueColor` (Node.js convention)
/// 2. `CLICOLOR_FORCE` (non-zero) → `Ansi16`
/// 3. `NO_COLOR` → `None`
/// 4. `CLICOLOR=0` → `None`
/// 5. `TERM=dumb` → `None`
/// 6. `COLORTERM` of `truecolor`/`24bit` → `TrueColor`
/// 7. `TERM` containing `256color` → `Ansi256`; any other `TERM` → `Ansi16`
/// 8. A tty stdout → `TrueColor` (the modern default)
/// 9. Otherwise `None`
///
/// # Examples
///
/// ```
/// use pretty_console::{supports_color_level, ColorLevel};
///
/// let level = std::env::var("COLORTERM")
///     .ok()
///     .filter(|v| v == "truecolor" || v == "24bit")
///     .map(|_| ColorLevel::TrueColor);
/// if level == supports_color_level().filter(|l| *l == ColorLevel::TrueColor) {
///     // the terminal advertises truecolor
/// }
/// ```
#[must_use]
pub fn supports_color_level() -> Option<ColorLevel> {
    #[cfg(feature = "no-color")]
    {
        None
    }
    #[cfg(not(feature = "no-color"))]
    {
        let non_empty = |key: &str| std::env::var(key).ok().filter(|v| !v.trim().is_empty());

        if let Some(value) = non_empty("FORCE_COLOR") {
            return match value.trim() {
                "0" => None,
                "1" => Some(ColorLevel::Ansi16),
                "2" => Some(ColorLevel::Ansi256),
                _ => Some(ColorLevel::TrueColor),
            };
        }
        if non_empty("CLICOLOR_FORCE").is_some_and(|v| v != "0") {
            return Some(ColorLevel::Ansi16);
        }
        if non_empty("NO_COLOR").is_some() {
            return None;
        }
        if non_empty("CLICOLOR").is_some_and(|v| v == "0") {
            return None;
        }
        if non_empty("TERM").is_some_and(|v| v == "dumb") {
            return None;
        }
        if let Some(term) = non_empty("COLORTERM") {
            if term == "truecolor" || term == "24bit" {
                return Some(ColorLevel::TrueColor);
            }
        }
        if let Some(term) = non_empty("TERM") {
            if term.contains("256color") {
                return Some(ColorLevel::Ansi256);
            }
            if term.contains("direct") {
                return Some(ColorLevel::TrueColor);
            }
            return Some(ColorLevel::Ansi16);
        }
        if std::io::stdout().is_terminal() {
            return Some(ColorLevel::TrueColor);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// Serializes tests that touch the global [`ColorChoice`] state.
    static GLOBAL_LOCK: Mutex<()> = Mutex::new(());

    fn env_of<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        let map: HashMap<&str, String> = pairs
            .iter()
            .map(|(key, value)| (*key, (*value).to_string()))
            .collect();
        move |key: &str| map.get(key).cloned()
    }

    type EnvCase = (&'static [(&'static str, &'static str)], bool, bool);

    #[test]
    fn env_detection_matches_v1() {
        let cases: &[EnvCase] = &[
            (&[("FORCE_COLOR", "1")], true, true),
            (&[("FORCE_COLOR", "0")], false, false),
            (&[("FORCE_COLOR", "")], true, false),
            (&[("CLICOLOR_FORCE", "1")], true, true),
            (&[("CLICOLOR_FORCE", "0")], true, false),
            (&[("NO_COLOR", "1")], false, false),
            (&[("NO_COLOR", "")], true, false),
            (&[("CLICOLOR", "0")], false, false),
            (&[("TERM", "dumb")], false, false),
            (&[], true, false),
            (
                &[("FORCE_COLOR", "0"), ("CLICOLOR_FORCE", "1")],
                false,
                false,
            ),
        ];

        for (env, with_tty, without_tty) in cases {
            assert_eq!(
                env_enabled(env_of(env), true),
                *with_tty,
                "env {env:?} with tty"
            );
            assert_eq!(
                env_enabled(env_of(env), false),
                *without_tty,
                "env {env:?} without tty"
            );
        }
    }

    #[test]
    fn color_choice_roundtrip_and_cache_invalidation() {
        let _guard = GLOBAL_LOCK.lock().unwrap();
        assert_eq!(color_choice(), ColorChoice::Auto);
        let _ = is_color_enabled();

        set_color_choice(ColorChoice::Always);
        assert_eq!(color_choice(), ColorChoice::Always);
        #[cfg(not(feature = "no-color"))]
        assert!(is_color_enabled());

        set_color_choice(ColorChoice::Never);
        assert_eq!(color_choice(), ColorChoice::Never);
        assert!(!is_color_enabled());

        set_color_choice(ColorChoice::Auto);
        assert_eq!(color_choice(), ColorChoice::Auto);
    }

    #[test]
    fn cached_enabled_matches_first_detection() {
        let _guard = GLOBAL_LOCK.lock().unwrap();
        // The cache must agree with a fresh decision and survive repeats.
        set_color_choice(ColorChoice::Auto);
        let first = is_color_enabled();
        for _ in 0..5 {
            assert_eq!(is_color_enabled(), first);
        }
    }
}
