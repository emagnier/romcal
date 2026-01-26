import type { SidebarsConfig } from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docsSidebar: [
    'intro',
    'getting-started',
    {
      type: 'category',
      label: 'Guides',
      items: ['guides/calendar', 'guides/masses', 'guides/entities', 'guides/dates', 'guides/locales'],
    },
    {
      type: 'category',
      label: 'CLI',
      link: {
        type: 'doc',
        id: 'cli/index',
      },
      items: ['cli/installation', 'cli/commands', 'cli/examples'],
    },
    {
      type: 'category',
      label: 'API Reference',
      link: {
        type: 'doc',
        id: 'api/index',
      },
      items: ['api/romcal', 'api/types', 'api/options'],
    },
    {
      type: 'category',
      label: 'Contributing',
      link: {
        type: 'doc',
        id: 'contributing/index',
      },
      items: [
        'contributing/definitions',
        'contributing/resources',
        'contributing/naming-conventions',
        'contributing/data-structure',
      ],
    },
    {
      type: 'category',
      label: 'Liturgical Reference',
      link: {
        type: 'doc',
        id: 'reference/index',
      },
      items: ['reference/girm', 'reference/gnly', 'reference/gilh'],
    },
    'glossary',
  ],
};

export default sidebars;
