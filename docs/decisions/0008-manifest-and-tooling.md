# ADR-0008: Manifest, schema, workspace, and licensing conventions

- **Status:** Accepted
- **Date:** 2026-11-22

## Context

FrameStack needs a readable project manifest and language-neutral contracts that can be consumed consistently by its Rust core and TypeScript tooling. It also needs predictable workspace conventions without prematurely pinning unverified toolchain minimums.

## Decision

### Project manifest and normalized contract

- The project manifest filename is `framestack.toml`.
- The manifest is versioned independently through a required manifest format version.
- TOML is parsed into a normalized data model before schema validation.
- The normalized contract is JSON-compatible and language-neutral.
- JSON Schema Draft 2020-12 is the selected schema dialect for the normalized representation.
- Shared valid, invalid, boundary, and compatibility fixtures are consumed by each language implementation.

JSON Schema validates structural shape. The core performs additional semantic validation, including unique component IDs, relationship references, build dependency cycles, integration composition, capability satisfaction, and compatibility rules.

### Unknown fields and extensions

Unknown fields in core-defined sections are rejected so misspellings and unsupported contract features do not silently pass validation.

Integration-specific configuration is namespaced by integration identity and preserved for the relevant adapter to validate. Extension points must be explicit; arbitrary unknown top-level fields are not silently accepted.

Secrets are not stored in the manifest. The manifest may refer to environment variables or other explicitly supported secret sources by reference.

### Workspace and implementation conventions

- The core and initial CLI are implemented in Rust.
- Rust code uses a Cargo workspace.
- TypeScript packages use a pnpm workspace.
- Rust uses the stable toolchain.
- Node.js uses an Active LTS release for the implementation baseline.
- Exact minimum supported Rust and Node.js versions are selected when the first implementation is tested in CI and are recorded in project documentation and toolchain configuration.

No empty crates, packages, or adapters are created solely to match a proposed directory layout.

### License

The project uses the MIT License. The repository license file must identify the chosen project copyright holder and year when it is added.

## Consequences

- TOML remains approachable for project authors while the normalized contract is portable across languages.
- Rust and TypeScript implementations can validate the same fixtures.
- Structural schema validation is clearly separated from semantic graph and compatibility validation.
- Workspace conventions are consistent without claiming untested minimum toolchain support.
- The licensing choice is settled before implementation begins.

## Revisit when

The normalized contract becomes difficult to consume consistently across languages, schema evolution requires a different compatibility model, or verified toolchain constraints require a change to the workspace conventions.
