# 0002 — Logo format, variants, favicon and accent color

## Source asset

- Canonical file: `docs/logo.png` from the repository (1264×1264 px, PNG, RGBA,
  sRGB). Copied unchanged to `src/assets/logo.png`; md5 before and after:
  `56c754c33fc47d48dd55304fafcfb3cb`. No re-encoding, no resizing, no
  optimization was applied to the copy.

## Variant analysis

The mark is a multicolor 3D illustration: a charcoal terminal window with a
white/purple prompt and a stack of rainbow waves (violet, blue, cyan, orange,
red). Skill §7.2 case 3 applies: recoloring would damage the mark, so the same
PNG serves both themes (`logo: { src: './src/assets/logo.png' }`).

Legibility check on a dark background (logo flattened onto `#1a1a1a`): the
bright waves, the white prompt glyphs and the shaded terminal edges keep the
silhouette readable; the charcoal body blends slightly but the mark remains
clearly visible. No CSS backdrop was needed. Verified again in the Phase-8
eyeball pass (see `docs/qa-report.md`).

## Favicon

`public/favicon.png` is derived from the original PNG (never from a variant):
resized 1264→512 with `sharp` lanczos3 and written with
`.png({ compressionLevel: 9, palette: false })` — no quality setting, no
quantization, no upscaling.

```js
sharp('src/assets/logo.png')
	.resize(512, 512, { kernel: 'lanczos3' })
	.png({ compressionLevel: 9, palette: false })
	.toFile('public/favicon.png');
```

## Accent color

Pixel sampling of the opaque, high-saturation pixels (sharp raw buffer, hue
histogram): the vivid pixels distribute over cyan (~18 966 px), violet at hue
270° (~13 424 px), orange (~12 960 px) and red (~10 346 px). The violet wave
and prompt glyph are the brand's signature — the README's flagship example
color is the violet `#7C4DFF` — so violet is the accent anchor.

Sampled violet average: **rgb(161, 83, 249)** (`#a153f9`). Its WCAG contrast is
4.10:1 on white — below the AA threshold for normal text — so the shipped
values are lightness-adjusted derivatives at the same hue:

| Token | Light | Dark | Verified contrast |
| --- | --- | --- | --- |
| `--sl-color-accent-low` | `#f3ecfe` | `#28203f` | (background tint) |
| `--sl-color-accent` | `#7c3aed` | `#c084fc` | 5.70:1 on white / 6.44:1 on `#1b1c20` |
| `--sl-color-accent-high` | `#5b21b6` | `#ddd6fe` | 7.57:1 on low / 11.05:1 on low |

All four text-bearing pairs pass WCAG AA (≥ 4.5:1). Computation script and
values are embedded in `src/styles/custom.css`.
