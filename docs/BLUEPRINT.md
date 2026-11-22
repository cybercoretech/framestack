# FrameStack Architecture Blueprint

**Document status:** Proposed architecture
**Last updated:** 2026-11-22

## 1. Vision

FrameStack is a modular, multi-language full-stack application framework and development ecosystem. It helps developers assemble, inspect, validate, develop, test, build, and eventually ship applications across multiple frontend, backend, and service technologies through a consistent project model and developer experience.

FrameStack connects existing ecosystems through explicit adapters and versioned contracts. It does not aim to replace Leptos, Dioxus, Yew, React, Svelte, Vue, Next.js, Nuxt, Axum, Actix Web, or the wider ecosystems.

The project model must represent applications composed of multiple components and services. It must not assume that every project is one frontend paired with one backend, or that every component uses the same language, runtime, protocol, or deployment model.

## 2. Goals

- Support Rust-native frontend frameworks, initially evaluating Leptos, Dioxus, and Yew.
- Support TypeScript frontend ecosystems, initially evaluating React, Svelte, Vue, Next.js, and Nuxt.
- Support multiple Rust and TypeScript backend and application frameworks.
- Represent HTTP, RPC, real-time, and other communication boundaries explicitly.
- Provide one coherent CLI and project configuration experience.
- Keep core application concepts independent of framework-specific types.
- Model project components, dependencies, capabilities, and operation targets.
- Make adapters independently testable, replaceable, and versionable.
- Establish contract-first interfaces for cross-language communication.
- Provide clear diagnostics, predictable builds, and secure execution defaults.
- Grow from small, working vertical slices rather than a large collection of placeholders.

## 3. Non-goals

- Reimplementing supported frontend, backend, or application frameworks.
- Forcing every application into one language, runtime, rendering model, or deployment topology.
- Hiding meaningful framework differences behind a lowest-common-denominator API.
- Requiring every application to use both a frontend and a backend.
- Treating protocols and transports as interchangeable with application frameworks.
- Building a remote plugin marketplace before local adapter contracts are proven.
- Promising that every integration supports every capability.
- Introducing distributed orchestration when a local process model is sufficient.

## 4. Architectural principles

### 4.1 Small core, explicit adapters

The core owns the language-neutral project model, manifest semantics, dependency graph, capability resolution, operation planning, lifecycle coordination, normalized diagnostics, and stable contracts.

Framework-specific behavior belongs in adapters. The core must not import framework-specific crates or packages such as React, Leptos, Dioxus, Axum, Actix Web, or vendor-specific database clients.

### 4.2 Model the project as a graph

A FrameStack project is a graph of declared components and relationships, not a fixed frontend/backend pair.

A component may represent a browser frontend, server application, API service, worker, shared package, or another explicitly supported component kind. Relationships may describe dependencies, service calls, shared contracts, build ordering, or other declared connections.

The initial model should stay small. Add component kinds and relationship types only when a real reference application requires them. The graph must preserve distinctions between build-time dependencies, runtime dependencies, and communication links.

### 4.3 Integration taxonomy is not an execution contract

FrameStack classifies integrations by role, ecosystem, and composition model. Categories help users discover and select integrations; they do not imply that integrations in the same category expose identical behavior.

An integration may occupy more than one role. For example, Next.js and Nuxt are full-stack meta-frameworks built around frontend ecosystems, while Loco is a batteries-included Rust application framework built on the Axum ecosystem. Tonic is an RPC framework, not a general-purpose browser-facing HTTP application framework.

The catalog is a candidate inventory, not a support guarantee. Every entry must carry an explicit lifecycle/support status.

### 4.4 Capability-oriented integrations

An adapter declares the operations and capabilities it supports, including relevant constraints and dependencies. Capabilities may include development server, build, test, server-side rendering, hydration, static generation, routing, HTTP endpoints, RPC services, WebSockets, asset output, health checks, or deployment packaging.

Capabilities should be precise enough to express important differences. Do not infer that an integration supports a capability merely because another integration in its category does.

