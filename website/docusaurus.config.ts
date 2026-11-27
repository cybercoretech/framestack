import {readFileSync} from 'node:fs';
import type {Config} from '@docusaurus/types';

const cybercoreChooserCss =
  'https://cybercoretech.github.io/kit/chooser.css';
const cybercoreChooserModule =
  'https://cybercoretech.github.io/kit/chooser.js';
const cybercoreContactModule =
  'https://cybercoretech.github.io/kit/contact.js';
const fontsCss =
  'https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,400;0,500;0,600;0,700;0,800;1,400&display=swap';

// Phone and tablet fixes, kept apart so desktop renders identically without them
// (checked with the Cybercore rollout tool deskdiff.mjs).
const mobileFirstCss = readFileSync(new URL('./src/css/mobile-first.css', import.meta.url), 'utf8');

const config: Config = {
  title: 'FrameStack',
  tagline: 'One manifest for the whole system. Checked, graphed and planned before anything runs.',
  favicon: 'img/favicon.svg',
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
          defaultTheme: 'framestack-stack-trace',
          siteLabel: 'FRAMESTACK',
          mashups: [
            { id: 'framestack-stack-trace', label: 'STACK TRACE', base: 'cyberdyne-steel', accent: 'neosynth-plasma' },
            { id: 'framestack-graph-noir', label: 'GRAPH NOIR', base: 'dystopian-ward', accent: 'cyberpunk-hacker' },
            { id: 'framestack-signal-weave', label: 'SIGNAL WEAVE', base: 'kanagawa', accent: 'synthwave-sunset' }
          ]
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
    {
      tagName: 'script',
      attributes: {
        type: 'module',
        src: cybercoreContactModule,
      },
    },
    {tagName: 'meta', attributes: {name: 'theme-color', content: '#05070b'}},
    {tagName: 'link', attributes: {rel: 'preconnect', href: 'https://fonts.googleapis.com'}},
    {tagName: 'link', attributes: {rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: 'anonymous'}},
    {tagName: 'style', attributes: {id: 'cc-mobile-first'}, innerHTML: mobileFirstCss},
  ],

  stylesheets: [
    {
      href: fontsCss,
      rel: 'stylesheet',
    },
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
    image: 'img/social.png',
    metadata: [
      {name: 'twitter:card', content: 'summary_large_image'},
      {name: 'color-scheme', content: 'dark'},
    ],
    colorMode: {
      defaultMode: 'dark',
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    navbar: {
      title: 'FrameStack',
      logo: {alt: 'FrameStack', src: 'img/favicon.svg', width: 26, height: 26},
      items: [
        {
          type: 'html',
          position: 'left',
          value: '<span class="framestack-path" aria-hidden="true">~/cybercore/framestack <b>main</b></span>',
        },
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'left',
          label: 'docs',
        },
        {to: '/docs/reference/cli', label: 'cli', position: 'left'},
        {to: '/docs/reference/manifest', label: 'manifest', position: 'left'},
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
          title: 'project',
          items: [
            {label: 'GitHub', href: 'https://github.com/cybercoretech/framestack'},
            {label: 'Reference app', href: 'https://github.com/cybercoretech/framestack/tree/main/examples/rust-leptos-axum'},
            {label: 'Architecture blueprint', to: '/docs/BLUEPRINT'},
            {label: 'Roadmap', to: '/docs/ROADMAP'},
          ],
        },
        {
          title: 'reference',
          items: [
            {label: 'CLI', to: '/docs/reference/cli'},
            {label: 'Manifest', to: '/docs/reference/manifest'},
            {label: 'Adapters', to: '/docs/reference/adapters'},
            {label: 'Documentation standards', to: '/docs/documentation/STANDARDS'},
            {label: 'Architecture decisions', to: '/docs/decisions/project-scope'},
          ],
        },
      ],
      copyright: 'Copyright (c) 2026 Cybercore Tech · FrameStack · built with Docusaurus',
    },
  },
};

export default config;
