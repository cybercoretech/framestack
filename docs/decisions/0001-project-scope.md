# ADR-0001: FrameStack project scope

- **Status:** Proposed
- **Date:** 2026-11-22

## Context

FrameStack is intended to support a multi-language full-stack development workflow. The target ecosystem includes Rust-native frontends, TypeScript frontend frameworks and meta-frameworks, multiple backend and application frameworks, and more than one communication protocol.

A single-framework or single-language identity would unnecessarily constrain the project. At the same time, attempting to implement every candidate integration at once would create an untestable scope.

Applications may contain multiple frontends, backends, workers, shared packages, and services. They may use different languages, runtimes, protocols, and deployment models. A fixed frontend/backend pair is therefore too restrictive as the project model.

## Proposal

FrameStack should be a modular framework and development ecosystem that coordinates existing technologies through:

- A language-neutral project model and component graph.
- A coherent CLI and manifest.
- Explicit, independently testable adapters.
- Versioned cross-language contracts and shared fixtures.
- A capability model that describes supported operations and constraints.
- A staged integration catalog that distinguishes candidates from verified support.

The initial candidate catalog is organized by architectural role:

### Rust

- Frontend: Leptos, Dioxus, Yew.
- HTTP/backend: Axum, Actix Web, Poem.
- Full-stack application framework: Loco.
- RPC: Tonic.

### TypeScript

- Frontend: React, Svelte, Vue.
- Full-stack frontend meta-frameworks: Next.js, Nuxt.
- Backend/HTTP: Fastify, Hono, Express, NestJS, AdonisJS.
- API and real-time services: Feathers.

Protocols and transports such as HTTP/REST, gRPC, WebSocket, GraphQL, and Server-Sent Events are modeled separately from framework categories.

These entries are candidates for evaluation, not implemented or supported integrations. An integration becomes supported only after its reference example, adapter contract tests, compatibility checks, and required CI workflows pass.

## Consequences

- The core must remain independent of individual frameworks and vendors.
- The project model must represent a graph of components and relationships rather than a mandatory frontend/backend pair.
- Integrations must be optional, role-classified, capability-described, and versioned.
- Frameworks with overlapping foundations must retain distinct identities and lifecycle requirements.
- Protocols and transports must not be confused with application frameworks.
- The first release should support a small, well-tested matrix rather than claim universal compatibility.
- The candidate catalog may be broad while the initial implementation remains deliberately narrow.

## Revisit when

The first reference applications reveal that the project boundary is too broad, the project graph cannot represent required compositions, or the integration taxonomy creates distinctions that are not useful in practice.
