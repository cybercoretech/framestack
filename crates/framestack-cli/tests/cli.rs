//! End-to-end tests of the `framestack` binary.
//!
//! Exit codes: 0 = success, 1 = the manifest is invalid, 2 = the command could
//! not run (missing file, refused overwrite, bad arguments).

use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/manifest")
        .join(name)
}

fn framestack() -> Command {
    Command::cargo_bin("framestack").expect("binary is built")
}

/// Runs `framestack <args>` and returns (exit code, stdout).
fn run(args: &[&str]) -> (i32, String) {
    let output = framestack().args(args).output().expect("framestack runs");
    (
        output.status.code().expect("exit code"),
        String::from_utf8(output.stdout).expect("UTF-8 output"),
    )
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("UTF-8 path")
}

// ---- version and help ---------------------------------------------------

#[test]
fn version_matches_the_crate_version() {
    framestack()
        .arg("--version")
        .assert()
        .success()
        .stdout("framestack 0.1.0\n");
}

#[test]
fn unknown_subcommand_is_a_usage_error() {
    framestack().arg("frobnicate").assert().code(2);
}

// ---- check -------------------------------------------------------------

#[test]
fn check_accepts_a_valid_manifest() {
    let manifest = fixture("valid/react-vite-fastify.toml");
    framestack()
        .args(["check", "--manifest", path_str(&manifest)])
        .assert()
        .code(0)
        .stdout(predicate::str::contains(
            "valid: 2 components, 1 relationship",
        ));
}

#[test]
fn check_reports_an_invalid_manifest_with_exit_code_1() {
    let manifest = fixture("invalid/build-cycle.toml");
    framestack()
        .args(["check", "--manifest", path_str(&manifest)])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS007] relationships"))
        .stdout(predicate::str::contains("a -> b -> a"))
        .stdout(predicate::str::contains("1 error, 0 warnings"));
}

#[test]
fn check_prints_the_line_of_a_parse_error() {
    let manifest = fixture("invalid/unknown-field.toml");
    framestack()
        .args(["check", "--manifest", path_str(&manifest)])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS001]"))
        .stdout(predicate::str::contains("unknown-field.toml:7:1"));
}

#[test]
fn check_warnings_do_not_fail_the_check() {
    let manifest = fixture("warning/runtime-cycle.toml");
    framestack()
        .args(["check", "--manifest", path_str(&manifest)])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("warning[FS015]"))
        .stdout(predicate::str::contains("0 errors, 1 warning"));
}

#[test]
fn check_json_lists_diagnostic_codes() {
    let manifest = fixture("invalid/duplicate-component-id.toml");
    let (code, stdout) = run(&[
        "check",
        "--manifest",
        path_str(&manifest),
        "--format",
        "json",
    ]);
    let report: Value = serde_json::from_str(&stdout).expect("JSON output");

    assert_eq!(code, 1);
    assert_eq!(report["valid"], false);
    assert_eq!(report["diagnostics"][0]["code"], "FS004");
    assert_eq!(report["diagnostics"][0]["path"], "components[1].id");
}

#[test]
fn check_reads_framestack_toml_from_a_directory() {
    let dir = tempfile::tempdir().unwrap();
    fs::copy(
        fixture("valid/minimal.toml"),
        dir.path().join("framestack.toml"),
    )
    .unwrap();

    framestack()
        .args(["check", "--manifest", path_str(dir.path())])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("framestack.toml: valid"));
}

#[test]
fn check_defaults_to_framestack_toml_in_the_current_directory() {
    let dir = tempfile::tempdir().unwrap();
    fs::copy(
        fixture("valid/minimal.toml"),
        dir.path().join("framestack.toml"),
    )
    .unwrap();

    framestack()
        .current_dir(dir.path())
        .arg("check")
        .assert()
        .code(0)
        .stdout(predicate::str::contains("framestack.toml: valid"));
}

#[test]
fn check_of_a_missing_file_exits_2_and_names_the_path() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("nope.toml");

    framestack()
        .args(["check", "--manifest", path_str(&missing)])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("nope.toml"));
}

// ---- inspect -----------------------------------------------------------

#[test]
fn inspect_prints_the_normalized_manifest() {
    let manifest = fixture("valid/minimal.toml");
    let (code, stdout) = run(&["inspect", "--manifest", path_str(&manifest)]);
    let document: Value = serde_json::from_str(&stdout).expect("JSON output");

    assert_eq!(code, 0);
    assert_eq!(document["components"][0]["id"], "app");
    assert_eq!(document["components"][0]["requires"], serde_json::json!([]));
    assert_eq!(document["relationships"], serde_json::json!([]));
}

#[test]
fn inspect_refuses_an_invalid_manifest() {
    let manifest = fixture("invalid/unknown-capability.toml");
    framestack()
        .args(["inspect", "--manifest", path_str(&manifest)])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("error[FS008]"));
}