Unsupported or incompatible capabilities must produce actionable diagnostics rather than silent fallback.

### 4.5 Contract-first interoperability

Language boundaries should be defined by versioned, language-neutral schemas and examples before independent Rust and TypeScript implementations are written.

JSON Schema is the initial candidate for portable data contracts. TOML manifests should be parsed into a normalized data model before schema validation; JSON Schema does not validate TOML source text directly.

Shared fixtures should cover valid, invalid, boundary, and compatibility cases. Internal implementation types must not automatically become public contracts.

### 4.6 No accidental vendor lock-in

The application model describes intent. Adapters translate that intent into framework-specific configuration and operations. Vendor-specific options remain available through clearly namespaced escape hatches rather than contaminating core types.

### 4.7 Explicit planning and execution

Separate adapter discovery, capability declaration, operation planning, and operation execution.

A plan should identify the adapter, requested operation, required capabilities, working directory, structured command and arguments (where applicable), declared inputs and outputs, environment-variable references, and relevant resource requirements.

Execution must occur only through an explicit boundary. Avoid implicit shell evaluation; pass commands and arguments as structured values. The initial implementation may use in-process adapters and child processes, but should not introduce arbitrary third-party code loading before its trust and permission model is established.

### 4.8 Secure and observable by default

Operations that execute commands, access networks, read secrets, or modify files must be explicit. Diagnostics should identify the adapter and operation that produced an error without leaking secret values.

## 5. Proposed logical architecture

```text
                 Developer / CI / Editor
                          |
                     FrameStack CLI
                          |
                Manifest + Project Model
                Components + Project Graph
                          |
                  Core Validation
                 Capability Resolution
                          |
                   Operation Planner
                          |
                Adapter Execution Boundary
                 /          |           \
          Frontend      Backend       Protocol /
          Adapters      Adapters      Tooling Adapters
             |              |              |
       Rust / TS UI    App / API       HTTP / RPC /
       frameworks      frameworks      Realtime / Build
                 \          |           /
                  Contracts + Shared Types
                          |
               Diagnostics / Artifacts
```

This is a logical view, not a required process topology. An adapter may run in-process or be invoked as a child process where isolation, runtime, or toolchain boundaries require it.

## 6. Integration taxonomy and initial candidate catalog

All entries below are **candidates**. Listing an integration does not mean it is implemented, tested, or officially supported. The catalog must track lifecycle status, adapter version, compatibility, capabilities, dependencies, and verification evidence.

### 6.1 Rust frontend frameworks

| Integration | Primary role | Initial status |
| --- | --- | --- |
| Leptos | Rust reactive web frontend | Candidate |
| Dioxus | Multi-platform Rust UI and application framework | Candidate |
| Yew | Rust WebAssembly frontend | Candidate |

These integrations require independent adapters. Their rendering, hydration, routing, target platforms, build tools, and server integration models must not be assumed identical.

### 6.2 Rust HTTP and backend frameworks

| Integration | Primary role | Initial status |
| --- | --- | --- |
| Axum | Async HTTP application framework | Candidate |
| Actix Web | HTTP application framework | Candidate |
| Poem | HTTP application framework | Candidate |

### 6.3 Rust full-stack application frameworks

| Integration | Primary role | Initial status |
| --- | --- | --- |
| Loco | Batteries-included application framework built on the Axum ecosystem | Candidate |

Loco must be modeled as an application-level integration with its own lifecycle and conventions. Its relationship to Axum should be represented as an implementation/dependency relationship, not as a requirement to register two competing application servers.

### 6.4 Rust RPC frameworks

| Integration | Primary role | Initial status |
| --- | --- | --- |
| Tonic | gRPC/RPC services and clients | Candidate |

Tonic belongs in the RPC category. It is not a substitute for a browser-facing HTTP application framework. Its maintenance state, compatibility, and ecosystem status must be checked when selecting it for a supported reference stack.

### 6.5 TypeScript frontend frameworks and libraries

