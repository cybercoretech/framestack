# FrameStack

**A modular, multi-language full-stack application framework and development ecosystem.**

FrameStack aims to make it practical to compose modern application stacks without forcing every project into one frontend, backend runtime, database, or deployment provider.

The project is being designed around a small, stable core; explicit integration boundaries; contract-first interfaces; and first-class developer tooling.

> **Status:** Early implementation. The `framestack` CLI can create, check and inspect project manifests. It does not build or run applications yet, and no framework integration is supported yet.

## Quickstart

With Rust 1.85 or newer:

```sh
cargo install --path crates/framestack-cli
cd my-project
framestack init            # writes a starter framestack.toml
framestack check           # validates it; exit code 1 if it has errors
framestack graph           # components, relationships and build order
framestack graph --format mermaid
framestack inspect         # normalized JSON
```

See the [CLI reference](docs/reference/cli.md) and the [manifest reference](docs/reference/manifest.md).

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
├── crates/
│   ├── framestack-core/  # Project model, manifest parsing and validation
│   └── framestack-cli/   # The framestack command
├── fixtures/manifest/    # Shared valid, invalid and warning manifests
├── packages/             # TypeScript packages (introduced with implementation)
├── integrations/         # Optional framework and service adapters
├── examples/             # Runnable reference applications
├── schemas/              # JSON Schema for the normalized manifest
├── website/              # Docusaurus documentation site
├── scripts/              # Maintainer and development scripts
├── PROJECT_STATE.md      # Tracked project status and next actions
└── AGENT_HANDOFF.md      # Local-only agent context; intentionally gitignored
```

Directories for implementation layers will be added as their APIs are defined; the blueprint does not imply that those integrations are already implemented.

## Development

```sh
cargo test --workspace                                # Rust core and CLI
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
pnpm install && pnpm run check                        # documentation site
```

The minimum supported Rust version is 1.85, and CI tests it as well as stable. Node.js follows `.nvmrc`.

The application model, manifest format and validation rules exist. The adapter boundary and the first reference stack (Leptos + Axum, [ADR-0007](docs/decisions/0007-reference-stacks.md)) are next; see [PROJECT_STATE.md](PROJECT_STATE.md).

Read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing architectural or implementation changes.
