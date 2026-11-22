# ADR-0007: Official reference stacks and verification strategy

- **Status:** Accepted
- **Date:** 2026-11-22

## Context

FrameStack needs working reference applications to test its project graph, integration descriptors, capability matching, operation planning, and execution boundary. The first stacks should exercise materially different development and runtime models rather than maximize the number of integrations.

## Decision

The first two official reference stacks are:

### Rust reference: Leptos + Axum

- Leptos provides the Rust frontend and rendering integration.
- Axum provides the HTTP server and application integration.
- Cargo manages the Rust workspace and dependency graph.
- The initial reference exercises server-side rendering and hydration.
- The Leptos/Axum composition must have one explicit application lifecycle and must not start competing server instances.

### TypeScript reference: React + Vite + Fastify

- React provides the browser UI.
- Vite provides frontend development and production build tooling.
- Fastify provides the independent HTTP API service.
- TypeScript is used across the frontend and backend.
- pnpm manages the TypeScript workspace.
- The project graph represents the frontend and API as separate components with an explicit communication relationship.

Vite is represented as a frontend build/development tool integration, not as a UI framework or backend service.

### Later mixed-language proof: React + Vite + Axum

After the Rust and TypeScript reference slices work independently, a mixed-language example will combine a React/Vite frontend with an Axum service. It will verify independent toolchains, explicit build and runtime relationships, and cross-language communication through a documented protocol.

This mixed-language example is a later verification milestone, not a prerequisite for the first Rust vertical slice.

## Verification requirements

A reference stack is considered verified only when:

- Its manifest parses and validates against the versioned contract.
- Its project graph and integration composition validate.
- Its required capabilities resolve.
- Its documented development and build operations produce structured plans.
- The reference application runs and builds through the documented FrameStack workflow.
- Adapter contract tests and reference-application CI pass.
- Diagnostics identify missing tools, incompatible versions, and unsatisfied requirements.

A stack's selection does not mean its adapters are already implemented or supported.

## Consequences

- The Rust reference proves an integrated frontend/server composition.
- The TypeScript reference proves separate frontend and API components.
- The later mixed-language example proves that the same project model works across Rust and TypeScript.
- Additional frameworks remain candidates until individually implemented and verified.

## Revisit when

A selected upstream framework or toolchain cannot meet the reference requirements, or implementation evidence shows that a different stack would test the architecture more effectively.
