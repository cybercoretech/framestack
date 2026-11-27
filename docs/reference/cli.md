---
description: Commands, options and exit codes of the framestack command-line tool.
---

# CLI reference

`framestack` creates, checks and inspects [`framestack.toml`](manifest.md) manifests. It does not build or run applications yet; that arrives with the first adapters ([roadmap](../ROADMAP.md)).

## Install

From a clone of the repository, with Rust 1.85 or newer:

```sh
cargo install --path crates/framestack-cli
```

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Success. Warnings may have been printed. |
| 1 | The manifest has at least one error. |
| 2 | The command could not run: a file is missing or unreadable, `init` refused to overwrite, or the arguments were wrong. |

## framestack init

Writes a starter `framestack.toml` into a directory.

```sh
framestack init [DIR] [--name NAME] [--force] [--dry-run]
```

- `DIR` defaults to the current directory. It must already exist; `init` does not create directories.
- `--name` sets the project name. Without it, the directory's name is used.
- `--force` replaces an existing `framestack.toml`. Without it, `init` stops with exit code 2 and changes nothing.
- `--dry-run` prints the manifest instead of writing it.

## framestack check

Validates a manifest and prints every problem found.

```sh
framestack check [--manifest PATH] [--format text|json]
```

- `--manifest` is a manifest file, or a directory containing `framestack.toml`. It defaults to `framestack.toml` in the current directory.
- `--format json` prints `manifest`, `valid` and `diagnostics` as JSON, for scripts and editors.

Text output for an invalid manifest:

```text
error[FS007] relationships: build dependencies form a cycle: a -> b -> a
build-cycle.toml: 1 error, 0 warnings
```

Parse errors include the line and column:

```text
error[FS001]: unknown field `colour`, expected `name` or `description`
  --> unknown-field.toml:7:1
unknown-field.toml: 1 error, 0 warnings
```

The codes are listed in the [manifest reference](manifest.md#validation-rules).

## framestack graph

Shows the components, the relationships and the order components must be built in.

```sh
framestack graph [--manifest PATH] [--format text|json|mermaid]
```

- `text` lists components, relationships and the numbered build order.
- `json` prints `project`, `components`, `relationships` and `build_order`.
- `mermaid` prints a flowchart that GitHub and Docusaurus can render.

In the build order, each component comes after everything it has a `build_dependency` on. Components that could be built at the same point keep their manifest order. An invalid manifest is reported as by `check`, with exit code 1.

## framestack inspect

Prints the [normalized JSON](manifest.md#normalized-json) form of a valid manifest.

```sh
framestack inspect [--manifest PATH]
```
