# Leptos + Axum Reference Stack Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** FrameStack plans, builds and runs a real Leptos + Axum application through `framestack plan`, `build` and `dev`, proven in CI.

**Architecture:** `framestack-core` gains framework-free plan types, an `Adapter` trait and `plan_project`. A new workspace crate `integrations/framestack-leptos-axum` plans `cargo leptos` steps. The CLI holds the adapter list, runs tool probes, and executes steps with `std::process::Command` only under `--yes`.

**Tech Stack:** Rust 2024 (FrameStack MSRV 1.85), clap 4, serde; example: Leptos 0.8.23, leptos_axum 0.8.10, Axum 0.8, cargo-leptos 0.3.12, Rust 1.88.

**Spec:** `docs/agent/specs/2026-11-26-leptos-axum-reference-design.md`

## Global Constraints

- `framestack-core` and the adapter crate add no framework dependencies.
- No shell strings in plans: `program` plus `args`.
- Nothing executes without `--yes`; `plan` only starts declared version probes and names them.
- FrameStack crates stay on `rust-version = "1.85"`; the example is its own workspace with `rust-version = "1.88"`, listed in the root `exclude`.
- Exit codes: 0 success, 1 invalid manifest or failed step/preflight, 2 could not run (including `build`/`dev` without `--yes`).
- New diagnostic codes FS019–FS024 exactly as in the spec table.
- Every commit uses the project's conventional format and footer.

## Review Focus

1. `--manifest` pointing at another directory: step `cwd` must resolve against the manifest's directory, not the process working directory (Task 3 test `plan_resolves_cwd_from_the_manifest_directory`).
2. A tool that exists but whose probe fails, such as `cargo` without the `leptos` subcommand ("no such command", exit 101): FS021, not a crash (Task 3 test `probe_failure_is_reported_as_missing_tool`).
3. A step whose program cannot start: FS022 "could not start", not a panic (Task 4 test `unstartable_step_reports_fs022`).
4. A step ended by a signal (Ctrl-C during `dev`): reported as failed with "terminated by signal", exit 1 (Task 4 test `signal_terminated_step_is_a_failure`).
5. A valid manifest with nothing to build (only external resources): plan says "nothing to build", `build --yes` exits 0 (Task 1 test `components_without_integrations_are_skipped`, Task 4 test `empty_plan_succeeds`).

---

### Task 1: Core plan types, `Adapter` trait and `plan_project`

**Files:**

- Create: `crates/framestack-core/src/plan.rs`
- Modify: `crates/framestack-core/src/lib.rs` (module + re-exports), `crates/framestack-core/src/diagnostic.rs` (FS019–FS024)
- Test: `crates/framestack-core/tests/plan.rs` (test adapters defined in this file only)

**Interfaces:**

- Produces:
  - `enum Operation { Build, DevStart }`, serde `"build"` / `"dev.start"`, `fn as_str(self) -> &'static str`
  - `struct ToolRequirement { program: String, version_args: Vec<String>, install_hint: String }`
  - `struct IntegrationDescriptor { id, version: String, handles, provides: Vec<String>, operations: Vec<Operation>, tools: Vec<ToolRequirement> }`
  - `enum Effect { WritesDir(String), Network(String), ListensOn(String) }`, serde `{"writes_dir": ".."}` style (externally tagged, snake_case)
  - `struct Step { id, component, description, program: String, args: Vec<String>, cwd: String, env: Vec<String>, effects: Vec<Effect>, long_running: bool }`
  - `struct Plan { operation: Operation, steps: Vec<Step>, skipped: Vec<String>, tools: Vec<ToolRequirement> }` (tools deduplicated by `program`, in first-use order)
  - `trait Adapter { fn describe(&self) -> IntegrationDescriptor; fn validate(&self, component: &Component) -> Vec<Diagnostic>; fn plan(&self, operation: Operation, component: &Component) -> Result<Vec<Step>, Diagnostic>; }`
  - `fn plan_project(manifest: &Manifest, adapters: &[&dyn Adapter], operation: Operation) -> Result<Plan, Vec<Diagnostic>>`
  - `Code::{CapabilityNotProvided (FS019), NoAdapter (FS020), ToolMissing (FS021), StepFailed (FS022), UnsupportedSetting (FS023), UnsupportedOperation (FS024)}`

