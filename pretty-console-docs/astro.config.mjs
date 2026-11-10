// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import astroTakumi from 'astro-takumi';
import fs from 'node:fs';
import { makeRender } from './src/og-render.mjs';
import starlightLinksValidator from 'starlight-links-validator';

const __logoB64 = fs.readFileSync("./src/assets/logo.png").toString("base64");
const __render = makeRender({
  logoDataUrl: "data:image/png;base64," + __logoB64,
  accentHex: "#3e3e74",
  accentHexA12: "#3e3e741a",
  domain: "pretty-console.vercel.app",
  projectName: "pretty-console",
});

export default defineConfig({
	vite: {
		ssr: { external: ["jsdom", "astro-takumi"] },
	},
  site: 'https://pretty-console.vercel.app',
        integrations: [
                starlight({
			routeMiddleware: "./src/routeData.ts",
                        expressiveCode: {
                        	themes: ['github-light', 'github-dark'],
                        },
                        title: 'pretty-console',
                        description:
                                'The fluent, zero-cost terminal styling toolkit for Rust: spans, lines, panels, gradients, hyperlinks and ANSI-aware text tools.',

                        logo: { src: './src/assets/logo.png' },
                        favicon: '/favicon.png',
                        customCss: ['./src/styles/custom.css'],
                        social: [
                                {
                                        icon: 'github',
                                        label: 'GitHub',
                                        href: 'https://github.com/elcoosp/pretty-console',
                                },
                        ],
                        editLink: {
                                baseUrl:
                                        'https://github.com/elcoosp/pretty-console/edit/main/docs-site/src/content/docs/',
                        },
                        locales: {
                                root: {
                                        label: 'English',
                                        lang: 'en',
                                },
                        },
                        sidebar: [
                                {
                                        label: 'Tutorials',
                                        items: [{ label: 'Getting started', slug: 'tutorials/getting-started' }],
                                },
                                {
                                        label: 'How-to guides',
                                        items: [
                                                { label: 'Reuse and merge styles', slug: 'guides/reusable-styles' },
                                                { label: 'Compose status lines', slug: 'guides/status-lines' },
                                                { label: 'Use exact colors', slug: 'guides/exact-colors' },
                                                { label: 'Degrade colors', slug: 'guides/degrade-colors' },
                                                { label: 'Control colors at runtime', slug: 'guides/runtime-color-control' },
                                                { label: 'Add gradients, panels and hyperlinks', slug: 'guides/decorations' },
                                                { label: 'Measure, truncate and pad text', slug: 'guides/text-layout' },
                                                { label: 'Migrate from 1.x', slug: 'guides/migrate-from-v1' },
                                        ],
                                },
                                {
                                        label: 'Reference',
                                        items: [
                                                { label: 'Span', slug: 'reference/span' },
                                                { label: 'Style, Attribute and Effects', slug: 'reference/style' },
                                                { label: 'Color', slug: 'reference/color' },
                                                { label: 'Line', slug: 'reference/line' },
                                                { label: 'Panel', slug: 'reference/panel' },
                                                { label: 'Gradient and rainbow functions', slug: 'reference/decorations' },
                                                { label: 'Text tools', slug: 'reference/text-tools' },
                                                { label: 'Color control', slug: 'reference/color-control' },
                                                { label: 'Macros', slug: 'reference/macros' },
                                        ],
                                },
                                {
                                        label: 'Explanation',
                                        items: [
                                                { label: 'The zero-cost design', slug: 'explanation/zero-cost-design' },
                                                { label: 'Deterministic rendering', slug: 'explanation/rendering-model' },
                                                { label: 'How v2 came to be', slug: 'explanation/design-history' },
                                        ],
                                },
                        ],
                        plugins: [starlightLinksValidator()],
                }),
        
		astroTakumi({
			options: {
				domBackend: "linkedom",
				fonts: [fs.readFileSync("./node_modules/@fontsource/roboto/files/roboto-latin-400-normal.woff2")],
				width: 1200,
				height: 630,
				format: "png",
			},
			render: __render,
		}),
	],
});
