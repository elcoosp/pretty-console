//! Demonstrates the log-level helpers, banners and runtime color control.

use pretty_console::{ColorChoice, Console, Styled, set_color_choice};

fn main() {
    // Log-level helpers.
    Console::info("Compiling pretty-console").println();
    Console::success("All tests passed").println();
    Console::warn("This API is deprecated").println();
    Console::error("Disk full").println();
    Console::debug("cache_size = 42").println();

    // Banners.
    Console::banner("Build report", 60).println();
    Console::banner("narrow", 10).println();

    // Runtime color control: force colors even when piped.
    set_color_choice(ColorChoice::Always);
    "forced colors".magenta().println();

    // Disable colors entirely.
    set_color_choice(ColorChoice::Never);
    Console::new("no colors here").red().bold().println();

    // Back to automatic detection (env vars + terminal detection).
    set_color_choice(ColorChoice::Auto);
    Console::new("back to auto").cyan().println();
}