| Integration | Primary role | Initial status |
| --- | --- | --- |
| React | UI library and frontend ecosystem | Candidate |
| Svelte | UI framework | Candidate |
| Vue | UI framework | Candidate |

### 6.6 TypeScript full-stack frontend meta-frameworks

| Integration | Primary role | Initial status |
| --- | --- | --- |
| Next.js | React-based full-stack web framework | Candidate |
| Nuxt | Vue-based full-stack web framework | Candidate |

Next.js and Nuxt require their own adapters. They must not be represented as merely aliases for React and Vue because they add routing, server execution, rendering, build, and deployment behavior.

### 6.7 TypeScript backend and HTTP frameworks

| Integration | Primary role | Initial status |
| --- | --- | --- |
| Fastify | HTTP application framework | Candidate |
| Hono | Lightweight web framework with multiple runtime targets | Candidate |
| Express | Minimal HTTP application framework | Candidate |
| NestJS | Structured application framework | Candidate |
| AdonisJS | Batteries-included application framework | Candidate |

These frameworks have different levels of convention, dependency injection, middleware, runtime support, and application structure. The adapter contract must preserve these differences.

### 6.8 TypeScript API and real-time service frameworks

| Integration | Primary role | Initial status |
| --- | --- | --- |
| Feathers | Service-oriented API and real-time application framework | Candidate |

Feathers should be represented by its service-oriented capabilities, not forced into a generic HTTP-only abstraction.

### 6.9 Protocols and transports

Protocols and transports are separate from frontend and backend framework adapters. Candidate capability families include:

- HTTP and REST.
- gRPC and RPC.
- WebSocket.
- GraphQL.
- Server-Sent Events.
- Messaging and event-broker transports.

The initial implementation should only add protocol adapters when a reference application demonstrates a concrete need. A protocol may be provided by a framework adapter, a dedicated adapter, or both, but ownership and composition must be explicit.

### 6.10 Future integration categories

The architecture may later include adapters for:

- Databases and persistence.
- Authentication and identity.
- Background jobs and schedulers.
- Messaging and event brokers.
- Observability and telemetry.
- Code generation.
- Deployment and hosting.
- Infrastructure and local development services.

These are architectural extension points, not commitments to implement every vendor or product.

## 7. Integration metadata and capability model

Each catalog entry should eventually have machine-readable metadata covering at least:

1. **Identity:** stable integration ID, display name, ecosystem, and integration type.
2. **Lifecycle status:** candidate, experimental, supported, deprecated, or removed.
3. **Adapter version:** version of the FrameStack adapter implementation.
4. **Compatibility:** supported manifest contract versions, FrameStack core versions, framework versions, runtimes, and toolchains.
5. **Composition:** integrations it builds on, requires, replaces, or conflicts with.
6. **Capabilities:** supported operations and their constraints.
7. **Configuration:** required and optional configuration, including namespaced framework-specific settings.
8. **Operations:** detection, validation, planning, and execution behavior.
9. **Requirements:** required tools, runtime versions, environment variables, and external services.
10. **Evidence:** reference examples, contract tests, CI workflows, and last verification information.

Capabilities should be structured identifiers with optional parameters and constraints, not informal marketing labels. For example, `frontend.ssr` and `frontend.static_generation` should be distinct capabilities. A capability declaration may also describe supported targets or limitations.

The initial capability vocabulary should be deliberately small and expanded from working integrations. Do not define a massive speculative capability registry before the first adapters are compared.

## 8. Core, CLI, and adapter responsibilities

### 8.1 Core

Responsibilities:

- Parse the normalized manifest representation and validate project structure.
- Build and validate the project component/dependency graph.
- Resolve integration identities, versions, dependencies, and declared capabilities.
- Coordinate operation planning and approved execution.
- Normalize diagnostics, task results, and generated artifacts.
- Enforce policy decisions and explicit execution boundaries.

The core should remain small and avoid owning framework-specific build logic.

### 8.2 CLI

Proposed command families:

