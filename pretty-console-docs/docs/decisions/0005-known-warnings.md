# 0005 — Known warnings

The zero-warning gate (`scripts/warning-gate.sh`) greps the captured build,
check and lint logs for warning lines. Two warning lines survive, both from
Starlight's own code paths. Each is recorded here with the narrow, commented
exclusion required by the documentation standard.

## Permitted exception 1 — sitemap without a site URL

```
[WARN] [@astrojs/sitemap] The Sitemap integration requires the `site` astro.config option. Skipping.
```

**Source.** Starlight unconditionally registers `@astrojs/sitemap` (its own
dependency, v3.7.4) unless the user already added that integration; see
`node_modules/@astrojs/starlight/dist/index.js`:
`if (!allIntegrations.find(({ name }) => name === "@astrojs/sitemap")) integrations.push(starlightSitemap(starlightConfig))`.

**Why it fires.** `@astrojs/sitemap` warns and skips when the Astro `site`
option is unset.

**Why it is not fixed.** The documentation standard forbids adding `site`
unless the deployment target is known; this site ships as a portable
codebase with no deployment URL recorded. Inventing a canonical URL would
produce a wrong sitemap — skipping the feature is the correct behavior until
the site is deployed.

## Permitted exception 2 — Starlight's custom-404 probe

```
[WARN] [content] Entry docs → 404 was not found.
```

**Source.** Starlight's route data calls `await getEntry("docs", "404")`
(`node_modules/@astrojs/starlight/dist/utils/routing/data.js`, `get404Route`)
to look for a user-provided 404 page, falling back to a built-in entry. The
Astro content layer logs a warning whenever that probe misses.

**Why it is not fixed.** Providing a content entry to silence the probe
creates strictly worse outcomes:

- a `draft: true` entry is excluded from routes but Starlight renders its
  "This content is a draft…" notice on the shipped 404 page;
- a non-draft entry publishes a duplicate normal route at `/404/`.

The built-in fallback 404 page is clean (splash template, localized tagline,
no draft notice), so the site ships it and the probe warning is recorded.
The `i18n` half of the same content-layer warnings **was** properly fixed by
defining the `i18n` collection (`src/content.config.ts`) and shipping
`src/content/i18n/en.json`.

## Suppression

`scripts/warning-gate.sh` excludes exactly these two strings. Any other
warning line fails the gate.

## Resolved during this build (no longer firing)

- `[WARN] [content] The collection "i18n" does not exist or is empty.` —
  fixed by defining the `i18n` collection with Starlight's `i18nLoader()` +
  `i18nSchema()` and shipping `src/content/i18n/en.json`.
- `@astrojs/check` import failure — `@napi-rs/wasm-runtime@1.2.4` paired with
  `@emnapi/runtime@2.0.0-beta.2` crashed on import in this environment;
  fixed with pnpm overrides pinning `@napi-rs/wasm-runtime@1.2.7` and
  `@emnapi/{runtime,core}@1.11.3` (see `pnpm-workspace.yaml`).
