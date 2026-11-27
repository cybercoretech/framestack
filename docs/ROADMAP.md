# FrameStack Roadmap

**Document status:** Proposed
**Last updated:** 2026-11-22

This roadmap prioritizes a small, verifiable implementation over a broad collection of untested integrations. Catalog inclusion means candidate evaluation only; support requires working examples and passing CI.

## Stage 0 — Architecture and integration model

**Goal:** Establish the project boundary, project model, integration taxonomy, and contracts before implementation.

### Deliverables

- [x] Draft project scope and architecture blueprint.
- [x] Draft core/adapter boundary proposal.
- [x] Draft contract-first interoperability proposal.
- [x] Establish initial Rust and TypeScript integration candidate catalog.
- [x] Classify frontend, backend, full-stack, RPC, and service integrations.
- [x] Define candidate lifecycle statuses and initial integration metadata requirements.
- [ ] Review and accept or revise ADR-0001 through ADR-0004.
- [x] Define the smallest useful project graph and component relationship model.
- [x] Define the first capability vocabulary and constraint representation.
- [x] Choose the first Rust and TypeScript reference stacks.
- [x] Decide manifest format, schema dialect, and compatibility policy.
- [x] Decide Rust workspace and TypeScript package-manager conventions.
- [x] Decide license; defer minimum supported toolchain versions until CI establishes a tested baseline.

### Exit criteria

- Project scope and integration taxonomy are accepted.
- Candidate integrations are clearly distinguished from supported integrations.
- The project graph can describe Rust-only, TypeScript-only, and mixed-language applications.
- Initial capability and composition semantics are documented.
- The first reference application(s) and their verification requirements are selected.

## Stage 1 — Contracts, manifest, and catalog foundation

**Goal:** Implement the smallest language-neutral project model and validate it consistently.

### Deliverables

- Versioned manifest contract and schema.
- TOML parsing into a normalized representation, if TOML is accepted.
- Valid, invalid, boundary, and compatibility fixtures.
- Minimal project graph and component relationship types.
- Integration descriptor format with lifecycle status, identity, version, dependencies, and capabilities.
- Initial capability validation and composition diagnostics.
- Rust core crate and CLI foundation.
- Initial CLI commands: `new`, `inspect`, and `check`.
- Rust and TypeScript contract tests where both languages consume the same contract.

### Exit criteria

- A sample project manifest parses and validates.
- The project graph is inspectable.
- Invalid integrations, missing requirements, and unsupported capabilities produce actionable diagnostics.
- Rust and TypeScript contract implementations pass the same shared fixtures.

## Stage 2 — First Rust vertical slice

**Goal:** Prove the core and adapter boundary with a real Rust application.

### Candidate reference stack

- Frontend: select one of Leptos, Dioxus, or Yew.
- HTTP/backend: select one of Axum, Actix Web, or Poem, unless a selected full-stack application framework provides the more appropriate reference.
- RPC: defer Tonic until a concrete RPC use case is included.

### Deliverables

- One working Rust frontend adapter.
- One working Rust backend or application adapter.
- Minimal reference application.
- Development and build operation plans.
- Structured execution and normalized diagnostics.
- Adapter contract and composition tests.
- CI for the reference application.

### Exit criteria

- The reference application runs and builds using documented FrameStack commands.
- Capabilities are validated before execution.
- Adapter-specific behavior remains outside core public types.
- CI proves the advertised integration capabilities.

## Stage 3 — TypeScript and mixed-language interoperability

**Goal:** Prove that the same project model works across Rust and TypeScript without erasing runtime differences.

### Candidate reference stack

- Frontend: select React, Svelte, Vue, Next.js, or Nuxt according to the chosen reference architecture.
- Backend: select Fastify, Hono, Express, NestJS, AdonisJS, or Feathers.
- The selected integrations must represent materially different lifecycle requirements from the Rust reference slice.

### Deliverables

- At least one working TypeScript adapter.
- Shared contract fixtures consumed by Rust and TypeScript.
- A mixed-language reference application or service composition.
- Explicit runtime, build, and communication relationships.
- CI coverage for the mixed-language example.

### Exit criteria

- Rust and TypeScript implementations agree on shared contracts.
- The mixed-language project graph validates.
- Cross-language communication and build dependencies are documented and tested.
- No framework-specific types leak into core contracts.

## Stage 4 — Integration expansion and developer experience

**Goal:** Expand the catalog only where there is a tested implementation path and meaningful user value.

### Candidate expansion areas

- Additional Rust frontend adapters: Leptos, Dioxus, and Yew as individually verified integrations.
- Additional Rust HTTP/backend adapters: Axum, Actix Web, and Poem.
- Loco application-framework integration.
- Tonic and other RPC implementations when supported by a concrete reference use case.
- Additional TypeScript frontend adapters: React, Svelte, and Vue.
- Next.js and Nuxt as distinct full-stack meta-framework adapters.
- Additional TypeScript backend adapters: Fastify, Hono, Express, NestJS, and AdonisJS.
- Feathers service and real-time integration.
- Protocol and transport capabilities such as WebSocket, GraphQL, and Server-Sent Events.
- Optional database, authentication, background-job, observability, and deployment integrations.

### Developer experience

- Improve templates and project generation.
- Improve `doctor`, diagnostics, and integration inspection.
- Add compatibility matrices and adapter verification reporting.
- Document capability limitations and composition patterns.
- Improve editor and CI integration.

