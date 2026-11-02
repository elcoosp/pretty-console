//! Micro-benchmarks comparing pretty-console's hot paths against the
//! owo-colors, yansi and colored crates.
//!
//! Run with: `cargo run --release --example bench`
//!
//! This is a deliberately dependency-free harness (no criterion) so the crate
//! stays light; the numbers are wall-clock throughput on the current machine
//! and are meant for relative comparison.

// Throughput math only needs approximate f64 conversion.
#![allow(clippy::cast_precision_loss)]

use std::fmt::Write as _;
use std::time::Instant;

use pretty_console::Span;

const ITERATIONS: usize = 200_000;
const STRIP_INPUT: &str = "the quick brown fox jumps over \x1b[1;38;5;9mthe lazy dog\x1b[0m again and again \x1b[38;2;10;20;30mand over\x1b[0m";

fn bench(name: &str, iterations: usize, mut f: impl FnMut(usize)) -> f64 {
    // Warmup.
    f(100);
    let start = Instant::now();
    for i in 0..iterations {
        f(i);
    }
    let elapsed = start.elapsed().as_secs_f64();
    let ops = iterations as f64 / elapsed;
    println!("  {name:<44} {ops:>12.0} ops/s   ({elapsed:.3}s total)");
    ops
}

// Each competitor is isolated in its own function so only one extension trait
// is in scope at a time (they all define `red`/`bold`/...).

fn pc_to_string(i: usize) -> String {
    Span::new(format!("iteration {i}"))
        .red()
        .bold()
        .on_black()
        .to_string()
}

fn owo_to_string(i: usize) -> String {
    use owo_colors::OwoColorize as _;
    format!("iteration {i}").red().bold().on_black().to_string()
}

fn yansi_to_string(i: usize) -> String {
    use yansi::{Color, Paint};
    Paint::new(format!("iteration {i}"))
        .fg(Color::Red)
        .bold()
        .bg(Color::Black)
        .to_string()
}

fn colored_to_string(i: usize) -> String {
    use colored::Colorize as _;
    format!("iteration {i}").red().bold().on_black().to_string()
}

fn pc_display(i: usize) -> String {
    let mut buf = String::with_capacity(64);
    write!(
        &mut buf,
        "{}",
        Span::new(format!("iteration {i}")).red().bold()
    )
    .unwrap();
    buf
}

fn owo_display(i: usize) -> String {
    use owo_colors::OwoColorize as _;
    let mut buf = String::with_capacity(64);
    write!(&mut buf, "{}", format!("iteration {i}").red().bold()).unwrap();
    buf
}

fn main() {
    println!("Rendering a styled string 3 attributes deep, {ITERATIONS} iterations:\n");

    let pc = bench("pretty-console  to_string()", ITERATIONS, |i| {
        std::hint::black_box(pc_to_string(i));
    });
    let owo = bench("owo-colors      to_string()", ITERATIONS, |i| {
        std::hint::black_box(owo_to_string(i));
    });
    let yan = bench("yansi           to_string()", ITERATIONS, |i| {
        std::hint::black_box(yansi_to_string(i));
    });
    let col = bench("colored         to_string()", ITERATIONS, |i| {
        std::hint::black_box(colored_to_string(i));
    });

    println!("\nAllocation-free Display path (write! into a fresh buffer):\n");

    let pc_fmt = bench("pretty-console  write!()", ITERATIONS, |i| {
        std::hint::black_box(pc_display(i));
    });
    let owo_fmt = bench("owo-colors      write!()", ITERATIONS, |i| {
        std::hint::black_box(owo_display(i));
    });

    println!("\nANSI stripping (strip_ansi), 100k iterations:\n");

    let pc_strip = bench("pretty-console  strip_ansi", 100_000, |_| {
        std::hint::black_box(pretty_console::strip_ansi(STRIP_INPUT));
    });

    println!("\nSummary (pretty-console throughput vs competitor):\n");
    println!(
        "  to_string   owo-colors {:>5.2}x   yansi {:>5.2}x   colored {:>5.2}x",
        pc / owo,
        pc / yan,
        pc / col
    );
    println!("  write!      owo-colors {:>5.2}x", pc_fmt / owo_fmt);
    let bytes_per_sec = STRIP_INPUT.len() as f64 * pc_strip;
    println!(
        "  strip_ansi  throughput {:>8.1} MB/s",
        bytes_per_sec / 1_048_576.0
    );
}