- [ ] **Step 1: Write failing tests** in `tests/plan.rs`, with a test adapter `Fake { handles: ["axum","leptos"], provides: ["http.server"], operations: [Build] }` that returns one step per component:
  - `matches_adapter_by_exact_integration_set` — component `["leptos","axum"]` gets the fake's step; order of `integrations` in the manifest does not matter.
  - `partial_integration_set_has_no_adapter` — component `["axum"]` → `Err` containing FS020 at `components[0].integrations`.
  - `declared_capability_must_be_provided_by_adapter` — component provides `frontend.ssr` → FS019 at `components[0].provides[0]`.
  - `unsupported_operation_is_reported` — `Operation::DevStart` → FS024.
  - `components_without_integrations_are_skipped` — external `db` → `plan.skipped == ["db"]`, `plan.steps` empty.
  - `steps_follow_build_order` — two fake components with `web` build-depending on `api` → step components `["api","web"]`.
  - `adapter_validation_errors_stop_planning` — fake whose `validate` returns FS023 → `Err` with FS023.
  - `invalid_manifest_is_not_planned` — manifest with FS004 → `Err` containing FS004.
- [ ] **Step 2:** `cargo test -p framestack-core --test plan` — fails to compile (no `plan` module); add stubs returning `Err(vec![])`, rerun, confirm assertion failures.
- [ ] **Step 3:** Implement in `plan.rs`. Validity via `crate::check`-equivalent `validate()`; components visited in `graph::build_order`; matching compares sorted integration sets.
- [ ] **Step 4:** `cargo test -p framestack-core` — all pass (existing 30 + new).
- [ ] **Step 5:** Commit `feat: add the adapter boundary and project planning to the core`.

### Task 2: Leptos + Axum adapter crate

**Files:**

- Create: `integrations/framestack-leptos-axum/Cargo.toml` (workspace member, depends only on `framestack-core`), `src/lib.rs`
- Modify: root `Cargo.toml` (`members`, `workspace.dependencies`)
- Test: `integrations/framestack-leptos-axum/tests/adapter.rs`

**Interfaces:**

- Consumes: Task 1 types.
- Produces: `pub struct LeptosAxum;` implementing `Adapter`; `pub const ADAPTER_ID: &str = "leptos-axum";`

- [ ] **Step 1: Write failing tests:**
  - `describe_declares_handles_provides_and_tools` — `handles == ["axum","leptos"]`, `provides == ["frontend.ssr","frontend.hydration","http.server"]`, `operations == [Build, DevStart]`, tools programs `["cargo"]` with `version_args == ["leptos","--version"]` and `install_hint == "cargo install cargo-leptos --locked --version 0.3.12"`.
  - `build_plans_cargo_leptos_build_release` — one step: `program "cargo"`, `args ["leptos","build","--release"]`, `cwd` = component path, effects `[WritesDir("target"), Network("crates.io")]`, `long_running false`, id `"<component>.build"`.
  - `dev_plans_cargo_leptos_watch` — `args ["leptos","watch"]`, `long_running true`, effects include `ListensOn("127.0.0.1:3000")`, `env == ["LEPTOS_SITE_ADDR"]`, id `"<component>.dev"`.
  - `settings_are_rejected` — `config.leptos.anything = 1` → FS023 at `components[0].config.leptos`.
  - `external_component_cannot_be_planned` — `plan` on a component with `external` and no `path` returns `Err` with FS024 ("needs a `path` to build").
- [ ] **Step 2:** Run, confirm failures against `todo!`-free stubs (empty descriptor, `Ok(vec![])`).
- [ ] **Step 3:** Implement.
- [ ] **Step 4:** `cargo test --workspace` passes.
- [ ] **Step 5:** Commit `feat: add the Leptos + Axum adapter`.

### Task 3: `framestack plan build|dev` with tool probes

**Files:**

