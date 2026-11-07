#!/bin/sh
# Zero-warning gate for the documentation site.
# Fails on any warning line in the build/check/lint logs except the two
# third-party exceptions recorded in docs/decisions/0005-known-warnings.md.
set -e
cd "$(dirname "$0")/.."
mkdir -p .qa

pnpm build > .qa/build.log 2>&1 || { echo "BUILD FAILED"; tail -40 .qa/build.log; exit 1; }
pnpm check > .qa/check.log 2>&1 || { echo "CHECK FAILED"; tail -40 .qa/check.log; exit 1; }
pnpm lint  > .qa/lint.log  2>&1 || { echo "LINT FAILED";  tail -40 .qa/lint.log;  exit 1; }

# Narrow exclusions, one per recorded exception — blanket suppression is
# forbidden. Both come from Starlight's own code paths and are documented in
# docs/decisions/0005-known-warnings.md:
#   1. @astrojs/sitemap (injected by Starlight) warns without a `site` URL.
#   2. Starlight probes getEntry("docs", "404") for a custom 404 entry; the
#      probe miss is logged as a warning by Astro's content layer.
offenders() {
        grep -hiE 'warn' .qa/build.log .qa/check.log .qa/lint.log \
                | grep -v '\[@astrojs/sitemap\] The Sitemap integration requires' \
                | grep -v 'Entry docs → 404 was not found' \
                | grep -viE '^- 0 warnings?$' \
                | grep -iE 'warn' || true
}

# astro check's summary must also report zero on every line: "- N …" with N > 0.
if grep -qE '^- [1-9][0-9]* ' .qa/check.log; then
        echo "ZERO-WARNING GATE FAILED — astro check reported issues:"
        grep -E '^- [1-9][0-9]* ' .qa/check.log
        exit 1
fi

count=$(offenders | wc -l)
if [ "$count" -gt 0 ]; then
        echo "ZERO-WARNING GATE FAILED — $count offending line(s):"
        offenders
        exit 1
fi
echo "ZERO-WARNING GATE PASSED (build + check + lint)"
