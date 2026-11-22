---
id: index
title: FrameStack Documentation
slug: /
sidebar_position: 0
---

# FrameStack Documentation

FrameStack is being designed as a framework-composition and integration-orchestration toolkit for full-stack applications. Its project model treats applications, services, workers, and resources as explicit components in a project graph rather than assuming every project is a fixed frontend/backend pair.

The documentation is organized around the architecture and contracts being established. Implementation and compatibility claims will be added only after the relevant features and integrations are implemented and verified.

## Explore the project

- **[Architecture blueprint](./BLUEPRINT.md)** — project scope, system design, candidate integrations, and architectural direction.
- **[Roadmap](./ROADMAP.md)** — planned stages and implementation sequence.
- **[Architecture decisions](./decisions/0001-project-scope.md)** — accepted decisions and the reasoning behind them.
- **[Documentation standards](./documentation/STANDARDS.md)** — requirements for product documentation, examples, references, and releases.

## Current project maturity

FrameStack is in its architecture and foundation stage. The Rust and TypeScript reference stacks are architectural targets, not claims that their adapters are already implemented or supported.

As implementation progresses, this site will grow to include installation instructions, task-oriented guides, integration documentation, CLI and manifest references, API references, architecture internals, and release migration guides.

## Documentation principles

- `docs/` is the canonical source of maintained documentation.
- The Docusaurus application in `website/` renders that source; it does not maintain a competing copy.
- Internal agent handoff material is not part of the public documentation site.
- Release snapshots and migration guides will be published and retained alongside FrameStack releases.
