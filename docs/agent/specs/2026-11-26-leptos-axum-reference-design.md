# Design: Leptos + Axum reference stack (PR 3)

**Status:** Draft for owner approval
**Date:** 2026-11-26

## Intent

FrameStack must do real work, not only describe projects. This slice makes it build and run a real application through its own commands, which is the Stage 2 exit criterion in `docs/ROADMAP.md`: "The reference application runs and builds using documented FrameStack commands", proven in CI. The owner's goal for the repository is a working project; this is the first point where FrameStack runs an app.

## Goals

1. A minimal, real Leptos + Axum application in `examples/rust-leptos-axum/` with server-side rendering and hydration (ADR-0007).
2. An adapter boundary in `framestack-core` that has no framework dependencies (ADR-0002).
3. A Leptos + Axum adapter that plans `build` and `dev.start` (ADR-0006 operation IDs).
4. CLI commands: `framestack plan build|dev`, `framestack build`, `framestack dev`. Execution happens only with `--yes`.
5. Capabilities and required tools are checked before anything runs.
6. A CI job that builds the example through FrameStack and confirms the page is served and hydrates.

## Non-goals

- The TypeScript reference stack (Stage 3) and the mixed-language example.
- Deployment, packaging, release builds for distribution.
- Loading adapters from outside the binary (ADR-0002 forbids third-party code loading for now).
- Other frameworks (Dioxus, Actix, and so on).
- Controlling or replacing cargo-leptos's own hot reload.

## Anti-goals (failure even if it "works")

- FrameStack running any command the user did not see in the plan.
- Running anything without `--yes`.
- `framestack-core` depending on Leptos, Axum or any framework crate.
- Claiming Leptos + Axum support that CI does not prove.
- Raising FrameStack's own minimum Rust version (1.85) because of the example.

## Constraints

- Leptos 0.8.23 and leptos_axum 0.8.10 (latest stable on 2026-11-26) need Rust 1.88. The example therefore has its own Cargo workspace with `rust-version = "1.88"`, excluded from the root workspace, so `cargo test` at the root never compiles Leptos.
- cargo-leptos 0.3.12 (needs Rust 1.82) performs the two-target build: server for the host, client for `wasm32-unknown-unknown`, then wasm-bindgen.
- No shell strings: every step is a program plus an argument list (ADR-0006 execution boundary).
- Plans must show effects: files written, network use, ports opened.

## Approaches considered

| Approach | Trade-off | Decision |
| --- | --- | --- |
| A. Adapter plans one `cargo leptos build` / `cargo leptos watch` step | Standard Leptos path, small adapter; needs cargo-leptos installed locally and in CI; the step is one command, not three | **Chosen by the owner** |
| B. Adapter plans server build, client wasm build and wasm-bindgen itself | Nothing new to install, every compile visible; wasm-bindgen CLI must exactly match the crate version, and FrameStack would duplicate cargo-leptos | Rejected: fragile version pin, duplicated tooling |

## Design

### Core: adapter boundary (`framestack-core`, no new dependencies)

New public types, all serializable to JSON:

- `Operation`: `Build` (`"build"`) and `DevStart` (`"dev.start"`).
- `IntegrationDescriptor`: `id` (adapter ID, for example `leptos-axum`), `version`, `handles` (the exact set of integration IDs whose lifecycle it owns, here `["axum", "leptos"]`), `provides` (capabilities), `operations`, and `tools` (required external tools, each with a program, version argument and install hint).
- `Plan`: the operation and an ordered list of `Step`s, grouped by component in build order.
- `Step`: `id`, `component`, `description`, `program`, `args`, `cwd` (project-relative), `env` (names only), `effects`, `long_running`.
- `Effect`: `WritesDir(path)`, `Network(host)`, `ListensOn(address)`.
- Trait `Adapter`: `describe() -> IntegrationDescriptor`, `validate(&Component) -> Vec<Diagnostic>`, `plan(Operation, &Component) -> Result<Vec<Step>, Diagnostic>`.

New core function `plan_project(&Manifest, &[&dyn Adapter], Operation) -> Result<Plan, Vec<Diagnostic>>`:

1. The manifest must already be valid (`check`).
2. Each component with integrations is matched to the adapter whose `handles` set equals the component's integration set. A component with no integrations (such as an external database) is not built by FrameStack and appears in the plan as skipped.
3. Before planning: the component's `provides` must be a subset of the adapter's `provides`, and the adapter must support the operation.
4. The adapter validates the component, then plans its steps. Components are visited in build order.

New diagnostic codes:

| Code | Rule |
| --- | --- |
| FS019 | A component declares a capability its adapter does not provide. |
| FS020 | No adapter handles this component's integrations. |
| FS021 | A required tool is missing or cannot report its version. |
| FS022 | A step failed (non-zero exit or could not start). |
| FS023 | An integration setting is not supported by its adapter. |
| FS024 | The adapter does not support the requested operation. |

