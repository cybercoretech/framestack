# ADR-0003: Contract-first cross-language interoperability

- **Status:** Proposed
- **Date:** 2026-11-21

## Context

FrameStack is expected to contain Rust and TypeScript components. Duplicating implicit assumptions across languages creates drift in configuration parsing, serialization, validation, and error handling.

## Proposal

Define cross-language data boundaries using versioned, language-neutral contracts before implementing independent language bindings. JSON Schema is the initial candidate for portable data contracts. Shared fixtures should cover valid, invalid, boundary, and compatibility cases.

The schema dialect, validator libraries, code-generation policy, and compatibility rules remain open until the first contract prototype is evaluated.

## Consequences

- Contract changes become reviewable and versioned.
- Rust and TypeScript implementations can be tested against the same fixtures.
- Generated bindings, if adopted, remain derived artifacts rather than the only source of truth.
- Internal implementation types are not automatically public contracts.

## Revisit when

A prototype demonstrates that the chosen schema format cannot express required semantics or creates unacceptable validation differences.
