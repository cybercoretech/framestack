import type {Config} from '@docusaurus/types';

const cybercoreChooserCss =
  'https://cybercoretech.github.io/kit/chooser.css';
const cybercoreChooserModule =
  'https://cybercoretech.github.io/kit/chooser.js';

const config: Config = {
  title: 'FrameStack',
  tagline: 'Compose full-stack systems across frameworks and languages',
  url: 'https://cybercoretech.github.io',
  baseUrl: '/framestack/',
  organizationName: 'cybercoretech',
  projectName: 'framestack',
  deploymentBranch: 'gh-pages',
  trailingSlash: false,

  onBrokenLinks: 'throw',
  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
  },

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  // window.SITE must exist before the shared chooser module executes.
  // The chooser also writes its palette ID to data-theme. Preserve that ID
  // separately and keep Docusaurus's own dark-mode selector stable.
  headTags: [
    {
      tagName: 'script',
      attributes: {},
      innerHTML: `
        window.SITE = {
          defaultTheme: 'cybercore-tech',
          siteLabel: 'FrameStack'
        };

        document.addEventListener('cc:theme', () => {
          const root = document.documentElement;
          const selected = root.dataset.theme;
          if (selected && selected !== 'dark' && selected !== 'light') {
            root.dataset.cybercoreTheme = selected;
            root.dataset.theme = 'dark';
          }
        });
      `,
    },
    {
      tagName: 'script',
      attributes: {
        type: 'module',
        src: cybercoreChooserModule,
      },
    },
  ],

  stylesheets: [
    {
      href: cybercoreChooserCss,
      rel: 'stylesheet',
    },
  ],

  presets: [
    [
      'classic',
      {
        docs: {
          path: '../docs',
          routeBasePath: 'docs',
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/cybercoretech/framestack/tree/main/docs/',
          showLastUpdateTime: false,
          exclude: ['**/agent/**'],
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      },
    ],
  ],

  themes: [
    [
      '@easyops-cn/docusaurus-search-local',
      {
        hashed: true,
        language: ['en'],
        indexDocs: true,
        docsDir: '../docs',
        docsRouteBasePath: 'docs',
        indexBlog: false,
        indexPages: false,
      },
    ],
  ],

  themeConfig: {
    colorMode: {
      defaultMode: 'dark',
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    navbar: {
      title: 'FrameStack',
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'left',
          label: 'Documentation',
        },
        {
          type: 'search',
          position: 'right',
        },
        {
          href: 'https://github.com/cybercoretech/framestack',
          position: 'right',
          className: 'header-github-link',
          'aria-label': 'FrameStack on GitHub',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Project',
          items: [
            {label: 'GitHub', href: 'https://github.com/cybercoretech/framestack'},
            {label: 'Architecture blueprint', to: '/docs/BLUEPRINT'},
            {label: 'Roadmap', to: '/docs/ROADMAP'},
          ],
        },
        {
          title: 'Documentation',
          items: [
            {label: 'Documentation standards', to: '/docs/documentation/STANDARDS'},
            {label: 'Architecture decisions', to: '/docs/decisions/project-scope'},
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Cybercore Tech. Built with Docusaurus.`,
    },
  },
};

export default config;
