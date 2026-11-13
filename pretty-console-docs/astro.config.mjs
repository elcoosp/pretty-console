import { defineDocsConfig } from '@elcoosp/docs-toolkit/config';

export default await defineDocsConfig({
  llmsTxt: {},
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
                                        label: 'GitHub', translations: { fr: "GitHub" },
                                        href: 'https://github.com/elcoosp/pretty-console'
                                },
                        ],
                        editLink: {
                                baseUrl:
                                        'https://github.com/elcoosp/pretty-console/edit/main/docs-site/src/content/docs/'
                        },

                        sidebar: [
                                {
                                        label: 'Tutorials',
                                        items: [{ label: 'Getting started', slug: 'tutorials/getting-started' }], translations: { fr: "Commencer" }
                                },
                                {
                                        label: 'How-to guides',
                                        items: [
                                                { label: 'Reuse and merge styles', translations: { fr: "Réutiliser et fusionner des styles" }, slug: 'guides/reusable-styles' },
                                                { label: 'Compose status lines', translations: { fr: "Composer des lignes d'état" }, slug: 'guides/status-lines' },
                                                { label: 'Use exact colors', translations: { fr: "Utiliser des couleurs exactes" }, slug: 'guides/exact-colors' },
                                                { label: 'Degrade colors', translations: { fr: "Dégrader les couleurs" }, slug: 'guides/degrade-colors' },
                                                { label: 'Control colors at runtime', translations: { fr: "Contrôler les couleurs à l'exécution" }, slug: 'guides/runtime-color-control' },
                                                { label: 'Add gradients, panels and hyperlinks', translations: { fr: "Ajouter des dégradés, des panneaux et des hyperliens" }, slug: 'guides/decorations' },
                                                { label: 'Measure, truncate and pad text', translations: { fr: "Mesurer, tronquer et remplir le texte" }, slug: 'guides/text-layout' },
                                                { label: 'Migrate from 1.x', translations: { fr: "Migrer depuis la 1.x" }, slug: 'guides/migrate-from-v1' },
                                        ]
                                },
                                {
                                        label: 'Reference',
                                        items: [
                                                { label: 'Span', translations: { fr: "Span" }, slug: 'reference/span' },
                                                { label: 'Style, Attribute and Effects', translations: { fr: "Style, attribut et effets" }, slug: 'reference/style' },
                                                { label: 'Color', translations: { fr: "Couleur" }, slug: 'reference/color' },
                                                { label: 'Line', translations: { fr: "Ligne" }, slug: 'reference/line' },
                                                { label: 'Panel', translations: { fr: "Panneau" }, slug: 'reference/panel' },
                                                { label: 'Gradient and rainbow functions', translations: { fr: "Fonctions de dégradé et d'arc-en-ciel" }, slug: 'reference/decorations' },
                                                { label: 'Text tools', translations: { fr: "Outils de texte" }, slug: 'reference/text-tools' },
                                                { label: 'Color control', translations: { fr: "Contrôle des couleurs" }, slug: 'reference/color-control' },
                                                { label: 'Macros', slug: 'reference/macros' },
                                        ]
                                },
                                {
                                        label: 'Explanation',
                                        items: [
                                                { label: 'The zero-cost design', translations: { fr: "La conception à coût zéro" }, slug: 'explanation/zero-cost-design' },
                                                { label: 'Deterministic rendering', translations: { fr: "Rendu déterministe" }, slug: 'explanation/rendering-model' },
                                                { label: 'How v2 came to be', translations: { fr: "Comment la v2 a vu le jour" }, slug: 'explanation/design-history' },
                                        ]
                                },
                        ],
                },
});
