import { defineDocsConfig } from '@elcoosp/docs-toolkit/config';
import starlightLinksValidator from 'starlight-links-validator';


export default await defineDocsConfig({
  changelog: { repo: 'elcoosp/pretty-console' },
  site: 'https://pretty-console.vercel.app',
  domain: 'pretty-console.vercel.app',
  title: "pretty-console",
  description: "The fluent, zero-cost terminal styling toolkit for Rust: spans, lines, panels, gradients, hyperlinks and ANSI-aware text tools.",
  sidebar: [],
  extraStarlight: {
                        expressiveCode: {
                        	themes: ['github-light', 'github-dark']
                        },
                        title: 'pretty-console',
                        description:
                                'The fluent, zero-cost terminal styling toolkit for Rust: spans, lines, panels, gradients, hyperlinks and ANSI-aware text tools.',
                        favicon: '/favicon.png',
                        social: [
                                {
                                        icon: 'github',
                                        label: 'GitHub',
                                        href: 'https://github.com/elcoosp/pretty-console'
                                },
                        ],
                        editLink: {
                                baseUrl:
                                        'https://github.com/elcoosp/pretty-console/edit/main/docs-site/src/content/docs/'
                        },
                        locales: {
                                root: {
                                        label: 'English',
                                        lang: 'en'
                                }
                        },
                        sidebar: [
                                {
                                        label: 'Tutorials',
                                        items: [{ label: 'Getting started', slug: 'tutorials/getting-started' }]
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
                                        ]
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
                                        ]
                                },
                                {
                                        label: 'Explanation',
                                        items: [
                                                { label: 'The zero-cost design', slug: 'explanation/zero-cost-design' },
                                                { label: 'Deterministic rendering', slug: 'explanation/rendering-model' },
                                                { label: 'How v2 came to be', slug: 'explanation/design-history' },
                                        ]
                                },
                        ],
                        plugins: [starlightLinksValidator()]
                },
});
