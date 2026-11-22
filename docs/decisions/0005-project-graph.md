# ADR-0005: Project graph and component relationships

- **Status:** Accepted
- **Date:** 2026-11-22

## Context

FrameStack must describe Rust-only, TypeScript-only, and mixed-language projects. A fixed frontend/backend pair cannot represent projects with multiple frontends, APIs, workers, or external resources. Component kind must also remain separate from the role of a framework or integration.

## Decision

FrameStack v1 uses a directed project graph. The graph contains components and explicitly typed relationships.

### Component kinds

The initial component kinds are:

- `application`: a user-facing or independently runnable application.
- `service`: an independently running API or network service.
- `worker`: a background process or scheduled task runner.
- `resource`: a dependency such as a database or message broker.

These are structural kinds, not framework categories. An integration descriptor supplies the more specific role, ecosystem, and framework identity. For example, React and Fastify are different integrations; Next.js is a distinct full-stack integration and does not require a separate React component.

Each component has:

- A unique, stable project-local ID.
- A component kind.
- A working directory or explicitly declared external location.
- Zero or more integration references.
- Component configuration and environment-variable references.
- Declared capability requirements, where applicable.

Language and runtime metadata are declared when applicable; they are not mandatory for every resource.

### Relationship types

The initial relationship types are:

| Type | Direction and meaning |
| --- | --- |
| `build_dependency` | The source component requires the target component's build output or completed build before it can be built. |
| `runtime_dependency` | The source component requires the target component to be available at runtime. |
| `communicates_with` | The source component communicates with the target component through a declared interface or protocol. |

Relationships are directed. A reciprocal communication relationship is represented by two directed edges when both directions are needed. A communication relationship does not imply build ordering.

### Validation rules

The core validates that:

1. Component IDs are unique.
2. Every relationship endpoint references an existing component.
3. Relationship types are recognized.
4. The `build_dependency` subgraph is acyclic.
5. Required integration and capability references resolve.
6. Component kinds and integration composition rules are respected.

Runtime dependencies and communication edges describe runtime topology; they do not implicitly create build dependencies. Runtime dependency cycles may be reported diagnostically, but are not automatically interpreted as build cycles.

### Integration composition

A component may have more than one integration when those integrations form one coherent application lifecycle. Composition must be explicit in integration metadata.

Underlying libraries and implementation dependencies are not automatically registered as independent application components. For example, a Next.js component may declare its relationship to React in integration metadata without creating a second React component. The same principle applies to Nuxt and Vue.

FrameStack must not start competing application servers merely because an integration is built on another framework. The integration that owns the application lifecycle must be explicit.

## Consequences

- The project model supports multiple applications, services, workers, and resources.
- Build ordering is distinct from runtime availability and communication topology.
- The core can validate graph structure without knowing framework-specific types.
- Integrations can compose without flattening their distinct lifecycle behavior.
- The model can represent Rust-only, TypeScript-only, and mixed-language projects.

## Revisit when

The first reference stacks demonstrate that the component kinds or relationship types cannot express a real project cleanly, or that graph validation requires additional semantics.
