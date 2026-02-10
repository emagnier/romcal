// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
  site: 'https://romcal.js.org',
  integrations: [
    starlight({
      title: 'Romcal',
      logo: {
        src: './src/assets/logo.svg',
      },
      social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/romcal/romcal' }],
      editLink: {
        baseUrl: 'https://github.com/romcal/romcal/edit/main/docs/',
      },
      customCss: [
        '@fontsource/crimson-text/400.css',
        '@fontsource/crimson-text/600.css',
        '@fontsource/crimson-text/700.css',
        './src/styles/custom.css',
      ],
      components: {
        Footer: './src/components/Footer.astro',
      },
      sidebar: [
        { label: 'Getting Started', slug: 'getting-started' },
        {
          label: 'Guides',
          items: [
            { label: 'Generating Calendars', slug: 'guides/calendar' },
            { label: 'Mass-Centric Calendars', slug: 'guides/masses' },
            { label: 'Martyrology Entries', slug: 'guides/martyrology' },
            { label: 'Working with Dates', slug: 'guides/dates' },
            { label: 'Locales', slug: 'guides/locales' },
            { label: 'Data Loading', slug: 'guides/data-loading' },
            { label: 'Bundle Optimization', slug: 'guides/bundle-optimization' },
            { label: 'Bundler Plugin', slug: 'guides/bundler-plugin' },
            { label: 'Migration from v3', slug: 'guides/migration-v3' },
          ],
        },
        {
          label: 'CLI',
          items: [
            { label: 'Overview', slug: 'cli' },
            { label: 'Installation', slug: 'cli/installation' },
            { label: 'Commands', slug: 'cli/commands' },
            { label: 'Examples', slug: 'cli/examples' },
          ],
        },
        {
          label: 'API Reference',
          items: [
            { label: 'Overview', slug: 'api' },
            { label: 'Romcal Class', slug: 'api/romcal' },
            { label: 'Types', slug: 'api/types' },
            { label: 'Options', slug: 'api/options' },
          ],
        },
        {
          label: 'Contributing',
          items: [
            { label: 'Overview', slug: 'contributing' },
            { label: 'Calendar Definitions', slug: 'contributing/definitions' },
            { label: 'Entity Resources', slug: 'contributing/resources' },
            { label: 'Naming Conventions', slug: 'contributing/naming-conventions' },
            { label: 'Data Structure', slug: 'contributing/data-structure' },
          ],
        },
        {
          label: 'Liturgical Reference',
          items: [
            { label: 'Overview', slug: 'reference' },
            { label: 'GNLY — Liturgical Year and Calendar', slug: 'reference/gnly' },
            { label: 'CP — Particular Calendars', slug: 'reference/cp' },
            { label: 'GIRM — Roman Missal', slug: 'reference/girm' },
            { label: 'GILM — Lectionary for Mass', slug: 'reference/gilm' },
            { label: 'GILH — Liturgy of the Hours', slug: 'reference/gilh' },
            { label: 'PS — Easter Feasts', slug: 'reference/ps' },
          ],
        },
        { label: 'Glossary', slug: 'glossary' },
      ],
    }),
  ],
});
