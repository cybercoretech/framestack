# FrameStack documentation website

The documentation website is a Docusaurus 3 application. Canonical Markdown and MDX content remains in the repository's `docs/` directory; this directory contains the website application, theme, configuration, and publishing workflow.

## Toolchain

- Node.js 26.13.1 baseline, recorded in the repository root `.nvmrc`
- pnpm 12.22.0, pinned by the root `package.json`
- Docusaurus 3.10.2
- TypeScript and React
- Local search through `@easyops-cn/docusaurus-search-local`

Run commands from the repository root:

```zsh
pnpm install
pnpm docs:dev
pnpm docs:build
pnpm docs:serve
pnpm run check
```

## Documentation source

The Docusaurus docs plugin reads `../docs` directly. It excludes `docs/agent/` so local agent handoff material is not published or indexed. Architecture decisions, the blueprint, roadmap, and documentation standards are currently part of the public documentation source.

Do not create a second manually maintained copy of documentation under `website/`.

## CyberCore theme integration

FrameStack reuses the shared CyberCore theme chooser and palette registry rather than implementing a competing chooser or theme catalog.

- Canonical chooser source: `cybercoretech/cybercore`, `web/chooser.js` and `web/chooser.css`
- Published site-kit: `https://cybercoretech.github.io/kit/chooser.js` and `https://cybercoretech.github.io/kit/chooser.css`
- Shared registry and palettes: the published site-kit data directory

The site sets `window.SITE` before loading the chooser module and uses `cybercore-tech` as its site-default palette. Docusaurus CSS maps Infima design tokens to the CyberCore palette slots. A theme event bridge preserves Docusaurus's dark-mode selector while retaining the active CyberCore palette ID separately.

The chooser and its data are served by the CyberCore published site-kit. If those remote assets are unavailable, local CSS defaults remain in effect, but the chooser and dynamic palette switching will not be available.

## Documentation versioning

Do not create a versioned snapshot for the unreleased foundation stage. When a FrameStack release is prepared, generate a snapshot from the exact release documentation source:

```zsh
pnpm --filter @framestack/docs exec docusaurus docs:version 0.1.0
```

Replace `0.1.0` with the actual release version. Review the generated `website/versioned_docs/` and `website/versions.json` changes, build the site, and publish the snapshot with the corresponding FrameStack release. Keep previous snapshots and migration guides available.

## GitHub Pages

The repository includes `.github/workflows/docs-deploy.yml`. Before the first deployment:

1. Open **Settings → Pages**.
2. Set **Build and deployment → Source** to **GitHub Actions**.
3. Ensure the workflow's `github-pages` environment is permitted by repository policy.

The site is configured for `https://cybercoretech.github.io/framestack/`. A later custom domain can be introduced by updating Docusaurus `url`, `baseUrl`, and GitHub Pages configuration together.

## Content policy

Do not add empty placeholder guides just to populate navigation. Add pages when their behavior, interfaces, examples, and status can be described accurately. Planned integrations must remain clearly labeled as planned until implemented and verified.