### Adapter: `integrations/framestack-leptos-axum` (workspace crate)

Plans commands only; it does not depend on Leptos.

- `handles = ["axum", "leptos"]`, `provides = ["frontend.ssr", "frontend.hydration", "http.server"]`.
- Tools: `cargo` and `cargo-leptos` (checked with `cargo leptos --version`, install hint `cargo install cargo-leptos --locked --version 0.3.12`).
- `build`: one step, `cargo leptos build --release`, in the component's directory. Effects: writes `target/`, network `crates.io`.
- `dev.start`: one long-running step, `cargo leptos watch`. Effects: writes `target/`, network `crates.io`, listens on the address in `LEPTOS_SITE_ADDR` (default `127.0.0.1:3000`).
- `validate`: `config.leptos` and `config.axum` must be empty in this version (FS023), so settings are never silently ignored.

### CLI

- `framestack plan build|dev [--manifest PATH] [--format text|json]` prints the plan, preflight results (tools found, with versions), and effects. It never runs a step. The only programs it starts are the declared version probes (such as `cargo leptos --version`), and the output names each probe it ran.
- `framestack build [--manifest PATH] [--yes]` and `framestack dev [--manifest PATH] [--yes]`:
  - Without `--yes`: print the plan and "re-run with --yes to execute", exit code 2 (nothing ran).
  - With `--yes`: run the tool preflight; stop with FS021 if a tool is missing. Then run each step with `std::process::Command` (no shell), in its `cwd` resolved inside the project root, inheriting stdin, stdout, stderr and the environment. Stop at the first failed step with FS022.
  - Print a result summary per step (status, exit code, duration). Exit 0 if all steps succeed, 1 on a failed step or preflight problem.
- Adapters are compiled in: the CLI holds a fixed list containing the Leptos + Axum adapter.
- The project root is the directory containing the manifest.

### Example application (`examples/rust-leptos-axum/`)

- Its own workspace, `rust-version = "1.88"`, Leptos 0.8.23, leptos_axum 0.8.10, Axum 0.8, with `Cargo.lock` committed.
- One page: a heading and a counter button. The server renders the initial HTML; the counter only works once the WebAssembly hydrates, which proves hydration.
- One plain Axum route, `GET /healthz`, returning `ok`.
- `framestack.toml` with one `application` component, `integrations = ["leptos", "axum"]`, `provides` matching the adapter, `env = ["LEPTOS_SITE_ADDR"]`.

### Tests

- Core: `plan_project` with a test adapter (matching, skipped components, FS019, FS020, FS024, build order of steps). Test adapters live in the test files only.
- Adapter: literal expected steps for `build` and `dev.start`; FS023 for settings.
- CLI: `plan` output (text and JSON) for the example manifest; `build` without `--yes` exits 2 and runs nothing; FS021 when the tool is missing (by pointing PATH at an empty directory); step execution and FS022 using a harmless real program in a temporary project. No test runs cargo-leptos; that is the CI job's work.
- Shared fixture: the example's `framestack.toml` must pass `framestack check`.

### CI

New job `Rust reference stack (Leptos + Axum)`, on every pull request and push to `main`:

1. Stable Rust with the `wasm32-unknown-unknown` target, Rust build cache.
2. Install cargo-leptos 0.3.12 (`--locked`), cached between runs.
3. `framestack check`, `framestack plan build`, `framestack build --yes` on the example.
4. Start the built server, then check that `GET /` contains the server-rendered counter, `GET /healthz` returns `ok`, and the page's WebAssembly file is served.

### Documentation

- `docs/reference/cli.md`: `plan`, `build`, `dev`, `--yes`, the new exit-code cases.
- `docs/reference/adapters.md`: the Leptos + Axum adapter, what it needs, what it runs.
- `docs/reference/manifest.md`: codes FS019 to FS024.
- `examples/rust-leptos-axum/README.md`: how to run it with FrameStack.
- `PROJECT_STATE.md` and README status.

## Calls made by the author (not decided by the owner)

- Adapter matching is by exact integration set, so a component listing only `axum` does not match the Leptos + Axum adapter.
- `framestack build` without `--yes` exits 2.
- The adapter accepts no settings yet (FS023) rather than ignoring them.
- The example is excluded from the root workspace and has its own MSRV of 1.88.
- The CI job runs on every pull request rather than only when related paths change, because it is the proof that support is real.
- The adapter crate lives in `integrations/`, which the README already reserves for adapters.

## Open for the owner

- Approval to install cargo-leptos 0.3.12 on this machine (`cargo install cargo-leptos --locked --version 0.3.12`), so the build can be verified locally before CI.

## Left to the builder

Exact module layout, test names, output wording, and the HTML of the example page.
