# Reference application: Leptos + Axum

The Rust reference stack from [ADR-0007](../../docs/decisions/0007-reference-stacks.md): one Leptos application, rendered on the server by Axum and hydrated in the browser. FrameStack builds and runs it through the Leptos + Axum adapter, which plans `cargo-leptos` commands.

The page shows a counter button. The server sends it as `Clicked 0 times`; the button only counts once the WebAssembly client has loaded, which shows hydration works. `GET /healthz` is a plain Axum route that returns `ok`.

## Requirements

- Rust 1.88 or newer (Leptos 0.8 needs it; FrameStack itself needs 1.85)
- The WebAssembly target: `rustup target add wasm32-unknown-unknown`
- cargo-leptos 0.3.12: `cargo install cargo-leptos --locked --version 0.3.12`

## Run it with FrameStack

From the repository root:

```sh
framestack check --manifest examples/rust-leptos-axum
framestack plan build --manifest examples/rust-leptos-axum         # what would run
framestack build --yes --manifest examples/rust-leptos-axum        # build it
framestack dev --yes --manifest examples/rust-leptos-axum          # serve on 127.0.0.1:3000, rebuild on changes
scripts/smoke-leptos-axum.sh                                       # check a release build serves correctly
```

## wasm-bindgen version

The `wasm-bindgen` crate is pinned to `=0.2.129` in `Cargo.toml`. cargo-leptos runs the `wasm-bindgen` command-line tool, and the crate and the tool must be exactly the same version. If a different `wasm-bindgen` tool is installed on your `PATH`, the build fails with a schema version error. Update the pin and the tool together.

This is a separate Cargo workspace, so `cargo test` at the repository root does not compile it.