- Create: `crates/framestack-cli/src/adapters.rs` (`fn all() -> Vec<Box<dyn Adapter>>`), `crates/framestack-cli/src/probe.rs`
- Modify: `crates/framestack-cli/src/main.rs` (subcommand), `crates/framestack-cli/src/render.rs` (plan text/JSON), `crates/framestack-cli/Cargo.toml` (adapter dependency)
- Test: `crates/framestack-cli/tests/plan.rs`

**Interfaces:**

- Produces:
  - `struct ToolStatus { program: String, probe: Vec<String> /* full argv */, result: Result<String /* first line of stdout */, String /* reason */> }`
  - `fn probe(tools: &[ToolRequirement]) -> Vec<ToolStatus>` — runs `program version_args…` with stdout captured; non-zero exit or spawn error → `Err(reason)`.
  - `fn project_root(manifest_path: &Path) -> PathBuf` (manifest's parent; `.` when empty)
  - CLI: `framestack plan <build|dev> [--manifest PATH] [--format text|json]`; exit 0 when planned and tools found, 1 on FS019–FS024 or a missing tool.

- [ ] **Step 1: Write failing tests** (temp projects using a copy of the example's `framestack.toml` with `path = "app"`):
  - `plan_build_text_lists_step_command_and_effects` — stdout contains `cargo leptos build --release`, `writes target`, `network crates.io`, `probe: cargo leptos --version`.
  - `plan_json_has_operation_steps_and_tools` — `operation == "build"`, `steps[0].args == ["leptos","build","--release"]`.
  - `plan_resolves_cwd_from_the_manifest_directory` — run from an unrelated cwd with `--manifest <tmp>`; text shows the step directory `<tmp>/app`.
  - `probe_failure_is_reported_as_missing_tool` — `PATH` set to a temp dir holding an executable `cargo` script that prints `error: no such command: leptos` and exits 101 → exit 1, stdout contains `error[FS021]` and the install hint.
  - `missing_tool_is_reported` — `PATH` set to an empty temp dir → exit 1, `error[FS021]`.
  - `plan_of_partial_integrations_reports_fs020` — component with only `axum` → exit 1, `error[FS020]`.
- [ ] **Step 2:** Run; confirm failures (unknown subcommand).
- [ ] **Step 3:** Implement; text plan prints, per step: component, description, `$ program args…`, `in <resolved dir>`, effects; then a tools section with each probe and its result.
- [ ] **Step 4:** `cargo test --workspace` passes.
- [ ] **Step 5:** Commit `feat: add framestack plan with tool checks`.

### Task 4: `framestack build` and `framestack dev` execution

**Files:**

- Create: `crates/framestack-cli/src/execute.rs`
- Modify: `main.rs`, `render.rs`
- Test: `crates/framestack-cli/tests/execute.rs`

**Interfaces:**

- Consumes: Task 3 `probe`, `project_root`, `adapters::all`.
- Produces:
  - `enum StepOutcome { Succeeded, Failed { code: Option<i32>, signal: bool }, NotStarted { reason: String } }`
  - `struct StepResult { id: String, outcome: StepOutcome, duration: Duration }`
  - `fn run(root: &Path, plan: &Plan) -> Vec<StepResult>` — stops after the first non-success.
  - CLI: `framestack build|dev [--manifest PATH] [--yes]`.

Execution tests use the real adapter and executor with `PATH` set to a temp dir holding a fake executable `cargo` script (success, exit 3, `kill -INT $$`). No test-only hooks go into production code. These tests are `#[cfg(unix)]`.

- [ ] **Step 1: Write failing tests:**
  - `build_without_yes_prints_plan_and_exits_2` — stdout contains `cargo leptos build --release` and `--yes`; the fake `cargo` writes a marker file when run; marker absent.
  - `build_yes_runs_the_step_in_the_component_directory` — fake `cargo` writes `$PWD` to a marker; marker == `<tmp>/app`; exit 0; stdout `app.build  ok`.
  - `failing_step_reports_fs022_and_exit_1` — fake exits 3 → `error[FS022]`, `exit code 3`.
  - `unstartable_step_reports_fs022` — fake `cargo` passes the version probe but the component directory `app` does not exist → `could not start`, exit 1.
  - `signal_terminated_step_is_a_failure` — fake runs `kill -INT $$` on `leptos build` → `terminated by signal`, exit 1.
  - `missing_tool_stops_before_any_step` — probe fails → FS021, marker absent, exit 1.
  - `empty_plan_succeeds` — only an external resource → `nothing to build`, exit 0.
  - `dev_yes_runs_cargo_leptos_watch` — fake records args → `leptos watch`.
- [ ] **Step 2:** Run; confirm failures.
- [ ] **Step 3:** Implement: inherit stdio; `status.code()` / `ExitStatusExt::signal()` for the outcome; summary lines `"{id}  ok|failed|not started  ({duration})"`.
- [ ] **Step 4:** `cargo test --workspace` and `cargo +1.85 test --workspace` pass.
- [ ] **Step 5:** Commit `feat: add framestack build and dev`.

### Task 5: Example application

**Files:**

- Create: `examples/rust-leptos-axum/{Cargo.toml, Cargo.lock, framestack.toml, README.md, src/main.rs, src/lib.rs, src/app.rs, style/main.css}` laid out as the cargo-leptos `start-axum` template for Leptos 0.8 (`[package.metadata.leptos]`, features `ssr` and `hydrate`); the component path is `.`
- Modify: root `Cargo.toml` (`exclude = ["examples/rust-leptos-axum"]`)

**Interfaces:**

- Consumes: Tasks 3–4 commands.
- Produces: server routes `GET /` (heading "FrameStack reference: Leptos + Axum" and a button whose text is `Clicked 0 times` on first render) and `GET /healthz` → `ok`.

- [ ] **Step 1:** Install cargo-leptos 0.3.12 locally (approved by the owner).
- [ ] **Step 2:** Write the app and `framestack.toml` (`id = "app"`, `path = "."`, `integrations = ["leptos","axum"]`, `provides = ["frontend.ssr","frontend.hydration","http.server"]`, `env = ["LEPTOS_SITE_ADDR"]`).
- [ ] **Step 3:** `framestack check --manifest examples/rust-leptos-axum` exits 0.
- [ ] **Step 4:** `framestack build --yes --manifest examples/rust-leptos-axum` exits 0; start the server binary from `examples/rust-leptos-axum` with `LEPTOS_SITE_ROOT=target/site`; `curl /` contains `Clicked 0 times`, `curl /healthz` is `ok`, and the `/pkg/*.wasm` referenced by the page returns 200.
- [ ] **Step 5:** Add the example manifest to the CLI tests: `example_manifest_passes_check`.
- [ ] **Step 6:** Commit `feat: add the Leptos + Axum reference application`.

### Task 6: CI job and documentation

**Files:**

- Modify: `.github/workflows/ci.yml` (job `reference-leptos-axum`), `docs/reference/cli.md`, `docs/reference/manifest.md` (FS019–FS024), `README.md`, `PROJECT_STATE.md`
- Create: `docs/reference/adapters.md`, `scripts/smoke-leptos-axum.sh` (starts the server, polls `/healthz` up to 60 s, checks `/`, `/healthz`, wasm; kills the server on exit)

- [ ] **Step 1:** CI job: checkout, `dtolnay/rust-toolchain@stable` with `targets: wasm32-unknown-unknown`, `Swatinem/rust-cache` for the example, cache `~/.cargo/bin/cargo-leptos`, install it if absent, build `framestack`, run `check`, `plan build`, `build --yes`, then `scripts/smoke-leptos-axum.sh`.
- [ ] **Step 2:** Run the smoke script locally against the Task 5 build; exit 0.
- [ ] **Step 3:** Docs. Links to files added in this PR use paths relative to the repository where the site allows it; GitHub links to new files are avoided so the link check passes before merge.
- [ ] **Step 4:** Full verification: fmt, clippy `-D warnings`, `cargo test` on stable and 1.85, rustdoc `-D warnings`, markdownlint, docs site build, YAML parse.
- [ ] **Step 5:** Commit `docs: document plan, build, dev and the Leptos + Axum adapter` (CI change included).
