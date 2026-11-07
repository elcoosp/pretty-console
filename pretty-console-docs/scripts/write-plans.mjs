// Writes docs/page-plans/*.md — one binding plan per documentation page.
// Plans are authored from the Phase-1 source inventory of elcoosp/pretty-console.
import { writeFile, mkdir } from 'node:fs/promises';

const plans = {
        'index': `---
slug: /
type: (none — splash landing page, not a Diátaxis type)
audience: every visitor arriving at the site root
goal: route the reader into the correct Diátaxis quadrant within one screen
scope:
  in: hero with tagline and primary actions, four quadrant LinkCards, crates.io/docs.rs pointers
  out: any teaching, any API detail, any install steps (see tutorials/getting-started)
outline:
  - Splash hero (title, tagline, actions)
  - Four-quadrant CardGrid
  - Where to start pointers
source_refs:
  - README.md (tagline and highlight vocabulary)
approved_by: self (autonomous pipeline)
`,

        'tutorials/getting-started': `---
slug: tutorials/getting-started
type: tutorial
audience: Rust developers who have never used pretty-console
goal: install the crate in a fresh project and print styled spans and one multi-segment line, seeing visible results at each step
scope:
  in: cargo new, cargo add pretty-console, Span builders, Styled literals, Line composition, println, cargo run, expected output
  out: reusable Style templates (guides/reusable-styles), runtime color control (guides/runtime-color-control), why rendering is deterministic (explanation/rendering-model)
outline:
  - What you will build
  - Prerequisites
  - Create a project
  - Add the crate
  - Style a single span
  - Style a literal in place
  - Compose a multi-segment line
  - Run and confirm the result
  - Next steps
source_refs:
  - Cargo.toml (crate name, version, examples)
  - README.md (quick start)
  - src/span.rs (Span::new, println, Styled)
  - src/line.rs (Line::new, push)
  - examples/demo.rs (vocabulary)
approved_by: self (autonomous pipeline)
`,

        'guides/reusable-styles': `---
slug: guides/reusable-styles
type: how-to
audience: CLI authors who repeat the same styling in many places
goal: define a style once, apply it to many strings, and merge styles correctly
scope:
  in: Style::new in const context, Style::paint, Span::with_text, merge precedence, From<Color>/From<Attribute>/From<Effects>, Style::when
  out: multi-segment lines (guides/status-lines), full member tables (reference/style), design rationale (explanation/zero-cost-design)
outline:
  - Prerequisites
  - Define a style once
  - Apply it to different text
  - Merge two styles
  - Convert colors and attributes into styles
  - Adjust conditionally with when
source_refs:
  - src/style.rs (Style, merge, paint, when, From impls)
  - src/span.rs (with_text)
  - examples/advanced.rs (predefined styles, merging)
approved_by: self (autonomous pipeline)
`,

        'guides/status-lines': `---
slug: guides/status-lines
type: how-to
audience: CLI authors printing progress and summary rows
goal: compose one terminal row from differently styled segments without string concatenation
scope:
  in: Line::new/push/extend/from_spans, the last-span styling rule, push_mut in loops, with_style, visible_width, format-width alignment
  out: macro one-liners detail (reference/macros), Line member tables (reference/line)
outline:
  - Prerequisites
  - Push segments left to right
  - Understand the last-span rule
  - Build a line from an iterator
  - Restyle every span at once
  - Accumulate in a loop
  - Align a whole line with format widths
source_refs:
  - src/line.rs (Line, edit_last, with_style, Display)
  - src/span.rs (fmt_with visible-width padding)
  - examples/showcase.rs
approved_by: self (autonomous pipeline)
`,

        'guides/exact-colors': `---
slug: guides/exact-colors
type: how-to
audience: developers matching a design system or parsing colors from configuration
goal: apply exact RGB colors from packed values or hex strings and pick readable foregrounds
scope:
  in: Color::hex, fg_hex/bg_hex, Color::from_hex_str, FromStr, ParseColorError handling, Color::indexed, Color::rgb, readable_on, luminance
  out: degradation to 256/16 colors (guides/degrade-colors), full Color tables (reference/color)
outline:
  - Prerequisites
  - Use a packed hex value
  - Parse a hex string from config or CLI
  - Use 256-palette entries
  - Pick a readable foreground for a background
source_refs:
  - src/color.rs (hex, from_hex_str, ParseColorError, readable_on, luminance)
  - examples/advanced.rs (hex colors, theme accent parsing)
approved_by: self (autonomous pipeline)
`,

        'guides/degrade-colors': `---
slug: guides/degrade-colors
type: how-to
audience: developers whose output must survive on 256-color or 16-color terminals (CI logs, ssh, IDE consoles)
goal: detect the terminal capability level and render degraded colors accordingly
scope:
  in: ColorLevel, supports_color_level, Span::render_with, Line::render_with, Color::degrade_to/to_256/to_16
  out: turning colors fully off (guides/runtime-color-control), degradation math (reference/color)
outline:
  - Prerequisites
  - Detect the terminal capability
  - Render a span or line at a level
  - Degrade a single color for a palette swatch
  - Produce stable output for CI logs
source_refs:
  - src/choice.rs (supports_color_level, ColorLevel)
  - src/color.rs (degrade_to, to_256, to_16)
  - src/span.rs (render_with)
  - src/line.rs (render_with)
  - README.md (adaptive colors section)
approved_by: self (autonomous pipeline)
`,

        'guides/runtime-color-control': `---
slug: guides/runtime-color-control
type: how-to
audience: CLI authors wiring a --color flag or respecting no-color conventions
goal: control whether printed output carries colors at runtime, honoring the standard environment variables
scope:
  in: ColorChoice, set_color_choice, color_choice, is_color_enabled, env precedence table, stderr and try_* printers, broken-pipe handling
  out: capability detection and degradation (guides/degrade-colors), detection internals (reference/color-control), determinism rationale (explanation/rendering-model)
outline:
  - Prerequisites
  - Default behavior
  - The environment variables pretty-console honors
  - Wire a --color flag
  - Print to stderr
  - Handle broken pipes with try_*
source_refs:
  - src/choice.rs (ColorChoice, set_color_choice, is_color_enabled, env_enabled)
  - src/span.rs (print family, write_honoring_choice)
  - README.md (runtime color control tables)
approved_by: self (autonomous pipeline)
`,

        'guides/decorations': `---
slug: guides/decorations
type: how-to
audience: CLI authors building banners, report boxes or clickable output
goal: produce gradient and rainbow text, rounded panels with titles, and OSC 8 hyperlinks
scope:
  in: gradient/gradient_bg, rainbow/rainbow_bg, Panel builder (title, line/lines, width, padding, border), fixed-width truncation, Span::link/unlink/url
  out: function signatures (reference/decorations, reference/panel), span member tables (reference/span)
outline:
  - Prerequisites
  - Fade a foreground gradient across text
  - Paint a rainbow
  - Build a titled panel
  - Fix the panel width (and what happens to wide lines)
  - Make text clickable with OSC 8
source_refs:
  - src/decorate.rs (gradient, gradient_bg, rainbow, rainbow_bg)
  - src/panel.rs (Panel builder, render, content_width)
  - src/span.rs (link, unlink, url)
  - examples/showcase.rs, examples/advanced.rs
approved_by: self (autonomous pipeline)
`,

        'guides/text-layout': `---
slug: guides/text-layout
type: how-to
audience: TUI and table authors aligning columns in a terminal
goal: measure, truncate and pad strings exactly as the terminal renders them, including styled and CJK strings
scope:
  in: visible_width, char_width, truncate with tail and reset semantics, pad with Align, format-width padding on Span/Line
  out: escape-scanner grammar (reference/text-tools), width-table approximation limits (reference/text-tools)
outline:
  - Prerequisites
  - Measure visible width correctly
  - Truncate to a column budget
  - Pad and align columns
  - Rely on format-width padding for styled values
source_refs:
  - src/text.rs (visible_width, char_width, truncate, pad, Align)
  - src/span.rs (fmt_with, Display)
  - src/line.rs (fmt_with, Display)
approved_by: self (autonomous pipeline)
`,

        'guides/migrate-from-v1': `---
slug: guides/migrate-from-v1
type: how-to
audience: maintainers upgrading a codebase from pretty-console 1.x
goal: move to 2.0 knowing exactly what breaks and what keeps compiling
scope:
  in: Console alias status, canonical attribute ordering and dedup, Style::attributes() returning Effects, new_with_style spelling, contract tests replacing insta
  out: full v2 API (reference/*), the reasoning behind the redesign (explanation/design-history)
outline:
  - Prerequisites
  - Bump the dependency
  - Keep or rename Console
  - Fix exact-output tests
  - Update attributes() consumers
  - Adopt the Span-native spellings
source_refs:
  - CHANGELOG.md (2.0.0 breaking changes)
  - README.md (migrating from 1.x)
  - src/span.rs (Console alias, canonical ordering)
  - docs/DESIGN.md (2.4 compatibility)
approved_by: self (autonomous pipeline)
`,

        'reference/span': `---
slug: reference/span
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up any Span, Styled or Console member with signature, semantics and defaults
scope:
  in: Span struct and every public method (constructors, text/style accessors, style builders, level helpers, banner, link API, render family, print family), Styled trait surface, Console alias, trait impls and derives
  out: Style/Effects internals (reference/style), tutorials, why-methods (explanation/*)
outline:
  - Overview
  - Constructors
  - Text and style accessors
  - Style builder methods (tables)
  - Level helpers
  - Banner
  - Hyperlinks
  - Rendering
  - Printing
  - Styled extension trait
  - Derives and trait impls
source_refs:
  - src/span.rs (primary)
  - src/lib.rs (re-exports)
approved_by: self (autonomous pipeline)
`,

        'reference/style': `---
slug: reference/style
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up Style, Attribute, Effects and EffectIter members and their exact semantics
scope:
  in: Style methods and merge rule, Attribute variants with SGR codes, Effects bitset operations, EffectIter, From impls
  out: Span methods (reference/span), const-context rationale (explanation/zero-cost-design)
outline:
  - Overview
  - Style methods
  - Merge semantics
  - Attribute variants and SGR codes
  - Effects operations
  - EffectIter
source_refs:
  - src/style.rs (primary)
  - src/attr.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'reference/color': `---
slug: reference/color
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the Color model, parsing, color math and SGR emission
scope:
  in: Color enum and 16 constants, indexed/rgb/hex constructors, from_hex_str and ParseColorError, fg_code/bg_code, rgb_value, luminance, readable_on, lerp, to_256, to_16, degrade_to, level, palette_rgb, From/FromStr impls
  out: capability detection (reference/color-control), degradation guidance (guides/degrade-colors)
outline:
  - Overview
  - Constants and constructors
  - Parsing hex strings
  - SGR parameter accessors
  - Color math (luminance, readable_on, lerp)
  - Degradation (to_256, to_16, degrade_to, level)
  - palette_rgb
source_refs:
  - src/color.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'reference/line': `---
slug: reference/line
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up Line members and the exact last-span styling semantics
scope:
  in: Line struct and all methods (construction, push family, accessors, with_style, map_span, when, last-span builders, render family, print family), trait impls
  out: Span internals (reference/span), composition guidance (guides/status-lines)
outline:
  - Overview
  - Construction
  - Pushing and editing spans
  - Whole-line operations
  - Rendering and printing
  - Derives and trait impls
source_refs:
  - src/line.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'reference/panel': `---
slug: reference/panel
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up Panel members, defaults and the width computation
scope:
  in: Panel builder methods and defaults, content_width formula, render/render_plain/write_to/print/println, Display
  out: styling advice (guides/decorations)
outline:
  - Overview
  - Builder methods and defaults
  - Width computation
  - Rendering
source_refs:
  - src/panel.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'reference/decorations': `---
slug: reference/decorations
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the gradient and rainbow function signatures and their per-character behavior
scope:
  in: gradient, gradient_bg, rainbow, rainbow_bg, hsv_to_rgb
  out: usage recipes (guides/decorations)
outline:
  - Overview
  - Function signatures
  - Interpolation details
  - hsv_to_rgb
source_refs:
  - src/decorate.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'reference/text-tools': `---
slug: reference/text-tools
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the ANSI-aware text functions, their parameters and edge-case behavior
scope:
  in: strip_ansi (sequence grammar), char_width (range table summary), visible_width, truncate (tail/reset semantics), pad, Align
  out: recipes (guides/text-layout)
outline:
  - Overview
  - strip_ansi
  - char_width
  - visible_width
  - truncate
  - pad and Align
source_refs:
  - src/text.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'reference/color-control': `---
slug: reference/color-control
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the runtime color control surface and the exact environment-variable precedence
scope:
  in: ColorChoice, ColorLevel, set_color_choice, color_choice, is_color_enabled (cache), supports_color_level (detection order), environment variables
  out: flag-wiring recipe (guides/runtime-color-control), degradation recipe (guides/degrade-colors)
outline:
  - Overview
  - ColorChoice
  - Setting and reading the choice
  - is_color_enabled and the cache
  - supports_color_level detection order
  - Environment variable tables
source_refs:
  - src/choice.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'reference/macros': `---
slug: reference/macros
type: reference
audience: developers embedding pretty-console in a Rust application
goal: look up the paint!, style! and line! macro syntax and their constraints
scope:
  in: paint!, style!, line! invocation forms, accepted method names, argument-taking exclusions
  out: builder method tables (reference/span, reference/style)
outline:
  - Overview
  - paint!
  - style!
  - line!
  - Constraints
source_refs:
  - src/macros.rs (primary)
approved_by: self (autonomous pipeline)
`,

        'explanation/zero-cost-design': `---
slug: explanation/zero-cost-design
type: explanation
audience: Rust developers evaluating the crate or curious about its performance model
goal: understand how the crate renders styled output without heap allocation or repeated syscalls
scope:
  in: Copy styles and const construction, Effects bitset, direct fmt::Write emission, hand-rolled digit writing, cached environment detection, single-pass visible width
  out: how to use the API (tutorials, guides), measured benchmark tables (README)
outline:
  - What zero-cost means here
  - Styles are values, not builders with state
  - The Effects bitset
  - Writing codes straight into the formatter
  - One environment decision, cached
  - Visible width in a single pass
  - What this does not cover
source_refs:
  - docs/DESIGN.md (perf plan)
  - src/style.rs, src/attr.rs, src/color.rs (write_u8), src/choice.rs, src/text.rs
approved_by: self (autonomous pipeline)
`,

        'explanation/rendering-model': `---
slug: explanation/rendering-model
type: explanation
audience: developers who need to predict what their output will look like in logs and tests
goal: understand why string rendering is deterministic while printing honors the environment, and how ordering and idempotence follow
scope:
  in: render/Display path vs print path, ColorChoice scope, plain-style rendering, canonical SGR ordering, idempotent attributes, no-color feature behavior
  out: print-family signatures (reference/span)
outline:
  - Two paths, two contracts
  - What determinism buys you
  - The print family's environment contract
  - Canonical ordering and idempotence
  - Plain styles emit nothing
  - The no-color feature
source_refs:
  - src/lib.rs (rendering determinism, style ordering docs)
  - src/span.rs (write_body, write_honoring_choice)
  - src/style.rs (write_start, is_plain)
  - README.md (rendering note)
approved_by: self (autonomous pipeline)
`,

        'explanation/design-history': `---
slug: explanation/design-history
type: explanation
audience: contributors and evaluators comparing terminal-styling crates
goal: understand the problems v1 had, the decisions that shaped v2, and what the crate deliberately does not do
scope:
  in: v1 perf findings P1-P6, DX gaps D1-D14 (subset), core model decisions, compatibility strategy, non-goals
  out: migration mechanics (guides/migrate-from-v1), feature usage (guides/*)
outline:
  - Where this record comes from
  - What v1 cost
  - What v1 lacked
  - The core model decisions
  - Compatibility strategy
  - Non-goals
source_refs:
  - docs/DESIGN.md (primary — the project's own decision record)
  - CHANGELOG.md (2.0.0)
approved_by: self (autonomous pipeline)
`,
};

await mkdir('docs/page-plans', { recursive: true });
for (const [name, content] of Object.entries(plans)) {
        const file = `docs/page-plans/${name}.md`;
        await mkdir(file.slice(0, file.lastIndexOf('/')), { recursive: true });
        await writeFile(file, content.endsWith('\n') ? content : content + '\n');
}
console.log(`wrote ${Object.keys(plans).length} page plans`);
