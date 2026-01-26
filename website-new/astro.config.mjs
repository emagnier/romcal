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
      customCss: ['./src/styles/custom.css'],
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
            { label: 'Liturgical Entities', slug: 'guides/entities' },
            { label: 'Working with Dates', slug: 'guides/dates' },
            { label: 'Locales', slug: 'guides/locales' },
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
            { label: 'GIRM — General Instruction of the Roman Missal', slug: 'reference/girm' },
            { label: 'GNLY — Universal Norms on the Liturgical Year and the Calendar', slug: 'reference/gnly' },
            { label: 'GILH — General Instruction of the Liturgy of the Hours', slug: 'reference/gilh' },
          ],
        },
        { label: 'Glossary', slug: 'glossary' },
      ],
    }),
  ],
});
