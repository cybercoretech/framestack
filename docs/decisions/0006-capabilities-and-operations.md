# ADR-0006: Capability vocabulary and adapter operations

- **Status:** Accepted
- **Date:** 2026-11-22

## Context

FrameStack needs to determine whether integrations can provide the behavior a project requires. Framework categories alone do not express concrete behavior, and a single lowest-common-denominator adapter interface would erase meaningful differences.

Capabilities also need to remain distinct from operations that FrameStack can ask an adapter to perform.

## Decision

Capabilities are structured identifiers. Integration descriptors declare capabilities they provide; integrations and components may declare capabilities they require. Capability constraints describe supported variants and limitations.

### Initial capability vocabulary

The initial vocabulary is intentionally small:

| Family | Capability identifiers |
| --- | --- |
| Frontend | `frontend.browser`, `frontend.spa`, `frontend.ssr`, `frontend.ssg`, `frontend.hydration` |
| HTTP | `http.server`, `http.api`, `http.middleware` |
| API | `api.rest`, `api.graphql` |
| RPC | `rpc.grpc` |
| Realtime | `realtime.websocket`, `realtime.sse` |
| Background | `worker.background` |
| Persistence | `persistence.sql`, `persistence.nosql` |
| Messaging | `messaging.publish`, `messaging.subscribe` |

This vocabulary does not assert that every capability has an adapter implementation in v1. New identifiers are added when a reference integration or concrete project requirement demonstrates a need.

### Capability declarations

An integration descriptor may declare:

- `provides`: capabilities the integration implements.
- `requires`: capabilities that must be satisfied by another integration, component, or explicitly declared external dependency.
- `constraints`: structured parameters and limitations attached to provided or required capabilities.

Constraints must be machine-readable and validated. Examples include supported runtime targets, rendering modes, protocol versions, and framework version ranges. Free-form descriptions may supplement metadata but do not replace structured constraints.

Capability matching must not infer support from an integration's name or broad category. A requirement is satisfied only when a declared provider is compatible with the requirement and its constraints.

### Operations are separate

Operations describe work an adapter can perform; they are not application capabilities. The initial operation identifiers are:

- `inspect`
- `check`
- `dev.start`
- `dev.stop`
- `build`
- `test`

The adapter lifecycle is:

1. `discover`: identify applicable integrations and adapter versions.
2. `describe`: report identity, dependencies, compatibility, capabilities, and operations.
3. `validate`: validate integration configuration and project requirements.
4. `plan`: return a structured operation plan.
5. `execute`: perform an explicitly approved plan.
6. `report`: return normalized results, diagnostics, and declared artifacts.

The core coordinates this lifecycle. Framework-specific build and development behavior remains in adapters.

### Execution boundary

Plans use structured operation identifiers, typed inputs, declared working directories, environment references, and explicit effects. The core must not evaluate arbitrary shell strings supplied by an adapter as an implicit execution mechanism. Any process invocation must pass through the project's explicit execution and permission boundary.

## Consequences

- Capability compatibility can be checked before an operation is executed.
- Integrations can preserve their own behavior while participating in a common discovery and planning lifecycle.
- Operations can evolve independently of the capability vocabulary.
- The initial vocabulary remains small and evidence-driven.
- The execution boundary is explicit and testable.

## Revisit when

Reference adapters expose capability requirements that cannot be represented with structured identifiers and constraints, or when the operation lifecycle proves insufficient for a real workflow.