// ---- graph -------------------------------------------------------------

#[test]
fn graph_text_shows_relationships_and_build_order() {
    let manifest = fixture("valid/react-vite-axum.toml");
    let (code, stdout) = run(&["graph", "--manifest", path_str(&manifest)]);

    assert_eq!(code, 0);
    assert!(
        stdout.contains("web -> api  communicates_with (http)"),
        "{stdout}"
    );
    assert!(
        stdout.contains("web -> contracts  build_dependency"),
        "{stdout}"
    );
    // contracts has no build dependencies; web and api both need it.
    assert!(
        stdout.contains("1. contracts\n  2. web\n  3. api"),
        "{stdout}"
    );
}

#[test]
fn graph_mermaid_draws_one_edge_per_relationship() {
    let manifest = fixture("valid/react-vite-fastify.toml");
    let (code, stdout) = run(&[
        "graph",
        "--manifest",
        path_str(&manifest),
        "--format",
        "mermaid",
    ]);

    assert_eq!(code, 0);
    assert!(stdout.starts_with("flowchart LR\n"), "{stdout}");
    assert!(stdout.contains("  web[\"web (application)\"]"), "{stdout}");
    assert!(
        stdout.contains("  web -->|communicates_with| api"),
        "{stdout}"
    );
}

#[test]
fn graph_json_includes_the_build_order() {
    let manifest = fixture("valid/react-vite-axum.toml");
    let (code, stdout) = run(&[
        "graph",
        "--manifest",
        path_str(&manifest),
        "--format",
        "json",
    ]);
    let graph: Value = serde_json::from_str(&stdout).expect("JSON output");

    assert_eq!(code, 0);
    assert_eq!(
        graph["build_order"],
        serde_json::json!(["contracts", "web", "api"])
    );
    assert_eq!(graph["relationships"].as_array().unwrap().len(), 3);
}

#[test]
fn graph_refuses_an_invalid_manifest() {
    let manifest = fixture("invalid/build-cycle.toml");
    framestack()
        .args(["graph", "--manifest", path_str(&manifest)])
        .assert()
        .code(1);
}

// ---- init --------------------------------------------------------------

#[test]
fn init_writes_a_manifest_that_passes_check() {
    let dir = tempfile::tempdir().unwrap();

    framestack()
        .args(["init", path_str(dir.path()), "--name", "demo"])
        .assert()
        .code(0);

    let written = fs::read_to_string(dir.path().join("framestack.toml")).unwrap();
    assert!(written.contains("name = \"demo\""), "{written}");
    framestack()
        .args(["check", "--manifest", path_str(dir.path())])
        .assert()
        .code(0);
}

#[test]
fn init_names_the_project_after_its_directory_by_default() {
    let parent = tempfile::tempdir().unwrap();
    let dir = parent.path().join("Night Shift");
    fs::create_dir(&dir).unwrap();

    framestack().args(["init", path_str(&dir)]).assert().code(0);

    let written = fs::read_to_string(dir.join("framestack.toml")).unwrap();
    assert!(written.contains("name = \"Night Shift\""), "{written}");
}

#[test]
fn init_dry_run_prints_the_manifest_and_writes_nothing() {
    let dir = tempfile::tempdir().unwrap();

    framestack()
        .args(["init", path_str(dir.path()), "--name", "demo", "--dry-run"])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("manifest_version = 1"));

    assert!(!dir.path().join("framestack.toml").exists());
}

#[test]
fn init_refuses_to_overwrite_an_existing_manifest() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("framestack.toml");
    fs::write(&target, "keep me\n").unwrap();

    framestack()
        .args(["init", path_str(dir.path())])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--force"));

    assert_eq!(fs::read_to_string(&target).unwrap(), "keep me\n");
}

#[test]
fn init_force_replaces_an_existing_manifest() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("framestack.toml"), "old\n").unwrap();

    framestack()
        .args(["init", path_str(dir.path()), "--name", "fresh", "--force"])
        .assert()
        .code(0);

    let written = fs::read_to_string(dir.path().join("framestack.toml")).unwrap();
    assert!(written.contains("name = \"fresh\""), "{written}");
}

#[test]
fn init_into_a_missing_directory_exits_2() {
    let parent = tempfile::tempdir().unwrap();
    let missing = parent.path().join("not-here");

    framestack()
        .args(["init", path_str(&missing)])
        .assert()
        .code(2);
    assert!(!missing.exists(), "init must not create directories");
}

// ---- reference applications ----------------------------------------------

#[test]
fn example_leptos_axum_manifest_passes_check() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/rust-leptos-axum");
    framestack()
        .args(["check", "--manifest", path_str(&manifest)])
        .assert()
        .code(0)
        .stdout(predicate::str::contains("valid: 1 component"));
}
