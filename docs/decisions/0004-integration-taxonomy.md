# ADR-0004: Integration taxonomy and capability metadata

- **Status:** Proposed
- **Date:** 2026-11-22

## Context

FrameStack is expected to coordinate a broad ecosystem of Rust and TypeScript technologies. These technologies do not all occupy the same architectural role.

Some are UI libraries or frontend frameworks; some are full-stack meta-frameworks; some are HTTP servers or backend application frameworks; some are RPC or service frameworks; and some are protocols or transports.

Treating every entry as a generic frontend or backend would obscure important differences. Conversely, forcing all integrations in a category to expose identical behavior would create a lowest-common-denominator adapter API.

FrameStack also needs to distinguish an integration that is merely being considered from one that has a working, verified adapter.

## Proposal

Classify integrations by ecosystem, primary role, and composition model. Categories are for discovery and organization; they do not define a shared feature set or guarantee compatibility.

### Initial candidate taxonomy

#### Rust

| Category | Candidate integrations |
| --- | --- |
| Frontend frameworks | Leptos, Dioxus, Yew |
| HTTP/backend frameworks | Axum, Actix Web, Poem |
| Full-stack application frameworks | Loco |
| RPC frameworks | Tonic |

#### TypeScript

| Category | Candidate integrations |
| --- | --- |
| Frontend frameworks and UI libraries | React, Svelte, Vue |
| Full-stack frontend meta-frameworks | Next.js, Nuxt |
| Backend and HTTP frameworks | Fastify, Hono, Express, NestJS, AdonisJS |
| API and real-time service frameworks | Feathers |

#### Protocols and transports

Protocol and transport capabilities are classified separately from language-specific application frameworks. Initial candidate capability families include HTTP/REST, gRPC/RPC, WebSocket, GraphQL, Server-Sent Events, and messaging/event-broker transports.

A protocol may be implemented by a framework adapter or provided through a dedicated adapter. Ownership, composition, and capability reporting must be explicit.

### Integration lifecycle status

Every catalog entry must have one lifecycle status:

- `candidate`: identified for evaluation; no support claim.
- `experimental`: an adapter or prototype exists but is not production-supported.
- `supported`: reference example, contract tests, compatibility checks, and required CI pass.
- `deprecated`: support remains temporarily but migration is recommended.
- `removed`: no longer available in the current integration catalog.

Status changes require evidence and documentation. A catalog entry must not be marked supported solely because its upstream framework is mature or popular.

### Integration metadata

An integration descriptor should eventually include:

- Stable integration ID and display name.
- Ecosystem and primary role.
- Lifecycle status.
- FrameStack adapter version.
- Supported manifest contract and core version ranges.
- Upstream framework and runtime compatibility.
- Dependencies, composition relationships, and conflicts.
- Structured capability identifiers and constraints.
- Configuration requirements.
- Detection, validation, planning, execution, and reporting operations.
- Required tools, environment variables, and external services.
- Reference examples, contract tests, CI evidence, and verification date.

Capabilities should be structured identifiers with optional parameters and constraints. They must describe concrete behavior, not broad marketing claims. The initial vocabulary should remain small and grow in response to working integrations.

### Composition examples

- Next.js is a distinct full-stack adapter built around React; it is not merely a React alias.
- Nuxt is a distinct full-stack adapter built around Vue; it is not merely a Vue alias.
- Loco is a distinct application-framework adapter built on the Axum ecosystem; composition must not accidentally start competing server instances.
- Tonic is an RPC integration and does not replace a browser-facing HTTP application framework.
- Feathers is a service-oriented integration and must not be reduced to an HTTP-only label.

These relationships are descriptive starting points. Exact dependency and capability metadata must be verified against the versions supported by each adapter.

## Consequences

- The catalog can represent a broad ecosystem without claiming broad implementation support.
- Integration discovery becomes clearer and more useful.
- Adapters preserve framework-specific behavior and lifecycle differences.
- Composition and dependency relationships become explicit and testable.
- CI evidence, not catalog inclusion, determines supported status.
- Protocols and transports remain distinct from framework integrations.
- The capability vocabulary can evolve incrementally instead of being over-designed before implementation.

## Revisit when

The first reference integrations reveal that the taxonomy is confusing, the lifecycle statuses do not communicate support accurately, or the metadata model cannot express real composition and capability constraints.
