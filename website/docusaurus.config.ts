import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Romcal',
  tagline: 'Liturgical calendars of the Catholic Roman Rite',
  favicon: 'img/favicon.ico',

  future: {
    v4: true,
  },

  url: 'https://romcal.js.org',
  baseUrl: '/',

  organizationName: 'romcal',
  projectName: 'romcal',

  onBrokenLinks: 'throw',

  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          path: '../docs',
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/romcal/romcal/tree/main/docs/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/romcal-social-card.jpg',
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'Romcal',
      logo: {
        alt: 'Romcal Logo',
        src: 'img/logo.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'left',
          label: 'Docs',
        },
        {
          to: '/docs/cli',
          label: 'CLI',
          position: 'left',
        },
        {
          to: '/docs/api',
          label: 'API',
          position: 'left',
        },
        {
          to: '/editor',
          label: 'Editor',
          position: 'left',
        },
        {
          href: 'https://github.com/romcal/romcal',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'light',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Getting Started',
              to: '/docs/getting-started',
            },
            {
              label: 'CLI',
              to: '/docs/cli',
            },
            {
              label: 'API Reference',
              to: '/docs/api',
            },
          ],
        },
        {
          title: 'Packages',
          items: [
            {
              label: 'npm (TypeScript)',
              href: 'https://www.npmjs.com/package/romcal',
            },
            {
              label: 'PyPI (Python)',
              href: 'https://pypi.org/project/romcal/',
            },
            {
              label: 'crates.io (Rust)',
              href: 'https://crates.io/crates/romcal',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/romcal/romcal',
            },
            {
              label: 'Issues',
              href: 'https://github.com/romcal/romcal/issues',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Romcal. Apache 2.0 License.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['bash', 'json', 'toml', 'rust', 'python'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