### Exit criteria

- Every integration marked supported has a reference example and passing CI.
- Integration compatibility and lifecycle status are visible to users.
- Expansion does not weaken core boundaries or create misleading common abstractions.

## Stage 5 — Stabilization

**Goal:** Stabilize contracts, compatibility policy, and release practices.

### Deliverables

- Versioned core and adapter compatibility policy.
- Manifest and schema migration policy.
- Security review of operation planning and execution.
- Reproducible build and toolchain guidance.
- Documented support lifecycle and deprecation policy.
- Release automation and migration documentation.
- Performance and reliability baselines.

### Exit criteria

- Stable contracts have documented compatibility guarantees.
- Supported integrations have maintained CI coverage.
- Security boundaries and execution permissions are documented.
- Users can upgrade projects with predictable migration guidance.

## Ongoing project guardrails

- Do not claim an integration is supported until its reference example and required CI checks pass.
- Do not create empty crates, packages, or adapters just to match a proposed directory tree.
- Do not place framework-specific types in core public contracts.
- Do not force all integrations into the same lifecycle or capability set.
- Do not conflate UI frameworks, full-stack meta-frameworks, backend frameworks, application frameworks, protocols, and transports.
- Do not introduce arbitrary plugin execution before the adapter trust and permission model is established.
- Do not commit secrets, local machine state, or `AGENT_HANDOFF.md`.
- Prefer small, testable vertical slices over speculative infrastructure.

<!-- FRAMESTACK-V1-DECISIONS:START -->
### Accepted Stage 0 decisions

The following decisions are accepted and recorded in ADR-0005 through ADR-0008:

- Project graph: directed components with `application`, `service`, `worker`, and `resource` kinds.
- Relationships: `build_dependency`, `runtime_dependency`, and `communicates_with`, with build dependencies validated as an acyclic graph.
- Capability model: structured `provides`, `requires`, and `constraints`, with the initial vocabulary documented in ADR-0006.
- Adapter operations: `inspect`, `check`, `dev.start`, `dev.stop`, `build`, and `test`, separate from application capabilities.
- Rust reference stack: Leptos + Axum.
- TypeScript reference stack: React + Vite + Fastify.
- Later mixed-language proof: React + Vite communicating with Axum.
- Manifest: `framestack.toml`, normalized JSON-compatible contract, JSON Schema Draft 2020-12, and separate semantic validation.
- Tooling: Rust core and CLI with Cargo; TypeScript workspace with pnpm.
- License: MIT.
- Minimum supported Rust and Node.js versions remain open until selected versions pass CI.

Stage 0 is not complete until the remaining ADR review and toolchain-baseline work are finished.
<!-- FRAMESTACK-V1-DECISIONS:END -->

<!-- FRAMESTACK-DOCS-PLATFORM:START -->
## Documentation platform and publishing

**Decision:** ADR-0009 — Docusaurus 3.x, Markdown + MDX, TypeScript + React, and a custom FrameStack/CyberCore theme.

### Accepted direction

- Canonical documentation content remains in `docs/`.
- The Docusaurus application and theme live in `website/`.
- The site will use integrated documentation search.
- GitHub Pages is the initial hosting target; a custom domain may be added later.
- Documentation snapshots are published alongside FrameStack releases.
- Previous release snapshots and migration guides remain accessible.
- API references are generated where practical and supplemented with hand-written explanations.
- CI progressively validates Markdown/MDX, links, metadata, navigation, production builds, examples, and generated references.

### Documentation work sequence

1. Establish and maintain the documentation standards in `docs/documentation/STANDARDS.md`.
2. Build the Docusaurus application under `website/` and configure it to consume canonical `docs/` content.
3. Explicitly exclude internal-only planning and agent material from public pages and search.
4. Implement navigation, integrated search, and the custom theme.
5. Add substantive getting-started, concepts, guide, integration, reference, architecture, security, and contributor pages as implementation contracts become real.
6. Add generated Rust, TypeScript, CLI, and manifest reference workflows where authoritative source definitions exist.
7. Add CI documentation validation and example checks incrementally.
8. Publish versioned documentation snapshots and retain migration guides with releases.

Do not create empty placeholder pages merely to fill the navigation structure. Do not claim generated references, tested examples, or release snapshots exist until the corresponding workflows have been implemented and verified.
<!-- FRAMESTACK-DOCS-PLATFORM:END -->

<!-- FRAMESTACK-DOCS-SITE:START -->
## Docusaurus website foundation

The website foundation is being established under `website/`, using canonical content in `docs/`.

- Docusaurus **3.10.2** (stable Docusaurus 3 line)
- TypeScript + React
- pnpm workspace, pinned through the root `packageManager` field
- Node.js 26 Active LTS baseline, selected through `.nvmrc`
- Canonical docs loaded from `../docs`; internal `docs/agent/` material excluded
- CyberCore's published theme chooser and live theme registry integrated as shared site-kit assets
- Local documentation search
- GitHub Pages build/deployment workflow
- CI install, type-check, and production-build validation

The initial site contains a substantive documentation index and project landing page. It does not create empty placeholder guides or claim that planned integrations are implemented.

Versioned documentation snapshots are intentionally deferred until the first FrameStack release. Generate the snapshot from that release's canonical `docs/` content and publish it with the release. Migration guides remain retained.
<!-- FRAMESTACK-DOCS-SITE:END -->