| Command | Purpose |
| --- | --- |
| `framestack new` | Create a project from a template |
| `framestack init` | Initialize FrameStack in an existing project |
| `framestack inspect` | Display project graph and detected integrations |
| `framestack check` | Validate manifest, graph, contracts, and integration compatibility |
| `framestack dev` | Start the local development workflow |
| `framestack build` | Build selected application targets |
| `framestack test` | Run project and adapter tests |
| `framestack doctor` | Diagnose missing tools and environment problems |

Command names and exact behavior remain proposals until their implementation ADRs and reference slices are complete.

### 8.3 Adapter lifecycle

The initial adapter lifecycle should distinguish:

- **Discover:** identify applicable integrations and adapter versions.
- **Describe:** report identity, dependencies, compatibility, and capabilities.
- **Validate:** validate adapter-specific configuration and project requirements.
- **Plan:** return a structured operation plan.
- **Execute:** perform an explicitly approved operation.
- **Report:** return normalized results, diagnostics, and declared artifacts.

Do not stabilize Rust traits or TypeScript interfaces until at least two materially different adapters have been compared and exercised.

## 9. Project configuration

A versioned `framestack.toml` manifest is the leading candidate because it is readable and fits naturally into developer workflows.

The manifest should describe:

- Manifest format version.
- Application identity and workspace layout.
- Components and their kinds.
- Integration IDs and version constraints.
- Component dependencies and communication relationships.
- Development, build, test, and deployment targets.
- Required capabilities.
- Explicit environment-variable references, never secret values.
- Optional framework-specific configuration in namespaced sections.

The project model must support Rust-only, TypeScript-only, and mixed-language projects. It must also support projects with multiple frontends, backends, workers, or services without requiring all of them.

TOML parsing should produce a normalized representation that is validated against a versioned language-neutral schema. The schema dialect, strictness policy, unknown-field behavior, and compatibility rules must be decided in an implementation ADR.

## 10. Repository organization

The repository should evolve toward the following layout as implementation begins:

```text
framestack/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   └── workflows/
├── crates/
│   ├── framestack-core/
│   ├── framestack-cli/
│   └── adapters/
├── packages/
│   ├── core-types/
│   └── adapters/
├── schemas/
│   └── v1/
├── fixtures/
│   ├── valid/
│   ├── invalid/
│   └── compatibility/
├── integrations/
│   └── catalog/
├── examples/
├── templates/
├── docs/
│   └── decisions/
├── scripts/
├── Cargo.toml
├── Cargo.lock
├── package.json
└── README.md
```

Do not create empty crates or packages merely to match this diagram. Add each workspace member when it has a defined responsibility and a testable first implementation.

## 11. Testing strategy

- Unit tests for core graph, state transitions, manifest, and capability rules.
- Adapter contract tests shared across implementations.
- Golden fixtures for cross-language serialization and validation.
- Integration tests using minimal reference applications.
- CLI smoke tests for implemented commands.
- Compatibility tests for supported framework and toolchain versions.
- Security tests for path handling, command execution, environment propagation, and secret redaction.
- Composition tests for integrations that build on or require other integrations.
- Capability tests proving that declared capabilities actually work.

An integration is not supported until its reference example, adapter contract tests, and required CI checks pass. Candidate status must not be presented as supported status.

## 12. Security boundaries

- Never evaluate project configuration as executable code.
- Avoid implicit shell evaluation; pass commands and arguments as structured values.
- Make network access and filesystem writes explicit in operation plans.
- Do not log secret values.
- Validate generated paths and prevent path traversal.
- Keep optional adapters from silently expanding core permissions.
- Pin or constrain tool versions where reproducibility requires it.
- Do not load arbitrary third-party adapter code until the trust, permission, and isolation model is established.

## 13. Open architecture questions

These questions must be answered through decision records or prototypes:

