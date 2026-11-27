---
description: Commands, options and exit codes of the framestack command-line tool.
---

# CLI reference

`framestack` creates, checks and inspects [`framestack.toml`](manifest.md) manifests, and builds and runs projects through [adapters](adapters.md). Every command a build would run is shown first, and nothing runs without `--yes`.

## Install

From a clone of the repository, with Rust 1.85 or newer:

```sh
cargo install --path crates/framestack-cli
```

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | Success. Warnings may have been printed. |
| 1 | The manifest has an error, the project cannot be planned, a required tool is missing, or a step failed. |
| 2 | The command could not run: a file is missing or unreadable, `init` refused to overwrite, the arguments were wrong, or `build`/`dev` was run without `--yes`. |

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

## framestack plan

Shows what `build` or `dev` would run, without running it.

```sh
framestack plan build|dev [--manifest PATH] [--format text|json]
```

For every step it prints the command, the directory it runs in, the environment variables it reads, and its effects: directories it writes, hosts it contacts, addresses it listens on. Components FrameStack does not build, such as external databases, are listed as not built.

`plan` then checks the tools the steps need by running each tool's version command, such as `cargo leptos --version`, and prints each command it ran with the answer. Those version commands are the only programs `plan` starts. A missing tool is `FS021`, with the install command, and exit code 1.

```text
plan: build (1 step)

app.build  build the server and the WebAssembly client with cargo-leptos
  $ cargo leptos build --release
  in examples/rust-leptos-axum/.
  writes target
  network crates.io

tools
  probe: cargo leptos --version  ->  cargo-leptos 0.3.12
```

## framestack build and framestack dev

Run the plan for `build` or `dev`.

```sh
framestack build [--manifest PATH] [--yes]
framestack dev [--manifest PATH] [--yes]
```

- Without `--yes`, they print the plan, run nothing, and exit with code 2.
- With `--yes`, they check the tools first and stop before any step if one is missing (`FS021`). Then each step runs in order, as a program with arguments (never through a shell), with your terminal attached.
- The first step that fails, is stopped by a signal, or cannot start ends the run with `FS022` and exit code 1. A summary shows each step that ran with `ok`, `failed` or `not started` and how long it took.
- `dev` steps keep running until you stop them, for example with Ctrl-C.
