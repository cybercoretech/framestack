---
description: How FrameStack hands components to adapters, and what the Leptos + Axum adapter needs and runs.
---

# Adapters

An adapter knows how to build and run one kind of component. FrameStack's core decides which adapter owns each component, checks capabilities before anything runs, and collects the adapter's steps into a plan ([ADR-0002](../decisions/0002-core-and-adapters.md)). Adapters only plan; `framestack build` and `framestack dev` run the plan, and only with `--yes`.

Adapters are compiled into `framestack`. Loading adapters from elsewhere is out of scope until a trust model exists.

## How a component gets its adapter

A component is handled by the adapter whose integration set is exactly the component's `integrations`, in any order. A component that lists only `axum` is not handled by the Leptos + Axum adapter.

Before planning, FrameStack checks:

| Code | Check |
| --- | --- |
| FS020 | Some adapter handles the component's integrations. |
| FS019 | Every capability in the component's `provides` is one its adapter provides. |
| FS024 | The adapter can perform the operation for this component. |
| FS023 | The adapter accepts the component's `config` settings. |

Components with no `integrations`, such as an external database, are not built by FrameStack and are listed as such in the plan.

## Leptos + Axum

Adapter ID `leptos-axum`, for components with `integrations = ["leptos", "axum"]`: a Leptos application with server-side rendering and hydration, served by Axum, built with [cargo-leptos](https://github.com/leptos-rs/cargo-leptos).

| | |
| --- | --- |
| Provides | `frontend.ssr`, `frontend.hydration`, `http.server` |
| Operations | `build`, `dev.start` |
| Needs | cargo-leptos 0.3.12 (`cargo install cargo-leptos --locked --version 0.3.12`), the `wasm32-unknown-unknown` Rust target, and the Rust version the application itself needs (Leptos 0.8 needs 1.88) |
| Settings | None yet. Any `config.leptos` or `config.axum` table is reported as `FS023` rather than ignored. |

What it runs, in the component's `path`:

| Operation | Command | Effects |
| --- | --- | --- |
| `build` | `cargo leptos build --release` | writes `target`, network `crates.io` |
| `dev.start` | `cargo leptos watch` (runs until stopped, reads `LEPTOS_SITE_ADDR`) | writes `target`, network `crates.io`, listens on `127.0.0.1:3000` by default |

The tool check runs `cargo leptos --version`.

The `wasm-bindgen` crate in the application must be exactly the same version as the `wasm-bindgen` command-line tool cargo-leptos uses. When they differ, the build fails with a schema version error from wasm-bindgen.

A complete working application is in `examples/rust-leptos-axum/` in the repository, and CI builds it through FrameStack on every change.