1. What is the smallest useful project graph and component relationship model?
2. Should the core be a Rust library with a stable CLI protocol, and which contracts must be language-neutral?
3. Which Rust and TypeScript reference stacks form the first vertical slices?
4. Which manifest format, schema dialect, and compatibility policy should be adopted?
5. What is the initial capability vocabulary and how are capability constraints represented?
6. Which adapter operations run in-process versus as child processes?
7. How are integration dependencies, conflicts, and composition represented?
8. Which package manager should govern TypeScript workspace packages?
9. What are the minimum supported Rust, Node.js, and other runtime versions?
10. Which license should the project use?

## 14. Acceptance criteria for the first implementation milestone

The first implementation milestone is complete when:

- A minimal manifest can be parsed and validated against a versioned contract.
- The project graph can represent at least one frontend and one backend/service component.
- The CLI can inspect the project graph and explain selected integrations.
- At least one frontend and one backend adapter can be composed.
- Adapter capabilities are validated before operations are planned.
- The reference application can run locally using documented commands.
- Shared contract fixtures pass in every implementation language involved.
- CI verifies formatting, linting, tests, and the reference application.
- Unsupported capabilities and incompatible compositions fail with clear diagnostics.
- Candidate integrations are not described as supported without CI evidence.

## 15. Decision records

The architecture is governed by the following decision records:

- ADR-0001: Project scope.
- ADR-0002: Core and adapter boundaries.
- ADR-0003: Contract-first interoperability.
- ADR-0004: Integration taxonomy and capability metadata.

Additional decisions should be recorded before implementation choices become difficult to reverse.

<!-- FRAMESTACK-V1-DECISIONS:START -->
## Accepted v1 architecture baseline

The following decisions establish the FrameStack v1 architecture baseline. Their detailed rationale and consequences are recorded in ADR-0005 through ADR-0008.

### Project graph

FrameStack models a project as a directed graph of components. Initial component kinds are `application`, `service`, `worker`, and `resource`. Component kind is distinct from integration role.

Initial directed relationship types are:

- `build_dependency`: the source requires the target's build output or completed build.
- `runtime_dependency`: the source requires the target at runtime.
- `communicates_with`: the source communicates with the target through a declared interface or protocol.

The build-dependency subgraph must be acyclic. Runtime dependencies and communication relationships do not imply build ordering. Component IDs and relationship references must be valid and unique.

### Capabilities and operations

Integrations declare `provides`, `requires`, and structured `constraints`. The initial capability vocabulary is:

- Frontend: `frontend.browser`, `frontend.spa`, `frontend.ssr`, `frontend.ssg`, `frontend.hydration`.
- HTTP: `http.server`, `http.api`, `http.middleware`.
- API: `api.rest`, `api.graphql`.
- RPC: `rpc.grpc`.
- Realtime: `realtime.websocket`, `realtime.sse`.
- Background: `worker.background`.
- Persistence: `persistence.sql`, `persistence.nosql`.
- Messaging: `messaging.publish`, `messaging.subscribe`.

Operations are separate from capabilities. Initial operation identifiers are `inspect`, `check`, `dev.start`, `dev.stop`, `build`, and `test`. The adapter lifecycle is discover, describe, validate, plan, execute, and report. Execution uses explicit structured plans, not implicit evaluation of arbitrary adapter-provided shell strings.

### Official reference stacks

- Rust: Leptos + Axum, using Cargo and exercising server rendering and hydration.
- TypeScript: React + Vite + Fastify, using TypeScript and pnpm, with separate frontend and API components.
- Later mixed-language proof: React + Vite frontend communicating with an Axum service.

Selection does not imply implementation or support. Each adapter and reference stack must pass its documented contract tests and CI before support is claimed.

### Manifest and tooling

The project manifest is `framestack.toml`. TOML is parsed into a normalized JSON-compatible representation and validated with JSON Schema Draft 2020-12. Semantic graph, capability, composition, and compatibility checks are performed by the core.

The core and initial CLI use Rust and a Cargo workspace. TypeScript packages use pnpm workspaces. Use stable Rust and an Active LTS Node.js release for the implementation baseline; exact minimum versions remain open until verified in CI. The project license is MIT.

<!-- FRAMESTACK-V1-DECISIONS:END -->
