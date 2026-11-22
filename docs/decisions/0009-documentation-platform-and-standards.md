# ADR-0009: Documentation Platform and Standards

- **Status:** Accepted
- **Date:** 2026-11-22
- **Decision owners:** FrameStack project
- **Scope:** Public documentation, documentation source management, publishing, and release documentation

## Context

FrameStack needs documentation that can grow with a multi-language framework, its CLI, manifest format, integration adapters, public APIs, and contributor workflows. Documentation must be maintainable alongside the implementation, support versioned releases, and provide a useful experience for readers without making the repository maintain two independent copies of the same content.

The project also needs a clear distinction between canonical documentation source, internal engineering material, and the website application that renders and publishes documentation.

## Decision

FrameStack will use the following documentation platform and conventions:

| Area | Decision |
|---|---|
| Documentation engine | Docusaurus 3.x |
| Documentation format | Markdown and MDX |
| Website implementation | TypeScript and React |
| Styling | A custom FrameStack/CyberCore theme |
| Canonical content location | Repository `docs/` directory |
| Website application location | Repository `website/` directory |
| Documentation versioning | Versioned snapshots tied to FrameStack releases |
| API documentation | Generated reference material where practical, supplemented by maintained explanations and examples |
| Initial hosting | GitHub Pages |
| Later hosting | A custom domain may be added without changing the canonical content model |
| Search | Integrated documentation search |
| Validation | CI checks for formatting, links, structure, and documentation quality |
| Release obligations | Publish documentation snapshots with releases and retain migration guides |

## Source-of-truth model

1. Markdown and MDX content maintained in `docs/` is the canonical documentation source.
2. `website/` contains the Docusaurus application, configuration, theme, custom React components, and publishing assets.
3. The website must render canonical repository content rather than become a competing, manually maintained copy.
4. Generated reference output may be produced during the documentation build. Generated output must not silently replace hand-written conceptual explanations, usage guidance, or design rationale.
5. Internal planning and agent-handoff material must not be published accidentally. The Docusaurus content configuration must explicitly select or exclude internal-only material.
6. If a future technical constraint requires content to move within `docs/`, the move must preserve a single canonical source and update links, navigation, and CI checks together.

## Documentation architecture

The documentation information architecture will cover, as applicable:

- Getting started and installation
- Core concepts and terminology
- User guides and workflows
- Integration and adapter guides
- CLI, manifest, configuration, and API reference
- Architecture and internals
- Security
- Contributor and maintainer guidance
- Release notes and migration guides
- Architecture decision records

The project will create substantive pages as the relevant behavior and contracts become real. It will not populate the site with empty placeholder pages merely to make the navigation appear complete.

## Versioning and release policy

1. Documentation snapshots must be published alongside corresponding FrameStack releases.
2. The current documentation describes the current development line.
3. Every supported release must have a discoverable documentation version appropriate to that release.
4. Breaking changes must include migration guidance describing the affected behavior, the old and new approach, required actions, and any compatibility window.
5. Migration guides must remain available after the release that introduced the breaking change.
6. Release documentation must identify features that are experimental, incomplete, deprecated, or unsupported.
7. Documentation versioning must not imply that an integration or feature is supported unless the project has verified that support.

## API reference policy

- Rust API reference material should be generated from Rust documentation comments using rustdoc where applicable.
- TypeScript API reference material should be generated from source documentation using an appropriate TypeDoc-based workflow where applicable.
- Manifest and configuration reference material should be derived from the authoritative schema or definitions when feasible.
- CLI reference material must be checked against the actual command implementation.
- Generated references must be reviewed for completeness, meaningful descriptions, stable links, and usable examples.
- Hand-written pages must explain concepts, workflows, rationale, lifecycle behavior, limitations, and cross-component relationships that generated output cannot adequately communicate.

## Quality and validation

Documentation changes are part of normal project changes and must be reviewed accordingly. CI will progressively validate:

- Markdown and MDX formatting and lint rules
- Internal links and anchors
- Required frontmatter and metadata
- Navigation references and page availability
- Docusaurus production builds
- Generated API references
- Selected executable code examples and snippets
- Release-version and migration-guide consistency

Validation must be introduced in stages that match the implementation. Checks must not claim to verify examples or generated references that are not actually executed or produced.

## Consequences

### Positive

- One canonical source of documentation content
- A React/TypeScript-based website that can support custom FrameStack components and visualizations
- Documentation versioning aligned with software releases
- Clear standards for generated reference material and hand-written explanations
- Documentation quality becomes part of CI and release discipline

### Costs and trade-offs

- The website adds a Node.js and package-management toolchain to the repository.
- Docusaurus configuration, dependencies, and upgrades require maintenance.
- MDX introduces a build-time validation requirement and should be used intentionally rather than for ordinary prose.
- Versioned snapshots and migration guides create ongoing release-maintenance work.
- Generated documentation pipelines must be kept aligned with the Rust, TypeScript, CLI, and manifest implementations.

## Implementation approach

1. Establish the documentation standards and information architecture.
2. Add the Docusaurus application under `website/` using the repository's chosen package-management conventions.
3. Configure it to consume canonical content from `docs/` while excluding internal-only material.
4. Implement the custom theme, navigation, integrated search, and GitHub Pages deployment.
5. Add generated reference pipelines as the corresponding source contracts become available.
6. Add documentation validation to CI incrementally.
7. Publish and retain versioned snapshots and migration guides with releases.

## Reconsideration criteria

Revisit this decision only if Docusaurus no longer meets project needs, its maintenance or compatibility costs become disproportionate, or a replacement can preserve canonical source ownership, release versioning, custom UI requirements, and the established documentation workflow.
