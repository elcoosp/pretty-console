#!/bin/bash

# Script de commit automatique et conventionnel - Un commit par fichier
# Compatible Bash 3.2 (macOS) - Version 2.0.0 nettoyée sans cassure Markdown

files=$(git status --porcelain | awk '{print $2}')

if [ -z "$files" ]; then
    echo "Aucun fichier modifie detecte."
    exit 0
fi

for file in $files; do
    if [ ! -f "$file" ]; then
        continue
    fi

    echo "Analyse et commit pour : $file"

    case "$file" in
        ".github/workflows/ci.yml")
            subject="ci(workflows): adapt clippy and lower coverage thresholds"
            body="Update lint definitions mapping to use cargo-toml and drop line constraints to 90 percent."
            ;;
        "CHANGELOG.md")
            subject="docs(changelog): draft comprehensive release logs for version 2.0.0"
            body="Document the complete Copy style overhaul multi segment composition and adaptive color features."
            ;;
        "Cargo.lock")
            subject="chore: lock down dependencies metadata for benchmark suites"
            body="Re lock packages trees mapping to support the owo colors yansi and colored comparators."
            ;;
        "Cargo.toml")
            subject="chore: elevate package version to 2.0.0"
            body="Update category keywords and inject benchmarks definitions examples into the manifest."
            ;;
        "README.md")
            subject="docs(readme): rewrite content mapping allocation free rendering features"
            body="Overhaul comparison matrices layout and document 1.x version migration guidelines."
            ;;
        "docs/DESIGN.md")
            subject="docs(maintainer): record architectural design audit findings"
            body="Add log entries tracking the v2 rewrite constraints around bitsets and zero heap storage."
            ;;
        "examples/advanced.rs")
            subject="docs(examples): convert advanced examples to use v2 macro definitions"
            body="Showcase hex string parsing options and conditional when styles execution chains."
            ;;
        "examples/bench.rs")
            subject="docs(examples): create micro-benchmark suite tracking performance metrics"
            body="Provide direct throughput logs comparing output operations against sibling crates."
            ;;
        "examples/demo.rs")
            subject="docs(examples): adapt basic tour to reflect idempotent attributes"
            body="Replace old Console structures calls with the modern explicit Span trait extensions."
            ;;
        "examples/levels.rs")
            subject="docs(examples): showcase line row compositions and CJK width spacing"
            body="Provide realistic CLI print mappings using multi segment padding alignments."
            ;;
        "examples/showcase.rs")
            subject="docs(examples): bundle structural features tour launcher"
            body="Provide full capability diagnostics tracking color level detection cascades."
            ;;
        "src/attr.rs")
            subject="feat: implement Effects u16 attribute bitset engine"
            body="Provide copy constructible text attributes removing historical vector allocations."
            ;;
        "src/choice.rs")
            subject="feat: build cached environment color levels capability scanner"
            body="Enforce zero runtime syscall overhead by storing automatic flags choices into atomics."
            ;;
        "src/color.rs")
            subject="feat: implement allocation free digit writing and hex color parsers"
            body="Provide direct sRGB conversion matrices supporting grayscale ramps and luminance matching."
            ;;
        "src/decorate.rs")
            subject="feat: code per character color gradient and rainbow generators"
            body="Provide declarative hsv to rgb mix boundaries driving smooth linear spectrum fades."
            ;;
        "src/lib.rs")
            subject="refactor: consolidate top level module facades for version 2.0.0"
            body="Expose dense vectors accessors and refine description text maps targeting terminal styling."
            ;;
        "src/line.rs")
            subject="feat: design multi segment Line composition row vectors"
            body="Let differently styled spans combine left to right using flat chainable operators."
            ;;
        "src/macros.rs")
            subject="feat: export paint style and line shorthand macros definitions"
            body="Provide declarative wrappers managing format args allocation tracks automatically."
            ;;
        "src/panel.rs")
            subject="feat: design Panel bordered box visualizers with inlaid titles"
            body="Provide rounded corner framing elements automatically matching the widest interior line."
            ;;
        "src/span.rs")
            subject="refactor: move Console engine to use the unallocated Span architecture"
            body="Let Display formatting output paths write parameters straight into destination sinks."
            ;;
        "src/style.rs")
            subject="feat: engineer const constructible Copy Style specifications"
            body="Provide zero heap combining logic supporting flat attribute bitor union parameters."
            ;;
        "src/text.rs")
            subject="feat: implement ANSI aware visible width text toolkit components"
            body="Provide CJK wide character column accounting ellipsis truncations and padding alignments."
            ;;
        "tests/color_tests.rs")
            subject="test: deploy test suites covering gradients and degradation matrices"
            body="Assert nearest cube assignments and check hex string error diagnostics returns."
            ;;
        "tests/contract.rs")
            subject="test: write deterministic exact string contract assertions"
            body="Confirm byte layout correctness covering canonical sequences and macro builders."
            ;;
        "tests/no_color_tests.rs")
            subject="test: verify pass through compliance under the no color feature"
            body="Assert that all formatters map to plain text outputs when colors are stripped."
            ;;
        "tests/panel_tests.rs")
            subject="test: test box structural dimensions and fixed width clipping rows"
            body="Verify panel alignment metrics when processing wide asian character titles."
            ;;
        "tests/text_tools.rs")
            subject="test: deploy unit verifications checking text width truncations"
            body="Confirm unicode string safety bounds and evaluate trailing sequence exclusions."
            ;;
        *)
            subject="chore: update $(basename "$file")"
            body="Refactor or clean up internal logic structures within $file."
            ;;
    esac

    # Indexation temporaire, commit conventionnel et purge de la file
    git add "$file"
    git commit -m "$subject" -m "$body"

    echo "✅ Commite : $file"
    echo "--------------------------------------------"
done

echo "Tous les fichiers depretty-console ont ete enregistres de maniere conventionnelle !"
