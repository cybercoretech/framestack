# FrameStack

**A modular, multi-language full-stack application framework and development ecosystem.**

FrameStack aims to make it practical to compose modern application stacks without forcing every project into one frontend, backend runtime, database, or deployment provider.

The project is being designed around a small, stable core; explicit integration boundaries; contract-first interfaces; and first-class developer tooling.

> **Status:** Blueprint / architecture definition. Framework APIs and integration support are not yet stable.

## Project goals

- Support Rust-native frontend options such as [Leptos](https://leptos.dev/) and [Dioxus](https://dioxuslabs.com/), alongside TypeScript ecosystems such as React.
- Integrate multiple backend runtimes and service frameworks through adapters rather than hard-coding one stack.
- Provide a consistent project model, configuration format, CLI, and development workflow.
- Keep core contracts independent of optional integrations.
- Make integrations independently testable and replaceable.
- Favor secure defaults, explicit capabilities, reproducible builds, and clear diagnostics.

## Planned architecture

| Area | Responsibility |
| --- | --- |
| Core | Application model, configuration, lifecycle, shared contracts |
| CLI | Project creation, inspection, development, build, and validation commands |
| Frontend adapters | Leptos, Dioxus, React, and other frontend integrations |
| Backend adapters | Rust and TypeScript service runtimes and framework integrations |
| Contracts | Versioned schemas, fixtures, and cross-language compatibility checks |
| Tooling | Build orchestration, generators, diagnostics, and developer experience |
| Integrations | Databases, authentication, APIs, observability, and deployment targets |

See [the architecture blueprint](docs/BLUEPRINT.md), [the roadmap](docs/ROADMAP.md), and the [architecture decision records](docs/decisions/).

## Repository layout

```text
framestack/
├── .github/              # CI, issue templates, and repository automation
├── docs/                 # Architecture, roadmap, and decision records
├── crates/               # Rust workspace crates (introduced with implementation)
├── packages/             # TypeScript packages (introduced with implementation)
├── integrations/         # Optional framework and service adapters
├── examples/             # Runnable reference applications
├── schemas/              # Versioned cross-language contracts
├── scripts/              # Maintainer and development scripts
├── PROJECT_STATE.md      # Tracked project status and next actions
└── AGENT_HANDOFF.md      # Local-only agent context; intentionally gitignored
```

Directories for implementation layers will be added as their APIs are defined; the blueprint does not imply that those integrations are already implemented.

## Development status

FrameStack is in the blueprint phase. Initial work should establish the application model, configuration contract, adapter boundary, and CLI responsibilities before committing to a large set of framework-specific dependencies.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing architectural or implementation changes.
