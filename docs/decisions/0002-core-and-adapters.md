# ADR-0002: Small core, explicit adapters, and planned execution

- **Status:** Proposed
- **Date:** 2026-11-22

## Context

Frontend frameworks, backend frameworks, full-stack application frameworks, and protocol implementations have different lifecycle models, build systems, routing semantics, rendering capabilities, runtime requirements, and deployment assumptions.

A universal abstraction that erases those differences would be leaky and difficult to maintain. A catalog category also cannot be treated as a promise that every member supports the same operations.

FrameStack needs a stable coordination model without absorbing framework-specific build logic or granting integrations implicit execution authority.

## Proposal

Keep the core responsible for:

- The language-neutral manifest and project model.
- Component and dependency graph validation.
- Integration identity and compatibility resolution.
- Capability resolution.
- Operation planning and lifecycle coordination.
- Normalized diagnostics, task results, and artifacts.
- Policy enforcement and explicit execution boundaries.

Put framework- and protocol-specific behavior in adapters with explicit identities, versions, compatibility ranges, dependencies, capabilities, validation, planning, execution, and reporting responsibilities.

The core must not depend on framework-specific crates or packages.

### Adapter lifecycle

The conceptual adapter lifecycle is:

1. **Discover** applicable integrations.
2. **Describe** identity, version, compatibility, dependencies, and capabilities.
3. **Validate** adapter-specific configuration and project requirements.
4. **Plan** a requested operation and declare its requirements and effects.
5. **Execute** an explicitly approved operation.
6. **Report** normalized results, diagnostics, and declared artifacts.

The initial implementation may use in-process adapters and child-process adapters. It must not introduce arbitrary third-party code loading before a trust, permission, and isolation model is established.

### Capability semantics

Capabilities are explicit, structured declarations with constraints. The core must validate requested capabilities against the selected integration and composition before planning an operation.

Capabilities should not be inferred from an integration's category or from another integration on which it is built. For example, a full-stack meta-framework requires its own adapter and capability declarations even when it builds on a UI framework.

### Composition semantics

Integration metadata may declare that an integration:

- Builds on another integration.
- Requires another integration.
- Provides or consumes a protocol.
- Conflicts with another integration.
- Replaces or supersedes another integration.

These relationships must not automatically register duplicate runtimes or competing application servers. Composition behavior must be explicit and testable.

## Consequences

- Adapters can be developed and tested independently.
- Unsupported capabilities and incompatible compositions can be reported clearly.
- Framework-specific escape hatches remain possible without contaminating core types.
- Adapter plans make command execution, filesystem writes, network access, and environment requirements reviewable.
- The adapter contract must be proven against at least two materially different integrations before it is stabilized.
- The core does not promise identical lifecycle behavior across frameworks.
- The initial adapter system remains intentionally limited; a remote plugin marketplace and arbitrary plugin execution are out of scope.

## Revisit when

The first frontend, backend, or protocol adapters expose incompatible lifecycle requirements that cannot be represented cleanly by the proposed boundary, or when the initial execution model proves insufficient for a real reference application.
